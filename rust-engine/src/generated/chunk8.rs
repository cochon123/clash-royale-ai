#![allow(unused_parens,unreachable_code,unused_variables,unused_mut)]
use crate::{cpu::Cpu,runtime::Runtime};
use super::helpers::*;
#[inline]
pub(super) unsafe fn f_10789f34(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10789f34=>{
// 0x10789f34 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10789f38 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10789f3c stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10789f40 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10789f44 ldr x20, [x0, #0x38]
{let address=(cpu.x[0]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10789f48 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10789f4c cbz x20, #0x10789f84
if (cpu.x[20]) == 0 {pc=0x10789f84;continue;}pc=0x10789f50;continue;
}
0x10789f50=>{
// 0x10789f50 ldr x8, [x19, #0x40]
{let address=(cpu.x[19]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10789f54 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10789f58 cmp x8, x20
{let a=((cpu.x[8]));let b=((cpu.x[20]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10789f5c b.eq #0x10789f7c
if cpu.condition(0){pc=0x10789f7c;continue;}pc=0x10789f60;continue;
}
0x10789f60=>{
// 0x10789f60 sub x0, x8, #0x10
{let a=((cpu.x[8]));let b=((0x10u64));let result=a.wrapping_sub(b);cpu.x[0]=(result) as u64;}
// 0x10789f64 sub x21, x8, #0x18
{let a=((cpu.x[8]));let b=((0x18u64));let result=a.wrapping_sub(b);cpu.x[21]=(result) as u64;}
// 0x10789f68 bl #0x1142fcc4
cpu.x[30]=0x10789f6c;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
// 0x10789f6c cmp x21, x20
{let a=((cpu.x[21]));let b=((cpu.x[20]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10789f70 mov x8, x21
cpu.x[8]=(((cpu.x[21]))) as u64;
// 0x10789f74 b.ne #0x10789f60
if cpu.condition(1){pc=0x10789f60;continue;}pc=0x10789f78;continue;
}
0x10789f78=>{
// 0x10789f78 ldr x0, [x19, #0x38]
{let address=(cpu.x[19]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
pc=0x10789f7c;continue;
}
0x10789f7c=>{
// 0x10789f7c str x20, [x19, #0x40]
{let address=(cpu.x[19]).wrapping_add(0x40u64);let v0=cpu.x[20];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10789f80 bl #0x1187d2c0
cpu.x[30]=0x10789f84;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10789f84;continue;
}
0x10789f84=>{
// 0x10789f84 ldr x0, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10789f88 cbz x0, #0x10789f94
if (cpu.x[0]) == 0 {pc=0x10789f94;continue;}pc=0x10789f8c;continue;
}
0x10789f8c=>{
// 0x10789f8c str x0, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10789f90 bl #0x1187d2c0
cpu.x[30]=0x10789f94;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10789f94;continue;
}
0x10789f94=>{
// 0x10789f94 ldr x21, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10789f98 cbz x21, #0x10789fd8
if (cpu.x[21]) == 0 {pc=0x10789fd8;continue;}pc=0x10789f9c;continue;
}
0x10789f9c=>{
// 0x10789f9c ldr x20, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10789fa0 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10789fa4 cmp x20, x21
{let a=((cpu.x[20]));let b=((cpu.x[21]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10789fa8 b.eq #0x10789fc4
if cpu.condition(0){pc=0x10789fc4;continue;}pc=0x10789fac;continue;
}
0x10789fac=>{
// 0x10789fac sub x20, x20, #0x18
{let a=((cpu.x[20]));let b=((0x18u64));let result=a.wrapping_sub(b);cpu.x[20]=(result) as u64;}
// 0x10789fb0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10789fb4 bl #0x1142fcc4
cpu.x[30]=0x10789fb8;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
// 0x10789fb8 cmp x20, x21
{let a=((cpu.x[20]));let b=((cpu.x[21]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10789fbc b.ne #0x10789fac
if cpu.condition(1){pc=0x10789fac;continue;}pc=0x10789fc0;continue;
}
0x10789fc0=>{
// 0x10789fc0 ldr x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
pc=0x10789fc4;continue;
}
0x10789fc4=>{
// 0x10789fc4 str x21, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10789fc8 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10789fcc ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10789fd0 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10789fd4 b #0x1187d2c0
super::dispatch(cpu,rt,0x1187d2c0)?;return Ok(());
}
0x10789fd8=>{
// 0x10789fd8 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10789fdc ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10789fe0 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10789fe4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10789f34")),}}}
#[inline]
pub(super) unsafe fn f_10834700(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10834700=>{
// 0x10834700 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10834700")),}}}
#[inline]
pub(super) unsafe fn f_10d19614(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d19614=>{
// 0x10d19614 ldr x8, [x0, #0x18]
{let address=(cpu.x[0]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d19618 cbz x8, #0x10d19644
if (cpu.x[8]) == 0 {pc=0x10d19644;continue;}pc=0x10d1961c;continue;
}
0x10d1961c=>{
// 0x10d1961c ldr w9, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10d19620 sub w9, w9, #2
{let a=((cpu.x[9] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10d19624 cmp w9, #0x12
{let a=((cpu.x[9] as u32 as u64));let b=((0x12u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d19628 b.hs #0x10d1963c
if cpu.condition(2){pc=0x10d1963c;continue;}pc=0x10d1962c;continue;
}
0x10d1962c=>{
// 0x10d1962c mov w10, #0xa617
cpu.x[10]=(((0xa617u64))) as u32 as u64;
// 0x10d19630 movk w10, #2, lsl #16
cpu.x[10]=((cpu.x[10] as u32 as u64 & !(0xffff0000u64)) | 0x20000u64) as u32 as u64;
// 0x10d19634 lsr w9, w10, w9
cpu.x[9]=(((((cpu.x[10] as u32 as u64))) as u32).wrapping_shr(((((cpu.x[9] as u32 as u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10d19638 tbnz w9, #0, #0x10d19644
if ((cpu.x[9] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10d19644;continue;}pc=0x10d1963c;continue;
}
0x10d1963c=>{
// 0x10d1963c ldr x0, [x8, #0x1f0]
{let address=(cpu.x[8]).wrapping_add(0x1f0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d19640 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10d19644=>{
// 0x10d19644 mov x0, xzr
cpu.x[0]=(((0u64))) as u64;
// 0x10d19648 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d19614")),}}}
#[inline]
pub(super) unsafe fn f_10d27c68(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d27c68=>{
// 0x10d27c68 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d27c6c stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d27c70 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d27c74 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10d27c78 ldr x0, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d27c7c mov w19, w1
cpu.x[19]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x10d27c80 cbnz x0, #0x10d27c8c
if (cpu.x[0]) != 0 {pc=0x10d27c8c;continue;}pc=0x10d27c84;continue;
}
0x10d27c84=>{
// 0x10d27c84 ldr x0, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d27c88 bl #0x110cbf10
cpu.x[30]=0x10d27c8c;super::chunk14::f_110cbf10(cpu,rt,0x110cbf10)?;
pc=0x10d27c8c;continue;
}
0x10d27c8c=>{
// 0x10d27c8c bl #0x10e167b4
cpu.x[30]=0x10d27c90;super::chunk8::f_10e167b4(cpu,rt,0x10e167b4)?;
pc=0x10d27c90;continue;
}
0x10d27c90=>{
// 0x10d27c90 mov x2, x0
cpu.x[2]=(((cpu.x[0]))) as u64;
// 0x10d27c94 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d27c98 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10d27c9c mov w3, w19
cpu.x[3]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10d27ca0 mov w4, #1
cpu.x[4]=(((0x1u64))) as u32 as u64;
// 0x10d27ca4 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d27ca8 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10d27cac b #0x10d24ff4
super::chunk15::f_10d24ff4(cpu,rt,0x10d24ff4)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d27c68")),}}}
#[inline]
pub(super) unsafe fn f_10d282a4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d282a4=>{
// 0x10d282a4 tbnz w1, #0x1f, #0x10d282f0
if ((cpu.x[1] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x10d282f0;continue;}pc=0x10d282a8;continue;
}
0x10d282a8=>{
// 0x10d282a8 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d282ac stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d282b0 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d282b4 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10d282b8 ldr x0, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d282bc mov w19, w1
cpu.x[19]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x10d282c0 cbnz x0, #0x10d282cc
if (cpu.x[0]) != 0 {pc=0x10d282cc;continue;}pc=0x10d282c4;continue;
}
0x10d282c4=>{
// 0x10d282c4 ldr x0, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d282c8 bl #0x110cbf10
cpu.x[30]=0x10d282cc;super::chunk14::f_110cbf10(cpu,rt,0x110cbf10)?;
pc=0x10d282cc;continue;
}
0x10d282cc=>{
// 0x10d282cc bl #0x10e1afac
cpu.x[30]=0x10d282d0;super::chunk13::f_10e1afac(cpu,rt,0x10e1afac)?;
pc=0x10d282d0;continue;
}
0x10d282d0=>{
// 0x10d282d0 mov x2, x0
cpu.x[2]=(((cpu.x[0]))) as u64;
// 0x10d282d4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d282d8 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10d282dc mov w3, w19
cpu.x[3]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10d282e0 mov w4, #1
cpu.x[4]=(((0x1u64))) as u32 as u64;
// 0x10d282e4 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d282e8 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10d282ec b #0x10d24ff4
super::chunk15::f_10d24ff4(cpu,rt,0x10d24ff4)?;return Ok(());
}
0x10d282f0=>{
// 0x10d282f0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d282a4")),}}}
#[inline]
pub(super) unsafe fn f_10d296d0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d296d0=>{
// 0x10d296d0 b #0x10d294b0
super::chunk12::f_10d294b0(cpu,rt,0x10d294b0)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d296d0")),}}}
#[inline]
pub(super) unsafe fn f_10d2d554(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d2d554=>{
// 0x10d2d554 adrp x8, #0x1190d000
cpu.x[8]=(0x1190d000u64) as u64;
// 0x10d2d558 add x8, x8, #0x508
{let a=((cpu.x[8]));let b=((0x508u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d2d55c str x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d2d560 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d2d554")),}}}
#[inline]
pub(super) unsafe fn f_10d339d0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d339d0=>{
// 0x10d339d0 mov x8, x0
cpu.x[8]=(((cpu.x[0]))) as u64;
// 0x10d339d4 mov x0, xzr
cpu.x[0]=(((0u64))) as u64;
// 0x10d339d8 tbnz w1, #0x1f, #0x10d339f4
if ((cpu.x[1] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x10d339f4;continue;}pc=0x10d339dc;continue;
}
0x10d339dc=>{
// 0x10d339dc ldr x8, [x8, #0x2e8]
{let address=(cpu.x[8]).wrapping_add(0x2e8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d339e0 ldr w9, [x8, #0xc]
{let address=(cpu.x[8]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10d339e4 cmp w9, w1
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d339e8 b.le #0x10d339f4
if cpu.condition(13){pc=0x10d339f4;continue;}pc=0x10d339ec;continue;
}
0x10d339ec=>{
// 0x10d339ec ldr x8, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d339f0 ldr x0, [x8, w1, uxtw #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[1] as u32 as u64) as u32 as u64)) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
pc=0x10d339f4;continue;
}
0x10d339f4=>{
// 0x10d339f4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d339d0")),}}}
#[inline]
pub(super) unsafe fn f_10d398e8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d398e8=>{
// 0x10d398e8 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d398ec str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d398f0 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d398f4 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d398f8 ldr x8, [x1]
{let address=(cpu.x[1]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d398fc mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10d39900 ldr w1, [x0, #0x60]
{let address=(cpu.x[0]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d39904 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10d39908 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d3990c mov w21, w2
cpu.x[21]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x10d39910 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d39914 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d39918;super::dispatch(cpu,rt,target)?;}
pc=0x10d39918;continue;
}
0x10d39918=>{
// 0x10d39918 ldr x8, [x20, #0x2a8]
{let address=(cpu.x[20]).wrapping_add(0x2a8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3991c ldr x9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d39920 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d39924 cmp x8, #0
{let a=((cpu.x[8]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d39928 ldr x8, [x9, #0x40]
{let address=(cpu.x[9]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3992c cset w9, ne
cpu.x[9]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x10d39930 and w1, w21, w9
{let a=((cpu.x[21] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a & b;cpu.x[1]=(result) as u32 as u64;}
// 0x10d39934 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d39938;super::dispatch(cpu,rt,target)?;}
pc=0x10d39938;continue;
}
0x10d39938=>{
// 0x10d39938 tbz w0, #0, #0x10d39948
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d39948;continue;}pc=0x10d3993c;continue;
}
0x10d3993c=>{
// 0x10d3993c ldr x0, [x20, #0x2a8]
{let address=(cpu.x[20]).wrapping_add(0x2a8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d39940 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10d39944 bl #0x10f19548
cpu.x[30]=0x10d39948;super::dispatch(cpu,rt,0x10f19548)?;
pc=0x10d39948;continue;
}
0x10d39948=>{
// 0x10d39948 ldr x8, [x20, #0x2c8]
{let address=(cpu.x[20]).wrapping_add(0x2c8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3994c ldr x9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d39950 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d39954 cmp x8, #0
{let a=((cpu.x[8]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d39958 ldr x8, [x9, #0x40]
{let address=(cpu.x[9]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3995c cset w1, ne
cpu.x[1]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x10d39960 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d39964;super::dispatch(cpu,rt,target)?;}
pc=0x10d39964;continue;
}
0x10d39964=>{
// 0x10d39964 tbz w0, #0, #0x10d3997c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d3997c;continue;}pc=0x10d39968;continue;
}
0x10d39968=>{
// 0x10d39968 ldr x1, [x20, #0x2c8]
{let address=(cpu.x[20]).wrapping_add(0x2c8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10d3996c ldr x3, [x20, #0x20]
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[3]=(v0) as u64;}
// 0x10d39970 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d39974 mov w2, #0x5d
cpu.x[2]=(((0x5du64))) as u32 as u64;
// 0x10d39978 bl #0x111dd77c
cpu.x[30]=0x10d3997c;super::chunk15::f_111dd77c(cpu,rt,0x111dd77c)?;
pc=0x10d3997c;continue;
}
0x10d3997c=>{
// 0x10d3997c ldr x8, [x20, #0x2d0]
{let address=(cpu.x[20]).wrapping_add(0x2d0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d39980 ldr x9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d39984 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d39988 cmp x8, #0
{let a=((cpu.x[8]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d3998c ldr x8, [x9, #0x40]
{let address=(cpu.x[9]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d39990 cset w1, ne
cpu.x[1]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x10d39994 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d39998;super::dispatch(cpu,rt,target)?;}
pc=0x10d39998;continue;
}
0x10d39998=>{
// 0x10d39998 tbz w0, #0, #0x10d399b0
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d399b0;continue;}pc=0x10d3999c;continue;
}
0x10d3999c=>{
// 0x10d3999c ldr x1, [x20, #0x2d0]
{let address=(cpu.x[20]).wrapping_add(0x2d0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10d399a0 ldr x3, [x20, #0x20]
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[3]=(v0) as u64;}
// 0x10d399a4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d399a8 mov w2, #0x5d
cpu.x[2]=(((0x5du64))) as u32 as u64;
// 0x10d399ac bl #0x111dd77c
cpu.x[30]=0x10d399b0;super::chunk15::f_111dd77c(cpu,rt,0x111dd77c)?;
pc=0x10d399b0;continue;
}
0x10d399b0=>{
// 0x10d399b0 ldr w8, [x20, #0x60]
{let address=(cpu.x[20]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d399b4 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d399b8 b.lt #0x10d399e4
if cpu.condition(11){pc=0x10d399e4;continue;}pc=0x10d399bc;continue;
}
0x10d399bc=>{
// 0x10d399bc mov x21, xzr
cpu.x[21]=(((0u64))) as u64;
pc=0x10d399c0;continue;
}
0x10d399c0=>{
// 0x10d399c0 ldr x8, [x20, #0x2e8]
{let address=(cpu.x[20]).wrapping_add(0x2e8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d399c4 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10d399c8 ldr x8, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d399cc ldr x0, [x8, x21, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[21])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d399d0 bl #0x110278cc
cpu.x[30]=0x10d399d4;super::chunk5::f_110278cc(cpu,rt,0x110278cc)?;
pc=0x10d399d4;continue;
}
0x10d399d4=>{
// 0x10d399d4 ldrsw x8, [x20, #0x60]
{let address=(cpu.x[20]).wrapping_add(0x60u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10d399d8 add x21, x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[21]=(result) as u64;}
// 0x10d399dc cmp x21, x8
{let a=((cpu.x[21]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d399e0 b.lt #0x10d399c0
if cpu.condition(11){pc=0x10d399c0;continue;}pc=0x10d399e4;continue;
}
0x10d399e4=>{
// 0x10d399e4 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d399e8 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10d399ec ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10d399f0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d398e8")),}}}
#[inline]
pub(super) unsafe fn f_10d3d63c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d3d63c=>{
// 0x10d3d63c stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d3d640 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d3d644 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d3d648 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d3d64c ldp x19, x20, [x0, #0xe0]
{let address=(cpu.x[0]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[19]=(v0) as u64;cpu.x[20]=(v1) as u64;}
// 0x10d3d650 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d3d654 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d3d658 bl #0x10fa4038
cpu.x[30]=0x10d3d65c;super::chunk15::f_10fa4038(cpu,rt,0x10fa4038)?;
pc=0x10d3d65c;continue;
}
0x10d3d65c=>{
// 0x10d3d65c mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d3d660 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3d664 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10d3d668 bl #0x10fa4038
cpu.x[30]=0x10d3d66c;super::chunk15::f_10fa4038(cpu,rt,0x10fa4038)?;
pc=0x10d3d66c;continue;
}
0x10d3d66c=>{
// 0x10d3d66c cmp w21, w0
{let a=((cpu.x[21] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3d670 cset w0, eq
cpu.x[0]=((cpu.condition(0)) as u64) as u32 as u64;
// 0x10d3d674 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d3d678 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10d3d67c ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10d3d680 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d3d63c")),}}}
#[inline]
pub(super) unsafe fn f_10d3dcac(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d3dcac=>{
// 0x10d3dcac cbz x1, #0x10d3dd88
if (cpu.x[1]) == 0 {pc=0x10d3dd88;continue;}pc=0x10d3dcb0;continue;
}
0x10d3dcb0=>{
// 0x10d3dcb0 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d3dcb4 str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d3dcb8 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d3dcbc mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10d3dcc0 mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x10d3dcc4 bl #0x10d22e78
cpu.x[30]=0x10d3dcc8;super::chunk0::f_10d22e78(cpu,rt,0x10d22e78)?;
pc=0x10d3dcc8;continue;
}
0x10d3dcc8=>{
// 0x10d3dcc8 ldr w11, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
// 0x10d3dccc ldp w10, w9, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[10]=(v0) as u32 as u64;cpu.x[9]=(v1) as u32 as u64;}
// 0x10d3dcd0 cmp w11, #1
{let a=((cpu.x[11] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3dcd4 b.lt #0x10d3dd3c
if cpu.condition(11){pc=0x10d3dd3c;continue;}pc=0x10d3dcd8;continue;
}
0x10d3dcd8=>{
// 0x10d3dcd8 mov x8, xzr
cpu.x[8]=(((0u64))) as u64;
// 0x10d3dcdc add x12, x19, #0x30
{let a=((cpu.x[19]));let b=((0x30u64));let result=a.wrapping_add(b);cpu.x[12]=(result) as u64;}
// 0x10d3dce0 b #0x10d3dcf0
pc=0x10d3dcf0;continue;
}
0x10d3dce4=>{
// 0x10d3dce4 add x8, x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d3dce8 cmp x11, x8
{let a=((cpu.x[11]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d3dcec b.eq #0x10d3dd3c
if cpu.condition(0){pc=0x10d3dd3c;continue;}pc=0x10d3dcf0;continue;
}
0x10d3dcf0=>{
// 0x10d3dcf0 ldr x13, [x12, x8, lsl #3]
{let address=((cpu.x[12]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[8])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[13]=(v0) as u64;}
// 0x10d3dcf4 ldr w14, [x13]
{let address=(cpu.x[13]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[14]=(v0) as u32 as u64;}
// 0x10d3dcf8 cmp w14, w10
{let a=((cpu.x[14] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3dcfc b.ne #0x10d3dce4
if cpu.condition(1){pc=0x10d3dce4;continue;}pc=0x10d3dd00;continue;
}
0x10d3dd00=>{
// 0x10d3dd00 ldr w13, [x13, #4]
{let address=(cpu.x[13]).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[13]=(v0) as u32 as u64;}
// 0x10d3dd04 cmp w13, w9
{let a=((cpu.x[13] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3dd08 b.ne #0x10d3dce4
if cpu.condition(1){pc=0x10d3dce4;continue;}pc=0x10d3dd0c;continue;
}
0x10d3dd0c=>{
// 0x10d3dd0c tbnz w8, #0x1f, #0x10d3dd6c
if ((cpu.x[8] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x10d3dd6c;continue;}pc=0x10d3dd10;continue;
}
0x10d3dd10=>{
// 0x10d3dd10 ldr x9, [x19, #0x2e8]
{let address=(cpu.x[19]).wrapping_add(0x2e8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d3dd14 ldr w10, [x9, #0xc]
{let address=(cpu.x[9]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10d3dd18 cmp w10, w8
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3dd1c b.le #0x10d3dd6c
if cpu.condition(13){pc=0x10d3dd6c;continue;}pc=0x10d3dd20;continue;
}
0x10d3dd20=>{
// 0x10d3dd20 ldr x9, [x9]
{let address=(cpu.x[9]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d3dd24 ldr x0, [x9, w8, uxtw #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as u32 as u64)) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3dd28 cbz x0, #0x10d3dd7c
if (cpu.x[0]) == 0 {pc=0x10d3dd7c;continue;}pc=0x10d3dd2c;continue;
}
0x10d3dd2c=>{
// 0x10d3dd2c bl #0x110293f8
cpu.x[30]=0x10d3dd30;super::chunk11::f_110293f8(cpu,rt,0x110293f8)?;
pc=0x10d3dd30;continue;
}
0x10d3dd30=>{
// 0x10d3dd30 cmp w0, #3
{let a=((cpu.x[0] as u32 as u64));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3dd34 cset w0, eq
cpu.x[0]=((cpu.condition(0)) as u64) as u32 as u64;
// 0x10d3dd38 b #0x10d3dd7c
pc=0x10d3dd7c;continue;
}
0x10d3dd3c=>{
// 0x10d3dd3c ldr w8, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3dd40 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10d3dd44 cmp w8, w10
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3dd48 b.ne #0x10d3dd7c
if cpu.condition(1){pc=0x10d3dd7c;continue;}pc=0x10d3dd4c;continue;
}
0x10d3dd4c=>{
// 0x10d3dd4c ldr w8, [x19, #0x2c]
{let address=(cpu.x[19]).wrapping_add(0x2cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3dd50 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3dd54 b.ne #0x10d3dd7c
if cpu.condition(1){pc=0x10d3dd7c;continue;}pc=0x10d3dd58;continue;
}
0x10d3dd58=>{
// 0x10d3dd58 mov w8, #0x64
cpu.x[8]=(((0x64u64))) as u32 as u64;
// 0x10d3dd5c ldr x9, [x19, #0x2e8]
{let address=(cpu.x[19]).wrapping_add(0x2e8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d3dd60 ldr w10, [x9, #0xc]
{let address=(cpu.x[9]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10d3dd64 cmp w10, w8
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3dd68 b.gt #0x10d3dd20
if cpu.condition(12){pc=0x10d3dd20;continue;}pc=0x10d3dd6c;continue;
}
0x10d3dd6c=>{
// 0x10d3dd6c mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10d3dd70 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10d3dd74 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10d3dd78 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10d3dd7c=>{
// 0x10d3dd7c ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10d3dd80 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10d3dd84 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10d3dd88=>{
// 0x10d3dd88 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10d3dd8c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d3dcac")),}}}
#[inline]
pub(super) unsafe fn f_10d44be8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d44be8=>{
// 0x10d44be8 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d44bec str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d44bf0 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d44bf4 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d44bf8 movi v0.2d, #0xffffffffffffffff
cpu.q[0]=(lane_dup(18446744073709551615u128,64,2)) & 0xffffffffffffffffffffffffffffffffu128;
// 0x10d44bfc mov x8, #-1
cpu.x[8]=(((0xffffffffffffffffu64))) as u64;
pc=0x10d44c00;continue;
}
0x10d44c00=>{
// 0x10d44c00 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10d44c04 str wzr, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d44c08 mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x10d44c0c stur x8, [x0, #4]
{let address=(cpu.x[0]).wrapping_add(0x4u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d44c10 str w8, [x0, #0xc]
{let address=(cpu.x[0]).wrapping_add(0xcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d44c14 stp xzr, xzr, [x0, #0x118]
{let address=(cpu.x[0]).wrapping_add(0x118u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d44c18 stp xzr, xzr, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d44c1c stur q0, [x0, #0x48]
{let address=(cpu.x[0]).wrapping_add(0x48u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d44c20 stur q0, [x0, #0x58]
{let address=(cpu.x[0]).wrapping_add(0x58u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d44c24 stur q0, [x0, #0x68]
{let address=(cpu.x[0]).wrapping_add(0x68u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d44c28 stur q0, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d44c2c stp xzr, xzr, [x0, #0x160]
{let address=(cpu.x[0]).wrapping_add(0x160u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d44c30 stur q0, [x0, #0x98]
{let address=(cpu.x[0]).wrapping_add(0x98u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d44c34 stur q0, [x0, #0x88]
{let address=(cpu.x[0]).wrapping_add(0x88u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d44c38 stp xzr, xzr, [x0, #0x138]
{let address=(cpu.x[0]).wrapping_add(0x138u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d44c3c stp x8, x8, [x0, #0x38]
{let address=(cpu.x[0]).wrapping_add(0x38u64);let v0=cpu.x[8];let v1=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d44c40 stp x8, x8, [x0, #0x28]
{let address=(cpu.x[0]).wrapping_add(0x28u64);let v0=cpu.x[8];let v1=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d44c44 stur q0, [x0, #0xb8]
{let address=(cpu.x[0]).wrapping_add(0xb8u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d44c48 stur q0, [x0, #0xa8]
{let address=(cpu.x[0]).wrapping_add(0xa8u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d44c4c stp xzr, xzr, [x0, #0x128]
{let address=(cpu.x[0]).wrapping_add(0x128u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d44c50 str xzr, [x0, #0x20]
{let address=(cpu.x[0]).wrapping_add(0x20u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d44c54 stur q0, [x0, #0xd8]
{let address=(cpu.x[0]).wrapping_add(0xd8u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d44c58 stur q0, [x0, #0xc8]
{let address=(cpu.x[0]).wrapping_add(0xc8u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d44c5c stp xzr, xzr, [x0, #0x150]
{let address=(cpu.x[0]).wrapping_add(0x150u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d44c60 str xzr, [x0, #0x148]
{let address=(cpu.x[0]).wrapping_add(0x148u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d44c64 stur q0, [x0, #0xf8]
{let address=(cpu.x[0]).wrapping_add(0xf8u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d44c68 stur q0, [x0, #0xe8]
{let address=(cpu.x[0]).wrapping_add(0xe8u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d44c6c str wzr, [x0, #0x180]
{let address=(cpu.x[0]).wrapping_add(0x180u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d44c70 stp xzr, xzr, [x0, #0x170]
{let address=(cpu.x[0]).wrapping_add(0x170u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d44c74 str x1, [x0, #0x188]
{let address=(cpu.x[0]).wrapping_add(0x188u64);let v0=cpu.x[1];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d44c78 mov w0, #0x28
cpu.x[0]=(((0x28u64))) as u32 as u64;
// 0x10d44c7c bl #0x1187d2b0
cpu.x[30]=0x10d44c80;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
pc=0x10d44c80;continue;
}
0x10d44c80=>{
// 0x10d44c80 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d44c84 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10d44c88 bl #0x10d5402c
cpu.x[30]=0x10d44c8c;super::chunk9::f_10d5402c(cpu,rt,0x10d5402c)?;
pc=0x10d44c8c;continue;
}
0x10d44c8c=>{
// 0x10d44c8c ldr x20, [x19, #0x188]
{let address=(cpu.x[19]).wrapping_add(0x188u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10d44c90 mov w0, #0x28
cpu.x[0]=(((0x28u64))) as u32 as u64;
// 0x10d44c94 str x21, [x19, #0x108]
{let address=(cpu.x[19]).wrapping_add(0x108u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d44c98 bl #0x1187d2b0
cpu.x[30]=0x10d44c9c;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
pc=0x10d44c9c;continue;
}
0x10d44c9c=>{
// 0x10d44c9c mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d44ca0 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10d44ca4 bl #0x10d5402c
cpu.x[30]=0x10d44ca8;super::chunk9::f_10d5402c(cpu,rt,0x10d5402c)?;
pc=0x10d44ca8;continue;
}
0x10d44ca8=>{
// 0x10d44ca8 str x21, [x19, #0x110]
{let address=(cpu.x[19]).wrapping_add(0x110u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d44cac ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d44cb0 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10d44cb4 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10d44cb8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d44be8")),}}}
#[inline]
pub(super) unsafe fn f_10d54838(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d54838=>{
// 0x10d54838 stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d5483c str x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d54840 stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d54844 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d54848 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d5484c mov w19, w4
cpu.x[19]=(((cpu.x[4] as u32 as u64))) as u32 as u64;
// 0x10d54850 mov w4, wzr
cpu.x[4]=(((0u64))) as u32 as u64;
// 0x10d54854 mov w5, #1
cpu.x[5]=(((0x1u64))) as u32 as u64;
// 0x10d54858 mov w20, w3
cpu.x[20]=(((cpu.x[3] as u32 as u64))) as u32 as u64;
// 0x10d5485c mov w21, w2
cpu.x[21]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x10d54860 mov x22, x1
cpu.x[22]=(((cpu.x[1]))) as u64;
// 0x10d54864 mov x23, x0
cpu.x[23]=(((cpu.x[0]))) as u64;
// 0x10d54868 bl #0x10d54530
cpu.x[30]=0x10d5486c;super::chunk14::f_10d54530(cpu,rt,0x10d54530)?;
pc=0x10d5486c;continue;
}
0x10d5486c=>{
// 0x10d5486c mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d54870 mov x1, x22
cpu.x[1]=(((cpu.x[22]))) as u64;
// 0x10d54874 mov w2, w21
cpu.x[2]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10d54878 mov w3, w20
cpu.x[3]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x10d5487c mov w4, #0x10
cpu.x[4]=(((0x10u64))) as u32 as u64;
// 0x10d54880 mov w5, w19
cpu.x[5]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10d54884 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d54888 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10d5488c ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d54890 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10d54894 b #0x10d54530
super::chunk14::f_10d54530(cpu,rt,0x10d54530)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d54838")),}}}
#[inline]
pub(super) unsafe fn f_10d54a44(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d54a44=>{
// 0x10d54a44 stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d54a48 str x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d54a4c stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d54a50 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d54a54 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d54a58 ldr w8, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d54a5c mov w19, w2
cpu.x[19]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x10d54a60 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10d54a64 mov x22, x1
cpu.x[22]=(((cpu.x[1]))) as u64;
// 0x10d54a68 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d54a6c b.lt #0x10d54a9c
if cpu.condition(11){pc=0x10d54a9c;continue;}pc=0x10d54a70;continue;
}
0x10d54a70=>{
// 0x10d54a70 mov x23, xzr
cpu.x[23]=(((0u64))) as u64;
pc=0x10d54a74;continue;
}
0x10d54a74=>{
// 0x10d54a74 ldr x8, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d54a78 mov x1, x22
cpu.x[1]=(((cpu.x[22]))) as u64;
// 0x10d54a7c ldr x21, [x8, x23, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[23])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10d54a80 ldr x0, [x21, #8]
{let address=(cpu.x[21]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d54a84 bl #0x10e0bc20
cpu.x[30]=0x10d54a88;super::chunk1::f_10e0bc20(cpu,rt,0x10e0bc20)?;
pc=0x10d54a88;continue;
}
0x10d54a88=>{
// 0x10d54a88 tbnz w0, #0, #0x10d54af8
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10d54af8;continue;}pc=0x10d54a8c;continue;
}
0x10d54a8c=>{
// 0x10d54a8c ldrsw x8, [x20, #0x14]
{let address=(cpu.x[20]).wrapping_add(0x14u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10d54a90 add x23, x23, #1
{let a=((cpu.x[23]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[23]=(result) as u64;}
// 0x10d54a94 cmp x23, x8
{let a=((cpu.x[23]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d54a98 b.lt #0x10d54a74
if cpu.condition(11){pc=0x10d54a74;continue;}pc=0x10d54a9c;continue;
}
0x10d54a9c=>{
// 0x10d54a9c ldr x23, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10d54aa0 mov w0, #0x100
cpu.x[0]=(((0x100u64))) as u32 as u64;
// 0x10d54aa4 bl #0x1187d2b0
cpu.x[30]=0x10d54aa8;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
pc=0x10d54aa8;continue;
}
0x10d54aa8=>{
// 0x10d54aa8 mov x1, x23
cpu.x[1]=(((cpu.x[23]))) as u64;
// 0x10d54aac mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10d54ab0 bl #0x10d56f30
cpu.x[30]=0x10d54ab4;super::chunk0::f_10d56f30(cpu,rt,0x10d56f30)?;
pc=0x10d54ab4;continue;
}
0x10d54ab4=>{
// 0x10d54ab4 ldp w9, w8, [x20, #0x10]
{let address=(cpu.x[20]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10d54ab8 str x22, [x21, #8]
{let address=(cpu.x[21]).wrapping_add(0x8u64);let v0=cpu.x[22];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d54abc cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d54ac0 b.ne #0x10d54ae8
if cpu.condition(1){pc=0x10d54ae8;continue;}pc=0x10d54ac4;continue;
}
0x10d54ac4=>{
// 0x10d54ac4 lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10d54ac8 mov w10, #5
cpu.x[10]=(((0x5u64))) as u32 as u64;
// 0x10d54acc cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d54ad0 csel w1, w10, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[10] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10d54ad4 cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d54ad8 b.ge #0x10d54ae8
if cpu.condition(10){pc=0x10d54ae8;continue;}pc=0x10d54adc;continue;
}
0x10d54adc=>{
// 0x10d54adc add x0, x20, #8
{let a=((cpu.x[20]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d54ae0 bl #0x10d55568
cpu.x[30]=0x10d54ae4;super::chunk13::f_10d55568(cpu,rt,0x10d55568)?;
// 0x10d54ae4 ldr w8, [x20, #0x14]
{let address=(cpu.x[20]).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
pc=0x10d54ae8;continue;
}
0x10d54ae8=>{
// 0x10d54ae8 ldr x9, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d54aec add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10d54af0 str w10, [x20, #0x14]
{let address=(cpu.x[20]).wrapping_add(0x14u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d54af4 str x21, [x9, w8, sxtw #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10d54af8;continue;
}
0x10d54af8=>{
// 0x10d54af8 ldr w8, [x21, #0x98]
{let address=(cpu.x[21]).wrapping_add(0x98u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d54afc mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10d54b00 mov w1, #0x21
cpu.x[1]=(((0x21u64))) as u32 as u64;
// 0x10d54b04 mov w2, #1
cpu.x[2]=(((0x1u64))) as u32 as u64;
// 0x10d54b08 mov w3, w19
cpu.x[3]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10d54b0c add w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10d54b10 str w8, [x21, #0x98]
{let address=(cpu.x[21]).wrapping_add(0x98u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d54b14 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d54b18 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10d54b1c ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d54b20 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10d54b24 b #0x10d5701c
super::chunk14::f_10d5701c(cpu,rt,0x10d5701c)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d54a44")),}}}
#[inline]
pub(super) unsafe fn f_10d5645c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d5645c=>{
// 0x10d5645c ldrb w8, [x0, #0x19]
{let address=(cpu.x[0]).wrapping_add(0x19u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d56460 cbz w8, #0x10d56468
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10d56468;continue;}pc=0x10d56464;continue;
}
0x10d56464=>{
// 0x10d56464 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10d56468=>{
// 0x10d56468 ldr w9, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10d5646c mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10d56470 strb w8, [x0, #0x19]
{let address=(cpu.x[0]).wrapping_add(0x19u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10d56474 str w9, [x0, #0x5c]
{let address=(cpu.x[0]).wrapping_add(0x5cu64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d56478 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d5645c")),}}}
#[inline]
pub(super) unsafe fn f_10d56e58(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d56e58=>{
// 0x10d56e58 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d56e5c str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d56e60 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d56e64 ldr x8, [x1]
{let address=(cpu.x[1]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d56e68 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10d56e6c mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x10d56e70 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d56e74 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d56e78;super::dispatch(cpu,rt,target)?;}
pc=0x10d56e78;continue;
}
0x10d56e78=>{
// 0x10d56e78 str w0, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d56e7c ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10d56e80 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10d56e84 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d56e58")),}}}
#[inline]
pub(super) unsafe fn f_10d5a27c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d5a27c=>{
// 0x10d5a27c movi v0.2d, #0000000000000000
cpu.q[0]=(lane_dup(0u128,64,2)) & 0xffffffffffffffffffffffffffffffffu128;
// 0x10d5a280 mov w8, #0x3f800000
cpu.x[8]=(((0x3f800000u64))) as u32 as u64;
// 0x10d5a284 str xzr, [x0, #0x28]
{let address=(cpu.x[0]).wrapping_add(0x28u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d5a288 str w8, [x0, #0x28]
{let address=(cpu.x[0]).wrapping_add(0x28u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d5a28c adrp x8, #0x1190e000
cpu.x[8]=(0x1190e000u64) as u64;
// 0x10d5a290 add x8, x8, #0xb50
{let a=((cpu.x[8]));let b=((0xb50u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d5a294 str x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d5a298 stur q0, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d5a29c stur q0, [x0, #0x18]
{let address=(cpu.x[0]).wrapping_add(0x18u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d5a2a0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d5a27c")),}}}
#[inline]
pub(super) unsafe fn f_10d75930(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d75930=>{
// 0x10d75930 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d75934 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d75938 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d7593c movi v0.2d, #0000000000000000
cpu.q[0]=(lane_dup(0u128,64,2)) & 0xffffffffffffffffffffffffffffffffu128;
// 0x10d75940 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10d75944 stur q0, [x0, #0x19]
{let address=(cpu.x[0]).wrapping_add(0x19u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d75948 stp q0, q0, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10d7594c str x1, [x0, #0x20]
{let address=(cpu.x[0]).wrapping_add(0x20u64);let v0=cpu.x[1];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d75950 mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x10d75954 bl #0x110cbf10
cpu.x[30]=0x10d75958;super::chunk14::f_110cbf10(cpu,rt,0x110cbf10)?;
pc=0x10d75958;continue;
}
0x10d75958=>{
// 0x10d75958 ldr w8, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d7595c str x0, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d75960 cmp w8, #9
{let a=((cpu.x[8] as u32 as u64));let b=((0x9u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d75964 b.gt #0x10d75974
if cpu.condition(12){pc=0x10d75974;continue;}pc=0x10d75968;continue;
}
0x10d75968=>{
// 0x10d75968 add x0, x19, #8
{let a=((cpu.x[19]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d7596c mov w1, #0xa
cpu.x[1]=(((0xau64))) as u32 as u64;
// 0x10d75970 bl #0x1081b608
cpu.x[30]=0x10d75974;super::chunk12::f_1081b608(cpu,rt,0x1081b608)?;
pc=0x10d75974;continue;
}
0x10d75974=>{
// 0x10d75974 mov w0, #8
cpu.x[0]=(((0x8u64))) as u32 as u64;
// 0x10d75978 bl #0x1187d2b0
cpu.x[30]=0x10d7597c;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
pc=0x10d7597c;continue;
}
0x10d7597c=>{
// 0x10d7597c mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10d75980 bl #0x10daaeb0
cpu.x[30]=0x10d75984;super::chunk13::f_10daaeb0(cpu,rt,0x10daaeb0)?;
pc=0x10d75984;continue;
}
0x10d75984=>{
// 0x10d75984 str x20, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=cpu.x[20];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d75988 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d7598c ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10d75990 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d75930")),}}}
#[inline]
pub(super) unsafe fn f_10d77c8c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d77c8c=>{
// 0x10d77c8c ldr w8, [x1, #0x10]
{let address=(cpu.x[1]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d77c90 ldr q0, [x1]
{let address=(cpu.x[1]).wrapping_add(0x0u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));cpu.q[0]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x10d77c94 str w8, [x0, #0x48]
{let address=(cpu.x[0]).wrapping_add(0x48u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d77c98 stur q0, [x0, #0x38]
{let address=(cpu.x[0]).wrapping_add(0x38u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d77c9c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d77c8c")),}}}
#[inline]
pub(super) unsafe fn f_10d79748(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d79748=>{
// 0x10d79748 mov w0, #0xbd
cpu.x[0]=(((0xbdu64))) as u32 as u64;
// 0x10d7974c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d79748")),}}}
#[inline]
pub(super) unsafe fn f_10dbbd5c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10dbbd5c=>{
// 0x10dbbd5c ldr x9, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10dbbd60 cbz x9, #0x10dbbe2c
if (cpu.x[9]) == 0 {pc=0x10dbbe2c;continue;}pc=0x10dbbd64;continue;
}
0x10dbbd64=>{
// 0x10dbbd64 fmov d0, x9
cpu.q[0]=(cpu.x[9]) as u128 & 0xffffffffffffffffu128;
// 0x10dbbd68 sxtw x8, w1
cpu.x[8]=((((cpu.x[1] as u32 as u64))) as i32 as i64 as u64) as u64;
// 0x10dbbd6c cnt v0.8b, v0.8b
cpu.q[0]=(lane_count((cpu.q[0] & 0xffffffffffffffffu128),8,8)) & 0xffffffffffffffffu128;
// 0x10dbbd70 uaddlv h0, v0.8b
cpu.q[0]=(lane_reduce((cpu.q[0] & 0xffffffffffffffffu128),8,8,"uaddlv")) & 0xffffu128;
// 0x10dbbd74 fmov w12, s0
cpu.x[12]=((cpu.q[0] & 0xffffffffu128) as u64) as u32 as u64;
// 0x10dbbd78 cmp x12, #1
{let a=((cpu.x[12]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10dbbd7c b.hi #0x10dbbd8c
if cpu.condition(8){pc=0x10dbbd8c;continue;}pc=0x10dbbd80;continue;
}
0x10dbbd80=>{
// 0x10dbbd80 sub x10, x9, #1
{let a=((cpu.x[9]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[10]=(result) as u64;}
// 0x10dbbd84 and x10, x10, x8
{let a=((cpu.x[10]));let b=((cpu.x[8]));let result=a & b;cpu.x[10]=(result) as u64;}
// 0x10dbbd88 b #0x10dbbda0
pc=0x10dbbda0;continue;
}
0x10dbbd8c=>{
// 0x10dbbd8c cmp x9, x8
{let a=((cpu.x[9]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10dbbd90 mov x10, x8
cpu.x[10]=(((cpu.x[8]))) as u64;
// 0x10dbbd94 b.hi #0x10dbbda0
if cpu.condition(8){pc=0x10dbbda0;continue;}pc=0x10dbbd98;continue;
}
0x10dbbd98=>{
// 0x10dbbd98 udiv x10, x8, x9
{let a=((cpu.x[8])) as u64;let b=((cpu.x[9])) as u64;cpu.x[10]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u64;}
// 0x10dbbd9c msub x10, x10, x9, x8
cpu.x[10]=((((cpu.x[8]))).wrapping_sub((((cpu.x[10]))).wrapping_mul(((cpu.x[9]))))) as u64;
pc=0x10dbbda0;continue;
}
0x10dbbda0=>{
// 0x10dbbda0 ldr x11, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[11]=(v0) as u64;}
// 0x10dbbda4 ldr x11, [x11, x10, lsl #3]
{let address=((cpu.x[11]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[10])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[11]=(v0) as u64;}
// 0x10dbbda8 cbz x11, #0x10dbbe2c
if (cpu.x[11]) == 0 {pc=0x10dbbe2c;continue;}pc=0x10dbbdac;continue;
}
0x10dbbdac=>{
// 0x10dbbdac ldr x11, [x11]
{let address=(cpu.x[11]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[11]=(v0) as u64;}
// 0x10dbbdb0 cbz x11, #0x10dbbe2c
if (cpu.x[11]) == 0 {pc=0x10dbbe2c;continue;}pc=0x10dbbdb4;continue;
}
0x10dbbdb4=>{
// 0x10dbbdb4 cmp x12, #2
{let a=((cpu.x[12]));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10dbbdb8 b.hs #0x10dbbe08
if cpu.condition(2){pc=0x10dbbe08;continue;}pc=0x10dbbdbc;continue;
}
0x10dbbdbc=>{
// 0x10dbbdbc sub x9, x9, #1
{let a=((cpu.x[9]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u64;}
// 0x10dbbdc0 b #0x10dbbdd8
pc=0x10dbbdd8;continue;
}
0x10dbbdc4=>{
// 0x10dbbdc4 ldr w12, [x11, #0x10]
{let address=(cpu.x[11]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[12]=(v0) as u32 as u64;}
// 0x10dbbdc8 cmp w12, w1
{let a=((cpu.x[12] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10dbbdcc b.eq #0x10dbbe34
if cpu.condition(0){pc=0x10dbbe34;continue;}pc=0x10dbbdd0;continue;
}
0x10dbbdd0=>{
// 0x10dbbdd0 ldr x11, [x11]
{let address=(cpu.x[11]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[11]=(v0) as u64;}
// 0x10dbbdd4 cbz x11, #0x10dbbe2c
if (cpu.x[11]) == 0 {pc=0x10dbbe2c;continue;}pc=0x10dbbdd8;continue;
}
0x10dbbdd8=>{
// 0x10dbbdd8 ldr x12, [x11, #8]
{let address=(cpu.x[11]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[12]=(v0) as u64;}
// 0x10dbbddc cmp x12, x8
{let a=((cpu.x[12]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10dbbde0 b.eq #0x10dbbdc4
if cpu.condition(0){pc=0x10dbbdc4;continue;}pc=0x10dbbde4;continue;
}
0x10dbbde4=>{
// 0x10dbbde4 and x12, x12, x9
{let a=((cpu.x[12]));let b=((cpu.x[9]));let result=a & b;cpu.x[12]=(result) as u64;}
// 0x10dbbde8 cmp x12, x10
{let a=((cpu.x[12]));let b=((cpu.x[10]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10dbbdec b.eq #0x10dbbdd0
if cpu.condition(0){pc=0x10dbbdd0;continue;}pc=0x10dbbdf0;continue;
}
0x10dbbdf0=>{
// 0x10dbbdf0 b #0x10dbbe2c
pc=0x10dbbe2c;continue;
}
0x10dbbdf4=>{
// 0x10dbbdf4 ldr w12, [x11, #0x10]
{let address=(cpu.x[11]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[12]=(v0) as u32 as u64;}
// 0x10dbbdf8 cmp w12, w1
{let a=((cpu.x[12] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10dbbdfc b.eq #0x10dbbe34
if cpu.condition(0){pc=0x10dbbe34;continue;}pc=0x10dbbe00;continue;
}
0x10dbbe00=>{
// 0x10dbbe00 ldr x11, [x11]
{let address=(cpu.x[11]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[11]=(v0) as u64;}
// 0x10dbbe04 cbz x11, #0x10dbbe2c
if (cpu.x[11]) == 0 {pc=0x10dbbe2c;continue;}pc=0x10dbbe08;continue;
}
0x10dbbe08=>{
// 0x10dbbe08 ldr x12, [x11, #8]
{let address=(cpu.x[11]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[12]=(v0) as u64;}
// 0x10dbbe0c cmp x12, x8
{let a=((cpu.x[12]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10dbbe10 b.eq #0x10dbbdf4
if cpu.condition(0){pc=0x10dbbdf4;continue;}pc=0x10dbbe14;continue;
}
0x10dbbe14=>{
// 0x10dbbe14 cmp x12, x9
{let a=((cpu.x[12]));let b=((cpu.x[9]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10dbbe18 b.lo #0x10dbbe24
if cpu.condition(3){pc=0x10dbbe24;continue;}pc=0x10dbbe1c;continue;
}
0x10dbbe1c=>{
// 0x10dbbe1c udiv x13, x12, x9
{let a=((cpu.x[12])) as u64;let b=((cpu.x[9])) as u64;cpu.x[13]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u64;}
// 0x10dbbe20 msub x12, x13, x9, x12
cpu.x[12]=((((cpu.x[12]))).wrapping_sub((((cpu.x[13]))).wrapping_mul(((cpu.x[9]))))) as u64;
pc=0x10dbbe24;continue;
}
0x10dbbe24=>{
// 0x10dbbe24 cmp x12, x10
{let a=((cpu.x[12]));let b=((cpu.x[10]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10dbbe28 b.eq #0x10dbbe00
if cpu.condition(0){pc=0x10dbbe00;continue;}pc=0x10dbbe2c;continue;
}
0x10dbbe2c=>{
// 0x10dbbe2c mov x0, xzr
cpu.x[0]=(((0u64))) as u64;
// 0x10dbbe30 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10dbbe34=>{
// 0x10dbbe34 ldr x0, [x11, #0x18]
{let address=(cpu.x[11]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10dbbe38 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10dbbd5c")),}}}
#[inline]
pub(super) unsafe fn f_10dc1170(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10dc1170=>{
// 0x10dc1170 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10dc1174 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10dc1170")),}}}
#[inline]
pub(super) unsafe fn f_10ddad10(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10ddad10=>{
// 0x10ddad10 ldr w0, [x0, #0x1d8]
{let address=(cpu.x[0]).wrapping_add(0x1d8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10ddad14 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10ddad10")),}}}
#[inline]
pub(super) unsafe fn f_10de01bc(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10de01bc=>{
// 0x10de01bc stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10de01c0 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10de01c4 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10de01c8 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10de01cc ldrb w8, [x0, #0x572]
{let address=(cpu.x[0]).wrapping_add(0x572u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10de01d0 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10de01d4 mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10de01d8 cbz w8, #0x10de0208
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10de0208;continue;}pc=0x10de01dc;continue;
}
0x10de01dc=>{
// 0x10de01dc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10de01e0 bl #0x10e0b6b4
cpu.x[30]=0x10de01e4;super::chunk8::f_10e0b6b4(cpu,rt,0x10e0b6b4)?;
pc=0x10de01e4;continue;
}
0x10de01e4=>{
// 0x10de01e4 ldr x0, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10de01e8 ldr w1, [x20, #0x7b0]
{let address=(cpu.x[20]).wrapping_add(0x7b0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10de01ec and x2, x19, #0xffffffff
{let a=((cpu.x[19]));let b=((0xffffffffu64));let result=a & b;cpu.x[2]=(result) as u64;}
// 0x10de01f0 ldr x4, [x20, #0x270]
{let address=(cpu.x[20]).wrapping_add(0x270u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[4]=(v0) as u64;}
// 0x10de01f4 mov w3, #3
cpu.x[3]=(((0x3u64))) as u32 as u64;
// 0x10de01f8 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10de01fc ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10de0200 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10de0204 b #0x10e7e418
super::chunk9::f_10e7e418(cpu,rt,0x10e7e418)?;return Ok(());
}
0x10de0208=>{
// 0x10de0208 ldrb w21, [x20, #0x571]
{let address=(cpu.x[20]).wrapping_add(0x571u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[21]=(v0) as u32 as u64;}
// 0x10de020c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10de0210 bl #0x10e0b6b4
cpu.x[30]=0x10de0214;super::chunk8::f_10e0b6b4(cpu,rt,0x10e0b6b4)?;
pc=0x10de0214;continue;
}
0x10de0214=>{
// 0x10de0214 ldr w1, [x20, #0x7b0]
{let address=(cpu.x[20]).wrapping_add(0x7b0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10de0218 ldr x0, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10de021c ldr x4, [x20, #0x270]
{let address=(cpu.x[20]).wrapping_add(0x270u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[4]=(v0) as u64;}
// 0x10de0220 cbz w21, #0x10de023c
if (cpu.x[21] as u32 as u64) == 0 {pc=0x10de023c;continue;}pc=0x10de0224;continue;
}
0x10de0224=>{
// 0x10de0224 and x2, x19, #0xffffffff
{let a=((cpu.x[19]));let b=((0xffffffffu64));let result=a & b;cpu.x[2]=(result) as u64;}
// 0x10de0228 mov w3, #5
cpu.x[3]=(((0x5u64))) as u32 as u64;
// 0x10de022c ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10de0230 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10de0234 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10de0238 b #0x10e7e418
super::chunk9::f_10e7e418(cpu,rt,0x10e7e418)?;return Ok(());
}
0x10de023c=>{
// 0x10de023c and x2, x19, #0xffffffff
{let a=((cpu.x[19]));let b=((0xffffffffu64));let result=a & b;cpu.x[2]=(result) as u64;}
// 0x10de0240 mov w3, #1
cpu.x[3]=(((0x1u64))) as u32 as u64;
// 0x10de0244 bl #0x10e7e418
cpu.x[30]=0x10de0248;super::chunk9::f_10e7e418(cpu,rt,0x10e7e418)?;
pc=0x10de0248;continue;
}
0x10de0248=>{
// 0x10de0248 cmp w0, #0
{let a=((cpu.x[0] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10de024c b.le #0x10de0260
if cpu.condition(13){pc=0x10de0260;continue;}pc=0x10de0250;continue;
}
0x10de0250=>{
// 0x10de0250 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10de0254 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10de0258 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10de025c ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10de0260=>{
// 0x10de0260 ldr x8, [x20, #0xc0]
{let address=(cpu.x[20]).wrapping_add(0xc0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10de0264 cbz x8, #0x10de0250
if (cpu.x[8]) == 0 {pc=0x10de0250;continue;}pc=0x10de0268;continue;
}
0x10de0268=>{
// 0x10de0268 and x1, x19, #0xffffffff
{let a=((cpu.x[19]));let b=((0xffffffffu64));let result=a & b;cpu.x[1]=(result) as u64;}
// 0x10de026c mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10de0270 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10de0274 bl #0x10e5ae54
cpu.x[30]=0x10de0278;super::chunk9::f_10e5ae54(cpu,rt,0x10e5ae54)?;
pc=0x10de0278;continue;
}
0x10de0278=>{
// 0x10de0278 mov w8, w0
cpu.x[8]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10de027c mov w0, w21
cpu.x[0]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10de0280 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10de0284 b.lt #0x10de0250
if cpu.condition(11){pc=0x10de0250;continue;}pc=0x10de0288;continue;
}
0x10de0288=>{
// 0x10de0288 ldr x0, [x20, #0xc0]
{let address=(cpu.x[20]).wrapping_add(0xc0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10de028c and x1, x19, #0xffffffff
{let a=((cpu.x[19]));let b=((0xffffffffu64));let result=a & b;cpu.x[1]=(result) as u64;}
// 0x10de0290 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10de0294 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10de0298 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10de029c b #0x10e5ae54
super::chunk9::f_10e5ae54(cpu,rt,0x10e5ae54)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10de01bc")),}}}
#[inline]
pub(super) unsafe fn f_10de0aec(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10de0aec=>{
// 0x10de0aec ldr w0, [x0, #0x328]
{let address=(cpu.x[0]).wrapping_add(0x328u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10de0af0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10de0aec")),}}}
#[inline]
pub(super) unsafe fn f_10de0b1c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10de0b1c=>{
// 0x10de0b1c ldr w0, [x0, #0x6a4]
{let address=(cpu.x[0]).wrapping_add(0x6a4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10de0b20 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10de0b1c")),}}}
#[inline]
pub(super) unsafe fn f_10de1248(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10de1248=>{
// 0x10de1248 ldrb w0, [x0, #0x39a]
{let address=(cpu.x[0]).wrapping_add(0x39au64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10de124c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10de1248")),}}}
#[inline]
pub(super) unsafe fn f_10de1648(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10de1648=>{
// 0x10de1648 ldr x8, [x0, #0x710]
{let address=(cpu.x[0]).wrapping_add(0x710u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10de164c cbz x8, #0x10de1660
if (cpu.x[8]) == 0 {pc=0x10de1660;continue;}pc=0x10de1650;continue;
}
0x10de1650=>{
// 0x10de1650 ldrb w8, [x8, #0xac]
{let address=(cpu.x[8]).wrapping_add(0xacu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10de1654 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10de1658 cset w0, ne
cpu.x[0]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x10de165c ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10de1660=>{
// 0x10de1660 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10de1664 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10de1648")),}}}
#[inline]
pub(super) unsafe fn f_10e0b6b4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e0b6b4=>{
// 0x10e0b6b4 ldr x0, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10e0b6b8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e0b6b4")),}}}
#[inline]
pub(super) unsafe fn f_10e0bff8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e0bff8=>{
// 0x10e0bff8 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10e0bffc strb w8, [x0, #0x60]
{let address=(cpu.x[0]).wrapping_add(0x60u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
pc=0x10e0c000;continue;
}
0x10e0c000=>{
// 0x10e0c000 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e0bff8")),}}}
#[inline]
pub(super) unsafe fn f_10e130ec(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e130ec=>{
// 0x10e130ec stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10e130f0 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10e130f4 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10e130f8 cbz x1, #0x10e13128
if (cpu.x[1]) == 0 {pc=0x10e13128;continue;}pc=0x10e130fc;continue;
}
0x10e130fc=>{
// 0x10e130fc mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10e13100 mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x10e13104 mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10e13108 bl #0x10e0b614
cpu.x[30]=0x10e1310c;super::chunk0::f_10e0b614(cpu,rt,0x10e0b614)?;
pc=0x10e1310c;continue;
}
0x10e1310c=>{
// 0x10e1310c ldr w8, [x20, #0x84]
{let address=(cpu.x[20]).wrapping_add(0x84u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e13110 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e13114 b.ne #0x10e13128
if cpu.condition(1){pc=0x10e13128;continue;}pc=0x10e13118;continue;
}
0x10e13118=>{
// 0x10e13118 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10e1311c ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10e13120 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10e13124 b #0x10e0b668
super::chunk4::f_10e0b668(cpu,rt,0x10e0b668)?;return Ok(());
}
0x10e13128=>{
// 0x10e13128 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10e1312c ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10e13130 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10e13134 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e130ec")),}}}
#[inline]
pub(super) unsafe fn f_10e167b4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e167b4=>{
// 0x10e167b4 ldr x0, [x0, #0x800]
{let address=(cpu.x[0]).wrapping_add(0x800u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10e167b8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e167b4")),}}}
#[inline]
pub(super) unsafe fn f_10e19ce0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e19ce0=>{
// 0x10e19ce0 ldr x0, [x0, #0xe80]
{let address=(cpu.x[0]).wrapping_add(0xe80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10e19ce4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e19ce0")),}}}
#[inline]
pub(super) unsafe fn f_10e1b00c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e1b00c=>{
// 0x10e1b00c ldr x0, [x0, #0x7a8]
{let address=(cpu.x[0]).wrapping_add(0x7a8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10e1b010 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e1b00c")),}}}
#[inline]
pub(super) unsafe fn f_10e64aec(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e64aec=>{
// 0x10e64aec ldr w0, [x0, #0x230]
{let address=(cpu.x[0]).wrapping_add(0x230u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10e64af0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e64aec")),}}}
#[inline]
pub(super) unsafe fn f_10e7e934(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e7e934=>{
// 0x10e7e934 ldr w0, [x0, #0x13c]
{let address=(cpu.x[0]).wrapping_add(0x13cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10e7e938 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e7e934")),}}}
#[inline]
pub(super) unsafe fn f_10e8bcc0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e8bcc0=>{
// 0x10e8bcc0 ldr x8, [x0, #0x38]
{let address=(cpu.x[0]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e8bcc4 ldr x0, [x8, w1, sxtw #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[1] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10e8bcc8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e8bcc0")),}}}
#[inline]
pub(super) unsafe fn f_10eba79c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10eba79c=>{
// 0x10eba79c mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
// 0x10eba7a0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10eba79c")),}}}
#[inline]
pub(super) unsafe fn f_10eba7a4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10eba7a4=>{
// 0x10eba7a4 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10eba7a8 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10eba7ac stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10eba7b0 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10eba7b4 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10eba7b8 mov w0, #0x68
cpu.x[0]=(((0x68u64))) as u32 as u64;
// 0x10eba7bc mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10eba7c0 bl #0x1187d2b0
cpu.x[30]=0x10eba7c4;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
pc=0x10eba7c4;continue;
}
0x10eba7c4=>{
// 0x10eba7c4 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10eba7c8 mov x2, x20
cpu.x[2]=(((cpu.x[20]))) as u64;
// 0x10eba7cc mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10eba7d0 bl #0x10fb76d0
cpu.x[30]=0x10eba7d4;super::chunk6::f_10fb76d0(cpu,rt,0x10fb76d0)?;
pc=0x10eba7d4;continue;
}
0x10eba7d4=>{
// 0x10eba7d4 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10eba7d8 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10eba7dc ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10eba7e0 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10eba7e4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10eba7a4")),}}}
#[inline]
pub(super) unsafe fn f_10f07b70(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f07b70=>{
// 0x10f07b70 sub sp, sp, #0x40
{let a=((cpu.sp));let b=((0x40u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x10f07b74 stp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f07b78 stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f07b7c stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f07b80 add x29, sp, #0x10
{let a=((cpu.sp));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x10f07b84 ldp x21, x22, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[21]=(v0) as u64;cpu.x[22]=(v1) as u64;}
// 0x10f07b88 mov w0, #0x10
cpu.x[0]=(((0x10u64))) as u32 as u64;
// 0x10f07b8c mov x20, x8
cpu.x[20]=(((cpu.x[8]))) as u64;
// 0x10f07b90 bl #0x1187d2b0
cpu.x[30]=0x10f07b94;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
pc=0x10f07b94;continue;
}
0x10f07b94=>{
// 0x10f07b94 ubfiz x8, x22, #2, #0x20
cpu.x[8]=(((((cpu.x[22])) & 0xffffffffu64) << 2)) as u64;
// 0x10f07b98 cmp w22, #0
{let a=((cpu.x[22] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f07b9c mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f07ba0 str xzr, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f07ba4 str w22, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=cpu.x[22] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f07ba8 csinv x0, x8, xzr, ge
cpu.x[0]=(if cpu.condition(10){((cpu.x[8]))}else{!(((0u64)))}) as u64;
// 0x10f07bac bl #0x1187d520
cpu.x[30]=0x10f07bb0;super::chunk8::f_1187d520(cpu,rt,0x1187d520)?;
pc=0x10f07bb0;continue;
}
0x10f07bb0=>{
// 0x10f07bb0 str x19, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f07bb4 str x0, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f07bb8 cbz x21, #0x10f07c18
if (cpu.x[21]) == 0 {pc=0x10f07c18;continue;}pc=0x10f07bbc;continue;
}
0x10f07bbc=>{
// 0x10f07bbc mov w20, #5
cpu.x[20]=(((0x5u64))) as u32 as u64;
// 0x10f07bc0 b #0x10f07bd8
pc=0x10f07bd8;continue;
}
0x10f07bc4=>{
// 0x10f07bc4 ldr x21, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f07bc8 add w9, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10f07bcc str w9, [x19, #0xc]
{let address=(cpu.x[19]).wrapping_add(0xcu64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f07bd0 str w22, [x0, w8, sxtw #2]
{let address=((cpu.x[0]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=cpu.x[22] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f07bd4 cbz x21, #0x10f07c10
if (cpu.x[21]) == 0 {pc=0x10f07c10;continue;}pc=0x10f07bd8;continue;
}
0x10f07bd8=>{
// 0x10f07bd8 ldp w9, w8, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10f07bdc ldr w22, [x21, #0x10]
{let address=(cpu.x[21]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[22]=(v0) as u32 as u64;}
// 0x10f07be0 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f07be4 b.ne #0x10f07bc4
if cpu.condition(1){pc=0x10f07bc4;continue;}pc=0x10f07be8;continue;
}
0x10f07be8=>{
// 0x10f07be8 lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10f07bec cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f07bf0 csel w1, w20, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[20] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10f07bf4 cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f07bf8 b.ge #0x10f07bc4
if cpu.condition(10){pc=0x10f07bc4;continue;}pc=0x10f07bfc;continue;
}
0x10f07bfc=>{
// 0x10f07bfc mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f07c00 bl #0x1078f4d4
cpu.x[30]=0x10f07c04;super::chunk2::f_1078f4d4(cpu,rt,0x1078f4d4)?;
// 0x10f07c04 ldr w8, [x19, #0xc]
{let address=(cpu.x[19]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f07c08 ldr x0, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f07c0c b #0x10f07bc4
pc=0x10f07bc4;continue;
}
0x10f07c10=>{
// 0x10f07c10 ldrsw x8, [x19, #0xc]
{let address=(cpu.x[19]).wrapping_add(0xcu64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10f07c14 b #0x10f07c1c
pc=0x10f07c1c;continue;
}
0x10f07c18=>{
// 0x10f07c18 mov x8, xzr
cpu.x[8]=(((0u64))) as u64;
pc=0x10f07c1c;continue;
}
0x10f07c1c=>{
// 0x10f07c1c add x1, x0, x8, lsl #2
{let a=((cpu.x[0]));let b=(((cpu.x[8])) as u64).wrapping_shl(2) as u64;let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x10f07c20 sub x2, x29, #1
{let a=((cpu.x[29]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[2]=(result) as u64;}
// 0x10f07c24 bl #0x1187dc10
cpu.x[30]=0x10f07c28;super::chunk4::f_1187dc10(cpu,rt,0x1187dc10)?;
pc=0x10f07c28;continue;
}
0x10f07c28=>{
// 0x10f07c28 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f07c2c ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f07c30 ldp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10f07c34 add sp, sp, #0x40
{let a=((cpu.sp));let b=((0x40u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10f07c38 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f07b70")),}}}
#[inline]
pub(super) unsafe fn f_10f782c8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f782c8=>{
// 0x10f782c8 ldr w8, [x0, #0x5c]
{let address=(cpu.x[0]).wrapping_add(0x5cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f782cc cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f782d0 b.lt #0x10f7832c
if cpu.condition(11){pc=0x10f7832c;continue;}pc=0x10f782d4;continue;
}
0x10f782d4=>{
// 0x10f782d4 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f782d8 stp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f782dc stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f782e0 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f782e4 mov x19, x2
cpu.x[19]=(((cpu.x[2]))) as u64;
// 0x10f782e8 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10f782ec mov x21, x1
cpu.x[21]=(((cpu.x[1]))) as u64;
// 0x10f782f0 sub x22, x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[22]=(result) as u64;}
pc=0x10f782f4;continue;
}
0x10f782f4=>{
// 0x10f782f4 ldr x8, [x20, #0x50]
{let address=(cpu.x[20]).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f782f8 mov x1, x21
cpu.x[1]=(((cpu.x[21]))) as u64;
// 0x10f782fc mov x2, x19
cpu.x[2]=(((cpu.x[19]))) as u64;
// 0x10f78300 ldr x0, [x8, x22, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[22])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f78304 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f78308 ldr x8, [x8, #0xa0]
{let address=(cpu.x[8]).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7830c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f78310;super::dispatch(cpu,rt,target)?;}
pc=0x10f78310;continue;
}
0x10f78310=>{
// 0x10f78310 add x8, x22, #1
{let a=((cpu.x[22]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10f78314 sub x22, x22, #1
{let a=((cpu.x[22]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[22]=(result) as u64;}
// 0x10f78318 cmp x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f7831c b.hi #0x10f782f4
if cpu.condition(8){pc=0x10f782f4;continue;}pc=0x10f78320;continue;
}
0x10f78320=>{
// 0x10f78320 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f78324 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f78328 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
pc=0x10f7832c;continue;
}
0x10f7832c=>{
// 0x10f7832c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f782c8")),}}}
#[inline]
pub(super) unsafe fn f_10f7b3d4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f7b3d4=>{
// 0x10f7b3d4 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f7b3d8 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f7b3dc stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f7b3e0 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f7b3e4 ldr x21, [x0, #0x48]
{let address=(cpu.x[0]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f7b3e8 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10f7b3ec and x0, x1, #0xffffffff
{let a=((cpu.x[1]));let b=((0xffffffffu64));let result=a & b;cpu.x[0]=(result) as u64;}
// 0x10f7b3f0 mov x19, x2
cpu.x[19]=(((cpu.x[2]))) as u64;
// 0x10f7b3f4 ldr x8, [x21, #0x168]
{let address=(cpu.x[21]).wrapping_add(0x168u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7b3f8 mov x1, x8
cpu.x[1]=(((cpu.x[8]))) as u64;
// 0x10f7b3fc bl #0x10fa5c64
cpu.x[30]=0x10f7b400;super::chunk6::f_10fa5c64(cpu,rt,0x10fa5c64)?;
pc=0x10f7b400;continue;
}
0x10f7b400=>{
// 0x10f7b400 str w0, [x20, #0xfc]
{let address=(cpu.x[20]).wrapping_add(0xfcu64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f7b404 ldr w8, [x21, #0x148]
{let address=(cpu.x[21]).wrapping_add(0x148u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f7b408 str w8, [x20, #0x100]
{let address=(cpu.x[20]).wrapping_add(0x100u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f7b40c cbz x19, #0x10f7b420
if (cpu.x[19]) == 0 {pc=0x10f7b420;continue;}pc=0x10f7b410;continue;
}
0x10f7b410=>{
// 0x10f7b410 ldr w8, [x21, #0x1a0]
{let address=(cpu.x[21]).wrapping_add(0x1a0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f7b414 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7b418 b.ne #0x10f7b420
if cpu.condition(1){pc=0x10f7b420;continue;}pc=0x10f7b41c;continue;
}
0x10f7b41c=>{
// 0x10f7b41c str x19, [x20, #0x138]
{let address=(cpu.x[20]).wrapping_add(0x138u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10f7b420;continue;
}
0x10f7b420=>{
// 0x10f7b420 str x19, [x20, #0x130]
{let address=(cpu.x[20]).wrapping_add(0x130u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f7b424 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f7b428 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f7b42c ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f7b430 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f7b3d4")),}}}
#[inline]
pub(super) unsafe fn f_10f7f2c8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f7f2c8=>{
// 0x10f7f2c8 sub sp, sp, #0x80
{let a=((cpu.sp));let b=((0x80u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x10f7f2cc stp x29, x30, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f7f2d0 stp x28, x27, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[28];let v1=cpu.x[27];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f7f2d4 stp x26, x25, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[26];let v1=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f7f2d8 stp x24, x23, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f7f2dc stp x22, x21, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f7f2e0 stp x20, x19, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f7f2e4 add x29, sp, #0x20
{let a=((cpu.sp));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x10f7f2e8 ldr w21, [x0, #0x11c]
{let address=(cpu.x[0]).wrapping_add(0x11cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[21]=(v0) as u32 as u64;}
// 0x10f7f2ec cmp w21, w1
{let a=((cpu.x[21] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7f2f0 b.eq #0x10f7fdb4
if cpu.condition(0){pc=0x10f7fdb4;continue;}pc=0x10f7f2f4;continue;
}
0x10f7f2f4=>{
// 0x10f7f2f4 ldr w8, [x0, #0x15c]
{let address=(cpu.x[0]).wrapping_add(0x15cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f7f2f8 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f7f2fc mov w23, w1
cpu.x[23]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x10f7f300 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7f304 b.lt #0x10f7f324
if cpu.condition(11){pc=0x10f7f324;continue;}pc=0x10f7f308;continue;
}
0x10f7f308=>{
// 0x10f7f308 cmp w23, #0xd
{let a=((cpu.x[23] as u32 as u64));let b=((0xdu64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7f30c b.hi #0x10f7fdb4
if cpu.condition(8){pc=0x10f7fdb4;continue;}pc=0x10f7f310;continue;
}
0x10f7f310=>{
// 0x10f7f310 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10f7f314 mov w9, #0x3100
cpu.x[9]=(((0x3100u64))) as u32 as u64;
// 0x10f7f318 lsl w8, w8, w23
cpu.x[8]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((cpu.x[23] as u32 as u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10f7f31c tst w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a & b;cpu.logical_flags(result,32);}
// 0x10f7f320 b.eq #0x10f7fdb4
if cpu.condition(0){pc=0x10f7fdb4;continue;}pc=0x10f7f324;continue;
}
0x10f7f324=>{
// 0x10f7f324 ldr x24, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[24]=(v0) as u64;}
// 0x10f7f328 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7f32c bl #0x10f9433c
cpu.x[30]=0x10f7f330;super::chunk3::f_10f9433c(cpu,rt,0x10f9433c)?;
pc=0x10f7f330;continue;
}
0x10f7f330=>{
// 0x10f7f330 ldr w8, [x19, #0x11c]
{let address=(cpu.x[19]).wrapping_add(0x11cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f7f334 ldr x20, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10f7f338 mov x22, x0
cpu.x[22]=(((cpu.x[0]))) as u64;
// 0x10f7f33c cmp w8, #7
{let a=((cpu.x[8] as u32 as u64));let b=((0x7u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7f340 b.gt #0x10f7f368
if cpu.condition(12){pc=0x10f7f368;continue;}pc=0x10f7f344;continue;
}
0x10f7f344=>{
// 0x10f7f344 cmp w8, #4
{let a=((cpu.x[8] as u32 as u64));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7f348 b.gt #0x10f7f3f0
if cpu.condition(12){pc=0x10f7f3f0;continue;}pc=0x10f7f34c;continue;
}
0x10f7f34c=>{
// 0x10f7f34c cmp w8, #2
{let a=((cpu.x[8] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7f350 b.eq #0x10f7f640
if cpu.condition(0){pc=0x10f7f640;continue;}pc=0x10f7f354;continue;
}
0x10f7f354=>{
// 0x10f7f354 cmp w8, #3
{let a=((cpu.x[8] as u32 as u64));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7f358 b.eq #0x10f7f68c
if cpu.condition(0){pc=0x10f7f68c;continue;}pc=0x10f7f35c;continue;
}
0x10f7f35c=>{
// 0x10f7f35c cmp w8, #4
{let a=((cpu.x[8] as u32 as u64));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7f360 b.eq #0x10f7f3fc
if cpu.condition(0){pc=0x10f7f3fc;continue;}pc=0x10f7f364;continue;
}
0x10f7f364=>{
// 0x10f7f364 b #0x10f7f9dc
pc=0x10f7f9dc;continue;
}
0x10f7f368=>{
// 0x10f7f368 sub w9, w8, #0xc
{let a=((cpu.x[8] as u32 as u64));let b=((0xcu64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10f7f36c cmp w9, #2
{let a=((cpu.x[9] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7f370 b.lo #0x10f7f434
if cpu.condition(3){pc=0x10f7f434;continue;}pc=0x10f7f374;continue;
}
0x10f7f374=>{
// 0x10f7f374 cmp w8, #8
{let a=((cpu.x[8] as u32 as u64));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7f378 b.eq #0x10f7f78c
if cpu.condition(0){pc=0x10f7f78c;continue;}pc=0x10f7f37c;continue;
}
0x10f7f37c=>{
// 0x10f7f37c cmp w8, #0xa
{let a=((cpu.x[8] as u32 as u64));let b=((0xau64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7f380 b.ne #0x10f7f9dc
if cpu.condition(1){pc=0x10f7f9dc;continue;}pc=0x10f7f384;continue;
}
0x10f7f384=>{
// 0x10f7f384 ldr w8, [x19, #0x1e4]
{let address=(cpu.x[19]).wrapping_add(0x1e4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f7f388 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7f38c b.lt #0x10f7f3d0
if cpu.condition(11){pc=0x10f7f3d0;continue;}pc=0x10f7f390;continue;
}
0x10f7f390=>{
// 0x10f7f390 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7f394 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7f398 ldr x8, [x8, #0x140]
{let address=(cpu.x[8]).wrapping_add(0x140u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7f39c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7f3a0;super::dispatch(cpu,rt,target)?;}
// 0x10f7f3a0 tbz w0, #0, #0x10f7f3b0
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f7f3b0;continue;}pc=0x10f7f3a4;continue;
}
0x10f7f3a4=>{
// 0x10f7f3a4 ldr x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f7f3a8 bl #0x10de1648
cpu.x[30]=0x10f7f3ac;super::chunk8::f_10de1648(cpu,rt,0x10de1648)?;
// 0x10f7f3ac tbnz w0, #0, #0x10f7f3d0
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10f7f3d0;continue;}pc=0x10f7f3b0;continue;
}
0x10f7f3b0=>{
// 0x10f7f3b0 adrp x8, #0x11980000
cpu.x[8]=(0x11980000u64) as u64;
// 0x10f7f3b4 mov w10, #1
cpu.x[10]=(((0x1u64))) as u32 as u64;
// 0x10f7f3b8 ldr x8, [x8, #0x580]
{let address=(cpu.x[8]).wrapping_add(0x580u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7f3bc ldr x9, [x19, #0xb8]
{let address=(cpu.x[19]).wrapping_add(0xb8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f7f3c0 strb w10, [x19, #0x1e0]
{let address=(cpu.x[19]).wrapping_add(0x1e0u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f7f3c4 ldr x8, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7f3c8 orr x8, x9, x8
{let a=((cpu.x[9]));let b=((cpu.x[8]));let result=a | b;cpu.x[8]=(result) as u64;}
// 0x10f7f3cc str x8, [x19, #0xb8]
{let address=(cpu.x[19]).wrapping_add(0xb8u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10f7f3d0;continue;
}
0x10f7f3d0=>{
// 0x10f7f3d0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7f3d4 bl #0x10f942c8
cpu.x[30]=0x10f7f3d8;super::chunk1::f_10f942c8(cpu,rt,0x10f942c8)?;
pc=0x10f7f3d8;continue;
}
0x10f7f3d8=>{
// 0x10f7f3d8 cbz x0, #0x10f7f9dc
if (cpu.x[0]) == 0 {pc=0x10f7f9dc;continue;}pc=0x10f7f3dc;continue;
}
0x10f7f3dc=>{
// 0x10f7f3dc bl #0x10fcc464
cpu.x[30]=0x10f7f3e0;super::chunk2::f_10fcc464(cpu,rt,0x10fcc464)?;
pc=0x10f7f3e0;continue;
}
0x10f7f3e0=>{
// 0x10f7f3e0 cmp w23, #0x10
{let a=((cpu.x[23] as u32 as u64));let b=((0x10u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7f3e4 str w23, [x19, #0x11c]
{let address=(cpu.x[19]).wrapping_add(0x11cu64);let v0=cpu.x[23] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f7f3e8 b.ls #0x10f7f9e8
if cpu.condition(9){pc=0x10f7f9e8;continue;}pc=0x10f7f3ec;continue;
}
0x10f7f3ec=>{
// 0x10f7f3ec b #0x10f7fd9c
pc=0x10f7fd9c;continue;
}
0x10f7f3f0=>{
// 0x10f7f3f0 sub w9, w8, #6
{let a=((cpu.x[8] as u32 as u64));let b=((0x6u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10f7f3f4 cmp w9, #2
{let a=((cpu.x[9] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7f3f8 b.hs #0x10f7f5fc
if cpu.condition(2){pc=0x10f7f5fc;continue;}pc=0x10f7f3fc;continue;
}
0x10f7f3fc=>{
// 0x10f7f3fc cmp w23, #8
{let a=((cpu.x[23] as u32 as u64));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
pc=0x10f7f400;continue;
}
0x10f7f400=>{
// 0x10f7f400 b.ne #0x10f7f4d0
if cpu.condition(1){pc=0x10f7f4d0;continue;}pc=0x10f7f404;continue;
}
0x10f7f404=>{
// 0x10f7f404 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7f408 bl #0x10de1240
cpu.x[30]=0x10f7f40c;super::chunk12::f_10de1240(cpu,rt,0x10de1240)?;
// 0x10f7f40c tbz w0, #0, #0x10f7f4f0
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f7f4f0;continue;}pc=0x10f7f410;continue;
}
0x10f7f410=>{
// 0x10f7f410 ldr x0, [x19, #0x58]
{let address=(cpu.x[19]).wrapping_add(0x58u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f7f414 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10f7f418 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7f41c ldr x8, [x8, #0x128]
{let address=(cpu.x[8]).wrapping_add(0x128u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7f420 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7f424;super::dispatch(cpu,rt,target)?;}
// 0x10f7f424 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7f428 bl #0x10de0720
cpu.x[30]=0x10f7f42c;super::chunk3::f_10de0720(cpu,rt,0x10de0720)?;
// 0x10f7f42c cbnz x0, #0x10f7f4fc
if (cpu.x[0]) != 0 {pc=0x10f7f4fc;continue;}pc=0x10f7f430;continue;
}
0x10f7f430=>{
// 0x10f7f430 b #0x10f7f5b4
pc=0x10f7f5b4;continue;
}
0x10f7f434=>{
// 0x10f7f434 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7f438 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10f7f43c mov w2, #1
cpu.x[2]=(((0x1u64))) as u32 as u64;
// 0x10f7f440 bl #0x10f89258
cpu.x[30]=0x10f7f444;super::chunk5::f_10f89258(cpu,rt,0x10f89258)?;
// 0x10f7f444 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7f448 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10f7f44c mov w2, #1
cpu.x[2]=(((0x1u64))) as u32 as u64;
// 0x10f7f450 bl #0x10f89258
cpu.x[30]=0x10f7f454;super::chunk5::f_10f89258(cpu,rt,0x10f89258)?;
// 0x10f7f454 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7f458 bl #0x10f942c8
cpu.x[30]=0x10f7f45c;super::chunk1::f_10f942c8(cpu,rt,0x10f942c8)?;
// 0x10f7f45c cbz x0, #0x10f7f4bc
if (cpu.x[0]) == 0 {pc=0x10f7f4bc;continue;}pc=0x10f7f460;continue;
}
0x10f7f460=>{
// 0x10f7f460 mov x24, x0
cpu.x[24]=(((cpu.x[0]))) as u64;
// 0x10f7f464 bl #0x10fce8f4
cpu.x[30]=0x10f7f468;super::chunk9::f_10fce8f4(cpu,rt,0x10fce8f4)?;
// 0x10f7f468 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10f7f46c bl #0x10fd0310
cpu.x[30]=0x10f7f470;super::dispatch(cpu,rt,0x10fd0310)?;
// 0x10f7f470 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10f7f474 bl #0x10fd02e4
cpu.x[30]=0x10f7f478;super::dispatch(cpu,rt,0x10fd02e4)?;
// 0x10f7f478 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10f7f47c bl #0x10fcc464
cpu.x[30]=0x10f7f480;super::chunk2::f_10fcc464(cpu,rt,0x10fcc464)?;
// 0x10f7f480 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10f7f484 bl #0x10fcfabc
cpu.x[30]=0x10f7f488;super::dispatch(cpu,rt,0x10fcfabc)?;
// 0x10f7f488 cbz x22, #0x10f7f4bc
if (cpu.x[22]) == 0 {pc=0x10f7f4bc;continue;}pc=0x10f7f48c;continue;
}
0x10f7f48c=>{
// 0x10f7f48c mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f7f490 bl #0x10fc3ca0
cpu.x[30]=0x10f7f494;super::chunk14::f_10fc3ca0(cpu,rt,0x10fc3ca0)?;
// 0x10f7f494 cbz x0, #0x10f7f4bc
if (cpu.x[0]) == 0 {pc=0x10f7f4bc;continue;}pc=0x10f7f498;continue;
}
0x10f7f498=>{
// 0x10f7f498 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f7f49c bl #0x10fc3ca0
cpu.x[30]=0x10f7f4a0;super::chunk14::f_10fc3ca0(cpu,rt,0x10fc3ca0)?;
// 0x10f7f4a0 mov x1, x0
cpu.x[1]=(((cpu.x[0]))) as u64;
// 0x10f7f4a4 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f7f4a8 mov w2, wzr
cpu.x[2]=(((0u64))) as u32 as u64;
// 0x10f7f4ac bl #0x10fc3f74
cpu.x[30]=0x10f7f4b0;super::chunk15::f_10fc3f74(cpu,rt,0x10fc3f74)?;
// 0x10f7f4b0 tbnz w0, #0, #0x10f7f4bc
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10f7f4bc;continue;}pc=0x10f7f4b4;continue;
}
0x10f7f4b4=>{
// 0x10f7f4b4 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f7f4b8 bl #0x10fc9498
cpu.x[30]=0x10f7f4bc;super::dispatch(cpu,rt,0x10fc9498)?;
pc=0x10f7f4bc;continue;
}
0x10f7f4bc=>{
// 0x10f7f4bc cmp w23, #8
{let a=((cpu.x[23] as u32 as u64));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7f4c0 b.ne #0x10f7f718
if cpu.condition(1){pc=0x10f7f718;continue;}pc=0x10f7f4c4;continue;
}
0x10f7f4c4=>{
// 0x10f7f4c4 mov w8, #8
cpu.x[8]=(((0x8u64))) as u32 as u64;
// 0x10f7f4c8 str w8, [x19, #0x11c]
{let address=(cpu.x[19]).wrapping_add(0x11cu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f7f4cc b #0x10f7fa28
pc=0x10f7fa28;continue;
}
0x10f7f4d0=>{
// 0x10f7f4d0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7f4d4 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10f7f4d8 mov w2, #1
cpu.x[2]=(((0x1u64))) as u32 as u64;
// 0x10f7f4dc bl #0x10f89258
cpu.x[30]=0x10f7f4e0;super::chunk5::f_10f89258(cpu,rt,0x10f89258)?;
pc=0x10f7f4e0;continue;
}
0x10f7f4e0=>{
// 0x10f7f4e0 str wzr, [x19, #0x15c]
{let address=(cpu.x[19]).wrapping_add(0x15cu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f7f4e4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7f4e8 bl #0x10de1240
cpu.x[30]=0x10f7f4ec;super::chunk12::f_10de1240(cpu,rt,0x10de1240)?;
pc=0x10f7f4ec;continue;
}
0x10f7f4ec=>{
// 0x10f7f4ec tbnz w0, #0, #0x10f7f410
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10f7f410;continue;}pc=0x10f7f4f0;continue;
}
0x10f7f4f0=>{
// 0x10f7f4f0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7f4f4 bl #0x10de0720
cpu.x[30]=0x10f7f4f8;super::chunk3::f_10de0720(cpu,rt,0x10de0720)?;
pc=0x10f7f4f8;continue;
}
0x10f7f4f8=>{
// 0x10f7f4f8 cbz x0, #0x10f7f5b4
if (cpu.x[0]) == 0 {pc=0x10f7f5b4;continue;}pc=0x10f7f4fc;continue;
}
0x10f7f4fc=>{
// 0x10f7f4fc mov x25, x0
cpu.x[25]=(((cpu.x[0]))) as u64;
// 0x10f7f500 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7f504 bl #0x10de06f0
cpu.x[30]=0x10f7f508;super::chunk7::f_10de06f0(cpu,rt,0x10de06f0)?;
pc=0x10f7f508;continue;
}
0x10f7f508=>{
// 0x10f7f508 cbnz w0, #0x10f7f5b4
if (cpu.x[0] as u32 as u64) != 0 {pc=0x10f7f5b4;continue;}pc=0x10f7f50c;continue;
}
0x10f7f50c=>{
// 0x10f7f50c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7f510 bl #0x10de06e8
cpu.x[30]=0x10f7f514;super::chunk12::f_10de06e8(cpu,rt,0x10de06e8)?;
// 0x10f7f514 cbnz w0, #0x10f7f5b4
if (cpu.x[0] as u32 as u64) != 0 {pc=0x10f7f5b4;continue;}pc=0x10f7f518;continue;
}
0x10f7f518=>{
// 0x10f7f518 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7f51c bl #0x10de0710
cpu.x[30]=0x10f7f520;super::chunk15::f_10de0710(cpu,rt,0x10de0710)?;
// 0x10f7f520 tbnz w0, #0, #0x10f7f5b4
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10f7f5b4;continue;}pc=0x10f7f524;continue;
}
0x10f7f524=>{
// 0x10f7f524 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7f528 bl #0x10f93f94
cpu.x[30]=0x10f7f52c;super::chunk3::f_10f93f94(cpu,rt,0x10f93f94)?;
// 0x10f7f52c ldr x24, [x0, #0xe0]
{let address=(cpu.x[0]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[24]=(v0) as u64;}
// 0x10f7f530 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10f7f534 bl #0x10f8a060
cpu.x[30]=0x10f7f538;super::chunk5::f_10f8a060(cpu,rt,0x10f8a060)?;
// 0x10f7f538 mov x1, x25
cpu.x[1]=(((cpu.x[25]))) as u64;
// 0x10f7f53c bl #0x10f870a4
cpu.x[30]=0x10f7f540;super::chunk10::f_10f870a4(cpu,rt,0x10f870a4)?;
// 0x10f7f540 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f7f544 bl #0x10f870b8
cpu.x[30]=0x10f7f548;super::chunk7::f_10f870b8(cpu,rt,0x10f870b8)?;
// 0x10f7f548 mov x25, x0
cpu.x[25]=(((cpu.x[0]))) as u64;
// 0x10f7f54c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7f550 bl #0x10f94298
cpu.x[30]=0x10f7f554;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
// 0x10f7f554 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f7f558 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7f55c bl #0x10f942a0
cpu.x[30]=0x10f7f560;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
// 0x10f7f560 mov w27, w0
cpu.x[27]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f7f564 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7f568 bl #0x10f942a8
cpu.x[30]=0x10f7f56c;super::chunk11::f_10f942a8(cpu,rt,0x10f942a8)?;
// 0x10f7f56c mov w3, w0
cpu.x[3]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f7f570 mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10f7f574 mov w1, w26
cpu.x[1]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10f7f578 mov w2, w27
cpu.x[2]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10f7f57c bl #0x10f870ac
cpu.x[30]=0x10f7f580;super::chunk2::f_10f870ac(cpu,rt,0x10f870ac)?;
// 0x10f7f580 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10f7f584 bl #0x10f870c0
cpu.x[30]=0x10f7f588;super::chunk12::f_10f870c0(cpu,rt,0x10f870c0)?;
// 0x10f7f588 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10f7f58c bl #0x10f87178
cpu.x[30]=0x10f7f590;super::chunk7::f_10f87178(cpu,rt,0x10f87178)?;
// 0x10f7f590 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10f7f594 bl #0x10f87118
cpu.x[30]=0x10f7f598;super::chunk15::f_10f87118(cpu,rt,0x10f87118)?;
// 0x10f7f598 mov x25, x0
cpu.x[25]=(((cpu.x[0]))) as u64;
// 0x10f7f59c mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10f7f5a0 mov x1, x25
cpu.x[1]=(((cpu.x[25]))) as u64;
// 0x10f7f5a4 bl #0x10f8aac8
cpu.x[30]=0x10f7f5a8;super::chunk4::f_10f8aac8(cpu,rt,0x10f8aac8)?;
// 0x10f7f5a8 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10f7f5ac mov x1, x25
cpu.x[1]=(((cpu.x[25]))) as u64;
// 0x10f7f5b0 bl #0x10f8a0cc
cpu.x[30]=0x10f7f5b4;super::chunk0::f_10f8a0cc(cpu,rt,0x10f8a0cc)?;
pc=0x10f7f5b4;continue;
}
0x10f7f5b4=>{
// 0x10f7f5b4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7f5b8 bl #0x10de1248
cpu.x[30]=0x10f7f5bc;super::chunk8::f_10de1248(cpu,rt,0x10de1248)?;
pc=0x10f7f5bc;continue;
}
0x10f7f5bc=>{
// 0x10f7f5bc tbz w0, #0, #0x10f7f9dc
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f7f9dc;continue;}pc=0x10f7f5c0;continue;
}
0x10f7f5c0=>{
// 0x10f7f5c0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7f5c4 bl #0x10de1278
cpu.x[30]=0x10f7f5c8;super::chunk1::f_10de1278(cpu,rt,0x10de1278)?;
// 0x10f7f5c8 str w0, [x19, #0x138]
{let address=(cpu.x[19]).wrapping_add(0x138u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f7f5cc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7f5d0 bl #0x10ddff74
cpu.x[30]=0x10f7f5d4;super::chunk11::f_10ddff74(cpu,rt,0x10ddff74)?;
// 0x10f7f5d4 tbz w0, #0, #0x10f7f9dc
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f7f9dc;continue;}pc=0x10f7f5d8;continue;
}
0x10f7f5d8=>{
// 0x10f7f5d8 ldr x0, [x19, #0x58]
{let address=(cpu.x[19]).wrapping_add(0x58u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f7f5dc ldr x8, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7f5e0 ldr x9, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f7f5e4 ldr x1, [x8, #0x3f8]
{let address=(cpu.x[8]).wrapping_add(0x3f8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10f7f5e8 ldr x8, [x9, #0x80]
{let address=(cpu.x[9]).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7f5ec blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7f5f0;super::dispatch(cpu,rt,target)?;}
// 0x10f7f5f0 ldr x8, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7f5f4 ldr x2, [x8, #0x1b8]
{let address=(cpu.x[8]).wrapping_add(0x1b8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10f7f5f8 b #0x10f7f9d0
pc=0x10f7f9d0;continue;
}
0x10f7f5fc=>{
// 0x10f7f5fc cmp w8, #5
{let a=((cpu.x[8] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7f600 b.ne #0x10f7f9dc
if cpu.condition(1){pc=0x10f7f9dc;continue;}pc=0x10f7f604;continue;
}
0x10f7f604=>{
// 0x10f7f604 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7f608 bl #0x10de066c
cpu.x[30]=0x10f7f60c;super::chunk4::f_10de066c(cpu,rt,0x10de066c)?;
pc=0x10f7f60c;continue;
}
0x10f7f60c=>{
// 0x10f7f60c cbz x0, #0x10f7f9dc
if (cpu.x[0]) == 0 {pc=0x10f7f9dc;continue;}pc=0x10f7f610;continue;
}
0x10f7f610=>{
// 0x10f7f610 ldr x24, [x19, #0x58]
{let address=(cpu.x[19]).wrapping_add(0x58u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[24]=(v0) as u64;}
// 0x10f7f614 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7f618 bl #0x10de066c
cpu.x[30]=0x10f7f61c;super::chunk4::f_10de066c(cpu,rt,0x10de066c)?;
pc=0x10f7f61c;continue;
}
0x10f7f61c=>{
// 0x10f7f61c ldr x8, [x24]
{let address=(cpu.x[24]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7f620 mov x1, x0
cpu.x[1]=(((cpu.x[0]))) as u64;
// 0x10f7f624 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10f7f628 ldr x8, [x8, #0x80]
{let address=(cpu.x[8]).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7f62c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7f630;super::dispatch(cpu,rt,target)?;}
pc=0x10f7f630;continue;
}
0x10f7f630=>{
// 0x10f7f630 cmp w23, #0x10
{let a=((cpu.x[23] as u32 as u64));let b=((0x10u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7f634 str w23, [x19, #0x11c]
{let address=(cpu.x[19]).wrapping_add(0x11cu64);let v0=cpu.x[23] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f7f638 b.ls #0x10f7f9e8
if cpu.condition(9){pc=0x10f7f9e8;continue;}pc=0x10f7f63c;continue;
}
0x10f7f63c=>{
// 0x10f7f63c b #0x10f7fd9c
pc=0x10f7fd9c;continue;
}
0x10f7f640=>{
// 0x10f7f640 cbz x22, #0x10f7f64c
if (cpu.x[22]) == 0 {pc=0x10f7f64c;continue;}pc=0x10f7f644;continue;
}
0x10f7f644=>{
// 0x10f7f644 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f7f648 bl #0x10fc952c
cpu.x[30]=0x10f7f64c;super::chunk6::f_10fc952c(cpu,rt,0x10fc952c)?;
pc=0x10f7f64c;continue;
}
0x10f7f64c=>{
// 0x10f7f64c ldr x8, [x20, #0x298]
{let address=(cpu.x[20]).wrapping_add(0x298u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7f650 cbz x8, #0x10f7f65c
if (cpu.x[8]) == 0 {pc=0x10f7f65c;continue;}pc=0x10f7f654;continue;
}
0x10f7f654=>{
// 0x10f7f654 ldr w8, [x20, #0x5a8]
{let address=(cpu.x[20]).wrapping_add(0x5a8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f7f658 str w8, [x19, #0x1ac]
{let address=(cpu.x[19]).wrapping_add(0x1acu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10f7f65c;continue;
}
0x10f7f65c=>{
// 0x10f7f65c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7f660 bl #0x10de1240
cpu.x[30]=0x10f7f664;super::chunk12::f_10de1240(cpu,rt,0x10de1240)?;
pc=0x10f7f664;continue;
}
0x10f7f664=>{
// 0x10f7f664 tbz w0, #0, #0x10f7f9dc
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f7f9dc;continue;}pc=0x10f7f668;continue;
}
0x10f7f668=>{
// 0x10f7f668 ldr x0, [x19, #0x58]
{let address=(cpu.x[19]).wrapping_add(0x58u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f7f66c mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10f7f670 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7f674 ldr x8, [x8, #0x128]
{let address=(cpu.x[8]).wrapping_add(0x128u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7f678 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7f67c;super::dispatch(cpu,rt,target)?;}
// 0x10f7f67c cmp w23, #0x10
{let a=((cpu.x[23] as u32 as u64));let b=((0x10u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7f680 str w23, [x19, #0x11c]
{let address=(cpu.x[19]).wrapping_add(0x11cu64);let v0=cpu.x[23] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f7f684 b.ls #0x10f7f9e8
if cpu.condition(9){pc=0x10f7f9e8;continue;}pc=0x10f7f688;continue;
}
0x10f7f688=>{
// 0x10f7f688 b #0x10f7fd9c
pc=0x10f7fd9c;continue;
}
0x10f7f68c=>{
// 0x10f7f68c cmp w23, #1
{let a=((cpu.x[23] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7f690 b.ne #0x10f7f9c0
if cpu.condition(1){pc=0x10f7f9c0;continue;}pc=0x10f7f694;continue;
}
0x10f7f694=>{
// 0x10f7f694 cbz x22, #0x10f7f9c0
if (cpu.x[22]) == 0 {pc=0x10f7f9c0;continue;}pc=0x10f7f698;continue;
}
0x10f7f698=>{
// 0x10f7f698 ldr w8, [x22, #0x78]
{let address=(cpu.x[22]).wrapping_add(0x78u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f7f69c cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7f6a0 b.lt #0x10f7f9c4
if cpu.condition(11){pc=0x10f7f9c4;continue;}pc=0x10f7f6a4;continue;
}
0x10f7f6a4=>{
// 0x10f7f6a4 ldr w9, [x20, #0x738]
{let address=(cpu.x[20]).wrapping_add(0x738u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f7f6a8 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7f6ac b.ge #0x10f7f9c4
if cpu.condition(10){pc=0x10f7f9c4;continue;}pc=0x10f7f6b0;continue;
}
0x10f7f6b0=>{
// 0x10f7f6b0 ldr w8, [x20, #0x73c]
{let address=(cpu.x[20]).wrapping_add(0x73cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f7f6b4 ldr w9, [x20, #0x484]
{let address=(cpu.x[20]).wrapping_add(0x484u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f7f6b8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7f6bc ldr w28, [x20, #0x488]
{let address=(cpu.x[20]).wrapping_add(0x488u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[28]=(v0) as u32 as u64;}
// 0x10f7f6c0 stp xzr, xzr, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f7f6c4 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7f6c8 csel w25, w8, w9, gt
cpu.x[25]=(if cpu.condition(12){((cpu.x[8] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10f7f6cc bl #0x10f94298
cpu.x[30]=0x10f7f6d0;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
// 0x10f7f6d0 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f7f6d4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7f6d8 bl #0x10f942a0
cpu.x[30]=0x10f7f6dc;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
// 0x10f7f6dc mov w2, w0
cpu.x[2]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f7f6e0 add x4, sp, #0x10
{let a=((cpu.sp));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[4]=(result) as u64;}
// 0x10f7f6e4 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10f7f6e8 mov w1, w26
cpu.x[1]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10f7f6ec mov w3, w25
cpu.x[3]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10f7f6f0 bl #0x10f98a88
cpu.x[30]=0x10f7f6f4;super::dispatch(cpu,rt,0x10f98a88)?;
// 0x10f7f6f4 ldr w8, [sp, #0x1c]
{let address=(cpu.sp).wrapping_add(0x1cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f7f6f8 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7f6fc b.lt #0x10f7f960
if cpu.condition(11){pc=0x10f7f960;continue;}pc=0x10f7f700;continue;
}
0x10f7f700=>{
// 0x10f7f700 mul w8, w28, w28
cpu.x[8]=((((cpu.x[28] as u32 as u64))).wrapping_mul(((cpu.x[28] as u32 as u64)))) as u32 as u64;
// 0x10f7f704 mov x27, xzr
cpu.x[27]=(((0u64))) as u64;
// 0x10f7f708 mov w24, #0x7fffffff
cpu.x[24]=(((0x7fffffffu64))) as u32 as u64;
// 0x10f7f70c str xzr, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f7f710 str w8, [sp, #4]
{let address=(cpu.sp).wrapping_add(0x4u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f7f714 b #0x10f7f86c
pc=0x10f7f86c;continue;
}
0x10f7f718=>{
// 0x10f7f718 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7f71c bl #0x10f93d2c
cpu.x[30]=0x10f7f720;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
// 0x10f7f720 ldr x24, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[24]=(v0) as u64;}
// 0x10f7f724 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7f728 bl #0x10f94298
cpu.x[30]=0x10f7f72c;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
// 0x10f7f72c mov w25, w0
cpu.x[25]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f7f730 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7f734 bl #0x10f942a0
cpu.x[30]=0x10f7f738;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
// 0x10f7f738 mov w2, w0
cpu.x[2]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f7f73c mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10f7f740 mov w1, w25
cpu.x[1]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10f7f744 bl #0x111d96c8
cpu.x[30]=0x10f7f748;super::chunk7::f_111d96c8(cpu,rt,0x111d96c8)?;
// 0x10f7f748 tbz w0, #0, #0x10f7f9dc
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f7f9dc;continue;}pc=0x10f7f74c;continue;
}
0x10f7f74c=>{
// 0x10f7f74c ldr x0, [x19, #0x188]
{let address=(cpu.x[19]).wrapping_add(0x188u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f7f750 cbz x0, #0x10f7f7b8
if (cpu.x[0]) == 0 {pc=0x10f7f7b8;continue;}pc=0x10f7f754;continue;
}
0x10f7f754=>{
// 0x10f7f754 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7f758 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7f75c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7f760;super::dispatch(cpu,rt,target)?;}
// 0x10f7f760 cmp w0, #4
{let a=((cpu.x[0] as u32 as u64));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7f764 b.ne #0x10f7f7b8
if cpu.condition(1){pc=0x10f7f7b8;continue;}pc=0x10f7f768;continue;
}
0x10f7f768=>{
// 0x10f7f768 ldr x0, [x19, #0x188]
{let address=(cpu.x[19]).wrapping_add(0x188u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f7f76c bl #0x10f94058
cpu.x[30]=0x10f7f770;super::chunk6::f_10f94058(cpu,rt,0x10f94058)?;
// 0x10f7f770 tbnz w0, #0, #0x10f7f7b8
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10f7f7b8;continue;}pc=0x10f7f774;continue;
}
0x10f7f774=>{
// 0x10f7f774 ldr x8, [x19, #0x188]
{let address=(cpu.x[19]).wrapping_add(0x188u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7f778 ldr x0, [x8, #0x100]
{let address=(cpu.x[8]).wrapping_add(0x100u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f7f77c cbz x0, #0x10f7f7b8
if (cpu.x[0]) == 0 {pc=0x10f7f7b8;continue;}pc=0x10f7f780;continue;
}
0x10f7f780=>{
// 0x10f7f780 bl #0x10f942a0
cpu.x[30]=0x10f7f784;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
// 0x10f7f784 mov w24, w0
cpu.x[24]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f7f788 b #0x10f7f7bc
pc=0x10f7f7bc;continue;
}
0x10f7f78c=>{
// 0x10f7f78c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7f790 bl #0x10f942c8
cpu.x[30]=0x10f7f794;super::chunk1::f_10f942c8(cpu,rt,0x10f942c8)?;
// 0x10f7f794 cbz x0, #0x10f7f9dc
if (cpu.x[0]) == 0 {pc=0x10f7f9dc;continue;}pc=0x10f7f798;continue;
}
0x10f7f798=>{
// 0x10f7f798 mov x24, x0
cpu.x[24]=(((cpu.x[0]))) as u64;
// 0x10f7f79c bl #0x10fce8f4
cpu.x[30]=0x10f7f7a0;super::chunk9::f_10fce8f4(cpu,rt,0x10fce8f4)?;
// 0x10f7f7a0 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10f7f7a4 bl #0x10fd0310
cpu.x[30]=0x10f7f7a8;super::dispatch(cpu,rt,0x10fd0310)?;
// 0x10f7f7a8 cmp w23, #0x10
{let a=((cpu.x[23] as u32 as u64));let b=((0x10u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7f7ac str w23, [x19, #0x11c]
{let address=(cpu.x[19]).wrapping_add(0x11cu64);let v0=cpu.x[23] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f7f7b0 b.ls #0x10f7f9e8
if cpu.condition(9){pc=0x10f7f9e8;continue;}pc=0x10f7f7b4;continue;
}
0x10f7f7b4=>{
// 0x10f7f7b4 b #0x10f7fd9c
pc=0x10f7fd9c;continue;
}
0x10f7f7b8=>{
// 0x10f7f7b8 mov w24, #-1
cpu.x[24]=(((0xffffffffffffffffu64))) as u32 as u64;
pc=0x10f7f7bc;continue;
}
0x10f7f7bc=>{
// 0x10f7f7bc mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7f7c0 bl #0x10f93d2c
cpu.x[30]=0x10f7f7c4;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
// 0x10f7f7c4 mov x25, x0
cpu.x[25]=(((cpu.x[0]))) as u64;
// 0x10f7f7c8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7f7cc bl #0x10f94298
cpu.x[30]=0x10f7f7d0;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
// 0x10f7f7d0 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f7f7d4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7f7d8 bl #0x10f942a0
cpu.x[30]=0x10f7f7dc;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
// 0x10f7f7dc mov w2, w0
cpu.x[2]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f7f7e0 mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10f7f7e4 mov w1, w26
cpu.x[1]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10f7f7e8 mov w3, w24
cpu.x[3]=(((cpu.x[24] as u32 as u64))) as u32 as u64;
// 0x10f7f7ec bl #0x10d392c8
cpu.x[30]=0x10f7f7f0;super::chunk4::f_10d392c8(cpu,rt,0x10d392c8)?;
// 0x10f7f7f0 mov w24, w0
cpu.x[24]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f7f7f4 asr w25, w0, #0x10
cpu.x[25]=(((((cpu.x[0] as u32 as u64))) as i32).wrapping_shr(((((0x10u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10f7f7f8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7f7fc bl #0x10f942b8
cpu.x[30]=0x10f7f800;super::chunk4::f_10f942b8(cpu,rt,0x10f942b8)?;
// 0x10f7f800 mov w3, w0
cpu.x[3]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f7f804 and w1, w24, #0xffff
{let a=((cpu.x[24] as u32 as u64));let b=((0xffffu64));let result=a & b;cpu.x[1]=(result) as u32 as u64;}
// 0x10f7f808 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7f80c mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10f7f810 bl #0x10f94250
cpu.x[30]=0x10f7f814;super::chunk3::f_10f94250(cpu,rt,0x10f94250)?;
// 0x10f7f814 cmp w23, #0x10
{let a=((cpu.x[23] as u32 as u64));let b=((0x10u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7f818 str w23, [x19, #0x11c]
{let address=(cpu.x[19]).wrapping_add(0x11cu64);let v0=cpu.x[23] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f7f81c b.ls #0x10f7f9e8
if cpu.condition(9){pc=0x10f7f9e8;continue;}pc=0x10f7f820;continue;
}
0x10f7f820=>{
// 0x10f7f820 b #0x10f7fd9c
pc=0x10f7fd9c;continue;
}
0x10f7f824=>{
// 0x10f7f824 mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10f7f828 bl #0x10f94298
cpu.x[30]=0x10f7f82c;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
// 0x10f7f82c str w0, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f7f830 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7f834 bl #0x10f94298
cpu.x[30]=0x10f7f838;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
// 0x10f7f838 ldp w8, w10, [x22, #0x7c]
{let address=(cpu.x[22]).wrapping_add(0x7cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[8]=(v0) as u32 as u64;cpu.x[10]=(v1) as u32 as u64;}
// 0x10f7f83c ldr w9, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f7f840 sub w9, w9, w0
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10f7f844 mov w0, w28
cpu.x[0]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10f7f848 mul w8, w9, w8
cpu.x[8]=((((cpu.x[9] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64)))) as u32 as u64;
// 0x10f7f84c madd w8, w10, w26, w8
cpu.x[8]=((((cpu.x[8] as u32 as u64))).wrapping_add((((cpu.x[10] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64))))) as u32 as u64;
// 0x10f7f850 tbnz w8, #0x1f, #0x10f7f85c
if ((cpu.x[8] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x10f7f85c;continue;}pc=0x10f7f854;continue;
}
0x10f7f854=>{
// 0x10f7f854 mov w24, w0
cpu.x[24]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f7f858 str x25, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10f7f85c;continue;
}
0x10f7f85c=>{
// 0x10f7f85c ldrsw x8, [sp, #0x1c]
{let address=(cpu.sp).wrapping_add(0x1cu64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10f7f860 add x27, x27, #1
{let a=((cpu.x[27]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[27]=(result) as u64;}
// 0x10f7f864 cmp x27, x8
{let a=((cpu.x[27]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f7f868 b.ge #0x10f7f91c
if cpu.condition(10){pc=0x10f7f91c;continue;}pc=0x10f7f86c;continue;
}
0x10f7f86c=>{
// 0x10f7f86c ldr x8, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7f870 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f7f874 ldr x25, [x8, x27, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[27])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[25]=(v0) as u64;}
// 0x10f7f878 bl #0x10fc3ca0
cpu.x[30]=0x10f7f87c;super::chunk14::f_10fc3ca0(cpu,rt,0x10fc3ca0)?;
// 0x10f7f87c cmp x25, x0
{let a=((cpu.x[25]));let b=((cpu.x[0]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f7f880 b.eq #0x10f7f85c
if cpu.condition(0){pc=0x10f7f85c;continue;}pc=0x10f7f884;continue;
}
0x10f7f884=>{
// 0x10f7f884 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f7f888 mov x1, x25
cpu.x[1]=(((cpu.x[25]))) as u64;
// 0x10f7f88c mov w2, #1
cpu.x[2]=(((0x1u64))) as u32 as u64;
// 0x10f7f890 bl #0x10fc4008
cpu.x[30]=0x10f7f894;super::chunk5::f_10fc4008(cpu,rt,0x10fc4008)?;
// 0x10f7f894 tbz w0, #0, #0x10f7f85c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f7f85c;continue;}pc=0x10f7f898;continue;
}
0x10f7f898=>{
// 0x10f7f898 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7f89c mov x1, x25
cpu.x[1]=(((cpu.x[25]))) as u64;
// 0x10f7f8a0 bl #0x10f94f34
cpu.x[30]=0x10f7f8a4;super::chunk8::f_10f94f34(cpu,rt,0x10f94f34)?;
// 0x10f7f8a4 cmp w0, w24
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[24] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7f8a8 b.gt #0x10f7f85c
if cpu.condition(12){pc=0x10f7f85c;continue;}pc=0x10f7f8ac;continue;
}
0x10f7f8ac=>{
// 0x10f7f8ac ldr w8, [sp, #4]
{let address=(cpu.sp).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f7f8b0 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7f8b4 b.le #0x10f7f854
if cpu.condition(13){pc=0x10f7f854;continue;}pc=0x10f7f8b8;continue;
}
0x10f7f8b8=>{
// 0x10f7f8b8 mov w28, w0
cpu.x[28]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f7f8bc mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10f7f8c0 bl #0x10f942a0
cpu.x[30]=0x10f7f8c4;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
// 0x10f7f8c4 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f7f8c8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7f8cc bl #0x10f942a0
cpu.x[30]=0x10f7f8d0;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
// 0x10f7f8d0 ldr x8, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7f8d4 sub w26, w26, w0
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[26]=(result) as u32 as u64;}
// 0x10f7f8d8 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10f7f8dc bl #0x10e0b6b4
cpu.x[30]=0x10f7f8e0;super::chunk8::f_10e0b6b4(cpu,rt,0x10e0b6b4)?;
// 0x10f7f8e0 ldr x0, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f7f8e4 bl #0x10e16208
cpu.x[30]=0x10f7f8e8;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
// 0x10f7f8e8 ldrb w8, [x0, #0x757]
{let address=(cpu.x[0]).wrapping_add(0x757u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f7f8ec cbz w8, #0x10f7f824
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f7f824;continue;}pc=0x10f7f8f0;continue;
}
0x10f7f8f0=>{
// 0x10f7f8f0 ldp w8, w9, [x22, #0x7c]
{let address=(cpu.x[22]).wrapping_add(0x7cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[8]=(v0) as u32 as u64;cpu.x[9]=(v1) as u32 as u64;}
// 0x10f7f8f4 mov w0, w28
cpu.x[0]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10f7f8f8 orr w8, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10f7f8fc cbz w8, #0x10f7f854
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f7f854;continue;}pc=0x10f7f900;continue;
}
0x10f7f900=>{
// 0x10f7f900 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7f904 bl #0x10f945ac
cpu.x[30]=0x10f7f908;super::chunk10::f_10f945ac(cpu,rt,0x10f945ac)?;
// 0x10f7f908 mov w8, w0
cpu.x[8]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f7f90c mov w0, w28
cpu.x[0]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10f7f910 mul w8, w8, w26
cpu.x[8]=((((cpu.x[8] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10f7f914 tbz w8, #0x1f, #0x10f7f854
if ((cpu.x[8] as u32 as u64) & (1u64 << 31)) == 0 {pc=0x10f7f854;continue;}pc=0x10f7f918;continue;
}
0x10f7f918=>{
// 0x10f7f918 b #0x10f7f85c
pc=0x10f7f85c;continue;
}
0x10f7f91c=>{
// 0x10f7f91c ldr x24, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[24]=(v0) as u64;}
// 0x10f7f920 cbz x24, #0x10f7f9b0
if (cpu.x[24]) == 0 {pc=0x10f7f9b0;continue;}pc=0x10f7f924;continue;
}
0x10f7f924=>{
// 0x10f7f924 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f7f928 bl #0x10fc3ca0
cpu.x[30]=0x10f7f92c;super::chunk14::f_10fc3ca0(cpu,rt,0x10fc3ca0)?;
// 0x10f7f92c cbz x0, #0x10f7f968
if (cpu.x[0]) == 0 {pc=0x10f7f968;continue;}pc=0x10f7f930;continue;
}
0x10f7f930=>{
// 0x10f7f930 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f7f934 bl #0x10fc3ca0
cpu.x[30]=0x10f7f938;super::chunk14::f_10fc3ca0(cpu,rt,0x10fc3ca0)?;
// 0x10f7f938 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7f93c ldr x8, [x8, #0x178]
{let address=(cpu.x[8]).wrapping_add(0x178u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7f940 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7f944;super::dispatch(cpu,rt,target)?;}
// 0x10f7f944 tbz w0, #0, #0x10f7f968
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f7f968;continue;}pc=0x10f7f948;continue;
}
0x10f7f948=>{
// 0x10f7f948 ldr x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f7f94c bl #0x10e0b6b4
cpu.x[30]=0x10f7f950;super::chunk8::f_10e0b6b4(cpu,rt,0x10e0b6b4)?;
// 0x10f7f950 ldr x0, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f7f954 bl #0x10e16208
cpu.x[30]=0x10f7f958;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
// 0x10f7f958 ldrb w8, [x0, #0x6a5]
{let address=(cpu.x[0]).wrapping_add(0x6a5u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f7f95c cbz w8, #0x10f7f968
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f7f968;continue;}pc=0x10f7f960;continue;
}
0x10f7f960=>{
// 0x10f7f960 mov w24, wzr
cpu.x[24]=(((0u64))) as u32 as u64;
// 0x10f7f964 b #0x10f7f9b0
pc=0x10f7f9b0;continue;
}
0x10f7f968=>{
// 0x10f7f968 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f7f96c mov x1, x24
cpu.x[1]=(((cpu.x[24]))) as u64;
// 0x10f7f970 mov w2, wzr
cpu.x[2]=(((0u64))) as u32 as u64;
// 0x10f7f974 mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10f7f978 mov w4, wzr
cpu.x[4]=(((0u64))) as u32 as u64;
// 0x10f7f97c bl #0x10fc3cb4
cpu.x[30]=0x10f7f980;super::chunk12::f_10fc3cb4(cpu,rt,0x10fc3cb4)?;
// 0x10f7f980 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10f7f984 bl #0x10f94298
cpu.x[30]=0x10f7f988;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
// 0x10f7f988 mov w25, w0
cpu.x[25]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f7f98c mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10f7f990 bl #0x10f942a0
cpu.x[30]=0x10f7f994;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
// 0x10f7f994 mov w2, w0
cpu.x[2]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f7f998 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f7f99c mov w1, w25
cpu.x[1]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10f7f9a0 mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10f7f9a4 mov w4, #1
cpu.x[4]=(((0x1u64))) as u32 as u64;
// 0x10f7f9a8 mov w24, #1
cpu.x[24]=(((0x1u64))) as u32 as u64;
// 0x10f7f9ac bl #0x10fc7a78
cpu.x[30]=0x10f7f9b0;super::dispatch(cpu,rt,0x10fc7a78)?;
pc=0x10f7f9b0;continue;
}
0x10f7f9b0=>{
// 0x10f7f9b0 ldr x0, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f7f9b4 cbz x0, #0x10f7f9bc
if (cpu.x[0]) == 0 {pc=0x10f7f9bc;continue;}pc=0x10f7f9b8;continue;
}
0x10f7f9b8=>{
// 0x10f7f9b8 bl #0x1187d360
cpu.x[30]=0x10f7f9bc;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x10f7f9bc;continue;
}
0x10f7f9bc=>{
// 0x10f7f9bc tbnz w24, #0, #0x10f7fdb4
if ((cpu.x[24] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10f7fdb4;continue;}pc=0x10f7f9c0;continue;
}
0x10f7f9c0=>{
// 0x10f7f9c0 cbz x22, #0x10f7f9cc
if (cpu.x[22]) == 0 {pc=0x10f7f9cc;continue;}pc=0x10f7f9c4;continue;
}
0x10f7f9c4=>{
// 0x10f7f9c4 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f7f9c8 bl #0x10fc7a1c
cpu.x[30]=0x10f7f9cc;super::dispatch(cpu,rt,0x10fc7a1c)?;
pc=0x10f7f9cc;continue;
}
0x10f7f9cc=>{
// 0x10f7f9cc ldr x2, [x20, #0x728]
{let address=(cpu.x[20]).wrapping_add(0x728u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
pc=0x10f7f9d0;continue;
}
0x10f7f9d0=>{
// 0x10f7f9d0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7f9d4 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f7f9d8 bl #0x10f89720
cpu.x[30]=0x10f7f9dc;super::chunk13::f_10f89720(cpu,rt,0x10f89720)?;
pc=0x10f7f9dc;continue;
}
0x10f7f9dc=>{
// 0x10f7f9dc cmp w23, #0x10
{let a=((cpu.x[23] as u32 as u64));let b=((0x10u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7f9e0 str w23, [x19, #0x11c]
{let address=(cpu.x[19]).wrapping_add(0x11cu64);let v0=cpu.x[23] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f7f9e4 b.hi #0x10f7fd9c
if cpu.condition(8){pc=0x10f7fd9c;continue;}pc=0x10f7f9e8;continue;
}
0x10f7f9e8=>{
// 0x10f7f9e8 mov w8, w23
cpu.x[8]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10f7f9ec adrp x9, #0x1033f000
cpu.x[9]=(0x1033f000u64) as u64;
// 0x10f7f9f0 add x9, x9, #0x820
{let a=((cpu.x[9]));let b=((0x820u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
// 0x10f7f9f4 adr x10, #0x10f7fa04
cpu.x[10]=(0x10f7fa04u64) as u64;
// 0x10f7f9f8 ldrh w11, [x9, x8, lsl #1]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[8])) as u64).wrapping_shl(1) as u64);let v0=rt.memory.read::<u16>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
// 0x10f7f9fc add x10, x10, x11, lsl #2
{let a=((cpu.x[10]));let b=(((cpu.x[11])) as u64).wrapping_shl(2) as u64;let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x10f7fa00 br x10
{let target=cpu.x[10];if target>=0x10f7f2c8 && target<0x10f7fedc{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
0x10f7fa04=>{
// 0x10f7fa04 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7fa08 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7fa0c ldr x8, [x8, #0x1d0]
{let address=(cpu.x[8]).wrapping_add(0x1d0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7fa10 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7fa14;super::dispatch(cpu,rt,target)?;}
// 0x10f7fa14 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7fa18 bl #0x10f942c8
cpu.x[30]=0x10f7fa1c;super::chunk1::f_10f942c8(cpu,rt,0x10f942c8)?;
// 0x10f7fa1c cbz x0, #0x10f7fd9c
if (cpu.x[0]) == 0 {pc=0x10f7fd9c;continue;}pc=0x10f7fa20;continue;
}
0x10f7fa20=>{
// 0x10f7fa20 bl #0x10fcfabc
cpu.x[30]=0x10f7fa24;super::dispatch(cpu,rt,0x10fcfabc)?;
// 0x10f7fa24 b #0x10f7fd9c
pc=0x10f7fd9c;continue;
}
0x10f7fa28=>{
// 0x10f7fa28 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7fa2c mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10f7fa30 mov w2, #1
cpu.x[2]=(((0x1u64))) as u32 as u64;
// 0x10f7fa34 bl #0x10f89258
cpu.x[30]=0x10f7fa38;super::chunk5::f_10f89258(cpu,rt,0x10f89258)?;
// 0x10f7fa38 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7fa3c bl #0x10f942c8
cpu.x[30]=0x10f7fa40;super::chunk1::f_10f942c8(cpu,rt,0x10f942c8)?;
// 0x10f7fa40 cbz x0, #0x10f7fd9c
if (cpu.x[0]) == 0 {pc=0x10f7fd9c;continue;}pc=0x10f7fa44;continue;
}
0x10f7fa44=>{
// 0x10f7fa44 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10f7fa48 ldr x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f7fa4c bl #0x10e0b6b4
cpu.x[30]=0x10f7fa50;super::chunk8::f_10e0b6b4(cpu,rt,0x10e0b6b4)?;
// 0x10f7fa50 ldr x0, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f7fa54 bl #0x10e16208
cpu.x[30]=0x10f7fa58;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
// 0x10f7fa58 ldrb w8, [x0, #0x405]
{let address=(cpu.x[0]).wrapping_add(0x405u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f7fa5c cbz w8, #0x10f7fa68
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f7fa68;continue;}pc=0x10f7fa60;continue;
}
0x10f7fa60=>{
// 0x10f7fa60 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7fa64 bl #0x10fcc464
cpu.x[30]=0x10f7fa68;super::chunk2::f_10fcc464(cpu,rt,0x10fcc464)?;
pc=0x10f7fa68;continue;
}
0x10f7fa68=>{
// 0x10f7fa68 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7fa6c bl #0x10fce8f4
cpu.x[30]=0x10f7fa70;super::chunk9::f_10fce8f4(cpu,rt,0x10fce8f4)?;
// 0x10f7fa70 b #0x10f7fd9c
pc=0x10f7fd9c;continue;
}
0x10f7fa74=>{
// 0x10f7fa74 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7fa78 bl #0x10ddff74
cpu.x[30]=0x10f7fa7c;super::chunk11::f_10ddff74(cpu,rt,0x10ddff74)?;
// 0x10f7fa7c tbz w0, #0, #0x10f7fa88
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f7fa88;continue;}pc=0x10f7fa80;continue;
}
0x10f7fa80=>{
// 0x10f7fa80 str wzr, [x19, #0x11c]
{let address=(cpu.x[19]).wrapping_add(0x11cu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f7fa84 b #0x10f7fd9c
pc=0x10f7fd9c;continue;
}
0x10f7fa88=>{
// 0x10f7fa88 cbz x22, #0x10f7fb20
if (cpu.x[22]) == 0 {pc=0x10f7fb20;continue;}pc=0x10f7fa8c;continue;
}
0x10f7fa8c=>{
// 0x10f7fa8c mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f7fa90 bl #0x10fc3ca0
cpu.x[30]=0x10f7fa94;super::chunk14::f_10fc3ca0(cpu,rt,0x10fc3ca0)?;
// 0x10f7fa94 cbz x0, #0x10f7fb20
if (cpu.x[0]) == 0 {pc=0x10f7fb20;continue;}pc=0x10f7fa98;continue;
}
0x10f7fa98=>{
// 0x10f7fa98 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f7fa9c bl #0x10fc3ca0
cpu.x[30]=0x10f7faa0;super::chunk14::f_10fc3ca0(cpu,rt,0x10fc3ca0)?;
// 0x10f7faa0 mov x1, x0
cpu.x[1]=(((cpu.x[0]))) as u64;
// 0x10f7faa4 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f7faa8 mov w2, wzr
cpu.x[2]=(((0u64))) as u32 as u64;
// 0x10f7faac bl #0x10fc3f74
cpu.x[30]=0x10f7fab0;super::chunk15::f_10fc3f74(cpu,rt,0x10fc3f74)?;
// 0x10f7fab0 tbnz w0, #0, #0x10f7fb20
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10f7fb20;continue;}pc=0x10f7fab4;continue;
}
0x10f7fab4=>{
// 0x10f7fab4 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f7fab8 bl #0x10fc9498
cpu.x[30]=0x10f7fabc;super::dispatch(cpu,rt,0x10fc9498)?;
// 0x10f7fabc b #0x10f7fb20
pc=0x10f7fb20;continue;
}
0x10f7fac0=>{
// 0x10f7fac0 cmp w21, #0xe
{let a=((cpu.x[21] as u32 as u64));let b=((0xeu64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7fac4 b.ne #0x10f7fad8
if cpu.condition(1){pc=0x10f7fad8;continue;}pc=0x10f7fac8;continue;
}
0x10f7fac8=>{
// 0x10f7fac8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7facc mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10f7fad0 mov w2, #1
cpu.x[2]=(((0x1u64))) as u32 as u64;
// 0x10f7fad4 bl #0x10f89258
cpu.x[30]=0x10f7fad8;super::chunk5::f_10f89258(cpu,rt,0x10f89258)?;
pc=0x10f7fad8;continue;
}
0x10f7fad8=>{
// 0x10f7fad8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7fadc bl #0x10f942c8
cpu.x[30]=0x10f7fae0;super::chunk1::f_10f942c8(cpu,rt,0x10f942c8)?;
pc=0x10f7fae0;continue;
}
0x10f7fae0=>{
// 0x10f7fae0 cbz x0, #0x10f7fae8
if (cpu.x[0]) == 0 {pc=0x10f7fae8;continue;}pc=0x10f7fae4;continue;
}
0x10f7fae4=>{
// 0x10f7fae4 bl #0x10fce8f4
cpu.x[30]=0x10f7fae8;super::chunk9::f_10fce8f4(cpu,rt,0x10fce8f4)?;
pc=0x10f7fae8;continue;
}
0x10f7fae8=>{
// 0x10f7fae8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7faec bl #0x10de1240
cpu.x[30]=0x10f7faf0;super::chunk12::f_10de1240(cpu,rt,0x10de1240)?;
pc=0x10f7faf0;continue;
}
0x10f7faf0=>{
// 0x10f7faf0 tbz w0, #0, #0x10f7fd9c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f7fd9c;continue;}pc=0x10f7faf4;continue;
}
0x10f7faf4=>{
// 0x10f7faf4 ldr x0, [x19, #0x58]
{let address=(cpu.x[19]).wrapping_add(0x58u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f7faf8 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10f7fafc ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7fb00 ldr x8, [x8, #0x128]
{let address=(cpu.x[8]).wrapping_add(0x128u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7fb04 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7fb08;super::dispatch(cpu,rt,target)?;}
// 0x10f7fb08 b #0x10f7fd9c
pc=0x10f7fd9c;continue;
}
0x10f7fb0c=>{
// 0x10f7fb0c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7fb10 bl #0x10f93d2c
cpu.x[30]=0x10f7fb14;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
pc=0x10f7fb14;continue;
}
0x10f7fb14=>{
// 0x10f7fb14 ldr x0, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f7fb18 bl #0x110ca998
cpu.x[30]=0x10f7fb1c;super::chunk3::f_110ca998(cpu,rt,0x110ca998)?;
pc=0x10f7fb1c;continue;
}
0x10f7fb1c=>{
// 0x10f7fb1c str w0, [x19, #0x12c]
{let address=(cpu.x[19]).wrapping_add(0x12cu64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10f7fb20;continue;
}
0x10f7fb20=>{
// 0x10f7fb20 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7fb24 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10f7fb28 mov w2, wzr
cpu.x[2]=(((0u64))) as u32 as u64;
// 0x10f7fb2c bl #0x10f89258
cpu.x[30]=0x10f7fb30;super::chunk5::f_10f89258(cpu,rt,0x10f89258)?;
pc=0x10f7fb30;continue;
}
0x10f7fb30=>{
// 0x10f7fb30 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7fb34 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10f7fb38 mov w2, wzr
cpu.x[2]=(((0u64))) as u32 as u64;
// 0x10f7fb3c b #0x10f7fd98
pc=0x10f7fd98;continue;
}
0x10f7fb40=>{
// 0x10f7fb40 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7fb44 bl #0x10f942c8
cpu.x[30]=0x10f7fb48;super::chunk1::f_10f942c8(cpu,rt,0x10f942c8)?;
pc=0x10f7fb48;continue;
}
0x10f7fb48=>{
// 0x10f7fb48 cbz x0, #0x10f7fd9c
if (cpu.x[0]) == 0 {pc=0x10f7fd9c;continue;}pc=0x10f7fb4c;continue;
}
0x10f7fb4c=>{
// 0x10f7fb4c bl #0x10fce8f4
cpu.x[30]=0x10f7fb50;super::chunk9::f_10fce8f4(cpu,rt,0x10fce8f4)?;
pc=0x10f7fb50;continue;
}
0x10f7fb50=>{
// 0x10f7fb50 b #0x10f7fd9c
pc=0x10f7fd9c;continue;
}
0x10f7fb54=>{
// 0x10f7fb54 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7fb58 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10f7fb5c mov w2, #1
cpu.x[2]=(((0x1u64))) as u32 as u64;
// 0x10f7fb60 bl #0x10f89258
cpu.x[30]=0x10f7fb64;super::chunk5::f_10f89258(cpu,rt,0x10f89258)?;
pc=0x10f7fb64;continue;
}
0x10f7fb64=>{
// 0x10f7fb64 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7fb68 bl #0x10f942c8
cpu.x[30]=0x10f7fb6c;super::chunk1::f_10f942c8(cpu,rt,0x10f942c8)?;
pc=0x10f7fb6c;continue;
}
0x10f7fb6c=>{
// 0x10f7fb6c cbz x0, #0x10f7fd9c
if (cpu.x[0]) == 0 {pc=0x10f7fd9c;continue;}pc=0x10f7fb70;continue;
}
0x10f7fb70=>{
// 0x10f7fb70 bl #0x10fcca18
cpu.x[30]=0x10f7fb74;super::chunk13::f_10fcca18(cpu,rt,0x10fcca18)?;
pc=0x10f7fb74;continue;
}
0x10f7fb74=>{
// 0x10f7fb74 b #0x10f7fd9c
pc=0x10f7fd9c;continue;
}
0x10f7fb78=>{
// 0x10f7fb78 adrp x8, #0x11a65000
cpu.x[8]=(0x11a65000u64) as u64;
// 0x10f7fb7c add x8, x8, #0xf78
{let a=((cpu.x[8]));let b=((0xf78u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10f7fb80 ldr x9, [x19, #0xb8]
{let address=(cpu.x[19]).wrapping_add(0xb8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f7fb84 ldr x8, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7fb88 orr x8, x9, x8
{let a=((cpu.x[9]));let b=((cpu.x[8]));let result=a | b;cpu.x[8]=(result) as u64;}
// 0x10f7fb8c ldr x9, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f7fb90 str x8, [x19, #0xb8]
{let address=(cpu.x[19]).wrapping_add(0xb8u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f7fb94 ldr x2, [x9, #0x720]
{let address=(cpu.x[9]).wrapping_add(0x720u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10f7fb98 cbz x2, #0x10f7fba8
if (cpu.x[2]) == 0 {pc=0x10f7fba8;continue;}pc=0x10f7fb9c;continue;
}
0x10f7fb9c=>{
// 0x10f7fb9c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7fba0 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f7fba4 bl #0x10f89720
cpu.x[30]=0x10f7fba8;super::chunk13::f_10f89720(cpu,rt,0x10f89720)?;
pc=0x10f7fba8;continue;
}
0x10f7fba8=>{
// 0x10f7fba8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7fbac bl #0x10f942c8
cpu.x[30]=0x10f7fbb0;super::chunk1::f_10f942c8(cpu,rt,0x10f942c8)?;
// 0x10f7fbb0 cbz x0, #0x10f7fd9c
if (cpu.x[0]) == 0 {pc=0x10f7fd9c;continue;}pc=0x10f7fbb4;continue;
}
0x10f7fbb4=>{
// 0x10f7fbb4 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10f7fbb8 bl #0x10fcc464
cpu.x[30]=0x10f7fbbc;super::chunk2::f_10fcc464(cpu,rt,0x10fcc464)?;
// 0x10f7fbbc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7fbc0 bl #0x10fcf7b0
cpu.x[30]=0x10f7fbc4;super::dispatch(cpu,rt,0x10fcf7b0)?;
// 0x10f7fbc4 str wzr, [x19, #0x1f0]
{let address=(cpu.x[19]).wrapping_add(0x1f0u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f7fbc8 b #0x10f7fd9c
pc=0x10f7fd9c;continue;
}
0x10f7fbcc=>{
// 0x10f7fbcc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7fbd0 bl #0x10de0720
cpu.x[30]=0x10f7fbd4;super::chunk3::f_10de0720(cpu,rt,0x10de0720)?;
pc=0x10f7fbd4;continue;
}
0x10f7fbd4=>{
// 0x10f7fbd4 cbz x0, #0x10f7fcb8
if (cpu.x[0]) == 0 {pc=0x10f7fcb8;continue;}pc=0x10f7fbd8;continue;
}
0x10f7fbd8=>{
// 0x10f7fbd8 mov x23, x0
cpu.x[23]=(((cpu.x[0]))) as u64;
// 0x10f7fbdc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7fbe0 bl #0x10de0710
cpu.x[30]=0x10f7fbe4;super::chunk15::f_10de0710(cpu,rt,0x10de0710)?;
pc=0x10f7fbe4;continue;
}
0x10f7fbe4=>{
// 0x10f7fbe4 tbz w0, #0, #0x10f7fcb8
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f7fcb8;continue;}pc=0x10f7fbe8;continue;
}
0x10f7fbe8=>{
// 0x10f7fbe8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7fbec bl #0x10f93f94
cpu.x[30]=0x10f7fbf0;super::chunk3::f_10f93f94(cpu,rt,0x10f93f94)?;
pc=0x10f7fbf0;continue;
}
0x10f7fbf0=>{
// 0x10f7fbf0 ldr x22, [x0, #0xe0]
{let address=(cpu.x[0]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[22]=(v0) as u64;}
// 0x10f7fbf4 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f7fbf8 bl #0x10f8a060
cpu.x[30]=0x10f7fbfc;super::chunk5::f_10f8a060(cpu,rt,0x10f8a060)?;
pc=0x10f7fbfc;continue;
}
0x10f7fbfc=>{
// 0x10f7fbfc mov x1, x23
cpu.x[1]=(((cpu.x[23]))) as u64;
pc=0x10f7fc00;continue;
}
0x10f7fc00=>{
// 0x10f7fc00 bl #0x10f870a4
cpu.x[30]=0x10f7fc04;super::chunk10::f_10f870a4(cpu,rt,0x10f870a4)?;
pc=0x10f7fc04;continue;
}
0x10f7fc04=>{
// 0x10f7fc04 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f7fc08 bl #0x10f870b8
cpu.x[30]=0x10f7fc0c;super::chunk7::f_10f870b8(cpu,rt,0x10f870b8)?;
pc=0x10f7fc0c;continue;
}
0x10f7fc0c=>{
// 0x10f7fc0c mov x23, x0
cpu.x[23]=(((cpu.x[0]))) as u64;
// 0x10f7fc10 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7fc14 bl #0x10f94298
cpu.x[30]=0x10f7fc18;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
pc=0x10f7fc18;continue;
}
0x10f7fc18=>{
// 0x10f7fc18 mov w24, w0
cpu.x[24]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f7fc1c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7fc20 bl #0x10f942a0
cpu.x[30]=0x10f7fc24;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
pc=0x10f7fc24;continue;
}
0x10f7fc24=>{
// 0x10f7fc24 mov w25, w0
cpu.x[25]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f7fc28 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7fc2c bl #0x10f942a8
cpu.x[30]=0x10f7fc30;super::chunk11::f_10f942a8(cpu,rt,0x10f942a8)?;
pc=0x10f7fc30;continue;
}
0x10f7fc30=>{
// 0x10f7fc30 mov w3, w0
cpu.x[3]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f7fc34 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10f7fc38 mov w1, w24
cpu.x[1]=(((cpu.x[24] as u32 as u64))) as u32 as u64;
// 0x10f7fc3c mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10f7fc40 bl #0x10f870ac
cpu.x[30]=0x10f7fc44;super::chunk2::f_10f870ac(cpu,rt,0x10f870ac)?;
pc=0x10f7fc44;continue;
}
0x10f7fc44=>{
// 0x10f7fc44 mov x23, x0
cpu.x[23]=(((cpu.x[0]))) as u64;
// 0x10f7fc48 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7fc4c bl #0x10de06f0
cpu.x[30]=0x10f7fc50;super::chunk7::f_10de06f0(cpu,rt,0x10de06f0)?;
pc=0x10f7fc50;continue;
}
0x10f7fc50=>{
// 0x10f7fc50 mov w1, w0
cpu.x[1]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f7fc54 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10f7fc58 bl #0x10f870c0
cpu.x[30]=0x10f7fc5c;super::chunk12::f_10f870c0(cpu,rt,0x10f870c0)?;
pc=0x10f7fc5c;continue;
}
0x10f7fc5c=>{
// 0x10f7fc5c mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10f7fc60 bl #0x10f87118
cpu.x[30]=0x10f7fc64;super::chunk15::f_10f87118(cpu,rt,0x10f87118)?;
pc=0x10f7fc64;continue;
}
0x10f7fc64=>{
// 0x10f7fc64 mov x23, x0
cpu.x[23]=(((cpu.x[0]))) as u64;
// 0x10f7fc68 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7fc6c bl #0x10de0740
cpu.x[30]=0x10f7fc70;super::chunk14::f_10de0740(cpu,rt,0x10de0740)?;
pc=0x10f7fc70;continue;
}
0x10f7fc70=>{
// 0x10f7fc70 mov w1, w0
cpu.x[1]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f7fc74 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10f7fc78 bl #0x10f870c8
cpu.x[30]=0x10f7fc7c;super::chunk11::f_10f870c8(cpu,rt,0x10f870c8)?;
pc=0x10f7fc7c;continue;
}
0x10f7fc7c=>{
// 0x10f7fc7c mov x23, x0
cpu.x[23]=(((cpu.x[0]))) as u64;
// 0x10f7fc80 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7fc84 bl #0x10de0bd8
cpu.x[30]=0x10f7fc88;super::chunk11::f_10de0bd8(cpu,rt,0x10de0bd8)?;
pc=0x10f7fc88;continue;
}
0x10f7fc88=>{
// 0x10f7fc88 mov w1, w0
cpu.x[1]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f7fc8c mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10f7fc90 bl #0x10f870d0
cpu.x[30]=0x10f7fc94;super::chunk2::f_10f870d0(cpu,rt,0x10f870d0)?;
pc=0x10f7fc94;continue;
}
0x10f7fc94=>{
// 0x10f7fc94 ldr w1, [x19, #0x124]
{let address=(cpu.x[19]).wrapping_add(0x124u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7fc98 bl #0x10f870e0
cpu.x[30]=0x10f7fc9c;super::chunk4::f_10f870e0(cpu,rt,0x10f870e0)?;
pc=0x10f7fc9c;continue;
}
0x10f7fc9c=>{
// 0x10f7fc9c mov x23, x0
cpu.x[23]=(((cpu.x[0]))) as u64;
// 0x10f7fca0 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f7fca4 mov x1, x23
cpu.x[1]=(((cpu.x[23]))) as u64;
// 0x10f7fca8 bl #0x10f8aac8
cpu.x[30]=0x10f7fcac;super::chunk4::f_10f8aac8(cpu,rt,0x10f8aac8)?;
pc=0x10f7fcac;continue;
}
0x10f7fcac=>{
// 0x10f7fcac mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f7fcb0 mov x1, x23
cpu.x[1]=(((cpu.x[23]))) as u64;
// 0x10f7fcb4 bl #0x10f8a0cc
cpu.x[30]=0x10f7fcb8;super::chunk0::f_10f8a0cc(cpu,rt,0x10f8a0cc)?;
pc=0x10f7fcb8;continue;
}
0x10f7fcb8=>{
// 0x10f7fcb8 cmp w21, #0xb
{let a=((cpu.x[21] as u32 as u64));let b=((0xbu64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7fcbc b.ne #0x10f7fcd4
if cpu.condition(1){pc=0x10f7fcd4;continue;}pc=0x10f7fcc0;continue;
}
0x10f7fcc0=>{
// 0x10f7fcc0 ldr x0, [x19, #0x58]
{let address=(cpu.x[19]).wrapping_add(0x58u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f7fcc4 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10f7fcc8 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7fccc ldr x8, [x8, #0x138]
{let address=(cpu.x[8]).wrapping_add(0x138u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7fcd0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7fcd4;super::dispatch(cpu,rt,target)?;}
pc=0x10f7fcd4;continue;
}
0x10f7fcd4=>{
// 0x10f7fcd4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7fcd8 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10f7fcdc mov w2, #1
cpu.x[2]=(((0x1u64))) as u32 as u64;
// 0x10f7fce0 bl #0x10f89258
cpu.x[30]=0x10f7fce4;super::chunk5::f_10f89258(cpu,rt,0x10f89258)?;
pc=0x10f7fce4;continue;
}
0x10f7fce4=>{
// 0x10f7fce4 ldr w21, [x19, #0x15c]
{let address=(cpu.x[19]).wrapping_add(0x15cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[21]=(v0) as u32 as u64;}
// 0x10f7fce8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7fcec bl #0x10de0bd8
cpu.x[30]=0x10f7fcf0;super::chunk11::f_10de0bd8(cpu,rt,0x10de0bd8)?;
pc=0x10f7fcf0;continue;
}
0x10f7fcf0=>{
// 0x10f7fcf0 cmp w21, w0
{let a=((cpu.x[21] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7fcf4 csel w8, w21, w0, gt
cpu.x[8]=(if cpu.condition(12){((cpu.x[21] as u32 as u64))}else{((cpu.x[0] as u32 as u64))}) as u32 as u64;
// 0x10f7fcf8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7fcfc str w8, [x19, #0x15c]
{let address=(cpu.x[19]).wrapping_add(0x15cu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f7fd00 str w8, [x19, #0x160]
{let address=(cpu.x[19]).wrapping_add(0x160u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f7fd04 bl #0x10f93d2c
cpu.x[30]=0x10f7fd08;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
pc=0x10f7fd08;continue;
}
0x10f7fd08=>{
// 0x10f7fd08 ldr x0, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f7fd0c bl #0x110ca998
cpu.x[30]=0x10f7fd10;super::chunk3::f_110ca998(cpu,rt,0x110ca998)?;
pc=0x10f7fd10;continue;
}
0x10f7fd10=>{
// 0x10f7fd10 ldr w8, [x20, #0x5d0]
{let address=(cpu.x[20]).wrapping_add(0x5d0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f7fd14 str w0, [x19, #0x12c]
{let address=(cpu.x[19]).wrapping_add(0x12cu64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f7fd18 cbz w8, #0x10f7fd68
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f7fd68;continue;}pc=0x10f7fd1c;continue;
}
0x10f7fd1c=>{
// 0x10f7fd1c ldr w8, [x20, #0x5d4]
{let address=(cpu.x[20]).wrapping_add(0x5d4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f7fd20 cbz w8, #0x10f7fd68
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f7fd68;continue;}pc=0x10f7fd24;continue;
}
0x10f7fd24=>{
// 0x10f7fd24 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7fd28 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7fd2c ldr x8, [x8, #0x140]
{let address=(cpu.x[8]).wrapping_add(0x140u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7fd30 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7fd34;super::dispatch(cpu,rt,target)?;}
// 0x10f7fd34 tbnz w0, #0, #0x10f7fd68
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10f7fd68;continue;}pc=0x10f7fd38;continue;
}
0x10f7fd38=>{
// 0x10f7fd38 ldrb w8, [x20, #0x58a]
{let address=(cpu.x[20]).wrapping_add(0x58au64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f7fd3c ldrb w9, [x20, #0x589]
{let address=(cpu.x[20]).wrapping_add(0x589u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f7fd40 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7fd44 ldr w1, [x20, #0x5d0]
{let address=(cpu.x[20]).wrapping_add(0x5d0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7fd48 ldr w2, [x20, #0x5d4]
{let address=(cpu.x[20]).wrapping_add(0x5d4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[2]=(v0) as u32 as u64;}
// 0x10f7fd4c mov w5, wzr
cpu.x[5]=(((0u64))) as u32 as u64;
// 0x10f7fd50 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7fd54 cset w8, ne
cpu.x[8]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x10f7fd58 cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7fd5c csinc w3, w8, wzr, ne
cpu.x[3]=(if cpu.condition(1){((cpu.x[8] as u32 as u64))}else{(((0u64))).wrapping_add(1)}) as u32 as u64;
// 0x10f7fd60 cset w4, ne
cpu.x[4]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x10f7fd64 bl #0x10f95c5c
cpu.x[30]=0x10f7fd68;super::chunk14::f_10f95c5c(cpu,rt,0x10f95c5c)?;
pc=0x10f7fd68;continue;
}
0x10f7fd68=>{
// 0x10f7fd68 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7fd6c bl #0x10de07cc
cpu.x[30]=0x10f7fd70;super::chunk14::f_10de07cc(cpu,rt,0x10de07cc)?;
pc=0x10f7fd70;continue;
}
0x10f7fd70=>{
// 0x10f7fd70 tbz w0, #0, #0x10f7fd9c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f7fd9c;continue;}pc=0x10f7fd74;continue;
}
0x10f7fd74=>{
// 0x10f7fd74 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7fd78 bl #0x10de06a0
cpu.x[30]=0x10f7fd7c;super::chunk9::f_10de06a0(cpu,rt,0x10de06a0)?;
// 0x10f7fd7c tbz w0, #0, #0x10f7fd9c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f7fd9c;continue;}pc=0x10f7fd80;continue;
}
0x10f7fd80=>{
// 0x10f7fd80 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7fd84 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10f7fd88 b #0x10f7fd94
pc=0x10f7fd94;continue;
}
0x10f7fd8c=>{
// 0x10f7fd8c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7fd90 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
pc=0x10f7fd94;continue;
}
0x10f7fd94=>{
// 0x10f7fd94 mov w2, #1
cpu.x[2]=(((0x1u64))) as u32 as u64;
pc=0x10f7fd98;continue;
}
0x10f7fd98=>{
// 0x10f7fd98 bl #0x10f89258
cpu.x[30]=0x10f7fd9c;super::chunk5::f_10f89258(cpu,rt,0x10f89258)?;
pc=0x10f7fd9c;continue;
}
0x10f7fd9c=>{
// 0x10f7fd9c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7fda0 bl #0x10f7fee4
cpu.x[30]=0x10f7fda4;super::chunk0::f_10f7fee4(cpu,rt,0x10f7fee4)?;
pc=0x10f7fda4;continue;
}
0x10f7fda4=>{
// 0x10f7fda4 ldr x0, [x19, #0x58]
{let address=(cpu.x[19]).wrapping_add(0x58u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f7fda8 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7fdac ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7fdb0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7fdb4;super::dispatch(cpu,rt,target)?;}
pc=0x10f7fdb4;continue;
}
0x10f7fdb4=>{
// 0x10f7fdb4 ldp x20, x19, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f7fdb8 ldp x22, x21, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f7fdbc ldp x24, x23, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10f7fdc0 ldp x26, x25, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10f7fdc4 ldp x28, x27, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10f7fdc8 ldp x29, x30, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10f7fdcc add sp, sp, #0x80
{let a=((cpu.sp));let b=((0x80u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10f7fdd0 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f7fdd4=>{
// 0x10f7fdd4 ldr x8, [x20, #0x710]
{let address=(cpu.x[20]).wrapping_add(0x710u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7fdd8 mov w9, #0x851f
cpu.x[9]=(((0x851fu64))) as u32 as u64;
// 0x10f7fddc movk w9, #0x51eb, lsl #16
cpu.x[9]=((cpu.x[9] as u32 as u64 & !(0xffff0000u64)) | 0x51eb0000u64) as u32 as u64;
// 0x10f7fde0 ldrsw x8, [x8, #0x15c]
{let address=(cpu.x[8]).wrapping_add(0x15cu64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10f7fde4 smull x8, w8, w9
cpu.x[8]=(((((cpu.x[8] as u32 as u64))) as i32 as i64).wrapping_mul((((cpu.x[9] as u32 as u64))) as i32 as i64) as u64) as u64;
// 0x10f7fde8 lsr x9, x8, #0x3f
cpu.x[9]=(((((cpu.x[8]))) as u64).wrapping_shr(((((0x3fu64))) & 63) as u32) as u64) as u64;
// 0x10f7fdec asr x8, x8, #0x24
cpu.x[8]=(((((cpu.x[8]))) as i64).wrapping_shr(((((0x24u64))) & 63) as u32) as u64) as u64;
// 0x10f7fdf0 add w8, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10f7fdf4 str w8, [x19, #0x1e8]
{let address=(cpu.x[19]).wrapping_add(0x1e8u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f7fdf8 b #0x10f7fd9c
pc=0x10f7fd9c;continue;
}
0x10f7fdfc=>{
// 0x10f7fdfc adrp x8, #0x11980000
cpu.x[8]=(0x11980000u64) as u64;
// 0x10f7fe00 mov w10, #0x851f
cpu.x[10]=(((0x851fu64))) as u32 as u64;
// 0x10f7fe04 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7fe08 ldr x8, [x8, #0x560]
{let address=(cpu.x[8]).wrapping_add(0x560u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7fe0c ldr x9, [x19, #0xb8]
{let address=(cpu.x[19]).wrapping_add(0xb8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f7fe10 movk w10, #0x51eb, lsl #16
cpu.x[10]=((cpu.x[10] as u32 as u64 & !(0xffff0000u64)) | 0x51eb0000u64) as u32 as u64;
// 0x10f7fe14 ldr x8, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7fe18 orr x8, x9, x8
{let a=((cpu.x[9]));let b=((cpu.x[8]));let result=a | b;cpu.x[8]=(result) as u64;}
// 0x10f7fe1c str x8, [x19, #0xb8]
{let address=(cpu.x[19]).wrapping_add(0xb8u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f7fe20 ldr x8, [x20, #0x710]
{let address=(cpu.x[20]).wrapping_add(0x710u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7fe24 ldpsw x9, x8, [x8, #0xa0]
{let address=(cpu.x[8]).wrapping_add(0xa0u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;let v1=rt.memory.read::<i32>(address.wrapping_add(4)) as i64 as u64;cpu.x[9]=(v0) as u64;cpu.x[8]=(v1) as u64;}
// 0x10f7fe28 smull x9, w9, w10
cpu.x[9]=(((((cpu.x[9] as u32 as u64))) as i32 as i64).wrapping_mul((((cpu.x[10] as u32 as u64))) as i32 as i64) as u64) as u64;
// 0x10f7fe2c smull x8, w8, w10
cpu.x[8]=(((((cpu.x[8] as u32 as u64))) as i32 as i64).wrapping_mul((((cpu.x[10] as u32 as u64))) as i32 as i64) as u64) as u64;
// 0x10f7fe30 lsr x10, x9, #0x3f
cpu.x[10]=(((((cpu.x[9]))) as u64).wrapping_shr(((((0x3fu64))) & 63) as u32) as u64) as u64;
// 0x10f7fe34 asr x9, x9, #0x24
cpu.x[9]=(((((cpu.x[9]))) as i64).wrapping_shr(((((0x24u64))) & 63) as u32) as u64) as u64;
// 0x10f7fe38 lsr x11, x8, #0x3f
cpu.x[11]=(((((cpu.x[8]))) as u64).wrapping_shr(((((0x3fu64))) & 63) as u32) as u64) as u64;
// 0x10f7fe3c asr x8, x8, #0x24
cpu.x[8]=(((((cpu.x[8]))) as i64).wrapping_shr(((((0x24u64))) & 63) as u32) as u64) as u64;
// 0x10f7fe40 add w9, w9, w10
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10f7fe44 add w8, w8, w11
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[11] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10f7fe48 str w9, [x19, #0x1e8]
{let address=(cpu.x[19]).wrapping_add(0x1e8u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f7fe4c str w8, [x19, #0x1e4]
{let address=(cpu.x[19]).wrapping_add(0x1e4u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f7fe50 bl #0x10f942c8
cpu.x[30]=0x10f7fe54;super::chunk1::f_10f942c8(cpu,rt,0x10f942c8)?;
pc=0x10f7fe54;continue;
}
0x10f7fe54=>{
// 0x10f7fe54 cbz x0, #0x10f7fe5c
if (cpu.x[0]) == 0 {pc=0x10f7fe5c;continue;}pc=0x10f7fe58;continue;
}
0x10f7fe58=>{
// 0x10f7fe58 bl #0x10fce8f4
cpu.x[30]=0x10f7fe5c;super::chunk9::f_10fce8f4(cpu,rt,0x10fce8f4)?;
pc=0x10f7fe5c;continue;
}
0x10f7fe5c=>{
// 0x10f7fe5c ldr x0, [x19, #0x58]
{let address=(cpu.x[19]).wrapping_add(0x58u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f7fe60 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7fe64 ldr x8, [x8, #0x1e8]
{let address=(cpu.x[8]).wrapping_add(0x1e8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7fe68 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7fe6c;super::dispatch(cpu,rt,target)?;}
pc=0x10f7fe6c;continue;
}
0x10f7fe6c=>{
// 0x10f7fe6c ldr w8, [x19, #0x1e8]
{let address=(cpu.x[19]).wrapping_add(0x1e8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f7fe70 ldr w9, [x19, #0x1e4]
{let address=(cpu.x[19]).wrapping_add(0x1e4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f7fe74 orr w8, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10f7fe78 cbnz w8, #0x10f7fd9c
if (cpu.x[8] as u32 as u64) != 0 {pc=0x10f7fd9c;continue;}pc=0x10f7fe7c;continue;
}
0x10f7fe7c=>{
// 0x10f7fe7c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7fe80 bl #0x10f944ac
cpu.x[30]=0x10f7fe84;super::chunk4::f_10f944ac(cpu,rt,0x10f944ac)?;
// 0x10f7fe84 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f7fe88 bl #0x10fa3808
cpu.x[30]=0x10f7fe8c;super::chunk12::f_10fa3808(cpu,rt,0x10fa3808)?;
// 0x10f7fe8c ldr w8, [x19, #0x1e8]
{let address=(cpu.x[19]).wrapping_add(0x1e8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f7fe90 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7fe94 b.gt #0x10f7fd9c
if cpu.condition(12){pc=0x10f7fd9c;continue;}pc=0x10f7fe98;continue;
}
0x10f7fe98=>{
// 0x10f7fe98 ldr w8, [x19, #0x11c]
{let address=(cpu.x[19]).wrapping_add(0x11cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f7fe9c cmp w8, #0xa
{let a=((cpu.x[8] as u32 as u64));let b=((0xau64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7fea0 b.ne #0x10f7fd9c
if cpu.condition(1){pc=0x10f7fd9c;continue;}pc=0x10f7fea4;continue;
}
0x10f7fea4=>{
// 0x10f7fea4 cmp w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7fea8 b.ne #0x10f7fecc
if cpu.condition(1){pc=0x10f7fecc;continue;}pc=0x10f7feac;continue;
}
0x10f7feac=>{
// 0x10f7feac mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10f7feb0 str w8, [x19, #0x11c]
{let address=(cpu.x[19]).wrapping_add(0x11cu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f7feb4 b #0x10f7fdb4
pc=0x10f7fdb4;continue;
}
0x10f7feb8=>{
// 0x10f7feb8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7febc bl #0x10de08d0
cpu.x[30]=0x10f7fec0;super::chunk15::f_10de08d0(cpu,rt,0x10de08d0)?;
// 0x10f7fec0 str w0, [x19, #0x1a8]
{let address=(cpu.x[19]).wrapping_add(0x1a8u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f7fec4 str w0, [x19, #0x1b0]
{let address=(cpu.x[19]).wrapping_add(0x1b0u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f7fec8 b #0x10f7fd9c
pc=0x10f7fd9c;continue;
}
0x10f7fecc=>{
// 0x10f7fecc mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7fed0 mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10f7fed4 bl #0x10f7f2c8
cpu.x[30]=0x10f7fed8;super::chunk8::f_10f7f2c8(cpu,rt,0x10f7f2c8)?;
// 0x10f7fed8 b #0x10f7fdb4
pc=0x10f7fdb4;continue;
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f7f2c8")),}}}
#[inline]
pub(super) unsafe fn f_10f817a8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f817a8=>{
// 0x10f817a8 ldr w0, [x0, #0x11c]
{let address=(cpu.x[0]).wrapping_add(0x11cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10f817ac ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f817a8")),}}}
#[inline]
pub(super) unsafe fn f_10f8287c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f8287c=>{
// 0x10f8287c ldr x0, [x0, #0x48]
{let address=(cpu.x[0]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f82880 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10f82884 b #0x10de0450
super::chunk15::f_10de0450(cpu,rt,0x10de0450)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f8287c")),}}}
#[inline]
pub(super) unsafe fn f_10f82d14(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f82d14=>{
// 0x10f82d14 adrp x8, #0x11a65000
cpu.x[8]=(0x11a65000u64) as u64;
// 0x10f82d18 add x8, x8, #0xfe0
{let a=((cpu.x[8]));let b=((0xfe0u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10f82d1c ldr x9, [x0, #0xb0]
{let address=(cpu.x[0]).wrapping_add(0xb0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f82d20 ldr x8, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f82d24 tst x9, x8
{let a=((cpu.x[9]));let b=((cpu.x[8]));let result=a & b;cpu.logical_flags(result,64);}
// 0x10f82d28 b.eq #0x10f82d34
if cpu.condition(0){pc=0x10f82d34;continue;}pc=0x10f82d2c;continue;
}
0x10f82d2c=>{
// 0x10f82d2c mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10f82d30 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f82d34=>{
// 0x10f82d34 ldr w8, [x0, #0x11c]
{let address=(cpu.x[0]).wrapping_add(0x11cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f82d38 cmp w8, #0xe
{let a=((cpu.x[8] as u32 as u64));let b=((0xeu64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f82d3c b.hs #0x10f82d4c
if cpu.condition(2){pc=0x10f82d4c;continue;}pc=0x10f82d40;continue;
}
0x10f82d40=>{
// 0x10f82d40 mov w9, #0x35ed
cpu.x[9]=(((0x35edu64))) as u32 as u64;
// 0x10f82d44 lsr w8, w9, w8
cpu.x[8]=(((((cpu.x[9] as u32 as u64))) as u32).wrapping_shr(((((cpu.x[8] as u32 as u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10f82d48 tbnz w8, #0, #0x10f82d2c
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10f82d2c;continue;}pc=0x10f82d4c;continue;
}
0x10f82d4c=>{
// 0x10f82d4c stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f82d50 str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f82d54 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f82d58 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f82d5c bl #0x10f9433c
cpu.x[30]=0x10f82d60;super::chunk3::f_10f9433c(cpu,rt,0x10f9433c)?;
pc=0x10f82d60;continue;
}
0x10f82d60=>{
// 0x10f82d60 cbz x0, #0x10f82d94
if (cpu.x[0]) == 0 {pc=0x10f82d94;continue;}pc=0x10f82d64;continue;
}
0x10f82d64=>{
// 0x10f82d64 ldrb w8, [x0, #0x41]
{let address=(cpu.x[0]).wrapping_add(0x41u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f82d68 cbz w8, #0x10f82d7c
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f82d7c;continue;}pc=0x10f82d6c;continue;
}
0x10f82d6c=>{
// 0x10f82d6c mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10f82d70 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10f82d74 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f82d78 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f82d7c=>{
// 0x10f82d7c bl #0x10fc40ec
cpu.x[30]=0x10f82d80;super::chunk15::f_10fc40ec(cpu,rt,0x10fc40ec)?;
pc=0x10f82d80;continue;
}
0x10f82d80=>{
// 0x10f82d80 tbz w0, #0, #0x10f82d94
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f82d94;continue;}pc=0x10f82d84;continue;
}
0x10f82d84=>{
// 0x10f82d84 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10f82d88 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10f82d8c ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f82d90 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f82d94=>{
// 0x10f82d94 ldr w8, [x19, #0x1f0]
{let address=(cpu.x[19]).wrapping_add(0x1f0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f82d98 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f82d9c cset w0, lt
cpu.x[0]=((cpu.condition(11)) as u64) as u32 as u64;
// 0x10f82da0 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10f82da4 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f82da8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f82d14")),}}}
#[inline]
pub(super) unsafe fn f_10f85214(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f85214=>{
// 0x10f85214 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f85218 str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f8521c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f85220 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f85224 bl #0x10f95ac4
cpu.x[30]=0x10f85228;super::chunk12::f_10f95ac4(cpu,rt,0x10f95ac4)?;
pc=0x10f85228;continue;
}
0x10f85228=>{
// 0x10f85228 ldr w8, [x19, #0x15c]
{let address=(cpu.x[19]).wrapping_add(0x15cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8522c ldr w9, [x19, #0x1a8]
{let address=(cpu.x[19]).wrapping_add(0x1a8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f85230 str w8, [x19, #0x160]
{let address=(cpu.x[19]).wrapping_add(0x160u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f85234 str w9, [x19, #0x1b0]
{let address=(cpu.x[19]).wrapping_add(0x1b0u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f85238 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10f8523c ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f85240 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f85214")),}}}
#[inline]
pub(super) unsafe fn f_10f865f4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f865f4=>{
// 0x10f865f4 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f865f8 str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f865fc mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f86600 ldp x9, x8, [x0, #0x18]
{let address=(cpu.x[0]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[9]=(v0) as u64;cpu.x[8]=(v1) as u64;}
// 0x10f86604 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f86608 ldr x0, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f8660c ldr x2, [x9, #0x210]
{let address=(cpu.x[9]).wrapping_add(0x210u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10f86610 mov x1, x0
cpu.x[1]=(((cpu.x[0]))) as u64;
// 0x10f86614 bl #0x10f89954
cpu.x[30]=0x10f86618;super::chunk11::f_10f89954(cpu,rt,0x10f89954)?;
pc=0x10f86618;continue;
}
0x10f86618=>{
// 0x10f86618 ldr w1, [x19, #0x70]
{let address=(cpu.x[19]).wrapping_add(0x70u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f8661c cmn w1, #1
{let a=((cpu.x[1] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.flags(a,b,result,32,false);}
// 0x10f86620 b.eq #0x10f86644
if cpu.condition(0){pc=0x10f86644;continue;}pc=0x10f86624;continue;
}
0x10f86624=>{
// 0x10f86624 ldr x8, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f86628 cbz x8, #0x10f86644
if (cpu.x[8]) == 0 {pc=0x10f86644;continue;}pc=0x10f8662c;continue;
}
0x10f8662c=>{
// 0x10f8662c ldr x0, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f86630 cbz x0, #0x10f8663c
if (cpu.x[0]) == 0 {pc=0x10f8663c;continue;}pc=0x10f86634;continue;
}
0x10f86634=>{
// 0x10f86634 ldr w2, [x19, #0x74]
{let address=(cpu.x[19]).wrapping_add(0x74u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[2]=(v0) as u32 as u64;}
// 0x10f86638 bl #0x10f85b94
cpu.x[30]=0x10f8663c;super::chunk10::f_10f85b94(cpu,rt,0x10f85b94)?;
pc=0x10f8663c;continue;
}
0x10f8663c=>{
// 0x10f8663c mov w8, #-1
cpu.x[8]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10f86640 str w8, [x19, #0x70]
{let address=(cpu.x[19]).wrapping_add(0x70u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10f86644;continue;
}
0x10f86644=>{
// 0x10f86644 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10f86648 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f8664c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f865f4")),}}}
#[inline]
pub(super) unsafe fn f_10f87024(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f87024=>{
// 0x10f87024 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f87028 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f8702c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f87030 ldr q0, [x1, #0x10]
{let address=(cpu.x[1]).wrapping_add(0x10u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));cpu.q[0]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x10f87034 mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10f87038 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10f8703c str q0, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10f87040 ldr q0, [x1, #0x20]
{let address=(cpu.x[1]).wrapping_add(0x20u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));cpu.q[0]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x10f87044 str q0, [x0, #0x20]
{let address=(cpu.x[0]).wrapping_add(0x20u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10f87048 ldr q0, [x1]
{let address=(cpu.x[1]).wrapping_add(0x0u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));cpu.q[0]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x10f8704c ldr w8, [x1, #0x30]
{let address=(cpu.x[1]).wrapping_add(0x30u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f87050 str q0, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10f87054 str w8, [x0, #0x30]
{let address=(cpu.x[0]).wrapping_add(0x30u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f87058 ldur q0, [x1, #0x38]
{let address=(cpu.x[1]).wrapping_add(0x38u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));cpu.q[0]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x10f8705c ldr w8, [x1, #0x34]
{let address=(cpu.x[1]).wrapping_add(0x34u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f87060 add x1, x1, #0x48
{let a=((cpu.x[1]));let b=((0x48u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x10f87064 stur q0, [x0, #0x38]
{let address=(cpu.x[0]).wrapping_add(0x38u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10f87068 str w8, [x0, #0x34]
{let address=(cpu.x[0]).wrapping_add(0x34u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8706c add x0, x0, #0x48
{let a=((cpu.x[0]));let b=((0x48u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10f87070 bl #0x114302c8
cpu.x[30]=0x10f87074;super::chunk11::f_114302c8(cpu,rt,0x114302c8)?;
pc=0x10f87074;continue;
}
0x10f87074=>{
// 0x10f87074 ldr d0, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.q[0]=(v0) as u128 & 0xffffffffffffffffu128;}
// 0x10f87078 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f8707c str d0, [x20, #0x60]
{let address=(cpu.x[20]).wrapping_add(0x60u64);let v0=(cpu.q[0] & 0xffffffffffffffffu128) as u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f87080 ldrh w8, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=rt.memory.read::<u16>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f87084 strh w8, [x20, #0x68]
{let address=(cpu.x[20]).wrapping_add(0x68u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u16>(address.wrapping_add(0),v0 as u16);}
// 0x10f87088 ldrb w8, [x19, #0x6a]
{let address=(cpu.x[19]).wrapping_add(0x6au64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8708c ldr x9, [x19, #0x58]
{let address=(cpu.x[19]).wrapping_add(0x58u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f87090 strb w8, [x20, #0x6a]
{let address=(cpu.x[20]).wrapping_add(0x6au64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f87094 str x9, [x20, #0x58]
{let address=(cpu.x[20]).wrapping_add(0x58u64);let v0=cpu.x[9];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f87098 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f8709c ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f870a0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f87024")),}}}
#[inline]
pub(super) unsafe fn f_10f87ad8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f87ad8=>{
// 0x10f87ad8 cbz x1, #0x10f87ae4
if (cpu.x[1]) == 0 {pc=0x10f87ae4;continue;}pc=0x10f87adc;continue;
}
0x10f87adc=>{
// 0x10f87adc str x1, [x0, #0x20]
{let address=(cpu.x[0]).wrapping_add(0x20u64);let v0=cpu.x[1];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f87ae0 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f87ae4=>{
// 0x10f87ae4 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f87ae8 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f87aec stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f87af0 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f87af4 ldr x19, [x0, #0x20]
{let address=(cpu.x[0]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10f87af8 cbz x19, #0x10f87b50
if (cpu.x[19]) == 0 {pc=0x10f87b50;continue;}pc=0x10f87afc;continue;
}
0x10f87afc=>{
// 0x10f87afc ldrb w8, [x0, #0x18]
{let address=(cpu.x[0]).wrapping_add(0x18u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f87b00 cbz w8, #0x10f87b50
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f87b50;continue;}pc=0x10f87b04;continue;
}
0x10f87b04=>{
// 0x10f87b04 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10f87b08 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f87b0c mov x21, x1
cpu.x[21]=(((cpu.x[1]))) as u64;
// 0x10f87b10 bl #0x10f93d3c
cpu.x[30]=0x10f87b14;super::chunk3::f_10f93d3c(cpu,rt,0x10f93d3c)?;
pc=0x10f87b14;continue;
}
0x10f87b14=>{
// 0x10f87b14 str w0, [x20, #0x28]
{let address=(cpu.x[20]).wrapping_add(0x28u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f87b18 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f87b1c bl #0x10f95c00
cpu.x[30]=0x10f87b20;super::chunk11::f_10f95c00(cpu,rt,0x10f95c00)?;
pc=0x10f87b20;continue;
}
0x10f87b20=>{
// 0x10f87b20 str w0, [x20, #0x2c]
{let address=(cpu.x[20]).wrapping_add(0x2cu64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f87b24 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f87b28 bl #0x10f95c08
cpu.x[30]=0x10f87b2c;super::chunk13::f_10f95c08(cpu,rt,0x10f95c08)?;
pc=0x10f87b2c;continue;
}
0x10f87b2c=>{
// 0x10f87b2c ldr w8, [x19, #0xac]
{let address=(cpu.x[19]).wrapping_add(0xacu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f87b30 str w0, [x20, #0x34]
{let address=(cpu.x[20]).wrapping_add(0x34u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f87b34 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f87b38 str w8, [x20, #0x3c]
{let address=(cpu.x[20]).wrapping_add(0x3cu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f87b3c bl #0x10f95f8c
cpu.x[30]=0x10f87b40;super::chunk7::f_10f95f8c(cpu,rt,0x10f95f8c)?;
pc=0x10f87b40;continue;
}
0x10f87b40=>{
// 0x10f87b40 mov x1, x21
cpu.x[1]=(((cpu.x[21]))) as u64;
// 0x10f87b44 mov w8, w0
cpu.x[8]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f87b48 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f87b4c str w8, [x20, #0x40]
{let address=(cpu.x[20]).wrapping_add(0x40u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10f87b50;continue;
}
0x10f87b50=>{
// 0x10f87b50 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f87b54 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f87b58 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f87b5c str x1, [x0, #0x20]
{let address=(cpu.x[0]).wrapping_add(0x20u64);let v0=cpu.x[1];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f87b60 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f87ad8")),}}}
#[inline]
pub(super) unsafe fn f_10f87be8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f87be8=>{
// 0x10f87be8 mov x8, x0
cpu.x[8]=(((cpu.x[0]))) as u64;
// 0x10f87bec ldr x0, [x0, #0x20]
{let address=(cpu.x[0]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f87bf0 cbz x0, #0x10f87c00
if (cpu.x[0]) == 0 {pc=0x10f87c00;continue;}pc=0x10f87bf4;continue;
}
0x10f87bf4=>{
// 0x10f87bf4 ldrb w9, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f87bf8 cbz w9, #0x10f87c00
if (cpu.x[9] as u32 as u64) == 0 {pc=0x10f87c00;continue;}pc=0x10f87bfc;continue;
}
0x10f87bfc=>{
// 0x10f87bfc b #0x10f95f8c
super::chunk7::f_10f95f8c(cpu,rt,0x10f95f8c)?;return Ok(());
}
0x10f87c00=>{
// 0x10f87c00 ldr w0, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10f87c04 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f87be8")),}}}
#[inline]
pub(super) unsafe fn f_10f89abc(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f89abc=>{
// 0x10f89abc stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f89ac0 str x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f89ac4 stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f89ac8 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f89acc mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f89ad0 cbz x2, #0x10f89b24
if (cpu.x[2]) == 0 {pc=0x10f89b24;continue;}pc=0x10f89ad4;continue;
}
0x10f89ad4=>{
// 0x10f89ad4 ldr x8, [x2]
{let address=(cpu.x[2]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f89ad8 mov x22, x0
cpu.x[22]=(((cpu.x[0]))) as u64;
// 0x10f89adc mov x0, x2
cpu.x[0]=(((cpu.x[2]))) as u64;
// 0x10f89ae0 mov x20, x3
cpu.x[20]=(((cpu.x[3]))) as u64;
// 0x10f89ae4 mov x19, x2
cpu.x[19]=(((cpu.x[2]))) as u64;
// 0x10f89ae8 mov x21, x1
cpu.x[21]=(((cpu.x[1]))) as u64;
// 0x10f89aec ldr x8, [x8, #0x100]
{let address=(cpu.x[8]).wrapping_add(0x100u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f89af0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f89af4;super::dispatch(cpu,rt,target)?;}
pc=0x10f89af4;continue;
}
0x10f89af4=>{
// 0x10f89af4 tbz w0, #0, #0x10f89b38
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f89b38;continue;}pc=0x10f89af8;continue;
}
0x10f89af8=>{
// 0x10f89af8 ldr x8, [x22]
{let address=(cpu.x[22]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f89afc mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f89b00 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f89b04 mov x2, x21
cpu.x[2]=(((cpu.x[21]))) as u64;
// 0x10f89b08 ldr x4, [x8, #0x60]
{let address=(cpu.x[8]).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[4]=(v0) as u64;}
// 0x10f89b0c mov w3, #-1
cpu.x[3]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10f89b10 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f89b14 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10f89b18 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f89b1c ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10f89b20 br x4
{let target=cpu.x[4];if target>=0x10f89abc && target<0x10f89b84{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
0x10f89b24=>{
// 0x10f89b24 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f89b28 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10f89b2c ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f89b30 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10f89b34 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f89b38=>{
// 0x10f89b38 ldr x0, [x22, #0x28]
{let address=(cpu.x[22]).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f89b3c cbnz x0, #0x10f89b5c
if (cpu.x[0]) != 0 {pc=0x10f89b5c;continue;}pc=0x10f89b40;continue;
}
0x10f89b40=>{
// 0x10f89b40 mov w0, #0x68
cpu.x[0]=(((0x68u64))) as u32 as u64;
// 0x10f89b44 bl #0x1187d2b0
cpu.x[30]=0x10f89b48;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
// 0x10f89b48 mov x1, x22
cpu.x[1]=(((cpu.x[22]))) as u64;
// 0x10f89b4c mov x23, x0
cpu.x[23]=(((cpu.x[0]))) as u64;
// 0x10f89b50 bl #0x10f77db8
cpu.x[30]=0x10f89b54;super::chunk10::f_10f77db8(cpu,rt,0x10f77db8)?;
// 0x10f89b54 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10f89b58 str x23, [x22, #0x28]
{let address=(cpu.x[22]).wrapping_add(0x28u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10f89b5c;continue;
}
0x10f89b5c=>{
// 0x10f89b5c mov x1, x21
cpu.x[1]=(((cpu.x[21]))) as u64;
// 0x10f89b60 mov x2, x19
cpu.x[2]=(((cpu.x[19]))) as u64;
// 0x10f89b64 mov w3, #-1
cpu.x[3]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10f89b68 mov w4, wzr
cpu.x[4]=(((0u64))) as u32 as u64;
// 0x10f89b6c mov x5, x20
cpu.x[5]=(((cpu.x[20]))) as u64;
// 0x10f89b70 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f89b74 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10f89b78 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f89b7c ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10f89b80 b #0x10f7922c
super::chunk9::f_10f7922c(cpu,rt,0x10f7922c)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f89abc")),}}}
#[inline]
pub(super) unsafe fn f_10f8ba98(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f8ba98=>{
// 0x10f8ba98 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f8ba9c stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f8baa0 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f8baa4 ldr w8, [x0, #0x24]
{let address=(cpu.x[0]).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8baa8 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f8baac cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8bab0 b.ge #0x10f8bb00
if cpu.condition(10){pc=0x10f8bb00;continue;}pc=0x10f8bab4;continue;
}
0x10f8bab4=>{
// 0x10f8bab4 ldr w8, [x19, #0x14]
{let address=(cpu.x[19]).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8bab8 str wzr, [x19, #0x24]
{let address=(cpu.x[19]).wrapping_add(0x24u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8babc cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8bac0 b.ge #0x10f8bb44
if cpu.condition(10){pc=0x10f8bb44;continue;}pc=0x10f8bac4;continue;
}
0x10f8bac4=>{
// 0x10f8bac4 ldr w8, [x19, #0x34]
{let address=(cpu.x[19]).wrapping_add(0x34u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8bac8 str wzr, [x19, #0x14]
{let address=(cpu.x[19]).wrapping_add(0x14u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8bacc cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8bad0 b.ge #0x10f8bb88
if cpu.condition(10){pc=0x10f8bb88;continue;}pc=0x10f8bad4;continue;
}
0x10f8bad4=>{
// 0x10f8bad4 ldr w8, [x19, #0x44]
{let address=(cpu.x[19]).wrapping_add(0x44u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8bad8 str wzr, [x19, #0x34]
{let address=(cpu.x[19]).wrapping_add(0x34u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8badc cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8bae0 b.ge #0x10f8bbcc
if cpu.condition(10){pc=0x10f8bbcc;continue;}pc=0x10f8bae4;continue;
}
0x10f8bae4=>{
// 0x10f8bae4 str wzr, [x19, #0x44]
{let address=(cpu.x[19]).wrapping_add(0x44u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8bae8 str wzr, [x19, #0x54]
{let address=(cpu.x[19]).wrapping_add(0x54u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8baec ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f8baf0 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f8baf4 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f8baf8=>{
// 0x10f8baf8 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8bafc b.le #0x10f8bab4
if cpu.condition(13){pc=0x10f8bab4;continue;}pc=0x10f8bb00;continue;
}
0x10f8bb00=>{
// 0x10f8bb00 ldr x9, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f8bb04 sub w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10f8bb08 str w8, [x19, #0x24]
{let address=(cpu.x[19]).wrapping_add(0x24u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8bb0c ldr x20, [x9, w8, uxtw #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as u32 as u64)) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10f8bb10 cbz x20, #0x10f8baf8
if (cpu.x[20]) == 0 {pc=0x10f8baf8;continue;}pc=0x10f8bb14;continue;
}
0x10f8bb14=>{
// 0x10f8bb14 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8bb18 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f8bb1c ldr x8, [x8, #0x68]
{let address=(cpu.x[8]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8bb20 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8bb24;super::dispatch(cpu,rt,target)?;}
pc=0x10f8bb24;continue;
}
0x10f8bb24=>{
// 0x10f8bb24 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8bb28 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f8bb2c ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8bb30 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8bb34;super::dispatch(cpu,rt,target)?;}
pc=0x10f8bb34;continue;
}
0x10f8bb34=>{
// 0x10f8bb34 ldr w8, [x19, #0x24]
{let address=(cpu.x[19]).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8bb38 b #0x10f8baf8
pc=0x10f8baf8;continue;
}
0x10f8bb3c=>{
// 0x10f8bb3c cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8bb40 b.le #0x10f8bac4
if cpu.condition(13){pc=0x10f8bac4;continue;}pc=0x10f8bb44;continue;
}
0x10f8bb44=>{
// 0x10f8bb44 ldr x9, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f8bb48 sub w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10f8bb4c str w8, [x19, #0x14]
{let address=(cpu.x[19]).wrapping_add(0x14u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8bb50 ldr x20, [x9, w8, uxtw #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as u32 as u64)) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10f8bb54 cbz x20, #0x10f8bb3c
if (cpu.x[20]) == 0 {pc=0x10f8bb3c;continue;}pc=0x10f8bb58;continue;
}
0x10f8bb58=>{
// 0x10f8bb58 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8bb5c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f8bb60 ldr x8, [x8, #0x68]
{let address=(cpu.x[8]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8bb64 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8bb68;super::dispatch(cpu,rt,target)?;}
pc=0x10f8bb68;continue;
}
0x10f8bb68=>{
// 0x10f8bb68 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8bb6c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f8bb70 ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8bb74 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8bb78;super::dispatch(cpu,rt,target)?;}
pc=0x10f8bb78;continue;
}
0x10f8bb78=>{
// 0x10f8bb78 ldr w8, [x19, #0x14]
{let address=(cpu.x[19]).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8bb7c b #0x10f8bb3c
pc=0x10f8bb3c;continue;
}
0x10f8bb80=>{
// 0x10f8bb80 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8bb84 b.le #0x10f8bad4
if cpu.condition(13){pc=0x10f8bad4;continue;}pc=0x10f8bb88;continue;
}
0x10f8bb88=>{
// 0x10f8bb88 ldr x9, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f8bb8c sub w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10f8bb90 str w8, [x19, #0x34]
{let address=(cpu.x[19]).wrapping_add(0x34u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8bb94 ldr x20, [x9, w8, uxtw #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as u32 as u64)) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10f8bb98 cbz x20, #0x10f8bb80
if (cpu.x[20]) == 0 {pc=0x10f8bb80;continue;}pc=0x10f8bb9c;continue;
}
0x10f8bb9c=>{
// 0x10f8bb9c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8bba0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f8bba4 ldr x8, [x8, #0x68]
{let address=(cpu.x[8]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8bba8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8bbac;super::dispatch(cpu,rt,target)?;}
// 0x10f8bbac ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8bbb0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f8bbb4 ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8bbb8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8bbbc;super::dispatch(cpu,rt,target)?;}
// 0x10f8bbbc ldr w8, [x19, #0x34]
{let address=(cpu.x[19]).wrapping_add(0x34u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8bbc0 b #0x10f8bb80
pc=0x10f8bb80;continue;
}
0x10f8bbc4=>{
// 0x10f8bbc4 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8bbc8 b.le #0x10f8bae4
if cpu.condition(13){pc=0x10f8bae4;continue;}pc=0x10f8bbcc;continue;
}
0x10f8bbcc=>{
// 0x10f8bbcc ldr x9, [x19, #0x38]
{let address=(cpu.x[19]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f8bbd0 sub w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10f8bbd4 str w8, [x19, #0x44]
{let address=(cpu.x[19]).wrapping_add(0x44u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8bbd8 ldr x20, [x9, w8, uxtw #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as u32 as u64)) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10f8bbdc cbz x20, #0x10f8bbc4
if (cpu.x[20]) == 0 {pc=0x10f8bbc4;continue;}pc=0x10f8bbe0;continue;
}
0x10f8bbe0=>{
// 0x10f8bbe0 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8bbe4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f8bbe8 ldr x8, [x8, #0x68]
{let address=(cpu.x[8]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8bbec blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8bbf0;super::dispatch(cpu,rt,target)?;}
// 0x10f8bbf0 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8bbf4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f8bbf8 ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8bbfc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8bc00;super::dispatch(cpu,rt,target)?;}
// 0x10f8bc00 ldr w8, [x19, #0x44]
{let address=(cpu.x[19]).wrapping_add(0x44u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8bc04 b #0x10f8bbc4
pc=0x10f8bbc4;continue;
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f8ba98")),}}}
#[inline]
pub(super) unsafe fn f_10f8c1a0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f8c1a0=>{
// 0x10f8c1a0 mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x10f8c1a4 mov w1, w2
cpu.x[1]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x10f8c1a8 b #0x10f8a138
super::chunk11::f_10f8a138(cpu,rt,0x10f8a138)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f8c1a0")),}}}
#[inline]
pub(super) unsafe fn f_10f93760(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f93760=>{
// 0x10f93760 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10f93764 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f93760")),}}}
#[inline]
pub(super) unsafe fn f_10f93d44(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f93d44=>{
// 0x10f93d44 stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f93d48 str x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f93d4c stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f93d50 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f93d54 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f93d58 mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x10f93d5c mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f93d60 bl #0x10f89cf8
cpu.x[30]=0x10f93d64;super::chunk3::f_10f89cf8(cpu,rt,0x10f89cf8)?;
pc=0x10f93d64;continue;
}
0x10f93d64=>{
// 0x10f93d64 add x1, x19, #0x7c
{let a=((cpu.x[19]));let b=((0x7cu64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x10f93d68 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f93d6c bl #0x111dd2e0
cpu.x[30]=0x10f93d70;super::chunk9::f_111dd2e0(cpu,rt,0x111dd2e0)?;
pc=0x10f93d70;continue;
}
0x10f93d70=>{
// 0x10f93d70 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f93d74 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f93d78 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f93d7c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f93d80;super::dispatch(cpu,rt,target)?;}
pc=0x10f93d80;continue;
}
0x10f93d80=>{
// 0x10f93d80 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f93d84 str w0, [x19, #0x8c]
{let address=(cpu.x[19]).wrapping_add(0x8cu64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f93d88 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f93d8c str wzr, [x19, #0x6c]
{let address=(cpu.x[19]).wrapping_add(0x6cu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f93d90 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f93d94 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f93d98;super::dispatch(cpu,rt,target)?;}
pc=0x10f93d98;continue;
}
0x10f93d98=>{
// 0x10f93d98 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f93d9c b.lt #0x10f93e04
if cpu.condition(11){pc=0x10f93e04;continue;}pc=0x10f93da0;continue;
}
0x10f93da0=>{
// 0x10f93da0 mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f93da4 mov w22, #5
cpu.x[22]=(((0x5u64))) as u32 as u64;
// 0x10f93da8 b #0x10f93dc4
pc=0x10f93dc4;continue;
}
0x10f93dac=>{
// 0x10f93dac ldr x9, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f93db0 add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10f93db4 subs w21, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[21]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10f93db8 str w10, [x19, #0x6c]
{let address=(cpu.x[19]).wrapping_add(0x6cu64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f93dbc str w0, [x9, w8, sxtw #2]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f93dc0 b.eq #0x10f93e04
if cpu.condition(0){pc=0x10f93e04;continue;}pc=0x10f93dc4;continue;
}
0x10f93dc4=>{
// 0x10f93dc4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f93dc8 bl #0x111dd1e0
cpu.x[30]=0x10f93dcc;super::chunk10::f_111dd1e0(cpu,rt,0x111dd1e0)?;
// 0x10f93dcc ldp w9, w8, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10f93dd0 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f93dd4 b.ne #0x10f93dac
if cpu.condition(1){pc=0x10f93dac;continue;}pc=0x10f93dd8;continue;
}
0x10f93dd8=>{
// 0x10f93dd8 lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10f93ddc cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f93de0 csel w1, w22, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[22] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10f93de4 cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f93de8 b.ge #0x10f93dac
if cpu.condition(10){pc=0x10f93dac;continue;}pc=0x10f93dec;continue;
}
0x10f93dec=>{
// 0x10f93dec mov w23, w0
cpu.x[23]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f93df0 add x0, x19, #0x60
{let a=((cpu.x[19]));let b=((0x60u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10f93df4 bl #0x1078f4d4
cpu.x[30]=0x10f93df8;super::chunk2::f_1078f4d4(cpu,rt,0x1078f4d4)?;
// 0x10f93df8 ldr w8, [x19, #0x6c]
{let address=(cpu.x[19]).wrapping_add(0x6cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f93dfc mov w0, w23
cpu.x[0]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10f93e00 b #0x10f93dac
pc=0x10f93dac;continue;
}
0x10f93e04=>{
// 0x10f93e04 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f93e08 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f93e0c ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f93e10 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f93e14;super::dispatch(cpu,rt,target)?;}
pc=0x10f93e14;continue;
}
0x10f93e14=>{
// 0x10f93e14 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f93e18 str w0, [x19, #0x94]
{let address=(cpu.x[19]).wrapping_add(0x94u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f93e1c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f93e20 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f93e24 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f93e28;super::dispatch(cpu,rt,target)?;}
pc=0x10f93e28;continue;
}
0x10f93e28=>{
// 0x10f93e28 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f93e2c str w0, [x19, #0x98]
{let address=(cpu.x[19]).wrapping_add(0x98u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f93e30 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f93e34 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f93e38 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f93e3c;super::dispatch(cpu,rt,target)?;}
pc=0x10f93e3c;continue;
}
0x10f93e3c=>{
// 0x10f93e3c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f93e40 str w0, [x19, #0xac]
{let address=(cpu.x[19]).wrapping_add(0xacu64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f93e44 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f93e48 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f93e4c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f93e50;super::dispatch(cpu,rt,target)?;}
pc=0x10f93e50;continue;
}
0x10f93e50=>{
// 0x10f93e50 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f93e54 str w0, [x19, #0xc0]
{let address=(cpu.x[19]).wrapping_add(0xc0u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f93e58 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f93e5c ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f93e60 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f93e64;super::dispatch(cpu,rt,target)?;}
pc=0x10f93e64;continue;
}
0x10f93e64=>{
// 0x10f93e64 str x0, [x19, #0xb8]
{let address=(cpu.x[19]).wrapping_add(0xb8u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f93e68 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f93e6c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f93e70 ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f93e74 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f93e78;super::dispatch(cpu,rt,target)?;}
pc=0x10f93e78;continue;
}
0x10f93e78=>{
// 0x10f93e78 str x0, [x19, #0xb0]
{let address=(cpu.x[19]).wrapping_add(0xb0u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f93e7c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f93e80 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f93e84 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f93e88 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f93e8c;super::dispatch(cpu,rt,target)?;}
pc=0x10f93e8c;continue;
}
0x10f93e8c=>{
// 0x10f93e8c mov w8, w0
cpu.x[8]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f93e90 add x1, x19, #0xc8
{let a=((cpu.x[19]));let b=((0xc8u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x10f93e94 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f93e98 str w8, [x19, #0x74]
{let address=(cpu.x[19]).wrapping_add(0x74u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f93e9c ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f93ea0 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10f93ea4 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f93ea8 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10f93eac b #0x111ddf84
super::chunk1::f_111ddf84(cpu,rt,0x111ddf84)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f93d44")),}}}
#[inline]
pub(super) unsafe fn f_10f94078(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f94078=>{
// 0x10f94078 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f94078")),}}}
#[inline]
pub(super) unsafe fn f_10f9408c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f9408c=>{
// 0x10f9408c stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f94090 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f94094 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f94098 ldrb w8, [x0, #0x30]
{let address=(cpu.x[0]).wrapping_add(0x30u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f9409c tbz w8, #3, #0x10f940e0
if ((cpu.x[8] as u32 as u64) & (1u64 << 3)) == 0 {pc=0x10f940e0;continue;}pc=0x10f940a0;continue;
}
0x10f940a0=>{
// 0x10f940a0 ldr w8, [x0, #0x24]
{let address=(cpu.x[0]).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f940a4 cmp w8, #4
{let a=((cpu.x[8] as u32 as u64));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f940a8 b.lt #0x10f940e0
if cpu.condition(11){pc=0x10f940e0;continue;}pc=0x10f940ac;continue;
}
0x10f940ac=>{
// 0x10f940ac ldr x8, [x0, #0x18]
{let address=(cpu.x[0]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f940b0 ldr x8, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f940b4 cbz x8, #0x10f940e0
if (cpu.x[8]) == 0 {pc=0x10f940e0;continue;}pc=0x10f940b8;continue;
}
0x10f940b8=>{
// 0x10f940b8 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10f940bc mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10f940c0 bl #0x10fc0db4
cpu.x[30]=0x10f940c4;super::chunk13::f_10fc0db4(cpu,rt,0x10fc0db4)?;
pc=0x10f940c4;continue;
}
0x10f940c4=>{
// 0x10f940c4 mov w19, w0
cpu.x[19]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f940c8 ldr x0, [x20, #0x10]
{let address=(cpu.x[20]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f940cc bl #0x10f966a8
cpu.x[30]=0x10f940d0;super::chunk2::f_10f966a8(cpu,rt,0x10f966a8)?;
pc=0x10f940d0;continue;
}
0x10f940d0=>{
// 0x10f940d0 mov w1, w19
cpu.x[1]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10f940d4 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f940d8 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f940dc b #0x10d39264
super::chunk4::f_10d39264(cpu,rt,0x10d39264)?;return Ok(());
}
0x10f940e0=>{
// 0x10f940e0 ldr w19, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[19]=(v0) as u32 as u64;}
// 0x10f940e4 ldr x0, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f940e8 bl #0x10f966a8
cpu.x[30]=0x10f940ec;super::chunk2::f_10f966a8(cpu,rt,0x10f966a8)?;
// 0x10f940ec mov w1, w19
cpu.x[1]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10f940f0 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f940f4 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f940f8 b #0x10d39264
super::chunk4::f_10d39264(cpu,rt,0x10d39264)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f9408c")),}}}
#[inline]
pub(super) unsafe fn f_10f94f34(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f94f34=>{
// 0x10f94f34 add x0, x0, #0x7c
{let a=((cpu.x[0]));let b=((0x7cu64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10f94f38 add x1, x1, #0x7c
{let a=((cpu.x[1]));let b=((0x7cu64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x10f94f3c b #0x11294e74
super::chunk8::f_11294e74(cpu,rt,0x11294e74)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f94f34")),}}}
#[inline]
pub(super) unsafe fn f_10f95c18(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f95c18=>{
// 0x10f95c18 ldrb w0, [x0, #0x9c]
{let address=(cpu.x[0]).wrapping_add(0x9cu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10f95c1c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f95c18")),}}}
#[inline]
pub(super) unsafe fn f_10f960fc(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f960fc=>{
// 0x10f960fc stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f96100 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f96104 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f96108 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f9610c ldr x0, [x0, #0x58]
{let address=(cpu.x[0]).wrapping_add(0x58u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f96110 mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x10f96114 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f96118 ldr x8, [x8, #0x1f0]
{let address=(cpu.x[8]).wrapping_add(0x1f0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f9611c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f96120;super::dispatch(cpu,rt,target)?;}
pc=0x10f96120;continue;
}
0x10f96120=>{
// 0x10f96120 ldr w8, [x19, #0x30]
{let address=(cpu.x[19]).wrapping_add(0x30u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f96124 ldr w9, [x19, #0x24]
{let address=(cpu.x[19]).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f96128 str x20, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=cpu.x[20];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f9612c tbz w8, #0, #0x10f96158
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f96158;continue;}pc=0x10f96130;continue;
}
0x10f96130=>{
// 0x10f96130 cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f96134 b.lt #0x10f96158
if cpu.condition(11){pc=0x10f96158;continue;}pc=0x10f96138;continue;
}
0x10f96138=>{
// 0x10f96138 ldr x10, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10f9613c ldr x0, [x10]
{let address=(cpu.x[10]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f96140 cbz x0, #0x10f96158
if (cpu.x[0]) == 0 {pc=0x10f96158;continue;}pc=0x10f96144;continue;
}
0x10f96144=>{
// 0x10f96144 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f96148 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f9614c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f96150;super::dispatch(cpu,rt,target)?;}
pc=0x10f96150;continue;
}
0x10f96150=>{
// 0x10f96150 ldr w8, [x19, #0x30]
{let address=(cpu.x[19]).wrapping_add(0x30u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f96154 ldr w9, [x19, #0x24]
{let address=(cpu.x[19]).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
pc=0x10f96158;continue;
}
0x10f96158=>{
// 0x10f96158 tbz w8, #1, #0x10f96184
if ((cpu.x[8] as u32 as u64) & (1u64 << 1)) == 0 {pc=0x10f96184;continue;}pc=0x10f9615c;continue;
}
0x10f9615c=>{
// 0x10f9615c cmp w9, #2
{let a=((cpu.x[9] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f96160 b.lt #0x10f96184
if cpu.condition(11){pc=0x10f96184;continue;}pc=0x10f96164;continue;
}
0x10f96164=>{
// 0x10f96164 ldr x10, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10f96168 ldr x0, [x10, #8]
{let address=(cpu.x[10]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f9616c cbz x0, #0x10f96184
if (cpu.x[0]) == 0 {pc=0x10f96184;continue;}pc=0x10f96170;continue;
}
0x10f96170=>{
// 0x10f96170 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f96174 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f96178 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f9617c;super::dispatch(cpu,rt,target)?;}
pc=0x10f9617c;continue;
}
0x10f9617c=>{
// 0x10f9617c ldr w8, [x19, #0x30]
{let address=(cpu.x[19]).wrapping_add(0x30u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f96180 ldr w9, [x19, #0x24]
{let address=(cpu.x[19]).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
pc=0x10f96184;continue;
}
0x10f96184=>{
// 0x10f96184 tbz w8, #2, #0x10f961b0
if ((cpu.x[8] as u32 as u64) & (1u64 << 2)) == 0 {pc=0x10f961b0;continue;}pc=0x10f96188;continue;
}
0x10f96188=>{
// 0x10f96188 cmp w9, #3
{let a=((cpu.x[9] as u32 as u64));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9618c b.lt #0x10f961b0
if cpu.condition(11){pc=0x10f961b0;continue;}pc=0x10f96190;continue;
}
0x10f96190=>{
// 0x10f96190 ldr x10, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10f96194 ldr x0, [x10, #0x10]
{let address=(cpu.x[10]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f96198 cbz x0, #0x10f961b0
if (cpu.x[0]) == 0 {pc=0x10f961b0;continue;}pc=0x10f9619c;continue;
}
0x10f9619c=>{
// 0x10f9619c ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f961a0 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f961a4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f961a8;super::dispatch(cpu,rt,target)?;}
pc=0x10f961a8;continue;
}
0x10f961a8=>{
// 0x10f961a8 ldr w8, [x19, #0x30]
{let address=(cpu.x[19]).wrapping_add(0x30u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f961ac ldr w9, [x19, #0x24]
{let address=(cpu.x[19]).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
pc=0x10f961b0;continue;
}
0x10f961b0=>{
// 0x10f961b0 tbz w8, #3, #0x10f961d4
if ((cpu.x[8] as u32 as u64) & (1u64 << 3)) == 0 {pc=0x10f961d4;continue;}pc=0x10f961b4;continue;
}
0x10f961b4=>{
// 0x10f961b4 cmp w9, #4
{let a=((cpu.x[9] as u32 as u64));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f961b8 b.lt #0x10f961d4
if cpu.condition(11){pc=0x10f961d4;continue;}pc=0x10f961bc;continue;
}
0x10f961bc=>{
// 0x10f961bc ldr x8, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f961c0 ldr x0, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f961c4 cbz x0, #0x10f961d4
if (cpu.x[0]) == 0 {pc=0x10f961d4;continue;}pc=0x10f961c8;continue;
}
0x10f961c8=>{
// 0x10f961c8 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f961cc ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f961d0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f961d4;super::dispatch(cpu,rt,target)?;}
pc=0x10f961d4;continue;
}
0x10f961d4=>{
// 0x10f961d4 ldr x0, [x19, #0x58]
{let address=(cpu.x[19]).wrapping_add(0x58u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f961d8 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f961dc ldr x1, [x8, #0x1f8]
{let address=(cpu.x[8]).wrapping_add(0x1f8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10f961e0 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f961e4 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f961e8 br x1
{let target=cpu.x[1];if target>=0x10f960fc && target<0x10f961ec{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f960fc")),}}}
#[inline]
pub(super) unsafe fn f_10f96828(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f96828=>{
// 0x10f96828 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f9682c stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f96830 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f96834 ldr w8, [x0, #0xdc]
{let address=(cpu.x[0]).wrapping_add(0xdcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f96838 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f9683c cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f96840 b.lt #0x10f9686c
if cpu.condition(11){pc=0x10f9686c;continue;}pc=0x10f96844;continue;
}
0x10f96844=>{
// 0x10f96844 mov x20, xzr
cpu.x[20]=(((0u64))) as u64;
pc=0x10f96848;continue;
}
0x10f96848=>{
// 0x10f96848 ldr x8, [x19, #0xd0]
{let address=(cpu.x[19]).wrapping_add(0xd0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f9684c ldr x0, [x8, x20, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[20])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f96850 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f96854 ldr x8, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f96858 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f9685c;super::dispatch(cpu,rt,target)?;}
pc=0x10f9685c;continue;
}
0x10f9685c=>{
// 0x10f9685c ldrsw x8, [x19, #0xdc]
{let address=(cpu.x[19]).wrapping_add(0xdcu64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10f96860 add x20, x20, #1
{let a=((cpu.x[20]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[20]=(result) as u64;}
// 0x10f96864 cmp x20, x8
{let a=((cpu.x[20]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f96868 b.lt #0x10f96848
if cpu.condition(11){pc=0x10f96848;continue;}pc=0x10f9686c;continue;
}
0x10f9686c=>{
// 0x10f9686c ldr x8, [x19, #0x98]
{let address=(cpu.x[19]).wrapping_add(0x98u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f96870 ldr x0, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f96874 bl #0x111d9130
cpu.x[30]=0x10f96878;super::chunk1::f_111d9130(cpu,rt,0x111d9130)?;
pc=0x10f96878;continue;
}
0x10f96878=>{
// 0x10f96878 ldr x0, [x19, #0xa0]
{let address=(cpu.x[19]).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f9687c cbz x0, #0x10f96884
if (cpu.x[0]) == 0 {pc=0x10f96884;continue;}pc=0x10f96880;continue;
}
0x10f96880=>{
// 0x10f96880 bl #0x10f9ad30
cpu.x[30]=0x10f96884;super::chunk13::f_10f9ad30(cpu,rt,0x10f9ad30)?;
pc=0x10f96884;continue;
}
0x10f96884=>{
// 0x10f96884 ldr x0, [x19, #0xb8]
{let address=(cpu.x[19]).wrapping_add(0xb8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f96888 cbz x0, #0x10f96898
if (cpu.x[0]) == 0 {pc=0x10f96898;continue;}pc=0x10f9688c;continue;
}
0x10f9688c=>{
// 0x10f9688c ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f96890 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f96894 b #0x10f74760
super::chunk0::f_10f74760(cpu,rt,0x10f74760)?;return Ok(());
}
0x10f96898=>{
// 0x10f96898 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f9689c ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f968a0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f96828")),}}}
#[inline]
pub(super) unsafe fn f_10f99780(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f99780=>{
// 0x10f99780 cmn w1, #1
{let a=((cpu.x[1] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.flags(a,b,result,32,false);}
// 0x10f99784 b.eq #0x10f997d0
if cpu.condition(0){pc=0x10f997d0;continue;}pc=0x10f99788;continue;
}
0x10f99788=>{
// 0x10f99788 ldr w8, [x0, #0x1c]
{let address=(cpu.x[0]).wrapping_add(0x1cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f9978c cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f99790 b.lt #0x10f997d0
if cpu.condition(11){pc=0x10f997d0;continue;}pc=0x10f99794;continue;
}
0x10f99794=>{
// 0x10f99794 ldr x9, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f99798 sub x9, x9, #8
{let a=((cpu.x[9]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u64;}
// 0x10f9979c b #0x10f997b0
pc=0x10f997b0;continue;
}
0x10f997a0=>{
// 0x10f997a0 sub x8, x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u64;}
// 0x10f997a4 add x10, x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x10f997a8 cmp x10, #1
{let a=((cpu.x[10]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f997ac b.ls #0x10f997d0
if cpu.condition(9){pc=0x10f997d0;continue;}pc=0x10f997b0;continue;
}
0x10f997b0=>{
// 0x10f997b0 ldr x10, [x9, x8, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[8])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10f997b4 ldr x11, [x10, #8]
{let address=(cpu.x[10]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[11]=(v0) as u64;}
// 0x10f997b8 cbz x11, #0x10f997a0
if (cpu.x[11]) == 0 {pc=0x10f997a0;continue;}pc=0x10f997bc;continue;
}
0x10f997bc=>{
// 0x10f997bc ldr w10, [x10, #4]
{let address=(cpu.x[10]).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10f997c0 cmp w10, w1
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f997c4 b.ne #0x10f997a0
if cpu.condition(1){pc=0x10f997a0;continue;}pc=0x10f997c8;continue;
}
0x10f997c8=>{
// 0x10f997c8 mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
// 0x10f997cc ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f997d0=>{
// 0x10f997d0 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10f997d4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f99780")),}}}
#[inline]
pub(super) unsafe fn f_10fa0238(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fa0238=>{
// 0x10fa0238 b #0x10f81da4
super::chunk5::f_10f81da4(cpu,rt,0x10f81da4)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fa0238")),}}}
#[inline]
pub(super) unsafe fn f_10fbace8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fbace8=>{
// 0x10fbace8 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fbacec str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fbacf0 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fbacf4 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10fbacf8 bl #0x10f74060
cpu.x[30]=0x10fbacfc;super::chunk2::f_10f74060(cpu,rt,0x10f74060)?;
pc=0x10fbacfc;continue;
}
0x10fbacfc=>{
// 0x10fbacfc mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fbad00 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10fbad04 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10fbad08 b #0x1187d2c0
super::dispatch(cpu,rt,0x1187d2c0)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fbace8")),}}}
#[inline]
pub(super) unsafe fn f_10fc4c38(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fc4c38=>{
// 0x10fc4c38 sub sp, sp, #0xc0
{let a=((cpu.sp));let b=((0xc0u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x10fc4c3c stp x29, x30, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc4c40 stp x28, x27, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=cpu.x[28];let v1=cpu.x[27];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc4c44 stp x26, x25, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=cpu.x[26];let v1=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc4c48 stp x24, x23, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc4c4c stp x22, x21, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc4c50 stp x20, x19, [sp, #0xb0]
{let address=(cpu.sp).wrapping_add(0xb0u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc4c54 add x29, sp, #0x60
{let a=((cpu.sp));let b=((0x60u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x10fc4c58 ldr x21, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10fc4c5c mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10fc4c60 mov w23, w2
cpu.x[23]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x10fc4c64 mov w24, w1
cpu.x[24]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x10fc4c68 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fc4c6c bl #0x10f7e050
cpu.x[30]=0x10fc4c70;super::chunk10::f_10f7e050(cpu,rt,0x10f7e050)?;
pc=0x10fc4c70;continue;
}
0x10fc4c70=>{
// 0x10fc4c70 mov x28, x0
cpu.x[28]=(((cpu.x[0]))) as u64;
// 0x10fc4c74 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fc4c78 bl #0x10f93d2c
cpu.x[30]=0x10fc4c7c;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
pc=0x10fc4c7c;continue;
}
0x10fc4c7c=>{
// 0x10fc4c7c mov x22, x0
cpu.x[22]=(((cpu.x[0]))) as u64;
// 0x10fc4c80 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc4c84 bl #0x10fd049c
cpu.x[30]=0x10fc4c88;super::chunk6::f_10fd049c(cpu,rt,0x10fd049c)?;
pc=0x10fc4c88;continue;
}
0x10fc4c88=>{
// 0x10fc4c88 ldr x8, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc4c8c mov x25, x0
cpu.x[25]=(((cpu.x[0]))) as u64;
// 0x10fc4c90 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10fc4c94 bl #0x10f940fc
cpu.x[30]=0x10fc4c98;super::chunk9::f_10f940fc(cpu,rt,0x10f940fc)?;
pc=0x10fc4c98;continue;
}
0x10fc4c98=>{
// 0x10fc4c98 mov x26, x0
cpu.x[26]=(((cpu.x[0]))) as u64;
// 0x10fc4c9c bl #0x10fc22f0
cpu.x[30]=0x10fc4ca0;super::chunk3::f_10fc22f0(cpu,rt,0x10fc22f0)?;
pc=0x10fc4ca0;continue;
}
0x10fc4ca0=>{
// 0x10fc4ca0 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc4ca4 b.lt #0x10fc4ce8
if cpu.condition(11){pc=0x10fc4ce8;continue;}pc=0x10fc4ca8;continue;
}
0x10fc4ca8=>{
// 0x10fc4ca8 mov w27, wzr
cpu.x[27]=(((0u64))) as u32 as u64;
// 0x10fc4cac b #0x10fc4cc4
pc=0x10fc4cc4;continue;
}
0x10fc4cb0=>{
// 0x10fc4cb0 mov x0, x26
cpu.x[0]=(((cpu.x[26]))) as u64;
// 0x10fc4cb4 add w27, w27, #1
{let a=((cpu.x[27] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[27]=(result) as u32 as u64;}
// 0x10fc4cb8 bl #0x10fc22f0
cpu.x[30]=0x10fc4cbc;super::chunk3::f_10fc22f0(cpu,rt,0x10fc22f0)?;
pc=0x10fc4cbc;continue;
}
0x10fc4cbc=>{
// 0x10fc4cbc cmp w27, w0
{let a=((cpu.x[27] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc4cc0 b.ge #0x10fc4ce8
if cpu.condition(10){pc=0x10fc4ce8;continue;}pc=0x10fc4cc4;continue;
}
0x10fc4cc4=>{
// 0x10fc4cc4 ldr x8, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc4cc8 cbz x8, #0x10fc4cb0
if (cpu.x[8]) == 0 {pc=0x10fc4cb0;continue;}pc=0x10fc4ccc;continue;
}
0x10fc4ccc=>{
// 0x10fc4ccc mov x0, x26
cpu.x[0]=(((cpu.x[26]))) as u64;
// 0x10fc4cd0 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fc4cd4 bl #0x10fc23e4
cpu.x[30]=0x10fc4cd8;super::chunk11::f_10fc23e4(cpu,rt,0x10fc23e4)?;
pc=0x10fc4cd8;continue;
}
0x10fc4cd8=>{
// 0x10fc4cd8 ldr x8, [x0, #0x18]
{let address=(cpu.x[0]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc4cdc ldrb w8, [x8, #0x200]
{let address=(cpu.x[8]).wrapping_add(0x200u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc4ce0 cbz w8, #0x10fc4cb0
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10fc4cb0;continue;}pc=0x10fc4ce4;continue;
}
0x10fc4ce4=>{
// 0x10fc4ce4 b #0x10fc52e8
pc=0x10fc52e8;continue;
}
0x10fc4ce8=>{
// 0x10fc4ce8 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fc4cec bl #0x10f817a8
cpu.x[30]=0x10fc4cf0;super::chunk8::f_10f817a8(cpu,rt,0x10f817a8)?;
pc=0x10fc4cf0;continue;
}
0x10fc4cf0=>{
// 0x10fc4cf0 cmp w0, #3
{let a=((cpu.x[0] as u32 as u64));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc4cf4 b.eq #0x10fc52e8
if cpu.condition(0){pc=0x10fc52e8;continue;}pc=0x10fc4cf8;continue;
}
0x10fc4cf8=>{
// 0x10fc4cf8 cmp w0, #5
{let a=((cpu.x[0] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc4cfc b.eq #0x10fc52e8
if cpu.condition(0){pc=0x10fc52e8;continue;}pc=0x10fc4d00;continue;
}
0x10fc4d00=>{
// 0x10fc4d00 adrp x8, #0x11a65000
cpu.x[8]=(0x11a65000u64) as u64;
// 0x10fc4d04 add x8, x8, #0xfc8
{let a=((cpu.x[8]));let b=((0xfc8u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fc4d08 ldr x9, [x21, #0xb0]
{let address=(cpu.x[21]).wrapping_add(0xb0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fc4d0c ldr x8, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc4d10 tst x9, x8
{let a=((cpu.x[9]));let b=((cpu.x[8]));let result=a & b;cpu.logical_flags(result,64);}
// 0x10fc4d14 b.ne #0x10fc52e8
if cpu.condition(1){pc=0x10fc52e8;continue;}pc=0x10fc4d18;continue;
}
0x10fc4d18=>{
// 0x10fc4d18 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fc4d1c bl #0x10f7e050
cpu.x[30]=0x10fc4d20;super::chunk10::f_10f7e050(cpu,rt,0x10f7e050)?;
pc=0x10fc4d20;continue;
}
0x10fc4d20=>{
// 0x10fc4d20 ldr x27, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[27]=(v0) as u64;}
// 0x10fc4d24 stur x0, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fc4d28 mov x0, x27
cpu.x[0]=(((cpu.x[27]))) as u64;
// 0x10fc4d2c bl #0x10f7e050
cpu.x[30]=0x10fc4d30;super::chunk10::f_10f7e050(cpu,rt,0x10f7e050)?;
pc=0x10fc4d30;continue;
}
0x10fc4d30=>{
// 0x10fc4d30 mov x26, x0
cpu.x[26]=(((cpu.x[0]))) as u64;
// 0x10fc4d34 mov x0, x27
cpu.x[0]=(((cpu.x[27]))) as u64;
// 0x10fc4d38 bl #0x10f85d10
cpu.x[30]=0x10fc4d3c;super::chunk1::f_10f85d10(cpu,rt,0x10f85d10)?;
pc=0x10fc4d3c;continue;
}
0x10fc4d3c=>{
// 0x10fc4d3c mov w20, w0
cpu.x[20]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fc4d40 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc4d44 bl #0x10fd04a8
cpu.x[30]=0x10fc4d48;super::chunk2::f_10fd04a8(cpu,rt,0x10fd04a8)?;
pc=0x10fc4d48;continue;
}
0x10fc4d48=>{
// 0x10fc4d48 bl #0x10e0b6b4
cpu.x[30]=0x10fc4d4c;super::chunk8::f_10e0b6b4(cpu,rt,0x10e0b6b4)?;
pc=0x10fc4d4c;continue;
}
0x10fc4d4c=>{
// 0x10fc4d4c ldr x0, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fc4d50 bl #0x10e16208
cpu.x[30]=0x10fc4d54;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
pc=0x10fc4d54;continue;
}
0x10fc4d54=>{
// 0x10fc4d54 ldrb w8, [x0, #0x360]
{let address=(cpu.x[0]).wrapping_add(0x360u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc4d58 cbz w8, #0x10fc4d68
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10fc4d68;continue;}pc=0x10fc4d5c;continue;
}
0x10fc4d5c=>{
// 0x10fc4d5c mov x0, x26
cpu.x[0]=(((cpu.x[26]))) as u64;
// 0x10fc4d60 bl #0x10de0430
cpu.x[30]=0x10fc4d64;super::chunk11::f_10de0430(cpu,rt,0x10de0430)?;
pc=0x10fc4d64;continue;
}
0x10fc4d64=>{
// 0x10fc4d64 add w20, w0, w20
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[20] as u32 as u64));let result=a.wrapping_add(b);cpu.x[20]=(result) as u32 as u64;}
pc=0x10fc4d68;continue;
}
0x10fc4d68=>{
// 0x10fc4d68 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc4d6c bl #0x10fc4a68
cpu.x[30]=0x10fc4d70;super::chunk12::f_10fc4a68(cpu,rt,0x10fc4a68)?;
pc=0x10fc4d70;continue;
}
0x10fc4d70=>{
// 0x10fc4d70 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fc4d74 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc4d78 bl #0x10fd04a8
cpu.x[30]=0x10fc4d7c;super::chunk2::f_10fd04a8(cpu,rt,0x10fd04a8)?;
pc=0x10fc4d7c;continue;
}
0x10fc4d7c=>{
// 0x10fc4d7c bl #0x10e0b6b4
cpu.x[30]=0x10fc4d80;super::chunk8::f_10e0b6b4(cpu,rt,0x10e0b6b4)?;
pc=0x10fc4d80;continue;
}
0x10fc4d80=>{
// 0x10fc4d80 ldr x0, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fc4d84 bl #0x10e16208
cpu.x[30]=0x10fc4d88;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
pc=0x10fc4d88;continue;
}
0x10fc4d88=>{
// 0x10fc4d88 ldr w27, [x0, #0xd0]
{let address=(cpu.x[0]).wrapping_add(0xd0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[27]=(v0) as u32 as u64;}
// 0x10fc4d8c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc4d90 bl #0x10fd04a8
cpu.x[30]=0x10fc4d94;super::chunk2::f_10fd04a8(cpu,rt,0x10fd04a8)?;
pc=0x10fc4d94;continue;
}
0x10fc4d94=>{
// 0x10fc4d94 bl #0x10e0b6b4
cpu.x[30]=0x10fc4d98;super::chunk8::f_10e0b6b4(cpu,rt,0x10e0b6b4)?;
pc=0x10fc4d98;continue;
}
0x10fc4d98=>{
// 0x10fc4d98 ldr x0, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fc4d9c bl #0x10e16208
cpu.x[30]=0x10fc4da0;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
pc=0x10fc4da0;continue;
}
0x10fc4da0=>{
// 0x10fc4da0 ldr x8, [x25, #0xa0]
{let address=(cpu.x[25]).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc4da4 ldr w9, [x0, #0xd4]
{let address=(cpu.x[0]).wrapping_add(0xd4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fc4da8 stur w20, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=cpu.x[20] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc4dac str x8, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fc4db0 stur w9, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc4db4 tbz w24, #0, #0x10fc4de8
if ((cpu.x[24] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fc4de8;continue;}pc=0x10fc4db8;continue;
}
0x10fc4db8=>{
// 0x10fc4db8 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10fc4dbc mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10fc4dc0 bl #0x10f9bf98
cpu.x[30]=0x10fc4dc4;super::dispatch(cpu,rt,0x10f9bf98)?;
// 0x10fc4dc4 ldr w8, [x0, #0xc]
{let address=(cpu.x[0]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc4dc8 mov x25, x0
cpu.x[25]=(((cpu.x[0]))) as u64;
// 0x10fc4dcc stur x0, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fc4dd0 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc4dd4 b.gt #0x10fc4e48
if cpu.condition(12){pc=0x10fc4e48;continue;}pc=0x10fc4dd8;continue;
}
0x10fc4dd8=>{
// 0x10fc4dd8 ldr x0, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fc4ddc sub x1, x29, #8
{let a=((cpu.x[29]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.x[1]=(result) as u64;}
// 0x10fc4de0 bl #0x10f9abb8
cpu.x[30]=0x10fc4de4;super::chunk9::f_10f9abb8(cpu,rt,0x10f9abb8)?;
// 0x10fc4de4 b #0x10fc5238
pc=0x10fc5238;continue;
}
0x10fc4de8=>{
// 0x10fc4de8 cmp w9, w27
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[27] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc4dec mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fc4df0 mov w25, w20
cpu.x[25]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x10fc4df4 csel w9, w9, w27, gt
cpu.x[9]=(if cpu.condition(12){((cpu.x[9] as u32 as u64))}else{((cpu.x[27] as u32 as u64))}) as u32 as u64;
// 0x10fc4df8 mov w20, w27
cpu.x[20]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fc4dfc mov x27, x8
cpu.x[27]=(((cpu.x[8]))) as u64;
// 0x10fc4e00 stur w9, [x29, #-0xc]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff4u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc4e04 bl #0x10f942c0
cpu.x[30]=0x10fc4e08;super::chunk2::f_10f942c0(cpu,rt,0x10f942c0)?;
pc=0x10fc4e08;continue;
}
0x10fc4e08=>{
// 0x10fc4e08 ldur w8, [x29, #-0xc]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc4e0c mov x1, x0
cpu.x[1]=(((cpu.x[0]))) as u64;
// 0x10fc4e10 mov x0, x27
cpu.x[0]=(((cpu.x[27]))) as u64;
// 0x10fc4e14 mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10fc4e18 mov w4, #1
cpu.x[4]=(((0x1u64))) as u32 as u64;
// 0x10fc4e1c mov w5, wzr
cpu.x[5]=(((0u64))) as u32 as u64;
// 0x10fc4e20 add w2, w8, w25
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[25] as u32 as u64));let result=a.wrapping_add(b);cpu.x[2]=(result) as u32 as u64;}
// 0x10fc4e24 mov w6, wzr
cpu.x[6]=(((0u64))) as u32 as u64;
// 0x10fc4e28 mov w7, #-1
cpu.x[7]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10fc4e2c mov w27, w20
cpu.x[27]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x10fc4e30 bl #0x10f9afc8
cpu.x[30]=0x10fc4e34;super::chunk2::f_10f9afc8(cpu,rt,0x10f9afc8)?;
pc=0x10fc4e34;continue;
}
0x10fc4e34=>{
// 0x10fc4e34 ldr w8, [x0, #0xc]
{let address=(cpu.x[0]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc4e38 mov x25, x0
cpu.x[25]=(((cpu.x[0]))) as u64;
// 0x10fc4e3c stur x0, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fc4e40 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc4e44 b.le #0x10fc4dd8
if cpu.condition(13){pc=0x10fc4dd8;continue;}pc=0x10fc4e48;continue;
}
0x10fc4e48=>{
// 0x10fc4e48 stur w24, [x29, #-0xc]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff4u64);let v0=cpu.x[24] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc4e4c lsl x24, x8, #3
cpu.x[24]=(((((cpu.x[8]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10fc4e50 mov x20, xzr
cpu.x[20]=(((0u64))) as u64;
// 0x10fc4e54 mov w8, #0x7fffffff
cpu.x[8]=(((0x7fffffffu64))) as u32 as u64;
// 0x10fc4e58 stur x28, [x29, #-0x28]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffd8u64);let v0=cpu.x[28];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fc4e5c str x22, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[22];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fc4e60 str xzr, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fc4e64 stp w8, wzr, [sp, #4]
{let address=(cpu.sp).wrapping_add(0x4u64);let v0=cpu.x[8] as u32 as u64;let v1=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10fc4e68 str w8, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc4e6c stur w23, [x29, #-0x2c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffd4u64);let v0=cpu.x[23] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc4e70 str w26, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[26] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc4e74 str w27, [sp, #0x1c]
{let address=(cpu.sp).wrapping_add(0x1cu64);let v0=cpu.x[27] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc4e78 b #0x10fc4e90
pc=0x10fc4e90;continue;
}
0x10fc4e7c=>{
// 0x10fc4e7c ldur w23, [x29, #-0x2c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffd4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[23]=(v0) as u32 as u64;}
pc=0x10fc4e80;continue;
}
0x10fc4e80=>{
// 0x10fc4e80 ldr w26, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[26]=(v0) as u32 as u64;}
pc=0x10fc4e84;continue;
}
0x10fc4e84=>{
// 0x10fc4e84 add x20, x20, #8
{let a=((cpu.x[20]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[20]=(result) as u64;}
// 0x10fc4e88 cmp x24, x20
{let a=((cpu.x[24]));let b=((cpu.x[20]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fc4e8c b.eq #0x10fc5218
if cpu.condition(0){pc=0x10fc5218;continue;}pc=0x10fc4e90;continue;
}
0x10fc4e90=>{
// 0x10fc4e90 ldr x8, [x25]
{let address=(cpu.x[25]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc4e94 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc4e98 mov w2, #1
cpu.x[2]=(((0x1u64))) as u32 as u64;
// 0x10fc4e9c ldr x28, [x8, x20]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[20])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[28]=(v0) as u64;}
// 0x10fc4ea0 mov x1, x28
cpu.x[1]=(((cpu.x[28]))) as u64;
// 0x10fc4ea4 bl #0x10fc4008
cpu.x[30]=0x10fc4ea8;super::chunk5::f_10fc4008(cpu,rt,0x10fc4008)?;
pc=0x10fc4ea8;continue;
}
0x10fc4ea8=>{
// 0x10fc4ea8 tbz w0, #0, #0x10fc4e84
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fc4e84;continue;}pc=0x10fc4eac;continue;
}
0x10fc4eac=>{
// 0x10fc4eac ldr x8, [x28]
{let address=(cpu.x[28]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc4eb0 mov x0, x28
cpu.x[0]=(((cpu.x[28]))) as u64;
// 0x10fc4eb4 ldr x8, [x8, #0x138]
{let address=(cpu.x[8]).wrapping_add(0x138u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc4eb8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc4ebc;super::dispatch(cpu,rt,target)?;}
pc=0x10fc4ebc;continue;
}
0x10fc4ebc=>{
// 0x10fc4ebc tbz w0, #0, #0x10fc4e84
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fc4e84;continue;}pc=0x10fc4ec0;continue;
}
0x10fc4ec0=>{
// 0x10fc4ec0 tbz w23, #0, #0x10fc4ed8
if ((cpu.x[23] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fc4ed8;continue;}pc=0x10fc4ec4;continue;
}
0x10fc4ec4=>{
// 0x10fc4ec4 ldr x8, [x28]
{let address=(cpu.x[28]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc4ec8 mov x0, x28
cpu.x[0]=(((cpu.x[28]))) as u64;
// 0x10fc4ecc ldr x8, [x8, #0x180]
{let address=(cpu.x[8]).wrapping_add(0x180u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc4ed0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc4ed4;super::dispatch(cpu,rt,target)?;}
// 0x10fc4ed4 tbz w0, #0, #0x10fc4e84
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fc4e84;continue;}pc=0x10fc4ed8;continue;
}
0x10fc4ed8=>{
// 0x10fc4ed8 mov x0, x28
cpu.x[0]=(((cpu.x[28]))) as u64;
// 0x10fc4edc bl #0x10f940fc
cpu.x[30]=0x10fc4ee0;super::chunk9::f_10f940fc(cpu,rt,0x10f940fc)?;
pc=0x10fc4ee0;continue;
}
0x10fc4ee0=>{
// 0x10fc4ee0 cbz x0, #0x10fc4ef8
if (cpu.x[0]) == 0 {pc=0x10fc4ef8;continue;}pc=0x10fc4ee4;continue;
}
0x10fc4ee4=>{
// 0x10fc4ee4 mov x0, x28
cpu.x[0]=(((cpu.x[28]))) as u64;
// 0x10fc4ee8 bl #0x10f940fc
cpu.x[30]=0x10fc4eec;super::chunk9::f_10f940fc(cpu,rt,0x10f940fc)?;
pc=0x10fc4eec;continue;
}
0x10fc4eec=>{
// 0x10fc4eec ldr w8, [x0, #0x34]
{let address=(cpu.x[0]).wrapping_add(0x34u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc4ef0 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc4ef4 b.gt #0x10fc4e84
if cpu.condition(12){pc=0x10fc4e84;continue;}pc=0x10fc4ef8;continue;
}
0x10fc4ef8=>{
// 0x10fc4ef8 cmp w26, #1
{let a=((cpu.x[26] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc4efc b.lt #0x10fc4f24
if cpu.condition(11){pc=0x10fc4f24;continue;}pc=0x10fc4f00;continue;
}
0x10fc4f00=>{
// 0x10fc4f00 ldr x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fc4f04 bl #0x10f942c0
cpu.x[30]=0x10fc4f08;super::chunk2::f_10f942c0(cpu,rt,0x10f942c0)?;
pc=0x10fc4f08;continue;
}
0x10fc4f08=>{
// 0x10fc4f08 mov x1, x0
cpu.x[1]=(((cpu.x[0]))) as u64;
// 0x10fc4f0c mov x0, x28
cpu.x[0]=(((cpu.x[28]))) as u64;
// 0x10fc4f10 mov w2, wzr
cpu.x[2]=(((0u64))) as u32 as u64;
// 0x10fc4f14 mov w3, w26
cpu.x[3]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fc4f18 mov w4, #1
cpu.x[4]=(((0x1u64))) as u32 as u64;
// 0x10fc4f1c bl #0x10f958e0
cpu.x[30]=0x10fc4f20;super::chunk6::f_10f958e0(cpu,rt,0x10f958e0)?;
pc=0x10fc4f20;continue;
}
0x10fc4f20=>{
// 0x10fc4f20 tbz w0, #0, #0x10fc4e84
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fc4e84;continue;}pc=0x10fc4f24;continue;
}
0x10fc4f24=>{
// 0x10fc4f24 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fc4f28 mov x1, x28
cpu.x[1]=(((cpu.x[28]))) as u64;
// 0x10fc4f2c bl #0x10f94f34
cpu.x[30]=0x10fc4f30;super::chunk8::f_10f94f34(cpu,rt,0x10f94f34)?;
pc=0x10fc4f30;continue;
}
0x10fc4f30=>{
// 0x10fc4f30 ldr x8, [x28]
{let address=(cpu.x[28]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc4f34 mov w22, w0
cpu.x[22]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fc4f38 mov x0, x28
cpu.x[0]=(((cpu.x[28]))) as u64;
// 0x10fc4f3c ldr x8, [x8, #0x1d8]
{let address=(cpu.x[8]).wrapping_add(0x1d8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc4f40 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc4f44;super::dispatch(cpu,rt,target)?;}
pc=0x10fc4f44;continue;
}
0x10fc4f44=>{
// 0x10fc4f44 ldr x8, [x28]
{let address=(cpu.x[28]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc4f48 mov w27, w0
cpu.x[27]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fc4f4c mov x0, x28
cpu.x[0]=(((cpu.x[28]))) as u64;
// 0x10fc4f50 ldr x8, [x8, #0xa0]
{let address=(cpu.x[8]).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc4f54 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc4f58;super::dispatch(cpu,rt,target)?;}
pc=0x10fc4f58;continue;
}
0x10fc4f58=>{
// 0x10fc4f58 ldur w8, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc4f5c add w26, w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_add(b);cpu.x[26]=(result) as u32 as u64;}
// 0x10fc4f60 ldur x0, [x29, #-0x28]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffd8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fc4f64 bl #0x10ddff74
cpu.x[30]=0x10fc4f68;super::chunk11::f_10ddff74(cpu,rt,0x10ddff74)?;
pc=0x10fc4f68;continue;
}
0x10fc4f68=>{
// 0x10fc4f68 stur w22, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=cpu.x[22] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc4f6c tbnz w0, #0, #0x10fc5014
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10fc5014;continue;}pc=0x10fc4f70;continue;
}
0x10fc4f70=>{
// 0x10fc4f70 ldr x8, [x28]
{let address=(cpu.x[28]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc4f74 mov x0, x28
cpu.x[0]=(((cpu.x[28]))) as u64;
// 0x10fc4f78 ldr x8, [x8, #0x190]
{let address=(cpu.x[8]).wrapping_add(0x190u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc4f7c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc4f80;super::dispatch(cpu,rt,target)?;}
pc=0x10fc4f80;continue;
}
0x10fc4f80=>{
// 0x10fc4f80 tbz w0, #0, #0x10fc5014
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fc5014;continue;}pc=0x10fc4f84;continue;
}
0x10fc4f84=>{
// 0x10fc4f84 mov x0, x28
cpu.x[0]=(((cpu.x[28]))) as u64;
// 0x10fc4f88 bl #0x10f7e050
cpu.x[30]=0x10fc4f8c;super::chunk10::f_10f7e050(cpu,rt,0x10f7e050)?;
pc=0x10fc4f8c;continue;
}
0x10fc4f8c=>{
// 0x10fc4f8c bl #0x10de05cc
cpu.x[30]=0x10fc4f90;super::chunk1::f_10de05cc(cpu,rt,0x10de05cc)?;
pc=0x10fc4f90;continue;
}
0x10fc4f90=>{
// 0x10fc4f90 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc4f94 b.lt #0x10fc5014
if cpu.condition(11){pc=0x10fc5014;continue;}pc=0x10fc4f98;continue;
}
0x10fc4f98=>{
// 0x10fc4f98 mov x0, x28
cpu.x[0]=(((cpu.x[28]))) as u64;
// 0x10fc4f9c bl #0x10f7e050
cpu.x[30]=0x10fc4fa0;super::chunk10::f_10f7e050(cpu,rt,0x10f7e050)?;
// 0x10fc4fa0 bl #0x10de05cc
cpu.x[30]=0x10fc4fa4;super::chunk1::f_10de05cc(cpu,rt,0x10de05cc)?;
// 0x10fc4fa4 ldur x22, [x29, #-0x28]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffd8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[22]=(v0) as u64;}
// 0x10fc4fa8 ldr x9, [x28]
{let address=(cpu.x[28]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fc4fac ldr w8, [x22, #0x420]
{let address=(cpu.x[22]).wrapping_add(0x420u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc4fb0 ldr x9, [x9, #0xa0]
{let address=(cpu.x[9]).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fc4fb4 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc4fb8 csel w26, w0, w8, gt
cpu.x[26]=(if cpu.condition(12){((cpu.x[0] as u32 as u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10fc4fbc mov x0, x28
cpu.x[0]=(((cpu.x[28]))) as u64;
// 0x10fc4fc0 blr x9
{let target=cpu.x[9];cpu.x[30]=0x10fc4fc4;super::dispatch(cpu,rt,target)?;}
// 0x10fc4fc4 add w26, w0, w26
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[26] as u32 as u64));let result=a.wrapping_add(b);cpu.x[26]=(result) as u32 as u64;}
// 0x10fc4fc8 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10fc4fcc bl #0x10de05ec
cpu.x[30]=0x10fc4fd0;super::chunk5::f_10de05ec(cpu,rt,0x10de05ec)?;
// 0x10fc4fd0 tbz w0, #0, #0x10fc51a8
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fc51a8;continue;}pc=0x10fc4fd4;continue;
}
0x10fc4fd4=>{
// 0x10fc4fd4 ldr x9, [x28]
{let address=(cpu.x[28]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fc4fd8 ldr w8, [x22, #0x484]
{let address=(cpu.x[22]).wrapping_add(0x484u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc4fdc mov x0, x28
cpu.x[0]=(((cpu.x[28]))) as u64;
// 0x10fc4fe0 ldr x9, [x9, #0xa0]
{let address=(cpu.x[9]).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fc4fe4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc4fe8 csel w22, w26, w8, gt
cpu.x[22]=(if cpu.condition(12){((cpu.x[26] as u32 as u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10fc4fec blr x9
{let target=cpu.x[9];cpu.x[30]=0x10fc4ff0;super::dispatch(cpu,rt,target)?;}
// 0x10fc4ff0 mov w23, #1
cpu.x[23]=(((0x1u64))) as u32 as u64;
// 0x10fc4ff4 add w26, w0, w22
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[22] as u32 as u64));let result=a.wrapping_add(b);cpu.x[26]=(result) as u32 as u64;}
// 0x10fc4ff8 ldr x8, [x28]
{let address=(cpu.x[28]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc4ffc mov x0, x28
cpu.x[0]=(((cpu.x[28]))) as u64;
// 0x10fc5000 ldr x8, [x8, #0x178]
{let address=(cpu.x[8]).wrapping_add(0x178u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc5004 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc5008;super::dispatch(cpu,rt,target)?;}
// 0x10fc5008 ldur w8, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc500c tbnz w0, #0, #0x10fc504c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10fc504c;continue;}pc=0x10fc5010;continue;
}
0x10fc5010=>{
// 0x10fc5010 b #0x10fc5030
pc=0x10fc5030;continue;
}
0x10fc5014=>{
// 0x10fc5014 mov w23, wzr
cpu.x[23]=(((0u64))) as u32 as u64;
// 0x10fc5018 ldr x8, [x28]
{let address=(cpu.x[28]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc501c mov x0, x28
cpu.x[0]=(((cpu.x[28]))) as u64;
// 0x10fc5020 ldr x8, [x8, #0x178]
{let address=(cpu.x[8]).wrapping_add(0x178u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc5024 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc5028;super::dispatch(cpu,rt,target)?;}
pc=0x10fc5028;continue;
}
0x10fc5028=>{
// 0x10fc5028 ldur w8, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc502c tbnz w0, #0, #0x10fc504c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10fc504c;continue;}pc=0x10fc5030;continue;
}
0x10fc5030=>{
// 0x10fc5030 ldr x8, [x28]
{let address=(cpu.x[28]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc5034 mov x0, x28
cpu.x[0]=(((cpu.x[28]))) as u64;
// 0x10fc5038 ldr x8, [x8, #0x180]
{let address=(cpu.x[8]).wrapping_add(0x180u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc503c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc5040;super::dispatch(cpu,rt,target)?;}
pc=0x10fc5040;continue;
}
0x10fc5040=>{
// 0x10fc5040 ldr w8, [sp, #0x1c]
{let address=(cpu.sp).wrapping_add(0x1cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc5044 tst w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.logical_flags(result,32);}
// 0x10fc5048 csel w8, w8, wzr, ne
cpu.x[8]=(if cpu.condition(1){((cpu.x[8] as u32 as u64))}else{((0u64))}) as u32 as u64;
pc=0x10fc504c;continue;
}
0x10fc504c=>{
// 0x10fc504c add w22, w8, w26
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[26] as u32 as u64));let result=a.wrapping_add(b);cpu.x[22]=(result) as u32 as u64;}
// 0x10fc5050 ldur w8, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc5054 mul w9, w22, w22
cpu.x[9]=((((cpu.x[22] as u32 as u64))).wrapping_mul(((cpu.x[22] as u32 as u64)))) as u32 as u64;
// 0x10fc5058 sub w8, w8, w27
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[27] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fc505c bic w27, w8, w8, asr #31
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[8] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[27]=(result) as u32 as u64;}
// 0x10fc5060 cmp w27, w9
{let a=((cpu.x[27] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc5064 b.ls #0x10fc5070
if cpu.condition(9){pc=0x10fc5070;continue;}pc=0x10fc5068;continue;
}
0x10fc5068=>{
// 0x10fc5068 ldur w8, [x29, #-0xc]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc506c tbz w8, #0, #0x10fc4e7c
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fc4e7c;continue;}pc=0x10fc5070;continue;
}
0x10fc5070=>{
// 0x10fc5070 ldur x0, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fc5074 bl #0x10de1230
cpu.x[30]=0x10fc5078;super::chunk15::f_10de1230(cpu,rt,0x10de1230)?;
pc=0x10fc5078;continue;
}
0x10fc5078=>{
// 0x10fc5078 ldr x8, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc507c mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fc5080 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fc5084 ldr x8, [x8, #0x180]
{let address=(cpu.x[8]).wrapping_add(0x180u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc5088 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc508c;super::dispatch(cpu,rt,target)?;}
pc=0x10fc508c;continue;
}
0x10fc508c=>{
// 0x10fc508c cmp w26, #1
{let a=((cpu.x[26] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc5090 ldur w9, [x29, #-0xc]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fc5094 cset w8, lt
cpu.x[8]=((cpu.condition(11)) as u64) as u32 as u64;
// 0x10fc5098 orr w8, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fc509c orr w8, w23, w8
{let a=((cpu.x[23] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fc50a0 orr w8, w8, w0
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fc50a4 tbz w8, #0, #0x10fc5138
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fc5138;continue;}pc=0x10fc50a8;continue;
}
0x10fc50a8=>{
// 0x10fc50a8 ldur x0, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fc50ac bl #0x10de1238
cpu.x[30]=0x10fc50b0;super::chunk14::f_10de1238(cpu,rt,0x10de1238)?;
pc=0x10fc50b0;continue;
}
0x10fc50b0=>{
// 0x10fc50b0 ldur w8, [x29, #-0xc]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc50b4 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc50b8 cset w9, lt
cpu.x[9]=((cpu.condition(11)) as u64) as u32 as u64;
// 0x10fc50bc orr w8, w23, w8
{let a=((cpu.x[23] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fc50c0 orr w8, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fc50c4 tbnz w8, #0, #0x10fc50f4
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10fc50f4;continue;}pc=0x10fc50c8;continue;
}
0x10fc50c8=>{
// 0x10fc50c8 sub w8, w22, w0
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fc50cc mov x0, x28
cpu.x[0]=(((cpu.x[28]))) as u64;
// 0x10fc50d0 bic w22, w8, w8, asr #31
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[8] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[22]=(result) as u32 as u64;}
// 0x10fc50d4 bl #0x10f94298
cpu.x[30]=0x10fc50d8;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
pc=0x10fc50d8;continue;
}
0x10fc50d8=>{
// 0x10fc50d8 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fc50dc mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fc50e0 bl #0x10f94298
cpu.x[30]=0x10fc50e4;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
pc=0x10fc50e4;continue;
}
0x10fc50e4=>{
// 0x10fc50e4 subs w8, w26, w0
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10fc50e8 cneg w8, w8, mi
cpu.x[8]=(if cpu.condition(4){0u64.wrapping_sub(((cpu.x[8] as u32 as u64)))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10fc50ec cmp w8, w22
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[22] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc50f0 b.hi #0x10fc4e7c
if cpu.condition(8){pc=0x10fc4e7c;continue;}pc=0x10fc50f4;continue;
}
0x10fc50f4=>{
// 0x10fc50f4 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fc50f8 mov x1, x28
cpu.x[1]=(((cpu.x[28]))) as u64;
// 0x10fc50fc bl #0x10f9469c
cpu.x[30]=0x10fc5100;super::chunk14::f_10f9469c(cpu,rt,0x10f9469c)?;
pc=0x10fc5100;continue;
}
0x10fc5100=>{
// 0x10fc5100 ldur x8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc5104 ldrb w8, [x8, #0x6fc]
{let address=(cpu.x[8]).wrapping_add(0x6fcu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc5108 cbz w8, #0x10fc518c
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10fc518c;continue;}pc=0x10fc510c;continue;
}
0x10fc510c=>{
// 0x10fc510c ldr x8, [x28]
{let address=(cpu.x[28]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc5110 mov x0, x28
cpu.x[0]=(((cpu.x[28]))) as u64;
// 0x10fc5114 ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc5118 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc511c;super::dispatch(cpu,rt,target)?;}
// 0x10fc511c cbz x0, #0x10fc51c8
if (cpu.x[0]) == 0 {pc=0x10fc51c8;continue;}pc=0x10fc5120;continue;
}
0x10fc5120=>{
// 0x10fc5120 ldr x8, [x28]
{let address=(cpu.x[28]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc5124 mov x0, x28
cpu.x[0]=(((cpu.x[28]))) as u64;
// 0x10fc5128 ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc512c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc5130;super::dispatch(cpu,rt,target)?;}
// 0x10fc5130 bl #0x10fca234
cpu.x[30]=0x10fc5134;super::chunk2::f_10fca234(cpu,rt,0x10fca234)?;
// 0x10fc5134 b #0x10fc51cc
pc=0x10fc51cc;continue;
}
0x10fc5138=>{
// 0x10fc5138 sub w8, w22, w26
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[26] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fc513c mov x0, x28
cpu.x[0]=(((cpu.x[28]))) as u64;
// 0x10fc5140 bic w8, w8, w8, asr #31
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[8] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fc5144 stur w8, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc5148 bl #0x10f942a0
cpu.x[30]=0x10fc514c;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
pc=0x10fc514c;continue;
}
0x10fc514c=>{
// 0x10fc514c mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fc5150 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fc5154 bl #0x10f942a0
cpu.x[30]=0x10fc5158;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
pc=0x10fc5158;continue;
}
0x10fc5158=>{
// 0x10fc5158 sub w26, w26, w0
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[26]=(result) as u32 as u64;}
// 0x10fc515c mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fc5160 bl #0x10f9414c
cpu.x[30]=0x10fc5164;super::chunk3::f_10f9414c(cpu,rt,0x10f9414c)?;
pc=0x10fc5164;continue;
}
0x10fc5164=>{
// 0x10fc5164 tbz w0, #0, #0x10fc517c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fc517c;continue;}pc=0x10fc5168;continue;
}
0x10fc5168=>{
// 0x10fc5168 ldur w8, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc516c neg w8, w8
cpu.x[8]=(0u64.wrapping_sub(((cpu.x[8] as u32 as u64)))) as u32 as u64;
// 0x10fc5170 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc5174 b.lt #0x10fc4e7c
if cpu.condition(11){pc=0x10fc4e7c;continue;}pc=0x10fc5178;continue;
}
0x10fc5178=>{
// 0x10fc5178 b #0x10fc50a8
pc=0x10fc50a8;continue;
}
0x10fc517c=>{
// 0x10fc517c ldur w8, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc5180 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc5184 b.le #0x10fc50a8
if cpu.condition(13){pc=0x10fc50a8;continue;}pc=0x10fc5188;continue;
}
0x10fc5188=>{
// 0x10fc5188 b #0x10fc4e7c
pc=0x10fc4e7c;continue;
}
0x10fc518c=>{
// 0x10fc518c ldr w8, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc5190 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc5194 b.le #0x10fc51f4
if cpu.condition(13){pc=0x10fc51f4;continue;}pc=0x10fc5198;continue;
}
0x10fc5198=>{
// 0x10fc5198 ldur w23, [x29, #-0x2c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffd4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[23]=(v0) as u32 as u64;}
// 0x10fc519c stp w27, w0, [sp, #4]
{let address=(cpu.sp).wrapping_add(0x4u64);let v0=cpu.x[27] as u32 as u64;let v1=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10fc51a0 str x28, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[28];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fc51a4 b #0x10fc4e80
pc=0x10fc4e80;continue;
}
0x10fc51a8=>{
// 0x10fc51a8 mov w23, #1
cpu.x[23]=(((0x1u64))) as u32 as u64;
// 0x10fc51ac ldr x8, [x28]
{let address=(cpu.x[28]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc51b0 mov x0, x28
cpu.x[0]=(((cpu.x[28]))) as u64;
// 0x10fc51b4 ldr x8, [x8, #0x178]
{let address=(cpu.x[8]).wrapping_add(0x178u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc51b8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc51bc;super::dispatch(cpu,rt,target)?;}
// 0x10fc51bc ldur w8, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc51c0 tbnz w0, #0, #0x10fc504c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10fc504c;continue;}pc=0x10fc51c4;continue;
}
0x10fc51c4=>{
// 0x10fc51c4 b #0x10fc5030
pc=0x10fc5030;continue;
}
0x10fc51c8=>{
// 0x10fc51c8 mov w0, #0x7fffffff
cpu.x[0]=(((0x7fffffffu64))) as u32 as u64;
pc=0x10fc51cc;continue;
}
0x10fc51cc=>{
// 0x10fc51cc ldr w8, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc51d0 ldur w23, [x29, #-0x2c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffd4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[23]=(v0) as u32 as u64;}
// 0x10fc51d4 ldr w26, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[26]=(v0) as u32 as u64;}
// 0x10fc51d8 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc51dc csel w8, w0, w8, lt
cpu.x[8]=(if cpu.condition(11){((cpu.x[0] as u32 as u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10fc51e0 str w8, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc51e4 ldr x8, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc51e8 csel x8, x28, x8, lt
cpu.x[8]=(if cpu.condition(11){((cpu.x[28]))}else{((cpu.x[8]))}) as u64;
// 0x10fc51ec str x8, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fc51f0 b #0x10fc4e84
pc=0x10fc4e84;continue;
}
0x10fc51f4=>{
// 0x10fc51f4 ldr w9, [sp, #4]
{let address=(cpu.sp).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fc51f8 ldur w23, [x29, #-0x2c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffd4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[23]=(v0) as u32 as u64;}
// 0x10fc51fc cmp w27, w9
{let a=((cpu.x[27] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc5200 b.ge #0x10fc4e80
if cpu.condition(10){pc=0x10fc4e80;continue;}pc=0x10fc5204;continue;
}
0x10fc5204=>{
// 0x10fc5204 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc5208 b.lt #0x10fc4e80
if cpu.condition(11){pc=0x10fc4e80;continue;}pc=0x10fc520c;continue;
}
0x10fc520c=>{
// 0x10fc520c stp w27, w0, [sp, #4]
{let address=(cpu.sp).wrapping_add(0x4u64);let v0=cpu.x[27] as u32 as u64;let v1=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10fc5210 str x28, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[28];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fc5214 b #0x10fc4e80
pc=0x10fc4e80;continue;
}
0x10fc5218=>{
// 0x10fc5218 ldr x0, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fc521c sub x1, x29, #8
{let a=((cpu.x[29]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.x[1]=(result) as u64;}
// 0x10fc5220 bl #0x10f9abb8
cpu.x[30]=0x10fc5224;super::chunk9::f_10f9abb8(cpu,rt,0x10f9abb8)?;
pc=0x10fc5224;continue;
}
0x10fc5224=>{
// 0x10fc5224 ldr x22, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[22]=(v0) as u64;}
// 0x10fc5228 ldr x1, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10fc522c cbz x1, #0x10fc5238
if (cpu.x[1]) == 0 {pc=0x10fc5238;continue;}pc=0x10fc5230;continue;
}
0x10fc5230=>{
// 0x10fc5230 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc5234 b #0x10fc52d8
pc=0x10fc52d8;continue;
}
0x10fc5238=>{
// 0x10fc5238 ldr x8, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc523c cbz x8, #0x10fc5298
if (cpu.x[8]) == 0 {pc=0x10fc5298;continue;}pc=0x10fc5240;continue;
}
0x10fc5240=>{
// 0x10fc5240 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fc5244 bl #0x10f942c8
cpu.x[30]=0x10fc5248;super::chunk1::f_10f942c8(cpu,rt,0x10f942c8)?;
pc=0x10fc5248;continue;
}
0x10fc5248=>{
// 0x10fc5248 cbz x0, #0x10fc5298
if (cpu.x[0]) == 0 {pc=0x10fc5298;continue;}pc=0x10fc524c;continue;
}
0x10fc524c=>{
// 0x10fc524c mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fc5250 bl #0x10f942c8
cpu.x[30]=0x10fc5254;super::chunk1::f_10f942c8(cpu,rt,0x10f942c8)?;
pc=0x10fc5254;continue;
}
0x10fc5254=>{
// 0x10fc5254 ldr x1, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10fc5258 bl #0x10fd00c8
cpu.x[30]=0x10fc525c;super::chunk4::f_10fd00c8(cpu,rt,0x10fd00c8)?;
pc=0x10fc525c;continue;
}
0x10fc525c=>{
// 0x10fc525c tbz w0, #0, #0x10fc5298
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fc5298;continue;}pc=0x10fc5260;continue;
}
0x10fc5260=>{
// 0x10fc5260 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc5264 bl #0x10fd04a8
cpu.x[30]=0x10fc5268;super::chunk2::f_10fd04a8(cpu,rt,0x10fd04a8)?;
pc=0x10fc5268;continue;
}
0x10fc5268=>{
// 0x10fc5268 bl #0x10e0b6b4
cpu.x[30]=0x10fc526c;super::chunk8::f_10e0b6b4(cpu,rt,0x10e0b6b4)?;
pc=0x10fc526c;continue;
}
0x10fc526c=>{
// 0x10fc526c ldr x0, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fc5270 bl #0x10e16208
cpu.x[30]=0x10fc5274;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
pc=0x10fc5274;continue;
}
0x10fc5274=>{
// 0x10fc5274 ldrb w8, [x0, #0x3bf]
{let address=(cpu.x[0]).wrapping_add(0x3bfu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc5278 cbz w8, #0x10fc5298
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10fc5298;continue;}pc=0x10fc527c;continue;
}
0x10fc527c=>{
// 0x10fc527c ldr x1, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10fc5280 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc5284 mov w2, #1
cpu.x[2]=(((0x1u64))) as u32 as u64;
// 0x10fc5288 bl #0x10fc4008
cpu.x[30]=0x10fc528c;super::chunk5::f_10fc4008(cpu,rt,0x10fc4008)?;
pc=0x10fc528c;continue;
}
0x10fc528c=>{
// 0x10fc528c tbz w0, #0, #0x10fc5298
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fc5298;continue;}pc=0x10fc5290;continue;
}
0x10fc5290=>{
// 0x10fc5290 ldr x21, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10fc5294 cbnz x21, #0x10fc52a4
if (cpu.x[21]) != 0 {pc=0x10fc52a4;continue;}pc=0x10fc5298;continue;
}
0x10fc5298=>{
// 0x10fc5298 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc529c bl #0x10fc5398
cpu.x[30]=0x10fc52a0;super::chunk0::f_10fc5398(cpu,rt,0x10fc5398)?;
pc=0x10fc52a0;continue;
}
0x10fc52a0=>{
// 0x10fc52a0 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
pc=0x10fc52a4;continue;
}
0x10fc52a4=>{
// 0x10fc52a4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc52a8 mov x1, x21
cpu.x[1]=(((cpu.x[21]))) as u64;
// 0x10fc52ac mov w2, #1
cpu.x[2]=(((0x1u64))) as u32 as u64;
// 0x10fc52b0 bl #0x10fc4008
cpu.x[30]=0x10fc52b4;super::chunk5::f_10fc4008(cpu,rt,0x10fc4008)?;
pc=0x10fc52b4;continue;
}
0x10fc52b4=>{
// 0x10fc52b4 tbz w0, #0, #0x10fc52c4
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fc52c4;continue;}pc=0x10fc52b8;continue;
}
0x10fc52b8=>{
// 0x10fc52b8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc52bc mov x1, x21
cpu.x[1]=(((cpu.x[21]))) as u64;
// 0x10fc52c0 b #0x10fc52d8
pc=0x10fc52d8;continue;
}
0x10fc52c4=>{
// 0x10fc52c4 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10fc52c8 bl #0x10d36084
cpu.x[30]=0x10fc52cc;super::chunk6::f_10d36084(cpu,rt,0x10d36084)?;
pc=0x10fc52cc;continue;
}
0x10fc52cc=>{
// 0x10fc52cc tbz w0, #0, #0x10fc52e8
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fc52e8;continue;}pc=0x10fc52d0;continue;
}
0x10fc52d0=>{
// 0x10fc52d0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc52d4 mov x1, xzr
cpu.x[1]=(((0u64))) as u64;
pc=0x10fc52d8;continue;
}
0x10fc52d8=>{
// 0x10fc52d8 mov w2, wzr
cpu.x[2]=(((0u64))) as u32 as u64;
// 0x10fc52dc mov w3, #1
cpu.x[3]=(((0x1u64))) as u32 as u64;
// 0x10fc52e0 mov w4, wzr
cpu.x[4]=(((0u64))) as u32 as u64;
// 0x10fc52e4 bl #0x10fc3cb4
cpu.x[30]=0x10fc52e8;super::chunk12::f_10fc3cb4(cpu,rt,0x10fc3cb4)?;
pc=0x10fc52e8;continue;
}
0x10fc52e8=>{
// 0x10fc52e8 ldp x20, x19, [sp, #0xb0]
{let address=(cpu.sp).wrapping_add(0xb0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fc52ec ldp x22, x21, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fc52f0 ldp x24, x23, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fc52f4 ldp x26, x25, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fc52f8 ldp x28, x27, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fc52fc ldp x29, x30, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fc5300 add sp, sp, #0xc0
{let a=((cpu.sp));let b=((0xc0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fc5304 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fc4c38")),}}}
#[inline]
pub(super) unsafe fn f_10fcbf74(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fcbf74=>{
// 0x10fcbf74 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fcbf78 str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fcbf7c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fcbf80 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10fcbf84 ldr x0, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fcbf88 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcbf8c ldr x8, [x8, #0xf0]
{let address=(cpu.x[8]).wrapping_add(0xf0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcbf90 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fcbf94;super::dispatch(cpu,rt,target)?;}
pc=0x10fcbf94;continue;
}
0x10fcbf94=>{
// 0x10fcbf94 ldr w8, [x19, #0x14]
{let address=(cpu.x[19]).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcbf98 mov w9, #0x851f
cpu.x[9]=(((0x851fu64))) as u32 as u64;
// 0x10fcbf9c movk w9, #0x51eb, lsl #16
cpu.x[9]=((cpu.x[9] as u32 as u64 & !(0xffff0000u64)) | 0x51eb0000u64) as u32 as u64;
// 0x10fcbfa0 mul w8, w8, w0
cpu.x[8]=((((cpu.x[8] as u32 as u64))).wrapping_mul(((cpu.x[0] as u32 as u64)))) as u32 as u64;
// 0x10fcbfa4 smull x8, w8, w9
cpu.x[8]=(((((cpu.x[8] as u32 as u64))) as i32 as i64).wrapping_mul((((cpu.x[9] as u32 as u64))) as i32 as i64) as u64) as u64;
// 0x10fcbfa8 lsr x9, x8, #0x3f
cpu.x[9]=(((((cpu.x[8]))) as u64).wrapping_shr(((((0x3fu64))) & 63) as u32) as u64) as u64;
// 0x10fcbfac asr x8, x8, #0x25
cpu.x[8]=(((((cpu.x[8]))) as i64).wrapping_shr(((((0x25u64))) & 63) as u32) as u64) as u64;
// 0x10fcbfb0 add w0, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u32 as u64;}
// 0x10fcbfb4 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10fcbfb8 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10fcbfbc ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fcbf74")),}}}
#[inline]
pub(super) unsafe fn f_10fcdefc(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fcdefc=>{
// 0x10fcdefc sub sp, sp, #0xb0
{let a=((cpu.sp));let b=((0xb0u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x10fcdf00 stp x29, x30, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fcdf04 stp x28, x27, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=cpu.x[28];let v1=cpu.x[27];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fcdf08 stp x26, x25, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=cpu.x[26];let v1=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fcdf0c stp x24, x23, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fcdf10 stp x22, x21, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fcdf14 stp x20, x19, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fcdf18 add x29, sp, #0x50
{let a=((cpu.sp));let b=((0x50u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x10fcdf1c ldr x20, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10fcdf20 mov x25, x0
cpu.x[25]=(((cpu.x[0]))) as u64;
// 0x10fcdf24 mov w26, w2
cpu.x[26]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x10fcdf28 mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10fcdf2c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcdf30 bl #0x10f93d2c
cpu.x[30]=0x10fcdf34;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
{// 0x10fcdf34 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10fcdf38 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcdf3c bl #0x10f94298
cpu.x[30]=0x10fcdf40;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
{// 0x10fcdf40 mov w22, w0
cpu.x[22]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fcdf44 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcdf48 bl #0x10f942a0
cpu.x[30]=0x10fcdf4c;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
{// 0x10fcdf4c stur w0, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fcdf50 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fcdf54 bl #0x10f94010
cpu.x[30]=0x10fcdf58;super::chunk6::f_10f94010(cpu,rt,0x10f94010)?;
{// 0x10fcdf58 mov w28, w0
cpu.x[28]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fcdf5c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fcdf60 stur x19, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fcdf64 bl #0x10f94030
cpu.x[30]=0x10fcdf68;super::chunk14::f_10f94030(cpu,rt,0x10f94030)?;
{// 0x10fcdf68 ldr x23, [x21, #0x10]
{let address=(cpu.x[21]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10fcdf6c mov w27, w0
cpu.x[27]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fcdf70 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fcdf74 stur x21, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fcdf78 bl #0x10d37e08
cpu.x[30]=0x10fcdf7c;super::chunk15::f_10d37e08(cpu,rt,0x10d37e08)?;
{// 0x10fcdf7c mov w8, #0x4dd3
cpu.x[8]=(((0x4dd3u64))) as u32 as u64;
// 0x10fcdf80 str x0, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fcdf84 mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10fcdf88 movk w8, #0x1062, lsl #16
cpu.x[8]=((cpu.x[8] as u32 as u64 & !(0xffff0000u64)) | 0x10620000u64) as u32 as u64;
// 0x10fcdf8c smull x8, w26, w8
cpu.x[8]=(((((cpu.x[26] as u32 as u64))) as i32 as i64).wrapping_mul((((cpu.x[8] as u32 as u64))) as i32 as i64) as u64) as u64;
// 0x10fcdf90 lsr x9, x8, #0x3f
cpu.x[9]=(((((cpu.x[8]))) as u64).wrapping_shr(((((0x3fu64))) & 63) as u32) as u64) as u64;
// 0x10fcdf94 asr x8, x8, #0x25
cpu.x[8]=(((((cpu.x[8]))) as i64).wrapping_shr(((((0x25u64))) & 63) as u32) as u64) as u64;
// 0x10fcdf98 add w8, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fcdf9c add w19, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[19]=(result) as u32 as u64;}
// 0x10fcdfa0 ldp w9, w8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10fcdfa4 add w10, w28, w19
{let a=((cpu.x[28] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10fcdfa8 add w11, w27, w19
{let a=((cpu.x[27] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u32 as u64;}
// 0x10fcdfac sub w24, w27, w19
{let a=((cpu.x[27] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[24]=(result) as u32 as u64;}
// 0x10fcdfb0 sub w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fcdfb4 sub w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fcdfb8 cmp w10, w9
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcdfbc csel w9, w10, w9, lt
cpu.x[9]=(if cpu.condition(11){((cpu.x[10] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10fcdfc0 cmp w11, w8
{let a=((cpu.x[11] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcdfc4 csel w8, w11, w8, lt
cpu.x[8]=(if cpu.condition(11){((cpu.x[11] as u32 as u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10fcdfc8 stp w9, w8, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[9] as u32 as u64;let v1=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10fcdfcc bl #0x10fd04a8
cpu.x[30]=0x10fcdfd0;super::chunk2::f_10fd04a8(cpu,rt,0x10fd04a8)?;
{// 0x10fcdfd0 bl #0x10e0b6b4
cpu.x[30]=0x10fcdfd4;super::chunk8::f_10e0b6b4(cpu,rt,0x10e0b6b4)?;
{// 0x10fcdfd4 ldr x0, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fcdfd8 bl #0x10e16208
cpu.x[30]=0x10fcdfdc;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
{// 0x10fcdfdc ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcdfe0 mov x27, x0
cpu.x[27]=(((cpu.x[0]))) as u64;
// 0x10fcdfe4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcdfe8 mov x21, x20
cpu.x[21]=(((cpu.x[20]))) as u64;
// 0x10fcdfec ldr x8, [x8, #0x108]
{let address=(cpu.x[8]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcdff0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fcdff4;super::dispatch(cpu,rt,target)?;}
{// 0x10fcdff4 tbz w0, #0, #0x10fce03c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x10fce03c mov w20, #1
cpu.x[20]=(((0x1u64))) as u32 as u64;
// 0x10fce040 ldr x8, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fce044 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fce048 bic w25, w24, w24, asr #31
{let a=((cpu.x[24] as u32 as u64));let b=(((cpu.x[24] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[25]=(result) as u32 as u64;}
// 0x10fce04c ldr x8, [x8, #0x108]
{let address=(cpu.x[8]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fce050 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fce054;super::dispatch(cpu,rt,target)?;}
{// 0x10fce054 tbz w0, #0, #0x10fce01c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce01c;continue;}{// 0x10fce058 str wzr, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce05c ldr w8, [sp, #0x14]
{let address=(cpu.sp).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce060 cmp w25, w8
{let a=((cpu.x[25] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce064 b.gt #0x10fce338
if cpu.condition(12){{// 0x10fce338 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
pc=0x10fce33c;continue;}}pc=0x10fce068;continue;}}}}{// 0x10fcdff8 ldrb w8, [x27, #0x80a]
{let address=(cpu.x[27]).wrapping_add(0x80au64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcdffc cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
pc=0x10fce000;continue;}}}}}}}}}}}
}
0x10fcdf34=>{
// 0x10fcdf34 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10fcdf38 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcdf3c bl #0x10f94298
cpu.x[30]=0x10fcdf40;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
{// 0x10fcdf40 mov w22, w0
cpu.x[22]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fcdf44 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcdf48 bl #0x10f942a0
cpu.x[30]=0x10fcdf4c;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
{// 0x10fcdf4c stur w0, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fcdf50 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fcdf54 bl #0x10f94010
cpu.x[30]=0x10fcdf58;super::chunk6::f_10f94010(cpu,rt,0x10f94010)?;
{// 0x10fcdf58 mov w28, w0
cpu.x[28]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fcdf5c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fcdf60 stur x19, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fcdf64 bl #0x10f94030
cpu.x[30]=0x10fcdf68;super::chunk14::f_10f94030(cpu,rt,0x10f94030)?;
{// 0x10fcdf68 ldr x23, [x21, #0x10]
{let address=(cpu.x[21]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10fcdf6c mov w27, w0
cpu.x[27]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fcdf70 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fcdf74 stur x21, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fcdf78 bl #0x10d37e08
cpu.x[30]=0x10fcdf7c;super::chunk15::f_10d37e08(cpu,rt,0x10d37e08)?;
{// 0x10fcdf7c mov w8, #0x4dd3
cpu.x[8]=(((0x4dd3u64))) as u32 as u64;
// 0x10fcdf80 str x0, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fcdf84 mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10fcdf88 movk w8, #0x1062, lsl #16
cpu.x[8]=((cpu.x[8] as u32 as u64 & !(0xffff0000u64)) | 0x10620000u64) as u32 as u64;
// 0x10fcdf8c smull x8, w26, w8
cpu.x[8]=(((((cpu.x[26] as u32 as u64))) as i32 as i64).wrapping_mul((((cpu.x[8] as u32 as u64))) as i32 as i64) as u64) as u64;
// 0x10fcdf90 lsr x9, x8, #0x3f
cpu.x[9]=(((((cpu.x[8]))) as u64).wrapping_shr(((((0x3fu64))) & 63) as u32) as u64) as u64;
// 0x10fcdf94 asr x8, x8, #0x25
cpu.x[8]=(((((cpu.x[8]))) as i64).wrapping_shr(((((0x25u64))) & 63) as u32) as u64) as u64;
// 0x10fcdf98 add w8, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fcdf9c add w19, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[19]=(result) as u32 as u64;}
// 0x10fcdfa0 ldp w9, w8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10fcdfa4 add w10, w28, w19
{let a=((cpu.x[28] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10fcdfa8 add w11, w27, w19
{let a=((cpu.x[27] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u32 as u64;}
// 0x10fcdfac sub w24, w27, w19
{let a=((cpu.x[27] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[24]=(result) as u32 as u64;}
// 0x10fcdfb0 sub w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fcdfb4 sub w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fcdfb8 cmp w10, w9
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcdfbc csel w9, w10, w9, lt
cpu.x[9]=(if cpu.condition(11){((cpu.x[10] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10fcdfc0 cmp w11, w8
{let a=((cpu.x[11] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcdfc4 csel w8, w11, w8, lt
cpu.x[8]=(if cpu.condition(11){((cpu.x[11] as u32 as u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10fcdfc8 stp w9, w8, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[9] as u32 as u64;let v1=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10fcdfcc bl #0x10fd04a8
cpu.x[30]=0x10fcdfd0;super::chunk2::f_10fd04a8(cpu,rt,0x10fd04a8)?;
{// 0x10fcdfd0 bl #0x10e0b6b4
cpu.x[30]=0x10fcdfd4;super::chunk8::f_10e0b6b4(cpu,rt,0x10e0b6b4)?;
{// 0x10fcdfd4 ldr x0, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fcdfd8 bl #0x10e16208
cpu.x[30]=0x10fcdfdc;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
{// 0x10fcdfdc ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcdfe0 mov x27, x0
cpu.x[27]=(((cpu.x[0]))) as u64;
// 0x10fcdfe4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcdfe8 mov x21, x20
cpu.x[21]=(((cpu.x[20]))) as u64;
// 0x10fcdfec ldr x8, [x8, #0x108]
{let address=(cpu.x[8]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcdff0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fcdff4;super::dispatch(cpu,rt,target)?;}
{// 0x10fcdff4 tbz w0, #0, #0x10fce03c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x10fce03c mov w20, #1
cpu.x[20]=(((0x1u64))) as u32 as u64;
// 0x10fce040 ldr x8, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fce044 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fce048 bic w25, w24, w24, asr #31
{let a=((cpu.x[24] as u32 as u64));let b=(((cpu.x[24] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[25]=(result) as u32 as u64;}
// 0x10fce04c ldr x8, [x8, #0x108]
{let address=(cpu.x[8]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fce050 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fce054;super::dispatch(cpu,rt,target)?;}
{// 0x10fce054 tbz w0, #0, #0x10fce01c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce01c;continue;}{// 0x10fce058 str wzr, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce05c ldr w8, [sp, #0x14]
{let address=(cpu.sp).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce060 cmp w25, w8
{let a=((cpu.x[25] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce064 b.gt #0x10fce338
if cpu.condition(12){{// 0x10fce338 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
pc=0x10fce33c;continue;}}{// 0x10fce068 sub w8, w28, w19
{let a=((cpu.x[28] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fce06c bic w9, w8, w8, asr #31
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[8] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[9]=(result) as u32 as u64;}
// 0x10fce070 ldr w8, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce074 cmp w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce078 str w9, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce07c b.gt #0x10fce338
if cpu.condition(12){pc=0x10fce338;continue;}pc=0x10fce080;continue;}}}}}pc=0x10fcdff8;continue;}}}}}}}}}
}
0x10fcdf40=>{
// 0x10fcdf40 mov w22, w0
cpu.x[22]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fcdf44 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcdf48 bl #0x10f942a0
cpu.x[30]=0x10fcdf4c;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
{// 0x10fcdf4c stur w0, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fcdf50 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fcdf54 bl #0x10f94010
cpu.x[30]=0x10fcdf58;super::chunk6::f_10f94010(cpu,rt,0x10f94010)?;
{// 0x10fcdf58 mov w28, w0
cpu.x[28]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fcdf5c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fcdf60 stur x19, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fcdf64 bl #0x10f94030
cpu.x[30]=0x10fcdf68;super::chunk14::f_10f94030(cpu,rt,0x10f94030)?;
{// 0x10fcdf68 ldr x23, [x21, #0x10]
{let address=(cpu.x[21]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10fcdf6c mov w27, w0
cpu.x[27]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fcdf70 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fcdf74 stur x21, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fcdf78 bl #0x10d37e08
cpu.x[30]=0x10fcdf7c;super::chunk15::f_10d37e08(cpu,rt,0x10d37e08)?;
{// 0x10fcdf7c mov w8, #0x4dd3
cpu.x[8]=(((0x4dd3u64))) as u32 as u64;
// 0x10fcdf80 str x0, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fcdf84 mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10fcdf88 movk w8, #0x1062, lsl #16
cpu.x[8]=((cpu.x[8] as u32 as u64 & !(0xffff0000u64)) | 0x10620000u64) as u32 as u64;
// 0x10fcdf8c smull x8, w26, w8
cpu.x[8]=(((((cpu.x[26] as u32 as u64))) as i32 as i64).wrapping_mul((((cpu.x[8] as u32 as u64))) as i32 as i64) as u64) as u64;
// 0x10fcdf90 lsr x9, x8, #0x3f
cpu.x[9]=(((((cpu.x[8]))) as u64).wrapping_shr(((((0x3fu64))) & 63) as u32) as u64) as u64;
// 0x10fcdf94 asr x8, x8, #0x25
cpu.x[8]=(((((cpu.x[8]))) as i64).wrapping_shr(((((0x25u64))) & 63) as u32) as u64) as u64;
// 0x10fcdf98 add w8, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fcdf9c add w19, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[19]=(result) as u32 as u64;}
// 0x10fcdfa0 ldp w9, w8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10fcdfa4 add w10, w28, w19
{let a=((cpu.x[28] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10fcdfa8 add w11, w27, w19
{let a=((cpu.x[27] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u32 as u64;}
// 0x10fcdfac sub w24, w27, w19
{let a=((cpu.x[27] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[24]=(result) as u32 as u64;}
// 0x10fcdfb0 sub w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fcdfb4 sub w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fcdfb8 cmp w10, w9
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcdfbc csel w9, w10, w9, lt
cpu.x[9]=(if cpu.condition(11){((cpu.x[10] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10fcdfc0 cmp w11, w8
{let a=((cpu.x[11] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcdfc4 csel w8, w11, w8, lt
cpu.x[8]=(if cpu.condition(11){((cpu.x[11] as u32 as u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10fcdfc8 stp w9, w8, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[9] as u32 as u64;let v1=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10fcdfcc bl #0x10fd04a8
cpu.x[30]=0x10fcdfd0;super::chunk2::f_10fd04a8(cpu,rt,0x10fd04a8)?;
{// 0x10fcdfd0 bl #0x10e0b6b4
cpu.x[30]=0x10fcdfd4;super::chunk8::f_10e0b6b4(cpu,rt,0x10e0b6b4)?;
{// 0x10fcdfd4 ldr x0, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fcdfd8 bl #0x10e16208
cpu.x[30]=0x10fcdfdc;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
{// 0x10fcdfdc ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcdfe0 mov x27, x0
cpu.x[27]=(((cpu.x[0]))) as u64;
// 0x10fcdfe4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcdfe8 mov x21, x20
cpu.x[21]=(((cpu.x[20]))) as u64;
// 0x10fcdfec ldr x8, [x8, #0x108]
{let address=(cpu.x[8]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcdff0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fcdff4;super::dispatch(cpu,rt,target)?;}
{// 0x10fcdff4 tbz w0, #0, #0x10fce03c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x10fce03c mov w20, #1
cpu.x[20]=(((0x1u64))) as u32 as u64;
// 0x10fce040 ldr x8, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fce044 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fce048 bic w25, w24, w24, asr #31
{let a=((cpu.x[24] as u32 as u64));let b=(((cpu.x[24] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[25]=(result) as u32 as u64;}
// 0x10fce04c ldr x8, [x8, #0x108]
{let address=(cpu.x[8]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fce050 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fce054;super::dispatch(cpu,rt,target)?;}
{// 0x10fce054 tbz w0, #0, #0x10fce01c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce01c;continue;}{// 0x10fce058 str wzr, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce05c ldr w8, [sp, #0x14]
{let address=(cpu.sp).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce060 cmp w25, w8
{let a=((cpu.x[25] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce064 b.gt #0x10fce338
if cpu.condition(12){{// 0x10fce338 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
{// 0x10fce33c ldp x20, x19, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fce340 ldp x22, x21, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fce344 ldp x24, x23, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fce348 ldp x26, x25, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fce34c ldp x28, x27, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fce350 ldp x29, x30, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fce354 add sp, sp, #0xb0
{let a=((cpu.sp));let b=((0xb0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fce358 ret
cpu.pc=cpu.x[30];return Ok(());}}}pc=0x10fce068;continue;}}}}pc=0x10fcdff8;continue;}}}}}}}}
}
0x10fcdf4c=>{
// 0x10fcdf4c stur w0, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fcdf50 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fcdf54 bl #0x10f94010
cpu.x[30]=0x10fcdf58;super::chunk6::f_10f94010(cpu,rt,0x10f94010)?;
{// 0x10fcdf58 mov w28, w0
cpu.x[28]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fcdf5c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fcdf60 stur x19, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fcdf64 bl #0x10f94030
cpu.x[30]=0x10fcdf68;super::chunk14::f_10f94030(cpu,rt,0x10f94030)?;
{// 0x10fcdf68 ldr x23, [x21, #0x10]
{let address=(cpu.x[21]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10fcdf6c mov w27, w0
cpu.x[27]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fcdf70 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fcdf74 stur x21, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fcdf78 bl #0x10d37e08
cpu.x[30]=0x10fcdf7c;super::chunk15::f_10d37e08(cpu,rt,0x10d37e08)?;
{// 0x10fcdf7c mov w8, #0x4dd3
cpu.x[8]=(((0x4dd3u64))) as u32 as u64;
// 0x10fcdf80 str x0, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fcdf84 mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10fcdf88 movk w8, #0x1062, lsl #16
cpu.x[8]=((cpu.x[8] as u32 as u64 & !(0xffff0000u64)) | 0x10620000u64) as u32 as u64;
// 0x10fcdf8c smull x8, w26, w8
cpu.x[8]=(((((cpu.x[26] as u32 as u64))) as i32 as i64).wrapping_mul((((cpu.x[8] as u32 as u64))) as i32 as i64) as u64) as u64;
// 0x10fcdf90 lsr x9, x8, #0x3f
cpu.x[9]=(((((cpu.x[8]))) as u64).wrapping_shr(((((0x3fu64))) & 63) as u32) as u64) as u64;
// 0x10fcdf94 asr x8, x8, #0x25
cpu.x[8]=(((((cpu.x[8]))) as i64).wrapping_shr(((((0x25u64))) & 63) as u32) as u64) as u64;
// 0x10fcdf98 add w8, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fcdf9c add w19, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[19]=(result) as u32 as u64;}
// 0x10fcdfa0 ldp w9, w8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10fcdfa4 add w10, w28, w19
{let a=((cpu.x[28] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10fcdfa8 add w11, w27, w19
{let a=((cpu.x[27] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u32 as u64;}
// 0x10fcdfac sub w24, w27, w19
{let a=((cpu.x[27] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[24]=(result) as u32 as u64;}
// 0x10fcdfb0 sub w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fcdfb4 sub w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fcdfb8 cmp w10, w9
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcdfbc csel w9, w10, w9, lt
cpu.x[9]=(if cpu.condition(11){((cpu.x[10] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10fcdfc0 cmp w11, w8
{let a=((cpu.x[11] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcdfc4 csel w8, w11, w8, lt
cpu.x[8]=(if cpu.condition(11){((cpu.x[11] as u32 as u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10fcdfc8 stp w9, w8, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[9] as u32 as u64;let v1=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10fcdfcc bl #0x10fd04a8
cpu.x[30]=0x10fcdfd0;super::chunk2::f_10fd04a8(cpu,rt,0x10fd04a8)?;
{// 0x10fcdfd0 bl #0x10e0b6b4
cpu.x[30]=0x10fcdfd4;super::chunk8::f_10e0b6b4(cpu,rt,0x10e0b6b4)?;
{// 0x10fcdfd4 ldr x0, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fcdfd8 bl #0x10e16208
cpu.x[30]=0x10fcdfdc;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
{// 0x10fcdfdc ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcdfe0 mov x27, x0
cpu.x[27]=(((cpu.x[0]))) as u64;
// 0x10fcdfe4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcdfe8 mov x21, x20
cpu.x[21]=(((cpu.x[20]))) as u64;
// 0x10fcdfec ldr x8, [x8, #0x108]
{let address=(cpu.x[8]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcdff0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fcdff4;super::dispatch(cpu,rt,target)?;}
{// 0x10fcdff4 tbz w0, #0, #0x10fce03c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x10fce03c mov w20, #1
cpu.x[20]=(((0x1u64))) as u32 as u64;
// 0x10fce040 ldr x8, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fce044 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fce048 bic w25, w24, w24, asr #31
{let a=((cpu.x[24] as u32 as u64));let b=(((cpu.x[24] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[25]=(result) as u32 as u64;}
// 0x10fce04c ldr x8, [x8, #0x108]
{let address=(cpu.x[8]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fce050 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fce054;super::dispatch(cpu,rt,target)?;}
{// 0x10fce054 tbz w0, #0, #0x10fce01c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce01c;continue;}{// 0x10fce058 str wzr, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce05c ldr w8, [sp, #0x14]
{let address=(cpu.sp).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce060 cmp w25, w8
{let a=((cpu.x[25] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce064 b.gt #0x10fce338
if cpu.condition(12){{// 0x10fce338 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
{// 0x10fce33c ldp x20, x19, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fce340 ldp x22, x21, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fce344 ldp x24, x23, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fce348 ldp x26, x25, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fce34c ldp x28, x27, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fce350 ldp x29, x30, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fce354 add sp, sp, #0xb0
{let a=((cpu.sp));let b=((0xb0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fce358 ret
cpu.pc=cpu.x[30];return Ok(());}}}pc=0x10fce068;continue;}}}}{// 0x10fcdff8 ldrb w8, [x27, #0x80a]
{let address=(cpu.x[27]).wrapping_add(0x80au64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcdffc cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
pc=0x10fce000;continue;}}}}}}}}
}
0x10fcdf58=>{
// 0x10fcdf58 mov w28, w0
cpu.x[28]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fcdf5c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fcdf60 stur x19, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fcdf64 bl #0x10f94030
cpu.x[30]=0x10fcdf68;super::chunk14::f_10f94030(cpu,rt,0x10f94030)?;
{// 0x10fcdf68 ldr x23, [x21, #0x10]
{let address=(cpu.x[21]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10fcdf6c mov w27, w0
cpu.x[27]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fcdf70 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fcdf74 stur x21, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fcdf78 bl #0x10d37e08
cpu.x[30]=0x10fcdf7c;super::chunk15::f_10d37e08(cpu,rt,0x10d37e08)?;
{// 0x10fcdf7c mov w8, #0x4dd3
cpu.x[8]=(((0x4dd3u64))) as u32 as u64;
// 0x10fcdf80 str x0, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fcdf84 mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10fcdf88 movk w8, #0x1062, lsl #16
cpu.x[8]=((cpu.x[8] as u32 as u64 & !(0xffff0000u64)) | 0x10620000u64) as u32 as u64;
// 0x10fcdf8c smull x8, w26, w8
cpu.x[8]=(((((cpu.x[26] as u32 as u64))) as i32 as i64).wrapping_mul((((cpu.x[8] as u32 as u64))) as i32 as i64) as u64) as u64;
// 0x10fcdf90 lsr x9, x8, #0x3f
cpu.x[9]=(((((cpu.x[8]))) as u64).wrapping_shr(((((0x3fu64))) & 63) as u32) as u64) as u64;
// 0x10fcdf94 asr x8, x8, #0x25
cpu.x[8]=(((((cpu.x[8]))) as i64).wrapping_shr(((((0x25u64))) & 63) as u32) as u64) as u64;
// 0x10fcdf98 add w8, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fcdf9c add w19, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[19]=(result) as u32 as u64;}
// 0x10fcdfa0 ldp w9, w8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10fcdfa4 add w10, w28, w19
{let a=((cpu.x[28] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10fcdfa8 add w11, w27, w19
{let a=((cpu.x[27] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u32 as u64;}
// 0x10fcdfac sub w24, w27, w19
{let a=((cpu.x[27] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[24]=(result) as u32 as u64;}
// 0x10fcdfb0 sub w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fcdfb4 sub w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fcdfb8 cmp w10, w9
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcdfbc csel w9, w10, w9, lt
cpu.x[9]=(if cpu.condition(11){((cpu.x[10] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10fcdfc0 cmp w11, w8
{let a=((cpu.x[11] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcdfc4 csel w8, w11, w8, lt
cpu.x[8]=(if cpu.condition(11){((cpu.x[11] as u32 as u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10fcdfc8 stp w9, w8, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[9] as u32 as u64;let v1=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10fcdfcc bl #0x10fd04a8
cpu.x[30]=0x10fcdfd0;super::chunk2::f_10fd04a8(cpu,rt,0x10fd04a8)?;
{// 0x10fcdfd0 bl #0x10e0b6b4
cpu.x[30]=0x10fcdfd4;super::chunk8::f_10e0b6b4(cpu,rt,0x10e0b6b4)?;
{// 0x10fcdfd4 ldr x0, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fcdfd8 bl #0x10e16208
cpu.x[30]=0x10fcdfdc;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
{// 0x10fcdfdc ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcdfe0 mov x27, x0
cpu.x[27]=(((cpu.x[0]))) as u64;
// 0x10fcdfe4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcdfe8 mov x21, x20
cpu.x[21]=(((cpu.x[20]))) as u64;
// 0x10fcdfec ldr x8, [x8, #0x108]
{let address=(cpu.x[8]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcdff0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fcdff4;super::dispatch(cpu,rt,target)?;}
{// 0x10fcdff4 tbz w0, #0, #0x10fce03c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x10fce03c mov w20, #1
cpu.x[20]=(((0x1u64))) as u32 as u64;
// 0x10fce040 ldr x8, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fce044 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fce048 bic w25, w24, w24, asr #31
{let a=((cpu.x[24] as u32 as u64));let b=(((cpu.x[24] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[25]=(result) as u32 as u64;}
// 0x10fce04c ldr x8, [x8, #0x108]
{let address=(cpu.x[8]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fce050 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fce054;super::dispatch(cpu,rt,target)?;}
{// 0x10fce054 tbz w0, #0, #0x10fce01c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce01c;continue;}{// 0x10fce058 str wzr, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce05c ldr w8, [sp, #0x14]
{let address=(cpu.sp).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce060 cmp w25, w8
{let a=((cpu.x[25] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce064 b.gt #0x10fce338
if cpu.condition(12){{// 0x10fce338 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
{// 0x10fce33c ldp x20, x19, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fce340 ldp x22, x21, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fce344 ldp x24, x23, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fce348 ldp x26, x25, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fce34c ldp x28, x27, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fce350 ldp x29, x30, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fce354 add sp, sp, #0xb0
{let a=((cpu.sp));let b=((0xb0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fce358 ret
cpu.pc=cpu.x[30];return Ok(());}}}{// 0x10fce068 sub w8, w28, w19
{let a=((cpu.x[28] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fce06c bic w9, w8, w8, asr #31
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[8] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[9]=(result) as u32 as u64;}
// 0x10fce070 ldr w8, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce074 cmp w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce078 str w9, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce07c b.gt #0x10fce338
if cpu.condition(12){{// 0x10fce338 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
pc=0x10fce33c;continue;}}pc=0x10fce080;continue;}}}}}pc=0x10fcdff8;continue;}}}}}}
}
0x10fcdf68=>{
// 0x10fcdf68 ldr x23, [x21, #0x10]
{let address=(cpu.x[21]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10fcdf6c mov w27, w0
cpu.x[27]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fcdf70 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fcdf74 stur x21, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fcdf78 bl #0x10d37e08
cpu.x[30]=0x10fcdf7c;super::chunk15::f_10d37e08(cpu,rt,0x10d37e08)?;
{// 0x10fcdf7c mov w8, #0x4dd3
cpu.x[8]=(((0x4dd3u64))) as u32 as u64;
// 0x10fcdf80 str x0, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fcdf84 mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10fcdf88 movk w8, #0x1062, lsl #16
cpu.x[8]=((cpu.x[8] as u32 as u64 & !(0xffff0000u64)) | 0x10620000u64) as u32 as u64;
// 0x10fcdf8c smull x8, w26, w8
cpu.x[8]=(((((cpu.x[26] as u32 as u64))) as i32 as i64).wrapping_mul((((cpu.x[8] as u32 as u64))) as i32 as i64) as u64) as u64;
// 0x10fcdf90 lsr x9, x8, #0x3f
cpu.x[9]=(((((cpu.x[8]))) as u64).wrapping_shr(((((0x3fu64))) & 63) as u32) as u64) as u64;
// 0x10fcdf94 asr x8, x8, #0x25
cpu.x[8]=(((((cpu.x[8]))) as i64).wrapping_shr(((((0x25u64))) & 63) as u32) as u64) as u64;
// 0x10fcdf98 add w8, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fcdf9c add w19, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[19]=(result) as u32 as u64;}
// 0x10fcdfa0 ldp w9, w8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10fcdfa4 add w10, w28, w19
{let a=((cpu.x[28] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10fcdfa8 add w11, w27, w19
{let a=((cpu.x[27] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u32 as u64;}
// 0x10fcdfac sub w24, w27, w19
{let a=((cpu.x[27] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[24]=(result) as u32 as u64;}
// 0x10fcdfb0 sub w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fcdfb4 sub w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fcdfb8 cmp w10, w9
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcdfbc csel w9, w10, w9, lt
cpu.x[9]=(if cpu.condition(11){((cpu.x[10] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10fcdfc0 cmp w11, w8
{let a=((cpu.x[11] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcdfc4 csel w8, w11, w8, lt
cpu.x[8]=(if cpu.condition(11){((cpu.x[11] as u32 as u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10fcdfc8 stp w9, w8, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[9] as u32 as u64;let v1=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10fcdfcc bl #0x10fd04a8
cpu.x[30]=0x10fcdfd0;super::chunk2::f_10fd04a8(cpu,rt,0x10fd04a8)?;
{// 0x10fcdfd0 bl #0x10e0b6b4
cpu.x[30]=0x10fcdfd4;super::chunk8::f_10e0b6b4(cpu,rt,0x10e0b6b4)?;
{// 0x10fcdfd4 ldr x0, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fcdfd8 bl #0x10e16208
cpu.x[30]=0x10fcdfdc;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
{// 0x10fcdfdc ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcdfe0 mov x27, x0
cpu.x[27]=(((cpu.x[0]))) as u64;
// 0x10fcdfe4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcdfe8 mov x21, x20
cpu.x[21]=(((cpu.x[20]))) as u64;
// 0x10fcdfec ldr x8, [x8, #0x108]
{let address=(cpu.x[8]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcdff0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fcdff4;super::dispatch(cpu,rt,target)?;}
{// 0x10fcdff4 tbz w0, #0, #0x10fce03c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x10fce03c mov w20, #1
cpu.x[20]=(((0x1u64))) as u32 as u64;
// 0x10fce040 ldr x8, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fce044 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fce048 bic w25, w24, w24, asr #31
{let a=((cpu.x[24] as u32 as u64));let b=(((cpu.x[24] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[25]=(result) as u32 as u64;}
// 0x10fce04c ldr x8, [x8, #0x108]
{let address=(cpu.x[8]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fce050 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fce054;super::dispatch(cpu,rt,target)?;}
{// 0x10fce054 tbz w0, #0, #0x10fce01c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce01c;continue;}{// 0x10fce058 str wzr, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce05c ldr w8, [sp, #0x14]
{let address=(cpu.sp).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce060 cmp w25, w8
{let a=((cpu.x[25] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce064 b.gt #0x10fce338
if cpu.condition(12){{// 0x10fce338 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
{// 0x10fce33c ldp x20, x19, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fce340 ldp x22, x21, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fce344 ldp x24, x23, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fce348 ldp x26, x25, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fce34c ldp x28, x27, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fce350 ldp x29, x30, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fce354 add sp, sp, #0xb0
{let a=((cpu.sp));let b=((0xb0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fce358 ret
cpu.pc=cpu.x[30];return Ok(());}}}{// 0x10fce068 sub w8, w28, w19
{let a=((cpu.x[28] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fce06c bic w9, w8, w8, asr #31
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[8] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[9]=(result) as u32 as u64;}
// 0x10fce070 ldr w8, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce074 cmp w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce078 str w9, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce07c b.gt #0x10fce338
if cpu.condition(12){{// 0x10fce338 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
pc=0x10fce33c;continue;}}{// 0x10fce080 mul w8, w26, w26
cpu.x[8]=((((cpu.x[26] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce084 stur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce088 tbz w20, #0, #0x10fce1e8
if ((cpu.x[20] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce1e8;continue;}pc=0x10fce08c;continue;}}}}}}{// 0x10fcdff8 ldrb w8, [x27, #0x80a]
{let address=(cpu.x[27]).wrapping_add(0x80au64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcdffc cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
pc=0x10fce000;continue;}}}}}}
}
0x10fcdf7c=>{
// 0x10fcdf7c mov w8, #0x4dd3
cpu.x[8]=(((0x4dd3u64))) as u32 as u64;
// 0x10fcdf80 str x0, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fcdf84 mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10fcdf88 movk w8, #0x1062, lsl #16
cpu.x[8]=((cpu.x[8] as u32 as u64 & !(0xffff0000u64)) | 0x10620000u64) as u32 as u64;
// 0x10fcdf8c smull x8, w26, w8
cpu.x[8]=(((((cpu.x[26] as u32 as u64))) as i32 as i64).wrapping_mul((((cpu.x[8] as u32 as u64))) as i32 as i64) as u64) as u64;
// 0x10fcdf90 lsr x9, x8, #0x3f
cpu.x[9]=(((((cpu.x[8]))) as u64).wrapping_shr(((((0x3fu64))) & 63) as u32) as u64) as u64;
// 0x10fcdf94 asr x8, x8, #0x25
cpu.x[8]=(((((cpu.x[8]))) as i64).wrapping_shr(((((0x25u64))) & 63) as u32) as u64) as u64;
// 0x10fcdf98 add w8, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fcdf9c add w19, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[19]=(result) as u32 as u64;}
// 0x10fcdfa0 ldp w9, w8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10fcdfa4 add w10, w28, w19
{let a=((cpu.x[28] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10fcdfa8 add w11, w27, w19
{let a=((cpu.x[27] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u32 as u64;}
// 0x10fcdfac sub w24, w27, w19
{let a=((cpu.x[27] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[24]=(result) as u32 as u64;}
// 0x10fcdfb0 sub w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fcdfb4 sub w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fcdfb8 cmp w10, w9
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcdfbc csel w9, w10, w9, lt
cpu.x[9]=(if cpu.condition(11){((cpu.x[10] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10fcdfc0 cmp w11, w8
{let a=((cpu.x[11] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcdfc4 csel w8, w11, w8, lt
cpu.x[8]=(if cpu.condition(11){((cpu.x[11] as u32 as u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10fcdfc8 stp w9, w8, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[9] as u32 as u64;let v1=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10fcdfcc bl #0x10fd04a8
cpu.x[30]=0x10fcdfd0;super::chunk2::f_10fd04a8(cpu,rt,0x10fd04a8)?;
{// 0x10fcdfd0 bl #0x10e0b6b4
cpu.x[30]=0x10fcdfd4;super::chunk8::f_10e0b6b4(cpu,rt,0x10e0b6b4)?;
{// 0x10fcdfd4 ldr x0, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fcdfd8 bl #0x10e16208
cpu.x[30]=0x10fcdfdc;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
{// 0x10fcdfdc ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcdfe0 mov x27, x0
cpu.x[27]=(((cpu.x[0]))) as u64;
// 0x10fcdfe4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcdfe8 mov x21, x20
cpu.x[21]=(((cpu.x[20]))) as u64;
// 0x10fcdfec ldr x8, [x8, #0x108]
{let address=(cpu.x[8]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcdff0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fcdff4;super::dispatch(cpu,rt,target)?;}
{// 0x10fcdff4 tbz w0, #0, #0x10fce03c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x10fce03c mov w20, #1
cpu.x[20]=(((0x1u64))) as u32 as u64;
// 0x10fce040 ldr x8, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fce044 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fce048 bic w25, w24, w24, asr #31
{let a=((cpu.x[24] as u32 as u64));let b=(((cpu.x[24] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[25]=(result) as u32 as u64;}
// 0x10fce04c ldr x8, [x8, #0x108]
{let address=(cpu.x[8]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fce050 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fce054;super::dispatch(cpu,rt,target)?;}
{// 0x10fce054 tbz w0, #0, #0x10fce01c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce01c;continue;}{// 0x10fce058 str wzr, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce05c ldr w8, [sp, #0x14]
{let address=(cpu.sp).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce060 cmp w25, w8
{let a=((cpu.x[25] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce064 b.gt #0x10fce338
if cpu.condition(12){{// 0x10fce338 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
{// 0x10fce33c ldp x20, x19, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fce340 ldp x22, x21, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fce344 ldp x24, x23, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fce348 ldp x26, x25, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fce34c ldp x28, x27, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fce350 ldp x29, x30, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fce354 add sp, sp, #0xb0
{let a=((cpu.sp));let b=((0xb0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fce358 ret
cpu.pc=cpu.x[30];return Ok(());}}}{// 0x10fce068 sub w8, w28, w19
{let a=((cpu.x[28] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fce06c bic w9, w8, w8, asr #31
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[8] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[9]=(result) as u32 as u64;}
// 0x10fce070 ldr w8, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce074 cmp w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce078 str w9, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce07c b.gt #0x10fce338
if cpu.condition(12){{// 0x10fce338 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
{// 0x10fce33c ldp x20, x19, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fce340 ldp x22, x21, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fce344 ldp x24, x23, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fce348 ldp x26, x25, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fce34c ldp x28, x27, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fce350 ldp x29, x30, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fce354 add sp, sp, #0xb0
{let a=((cpu.sp));let b=((0xb0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fce358 ret
cpu.pc=cpu.x[30];return Ok(());}}}{// 0x10fce080 mul w8, w26, w26
cpu.x[8]=((((cpu.x[26] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce084 stur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce088 tbz w20, #0, #0x10fce1e8
if ((cpu.x[20] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x10fce1e8 ldr w8, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1ec mov w9, #0x7fffffff
cpu.x[9]=(((0x7fffffffu64))) as u32 as u64;
// 0x10fce1f0 mov w26, #0xfa
cpu.x[26]=(((0xfau64))) as u32 as u64;
// 0x10fce1f4 str wzr, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1f8 sub w20, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[20]=(result) as u32 as u64;}
// 0x10fce1fc mov w8, #-1
cpu.x[8]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10fce200 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce204 mov w8, #0xfa
cpu.x[8]=(((0xfau64))) as u32 as u64;
// 0x10fce208 stp w8, w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=cpu.x[8] as u32 as u64;let v1=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10fce20c b #0x10fce220
{// 0x10fce220 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce224 ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce228 madd w27, w25, w8, w9
cpu.x[27]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[25] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce22c ldp w24, w19, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[24]=(v0) as u32 as u64;cpu.x[19]=(v1) as u32 as u64;}
// 0x10fce230 b #0x10fce258
pc=0x10fce258;continue;}}}pc=0x10fce08c;continue;}}}}}}pc=0x10fcdff8;continue;}}}}
}
0x10fcdfd0=>{
// 0x10fcdfd0 bl #0x10e0b6b4
cpu.x[30]=0x10fcdfd4;super::chunk8::f_10e0b6b4(cpu,rt,0x10e0b6b4)?;
{// 0x10fcdfd4 ldr x0, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fcdfd8 bl #0x10e16208
cpu.x[30]=0x10fcdfdc;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
{// 0x10fcdfdc ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcdfe0 mov x27, x0
cpu.x[27]=(((cpu.x[0]))) as u64;
// 0x10fcdfe4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcdfe8 mov x21, x20
cpu.x[21]=(((cpu.x[20]))) as u64;
// 0x10fcdfec ldr x8, [x8, #0x108]
{let address=(cpu.x[8]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcdff0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fcdff4;super::dispatch(cpu,rt,target)?;}
{// 0x10fcdff4 tbz w0, #0, #0x10fce03c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x10fce03c mov w20, #1
cpu.x[20]=(((0x1u64))) as u32 as u64;
// 0x10fce040 ldr x8, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fce044 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fce048 bic w25, w24, w24, asr #31
{let a=((cpu.x[24] as u32 as u64));let b=(((cpu.x[24] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[25]=(result) as u32 as u64;}
// 0x10fce04c ldr x8, [x8, #0x108]
{let address=(cpu.x[8]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fce050 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fce054;super::dispatch(cpu,rt,target)?;}
{// 0x10fce054 tbz w0, #0, #0x10fce01c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce01c;continue;}{// 0x10fce058 str wzr, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce05c ldr w8, [sp, #0x14]
{let address=(cpu.sp).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce060 cmp w25, w8
{let a=((cpu.x[25] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce064 b.gt #0x10fce338
if cpu.condition(12){{// 0x10fce338 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
{// 0x10fce33c ldp x20, x19, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fce340 ldp x22, x21, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fce344 ldp x24, x23, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fce348 ldp x26, x25, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fce34c ldp x28, x27, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fce350 ldp x29, x30, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fce354 add sp, sp, #0xb0
{let a=((cpu.sp));let b=((0xb0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fce358 ret
cpu.pc=cpu.x[30];return Ok(());}}}{// 0x10fce068 sub w8, w28, w19
{let a=((cpu.x[28] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fce06c bic w9, w8, w8, asr #31
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[8] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[9]=(result) as u32 as u64;}
// 0x10fce070 ldr w8, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce074 cmp w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce078 str w9, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce07c b.gt #0x10fce338
if cpu.condition(12){{// 0x10fce338 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
{// 0x10fce33c ldp x20, x19, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fce340 ldp x22, x21, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fce344 ldp x24, x23, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fce348 ldp x26, x25, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fce34c ldp x28, x27, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fce350 ldp x29, x30, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fce354 add sp, sp, #0xb0
{let a=((cpu.sp));let b=((0xb0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fce358 ret
cpu.pc=cpu.x[30];return Ok(());}}}{// 0x10fce080 mul w8, w26, w26
cpu.x[8]=((((cpu.x[26] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce084 stur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce088 tbz w20, #0, #0x10fce1e8
if ((cpu.x[20] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x10fce1e8 ldr w8, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1ec mov w9, #0x7fffffff
cpu.x[9]=(((0x7fffffffu64))) as u32 as u64;
// 0x10fce1f0 mov w26, #0xfa
cpu.x[26]=(((0xfau64))) as u32 as u64;
// 0x10fce1f4 str wzr, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1f8 sub w20, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[20]=(result) as u32 as u64;}
// 0x10fce1fc mov w8, #-1
cpu.x[8]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10fce200 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce204 mov w8, #0xfa
cpu.x[8]=(((0xfau64))) as u32 as u64;
// 0x10fce208 stp w8, w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=cpu.x[8] as u32 as u64;let v1=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10fce20c b #0x10fce220
{// 0x10fce220 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce224 ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce228 madd w27, w25, w8, w9
cpu.x[27]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[25] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce22c ldp w24, w19, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[24]=(v0) as u32 as u64;cpu.x[19]=(v1) as u32 as u64;}
// 0x10fce230 b #0x10fce258
{// 0x10fce258 ldur x0, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce25c bl #0x10d358a0
cpu.x[30]=0x10fce260;super::chunk0::f_10d358a0(cpu,rt,0x10d358a0)?;
pc=0x10fce260;continue;}}}}pc=0x10fce08c;continue;}}}}}}pc=0x10fcdff8;continue;}}}
}
0x10fcdfd4=>{
// 0x10fcdfd4 ldr x0, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fcdfd8 bl #0x10e16208
cpu.x[30]=0x10fcdfdc;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
{// 0x10fcdfdc ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcdfe0 mov x27, x0
cpu.x[27]=(((cpu.x[0]))) as u64;
// 0x10fcdfe4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcdfe8 mov x21, x20
cpu.x[21]=(((cpu.x[20]))) as u64;
// 0x10fcdfec ldr x8, [x8, #0x108]
{let address=(cpu.x[8]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcdff0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fcdff4;super::dispatch(cpu,rt,target)?;}
{// 0x10fcdff4 tbz w0, #0, #0x10fce03c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x10fce03c mov w20, #1
cpu.x[20]=(((0x1u64))) as u32 as u64;
// 0x10fce040 ldr x8, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fce044 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fce048 bic w25, w24, w24, asr #31
{let a=((cpu.x[24] as u32 as u64));let b=(((cpu.x[24] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[25]=(result) as u32 as u64;}
// 0x10fce04c ldr x8, [x8, #0x108]
{let address=(cpu.x[8]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fce050 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fce054;super::dispatch(cpu,rt,target)?;}
{// 0x10fce054 tbz w0, #0, #0x10fce01c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce01c;continue;}{// 0x10fce058 str wzr, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce05c ldr w8, [sp, #0x14]
{let address=(cpu.sp).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce060 cmp w25, w8
{let a=((cpu.x[25] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce064 b.gt #0x10fce338
if cpu.condition(12){{// 0x10fce338 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
{// 0x10fce33c ldp x20, x19, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fce340 ldp x22, x21, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fce344 ldp x24, x23, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fce348 ldp x26, x25, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fce34c ldp x28, x27, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fce350 ldp x29, x30, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fce354 add sp, sp, #0xb0
{let a=((cpu.sp));let b=((0xb0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fce358 ret
cpu.pc=cpu.x[30];return Ok(());}}}{// 0x10fce068 sub w8, w28, w19
{let a=((cpu.x[28] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fce06c bic w9, w8, w8, asr #31
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[8] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[9]=(result) as u32 as u64;}
// 0x10fce070 ldr w8, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce074 cmp w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce078 str w9, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce07c b.gt #0x10fce338
if cpu.condition(12){{// 0x10fce338 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
{// 0x10fce33c ldp x20, x19, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fce340 ldp x22, x21, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fce344 ldp x24, x23, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fce348 ldp x26, x25, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fce34c ldp x28, x27, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fce350 ldp x29, x30, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fce354 add sp, sp, #0xb0
{let a=((cpu.sp));let b=((0xb0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fce358 ret
cpu.pc=cpu.x[30];return Ok(());}}}{// 0x10fce080 mul w8, w26, w26
cpu.x[8]=((((cpu.x[26] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce084 stur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce088 tbz w20, #0, #0x10fce1e8
if ((cpu.x[20] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x10fce1e8 ldr w8, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1ec mov w9, #0x7fffffff
cpu.x[9]=(((0x7fffffffu64))) as u32 as u64;
// 0x10fce1f0 mov w26, #0xfa
cpu.x[26]=(((0xfau64))) as u32 as u64;
// 0x10fce1f4 str wzr, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1f8 sub w20, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[20]=(result) as u32 as u64;}
// 0x10fce1fc mov w8, #-1
cpu.x[8]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10fce200 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce204 mov w8, #0xfa
cpu.x[8]=(((0xfau64))) as u32 as u64;
// 0x10fce208 stp w8, w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=cpu.x[8] as u32 as u64;let v1=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10fce20c b #0x10fce220
{// 0x10fce220 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce224 ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce228 madd w27, w25, w8, w9
cpu.x[27]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[25] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce22c ldp w24, w19, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[24]=(v0) as u32 as u64;cpu.x[19]=(v1) as u32 as u64;}
// 0x10fce230 b #0x10fce258
{// 0x10fce258 ldur x0, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce25c bl #0x10d358a0
cpu.x[30]=0x10fce260;super::chunk0::f_10d358a0(cpu,rt,0x10d358a0)?;
pc=0x10fce260;continue;}}}}pc=0x10fce08c;continue;}}}}}}{// 0x10fcdff8 ldrb w8, [x27, #0x80a]
{let address=(cpu.x[27]).wrapping_add(0x80au64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcdffc cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
pc=0x10fce000;continue;}}}
}
0x10fcdfdc=>{
// 0x10fcdfdc ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcdfe0 mov x27, x0
cpu.x[27]=(((cpu.x[0]))) as u64;
// 0x10fcdfe4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcdfe8 mov x21, x20
cpu.x[21]=(((cpu.x[20]))) as u64;
// 0x10fcdfec ldr x8, [x8, #0x108]
{let address=(cpu.x[8]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcdff0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fcdff4;super::dispatch(cpu,rt,target)?;}
{// 0x10fcdff4 tbz w0, #0, #0x10fce03c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x10fce03c mov w20, #1
cpu.x[20]=(((0x1u64))) as u32 as u64;
// 0x10fce040 ldr x8, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fce044 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fce048 bic w25, w24, w24, asr #31
{let a=((cpu.x[24] as u32 as u64));let b=(((cpu.x[24] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[25]=(result) as u32 as u64;}
// 0x10fce04c ldr x8, [x8, #0x108]
{let address=(cpu.x[8]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fce050 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fce054;super::dispatch(cpu,rt,target)?;}
{// 0x10fce054 tbz w0, #0, #0x10fce01c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce01c;continue;}{// 0x10fce058 str wzr, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce05c ldr w8, [sp, #0x14]
{let address=(cpu.sp).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce060 cmp w25, w8
{let a=((cpu.x[25] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce064 b.gt #0x10fce338
if cpu.condition(12){{// 0x10fce338 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
{// 0x10fce33c ldp x20, x19, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fce340 ldp x22, x21, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fce344 ldp x24, x23, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fce348 ldp x26, x25, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fce34c ldp x28, x27, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fce350 ldp x29, x30, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fce354 add sp, sp, #0xb0
{let a=((cpu.sp));let b=((0xb0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fce358 ret
cpu.pc=cpu.x[30];return Ok(());}}}{// 0x10fce068 sub w8, w28, w19
{let a=((cpu.x[28] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fce06c bic w9, w8, w8, asr #31
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[8] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[9]=(result) as u32 as u64;}
// 0x10fce070 ldr w8, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce074 cmp w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce078 str w9, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce07c b.gt #0x10fce338
if cpu.condition(12){{// 0x10fce338 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
{// 0x10fce33c ldp x20, x19, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fce340 ldp x22, x21, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fce344 ldp x24, x23, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fce348 ldp x26, x25, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fce34c ldp x28, x27, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fce350 ldp x29, x30, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fce354 add sp, sp, #0xb0
{let a=((cpu.sp));let b=((0xb0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fce358 ret
cpu.pc=cpu.x[30];return Ok(());}}}{// 0x10fce080 mul w8, w26, w26
cpu.x[8]=((((cpu.x[26] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce084 stur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce088 tbz w20, #0, #0x10fce1e8
if ((cpu.x[20] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x10fce1e8 ldr w8, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1ec mov w9, #0x7fffffff
cpu.x[9]=(((0x7fffffffu64))) as u32 as u64;
// 0x10fce1f0 mov w26, #0xfa
cpu.x[26]=(((0xfau64))) as u32 as u64;
// 0x10fce1f4 str wzr, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1f8 sub w20, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[20]=(result) as u32 as u64;}
// 0x10fce1fc mov w8, #-1
cpu.x[8]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10fce200 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce204 mov w8, #0xfa
cpu.x[8]=(((0xfau64))) as u32 as u64;
// 0x10fce208 stp w8, w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=cpu.x[8] as u32 as u64;let v1=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10fce20c b #0x10fce220
{// 0x10fce220 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce224 ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce228 madd w27, w25, w8, w9
cpu.x[27]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[25] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce22c ldp w24, w19, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[24]=(v0) as u32 as u64;cpu.x[19]=(v1) as u32 as u64;}
// 0x10fce230 b #0x10fce258
{// 0x10fce258 ldur x0, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce25c bl #0x10d358a0
cpu.x[30]=0x10fce260;super::chunk0::f_10d358a0(cpu,rt,0x10d358a0)?;
pc=0x10fce260;continue;}}}}pc=0x10fce08c;continue;}}}}}}{// 0x10fcdff8 ldrb w8, [x27, #0x80a]
{let address=(cpu.x[27]).wrapping_add(0x80au64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcdffc cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
{// 0x10fce000 cset w20, eq
cpu.x[20]=((cpu.condition(0)) as u64) as u32 as u64;
// 0x10fce004 ldr x8, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fce008 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fce00c bic w25, w24, w24, asr #31
{let a=((cpu.x[24] as u32 as u64));let b=(((cpu.x[24] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[25]=(result) as u32 as u64;}
// 0x10fce010 ldr x8, [x8, #0x108]
{let address=(cpu.x[8]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fce014 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fce018;super::dispatch(cpu,rt,target)?;}
pc=0x10fce018;continue;}}}
}
0x10fcdff4=>{
// 0x10fcdff4 tbz w0, #0, #0x10fce03c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x10fce03c mov w20, #1
cpu.x[20]=(((0x1u64))) as u32 as u64;
// 0x10fce040 ldr x8, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fce044 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fce048 bic w25, w24, w24, asr #31
{let a=((cpu.x[24] as u32 as u64));let b=(((cpu.x[24] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[25]=(result) as u32 as u64;}
// 0x10fce04c ldr x8, [x8, #0x108]
{let address=(cpu.x[8]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fce050 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fce054;super::dispatch(cpu,rt,target)?;}
{// 0x10fce054 tbz w0, #0, #0x10fce01c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce01c;continue;}{// 0x10fce058 str wzr, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce05c ldr w8, [sp, #0x14]
{let address=(cpu.sp).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce060 cmp w25, w8
{let a=((cpu.x[25] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce064 b.gt #0x10fce338
if cpu.condition(12){{// 0x10fce338 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
{// 0x10fce33c ldp x20, x19, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fce340 ldp x22, x21, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fce344 ldp x24, x23, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fce348 ldp x26, x25, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fce34c ldp x28, x27, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fce350 ldp x29, x30, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fce354 add sp, sp, #0xb0
{let a=((cpu.sp));let b=((0xb0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fce358 ret
cpu.pc=cpu.x[30];return Ok(());}}}{// 0x10fce068 sub w8, w28, w19
{let a=((cpu.x[28] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fce06c bic w9, w8, w8, asr #31
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[8] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[9]=(result) as u32 as u64;}
// 0x10fce070 ldr w8, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce074 cmp w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce078 str w9, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce07c b.gt #0x10fce338
if cpu.condition(12){{// 0x10fce338 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
{// 0x10fce33c ldp x20, x19, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fce340 ldp x22, x21, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fce344 ldp x24, x23, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fce348 ldp x26, x25, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fce34c ldp x28, x27, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fce350 ldp x29, x30, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fce354 add sp, sp, #0xb0
{let a=((cpu.sp));let b=((0xb0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fce358 ret
cpu.pc=cpu.x[30];return Ok(());}}}{// 0x10fce080 mul w8, w26, w26
cpu.x[8]=((((cpu.x[26] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce084 stur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce088 tbz w20, #0, #0x10fce1e8
if ((cpu.x[20] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x10fce1e8 ldr w8, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1ec mov w9, #0x7fffffff
cpu.x[9]=(((0x7fffffffu64))) as u32 as u64;
// 0x10fce1f0 mov w26, #0xfa
cpu.x[26]=(((0xfau64))) as u32 as u64;
// 0x10fce1f4 str wzr, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1f8 sub w20, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[20]=(result) as u32 as u64;}
// 0x10fce1fc mov w8, #-1
cpu.x[8]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10fce200 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce204 mov w8, #0xfa
cpu.x[8]=(((0xfau64))) as u32 as u64;
// 0x10fce208 stp w8, w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=cpu.x[8] as u32 as u64;let v1=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10fce20c b #0x10fce220
{// 0x10fce220 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce224 ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce228 madd w27, w25, w8, w9
cpu.x[27]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[25] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce22c ldp w24, w19, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[24]=(v0) as u32 as u64;cpu.x[19]=(v1) as u32 as u64;}
// 0x10fce230 b #0x10fce258
{// 0x10fce258 ldur x0, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce25c bl #0x10d358a0
cpu.x[30]=0x10fce260;super::chunk0::f_10d358a0(cpu,rt,0x10d358a0)?;
{// 0x10fce260 mul w8, w0, w26
cpu.x[8]=((((cpu.x[0] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce264 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce268 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce26c mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10fce270 mov x4, x21
cpu.x[4]=(((cpu.x[21]))) as u64;
// 0x10fce274 cmp w22, w8
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce278 csel w28, w24, w19, lt
cpu.x[28]=(if cpu.condition(11){((cpu.x[24] as u32 as u64))}else{((cpu.x[19] as u32 as u64))}) as u32 as u64;
// 0x10fce27c mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce280 bl #0x111d9240
cpu.x[30]=0x10fce284;super::chunk1::f_111d9240(cpu,rt,0x111d9240)?;
pc=0x10fce284;continue;}}}}}pc=0x10fce08c;continue;}}}}}}pc=0x10fcdff8;continue;
}
0x10fcdff8=>{
// 0x10fcdff8 ldrb w8, [x27, #0x80a]
{let address=(cpu.x[27]).wrapping_add(0x80au64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcdffc cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
{// 0x10fce000 cset w20, eq
cpu.x[20]=((cpu.condition(0)) as u64) as u32 as u64;
// 0x10fce004 ldr x8, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fce008 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fce00c bic w25, w24, w24, asr #31
{let a=((cpu.x[24] as u32 as u64));let b=(((cpu.x[24] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[25]=(result) as u32 as u64;}
// 0x10fce010 ldr x8, [x8, #0x108]
{let address=(cpu.x[8]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fce014 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fce018;super::dispatch(cpu,rt,target)?;}
{// 0x10fce018 tbnz w0, #0, #0x10fce058
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {{// 0x10fce058 str wzr, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce05c ldr w8, [sp, #0x14]
{let address=(cpu.sp).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce060 cmp w25, w8
{let a=((cpu.x[25] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce064 b.gt #0x10fce338
if cpu.condition(12){{// 0x10fce338 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
{// 0x10fce33c ldp x20, x19, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fce340 ldp x22, x21, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fce344 ldp x24, x23, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fce348 ldp x26, x25, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fce34c ldp x28, x27, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fce350 ldp x29, x30, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fce354 add sp, sp, #0xb0
{let a=((cpu.sp));let b=((0xb0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fce358 ret
cpu.pc=cpu.x[30];return Ok(());}}}{// 0x10fce068 sub w8, w28, w19
{let a=((cpu.x[28] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fce06c bic w9, w8, w8, asr #31
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[8] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[9]=(result) as u32 as u64;}
// 0x10fce070 ldr w8, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce074 cmp w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce078 str w9, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce07c b.gt #0x10fce338
if cpu.condition(12){{// 0x10fce338 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
{// 0x10fce33c ldp x20, x19, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fce340 ldp x22, x21, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fce344 ldp x24, x23, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fce348 ldp x26, x25, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fce34c ldp x28, x27, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fce350 ldp x29, x30, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fce354 add sp, sp, #0xb0
{let a=((cpu.sp));let b=((0xb0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fce358 ret
cpu.pc=cpu.x[30];return Ok(());}}}{// 0x10fce080 mul w8, w26, w26
cpu.x[8]=((((cpu.x[26] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce084 stur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce088 tbz w20, #0, #0x10fce1e8
if ((cpu.x[20] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x10fce1e8 ldr w8, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1ec mov w9, #0x7fffffff
cpu.x[9]=(((0x7fffffffu64))) as u32 as u64;
// 0x10fce1f0 mov w26, #0xfa
cpu.x[26]=(((0xfau64))) as u32 as u64;
// 0x10fce1f4 str wzr, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1f8 sub w20, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[20]=(result) as u32 as u64;}
// 0x10fce1fc mov w8, #-1
cpu.x[8]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10fce200 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce204 mov w8, #0xfa
cpu.x[8]=(((0xfau64))) as u32 as u64;
// 0x10fce208 stp w8, w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=cpu.x[8] as u32 as u64;let v1=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10fce20c b #0x10fce220
{// 0x10fce220 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce224 ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce228 madd w27, w25, w8, w9
cpu.x[27]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[25] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce22c ldp w24, w19, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[24]=(v0) as u32 as u64;cpu.x[19]=(v1) as u32 as u64;}
// 0x10fce230 b #0x10fce258
{// 0x10fce258 ldur x0, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce25c bl #0x10d358a0
cpu.x[30]=0x10fce260;super::chunk0::f_10d358a0(cpu,rt,0x10d358a0)?;
{// 0x10fce260 mul w8, w0, w26
cpu.x[8]=((((cpu.x[0] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce264 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce268 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce26c mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10fce270 mov x4, x21
cpu.x[4]=(((cpu.x[21]))) as u64;
// 0x10fce274 cmp w22, w8
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce278 csel w28, w24, w19, lt
cpu.x[28]=(if cpu.condition(11){((cpu.x[24] as u32 as u64))}else{((cpu.x[19] as u32 as u64))}) as u32 as u64;
// 0x10fce27c mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce280 bl #0x111d9240
cpu.x[30]=0x10fce284;super::chunk1::f_111d9240(cpu,rt,0x111d9240)?;
pc=0x10fce284;continue;}}}}}pc=0x10fce08c;continue;}}}}pc=0x10fce01c;continue;}}
}
0x10fce000=>{
// 0x10fce000 cset w20, eq
cpu.x[20]=((cpu.condition(0)) as u64) as u32 as u64;
// 0x10fce004 ldr x8, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fce008 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fce00c bic w25, w24, w24, asr #31
{let a=((cpu.x[24] as u32 as u64));let b=(((cpu.x[24] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[25]=(result) as u32 as u64;}
// 0x10fce010 ldr x8, [x8, #0x108]
{let address=(cpu.x[8]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fce014 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fce018;super::dispatch(cpu,rt,target)?;}
{// 0x10fce018 tbnz w0, #0, #0x10fce058
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {{// 0x10fce058 str wzr, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce05c ldr w8, [sp, #0x14]
{let address=(cpu.sp).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce060 cmp w25, w8
{let a=((cpu.x[25] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce064 b.gt #0x10fce338
if cpu.condition(12){{// 0x10fce338 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
{// 0x10fce33c ldp x20, x19, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fce340 ldp x22, x21, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fce344 ldp x24, x23, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fce348 ldp x26, x25, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fce34c ldp x28, x27, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fce350 ldp x29, x30, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fce354 add sp, sp, #0xb0
{let a=((cpu.sp));let b=((0xb0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fce358 ret
cpu.pc=cpu.x[30];return Ok(());}}}{// 0x10fce068 sub w8, w28, w19
{let a=((cpu.x[28] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fce06c bic w9, w8, w8, asr #31
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[8] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[9]=(result) as u32 as u64;}
// 0x10fce070 ldr w8, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce074 cmp w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce078 str w9, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce07c b.gt #0x10fce338
if cpu.condition(12){{// 0x10fce338 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
{// 0x10fce33c ldp x20, x19, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fce340 ldp x22, x21, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fce344 ldp x24, x23, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fce348 ldp x26, x25, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fce34c ldp x28, x27, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fce350 ldp x29, x30, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fce354 add sp, sp, #0xb0
{let a=((cpu.sp));let b=((0xb0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fce358 ret
cpu.pc=cpu.x[30];return Ok(());}}}{// 0x10fce080 mul w8, w26, w26
cpu.x[8]=((((cpu.x[26] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce084 stur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce088 tbz w20, #0, #0x10fce1e8
if ((cpu.x[20] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x10fce1e8 ldr w8, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1ec mov w9, #0x7fffffff
cpu.x[9]=(((0x7fffffffu64))) as u32 as u64;
// 0x10fce1f0 mov w26, #0xfa
cpu.x[26]=(((0xfau64))) as u32 as u64;
// 0x10fce1f4 str wzr, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1f8 sub w20, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[20]=(result) as u32 as u64;}
// 0x10fce1fc mov w8, #-1
cpu.x[8]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10fce200 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce204 mov w8, #0xfa
cpu.x[8]=(((0xfau64))) as u32 as u64;
// 0x10fce208 stp w8, w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=cpu.x[8] as u32 as u64;let v1=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10fce20c b #0x10fce220
{// 0x10fce220 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce224 ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce228 madd w27, w25, w8, w9
cpu.x[27]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[25] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce22c ldp w24, w19, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[24]=(v0) as u32 as u64;cpu.x[19]=(v1) as u32 as u64;}
// 0x10fce230 b #0x10fce258
{// 0x10fce258 ldur x0, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce25c bl #0x10d358a0
cpu.x[30]=0x10fce260;super::chunk0::f_10d358a0(cpu,rt,0x10d358a0)?;
{// 0x10fce260 mul w8, w0, w26
cpu.x[8]=((((cpu.x[0] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce264 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce268 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce26c mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10fce270 mov x4, x21
cpu.x[4]=(((cpu.x[21]))) as u64;
// 0x10fce274 cmp w22, w8
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce278 csel w28, w24, w19, lt
cpu.x[28]=(if cpu.condition(11){((cpu.x[24] as u32 as u64))}else{((cpu.x[19] as u32 as u64))}) as u32 as u64;
// 0x10fce27c mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce280 bl #0x111d9240
cpu.x[30]=0x10fce284;super::chunk1::f_111d9240(cpu,rt,0x111d9240)?;
{// 0x10fce284 tbz w0, #0, #0x10fce248
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce248;continue;}pc=0x10fce288;continue;}}}}}}pc=0x10fce08c;continue;}}}}pc=0x10fce01c;continue;}
}
0x10fce018=>{
// 0x10fce018 tbnz w0, #0, #0x10fce058
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {{// 0x10fce058 str wzr, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce05c ldr w8, [sp, #0x14]
{let address=(cpu.sp).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce060 cmp w25, w8
{let a=((cpu.x[25] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce064 b.gt #0x10fce338
if cpu.condition(12){{// 0x10fce338 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
{// 0x10fce33c ldp x20, x19, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fce340 ldp x22, x21, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fce344 ldp x24, x23, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fce348 ldp x26, x25, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fce34c ldp x28, x27, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fce350 ldp x29, x30, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fce354 add sp, sp, #0xb0
{let a=((cpu.sp));let b=((0xb0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fce358 ret
cpu.pc=cpu.x[30];return Ok(());}}}{// 0x10fce068 sub w8, w28, w19
{let a=((cpu.x[28] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fce06c bic w9, w8, w8, asr #31
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[8] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[9]=(result) as u32 as u64;}
// 0x10fce070 ldr w8, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce074 cmp w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce078 str w9, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce07c b.gt #0x10fce338
if cpu.condition(12){{// 0x10fce338 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
{// 0x10fce33c ldp x20, x19, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fce340 ldp x22, x21, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fce344 ldp x24, x23, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fce348 ldp x26, x25, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fce34c ldp x28, x27, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fce350 ldp x29, x30, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fce354 add sp, sp, #0xb0
{let a=((cpu.sp));let b=((0xb0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fce358 ret
cpu.pc=cpu.x[30];return Ok(());}}}{// 0x10fce080 mul w8, w26, w26
cpu.x[8]=((((cpu.x[26] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce084 stur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce088 tbz w20, #0, #0x10fce1e8
if ((cpu.x[20] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x10fce1e8 ldr w8, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1ec mov w9, #0x7fffffff
cpu.x[9]=(((0x7fffffffu64))) as u32 as u64;
// 0x10fce1f0 mov w26, #0xfa
cpu.x[26]=(((0xfau64))) as u32 as u64;
// 0x10fce1f4 str wzr, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1f8 sub w20, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[20]=(result) as u32 as u64;}
// 0x10fce1fc mov w8, #-1
cpu.x[8]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10fce200 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce204 mov w8, #0xfa
cpu.x[8]=(((0xfau64))) as u32 as u64;
// 0x10fce208 stp w8, w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=cpu.x[8] as u32 as u64;let v1=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10fce20c b #0x10fce220
{// 0x10fce220 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce224 ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce228 madd w27, w25, w8, w9
cpu.x[27]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[25] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce22c ldp w24, w19, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[24]=(v0) as u32 as u64;cpu.x[19]=(v1) as u32 as u64;}
// 0x10fce230 b #0x10fce258
{// 0x10fce258 ldur x0, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce25c bl #0x10d358a0
cpu.x[30]=0x10fce260;super::chunk0::f_10d358a0(cpu,rt,0x10d358a0)?;
{// 0x10fce260 mul w8, w0, w26
cpu.x[8]=((((cpu.x[0] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce264 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce268 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce26c mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10fce270 mov x4, x21
cpu.x[4]=(((cpu.x[21]))) as u64;
// 0x10fce274 cmp w22, w8
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce278 csel w28, w24, w19, lt
cpu.x[28]=(if cpu.condition(11){((cpu.x[24] as u32 as u64))}else{((cpu.x[19] as u32 as u64))}) as u32 as u64;
// 0x10fce27c mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce280 bl #0x111d9240
cpu.x[30]=0x10fce284;super::chunk1::f_111d9240(cpu,rt,0x111d9240)?;
{// 0x10fce284 tbz w0, #0, #0x10fce248
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce248;continue;}pc=0x10fce288;continue;}}}}}}pc=0x10fce08c;continue;}}}}pc=0x10fce01c;continue;
}
0x10fce01c=>{
// 0x10fce01c ldrb w8, [x27, #0x80b]
{let address=(cpu.x[27]).wrapping_add(0x80bu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce020 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce024 cset w8, ne
cpu.x[8]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x10fce028 str w8, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce02c ldr w8, [sp, #0x14]
{let address=(cpu.sp).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce030 cmp w25, w8
{let a=((cpu.x[25] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce034 b.gt #0x10fce338
if cpu.condition(12){{// 0x10fce338 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
{// 0x10fce33c ldp x20, x19, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fce340 ldp x22, x21, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fce344 ldp x24, x23, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fce348 ldp x26, x25, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fce34c ldp x28, x27, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fce350 ldp x29, x30, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fce354 add sp, sp, #0xb0
{let a=((cpu.sp));let b=((0xb0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fce358 ret
cpu.pc=cpu.x[30];return Ok(());}}}{// 0x10fce038 b #0x10fce068
{// 0x10fce068 sub w8, w28, w19
{let a=((cpu.x[28] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fce06c bic w9, w8, w8, asr #31
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[8] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[9]=(result) as u32 as u64;}
// 0x10fce070 ldr w8, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce074 cmp w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce078 str w9, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce07c b.gt #0x10fce338
if cpu.condition(12){{// 0x10fce338 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
{// 0x10fce33c ldp x20, x19, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fce340 ldp x22, x21, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fce344 ldp x24, x23, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fce348 ldp x26, x25, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fce34c ldp x28, x27, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fce350 ldp x29, x30, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fce354 add sp, sp, #0xb0
{let a=((cpu.sp));let b=((0xb0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fce358 ret
cpu.pc=cpu.x[30];return Ok(());}}}{// 0x10fce080 mul w8, w26, w26
cpu.x[8]=((((cpu.x[26] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce084 stur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce088 tbz w20, #0, #0x10fce1e8
if ((cpu.x[20] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x10fce1e8 ldr w8, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1ec mov w9, #0x7fffffff
cpu.x[9]=(((0x7fffffffu64))) as u32 as u64;
// 0x10fce1f0 mov w26, #0xfa
cpu.x[26]=(((0xfau64))) as u32 as u64;
// 0x10fce1f4 str wzr, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1f8 sub w20, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[20]=(result) as u32 as u64;}
// 0x10fce1fc mov w8, #-1
cpu.x[8]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10fce200 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce204 mov w8, #0xfa
cpu.x[8]=(((0xfau64))) as u32 as u64;
// 0x10fce208 stp w8, w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=cpu.x[8] as u32 as u64;let v1=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10fce20c b #0x10fce220
{// 0x10fce220 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce224 ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce228 madd w27, w25, w8, w9
cpu.x[27]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[25] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce22c ldp w24, w19, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[24]=(v0) as u32 as u64;cpu.x[19]=(v1) as u32 as u64;}
// 0x10fce230 b #0x10fce258
{// 0x10fce258 ldur x0, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce25c bl #0x10d358a0
cpu.x[30]=0x10fce260;super::chunk0::f_10d358a0(cpu,rt,0x10d358a0)?;
{// 0x10fce260 mul w8, w0, w26
cpu.x[8]=((((cpu.x[0] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce264 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce268 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce26c mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10fce270 mov x4, x21
cpu.x[4]=(((cpu.x[21]))) as u64;
// 0x10fce274 cmp w22, w8
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce278 csel w28, w24, w19, lt
cpu.x[28]=(if cpu.condition(11){((cpu.x[24] as u32 as u64))}else{((cpu.x[19] as u32 as u64))}) as u32 as u64;
// 0x10fce27c mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce280 bl #0x111d9240
cpu.x[30]=0x10fce284;super::chunk1::f_111d9240(cpu,rt,0x111d9240)?;
{// 0x10fce284 tbz w0, #0, #0x10fce248
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce248;continue;}{// 0x10fce288 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce28c ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce290 ldur x0, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce294 madd w26, w28, w8, w9
cpu.x[26]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[28] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce298 mov w2, w27
cpu.x[2]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce29c mov w1, w26
cpu.x[1]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce2a0 bl #0x10f94f40
cpu.x[30]=0x10fce2a4;super::chunk12::f_10f94f40(cpu,rt,0x10f94f40)?;
pc=0x10fce2a4;continue;}}}}}}}pc=0x10fce08c;continue;}}}
}
0x10fce038=>{
// 0x10fce038 b #0x10fce068
{// 0x10fce068 sub w8, w28, w19
{let a=((cpu.x[28] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fce06c bic w9, w8, w8, asr #31
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[8] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[9]=(result) as u32 as u64;}
// 0x10fce070 ldr w8, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce074 cmp w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce078 str w9, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce07c b.gt #0x10fce338
if cpu.condition(12){{// 0x10fce338 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
{// 0x10fce33c ldp x20, x19, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fce340 ldp x22, x21, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fce344 ldp x24, x23, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fce348 ldp x26, x25, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fce34c ldp x28, x27, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fce350 ldp x29, x30, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fce354 add sp, sp, #0xb0
{let a=((cpu.sp));let b=((0xb0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fce358 ret
cpu.pc=cpu.x[30];return Ok(());}}}{// 0x10fce080 mul w8, w26, w26
cpu.x[8]=((((cpu.x[26] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce084 stur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce088 tbz w20, #0, #0x10fce1e8
if ((cpu.x[20] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x10fce1e8 ldr w8, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1ec mov w9, #0x7fffffff
cpu.x[9]=(((0x7fffffffu64))) as u32 as u64;
// 0x10fce1f0 mov w26, #0xfa
cpu.x[26]=(((0xfau64))) as u32 as u64;
// 0x10fce1f4 str wzr, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1f8 sub w20, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[20]=(result) as u32 as u64;}
// 0x10fce1fc mov w8, #-1
cpu.x[8]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10fce200 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce204 mov w8, #0xfa
cpu.x[8]=(((0xfau64))) as u32 as u64;
// 0x10fce208 stp w8, w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=cpu.x[8] as u32 as u64;let v1=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10fce20c b #0x10fce220
{// 0x10fce220 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce224 ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce228 madd w27, w25, w8, w9
cpu.x[27]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[25] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce22c ldp w24, w19, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[24]=(v0) as u32 as u64;cpu.x[19]=(v1) as u32 as u64;}
// 0x10fce230 b #0x10fce258
{// 0x10fce258 ldur x0, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce25c bl #0x10d358a0
cpu.x[30]=0x10fce260;super::chunk0::f_10d358a0(cpu,rt,0x10d358a0)?;
{// 0x10fce260 mul w8, w0, w26
cpu.x[8]=((((cpu.x[0] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce264 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce268 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce26c mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10fce270 mov x4, x21
cpu.x[4]=(((cpu.x[21]))) as u64;
// 0x10fce274 cmp w22, w8
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce278 csel w28, w24, w19, lt
cpu.x[28]=(if cpu.condition(11){((cpu.x[24] as u32 as u64))}else{((cpu.x[19] as u32 as u64))}) as u32 as u64;
// 0x10fce27c mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce280 bl #0x111d9240
cpu.x[30]=0x10fce284;super::chunk1::f_111d9240(cpu,rt,0x111d9240)?;
{// 0x10fce284 tbz w0, #0, #0x10fce248
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce248;continue;}{// 0x10fce288 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce28c ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce290 ldur x0, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce294 madd w26, w28, w8, w9
cpu.x[26]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[28] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce298 mov w2, w27
cpu.x[2]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce29c mov w1, w26
cpu.x[1]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce2a0 bl #0x10f94f40
cpu.x[30]=0x10fce2a4;super::chunk12::f_10f94f40(cpu,rt,0x10f94f40)?;
{// 0x10fce2a4 ldur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce2a8 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce2ac b.gt #0x10fce244
if cpu.condition(12){pc=0x10fce244;continue;}{// 0x10fce2b0 ldur w3, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10fce2b4 mov w0, w26
cpu.x[0]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce2b8 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce2bc mov w2, w22
cpu.x[2]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10fce2c0 bl #0x11295120
cpu.x[30]=0x10fce2c4;super::chunk6::f_11295120(cpu,rt,0x11295120)?;
{// 0x10fce2c4 ldr w8, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce2c8 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fce2cc cbz w8, #0x10fce2fc
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10fce2fc;continue;}pc=0x10fce2d0;continue;}}}}}}}}}}pc=0x10fce08c;continue;}}
}
0x10fce03c=>{
// 0x10fce03c mov w20, #1
cpu.x[20]=(((0x1u64))) as u32 as u64;
// 0x10fce040 ldr x8, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fce044 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fce048 bic w25, w24, w24, asr #31
{let a=((cpu.x[24] as u32 as u64));let b=(((cpu.x[24] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[25]=(result) as u32 as u64;}
// 0x10fce04c ldr x8, [x8, #0x108]
{let address=(cpu.x[8]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fce050 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fce054;super::dispatch(cpu,rt,target)?;}
{// 0x10fce054 tbz w0, #0, #0x10fce01c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce01c;continue;}{// 0x10fce058 str wzr, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce05c ldr w8, [sp, #0x14]
{let address=(cpu.sp).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce060 cmp w25, w8
{let a=((cpu.x[25] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce064 b.gt #0x10fce338
if cpu.condition(12){{// 0x10fce338 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
{// 0x10fce33c ldp x20, x19, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fce340 ldp x22, x21, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fce344 ldp x24, x23, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fce348 ldp x26, x25, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fce34c ldp x28, x27, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fce350 ldp x29, x30, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fce354 add sp, sp, #0xb0
{let a=((cpu.sp));let b=((0xb0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fce358 ret
cpu.pc=cpu.x[30];return Ok(());}}}{// 0x10fce068 sub w8, w28, w19
{let a=((cpu.x[28] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fce06c bic w9, w8, w8, asr #31
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[8] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[9]=(result) as u32 as u64;}
// 0x10fce070 ldr w8, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce074 cmp w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce078 str w9, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce07c b.gt #0x10fce338
if cpu.condition(12){{// 0x10fce338 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
{// 0x10fce33c ldp x20, x19, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fce340 ldp x22, x21, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fce344 ldp x24, x23, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fce348 ldp x26, x25, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fce34c ldp x28, x27, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fce350 ldp x29, x30, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fce354 add sp, sp, #0xb0
{let a=((cpu.sp));let b=((0xb0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fce358 ret
cpu.pc=cpu.x[30];return Ok(());}}}{// 0x10fce080 mul w8, w26, w26
cpu.x[8]=((((cpu.x[26] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce084 stur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce088 tbz w20, #0, #0x10fce1e8
if ((cpu.x[20] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x10fce1e8 ldr w8, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1ec mov w9, #0x7fffffff
cpu.x[9]=(((0x7fffffffu64))) as u32 as u64;
// 0x10fce1f0 mov w26, #0xfa
cpu.x[26]=(((0xfau64))) as u32 as u64;
// 0x10fce1f4 str wzr, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1f8 sub w20, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[20]=(result) as u32 as u64;}
// 0x10fce1fc mov w8, #-1
cpu.x[8]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10fce200 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce204 mov w8, #0xfa
cpu.x[8]=(((0xfau64))) as u32 as u64;
// 0x10fce208 stp w8, w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=cpu.x[8] as u32 as u64;let v1=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10fce20c b #0x10fce220
{// 0x10fce220 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce224 ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce228 madd w27, w25, w8, w9
cpu.x[27]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[25] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce22c ldp w24, w19, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[24]=(v0) as u32 as u64;cpu.x[19]=(v1) as u32 as u64;}
// 0x10fce230 b #0x10fce258
{// 0x10fce258 ldur x0, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce25c bl #0x10d358a0
cpu.x[30]=0x10fce260;super::chunk0::f_10d358a0(cpu,rt,0x10d358a0)?;
{// 0x10fce260 mul w8, w0, w26
cpu.x[8]=((((cpu.x[0] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce264 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce268 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce26c mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10fce270 mov x4, x21
cpu.x[4]=(((cpu.x[21]))) as u64;
// 0x10fce274 cmp w22, w8
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce278 csel w28, w24, w19, lt
cpu.x[28]=(if cpu.condition(11){((cpu.x[24] as u32 as u64))}else{((cpu.x[19] as u32 as u64))}) as u32 as u64;
// 0x10fce27c mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce280 bl #0x111d9240
cpu.x[30]=0x10fce284;super::chunk1::f_111d9240(cpu,rt,0x111d9240)?;
{// 0x10fce284 tbz w0, #0, #0x10fce248
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce248;continue;}pc=0x10fce288;continue;}}}}}}pc=0x10fce08c;continue;}}}}
}
0x10fce054=>{
// 0x10fce054 tbz w0, #0, #0x10fce01c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce01c;continue;}{// 0x10fce058 str wzr, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce05c ldr w8, [sp, #0x14]
{let address=(cpu.sp).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce060 cmp w25, w8
{let a=((cpu.x[25] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce064 b.gt #0x10fce338
if cpu.condition(12){{// 0x10fce338 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
{// 0x10fce33c ldp x20, x19, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fce340 ldp x22, x21, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fce344 ldp x24, x23, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fce348 ldp x26, x25, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fce34c ldp x28, x27, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fce350 ldp x29, x30, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fce354 add sp, sp, #0xb0
{let a=((cpu.sp));let b=((0xb0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fce358 ret
cpu.pc=cpu.x[30];return Ok(());}}}{// 0x10fce068 sub w8, w28, w19
{let a=((cpu.x[28] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fce06c bic w9, w8, w8, asr #31
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[8] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[9]=(result) as u32 as u64;}
// 0x10fce070 ldr w8, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce074 cmp w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce078 str w9, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce07c b.gt #0x10fce338
if cpu.condition(12){{// 0x10fce338 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
{// 0x10fce33c ldp x20, x19, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fce340 ldp x22, x21, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fce344 ldp x24, x23, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fce348 ldp x26, x25, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fce34c ldp x28, x27, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fce350 ldp x29, x30, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fce354 add sp, sp, #0xb0
{let a=((cpu.sp));let b=((0xb0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fce358 ret
cpu.pc=cpu.x[30];return Ok(());}}}{// 0x10fce080 mul w8, w26, w26
cpu.x[8]=((((cpu.x[26] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce084 stur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce088 tbz w20, #0, #0x10fce1e8
if ((cpu.x[20] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x10fce1e8 ldr w8, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1ec mov w9, #0x7fffffff
cpu.x[9]=(((0x7fffffffu64))) as u32 as u64;
// 0x10fce1f0 mov w26, #0xfa
cpu.x[26]=(((0xfau64))) as u32 as u64;
// 0x10fce1f4 str wzr, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1f8 sub w20, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[20]=(result) as u32 as u64;}
// 0x10fce1fc mov w8, #-1
cpu.x[8]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10fce200 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce204 mov w8, #0xfa
cpu.x[8]=(((0xfau64))) as u32 as u64;
// 0x10fce208 stp w8, w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=cpu.x[8] as u32 as u64;let v1=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10fce20c b #0x10fce220
{// 0x10fce220 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce224 ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce228 madd w27, w25, w8, w9
cpu.x[27]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[25] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce22c ldp w24, w19, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[24]=(v0) as u32 as u64;cpu.x[19]=(v1) as u32 as u64;}
// 0x10fce230 b #0x10fce258
{// 0x10fce258 ldur x0, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce25c bl #0x10d358a0
cpu.x[30]=0x10fce260;super::chunk0::f_10d358a0(cpu,rt,0x10d358a0)?;
{// 0x10fce260 mul w8, w0, w26
cpu.x[8]=((((cpu.x[0] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce264 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce268 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce26c mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10fce270 mov x4, x21
cpu.x[4]=(((cpu.x[21]))) as u64;
// 0x10fce274 cmp w22, w8
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce278 csel w28, w24, w19, lt
cpu.x[28]=(if cpu.condition(11){((cpu.x[24] as u32 as u64))}else{((cpu.x[19] as u32 as u64))}) as u32 as u64;
// 0x10fce27c mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce280 bl #0x111d9240
cpu.x[30]=0x10fce284;super::chunk1::f_111d9240(cpu,rt,0x111d9240)?;
{// 0x10fce284 tbz w0, #0, #0x10fce248
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce248;continue;}pc=0x10fce288;continue;}}}}}}pc=0x10fce08c;continue;}}}
}
0x10fce058=>{
// 0x10fce058 str wzr, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce05c ldr w8, [sp, #0x14]
{let address=(cpu.sp).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce060 cmp w25, w8
{let a=((cpu.x[25] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce064 b.gt #0x10fce338
if cpu.condition(12){{// 0x10fce338 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
{// 0x10fce33c ldp x20, x19, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fce340 ldp x22, x21, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fce344 ldp x24, x23, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fce348 ldp x26, x25, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fce34c ldp x28, x27, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fce350 ldp x29, x30, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fce354 add sp, sp, #0xb0
{let a=((cpu.sp));let b=((0xb0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fce358 ret
cpu.pc=cpu.x[30];return Ok(());}}}{// 0x10fce068 sub w8, w28, w19
{let a=((cpu.x[28] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fce06c bic w9, w8, w8, asr #31
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[8] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[9]=(result) as u32 as u64;}
// 0x10fce070 ldr w8, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce074 cmp w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce078 str w9, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce07c b.gt #0x10fce338
if cpu.condition(12){{// 0x10fce338 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
{// 0x10fce33c ldp x20, x19, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fce340 ldp x22, x21, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fce344 ldp x24, x23, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fce348 ldp x26, x25, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fce34c ldp x28, x27, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fce350 ldp x29, x30, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fce354 add sp, sp, #0xb0
{let a=((cpu.sp));let b=((0xb0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fce358 ret
cpu.pc=cpu.x[30];return Ok(());}}}{// 0x10fce080 mul w8, w26, w26
cpu.x[8]=((((cpu.x[26] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce084 stur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce088 tbz w20, #0, #0x10fce1e8
if ((cpu.x[20] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x10fce1e8 ldr w8, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1ec mov w9, #0x7fffffff
cpu.x[9]=(((0x7fffffffu64))) as u32 as u64;
// 0x10fce1f0 mov w26, #0xfa
cpu.x[26]=(((0xfau64))) as u32 as u64;
// 0x10fce1f4 str wzr, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1f8 sub w20, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[20]=(result) as u32 as u64;}
// 0x10fce1fc mov w8, #-1
cpu.x[8]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10fce200 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce204 mov w8, #0xfa
cpu.x[8]=(((0xfau64))) as u32 as u64;
// 0x10fce208 stp w8, w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=cpu.x[8] as u32 as u64;let v1=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10fce20c b #0x10fce220
{// 0x10fce220 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce224 ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce228 madd w27, w25, w8, w9
cpu.x[27]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[25] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce22c ldp w24, w19, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[24]=(v0) as u32 as u64;cpu.x[19]=(v1) as u32 as u64;}
// 0x10fce230 b #0x10fce258
{// 0x10fce258 ldur x0, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce25c bl #0x10d358a0
cpu.x[30]=0x10fce260;super::chunk0::f_10d358a0(cpu,rt,0x10d358a0)?;
{// 0x10fce260 mul w8, w0, w26
cpu.x[8]=((((cpu.x[0] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce264 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce268 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce26c mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10fce270 mov x4, x21
cpu.x[4]=(((cpu.x[21]))) as u64;
// 0x10fce274 cmp w22, w8
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce278 csel w28, w24, w19, lt
cpu.x[28]=(if cpu.condition(11){((cpu.x[24] as u32 as u64))}else{((cpu.x[19] as u32 as u64))}) as u32 as u64;
// 0x10fce27c mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce280 bl #0x111d9240
cpu.x[30]=0x10fce284;super::chunk1::f_111d9240(cpu,rt,0x111d9240)?;
{// 0x10fce284 tbz w0, #0, #0x10fce248
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce248;continue;}{// 0x10fce288 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce28c ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce290 ldur x0, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce294 madd w26, w28, w8, w9
cpu.x[26]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[28] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce298 mov w2, w27
cpu.x[2]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce29c mov w1, w26
cpu.x[1]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce2a0 bl #0x10f94f40
cpu.x[30]=0x10fce2a4;super::chunk12::f_10f94f40(cpu,rt,0x10f94f40)?;
{// 0x10fce2a4 ldur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce2a8 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce2ac b.gt #0x10fce244
if cpu.condition(12){pc=0x10fce244;continue;}pc=0x10fce2b0;continue;}}}}}}}}pc=0x10fce08c;continue;}}
}
0x10fce068=>{
// 0x10fce068 sub w8, w28, w19
{let a=((cpu.x[28] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fce06c bic w9, w8, w8, asr #31
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[8] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[9]=(result) as u32 as u64;}
// 0x10fce070 ldr w8, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce074 cmp w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce078 str w9, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce07c b.gt #0x10fce338
if cpu.condition(12){{// 0x10fce338 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
{// 0x10fce33c ldp x20, x19, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fce340 ldp x22, x21, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fce344 ldp x24, x23, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fce348 ldp x26, x25, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fce34c ldp x28, x27, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fce350 ldp x29, x30, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fce354 add sp, sp, #0xb0
{let a=((cpu.sp));let b=((0xb0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fce358 ret
cpu.pc=cpu.x[30];return Ok(());}}}{// 0x10fce080 mul w8, w26, w26
cpu.x[8]=((((cpu.x[26] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce084 stur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce088 tbz w20, #0, #0x10fce1e8
if ((cpu.x[20] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x10fce1e8 ldr w8, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1ec mov w9, #0x7fffffff
cpu.x[9]=(((0x7fffffffu64))) as u32 as u64;
// 0x10fce1f0 mov w26, #0xfa
cpu.x[26]=(((0xfau64))) as u32 as u64;
// 0x10fce1f4 str wzr, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1f8 sub w20, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[20]=(result) as u32 as u64;}
// 0x10fce1fc mov w8, #-1
cpu.x[8]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10fce200 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce204 mov w8, #0xfa
cpu.x[8]=(((0xfau64))) as u32 as u64;
// 0x10fce208 stp w8, w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=cpu.x[8] as u32 as u64;let v1=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10fce20c b #0x10fce220
{// 0x10fce220 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce224 ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce228 madd w27, w25, w8, w9
cpu.x[27]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[25] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce22c ldp w24, w19, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[24]=(v0) as u32 as u64;cpu.x[19]=(v1) as u32 as u64;}
// 0x10fce230 b #0x10fce258
{// 0x10fce258 ldur x0, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce25c bl #0x10d358a0
cpu.x[30]=0x10fce260;super::chunk0::f_10d358a0(cpu,rt,0x10d358a0)?;
{// 0x10fce260 mul w8, w0, w26
cpu.x[8]=((((cpu.x[0] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce264 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce268 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce26c mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10fce270 mov x4, x21
cpu.x[4]=(((cpu.x[21]))) as u64;
// 0x10fce274 cmp w22, w8
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce278 csel w28, w24, w19, lt
cpu.x[28]=(if cpu.condition(11){((cpu.x[24] as u32 as u64))}else{((cpu.x[19] as u32 as u64))}) as u32 as u64;
// 0x10fce27c mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce280 bl #0x111d9240
cpu.x[30]=0x10fce284;super::chunk1::f_111d9240(cpu,rt,0x111d9240)?;
{// 0x10fce284 tbz w0, #0, #0x10fce248
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce248;continue;}{// 0x10fce288 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce28c ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce290 ldur x0, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce294 madd w26, w28, w8, w9
cpu.x[26]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[28] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce298 mov w2, w27
cpu.x[2]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce29c mov w1, w26
cpu.x[1]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce2a0 bl #0x10f94f40
cpu.x[30]=0x10fce2a4;super::chunk12::f_10f94f40(cpu,rt,0x10f94f40)?;
{// 0x10fce2a4 ldur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce2a8 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce2ac b.gt #0x10fce244
if cpu.condition(12){pc=0x10fce244;continue;}{// 0x10fce2b0 ldur w3, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10fce2b4 mov w0, w26
cpu.x[0]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce2b8 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce2bc mov w2, w22
cpu.x[2]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10fce2c0 bl #0x11295120
cpu.x[30]=0x10fce2c4;super::chunk6::f_11295120(cpu,rt,0x11295120)?;
{// 0x10fce2c4 ldr w8, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce2c8 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fce2cc cbz w8, #0x10fce2fc
if (cpu.x[8] as u32 as u64) == 0 {{// 0x10fce2fc mov w8, #2
cpu.x[8]=(((0x2u64))) as u32 as u64;
// 0x10fce300 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce304 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce308 b.gt #0x10fce234
if cpu.condition(12){pc=0x10fce234;continue;}{// 0x10fce30c b.ne #0x10fce244
if cpu.condition(1){pc=0x10fce244;continue;}pc=0x10fce310;continue;}}}pc=0x10fce2d0;continue;}}}}}}}}}}pc=0x10fce08c;continue;}
}
0x10fce080=>{
// 0x10fce080 mul w8, w26, w26
cpu.x[8]=((((cpu.x[26] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce084 stur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce088 tbz w20, #0, #0x10fce1e8
if ((cpu.x[20] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x10fce1e8 ldr w8, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1ec mov w9, #0x7fffffff
cpu.x[9]=(((0x7fffffffu64))) as u32 as u64;
// 0x10fce1f0 mov w26, #0xfa
cpu.x[26]=(((0xfau64))) as u32 as u64;
// 0x10fce1f4 str wzr, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1f8 sub w20, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[20]=(result) as u32 as u64;}
// 0x10fce1fc mov w8, #-1
cpu.x[8]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10fce200 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce204 mov w8, #0xfa
cpu.x[8]=(((0xfau64))) as u32 as u64;
// 0x10fce208 stp w8, w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=cpu.x[8] as u32 as u64;let v1=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10fce20c b #0x10fce220
{// 0x10fce220 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce224 ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce228 madd w27, w25, w8, w9
cpu.x[27]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[25] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce22c ldp w24, w19, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[24]=(v0) as u32 as u64;cpu.x[19]=(v1) as u32 as u64;}
// 0x10fce230 b #0x10fce258
{// 0x10fce258 ldur x0, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce25c bl #0x10d358a0
cpu.x[30]=0x10fce260;super::chunk0::f_10d358a0(cpu,rt,0x10d358a0)?;
{// 0x10fce260 mul w8, w0, w26
cpu.x[8]=((((cpu.x[0] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce264 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce268 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce26c mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10fce270 mov x4, x21
cpu.x[4]=(((cpu.x[21]))) as u64;
// 0x10fce274 cmp w22, w8
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce278 csel w28, w24, w19, lt
cpu.x[28]=(if cpu.condition(11){((cpu.x[24] as u32 as u64))}else{((cpu.x[19] as u32 as u64))}) as u32 as u64;
// 0x10fce27c mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce280 bl #0x111d9240
cpu.x[30]=0x10fce284;super::chunk1::f_111d9240(cpu,rt,0x111d9240)?;
{// 0x10fce284 tbz w0, #0, #0x10fce248
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce248;continue;}{// 0x10fce288 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce28c ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce290 ldur x0, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce294 madd w26, w28, w8, w9
cpu.x[26]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[28] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce298 mov w2, w27
cpu.x[2]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce29c mov w1, w26
cpu.x[1]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce2a0 bl #0x10f94f40
cpu.x[30]=0x10fce2a4;super::chunk12::f_10f94f40(cpu,rt,0x10f94f40)?;
{// 0x10fce2a4 ldur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce2a8 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce2ac b.gt #0x10fce244
if cpu.condition(12){pc=0x10fce244;continue;}{// 0x10fce2b0 ldur w3, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10fce2b4 mov w0, w26
cpu.x[0]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce2b8 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce2bc mov w2, w22
cpu.x[2]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10fce2c0 bl #0x11295120
cpu.x[30]=0x10fce2c4;super::chunk6::f_11295120(cpu,rt,0x11295120)?;
{// 0x10fce2c4 ldr w8, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce2c8 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fce2cc cbz w8, #0x10fce2fc
if (cpu.x[8] as u32 as u64) == 0 {{// 0x10fce2fc mov w8, #2
cpu.x[8]=(((0x2u64))) as u32 as u64;
// 0x10fce300 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce304 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce308 b.gt #0x10fce234
if cpu.condition(12){pc=0x10fce234;continue;}{// 0x10fce30c b.ne #0x10fce244
if cpu.condition(1){pc=0x10fce244;continue;}{// 0x10fce310 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce314 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce318 b.ge #0x10fce244
if cpu.condition(10){pc=0x10fce244;continue;}{// 0x10fce31c orr w8, w25, w28, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[28] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce320 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce324 b #0x10fce240
pc=0x10fce240;continue;}}}}}pc=0x10fce2d0;continue;}}}}}}}}}}pc=0x10fce08c;continue;
}
0x10fce08c=>{
// 0x10fce08c ldr w8, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce090 mov w9, #0x7fffffff
cpu.x[9]=(((0x7fffffffu64))) as u32 as u64;
// 0x10fce094 mov w26, #0xfa
cpu.x[26]=(((0xfau64))) as u32 as u64;
// 0x10fce098 eor w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a ^ b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce09c stp w8, wzr, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=cpu.x[8] as u32 as u64;let v1=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10fce0a0 ldr w8, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce0a4 sub w20, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[20]=(result) as u32 as u64;}
// 0x10fce0a8 mov w8, #-1
cpu.x[8]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10fce0ac stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce0b0 mov w8, #0xfa
cpu.x[8]=(((0xfau64))) as u32 as u64;
// 0x10fce0b4 stp w8, w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=cpu.x[8] as u32 as u64;let v1=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10fce0b8 b #0x10fce0cc
{// 0x10fce0cc mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce0d0 ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce0d4 madd w28, w25, w8, w9
cpu.x[28]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[25] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce0d8 ldp w19, w24, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[19]=(v0) as u32 as u64;cpu.x[24]=(v1) as u32 as u64;}
// 0x10fce0dc b #0x10fce104
{// 0x10fce104 ldur x0, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce108 bl #0x10d358a0
cpu.x[30]=0x10fce10c;super::chunk0::f_10d358a0(cpu,rt,0x10d358a0)?;
{// 0x10fce10c mul w8, w0, w26
cpu.x[8]=((((cpu.x[0] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce110 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce114 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce118 mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10fce11c mov x4, x21
cpu.x[4]=(((cpu.x[21]))) as u64;
// 0x10fce120 cmp w22, w8
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce124 csel w27, w19, w24, lt
cpu.x[27]=(if cpu.condition(11){((cpu.x[19] as u32 as u64))}else{((cpu.x[24] as u32 as u64))}) as u32 as u64;
// 0x10fce128 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce12c bl #0x111d9240
cpu.x[30]=0x10fce130;super::chunk1::f_111d9240(cpu,rt,0x111d9240)?;
{// 0x10fce130 tbz w0, #0, #0x10fce0f4
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce0f4;continue;}{// 0x10fce134 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce138 ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce13c ldur x0, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce140 madd w26, w27, w8, w9
cpu.x[26]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[27] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce144 mov w2, w28
cpu.x[2]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce148 mov w1, w26
cpu.x[1]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce14c bl #0x10f94f40
cpu.x[30]=0x10fce150;super::chunk12::f_10f94f40(cpu,rt,0x10f94f40)?;
{// 0x10fce150 ldur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce154 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce158 b.gt #0x10fce0f0
if cpu.condition(12){pc=0x10fce0f0;continue;}{// 0x10fce15c ldur w3, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10fce160 mov w0, w26
cpu.x[0]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce164 mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce168 mov w2, w22
cpu.x[2]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10fce16c bl #0x11295120
cpu.x[30]=0x10fce170;super::chunk6::f_11295120(cpu,rt,0x11295120)?;
{// 0x10fce170 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fce174 ldr x0, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce178 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce17c mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce180 bl #0x10e8b6e0
cpu.x[30]=0x10fce184;super::chunk0::f_10e8b6e0(cpu,rt,0x10e8b6e0)?;
{// 0x10fce184 ldr w8, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce188 orr w8, w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce18c tbz w8, #0, #0x10fce1a8
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x10fce1a8 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce1ac mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce1b0 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce1b4 bl #0x111d9674
cpu.x[30]=0x10fce1b8;super::chunk8::f_111d9674(cpu,rt,0x111d9674)?;
{// 0x10fce1b8 tbz w0, #0, #0x10fce194
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce194;continue;}{// 0x10fce1bc mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce1c0 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce1c4 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1c8 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}{// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}{// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;}}}}}}}{// 0x10fce190 tbnz w0, #0, #0x10fce1bc
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {{// 0x10fce1bc mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce1c0 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce1c4 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1c8 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}pc=0x10fce1d0;continue;}}}pc=0x10fce194;continue;}}}}}}}}}}
}
0x10fce0bc=>{
// 0x10fce0bc ldr w8, [sp, #0x14]
{let address=(cpu.sp).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce0c0 cmp w25, w8
{let a=((cpu.x[25] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce0c4 add w25, w25, #1
{let a=((cpu.x[25] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[25]=(result) as u32 as u64;}
// 0x10fce0c8 b.eq #0x10fce328
if cpu.condition(0){{// 0x10fce328 ldur w9, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce32c mov w8, #0x7fffffff
cpu.x[8]=(((0x7fffffffu64))) as u32 as u64;
// 0x10fce330 cmp w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce334 b.ne #0x10fce35c
if cpu.condition(1){{// 0x10fce35c ldur w19, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[19]=(v0) as u32 as u64;}
// 0x10fce360 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce364 mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10fce368 mov x4, x21
cpu.x[4]=(((cpu.x[21]))) as u64;
// 0x10fce36c asr w1, w19, #0x10
cpu.x[1]=(((((cpu.x[19] as u32 as u64))) as i32).wrapping_shr(((((0x10u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10fce370 and w2, w19, #0xffff
{let a=((cpu.x[19] as u32 as u64));let b=((0xffffu64));let result=a & b;cpu.x[2]=(result) as u32 as u64;}
// 0x10fce374 bl #0x111d9240
cpu.x[30]=0x10fce378;super::chunk1::f_111d9240(cpu,rt,0x111d9240)?;
{// 0x10fce378 mov w0, w19
cpu.x[0]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10fce37c b #0x10fce33c
pc=0x10fce33c;continue;}}}{// 0x10fce338 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
{// 0x10fce33c ldp x20, x19, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fce340 ldp x22, x21, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fce344 ldp x24, x23, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fce348 ldp x26, x25, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fce34c ldp x28, x27, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fce350 ldp x29, x30, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fce354 add sp, sp, #0xb0
{let a=((cpu.sp));let b=((0xb0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fce358 ret
cpu.pc=cpu.x[30];return Ok(());}}}}{// 0x10fce0cc mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce0d0 ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce0d4 madd w28, w25, w8, w9
cpu.x[28]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[25] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce0d8 ldp w19, w24, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[19]=(v0) as u32 as u64;cpu.x[24]=(v1) as u32 as u64;}
// 0x10fce0dc b #0x10fce104
{// 0x10fce104 ldur x0, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce108 bl #0x10d358a0
cpu.x[30]=0x10fce10c;super::chunk0::f_10d358a0(cpu,rt,0x10d358a0)?;
{// 0x10fce10c mul w8, w0, w26
cpu.x[8]=((((cpu.x[0] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce110 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce114 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce118 mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10fce11c mov x4, x21
cpu.x[4]=(((cpu.x[21]))) as u64;
// 0x10fce120 cmp w22, w8
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce124 csel w27, w19, w24, lt
cpu.x[27]=(if cpu.condition(11){((cpu.x[19] as u32 as u64))}else{((cpu.x[24] as u32 as u64))}) as u32 as u64;
// 0x10fce128 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce12c bl #0x111d9240
cpu.x[30]=0x10fce130;super::chunk1::f_111d9240(cpu,rt,0x111d9240)?;
{// 0x10fce130 tbz w0, #0, #0x10fce0f4
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce0f4;continue;}{// 0x10fce134 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce138 ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce13c ldur x0, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce140 madd w26, w27, w8, w9
cpu.x[26]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[27] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce144 mov w2, w28
cpu.x[2]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce148 mov w1, w26
cpu.x[1]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce14c bl #0x10f94f40
cpu.x[30]=0x10fce150;super::chunk12::f_10f94f40(cpu,rt,0x10f94f40)?;
{// 0x10fce150 ldur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce154 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce158 b.gt #0x10fce0f0
if cpu.condition(12){pc=0x10fce0f0;continue;}{// 0x10fce15c ldur w3, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10fce160 mov w0, w26
cpu.x[0]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce164 mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce168 mov w2, w22
cpu.x[2]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10fce16c bl #0x11295120
cpu.x[30]=0x10fce170;super::chunk6::f_11295120(cpu,rt,0x11295120)?;
{// 0x10fce170 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fce174 ldr x0, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce178 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce17c mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce180 bl #0x10e8b6e0
cpu.x[30]=0x10fce184;super::chunk0::f_10e8b6e0(cpu,rt,0x10e8b6e0)?;
{// 0x10fce184 ldr w8, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce188 orr w8, w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce18c tbz w8, #0, #0x10fce1a8
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce1a8;continue;}{// 0x10fce190 tbnz w0, #0, #0x10fce1bc
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10fce1bc;continue;}pc=0x10fce194;continue;}}}}}}}}}}
}
0x10fce0cc=>{
// 0x10fce0cc mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce0d0 ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce0d4 madd w28, w25, w8, w9
cpu.x[28]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[25] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce0d8 ldp w19, w24, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[19]=(v0) as u32 as u64;cpu.x[24]=(v1) as u32 as u64;}
// 0x10fce0dc b #0x10fce104
{// 0x10fce104 ldur x0, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce108 bl #0x10d358a0
cpu.x[30]=0x10fce10c;super::chunk0::f_10d358a0(cpu,rt,0x10d358a0)?;
{// 0x10fce10c mul w8, w0, w26
cpu.x[8]=((((cpu.x[0] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce110 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce114 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce118 mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10fce11c mov x4, x21
cpu.x[4]=(((cpu.x[21]))) as u64;
// 0x10fce120 cmp w22, w8
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce124 csel w27, w19, w24, lt
cpu.x[27]=(if cpu.condition(11){((cpu.x[19] as u32 as u64))}else{((cpu.x[24] as u32 as u64))}) as u32 as u64;
// 0x10fce128 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce12c bl #0x111d9240
cpu.x[30]=0x10fce130;super::chunk1::f_111d9240(cpu,rt,0x111d9240)?;
{// 0x10fce130 tbz w0, #0, #0x10fce0f4
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce0f4;continue;}{// 0x10fce134 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce138 ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce13c ldur x0, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce140 madd w26, w27, w8, w9
cpu.x[26]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[27] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce144 mov w2, w28
cpu.x[2]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce148 mov w1, w26
cpu.x[1]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce14c bl #0x10f94f40
cpu.x[30]=0x10fce150;super::chunk12::f_10f94f40(cpu,rt,0x10f94f40)?;
{// 0x10fce150 ldur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce154 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce158 b.gt #0x10fce0f0
if cpu.condition(12){pc=0x10fce0f0;continue;}{// 0x10fce15c ldur w3, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10fce160 mov w0, w26
cpu.x[0]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce164 mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce168 mov w2, w22
cpu.x[2]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10fce16c bl #0x11295120
cpu.x[30]=0x10fce170;super::chunk6::f_11295120(cpu,rt,0x11295120)?;
{// 0x10fce170 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fce174 ldr x0, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce178 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce17c mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce180 bl #0x10e8b6e0
cpu.x[30]=0x10fce184;super::chunk0::f_10e8b6e0(cpu,rt,0x10e8b6e0)?;
{// 0x10fce184 ldr w8, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce188 orr w8, w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce18c tbz w8, #0, #0x10fce1a8
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x10fce1a8 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce1ac mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce1b0 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce1b4 bl #0x111d9674
cpu.x[30]=0x10fce1b8;super::chunk8::f_111d9674(cpu,rt,0x111d9674)?;
{// 0x10fce1b8 tbz w0, #0, #0x10fce194
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce194;continue;}{// 0x10fce1bc mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce1c0 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce1c4 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1c8 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}{// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}{// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;}}}}}}}{// 0x10fce190 tbnz w0, #0, #0x10fce1bc
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {{// 0x10fce1bc mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce1c0 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce1c4 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1c8 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}{// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}{// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;}}}}}pc=0x10fce194;continue;}}}}}}}}}
}
0x10fce0e0=>{
// 0x10fce0e0 orr w9, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[9]=(result) as u32 as u64;}
// 0x10fce0e4 str w8, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce0e8 stur w9, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
{// 0x10fce0ec stur w26, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=cpu.x[26] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
{// 0x10fce0f0 mov w26, #0xfa
cpu.x[26]=(((0xfau64))) as u32 as u64;
{// 0x10fce0f4 sub w24, w24, #1
{let a=((cpu.x[24] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[24]=(result) as u32 as u64;}
// 0x10fce0f8 add w19, w19, #1
{let a=((cpu.x[19] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[19]=(result) as u32 as u64;}
// 0x10fce0fc cmp w20, w24
{let a=((cpu.x[20] as u32 as u64));let b=((cpu.x[24] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce100 b.eq #0x10fce0bc
if cpu.condition(0){pc=0x10fce0bc;continue;}{// 0x10fce104 ldur x0, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce108 bl #0x10d358a0
cpu.x[30]=0x10fce10c;super::chunk0::f_10d358a0(cpu,rt,0x10d358a0)?;
{// 0x10fce10c mul w8, w0, w26
cpu.x[8]=((((cpu.x[0] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce110 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce114 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce118 mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10fce11c mov x4, x21
cpu.x[4]=(((cpu.x[21]))) as u64;
// 0x10fce120 cmp w22, w8
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce124 csel w27, w19, w24, lt
cpu.x[27]=(if cpu.condition(11){((cpu.x[19] as u32 as u64))}else{((cpu.x[24] as u32 as u64))}) as u32 as u64;
// 0x10fce128 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce12c bl #0x111d9240
cpu.x[30]=0x10fce130;super::chunk1::f_111d9240(cpu,rt,0x111d9240)?;
{// 0x10fce130 tbz w0, #0, #0x10fce0f4
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce0f4;continue;}{// 0x10fce134 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce138 ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce13c ldur x0, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce140 madd w26, w27, w8, w9
cpu.x[26]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[27] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce144 mov w2, w28
cpu.x[2]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce148 mov w1, w26
cpu.x[1]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce14c bl #0x10f94f40
cpu.x[30]=0x10fce150;super::chunk12::f_10f94f40(cpu,rt,0x10f94f40)?;
{// 0x10fce150 ldur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce154 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce158 b.gt #0x10fce0f0
if cpu.condition(12){pc=0x10fce0f0;continue;}{// 0x10fce15c ldur w3, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10fce160 mov w0, w26
cpu.x[0]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce164 mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce168 mov w2, w22
cpu.x[2]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10fce16c bl #0x11295120
cpu.x[30]=0x10fce170;super::chunk6::f_11295120(cpu,rt,0x11295120)?;
{// 0x10fce170 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fce174 ldr x0, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce178 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce17c mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce180 bl #0x10e8b6e0
cpu.x[30]=0x10fce184;super::chunk0::f_10e8b6e0(cpu,rt,0x10e8b6e0)?;
{// 0x10fce184 ldr w8, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce188 orr w8, w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce18c tbz w8, #0, #0x10fce1a8
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x10fce1a8 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce1ac mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce1b0 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce1b4 bl #0x111d9674
cpu.x[30]=0x10fce1b8;super::chunk8::f_111d9674(cpu,rt,0x111d9674)?;
{// 0x10fce1b8 tbz w0, #0, #0x10fce194
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce194;continue;}{// 0x10fce1bc mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce1c0 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce1c4 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1c8 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}{// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}{// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;}}}}}}}{// 0x10fce190 tbnz w0, #0, #0x10fce1bc
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {{// 0x10fce1bc mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce1c0 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce1c4 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1c8 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}pc=0x10fce1d0;continue;}}}pc=0x10fce194;continue;}}}}}}}}}}}}
}
0x10fce0ec=>{
// 0x10fce0ec stur w26, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=cpu.x[26] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
{// 0x10fce0f0 mov w26, #0xfa
cpu.x[26]=(((0xfau64))) as u32 as u64;
{// 0x10fce0f4 sub w24, w24, #1
{let a=((cpu.x[24] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[24]=(result) as u32 as u64;}
// 0x10fce0f8 add w19, w19, #1
{let a=((cpu.x[19] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[19]=(result) as u32 as u64;}
// 0x10fce0fc cmp w20, w24
{let a=((cpu.x[20] as u32 as u64));let b=((cpu.x[24] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce100 b.eq #0x10fce0bc
if cpu.condition(0){pc=0x10fce0bc;continue;}{// 0x10fce104 ldur x0, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce108 bl #0x10d358a0
cpu.x[30]=0x10fce10c;super::chunk0::f_10d358a0(cpu,rt,0x10d358a0)?;
{// 0x10fce10c mul w8, w0, w26
cpu.x[8]=((((cpu.x[0] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce110 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce114 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce118 mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10fce11c mov x4, x21
cpu.x[4]=(((cpu.x[21]))) as u64;
// 0x10fce120 cmp w22, w8
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce124 csel w27, w19, w24, lt
cpu.x[27]=(if cpu.condition(11){((cpu.x[19] as u32 as u64))}else{((cpu.x[24] as u32 as u64))}) as u32 as u64;
// 0x10fce128 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce12c bl #0x111d9240
cpu.x[30]=0x10fce130;super::chunk1::f_111d9240(cpu,rt,0x111d9240)?;
{// 0x10fce130 tbz w0, #0, #0x10fce0f4
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce0f4;continue;}{// 0x10fce134 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce138 ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce13c ldur x0, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce140 madd w26, w27, w8, w9
cpu.x[26]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[27] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce144 mov w2, w28
cpu.x[2]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce148 mov w1, w26
cpu.x[1]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce14c bl #0x10f94f40
cpu.x[30]=0x10fce150;super::chunk12::f_10f94f40(cpu,rt,0x10f94f40)?;
{// 0x10fce150 ldur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce154 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce158 b.gt #0x10fce0f0
if cpu.condition(12){pc=0x10fce0f0;continue;}{// 0x10fce15c ldur w3, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10fce160 mov w0, w26
cpu.x[0]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce164 mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce168 mov w2, w22
cpu.x[2]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10fce16c bl #0x11295120
cpu.x[30]=0x10fce170;super::chunk6::f_11295120(cpu,rt,0x11295120)?;
{// 0x10fce170 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fce174 ldr x0, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce178 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce17c mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce180 bl #0x10e8b6e0
cpu.x[30]=0x10fce184;super::chunk0::f_10e8b6e0(cpu,rt,0x10e8b6e0)?;
{// 0x10fce184 ldr w8, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce188 orr w8, w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce18c tbz w8, #0, #0x10fce1a8
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x10fce1a8 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce1ac mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce1b0 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce1b4 bl #0x111d9674
cpu.x[30]=0x10fce1b8;super::chunk8::f_111d9674(cpu,rt,0x111d9674)?;
{// 0x10fce1b8 tbz w0, #0, #0x10fce194
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce194;continue;}{// 0x10fce1bc mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce1c0 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce1c4 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1c8 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}{// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}{// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;}}}}}}}{// 0x10fce190 tbnz w0, #0, #0x10fce1bc
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {{// 0x10fce1bc mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce1c0 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce1c4 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1c8 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}pc=0x10fce1d0;continue;}}}pc=0x10fce194;continue;}}}}}}}}}}}
}
0x10fce0f0=>{
// 0x10fce0f0 mov w26, #0xfa
cpu.x[26]=(((0xfau64))) as u32 as u64;
{// 0x10fce0f4 sub w24, w24, #1
{let a=((cpu.x[24] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[24]=(result) as u32 as u64;}
// 0x10fce0f8 add w19, w19, #1
{let a=((cpu.x[19] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[19]=(result) as u32 as u64;}
// 0x10fce0fc cmp w20, w24
{let a=((cpu.x[20] as u32 as u64));let b=((cpu.x[24] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce100 b.eq #0x10fce0bc
if cpu.condition(0){pc=0x10fce0bc;continue;}{// 0x10fce104 ldur x0, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce108 bl #0x10d358a0
cpu.x[30]=0x10fce10c;super::chunk0::f_10d358a0(cpu,rt,0x10d358a0)?;
{// 0x10fce10c mul w8, w0, w26
cpu.x[8]=((((cpu.x[0] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce110 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce114 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce118 mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10fce11c mov x4, x21
cpu.x[4]=(((cpu.x[21]))) as u64;
// 0x10fce120 cmp w22, w8
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce124 csel w27, w19, w24, lt
cpu.x[27]=(if cpu.condition(11){((cpu.x[19] as u32 as u64))}else{((cpu.x[24] as u32 as u64))}) as u32 as u64;
// 0x10fce128 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce12c bl #0x111d9240
cpu.x[30]=0x10fce130;super::chunk1::f_111d9240(cpu,rt,0x111d9240)?;
{// 0x10fce130 tbz w0, #0, #0x10fce0f4
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce0f4;continue;}{// 0x10fce134 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce138 ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce13c ldur x0, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce140 madd w26, w27, w8, w9
cpu.x[26]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[27] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce144 mov w2, w28
cpu.x[2]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce148 mov w1, w26
cpu.x[1]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce14c bl #0x10f94f40
cpu.x[30]=0x10fce150;super::chunk12::f_10f94f40(cpu,rt,0x10f94f40)?;
{// 0x10fce150 ldur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce154 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce158 b.gt #0x10fce0f0
if cpu.condition(12){pc=0x10fce0f0;continue;}{// 0x10fce15c ldur w3, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10fce160 mov w0, w26
cpu.x[0]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce164 mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce168 mov w2, w22
cpu.x[2]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10fce16c bl #0x11295120
cpu.x[30]=0x10fce170;super::chunk6::f_11295120(cpu,rt,0x11295120)?;
{// 0x10fce170 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fce174 ldr x0, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce178 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce17c mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce180 bl #0x10e8b6e0
cpu.x[30]=0x10fce184;super::chunk0::f_10e8b6e0(cpu,rt,0x10e8b6e0)?;
{// 0x10fce184 ldr w8, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce188 orr w8, w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce18c tbz w8, #0, #0x10fce1a8
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x10fce1a8 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce1ac mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce1b0 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce1b4 bl #0x111d9674
cpu.x[30]=0x10fce1b8;super::chunk8::f_111d9674(cpu,rt,0x111d9674)?;
{// 0x10fce1b8 tbz w0, #0, #0x10fce194
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce194;continue;}{// 0x10fce1bc mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce1c0 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce1c4 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1c8 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}{// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}{// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;}}}}}}}{// 0x10fce190 tbnz w0, #0, #0x10fce1bc
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {{// 0x10fce1bc mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce1c0 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce1c4 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1c8 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}{// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}pc=0x10fce1dc;continue;}}}}pc=0x10fce194;continue;}}}}}}}}}}
}
0x10fce0f4=>{
// 0x10fce0f4 sub w24, w24, #1
{let a=((cpu.x[24] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[24]=(result) as u32 as u64;}
// 0x10fce0f8 add w19, w19, #1
{let a=((cpu.x[19] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[19]=(result) as u32 as u64;}
// 0x10fce0fc cmp w20, w24
{let a=((cpu.x[20] as u32 as u64));let b=((cpu.x[24] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce100 b.eq #0x10fce0bc
if cpu.condition(0){pc=0x10fce0bc;continue;}{// 0x10fce104 ldur x0, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce108 bl #0x10d358a0
cpu.x[30]=0x10fce10c;super::chunk0::f_10d358a0(cpu,rt,0x10d358a0)?;
{// 0x10fce10c mul w8, w0, w26
cpu.x[8]=((((cpu.x[0] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce110 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce114 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce118 mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10fce11c mov x4, x21
cpu.x[4]=(((cpu.x[21]))) as u64;
// 0x10fce120 cmp w22, w8
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce124 csel w27, w19, w24, lt
cpu.x[27]=(if cpu.condition(11){((cpu.x[19] as u32 as u64))}else{((cpu.x[24] as u32 as u64))}) as u32 as u64;
// 0x10fce128 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce12c bl #0x111d9240
cpu.x[30]=0x10fce130;super::chunk1::f_111d9240(cpu,rt,0x111d9240)?;
{// 0x10fce130 tbz w0, #0, #0x10fce0f4
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce0f4;continue;}{// 0x10fce134 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce138 ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce13c ldur x0, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce140 madd w26, w27, w8, w9
cpu.x[26]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[27] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce144 mov w2, w28
cpu.x[2]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce148 mov w1, w26
cpu.x[1]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce14c bl #0x10f94f40
cpu.x[30]=0x10fce150;super::chunk12::f_10f94f40(cpu,rt,0x10f94f40)?;
{// 0x10fce150 ldur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce154 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce158 b.gt #0x10fce0f0
if cpu.condition(12){pc=0x10fce0f0;continue;}{// 0x10fce15c ldur w3, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10fce160 mov w0, w26
cpu.x[0]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce164 mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce168 mov w2, w22
cpu.x[2]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10fce16c bl #0x11295120
cpu.x[30]=0x10fce170;super::chunk6::f_11295120(cpu,rt,0x11295120)?;
{// 0x10fce170 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fce174 ldr x0, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce178 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce17c mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce180 bl #0x10e8b6e0
cpu.x[30]=0x10fce184;super::chunk0::f_10e8b6e0(cpu,rt,0x10e8b6e0)?;
{// 0x10fce184 ldr w8, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce188 orr w8, w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce18c tbz w8, #0, #0x10fce1a8
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x10fce1a8 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce1ac mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce1b0 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce1b4 bl #0x111d9674
cpu.x[30]=0x10fce1b8;super::chunk8::f_111d9674(cpu,rt,0x111d9674)?;
{// 0x10fce1b8 tbz w0, #0, #0x10fce194
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce194;continue;}{// 0x10fce1bc mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce1c0 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce1c4 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1c8 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}{// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}{// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;}}}}}}}{// 0x10fce190 tbnz w0, #0, #0x10fce1bc
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {{// 0x10fce1bc mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce1c0 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce1c4 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1c8 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}{// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}{// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;}}}}}pc=0x10fce194;continue;}}}}}}}}}
}
0x10fce104=>{
// 0x10fce104 ldur x0, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce108 bl #0x10d358a0
cpu.x[30]=0x10fce10c;super::chunk0::f_10d358a0(cpu,rt,0x10d358a0)?;
{// 0x10fce10c mul w8, w0, w26
cpu.x[8]=((((cpu.x[0] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce110 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce114 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce118 mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10fce11c mov x4, x21
cpu.x[4]=(((cpu.x[21]))) as u64;
// 0x10fce120 cmp w22, w8
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce124 csel w27, w19, w24, lt
cpu.x[27]=(if cpu.condition(11){((cpu.x[19] as u32 as u64))}else{((cpu.x[24] as u32 as u64))}) as u32 as u64;
// 0x10fce128 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce12c bl #0x111d9240
cpu.x[30]=0x10fce130;super::chunk1::f_111d9240(cpu,rt,0x111d9240)?;
{// 0x10fce130 tbz w0, #0, #0x10fce0f4
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce0f4;continue;}{// 0x10fce134 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce138 ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce13c ldur x0, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce140 madd w26, w27, w8, w9
cpu.x[26]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[27] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce144 mov w2, w28
cpu.x[2]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce148 mov w1, w26
cpu.x[1]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce14c bl #0x10f94f40
cpu.x[30]=0x10fce150;super::chunk12::f_10f94f40(cpu,rt,0x10f94f40)?;
{// 0x10fce150 ldur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce154 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce158 b.gt #0x10fce0f0
if cpu.condition(12){pc=0x10fce0f0;continue;}{// 0x10fce15c ldur w3, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10fce160 mov w0, w26
cpu.x[0]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce164 mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce168 mov w2, w22
cpu.x[2]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10fce16c bl #0x11295120
cpu.x[30]=0x10fce170;super::chunk6::f_11295120(cpu,rt,0x11295120)?;
{// 0x10fce170 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fce174 ldr x0, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce178 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce17c mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce180 bl #0x10e8b6e0
cpu.x[30]=0x10fce184;super::chunk0::f_10e8b6e0(cpu,rt,0x10e8b6e0)?;
{// 0x10fce184 ldr w8, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce188 orr w8, w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce18c tbz w8, #0, #0x10fce1a8
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x10fce1a8 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce1ac mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce1b0 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce1b4 bl #0x111d9674
cpu.x[30]=0x10fce1b8;super::chunk8::f_111d9674(cpu,rt,0x111d9674)?;
{// 0x10fce1b8 tbz w0, #0, #0x10fce194
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce194;continue;}{// 0x10fce1bc mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce1c0 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce1c4 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1c8 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}{// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}{// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;}}}}}}}{// 0x10fce190 tbnz w0, #0, #0x10fce1bc
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {{// 0x10fce1bc mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce1c0 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce1c4 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1c8 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}{// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}{// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;}}}}}pc=0x10fce194;continue;}}}}}}}}
}
0x10fce10c=>{
// 0x10fce10c mul w8, w0, w26
cpu.x[8]=((((cpu.x[0] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce110 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce114 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce118 mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10fce11c mov x4, x21
cpu.x[4]=(((cpu.x[21]))) as u64;
// 0x10fce120 cmp w22, w8
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce124 csel w27, w19, w24, lt
cpu.x[27]=(if cpu.condition(11){((cpu.x[19] as u32 as u64))}else{((cpu.x[24] as u32 as u64))}) as u32 as u64;
// 0x10fce128 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce12c bl #0x111d9240
cpu.x[30]=0x10fce130;super::chunk1::f_111d9240(cpu,rt,0x111d9240)?;
{// 0x10fce130 tbz w0, #0, #0x10fce0f4
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce0f4;continue;}{// 0x10fce134 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce138 ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce13c ldur x0, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce140 madd w26, w27, w8, w9
cpu.x[26]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[27] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce144 mov w2, w28
cpu.x[2]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce148 mov w1, w26
cpu.x[1]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce14c bl #0x10f94f40
cpu.x[30]=0x10fce150;super::chunk12::f_10f94f40(cpu,rt,0x10f94f40)?;
{// 0x10fce150 ldur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce154 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce158 b.gt #0x10fce0f0
if cpu.condition(12){pc=0x10fce0f0;continue;}{// 0x10fce15c ldur w3, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10fce160 mov w0, w26
cpu.x[0]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce164 mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce168 mov w2, w22
cpu.x[2]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10fce16c bl #0x11295120
cpu.x[30]=0x10fce170;super::chunk6::f_11295120(cpu,rt,0x11295120)?;
{// 0x10fce170 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fce174 ldr x0, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce178 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce17c mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce180 bl #0x10e8b6e0
cpu.x[30]=0x10fce184;super::chunk0::f_10e8b6e0(cpu,rt,0x10e8b6e0)?;
{// 0x10fce184 ldr w8, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce188 orr w8, w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce18c tbz w8, #0, #0x10fce1a8
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x10fce1a8 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce1ac mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce1b0 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce1b4 bl #0x111d9674
cpu.x[30]=0x10fce1b8;super::chunk8::f_111d9674(cpu,rt,0x111d9674)?;
{// 0x10fce1b8 tbz w0, #0, #0x10fce194
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce194;continue;}{// 0x10fce1bc mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce1c0 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce1c4 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1c8 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}{// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}{// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;}}}}}}}{// 0x10fce190 tbnz w0, #0, #0x10fce1bc
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {{// 0x10fce1bc mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce1c0 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce1c4 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1c8 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}{// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}{// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;}}}}}{// 0x10fce194 mov w8, #2
cpu.x[8]=(((0x2u64))) as u32 as u64;
// 0x10fce198 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce19c cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1a0 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1a4 b #0x10fce1cc
{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}{// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}{// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;}}}}}}}}}}}}
}
0x10fce130=>{
// 0x10fce130 tbz w0, #0, #0x10fce0f4
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce0f4;continue;}{// 0x10fce134 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce138 ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce13c ldur x0, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce140 madd w26, w27, w8, w9
cpu.x[26]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[27] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce144 mov w2, w28
cpu.x[2]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce148 mov w1, w26
cpu.x[1]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce14c bl #0x10f94f40
cpu.x[30]=0x10fce150;super::chunk12::f_10f94f40(cpu,rt,0x10f94f40)?;
{// 0x10fce150 ldur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce154 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce158 b.gt #0x10fce0f0
if cpu.condition(12){pc=0x10fce0f0;continue;}{// 0x10fce15c ldur w3, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10fce160 mov w0, w26
cpu.x[0]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce164 mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce168 mov w2, w22
cpu.x[2]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10fce16c bl #0x11295120
cpu.x[30]=0x10fce170;super::chunk6::f_11295120(cpu,rt,0x11295120)?;
{// 0x10fce170 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fce174 ldr x0, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce178 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce17c mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce180 bl #0x10e8b6e0
cpu.x[30]=0x10fce184;super::chunk0::f_10e8b6e0(cpu,rt,0x10e8b6e0)?;
{// 0x10fce184 ldr w8, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce188 orr w8, w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce18c tbz w8, #0, #0x10fce1a8
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x10fce1a8 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce1ac mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce1b0 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce1b4 bl #0x111d9674
cpu.x[30]=0x10fce1b8;super::chunk8::f_111d9674(cpu,rt,0x111d9674)?;
{// 0x10fce1b8 tbz w0, #0, #0x10fce194
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce194;continue;}{// 0x10fce1bc mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce1c0 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce1c4 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1c8 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}{// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}{// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;}}}}}}}{// 0x10fce190 tbnz w0, #0, #0x10fce1bc
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {{// 0x10fce1bc mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce1c0 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce1c4 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1c8 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}{// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}{// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;}}}}}{// 0x10fce194 mov w8, #2
cpu.x[8]=(((0x2u64))) as u32 as u64;
// 0x10fce198 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce19c cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1a0 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1a4 b #0x10fce1cc
{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}{// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}{// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;}}}}}}}}}}}
}
0x10fce134=>{
// 0x10fce134 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce138 ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce13c ldur x0, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce140 madd w26, w27, w8, w9
cpu.x[26]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[27] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce144 mov w2, w28
cpu.x[2]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce148 mov w1, w26
cpu.x[1]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce14c bl #0x10f94f40
cpu.x[30]=0x10fce150;super::chunk12::f_10f94f40(cpu,rt,0x10f94f40)?;
{// 0x10fce150 ldur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce154 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce158 b.gt #0x10fce0f0
if cpu.condition(12){pc=0x10fce0f0;continue;}{// 0x10fce15c ldur w3, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10fce160 mov w0, w26
cpu.x[0]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce164 mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce168 mov w2, w22
cpu.x[2]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10fce16c bl #0x11295120
cpu.x[30]=0x10fce170;super::chunk6::f_11295120(cpu,rt,0x11295120)?;
{// 0x10fce170 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fce174 ldr x0, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce178 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce17c mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce180 bl #0x10e8b6e0
cpu.x[30]=0x10fce184;super::chunk0::f_10e8b6e0(cpu,rt,0x10e8b6e0)?;
{// 0x10fce184 ldr w8, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce188 orr w8, w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce18c tbz w8, #0, #0x10fce1a8
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x10fce1a8 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce1ac mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce1b0 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce1b4 bl #0x111d9674
cpu.x[30]=0x10fce1b8;super::chunk8::f_111d9674(cpu,rt,0x111d9674)?;
{// 0x10fce1b8 tbz w0, #0, #0x10fce194
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce194;continue;}{// 0x10fce1bc mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce1c0 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce1c4 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1c8 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}{// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}{// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;}}}}}}}{// 0x10fce190 tbnz w0, #0, #0x10fce1bc
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {{// 0x10fce1bc mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce1c0 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce1c4 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1c8 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}{// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}{// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;}}}}}{// 0x10fce194 mov w8, #2
cpu.x[8]=(((0x2u64))) as u32 as u64;
// 0x10fce198 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce19c cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1a0 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1a4 b #0x10fce1cc
{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}{// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}{// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;}}}}}}}}}}
}
0x10fce150=>{
// 0x10fce150 ldur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce154 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce158 b.gt #0x10fce0f0
if cpu.condition(12){pc=0x10fce0f0;continue;}{// 0x10fce15c ldur w3, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10fce160 mov w0, w26
cpu.x[0]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce164 mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce168 mov w2, w22
cpu.x[2]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10fce16c bl #0x11295120
cpu.x[30]=0x10fce170;super::chunk6::f_11295120(cpu,rt,0x11295120)?;
{// 0x10fce170 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fce174 ldr x0, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce178 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce17c mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce180 bl #0x10e8b6e0
cpu.x[30]=0x10fce184;super::chunk0::f_10e8b6e0(cpu,rt,0x10e8b6e0)?;
{// 0x10fce184 ldr w8, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce188 orr w8, w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce18c tbz w8, #0, #0x10fce1a8
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x10fce1a8 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce1ac mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce1b0 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce1b4 bl #0x111d9674
cpu.x[30]=0x10fce1b8;super::chunk8::f_111d9674(cpu,rt,0x111d9674)?;
{// 0x10fce1b8 tbz w0, #0, #0x10fce194
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce194;continue;}{// 0x10fce1bc mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce1c0 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce1c4 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1c8 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}{// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}{// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;}}}}}}}{// 0x10fce190 tbnz w0, #0, #0x10fce1bc
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {{// 0x10fce1bc mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce1c0 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce1c4 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1c8 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}{// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}{// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;}}}}}{// 0x10fce194 mov w8, #2
cpu.x[8]=(((0x2u64))) as u32 as u64;
// 0x10fce198 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce19c cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1a0 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1a4 b #0x10fce1cc
{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}{// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}{// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;}}}}}}}}}
}
0x10fce15c=>{
// 0x10fce15c ldur w3, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10fce160 mov w0, w26
cpu.x[0]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce164 mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce168 mov w2, w22
cpu.x[2]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10fce16c bl #0x11295120
cpu.x[30]=0x10fce170;super::chunk6::f_11295120(cpu,rt,0x11295120)?;
{// 0x10fce170 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fce174 ldr x0, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce178 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce17c mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce180 bl #0x10e8b6e0
cpu.x[30]=0x10fce184;super::chunk0::f_10e8b6e0(cpu,rt,0x10e8b6e0)?;
{// 0x10fce184 ldr w8, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce188 orr w8, w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce18c tbz w8, #0, #0x10fce1a8
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x10fce1a8 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce1ac mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce1b0 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce1b4 bl #0x111d9674
cpu.x[30]=0x10fce1b8;super::chunk8::f_111d9674(cpu,rt,0x111d9674)?;
{// 0x10fce1b8 tbz w0, #0, #0x10fce194
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce194;continue;}{// 0x10fce1bc mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce1c0 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce1c4 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1c8 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}{// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}{// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;}}}}}}}{// 0x10fce190 tbnz w0, #0, #0x10fce1bc
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {{// 0x10fce1bc mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce1c0 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce1c4 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1c8 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}{// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}{// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;}}}}}{// 0x10fce194 mov w8, #2
cpu.x[8]=(((0x2u64))) as u32 as u64;
// 0x10fce198 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce19c cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1a0 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1a4 b #0x10fce1cc
{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}{// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}{// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;}}}}}}}}
}
0x10fce170=>{
// 0x10fce170 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fce174 ldr x0, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce178 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce17c mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce180 bl #0x10e8b6e0
cpu.x[30]=0x10fce184;super::chunk0::f_10e8b6e0(cpu,rt,0x10e8b6e0)?;
{// 0x10fce184 ldr w8, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce188 orr w8, w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce18c tbz w8, #0, #0x10fce1a8
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x10fce1a8 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce1ac mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce1b0 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce1b4 bl #0x111d9674
cpu.x[30]=0x10fce1b8;super::chunk8::f_111d9674(cpu,rt,0x111d9674)?;
{// 0x10fce1b8 tbz w0, #0, #0x10fce194
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce194;continue;}{// 0x10fce1bc mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce1c0 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce1c4 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1c8 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}{// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}{// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;}}}}}}}{// 0x10fce190 tbnz w0, #0, #0x10fce1bc
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {{// 0x10fce1bc mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce1c0 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce1c4 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1c8 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}{// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}{// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;}}}}}{// 0x10fce194 mov w8, #2
cpu.x[8]=(((0x2u64))) as u32 as u64;
// 0x10fce198 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce19c cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1a0 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1a4 b #0x10fce1cc
{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}{// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}{// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;}}}}}}}
}
0x10fce184=>{
// 0x10fce184 ldr w8, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce188 orr w8, w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce18c tbz w8, #0, #0x10fce1a8
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) == 0 {{// 0x10fce1a8 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce1ac mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce1b0 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce1b4 bl #0x111d9674
cpu.x[30]=0x10fce1b8;super::chunk8::f_111d9674(cpu,rt,0x111d9674)?;
{// 0x10fce1b8 tbz w0, #0, #0x10fce194
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce194;continue;}{// 0x10fce1bc mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce1c0 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce1c4 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1c8 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}{// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}{// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;}}}}}}}{// 0x10fce190 tbnz w0, #0, #0x10fce1bc
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {{// 0x10fce1bc mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce1c0 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce1c4 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1c8 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}{// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}{// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;}}}}}{// 0x10fce194 mov w8, #2
cpu.x[8]=(((0x2u64))) as u32 as u64;
// 0x10fce198 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce19c cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1a0 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1a4 b #0x10fce1cc
{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}{// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}{// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;}}}}}}
}
0x10fce190=>{
// 0x10fce190 tbnz w0, #0, #0x10fce1bc
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {{// 0x10fce1bc mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce1c0 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce1c4 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1c8 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}{// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}{// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;}}}}}{// 0x10fce194 mov w8, #2
cpu.x[8]=(((0x2u64))) as u32 as u64;
// 0x10fce198 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce19c cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1a0 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1a4 b #0x10fce1cc
{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}{// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}{// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;}}}}}
}
0x10fce194=>{
// 0x10fce194 mov w8, #2
cpu.x[8]=(((0x2u64))) as u32 as u64;
// 0x10fce198 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce19c cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1a0 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1a4 b #0x10fce1cc
{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}{// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}{// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;}}}}
}
0x10fce1a4=>{
// 0x10fce1a4 b #0x10fce1cc
{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}{// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}{// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;}}}
}
0x10fce1a8=>{
// 0x10fce1a8 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce1ac mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce1b0 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce1b4 bl #0x111d9674
cpu.x[30]=0x10fce1b8;super::chunk8::f_111d9674(cpu,rt,0x111d9674)?;
{// 0x10fce1b8 tbz w0, #0, #0x10fce194
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce194;continue;}{// 0x10fce1bc mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce1c0 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce1c4 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1c8 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}{// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}{// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;}}}}}
}
0x10fce1b8=>{
// 0x10fce1b8 tbz w0, #0, #0x10fce194
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce194;continue;}{// 0x10fce1bc mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce1c0 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce1c4 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1c8 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}{// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}{// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;}}}}
}
0x10fce1bc=>{
// 0x10fce1bc mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce1c0 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce1c4 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1c8 b.gt #0x10fce0e0
if cpu.condition(12){pc=0x10fce0e0;continue;}{// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}{// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}{// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;}}}
}
0x10fce1cc=>{
// 0x10fce1cc b.ne #0x10fce0f0
if cpu.condition(1){pc=0x10fce0f0;continue;}{// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}{// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;}}
}
0x10fce1d0=>{
// 0x10fce1d0 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1d4 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce1d8 b.ge #0x10fce0f0
if cpu.condition(10){pc=0x10fce0f0;continue;}{// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;}
}
0x10fce1dc=>{
// 0x10fce1dc orr w8, w25, w27, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[27] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce1e0 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1e4 b #0x10fce0ec
pc=0x10fce0ec;continue;
}
0x10fce1e8=>{
// 0x10fce1e8 ldr w8, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce1ec mov w9, #0x7fffffff
cpu.x[9]=(((0x7fffffffu64))) as u32 as u64;
// 0x10fce1f0 mov w26, #0xfa
cpu.x[26]=(((0xfau64))) as u32 as u64;
// 0x10fce1f4 str wzr, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce1f8 sub w20, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[20]=(result) as u32 as u64;}
// 0x10fce1fc mov w8, #-1
cpu.x[8]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10fce200 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce204 mov w8, #0xfa
cpu.x[8]=(((0xfau64))) as u32 as u64;
// 0x10fce208 stp w8, w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=cpu.x[8] as u32 as u64;let v1=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10fce20c b #0x10fce220
{// 0x10fce220 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce224 ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce228 madd w27, w25, w8, w9
cpu.x[27]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[25] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce22c ldp w24, w19, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[24]=(v0) as u32 as u64;cpu.x[19]=(v1) as u32 as u64;}
// 0x10fce230 b #0x10fce258
{// 0x10fce258 ldur x0, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce25c bl #0x10d358a0
cpu.x[30]=0x10fce260;super::chunk0::f_10d358a0(cpu,rt,0x10d358a0)?;
{// 0x10fce260 mul w8, w0, w26
cpu.x[8]=((((cpu.x[0] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce264 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce268 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce26c mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10fce270 mov x4, x21
cpu.x[4]=(((cpu.x[21]))) as u64;
// 0x10fce274 cmp w22, w8
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce278 csel w28, w24, w19, lt
cpu.x[28]=(if cpu.condition(11){((cpu.x[24] as u32 as u64))}else{((cpu.x[19] as u32 as u64))}) as u32 as u64;
// 0x10fce27c mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce280 bl #0x111d9240
cpu.x[30]=0x10fce284;super::chunk1::f_111d9240(cpu,rt,0x111d9240)?;
{// 0x10fce284 tbz w0, #0, #0x10fce248
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce248;continue;}{// 0x10fce288 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce28c ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce290 ldur x0, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce294 madd w26, w28, w8, w9
cpu.x[26]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[28] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce298 mov w2, w27
cpu.x[2]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce29c mov w1, w26
cpu.x[1]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce2a0 bl #0x10f94f40
cpu.x[30]=0x10fce2a4;super::chunk12::f_10f94f40(cpu,rt,0x10f94f40)?;
{// 0x10fce2a4 ldur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce2a8 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce2ac b.gt #0x10fce244
if cpu.condition(12){pc=0x10fce244;continue;}{// 0x10fce2b0 ldur w3, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10fce2b4 mov w0, w26
cpu.x[0]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce2b8 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce2bc mov w2, w22
cpu.x[2]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10fce2c0 bl #0x11295120
cpu.x[30]=0x10fce2c4;super::chunk6::f_11295120(cpu,rt,0x11295120)?;
{// 0x10fce2c4 ldr w8, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce2c8 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fce2cc cbz w8, #0x10fce2fc
if (cpu.x[8] as u32 as u64) == 0 {{// 0x10fce2fc mov w8, #2
cpu.x[8]=(((0x2u64))) as u32 as u64;
// 0x10fce300 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce304 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce308 b.gt #0x10fce234
if cpu.condition(12){pc=0x10fce234;continue;}{// 0x10fce30c b.ne #0x10fce244
if cpu.condition(1){pc=0x10fce244;continue;}{// 0x10fce310 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce314 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce318 b.ge #0x10fce244
if cpu.condition(10){pc=0x10fce244;continue;}{// 0x10fce31c orr w8, w25, w28, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[28] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce320 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce324 b #0x10fce240
pc=0x10fce240;continue;}}}}}{// 0x10fce2d0 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce2d4 mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce2d8 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce2dc bl #0x111d9674
cpu.x[30]=0x10fce2e0;super::chunk8::f_111d9674(cpu,rt,0x111d9674)?;
// 0x10fce2e0 tst w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.logical_flags(result,32);}
// 0x10fce2e4 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce2e8 cinc w8, w8, eq
cpu.x[8]=(if cpu.condition(0){(((cpu.x[8] as u32 as u64))).wrapping_add(1)}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10fce2ec ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce2f0 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce2f4 b.gt #0x10fce234
if cpu.condition(12){pc=0x10fce234;continue;}{// 0x10fce2f8 b #0x10fce30c
{// 0x10fce30c b.ne #0x10fce244
if cpu.condition(1){pc=0x10fce244;continue;}{// 0x10fce310 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce314 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce318 b.ge #0x10fce244
if cpu.condition(10){pc=0x10fce244;continue;}{// 0x10fce31c orr w8, w25, w28, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[28] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce320 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce324 b #0x10fce240
pc=0x10fce240;continue;}}}}}}}}}}}}}
}
0x10fce210=>{
// 0x10fce210 ldr w8, [sp, #0x14]
{let address=(cpu.sp).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce214 cmp w25, w8
{let a=((cpu.x[25] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce218 add w25, w25, #1
{let a=((cpu.x[25] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[25]=(result) as u32 as u64;}
// 0x10fce21c b.eq #0x10fce328
if cpu.condition(0){{// 0x10fce328 ldur w9, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce32c mov w8, #0x7fffffff
cpu.x[8]=(((0x7fffffffu64))) as u32 as u64;
// 0x10fce330 cmp w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce334 b.ne #0x10fce35c
if cpu.condition(1){{// 0x10fce35c ldur w19, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[19]=(v0) as u32 as u64;}
// 0x10fce360 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce364 mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10fce368 mov x4, x21
cpu.x[4]=(((cpu.x[21]))) as u64;
// 0x10fce36c asr w1, w19, #0x10
cpu.x[1]=(((((cpu.x[19] as u32 as u64))) as i32).wrapping_shr(((((0x10u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10fce370 and w2, w19, #0xffff
{let a=((cpu.x[19] as u32 as u64));let b=((0xffffu64));let result=a & b;cpu.x[2]=(result) as u32 as u64;}
// 0x10fce374 bl #0x111d9240
cpu.x[30]=0x10fce378;super::chunk1::f_111d9240(cpu,rt,0x111d9240)?;
{// 0x10fce378 mov w0, w19
cpu.x[0]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10fce37c b #0x10fce33c
pc=0x10fce33c;continue;}}}{// 0x10fce338 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
{// 0x10fce33c ldp x20, x19, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fce340 ldp x22, x21, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fce344 ldp x24, x23, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fce348 ldp x26, x25, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fce34c ldp x28, x27, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fce350 ldp x29, x30, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fce354 add sp, sp, #0xb0
{let a=((cpu.sp));let b=((0xb0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fce358 ret
cpu.pc=cpu.x[30];return Ok(());}}}}{// 0x10fce220 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce224 ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce228 madd w27, w25, w8, w9
cpu.x[27]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[25] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce22c ldp w24, w19, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[24]=(v0) as u32 as u64;cpu.x[19]=(v1) as u32 as u64;}
// 0x10fce230 b #0x10fce258
{// 0x10fce258 ldur x0, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce25c bl #0x10d358a0
cpu.x[30]=0x10fce260;super::chunk0::f_10d358a0(cpu,rt,0x10d358a0)?;
{// 0x10fce260 mul w8, w0, w26
cpu.x[8]=((((cpu.x[0] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce264 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce268 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce26c mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10fce270 mov x4, x21
cpu.x[4]=(((cpu.x[21]))) as u64;
// 0x10fce274 cmp w22, w8
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce278 csel w28, w24, w19, lt
cpu.x[28]=(if cpu.condition(11){((cpu.x[24] as u32 as u64))}else{((cpu.x[19] as u32 as u64))}) as u32 as u64;
// 0x10fce27c mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce280 bl #0x111d9240
cpu.x[30]=0x10fce284;super::chunk1::f_111d9240(cpu,rt,0x111d9240)?;
{// 0x10fce284 tbz w0, #0, #0x10fce248
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce248;continue;}{// 0x10fce288 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce28c ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce290 ldur x0, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce294 madd w26, w28, w8, w9
cpu.x[26]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[28] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce298 mov w2, w27
cpu.x[2]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce29c mov w1, w26
cpu.x[1]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce2a0 bl #0x10f94f40
cpu.x[30]=0x10fce2a4;super::chunk12::f_10f94f40(cpu,rt,0x10f94f40)?;
{// 0x10fce2a4 ldur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce2a8 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce2ac b.gt #0x10fce244
if cpu.condition(12){pc=0x10fce244;continue;}{// 0x10fce2b0 ldur w3, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10fce2b4 mov w0, w26
cpu.x[0]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce2b8 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce2bc mov w2, w22
cpu.x[2]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10fce2c0 bl #0x11295120
cpu.x[30]=0x10fce2c4;super::chunk6::f_11295120(cpu,rt,0x11295120)?;
{// 0x10fce2c4 ldr w8, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce2c8 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fce2cc cbz w8, #0x10fce2fc
if (cpu.x[8] as u32 as u64) == 0 {{// 0x10fce2fc mov w8, #2
cpu.x[8]=(((0x2u64))) as u32 as u64;
// 0x10fce300 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce304 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce308 b.gt #0x10fce234
if cpu.condition(12){pc=0x10fce234;continue;}{// 0x10fce30c b.ne #0x10fce244
if cpu.condition(1){pc=0x10fce244;continue;}pc=0x10fce310;continue;}}}pc=0x10fce2d0;continue;}}}}}}}}
}
0x10fce220=>{
// 0x10fce220 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce224 ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce228 madd w27, w25, w8, w9
cpu.x[27]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[25] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce22c ldp w24, w19, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[24]=(v0) as u32 as u64;cpu.x[19]=(v1) as u32 as u64;}
// 0x10fce230 b #0x10fce258
{// 0x10fce258 ldur x0, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce25c bl #0x10d358a0
cpu.x[30]=0x10fce260;super::chunk0::f_10d358a0(cpu,rt,0x10d358a0)?;
{// 0x10fce260 mul w8, w0, w26
cpu.x[8]=((((cpu.x[0] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce264 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce268 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce26c mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10fce270 mov x4, x21
cpu.x[4]=(((cpu.x[21]))) as u64;
// 0x10fce274 cmp w22, w8
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce278 csel w28, w24, w19, lt
cpu.x[28]=(if cpu.condition(11){((cpu.x[24] as u32 as u64))}else{((cpu.x[19] as u32 as u64))}) as u32 as u64;
// 0x10fce27c mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce280 bl #0x111d9240
cpu.x[30]=0x10fce284;super::chunk1::f_111d9240(cpu,rt,0x111d9240)?;
{// 0x10fce284 tbz w0, #0, #0x10fce248
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce248;continue;}{// 0x10fce288 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce28c ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce290 ldur x0, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce294 madd w26, w28, w8, w9
cpu.x[26]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[28] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce298 mov w2, w27
cpu.x[2]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce29c mov w1, w26
cpu.x[1]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce2a0 bl #0x10f94f40
cpu.x[30]=0x10fce2a4;super::chunk12::f_10f94f40(cpu,rt,0x10f94f40)?;
{// 0x10fce2a4 ldur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce2a8 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce2ac b.gt #0x10fce244
if cpu.condition(12){pc=0x10fce244;continue;}{// 0x10fce2b0 ldur w3, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10fce2b4 mov w0, w26
cpu.x[0]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce2b8 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce2bc mov w2, w22
cpu.x[2]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10fce2c0 bl #0x11295120
cpu.x[30]=0x10fce2c4;super::chunk6::f_11295120(cpu,rt,0x11295120)?;
{// 0x10fce2c4 ldr w8, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce2c8 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fce2cc cbz w8, #0x10fce2fc
if (cpu.x[8] as u32 as u64) == 0 {{// 0x10fce2fc mov w8, #2
cpu.x[8]=(((0x2u64))) as u32 as u64;
// 0x10fce300 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce304 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce308 b.gt #0x10fce234
if cpu.condition(12){pc=0x10fce234;continue;}{// 0x10fce30c b.ne #0x10fce244
if cpu.condition(1){pc=0x10fce244;continue;}{// 0x10fce310 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce314 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce318 b.ge #0x10fce244
if cpu.condition(10){pc=0x10fce244;continue;}{// 0x10fce31c orr w8, w25, w28, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[28] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce320 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce324 b #0x10fce240
pc=0x10fce240;continue;}}}}}{// 0x10fce2d0 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce2d4 mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce2d8 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce2dc bl #0x111d9674
cpu.x[30]=0x10fce2e0;super::chunk8::f_111d9674(cpu,rt,0x111d9674)?;
// 0x10fce2e0 tst w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.logical_flags(result,32);}
// 0x10fce2e4 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce2e8 cinc w8, w8, eq
cpu.x[8]=(if cpu.condition(0){(((cpu.x[8] as u32 as u64))).wrapping_add(1)}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10fce2ec ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce2f0 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce2f4 b.gt #0x10fce234
if cpu.condition(12){pc=0x10fce234;continue;}{// 0x10fce2f8 b #0x10fce30c
{// 0x10fce30c b.ne #0x10fce244
if cpu.condition(1){pc=0x10fce244;continue;}{// 0x10fce310 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce314 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce318 b.ge #0x10fce244
if cpu.condition(10){pc=0x10fce244;continue;}{// 0x10fce31c orr w8, w25, w28, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[28] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce320 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce324 b #0x10fce240
pc=0x10fce240;continue;}}}}}}}}}}}}
}
0x10fce234=>{
// 0x10fce234 orr w9, w25, w28, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[28] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[9]=(result) as u32 as u64;}
// 0x10fce238 str w8, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce23c stur w9, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
{// 0x10fce240 stur w26, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=cpu.x[26] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
{// 0x10fce244 mov w26, #0xfa
cpu.x[26]=(((0xfau64))) as u32 as u64;
{// 0x10fce248 sub w19, w19, #1
{let a=((cpu.x[19] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[19]=(result) as u32 as u64;}
// 0x10fce24c add w24, w24, #1
{let a=((cpu.x[24] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[24]=(result) as u32 as u64;}
// 0x10fce250 cmp w20, w19
{let a=((cpu.x[20] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce254 b.eq #0x10fce210
if cpu.condition(0){pc=0x10fce210;continue;}{// 0x10fce258 ldur x0, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce25c bl #0x10d358a0
cpu.x[30]=0x10fce260;super::chunk0::f_10d358a0(cpu,rt,0x10d358a0)?;
{// 0x10fce260 mul w8, w0, w26
cpu.x[8]=((((cpu.x[0] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce264 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce268 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce26c mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10fce270 mov x4, x21
cpu.x[4]=(((cpu.x[21]))) as u64;
// 0x10fce274 cmp w22, w8
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce278 csel w28, w24, w19, lt
cpu.x[28]=(if cpu.condition(11){((cpu.x[24] as u32 as u64))}else{((cpu.x[19] as u32 as u64))}) as u32 as u64;
// 0x10fce27c mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce280 bl #0x111d9240
cpu.x[30]=0x10fce284;super::chunk1::f_111d9240(cpu,rt,0x111d9240)?;
{// 0x10fce284 tbz w0, #0, #0x10fce248
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce248;continue;}{// 0x10fce288 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce28c ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce290 ldur x0, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce294 madd w26, w28, w8, w9
cpu.x[26]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[28] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce298 mov w2, w27
cpu.x[2]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce29c mov w1, w26
cpu.x[1]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce2a0 bl #0x10f94f40
cpu.x[30]=0x10fce2a4;super::chunk12::f_10f94f40(cpu,rt,0x10f94f40)?;
{// 0x10fce2a4 ldur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce2a8 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce2ac b.gt #0x10fce244
if cpu.condition(12){pc=0x10fce244;continue;}{// 0x10fce2b0 ldur w3, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10fce2b4 mov w0, w26
cpu.x[0]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce2b8 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce2bc mov w2, w22
cpu.x[2]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10fce2c0 bl #0x11295120
cpu.x[30]=0x10fce2c4;super::chunk6::f_11295120(cpu,rt,0x11295120)?;
{// 0x10fce2c4 ldr w8, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce2c8 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fce2cc cbz w8, #0x10fce2fc
if (cpu.x[8] as u32 as u64) == 0 {{// 0x10fce2fc mov w8, #2
cpu.x[8]=(((0x2u64))) as u32 as u64;
// 0x10fce300 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce304 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce308 b.gt #0x10fce234
if cpu.condition(12){pc=0x10fce234;continue;}{// 0x10fce30c b.ne #0x10fce244
if cpu.condition(1){pc=0x10fce244;continue;}{// 0x10fce310 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce314 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce318 b.ge #0x10fce244
if cpu.condition(10){pc=0x10fce244;continue;}{// 0x10fce31c orr w8, w25, w28, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[28] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce320 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce324 b #0x10fce240
pc=0x10fce240;continue;}}}}}{// 0x10fce2d0 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce2d4 mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce2d8 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce2dc bl #0x111d9674
cpu.x[30]=0x10fce2e0;super::chunk8::f_111d9674(cpu,rt,0x111d9674)?;
// 0x10fce2e0 tst w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.logical_flags(result,32);}
// 0x10fce2e4 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce2e8 cinc w8, w8, eq
cpu.x[8]=(if cpu.condition(0){(((cpu.x[8] as u32 as u64))).wrapping_add(1)}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10fce2ec ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce2f0 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce2f4 b.gt #0x10fce234
if cpu.condition(12){pc=0x10fce234;continue;}{// 0x10fce2f8 b #0x10fce30c
{// 0x10fce30c b.ne #0x10fce244
if cpu.condition(1){pc=0x10fce244;continue;}{// 0x10fce310 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce314 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce318 b.ge #0x10fce244
if cpu.condition(10){pc=0x10fce244;continue;}pc=0x10fce31c;continue;}}}}}}}}}}}}}}
}
0x10fce240=>{
// 0x10fce240 stur w26, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=cpu.x[26] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
{// 0x10fce244 mov w26, #0xfa
cpu.x[26]=(((0xfau64))) as u32 as u64;
{// 0x10fce248 sub w19, w19, #1
{let a=((cpu.x[19] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[19]=(result) as u32 as u64;}
// 0x10fce24c add w24, w24, #1
{let a=((cpu.x[24] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[24]=(result) as u32 as u64;}
// 0x10fce250 cmp w20, w19
{let a=((cpu.x[20] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce254 b.eq #0x10fce210
if cpu.condition(0){pc=0x10fce210;continue;}{// 0x10fce258 ldur x0, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce25c bl #0x10d358a0
cpu.x[30]=0x10fce260;super::chunk0::f_10d358a0(cpu,rt,0x10d358a0)?;
{// 0x10fce260 mul w8, w0, w26
cpu.x[8]=((((cpu.x[0] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce264 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce268 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce26c mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10fce270 mov x4, x21
cpu.x[4]=(((cpu.x[21]))) as u64;
// 0x10fce274 cmp w22, w8
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce278 csel w28, w24, w19, lt
cpu.x[28]=(if cpu.condition(11){((cpu.x[24] as u32 as u64))}else{((cpu.x[19] as u32 as u64))}) as u32 as u64;
// 0x10fce27c mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce280 bl #0x111d9240
cpu.x[30]=0x10fce284;super::chunk1::f_111d9240(cpu,rt,0x111d9240)?;
{// 0x10fce284 tbz w0, #0, #0x10fce248
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce248;continue;}{// 0x10fce288 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce28c ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce290 ldur x0, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce294 madd w26, w28, w8, w9
cpu.x[26]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[28] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce298 mov w2, w27
cpu.x[2]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce29c mov w1, w26
cpu.x[1]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce2a0 bl #0x10f94f40
cpu.x[30]=0x10fce2a4;super::chunk12::f_10f94f40(cpu,rt,0x10f94f40)?;
{// 0x10fce2a4 ldur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce2a8 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce2ac b.gt #0x10fce244
if cpu.condition(12){pc=0x10fce244;continue;}{// 0x10fce2b0 ldur w3, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10fce2b4 mov w0, w26
cpu.x[0]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce2b8 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce2bc mov w2, w22
cpu.x[2]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10fce2c0 bl #0x11295120
cpu.x[30]=0x10fce2c4;super::chunk6::f_11295120(cpu,rt,0x11295120)?;
{// 0x10fce2c4 ldr w8, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce2c8 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fce2cc cbz w8, #0x10fce2fc
if (cpu.x[8] as u32 as u64) == 0 {{// 0x10fce2fc mov w8, #2
cpu.x[8]=(((0x2u64))) as u32 as u64;
// 0x10fce300 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce304 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce308 b.gt #0x10fce234
if cpu.condition(12){pc=0x10fce234;continue;}{// 0x10fce30c b.ne #0x10fce244
if cpu.condition(1){pc=0x10fce244;continue;}{// 0x10fce310 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce314 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce318 b.ge #0x10fce244
if cpu.condition(10){pc=0x10fce244;continue;}{// 0x10fce31c orr w8, w25, w28, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[28] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce320 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce324 b #0x10fce240
pc=0x10fce240;continue;}}}}}{// 0x10fce2d0 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce2d4 mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce2d8 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce2dc bl #0x111d9674
cpu.x[30]=0x10fce2e0;super::chunk8::f_111d9674(cpu,rt,0x111d9674)?;
// 0x10fce2e0 tst w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.logical_flags(result,32);}
// 0x10fce2e4 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce2e8 cinc w8, w8, eq
cpu.x[8]=(if cpu.condition(0){(((cpu.x[8] as u32 as u64))).wrapping_add(1)}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10fce2ec ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce2f0 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce2f4 b.gt #0x10fce234
if cpu.condition(12){pc=0x10fce234;continue;}{// 0x10fce2f8 b #0x10fce30c
{// 0x10fce30c b.ne #0x10fce244
if cpu.condition(1){pc=0x10fce244;continue;}{// 0x10fce310 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce314 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce318 b.ge #0x10fce244
if cpu.condition(10){pc=0x10fce244;continue;}{// 0x10fce31c orr w8, w25, w28, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[28] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce320 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce324 b #0x10fce240
pc=0x10fce240;continue;}}}}}}}}}}}}}}
}
0x10fce244=>{
// 0x10fce244 mov w26, #0xfa
cpu.x[26]=(((0xfau64))) as u32 as u64;
{// 0x10fce248 sub w19, w19, #1
{let a=((cpu.x[19] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[19]=(result) as u32 as u64;}
// 0x10fce24c add w24, w24, #1
{let a=((cpu.x[24] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[24]=(result) as u32 as u64;}
// 0x10fce250 cmp w20, w19
{let a=((cpu.x[20] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce254 b.eq #0x10fce210
if cpu.condition(0){pc=0x10fce210;continue;}{// 0x10fce258 ldur x0, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce25c bl #0x10d358a0
cpu.x[30]=0x10fce260;super::chunk0::f_10d358a0(cpu,rt,0x10d358a0)?;
{// 0x10fce260 mul w8, w0, w26
cpu.x[8]=((((cpu.x[0] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce264 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce268 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce26c mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10fce270 mov x4, x21
cpu.x[4]=(((cpu.x[21]))) as u64;
// 0x10fce274 cmp w22, w8
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce278 csel w28, w24, w19, lt
cpu.x[28]=(if cpu.condition(11){((cpu.x[24] as u32 as u64))}else{((cpu.x[19] as u32 as u64))}) as u32 as u64;
// 0x10fce27c mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce280 bl #0x111d9240
cpu.x[30]=0x10fce284;super::chunk1::f_111d9240(cpu,rt,0x111d9240)?;
{// 0x10fce284 tbz w0, #0, #0x10fce248
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce248;continue;}{// 0x10fce288 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce28c ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce290 ldur x0, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce294 madd w26, w28, w8, w9
cpu.x[26]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[28] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce298 mov w2, w27
cpu.x[2]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce29c mov w1, w26
cpu.x[1]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce2a0 bl #0x10f94f40
cpu.x[30]=0x10fce2a4;super::chunk12::f_10f94f40(cpu,rt,0x10f94f40)?;
{// 0x10fce2a4 ldur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce2a8 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce2ac b.gt #0x10fce244
if cpu.condition(12){pc=0x10fce244;continue;}{// 0x10fce2b0 ldur w3, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10fce2b4 mov w0, w26
cpu.x[0]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce2b8 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce2bc mov w2, w22
cpu.x[2]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10fce2c0 bl #0x11295120
cpu.x[30]=0x10fce2c4;super::chunk6::f_11295120(cpu,rt,0x11295120)?;
{// 0x10fce2c4 ldr w8, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce2c8 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fce2cc cbz w8, #0x10fce2fc
if (cpu.x[8] as u32 as u64) == 0 {{// 0x10fce2fc mov w8, #2
cpu.x[8]=(((0x2u64))) as u32 as u64;
// 0x10fce300 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce304 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce308 b.gt #0x10fce234
if cpu.condition(12){pc=0x10fce234;continue;}{// 0x10fce30c b.ne #0x10fce244
if cpu.condition(1){pc=0x10fce244;continue;}{// 0x10fce310 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce314 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce318 b.ge #0x10fce244
if cpu.condition(10){pc=0x10fce244;continue;}{// 0x10fce31c orr w8, w25, w28, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[28] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce320 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce324 b #0x10fce240
pc=0x10fce240;continue;}}}}}{// 0x10fce2d0 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce2d4 mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce2d8 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce2dc bl #0x111d9674
cpu.x[30]=0x10fce2e0;super::chunk8::f_111d9674(cpu,rt,0x111d9674)?;
// 0x10fce2e0 tst w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.logical_flags(result,32);}
// 0x10fce2e4 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce2e8 cinc w8, w8, eq
cpu.x[8]=(if cpu.condition(0){(((cpu.x[8] as u32 as u64))).wrapping_add(1)}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10fce2ec ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce2f0 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce2f4 b.gt #0x10fce234
if cpu.condition(12){pc=0x10fce234;continue;}{// 0x10fce2f8 b #0x10fce30c
{// 0x10fce30c b.ne #0x10fce244
if cpu.condition(1){pc=0x10fce244;continue;}{// 0x10fce310 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce314 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce318 b.ge #0x10fce244
if cpu.condition(10){pc=0x10fce244;continue;}{// 0x10fce31c orr w8, w25, w28, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[28] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce320 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce324 b #0x10fce240
pc=0x10fce240;continue;}}}}}}}}}}}}}
}
0x10fce248=>{
// 0x10fce248 sub w19, w19, #1
{let a=((cpu.x[19] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[19]=(result) as u32 as u64;}
// 0x10fce24c add w24, w24, #1
{let a=((cpu.x[24] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[24]=(result) as u32 as u64;}
// 0x10fce250 cmp w20, w19
{let a=((cpu.x[20] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce254 b.eq #0x10fce210
if cpu.condition(0){pc=0x10fce210;continue;}{// 0x10fce258 ldur x0, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce25c bl #0x10d358a0
cpu.x[30]=0x10fce260;super::chunk0::f_10d358a0(cpu,rt,0x10d358a0)?;
{// 0x10fce260 mul w8, w0, w26
cpu.x[8]=((((cpu.x[0] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce264 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce268 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce26c mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10fce270 mov x4, x21
cpu.x[4]=(((cpu.x[21]))) as u64;
// 0x10fce274 cmp w22, w8
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce278 csel w28, w24, w19, lt
cpu.x[28]=(if cpu.condition(11){((cpu.x[24] as u32 as u64))}else{((cpu.x[19] as u32 as u64))}) as u32 as u64;
// 0x10fce27c mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce280 bl #0x111d9240
cpu.x[30]=0x10fce284;super::chunk1::f_111d9240(cpu,rt,0x111d9240)?;
{// 0x10fce284 tbz w0, #0, #0x10fce248
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce248;continue;}{// 0x10fce288 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce28c ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce290 ldur x0, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce294 madd w26, w28, w8, w9
cpu.x[26]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[28] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce298 mov w2, w27
cpu.x[2]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce29c mov w1, w26
cpu.x[1]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce2a0 bl #0x10f94f40
cpu.x[30]=0x10fce2a4;super::chunk12::f_10f94f40(cpu,rt,0x10f94f40)?;
{// 0x10fce2a4 ldur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce2a8 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce2ac b.gt #0x10fce244
if cpu.condition(12){pc=0x10fce244;continue;}{// 0x10fce2b0 ldur w3, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10fce2b4 mov w0, w26
cpu.x[0]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce2b8 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce2bc mov w2, w22
cpu.x[2]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10fce2c0 bl #0x11295120
cpu.x[30]=0x10fce2c4;super::chunk6::f_11295120(cpu,rt,0x11295120)?;
{// 0x10fce2c4 ldr w8, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce2c8 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fce2cc cbz w8, #0x10fce2fc
if (cpu.x[8] as u32 as u64) == 0 {{// 0x10fce2fc mov w8, #2
cpu.x[8]=(((0x2u64))) as u32 as u64;
// 0x10fce300 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce304 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce308 b.gt #0x10fce234
if cpu.condition(12){pc=0x10fce234;continue;}{// 0x10fce30c b.ne #0x10fce244
if cpu.condition(1){pc=0x10fce244;continue;}{// 0x10fce310 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce314 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce318 b.ge #0x10fce244
if cpu.condition(10){pc=0x10fce244;continue;}{// 0x10fce31c orr w8, w25, w28, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[28] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce320 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce324 b #0x10fce240
pc=0x10fce240;continue;}}}}}{// 0x10fce2d0 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce2d4 mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce2d8 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce2dc bl #0x111d9674
cpu.x[30]=0x10fce2e0;super::chunk8::f_111d9674(cpu,rt,0x111d9674)?;
// 0x10fce2e0 tst w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.logical_flags(result,32);}
// 0x10fce2e4 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce2e8 cinc w8, w8, eq
cpu.x[8]=(if cpu.condition(0){(((cpu.x[8] as u32 as u64))).wrapping_add(1)}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10fce2ec ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce2f0 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce2f4 b.gt #0x10fce234
if cpu.condition(12){pc=0x10fce234;continue;}{// 0x10fce2f8 b #0x10fce30c
{// 0x10fce30c b.ne #0x10fce244
if cpu.condition(1){pc=0x10fce244;continue;}{// 0x10fce310 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce314 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce318 b.ge #0x10fce244
if cpu.condition(10){pc=0x10fce244;continue;}{// 0x10fce31c orr w8, w25, w28, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[28] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce320 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce324 b #0x10fce240
pc=0x10fce240;continue;}}}}}}}}}}}}
}
0x10fce258=>{
// 0x10fce258 ldur x0, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce25c bl #0x10d358a0
cpu.x[30]=0x10fce260;super::chunk0::f_10d358a0(cpu,rt,0x10d358a0)?;
{// 0x10fce260 mul w8, w0, w26
cpu.x[8]=((((cpu.x[0] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce264 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce268 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce26c mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10fce270 mov x4, x21
cpu.x[4]=(((cpu.x[21]))) as u64;
// 0x10fce274 cmp w22, w8
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce278 csel w28, w24, w19, lt
cpu.x[28]=(if cpu.condition(11){((cpu.x[24] as u32 as u64))}else{((cpu.x[19] as u32 as u64))}) as u32 as u64;
// 0x10fce27c mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce280 bl #0x111d9240
cpu.x[30]=0x10fce284;super::chunk1::f_111d9240(cpu,rt,0x111d9240)?;
{// 0x10fce284 tbz w0, #0, #0x10fce248
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce248;continue;}{// 0x10fce288 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce28c ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce290 ldur x0, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce294 madd w26, w28, w8, w9
cpu.x[26]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[28] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce298 mov w2, w27
cpu.x[2]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce29c mov w1, w26
cpu.x[1]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce2a0 bl #0x10f94f40
cpu.x[30]=0x10fce2a4;super::chunk12::f_10f94f40(cpu,rt,0x10f94f40)?;
{// 0x10fce2a4 ldur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce2a8 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce2ac b.gt #0x10fce244
if cpu.condition(12){pc=0x10fce244;continue;}{// 0x10fce2b0 ldur w3, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10fce2b4 mov w0, w26
cpu.x[0]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce2b8 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce2bc mov w2, w22
cpu.x[2]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10fce2c0 bl #0x11295120
cpu.x[30]=0x10fce2c4;super::chunk6::f_11295120(cpu,rt,0x11295120)?;
{// 0x10fce2c4 ldr w8, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce2c8 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fce2cc cbz w8, #0x10fce2fc
if (cpu.x[8] as u32 as u64) == 0 {{// 0x10fce2fc mov w8, #2
cpu.x[8]=(((0x2u64))) as u32 as u64;
// 0x10fce300 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce304 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce308 b.gt #0x10fce234
if cpu.condition(12){pc=0x10fce234;continue;}{// 0x10fce30c b.ne #0x10fce244
if cpu.condition(1){pc=0x10fce244;continue;}{// 0x10fce310 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce314 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce318 b.ge #0x10fce244
if cpu.condition(10){pc=0x10fce244;continue;}{// 0x10fce31c orr w8, w25, w28, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[28] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce320 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce324 b #0x10fce240
pc=0x10fce240;continue;}}}}}{// 0x10fce2d0 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce2d4 mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce2d8 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce2dc bl #0x111d9674
cpu.x[30]=0x10fce2e0;super::chunk8::f_111d9674(cpu,rt,0x111d9674)?;
// 0x10fce2e0 tst w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.logical_flags(result,32);}
// 0x10fce2e4 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce2e8 cinc w8, w8, eq
cpu.x[8]=(if cpu.condition(0){(((cpu.x[8] as u32 as u64))).wrapping_add(1)}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10fce2ec ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce2f0 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce2f4 b.gt #0x10fce234
if cpu.condition(12){pc=0x10fce234;continue;}{// 0x10fce2f8 b #0x10fce30c
{// 0x10fce30c b.ne #0x10fce244
if cpu.condition(1){pc=0x10fce244;continue;}{// 0x10fce310 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce314 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce318 b.ge #0x10fce244
if cpu.condition(10){pc=0x10fce244;continue;}{// 0x10fce31c orr w8, w25, w28, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[28] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce320 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce324 b #0x10fce240
pc=0x10fce240;continue;}}}}}}}}}}}
}
0x10fce260=>{
// 0x10fce260 mul w8, w0, w26
cpu.x[8]=((((cpu.x[0] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fce264 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce268 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce26c mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10fce270 mov x4, x21
cpu.x[4]=(((cpu.x[21]))) as u64;
// 0x10fce274 cmp w22, w8
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce278 csel w28, w24, w19, lt
cpu.x[28]=(if cpu.condition(11){((cpu.x[24] as u32 as u64))}else{((cpu.x[19] as u32 as u64))}) as u32 as u64;
// 0x10fce27c mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce280 bl #0x111d9240
cpu.x[30]=0x10fce284;super::chunk1::f_111d9240(cpu,rt,0x111d9240)?;
{// 0x10fce284 tbz w0, #0, #0x10fce248
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce248;continue;}{// 0x10fce288 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce28c ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce290 ldur x0, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce294 madd w26, w28, w8, w9
cpu.x[26]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[28] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce298 mov w2, w27
cpu.x[2]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce29c mov w1, w26
cpu.x[1]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce2a0 bl #0x10f94f40
cpu.x[30]=0x10fce2a4;super::chunk12::f_10f94f40(cpu,rt,0x10f94f40)?;
{// 0x10fce2a4 ldur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce2a8 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce2ac b.gt #0x10fce244
if cpu.condition(12){pc=0x10fce244;continue;}{// 0x10fce2b0 ldur w3, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10fce2b4 mov w0, w26
cpu.x[0]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce2b8 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce2bc mov w2, w22
cpu.x[2]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10fce2c0 bl #0x11295120
cpu.x[30]=0x10fce2c4;super::chunk6::f_11295120(cpu,rt,0x11295120)?;
{// 0x10fce2c4 ldr w8, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce2c8 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fce2cc cbz w8, #0x10fce2fc
if (cpu.x[8] as u32 as u64) == 0 {{// 0x10fce2fc mov w8, #2
cpu.x[8]=(((0x2u64))) as u32 as u64;
// 0x10fce300 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce304 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce308 b.gt #0x10fce234
if cpu.condition(12){pc=0x10fce234;continue;}{// 0x10fce30c b.ne #0x10fce244
if cpu.condition(1){pc=0x10fce244;continue;}{// 0x10fce310 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce314 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce318 b.ge #0x10fce244
if cpu.condition(10){pc=0x10fce244;continue;}{// 0x10fce31c orr w8, w25, w28, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[28] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce320 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce324 b #0x10fce240
pc=0x10fce240;continue;}}}}}{// 0x10fce2d0 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce2d4 mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce2d8 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce2dc bl #0x111d9674
cpu.x[30]=0x10fce2e0;super::chunk8::f_111d9674(cpu,rt,0x111d9674)?;
// 0x10fce2e0 tst w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.logical_flags(result,32);}
// 0x10fce2e4 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce2e8 cinc w8, w8, eq
cpu.x[8]=(if cpu.condition(0){(((cpu.x[8] as u32 as u64))).wrapping_add(1)}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10fce2ec ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce2f0 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce2f4 b.gt #0x10fce234
if cpu.condition(12){pc=0x10fce234;continue;}{// 0x10fce2f8 b #0x10fce30c
{// 0x10fce30c b.ne #0x10fce244
if cpu.condition(1){pc=0x10fce244;continue;}{// 0x10fce310 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce314 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce318 b.ge #0x10fce244
if cpu.condition(10){pc=0x10fce244;continue;}{// 0x10fce31c orr w8, w25, w28, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[28] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce320 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce324 b #0x10fce240
pc=0x10fce240;continue;}}}}}}}}}}
}
0x10fce284=>{
// 0x10fce284 tbz w0, #0, #0x10fce248
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce248;continue;}{// 0x10fce288 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce28c ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce290 ldur x0, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce294 madd w26, w28, w8, w9
cpu.x[26]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[28] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce298 mov w2, w27
cpu.x[2]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce29c mov w1, w26
cpu.x[1]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce2a0 bl #0x10f94f40
cpu.x[30]=0x10fce2a4;super::chunk12::f_10f94f40(cpu,rt,0x10f94f40)?;
{// 0x10fce2a4 ldur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce2a8 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce2ac b.gt #0x10fce244
if cpu.condition(12){pc=0x10fce244;continue;}{// 0x10fce2b0 ldur w3, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10fce2b4 mov w0, w26
cpu.x[0]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce2b8 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce2bc mov w2, w22
cpu.x[2]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10fce2c0 bl #0x11295120
cpu.x[30]=0x10fce2c4;super::chunk6::f_11295120(cpu,rt,0x11295120)?;
{// 0x10fce2c4 ldr w8, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce2c8 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fce2cc cbz w8, #0x10fce2fc
if (cpu.x[8] as u32 as u64) == 0 {{// 0x10fce2fc mov w8, #2
cpu.x[8]=(((0x2u64))) as u32 as u64;
// 0x10fce300 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce304 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce308 b.gt #0x10fce234
if cpu.condition(12){pc=0x10fce234;continue;}{// 0x10fce30c b.ne #0x10fce244
if cpu.condition(1){pc=0x10fce244;continue;}{// 0x10fce310 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce314 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce318 b.ge #0x10fce244
if cpu.condition(10){pc=0x10fce244;continue;}{// 0x10fce31c orr w8, w25, w28, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[28] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce320 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce324 b #0x10fce240
pc=0x10fce240;continue;}}}}}{// 0x10fce2d0 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce2d4 mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce2d8 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce2dc bl #0x111d9674
cpu.x[30]=0x10fce2e0;super::chunk8::f_111d9674(cpu,rt,0x111d9674)?;
// 0x10fce2e0 tst w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.logical_flags(result,32);}
// 0x10fce2e4 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce2e8 cinc w8, w8, eq
cpu.x[8]=(if cpu.condition(0){(((cpu.x[8] as u32 as u64))).wrapping_add(1)}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10fce2ec ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce2f0 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce2f4 b.gt #0x10fce234
if cpu.condition(12){pc=0x10fce234;continue;}{// 0x10fce2f8 b #0x10fce30c
{// 0x10fce30c b.ne #0x10fce244
if cpu.condition(1){pc=0x10fce244;continue;}{// 0x10fce310 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce314 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce318 b.ge #0x10fce244
if cpu.condition(10){pc=0x10fce244;continue;}{// 0x10fce31c orr w8, w25, w28, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[28] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce320 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce324 b #0x10fce240
pc=0x10fce240;continue;}}}}}}}}}
}
0x10fce288=>{
// 0x10fce288 mov w8, #0x1f4
cpu.x[8]=(((0x1f4u64))) as u32 as u64;
// 0x10fce28c ldur w9, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce290 ldur x0, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fce294 madd w26, w28, w8, w9
cpu.x[26]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[28] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fce298 mov w2, w27
cpu.x[2]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce29c mov w1, w26
cpu.x[1]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce2a0 bl #0x10f94f40
cpu.x[30]=0x10fce2a4;super::chunk12::f_10f94f40(cpu,rt,0x10f94f40)?;
{// 0x10fce2a4 ldur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce2a8 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce2ac b.gt #0x10fce244
if cpu.condition(12){pc=0x10fce244;continue;}{// 0x10fce2b0 ldur w3, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10fce2b4 mov w0, w26
cpu.x[0]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce2b8 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce2bc mov w2, w22
cpu.x[2]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10fce2c0 bl #0x11295120
cpu.x[30]=0x10fce2c4;super::chunk6::f_11295120(cpu,rt,0x11295120)?;
{// 0x10fce2c4 ldr w8, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce2c8 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fce2cc cbz w8, #0x10fce2fc
if (cpu.x[8] as u32 as u64) == 0 {{// 0x10fce2fc mov w8, #2
cpu.x[8]=(((0x2u64))) as u32 as u64;
// 0x10fce300 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce304 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce308 b.gt #0x10fce234
if cpu.condition(12){pc=0x10fce234;continue;}{// 0x10fce30c b.ne #0x10fce244
if cpu.condition(1){pc=0x10fce244;continue;}{// 0x10fce310 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce314 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce318 b.ge #0x10fce244
if cpu.condition(10){pc=0x10fce244;continue;}{// 0x10fce31c orr w8, w25, w28, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[28] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce320 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce324 b #0x10fce240
pc=0x10fce240;continue;}}}}}{// 0x10fce2d0 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce2d4 mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce2d8 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce2dc bl #0x111d9674
cpu.x[30]=0x10fce2e0;super::chunk8::f_111d9674(cpu,rt,0x111d9674)?;
// 0x10fce2e0 tst w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.logical_flags(result,32);}
// 0x10fce2e4 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce2e8 cinc w8, w8, eq
cpu.x[8]=(if cpu.condition(0){(((cpu.x[8] as u32 as u64))).wrapping_add(1)}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10fce2ec ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce2f0 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce2f4 b.gt #0x10fce234
if cpu.condition(12){pc=0x10fce234;continue;}{// 0x10fce2f8 b #0x10fce30c
{// 0x10fce30c b.ne #0x10fce244
if cpu.condition(1){pc=0x10fce244;continue;}{// 0x10fce310 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce314 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce318 b.ge #0x10fce244
if cpu.condition(10){pc=0x10fce244;continue;}{// 0x10fce31c orr w8, w25, w28, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[28] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce320 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce324 b #0x10fce240
pc=0x10fce240;continue;}}}}}}}}
}
0x10fce2a4=>{
// 0x10fce2a4 ldur w8, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce2a8 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce2ac b.gt #0x10fce244
if cpu.condition(12){pc=0x10fce244;continue;}{// 0x10fce2b0 ldur w3, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10fce2b4 mov w0, w26
cpu.x[0]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce2b8 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce2bc mov w2, w22
cpu.x[2]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10fce2c0 bl #0x11295120
cpu.x[30]=0x10fce2c4;super::chunk6::f_11295120(cpu,rt,0x11295120)?;
{// 0x10fce2c4 ldr w8, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce2c8 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fce2cc cbz w8, #0x10fce2fc
if (cpu.x[8] as u32 as u64) == 0 {{// 0x10fce2fc mov w8, #2
cpu.x[8]=(((0x2u64))) as u32 as u64;
// 0x10fce300 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce304 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce308 b.gt #0x10fce234
if cpu.condition(12){pc=0x10fce234;continue;}{// 0x10fce30c b.ne #0x10fce244
if cpu.condition(1){pc=0x10fce244;continue;}{// 0x10fce310 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce314 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce318 b.ge #0x10fce244
if cpu.condition(10){pc=0x10fce244;continue;}{// 0x10fce31c orr w8, w25, w28, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[28] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce320 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce324 b #0x10fce240
pc=0x10fce240;continue;}}}}}{// 0x10fce2d0 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce2d4 mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce2d8 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce2dc bl #0x111d9674
cpu.x[30]=0x10fce2e0;super::chunk8::f_111d9674(cpu,rt,0x111d9674)?;
// 0x10fce2e0 tst w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.logical_flags(result,32);}
// 0x10fce2e4 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce2e8 cinc w8, w8, eq
cpu.x[8]=(if cpu.condition(0){(((cpu.x[8] as u32 as u64))).wrapping_add(1)}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10fce2ec ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce2f0 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce2f4 b.gt #0x10fce234
if cpu.condition(12){pc=0x10fce234;continue;}{// 0x10fce2f8 b #0x10fce30c
{// 0x10fce30c b.ne #0x10fce244
if cpu.condition(1){pc=0x10fce244;continue;}{// 0x10fce310 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce314 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce318 b.ge #0x10fce244
if cpu.condition(10){pc=0x10fce244;continue;}{// 0x10fce31c orr w8, w25, w28, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[28] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce320 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce324 b #0x10fce240
pc=0x10fce240;continue;}}}}}}}
}
0x10fce2b0=>{
// 0x10fce2b0 ldur w3, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10fce2b4 mov w0, w26
cpu.x[0]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fce2b8 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fce2bc mov w2, w22
cpu.x[2]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10fce2c0 bl #0x11295120
cpu.x[30]=0x10fce2c4;super::chunk6::f_11295120(cpu,rt,0x11295120)?;
{// 0x10fce2c4 ldr w8, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce2c8 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fce2cc cbz w8, #0x10fce2fc
if (cpu.x[8] as u32 as u64) == 0 {{// 0x10fce2fc mov w8, #2
cpu.x[8]=(((0x2u64))) as u32 as u64;
// 0x10fce300 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce304 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce308 b.gt #0x10fce234
if cpu.condition(12){pc=0x10fce234;continue;}{// 0x10fce30c b.ne #0x10fce244
if cpu.condition(1){pc=0x10fce244;continue;}{// 0x10fce310 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce314 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce318 b.ge #0x10fce244
if cpu.condition(10){pc=0x10fce244;continue;}{// 0x10fce31c orr w8, w25, w28, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[28] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce320 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce324 b #0x10fce240
pc=0x10fce240;continue;}}}}}{// 0x10fce2d0 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce2d4 mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce2d8 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce2dc bl #0x111d9674
cpu.x[30]=0x10fce2e0;super::chunk8::f_111d9674(cpu,rt,0x111d9674)?;
// 0x10fce2e0 tst w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.logical_flags(result,32);}
// 0x10fce2e4 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce2e8 cinc w8, w8, eq
cpu.x[8]=(if cpu.condition(0){(((cpu.x[8] as u32 as u64))).wrapping_add(1)}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10fce2ec ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce2f0 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce2f4 b.gt #0x10fce234
if cpu.condition(12){pc=0x10fce234;continue;}{// 0x10fce2f8 b #0x10fce30c
{// 0x10fce30c b.ne #0x10fce244
if cpu.condition(1){pc=0x10fce244;continue;}{// 0x10fce310 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce314 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce318 b.ge #0x10fce244
if cpu.condition(10){pc=0x10fce244;continue;}{// 0x10fce31c orr w8, w25, w28, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[28] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce320 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce324 b #0x10fce240
pc=0x10fce240;continue;}}}}}}
}
0x10fce2c4=>{
// 0x10fce2c4 ldr w8, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce2c8 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fce2cc cbz w8, #0x10fce2fc
if (cpu.x[8] as u32 as u64) == 0 {{// 0x10fce2fc mov w8, #2
cpu.x[8]=(((0x2u64))) as u32 as u64;
// 0x10fce300 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce304 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce308 b.gt #0x10fce234
if cpu.condition(12){pc=0x10fce234;continue;}{// 0x10fce30c b.ne #0x10fce244
if cpu.condition(1){pc=0x10fce244;continue;}{// 0x10fce310 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce314 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce318 b.ge #0x10fce244
if cpu.condition(10){pc=0x10fce244;continue;}{// 0x10fce31c orr w8, w25, w28, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[28] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce320 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce324 b #0x10fce240
pc=0x10fce240;continue;}}}}}{// 0x10fce2d0 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce2d4 mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce2d8 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce2dc bl #0x111d9674
cpu.x[30]=0x10fce2e0;super::chunk8::f_111d9674(cpu,rt,0x111d9674)?;
// 0x10fce2e0 tst w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.logical_flags(result,32);}
// 0x10fce2e4 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce2e8 cinc w8, w8, eq
cpu.x[8]=(if cpu.condition(0){(((cpu.x[8] as u32 as u64))).wrapping_add(1)}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10fce2ec ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce2f0 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce2f4 b.gt #0x10fce234
if cpu.condition(12){pc=0x10fce234;continue;}{// 0x10fce2f8 b #0x10fce30c
{// 0x10fce30c b.ne #0x10fce244
if cpu.condition(1){pc=0x10fce244;continue;}{// 0x10fce310 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce314 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce318 b.ge #0x10fce244
if cpu.condition(10){pc=0x10fce244;continue;}{// 0x10fce31c orr w8, w25, w28, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[28] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce320 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce324 b #0x10fce240
pc=0x10fce240;continue;}}}}}
}
0x10fce2d0=>{
// 0x10fce2d0 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce2d4 mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fce2d8 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fce2dc bl #0x111d9674
cpu.x[30]=0x10fce2e0;super::chunk8::f_111d9674(cpu,rt,0x111d9674)?;
// 0x10fce2e0 tst w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.logical_flags(result,32);}
// 0x10fce2e4 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fce2e8 cinc w8, w8, eq
cpu.x[8]=(if cpu.condition(0){(((cpu.x[8] as u32 as u64))).wrapping_add(1)}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10fce2ec ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce2f0 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce2f4 b.gt #0x10fce234
if cpu.condition(12){pc=0x10fce234;continue;}{// 0x10fce2f8 b #0x10fce30c
{// 0x10fce30c b.ne #0x10fce244
if cpu.condition(1){pc=0x10fce244;continue;}{// 0x10fce310 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce314 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce318 b.ge #0x10fce244
if cpu.condition(10){pc=0x10fce244;continue;}{// 0x10fce31c orr w8, w25, w28, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[28] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce320 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce324 b #0x10fce240
pc=0x10fce240;continue;}}}}
}
0x10fce2f8=>{
// 0x10fce2f8 b #0x10fce30c
{// 0x10fce30c b.ne #0x10fce244
if cpu.condition(1){pc=0x10fce244;continue;}{// 0x10fce310 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce314 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce318 b.ge #0x10fce244
if cpu.condition(10){pc=0x10fce244;continue;}{// 0x10fce31c orr w8, w25, w28, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[28] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce320 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce324 b #0x10fce240
pc=0x10fce240;continue;}}}
}
0x10fce2fc=>{
// 0x10fce2fc mov w8, #2
cpu.x[8]=(((0x2u64))) as u32 as u64;
// 0x10fce300 ldr w9, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce304 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce308 b.gt #0x10fce234
if cpu.condition(12){pc=0x10fce234;continue;}{// 0x10fce30c b.ne #0x10fce244
if cpu.condition(1){pc=0x10fce244;continue;}{// 0x10fce310 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce314 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce318 b.ge #0x10fce244
if cpu.condition(10){pc=0x10fce244;continue;}{// 0x10fce31c orr w8, w25, w28, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[28] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce320 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce324 b #0x10fce240
pc=0x10fce240;continue;}}}
}
0x10fce30c=>{
// 0x10fce30c b.ne #0x10fce244
if cpu.condition(1){pc=0x10fce244;continue;}{// 0x10fce310 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce314 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce318 b.ge #0x10fce244
if cpu.condition(10){pc=0x10fce244;continue;}{// 0x10fce31c orr w8, w25, w28, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[28] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce320 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce324 b #0x10fce240
pc=0x10fce240;continue;}}
}
0x10fce310=>{
// 0x10fce310 ldur w8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fce314 cmp w26, w8
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce318 b.ge #0x10fce244
if cpu.condition(10){pc=0x10fce244;continue;}{// 0x10fce31c orr w8, w25, w28, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[28] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce320 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce324 b #0x10fce240
pc=0x10fce240;continue;}
}
0x10fce31c=>{
// 0x10fce31c orr w8, w25, w28, lsl #16
{let a=((cpu.x[25] as u32 as u64));let b=(((cpu.x[28] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fce320 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce324 b #0x10fce240
pc=0x10fce240;continue;
}
0x10fce328=>{
// 0x10fce328 ldur w9, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fce32c mov w8, #0x7fffffff
cpu.x[8]=(((0x7fffffffu64))) as u32 as u64;
// 0x10fce330 cmp w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fce334 b.ne #0x10fce35c
if cpu.condition(1){{// 0x10fce35c ldur w19, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[19]=(v0) as u32 as u64;}
// 0x10fce360 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce364 mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10fce368 mov x4, x21
cpu.x[4]=(((cpu.x[21]))) as u64;
// 0x10fce36c asr w1, w19, #0x10
cpu.x[1]=(((((cpu.x[19] as u32 as u64))) as i32).wrapping_shr(((((0x10u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10fce370 and w2, w19, #0xffff
{let a=((cpu.x[19] as u32 as u64));let b=((0xffffu64));let result=a & b;cpu.x[2]=(result) as u32 as u64;}
// 0x10fce374 bl #0x111d9240
cpu.x[30]=0x10fce378;super::chunk1::f_111d9240(cpu,rt,0x111d9240)?;
{// 0x10fce378 mov w0, w19
cpu.x[0]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10fce37c b #0x10fce33c
pc=0x10fce33c;continue;}}}{// 0x10fce338 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
{// 0x10fce33c ldp x20, x19, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fce340 ldp x22, x21, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fce344 ldp x24, x23, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fce348 ldp x26, x25, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fce34c ldp x28, x27, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fce350 ldp x29, x30, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fce354 add sp, sp, #0xb0
{let a=((cpu.sp));let b=((0xb0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fce358 ret
cpu.pc=cpu.x[30];return Ok(());}}
}
0x10fce338=>{
// 0x10fce338 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
{// 0x10fce33c ldp x20, x19, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fce340 ldp x22, x21, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fce344 ldp x24, x23, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fce348 ldp x26, x25, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fce34c ldp x28, x27, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fce350 ldp x29, x30, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fce354 add sp, sp, #0xb0
{let a=((cpu.sp));let b=((0xb0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fce358 ret
cpu.pc=cpu.x[30];return Ok(());}
}
0x10fce33c=>{
// 0x10fce33c ldp x20, x19, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fce340 ldp x22, x21, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fce344 ldp x24, x23, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fce348 ldp x26, x25, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fce34c ldp x28, x27, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fce350 ldp x29, x30, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fce354 add sp, sp, #0xb0
{let a=((cpu.sp));let b=((0xb0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fce358 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10fce35c=>{
// 0x10fce35c ldur w19, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[19]=(v0) as u32 as u64;}
// 0x10fce360 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fce364 mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10fce368 mov x4, x21
cpu.x[4]=(((cpu.x[21]))) as u64;
// 0x10fce36c asr w1, w19, #0x10
cpu.x[1]=(((((cpu.x[19] as u32 as u64))) as i32).wrapping_shr(((((0x10u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10fce370 and w2, w19, #0xffff
{let a=((cpu.x[19] as u32 as u64));let b=((0xffffu64));let result=a & b;cpu.x[2]=(result) as u32 as u64;}
// 0x10fce374 bl #0x111d9240
cpu.x[30]=0x10fce378;super::chunk1::f_111d9240(cpu,rt,0x111d9240)?;
{// 0x10fce378 mov w0, w19
cpu.x[0]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10fce37c b #0x10fce33c
pc=0x10fce33c;continue;}
}
0x10fce378=>{
// 0x10fce378 mov w0, w19
cpu.x[0]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10fce37c b #0x10fce33c
pc=0x10fce33c;continue;
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fcdefc")),}}}
#[inline]
pub(super) unsafe fn f_10fcef48(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fcef48=>{
// 0x10fcef48 sub sp, sp, #0x90
{let a=((cpu.sp));let b=((0x90u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x10fcef4c str d8, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=(cpu.q[8] & 0xffffffffffffffffu128) as u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fcef50 stp x29, x30, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fcef54 stp x28, x27, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[28];let v1=cpu.x[27];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fcef58 stp x26, x25, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=cpu.x[26];let v1=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fcef5c stp x24, x23, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fcef60 stp x22, x21, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fcef64 stp x20, x19, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fcef68 add x29, sp, #0x30
{let a=((cpu.sp));let b=((0x30u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x10fcef6c ldr x23, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10fcef70 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10fcef74 mov w25, w5
cpu.x[25]=(((cpu.x[5] as u32 as u64))) as u32 as u64;
// 0x10fcef78 mov w27, w4
cpu.x[27]=(((cpu.x[4] as u32 as u64))) as u32 as u64;
// 0x10fcef7c mov w21, w3
cpu.x[21]=(((cpu.x[3] as u32 as u64))) as u32 as u64;
// 0x10fcef80 mov w24, w2
cpu.x[24]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x10fcef84 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fcef88 mov w19, w1
cpu.x[19]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x10fcef8c bl #0x10f7e050
cpu.x[30]=0x10fcef90;super::chunk10::f_10f7e050(cpu,rt,0x10f7e050)?;
pc=0x10fcef90;continue;
}
0x10fcef90=>{
// 0x10fcef90 str x0, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fcef94 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fcef98 bl #0x10f93d2c
cpu.x[30]=0x10fcef9c;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
pc=0x10fcef9c;continue;
}
0x10fcef9c=>{
// 0x10fcef9c stur x0, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fcefa0 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fcefa4 bl #0x10f942c0
cpu.x[30]=0x10fcefa8;super::chunk2::f_10f942c0(cpu,rt,0x10f942c0)?;
pc=0x10fcefa8;continue;
}
0x10fcefa8=>{
// 0x10fcefa8 mov w1, w19
cpu.x[1]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10fcefac mov w2, w24
cpu.x[2]=(((cpu.x[24] as u32 as u64))) as u32 as u64;
// 0x10fcefb0 bl #0x11294ed8
cpu.x[30]=0x10fcefb4;super::chunk15::f_11294ed8(cpu,rt,0x11294ed8)?;
pc=0x10fcefb4;continue;
}
0x10fcefb4=>{
// 0x10fcefb4 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcefb8 mov w8, #0xfa
cpu.x[8]=(((0xfau64))) as u32 as u64;
// 0x10fcefbc csinc w22, w0, wzr, hi
cpu.x[22]=(if cpu.condition(8){((cpu.x[0] as u32 as u64))}else{(((0u64))).wrapping_add(1)}) as u32 as u64;
// 0x10fcefc0 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fcefc4 cmp w21, w22
{let a=((cpu.x[21] as u32 as u64));let b=((cpu.x[22] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcefc8 csel w9, w21, w22, lt
cpu.x[9]=(if cpu.condition(11){((cpu.x[21] as u32 as u64))}else{((cpu.x[22] as u32 as u64))}) as u32 as u64;
// 0x10fcefcc cmp w9, #0xfa
{let a=((cpu.x[9] as u32 as u64));let b=((0xfau64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcefd0 csel w26, w9, w8, lt
cpu.x[26]=(if cpu.condition(11){((cpu.x[9] as u32 as u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10fcefd4 bl #0x10f94298
cpu.x[30]=0x10fcefd8;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
pc=0x10fcefd8;continue;
}
0x10fcefd8=>{
// 0x10fcefd8 mov w28, w0
cpu.x[28]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fcefdc mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fcefe0 bl #0x10f942a0
cpu.x[30]=0x10fcefe4;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
pc=0x10fcefe4;continue;
}
0x10fcefe4=>{
// 0x10fcefe4 str w19, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=cpu.x[19] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fcefe8 sub w19, w19, w28
{let a=((cpu.x[19] as u32 as u64));let b=((cpu.x[28] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[19]=(result) as u32 as u64;}
// 0x10fcefec sub w21, w24, w0
{let a=((cpu.x[24] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[21]=(result) as u32 as u64;}
// 0x10fceff0 lsl w28, w19, #8
cpu.x[28]=(((((cpu.x[19] as u32 as u64))) as u32).wrapping_shl(((((0x8u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10fceff4 stur w24, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=cpu.x[24] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fceff8 orr w8, w28, w19
{let a=((cpu.x[28] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fceffc tbz w8, #0x1f, #0x10fcf010
if ((cpu.x[8] as u32 as u64) & (1u64 << 31)) == 0 {pc=0x10fcf010;continue;}pc=0x10fcf000;continue;
}
0x10fcf000=>{
// 0x10fcf000 cmp w19, #0
{let a=((cpu.x[19] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcf004 b.gt #0x10fcf040
if cpu.condition(12){pc=0x10fcf040;continue;}pc=0x10fcf008;continue;
}
0x10fcf008=>{
// 0x10fcf008 cmp w28, #1
{let a=((cpu.x[28] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcf00c b.ge #0x10fcf040
if cpu.condition(10){pc=0x10fcf040;continue;}pc=0x10fcf010;continue;
}
0x10fcf010=>{
// 0x10fcf010 lsl w24, w21, #8
cpu.x[24]=(((((cpu.x[21] as u32 as u64))) as u32).wrapping_shl(((((0x8u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10fcf014 orr w8, w24, w21
{let a=((cpu.x[24] as u32 as u64));let b=((cpu.x[21] as u32 as u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fcf018 tbz w8, #0x1f, #0x10fcf05c
if ((cpu.x[8] as u32 as u64) & (1u64 << 31)) == 0 {pc=0x10fcf05c;continue;}pc=0x10fcf01c;continue;
}
0x10fcf01c=>{
// 0x10fcf01c cmp w21, #0
{let a=((cpu.x[21] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcf020 b.gt #0x10fcf02c
if cpu.condition(12){pc=0x10fcf02c;continue;}pc=0x10fcf024;continue;
}
0x10fcf024=>{
// 0x10fcf024 cmp w24, #1
{let a=((cpu.x[24] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcf028 b.lt #0x10fcf05c
if cpu.condition(11){pc=0x10fcf05c;continue;}pc=0x10fcf02c;continue;
}
0x10fcf02c=>{
// 0x10fcf02c adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x10fcf030 add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fcf034 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10fcf038 bl #0x1128df18
cpu.x[30]=0x10fcf03c;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10fcf03c b #0x10fcf05c
pc=0x10fcf05c;continue;
}
0x10fcf040=>{
// 0x10fcf040 adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x10fcf044 add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fcf048 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10fcf04c bl #0x1128df18
cpu.x[30]=0x10fcf050;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10fcf050 lsl w24, w21, #8
cpu.x[24]=(((((cpu.x[21] as u32 as u64))) as u32).wrapping_shl(((((0x8u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10fcf054 orr w8, w24, w21
{let a=((cpu.x[24] as u32 as u64));let b=((cpu.x[21] as u32 as u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fcf058 tbnz w8, #0x1f, #0x10fcf01c
if ((cpu.x[8] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x10fcf01c;continue;}pc=0x10fcf05c;continue;
}
0x10fcf05c=>{
// 0x10fcf05c sdiv w8, w28, w22
{let a=((cpu.x[28] as u32 as u64)) as i32;let b=((cpu.x[22] as u32 as u64)) as i32;cpu.x[8]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u32 as u64;}
// 0x10fcf060 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fcf064 sdiv w9, w24, w22
{let a=((cpu.x[24] as u32 as u64)) as i32;let b=((cpu.x[22] as u32 as u64)) as i32;cpu.x[9]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u32 as u64;}
// 0x10fcf068 mul w8, w8, w26
cpu.x[8]=((((cpu.x[8] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fcf06c add w10, w8, #0xff
{let a=((cpu.x[8] as u32 as u64));let b=((0xffu64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10fcf070 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcf074 csel w22, w10, w8, lt
cpu.x[22]=(if cpu.condition(11){((cpu.x[10] as u32 as u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10fcf078 mul w9, w9, w26
cpu.x[9]=((((cpu.x[9] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fcf07c add w8, w9, #0xff
{let a=((cpu.x[9] as u32 as u64));let b=((0xffu64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fcf080 cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcf084 csel w28, w8, w9, lt
cpu.x[28]=(if cpu.condition(11){((cpu.x[8] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10fcf088 bl #0x10f940fc
cpu.x[30]=0x10fcf08c;super::chunk9::f_10f940fc(cpu,rt,0x10f940fc)?;
pc=0x10fcf08c;continue;
}
0x10fcf08c=>{
// 0x10fcf08c cbz x0, #0x10fcf0a0
if (cpu.x[0]) == 0 {pc=0x10fcf0a0;continue;}pc=0x10fcf090;continue;
}
0x10fcf090=>{
// 0x10fcf090 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fcf094 bl #0x10f940fc
cpu.x[30]=0x10fcf098;super::chunk9::f_10f940fc(cpu,rt,0x10f940fc)?;
pc=0x10fcf098;continue;
}
0x10fcf098=>{
// 0x10fcf098 bl #0x10fc21d8
cpu.x[30]=0x10fcf09c;super::chunk12::f_10fc21d8(cpu,rt,0x10fc21d8)?;
pc=0x10fcf09c;continue;
}
0x10fcf09c=>{
// 0x10fcf09c tbnz w0, #0, #0x10fcf0cc
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10fcf0cc;continue;}pc=0x10fcf0a0;continue;
}
0x10fcf0a0=>{
// 0x10fcf0a0 ldrb w8, [x20, #0x1fc]
{let address=(cpu.x[20]).wrapping_add(0x1fcu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcf0a4 cbnz w8, #0x10fcf0cc
if (cpu.x[8] as u32 as u64) != 0 {pc=0x10fcf0cc;continue;}pc=0x10fcf0a8;continue;
}
0x10fcf0a8=>{
// 0x10fcf0a8 tbz w27, #0, #0x10fcf0cc
if ((cpu.x[27] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fcf0cc;continue;}pc=0x10fcf0ac;continue;
}
0x10fcf0ac=>{
// 0x10fcf0ac ldr x0, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fcf0b0 bl #0x10f81b34
cpu.x[30]=0x10fcf0b4;super::chunk11::f_10f81b34(cpu,rt,0x10f81b34)?;
pc=0x10fcf0b4;continue;
}
0x10fcf0b4=>{
// 0x10fcf0b4 ldr d8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.q[8]=(v0) as u128 & 0xffffffffffffffffu128;}
// 0x10fcf0b8 mov w1, #0x100
cpu.x[1]=(((0x100u64))) as u32 as u64;
// 0x10fcf0bc mov x27, x0
cpu.x[27]=(((cpu.x[0]))) as u64;
// 0x10fcf0c0 stp w19, w21, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=cpu.x[19] as u32 as u64;let v1=cpu.x[21] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10fcf0c4 bl #0x11294d74
cpu.x[30]=0x10fcf0c8;super::chunk2::f_11294d74(cpu,rt,0x11294d74)?;
pc=0x10fcf0c8;continue;
}
0x10fcf0c8=>{
// 0x10fcf0c8 cbz w0, #0x10fcf418
if (cpu.x[0] as u32 as u64) == 0 {pc=0x10fcf418;continue;}pc=0x10fcf0cc;continue;
}
0x10fcf0cc=>{
// 0x10fcf0cc ldr w8, [x20, #0x1dc]
{let address=(cpu.x[20]).wrapping_add(0x1dcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcf0d0 ldur x24, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[24]=(v0) as u64;}
// 0x10fcf0d4 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcf0d8 b.gt #0x10fcf42c
if cpu.condition(12){pc=0x10fcf42c;continue;}pc=0x10fcf0dc;continue;
}
0x10fcf0dc=>{
// 0x10fcf0dc ldr w8, [x20, #0x1f4]
{let address=(cpu.x[20]).wrapping_add(0x1f4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcf0e0 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcf0e4 b.ge #0x10fcf42c
if cpu.condition(10){pc=0x10fcf42c;continue;}pc=0x10fcf0e8;continue;
}
0x10fcf0e8=>{
// 0x10fcf0e8 mov w19, wzr
cpu.x[19]=(((0u64))) as u32 as u64;
// 0x10fcf0ec asr w27, w22, #8
cpu.x[27]=(((((cpu.x[22] as u32 as u64))) as i32).wrapping_shr(((((0x8u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10fcf0f0 asr w28, w28, #8
cpu.x[28]=(((((cpu.x[28] as u32 as u64))) as i32).wrapping_shr(((((0x8u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10fcf0f4 cbz w8, #0x10fcf158
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10fcf158;continue;}pc=0x10fcf0f8;continue;
}
0x10fcf0f8=>{
// 0x10fcf0f8 cmp w8, #0x100
{let a=((cpu.x[8] as u32 as u64));let b=((0x100u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcf0fc mov w9, #0x100
cpu.x[9]=(((0x100u64))) as u32 as u64;
// 0x10fcf100 mov w10, #-0x100
cpu.x[10]=(((0xffffffffffffff00u64))) as u32 as u64;
// 0x10fcf104 csel w8, w8, w9, lt
cpu.x[8]=(if cpu.condition(11){((cpu.x[8] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10fcf108 add x0, x20, #0x208
{let a=((cpu.x[20]));let b=((0x208u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fcf10c mov w1, w26
cpu.x[1]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fcf110 cmn w8, #0x100
{let a=((cpu.x[8] as u32 as u64));let b=((0x100u64));let result=a.wrapping_add(b);cpu.flags(a,b,result,32,false);}
// 0x10fcf114 csel w8, w8, w10, gt
cpu.x[8]=(if cpu.condition(12){((cpu.x[8] as u32 as u64))}else{((cpu.x[10] as u32 as u64))}) as u32 as u64;
// 0x10fcf118 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcf11c mul w11, w8, w28
cpu.x[11]=((((cpu.x[8] as u32 as u64))).wrapping_mul(((cpu.x[28] as u32 as u64)))) as u32 as u64;
// 0x10fcf120 cneg w10, w8, mi
cpu.x[10]=(if cpu.condition(4){0u64.wrapping_sub(((cpu.x[8] as u32 as u64)))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10fcf124 mneg w8, w27, w8
cpu.x[8]=(0u64.wrapping_sub((((cpu.x[27] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10fcf128 sub w9, w9, w10
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fcf12c mul w10, w9, w27
cpu.x[10]=((((cpu.x[9] as u32 as u64))).wrapping_mul(((cpu.x[27] as u32 as u64)))) as u32 as u64;
// 0x10fcf130 mul w9, w9, w28
cpu.x[9]=((((cpu.x[9] as u32 as u64))).wrapping_mul(((cpu.x[28] as u32 as u64)))) as u32 as u64;
// 0x10fcf134 asr w10, w10, #8
cpu.x[10]=(((((cpu.x[10] as u32 as u64))) as i32).wrapping_shr(((((0x8u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10fcf138 asr w9, w9, #8
cpu.x[9]=(((((cpu.x[9] as u32 as u64))) as i32).wrapping_shr(((((0x8u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10fcf13c add w10, w10, w11, asr #8
{let a=((cpu.x[10] as u32 as u64));let b=(((cpu.x[11] as u32 as u64)) as i32).wrapping_shr(8) as u64;let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10fcf140 add w8, w9, w8, asr #8
{let a=((cpu.x[9] as u32 as u64));let b=(((cpu.x[8] as u32 as u64)) as i32).wrapping_shr(8) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fcf144 str w10, [x20, #0x208]
{let address=(cpu.x[20]).wrapping_add(0x208u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fcf148 str w8, [x20, #0x20c]
{let address=(cpu.x[20]).wrapping_add(0x20cu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fcf14c bl #0x11294d74
cpu.x[30]=0x10fcf150;super::chunk2::f_11294d74(cpu,rt,0x11294d74)?;
pc=0x10fcf150;continue;
}
0x10fcf150=>{
// 0x10fcf150 ldr w27, [x20, #0x208]
{let address=(cpu.x[20]).wrapping_add(0x208u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[27]=(v0) as u32 as u64;}
// 0x10fcf154 ldr w28, [x20, #0x20c]
{let address=(cpu.x[20]).wrapping_add(0x20cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[28]=(v0) as u32 as u64;}
pc=0x10fcf158;continue;
}
0x10fcf158=>{
// 0x10fcf158 ldr w8, [x20, #0x1dc]
{let address=(cpu.x[20]).wrapping_add(0x1dcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcf15c cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcf160 b.lt #0x10fcf1d8
if cpu.condition(11){pc=0x10fcf1d8;continue;}pc=0x10fcf164;continue;
}
0x10fcf164=>{
// 0x10fcf164 ldr w9, [x20, #0x1d4]
{let address=(cpu.x[20]).wrapping_add(0x1d4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fcf168 ldr w10, [x20, #0x1d8]
{let address=(cpu.x[20]).wrapping_add(0x1d8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10fcf16c add x21, x20, #0x1d4
{let a=((cpu.x[20]));let b=((0x1d4u64));let result=a.wrapping_add(b);cpu.x[21]=(result) as u64;}
// 0x10fcf170 sdiv w9, w9, w8
{let a=((cpu.x[9] as u32 as u64)) as i32;let b=((cpu.x[8] as u32 as u64)) as i32;cpu.x[9]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u32 as u64;}
// 0x10fcf174 sdiv w8, w10, w8
{let a=((cpu.x[10] as u32 as u64)) as i32;let b=((cpu.x[8] as u32 as u64)) as i32;cpu.x[8]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u32 as u64;}
// 0x10fcf178 ldrb w10, [x20, #0x217]
{let address=(cpu.x[20]).wrapping_add(0x217u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10fcf17c str w9, [x20, #0x208]
{let address=(cpu.x[20]).wrapping_add(0x208u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fcf180 str w8, [x20, #0x20c]
{let address=(cpu.x[20]).wrapping_add(0x20cu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fcf184 cbnz w10, #0x10fcf1a8
if (cpu.x[10] as u32 as u64) != 0 {pc=0x10fcf1a8;continue;}pc=0x10fcf188;continue;
}
0x10fcf188=>{
// 0x10fcf188 add x0, x20, #0x208
{let a=((cpu.x[20]));let b=((0x208u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fcf18c bl #0x11294cc4
cpu.x[30]=0x10fcf190;super::chunk4::f_11294cc4(cpu,rt,0x11294cc4)?;
pc=0x10fcf190;continue;
}
0x10fcf190=>{
// 0x10fcf190 mov w8, #0x57e5
cpu.x[8]=(((0x57e5u64))) as u32 as u64;
// 0x10fcf194 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcf198 b.lt #0x10fcf1a8
if cpu.condition(11){pc=0x10fcf1a8;continue;}pc=0x10fcf19c;continue;
}
0x10fcf19c=>{
// 0x10fcf19c add x0, x20, #0x208
{let a=((cpu.x[20]));let b=((0x208u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fcf1a0 mov w1, #0x96
cpu.x[1]=(((0x96u64))) as u32 as u64;
// 0x10fcf1a4 bl #0x11294d74
cpu.x[30]=0x10fcf1a8;super::chunk2::f_11294d74(cpu,rt,0x11294d74)?;
pc=0x10fcf1a8;continue;
}
0x10fcf1a8=>{
// 0x10fcf1a8 ldrb w8, [x20, #0x216]
{let address=(cpu.x[20]).wrapping_add(0x216u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcf1ac ldr w9, [x20, #0x208]
{let address=(cpu.x[20]).wrapping_add(0x208u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fcf1b0 ldr w10, [x20, #0x20c]
{let address=(cpu.x[20]).wrapping_add(0x20cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10fcf1b4 str xzr, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fcf1b8 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcf1bc add w27, w9, w27
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[27] as u32 as u64));let result=a.wrapping_add(b);cpu.x[27]=(result) as u32 as u64;}
// 0x10fcf1c0 add w28, w10, w28
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[28] as u32 as u64));let result=a.wrapping_add(b);cpu.x[28]=(result) as u32 as u64;}
// 0x10fcf1c4 cset w21, ne
cpu.x[21]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x10fcf1c8 str wzr, [x20, #0x1dc]
{let address=(cpu.x[20]).wrapping_add(0x1dcu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fcf1cc strh wzr, [x20, #0x216]
{let address=(cpu.x[20]).wrapping_add(0x216u64);let v0=0u64;rt.memory.write::<u16>(address.wrapping_add(0),v0 as u16);}
// 0x10fcf1d0 tbz w21, #0, #0x10fcf1e0
if ((cpu.x[21] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fcf1e0;continue;}pc=0x10fcf1d4;continue;
}
0x10fcf1d4=>{
// 0x10fcf1d4 b #0x10fcf258
pc=0x10fcf258;continue;
}
0x10fcf1d8=>{
// 0x10fcf1d8 mov w21, wzr
cpu.x[21]=(((0u64))) as u32 as u64;
// 0x10fcf1dc tbnz wzr, #0, #0x10fcf258
if ((0u64) & (1u64 << 0)) != 0 {pc=0x10fcf258;continue;}pc=0x10fcf1e0;continue;
}
0x10fcf1e0=>{
// 0x10fcf1e0 eor w8, w19, #1
{let a=((cpu.x[19] as u32 as u64));let b=((0x1u64));let result=a ^ b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fcf1e4 tbnz w8, #0, #0x10fcf258
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10fcf258;continue;}pc=0x10fcf1e8;continue;
}
0x10fcf1e8=>{
// 0x10fcf1e8 ldr x8, [x24, #0x10]
{let address=(cpu.x[24]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcf1ec mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fcf1f0 str x8, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fcf1f4 bl #0x10f94298
cpu.x[30]=0x10fcf1f8;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
// 0x10fcf1f8 mov w21, #0x4dd3
cpu.x[21]=(((0x4dd3u64))) as u32 as u64;
// 0x10fcf1fc mov w22, w19
cpu.x[22]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10fcf200 mov w19, w26
cpu.x[19]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fcf204 movk w21, #0x1062, lsl #16
cpu.x[21]=((cpu.x[21] as u32 as u64 & !(0xffff0000u64)) | 0x10620000u64) as u32 as u64;
// 0x10fcf208 mov w26, w25
cpu.x[26]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fcf20c smull x8, w0, w21
cpu.x[8]=(((((cpu.x[0] as u32 as u64))) as i32 as i64).wrapping_mul((((cpu.x[21] as u32 as u64))) as i32 as i64) as u64) as u64;
// 0x10fcf210 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fcf214 lsr x24, x8, #0x3f
cpu.x[24]=(((((cpu.x[8]))) as u64).wrapping_shr(((((0x3fu64))) & 63) as u32) as u64) as u64;
// 0x10fcf218 asr x25, x8, #0x25
cpu.x[25]=(((((cpu.x[8]))) as i64).wrapping_shr(((((0x25u64))) & 63) as u32) as u64) as u64;
// 0x10fcf21c bl #0x10f942a0
cpu.x[30]=0x10fcf220;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
// 0x10fcf220 smull x8, w0, w21
cpu.x[8]=(((((cpu.x[0] as u32 as u64))) as i32 as i64).wrapping_mul((((cpu.x[21] as u32 as u64))) as i32 as i64) as u64) as u64;
// 0x10fcf224 ldr x0, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fcf228 add w1, w25, w24
{let a=((cpu.x[25] as u32 as u64));let b=((cpu.x[24] as u32 as u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u32 as u64;}
// 0x10fcf22c ldur x24, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[24]=(v0) as u64;}
// 0x10fcf230 mov w3, #1
cpu.x[3]=(((0x1u64))) as u32 as u64;
// 0x10fcf234 mov x4, xzr
cpu.x[4]=(((0u64))) as u64;
// 0x10fcf238 mov w25, w26
cpu.x[25]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fcf23c mov w26, w19
cpu.x[26]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10fcf240 mov w19, w22
cpu.x[19]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10fcf244 lsr x9, x8, #0x3f
cpu.x[9]=(((((cpu.x[8]))) as u64).wrapping_shr(((((0x3fu64))) & 63) as u32) as u64) as u64;
// 0x10fcf248 asr x8, x8, #0x25
cpu.x[8]=(((((cpu.x[8]))) as i64).wrapping_shr(((((0x25u64))) & 63) as u32) as u64) as u64;
// 0x10fcf24c add w2, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_add(b);cpu.x[2]=(result) as u32 as u64;}
// 0x10fcf250 bl #0x111d9240
cpu.x[30]=0x10fcf254;super::chunk1::f_111d9240(cpu,rt,0x111d9240)?;
// 0x10fcf254 eor w21, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a ^ b;cpu.x[21]=(result) as u32 as u64;}
pc=0x10fcf258;continue;
}
0x10fcf258=>{
// 0x10fcf258 tbz w19, #0, #0x10fcf2b4
if ((cpu.x[19] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fcf2b4;continue;}pc=0x10fcf25c;continue;
}
0x10fcf25c=>{
// 0x10fcf25c ldr x22, [x24, #0x10]
{let address=(cpu.x[24]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[22]=(v0) as u64;}
// 0x10fcf260 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fcf264 bl #0x10f94298
cpu.x[30]=0x10fcf268;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
// 0x10fcf268 mov w19, w0
cpu.x[19]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fcf26c mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fcf270 bl #0x10f942a0
cpu.x[30]=0x10fcf274;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
// 0x10fcf274 mov w2, w0
cpu.x[2]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fcf278 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10fcf27c mov w1, w19
cpu.x[1]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10fcf280 bl #0x111d9838
cpu.x[30]=0x10fcf284;super::dispatch(cpu,rt,0x111d9838)?;
// 0x10fcf284 tbz w0, #0, #0x10fcf2b4
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fcf2b4;continue;}pc=0x10fcf288;continue;
}
0x10fcf288=>{
// 0x10fcf288 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fcf28c bl #0x10f942a0
cpu.x[30]=0x10fcf290;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
// 0x10fcf290 mov w19, w0
cpu.x[19]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fcf294 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10fcf298 bl #0x10d358a8
cpu.x[30]=0x10fcf29c;super::chunk13::f_10d358a8(cpu,rt,0x10d358a8)?;
// 0x10fcf29c mov w8, #0xfa
cpu.x[8]=(((0xfau64))) as u32 as u64;
// 0x10fcf2a0 mul w8, w0, w8
cpu.x[8]=((((cpu.x[0] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64)))) as u32 as u64;
// 0x10fcf2a4 cmp w19, w8
{let a=((cpu.x[19] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcf2a8 mov w8, #-0x100
cpu.x[8]=(((0xffffffffffffff00u64))) as u32 as u64;
// 0x10fcf2ac cneg w8, w8, ge
cpu.x[8]=(if cpu.condition(10){0u64.wrapping_sub(((cpu.x[8] as u32 as u64)))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10fcf2b0 add w28, w8, w28
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[28] as u32 as u64));let result=a.wrapping_add(b);cpu.x[28]=(result) as u32 as u64;}
pc=0x10fcf2b4;continue;
}
0x10fcf2b4=>{
// 0x10fcf2b4 ldr x19, [x24, #0x10]
{let address=(cpu.x[24]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10fcf2b8 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fcf2bc bl #0x10f942c0
cpu.x[30]=0x10fcf2c0;super::chunk2::f_10f942c0(cpu,rt,0x10f942c0)?;
pc=0x10fcf2c0;continue;
}
0x10fcf2c0=>{
// 0x10fcf2c0 mov x1, x0
cpu.x[1]=(((cpu.x[0]))) as u64;
// 0x10fcf2c4 and w5, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[5]=(result) as u32 as u64;}
// 0x10fcf2c8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fcf2cc mov w2, w27
cpu.x[2]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fcf2d0 mov w3, w28
cpu.x[3]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10fcf2d4 mov x4, x23
cpu.x[4]=(((cpu.x[23]))) as u64;
// 0x10fcf2d8 bl #0x111d988c
cpu.x[30]=0x10fcf2dc;super::chunk1::f_111d988c(cpu,rt,0x111d988c)?;
pc=0x10fcf2dc;continue;
}
0x10fcf2dc=>{
// 0x10fcf2dc mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fcf2e0 bl #0x10f94298
cpu.x[30]=0x10fcf2e4;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
pc=0x10fcf2e4;continue;
}
0x10fcf2e4=>{
// 0x10fcf2e4 ldr w8, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcf2e8 sub w19, w8, w0
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[19]=(result) as u32 as u64;}
// 0x10fcf2ec mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fcf2f0 bl #0x10f942a0
cpu.x[30]=0x10fcf2f4;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
pc=0x10fcf2f4;continue;
}
0x10fcf2f4=>{
// 0x10fcf2f4 ldp w8, w9, [x20, #0x30]
{let address=(cpu.x[20]).wrapping_add(0x30u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[8]=(v0) as u32 as u64;cpu.x[9]=(v1) as u32 as u64;}
// 0x10fcf2f8 ldur w10, [x29, #-0x14]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffecu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10fcf2fc sub w10, w10, w0
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10fcf300 mul w8, w8, w19
cpu.x[8]=((((cpu.x[8] as u32 as u64))).wrapping_mul(((cpu.x[19] as u32 as u64)))) as u32 as u64;
// 0x10fcf304 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcf308 mul w9, w9, w10
cpu.x[9]=((((cpu.x[9] as u32 as u64))).wrapping_mul(((cpu.x[10] as u32 as u64)))) as u32 as u64;
// 0x10fcf30c add w10, w8, #0xff
{let a=((cpu.x[8] as u32 as u64));let b=((0xffu64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10fcf310 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcf314 csel w8, w10, w8, lt
cpu.x[8]=(if cpu.condition(11){((cpu.x[10] as u32 as u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10fcf318 add w10, w9, #0xff
{let a=((cpu.x[9] as u32 as u64));let b=((0xffu64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10fcf31c cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcf320 csel w9, w10, w9, lt
cpu.x[9]=(if cpu.condition(11){((cpu.x[10] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10fcf324 asr w9, w9, #8
cpu.x[9]=(((((cpu.x[9] as u32 as u64))) as i32).wrapping_shr(((((0x8u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10fcf328 add w19, w9, w8, asr #8
{let a=((cpu.x[9] as u32 as u64));let b=(((cpu.x[8] as u32 as u64)) as i32).wrapping_shr(8) as u64;let result=a.wrapping_add(b);cpu.x[19]=(result) as u32 as u64;}
// 0x10fcf32c bl #0x10fd04a8
cpu.x[30]=0x10fcf330;super::chunk2::f_10fd04a8(cpu,rt,0x10fd04a8)?;
pc=0x10fcf330;continue;
}
0x10fcf330=>{
// 0x10fcf330 bl #0x10e0b6b4
cpu.x[30]=0x10fcf334;super::chunk8::f_10e0b6b4(cpu,rt,0x10e0b6b4)?;
pc=0x10fcf334;continue;
}
0x10fcf334=>{
// 0x10fcf334 ldr x0, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fcf338 bl #0x10e16208
cpu.x[30]=0x10fcf33c;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
pc=0x10fcf33c;continue;
}
0x10fcf33c=>{
// 0x10fcf33c ldrb w8, [x0, #0x3bd]
{let address=(cpu.x[0]).wrapping_add(0x3bdu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcf340 cbz w8, #0x10fcf3cc
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10fcf3cc;continue;}pc=0x10fcf344;continue;
}
0x10fcf344=>{
// 0x10fcf344 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fcf348 bl #0x10f817a8
cpu.x[30]=0x10fcf34c;super::chunk8::f_10f817a8(cpu,rt,0x10f817a8)?;
pc=0x10fcf34c;continue;
}
0x10fcf34c=>{
// 0x10fcf34c cmp w0, #6
{let a=((cpu.x[0] as u32 as u64));let b=((0x6u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcf350 b.ne #0x10fcf398
if cpu.condition(1){pc=0x10fcf398;continue;}pc=0x10fcf354;continue;
}
0x10fcf354=>{
// 0x10fcf354 ldr x0, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fcf358 bl #0x10de06b0
cpu.x[30]=0x10fcf35c;super::chunk2::f_10de06b0(cpu,rt,0x10de06b0)?;
// 0x10fcf35c ldr w8, [x20, #0x1e0]
{let address=(cpu.x[20]).wrapping_add(0x1e0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcf360 cmp w19, w0
{let a=((cpu.x[19] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcf364 cset w9, le
cpu.x[9]=((cpu.condition(13)) as u64) as u32 as u64;
// 0x10fcf368 cmn w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.flags(a,b,result,32,false);}
// 0x10fcf36c strb w9, [x20, #0x1ff]
{let address=(cpu.x[20]).wrapping_add(0x1ffu64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10fcf370 b.ne #0x10fcf3e8
if cpu.condition(1){pc=0x10fcf3e8;continue;}pc=0x10fcf374;continue;
}
0x10fcf374=>{
// 0x10fcf374 ldp x20, x19, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fcf378 ldr d8, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.q[8]=(v0) as u128 & 0xffffffffffffffffu128;}
// 0x10fcf37c ldp x22, x21, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fcf380 ldp x24, x23, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fcf384 ldp x26, x25, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fcf388 ldp x28, x27, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fcf38c ldp x29, x30, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fcf390 add sp, sp, #0x90
{let a=((cpu.sp));let b=((0x90u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fcf394 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10fcf398=>{
// 0x10fcf398 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fcf39c bl #0x10f817a8
cpu.x[30]=0x10fcf3a0;super::chunk8::f_10f817a8(cpu,rt,0x10f817a8)?;
pc=0x10fcf3a0;continue;
}
0x10fcf3a0=>{
// 0x10fcf3a0 cmp w0, #7
{let a=((cpu.x[0] as u32 as u64));let b=((0x7u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcf3a4 b.ne #0x10fcf3cc
if cpu.condition(1){pc=0x10fcf3cc;continue;}pc=0x10fcf3a8;continue;
}
0x10fcf3a8=>{
// 0x10fcf3a8 ldr x0, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fcf3ac bl #0x10de07ac
cpu.x[30]=0x10fcf3b0;super::dispatch(cpu,rt,0x10de07ac)?;
// 0x10fcf3b0 ldr w8, [x20, #0x1e0]
{let address=(cpu.x[20]).wrapping_add(0x1e0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcf3b4 cmp w19, w0
{let a=((cpu.x[19] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcf3b8 cset w9, le
cpu.x[9]=((cpu.condition(13)) as u64) as u32 as u64;
// 0x10fcf3bc cmn w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.flags(a,b,result,32,false);}
// 0x10fcf3c0 strb w9, [x20, #0x1ff]
{let address=(cpu.x[20]).wrapping_add(0x1ffu64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10fcf3c4 b.ne #0x10fcf3e8
if cpu.condition(1){pc=0x10fcf3e8;continue;}pc=0x10fcf3c8;continue;
}
0x10fcf3c8=>{
// 0x10fcf3c8 b #0x10fcf374
pc=0x10fcf374;continue;
}
0x10fcf3cc=>{
// 0x10fcf3cc mov w0, #0x3e8
cpu.x[0]=(((0x3e8u64))) as u32 as u64;
// 0x10fcf3d0 ldr w8, [x20, #0x1e0]
{let address=(cpu.x[20]).wrapping_add(0x1e0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcf3d4 cmp w19, w0
{let a=((cpu.x[19] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcf3d8 cset w9, le
cpu.x[9]=((cpu.condition(13)) as u64) as u32 as u64;
// 0x10fcf3dc cmn w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.flags(a,b,result,32,false);}
// 0x10fcf3e0 strb w9, [x20, #0x1ff]
{let address=(cpu.x[20]).wrapping_add(0x1ffu64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10fcf3e4 b.eq #0x10fcf374
if cpu.condition(0){pc=0x10fcf374;continue;}pc=0x10fcf3e8;continue;
}
0x10fcf3e8=>{
// 0x10fcf3e8 and w2, w25, #1
{let a=((cpu.x[25] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[2]=(result) as u32 as u64;}
// 0x10fcf3ec mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcf3f0 mov w1, w26
cpu.x[1]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fcf3f4 ldp x20, x19, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fcf3f8 ldr d8, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.q[8]=(v0) as u128 & 0xffffffffffffffffu128;}
// 0x10fcf3fc ldp x22, x21, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
pc=0x10fcf400;continue;
}
0x10fcf400=>{
// 0x10fcf400 ldp x24, x23, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fcf404 ldp x26, x25, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fcf408 ldp x28, x27, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fcf40c ldp x29, x30, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fcf410 add sp, sp, #0x90
{let a=((cpu.sp));let b=((0x90u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fcf414 b #0x10fcf8d4
super::chunk3::f_10fcf8d4(cpu,rt,0x10fcf8d4)?;return Ok(());
}
0x10fcf418=>{
// 0x10fcf418 str d8, [x27]
{let address=(cpu.x[27]).wrapping_add(0x0u64);let v0=(cpu.q[8] & 0xffffffffffffffffu128) as u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fcf41c ldr w8, [x20, #0x1dc]
{let address=(cpu.x[20]).wrapping_add(0x1dcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcf420 ldur x24, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[24]=(v0) as u64;}
// 0x10fcf424 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcf428 b.le #0x10fcf0dc
if cpu.condition(13){pc=0x10fcf0dc;continue;}pc=0x10fcf42c;continue;
}
0x10fcf42c=>{
// 0x10fcf42c mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10fcf430 bl #0x10d43a9c
cpu.x[30]=0x10fcf434;super::chunk11::f_10d43a9c(cpu,rt,0x10d43a9c)?;
pc=0x10fcf434;continue;
}
0x10fcf434=>{
// 0x10fcf434 tbz w0, #0, #0x10fcf44c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fcf44c;continue;}pc=0x10fcf438;continue;
}
0x10fcf438=>{
// 0x10fcf438 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcf43c mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fcf440 ldr x8, [x8, #0x108]
{let address=(cpu.x[8]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcf444 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fcf448;super::dispatch(cpu,rt,target)?;}
// 0x10fcf448 tbz w0, #0, #0x10fcf464
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fcf464;continue;}pc=0x10fcf44c;continue;
}
0x10fcf44c=>{
// 0x10fcf44c mov w19, wzr
cpu.x[19]=(((0u64))) as u32 as u64;
// 0x10fcf450 ldr w8, [x20, #0x1f4]
{let address=(cpu.x[20]).wrapping_add(0x1f4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcf454 asr w27, w22, #8
cpu.x[27]=(((((cpu.x[22] as u32 as u64))) as i32).wrapping_shr(((((0x8u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10fcf458 asr w28, w28, #8
cpu.x[28]=(((((cpu.x[28] as u32 as u64))) as i32).wrapping_shr(((((0x8u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10fcf45c cbnz w8, #0x10fcf0f8
if (cpu.x[8] as u32 as u64) != 0 {pc=0x10fcf0f8;continue;}pc=0x10fcf460;continue;
}
0x10fcf460=>{
// 0x10fcf460 b #0x10fcf158
pc=0x10fcf158;continue;
}
0x10fcf464=>{
// 0x10fcf464 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcf468 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fcf46c ldr x8, [x8, #0x118]
{let address=(cpu.x[8]).wrapping_add(0x118u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcf470 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fcf474;super::dispatch(cpu,rt,target)?;}
// 0x10fcf474 eor w19, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a ^ b;cpu.x[19]=(result) as u32 as u64;}
// 0x10fcf478 ldr w8, [x20, #0x1f4]
{let address=(cpu.x[20]).wrapping_add(0x1f4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcf47c asr w27, w22, #8
cpu.x[27]=(((((cpu.x[22] as u32 as u64))) as i32).wrapping_shr(((((0x8u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10fcf480 asr w28, w28, #8
cpu.x[28]=(((((cpu.x[28] as u32 as u64))) as i32).wrapping_shr(((((0x8u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10fcf484 cbnz w8, #0x10fcf0f8
if (cpu.x[8] as u32 as u64) != 0 {pc=0x10fcf0f8;continue;}pc=0x10fcf488;continue;
}
0x10fcf488=>{
// 0x10fcf488 b #0x10fcf158
pc=0x10fcf158;continue;
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fcef48")),}}}
#[inline]
pub(super) unsafe fn f_10fd04c8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fd04c8=>{
// 0x10fd04c8 str x1, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=cpu.x[1];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fd04cc ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fd04c8")),}}}
#[inline]
pub(super) unsafe fn f_10fd1184(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fd1184=>{
// 0x10fd1184 ldr x0, [x0, #0x30]
{let address=(cpu.x[0]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fd1188 cbz x0, #0x10fd1190
if (cpu.x[0]) == 0 {pc=0x10fd1190;continue;}pc=0x10fd118c;continue;
}
0x10fd118c=>{
// 0x10fd118c b #0x10f87f00
super::chunk12::f_10f87f00(cpu,rt,0x10f87f00)?;return Ok(());
}
0x10fd1190=>{
// 0x10fd1190 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fd1184")),}}}
#[inline]
pub(super) unsafe fn f_110c94f8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x110c94f8=>{
// 0x110c94f8 adrp x8, #0x11944000
cpu.x[8]=(0x11944000u64) as u64;
// 0x110c94fc add x8, x8, #0x720
{let a=((cpu.x[8]));let b=((0x720u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x110c9500 str x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x110c9504 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x110c94f8")),}}}
#[inline]
pub(super) unsafe fn f_110c9904(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x110c9904=>{
// 0x110c9904 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x110c9908 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x110c990c stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x110c9910 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x110c9914 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x110c9918 ldr x0, [x0, #0xa8]
{let address=(cpu.x[0]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c991c mov w19, w1
cpu.x[19]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x110c9920 strb w1, [x20, #0x1e8]
{let address=(cpu.x[20]).wrapping_add(0x1e8u64);let v0=cpu.x[1] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x110c9924 cbz x0, #0x110c9934
if (cpu.x[0]) == 0 {pc=0x110c9934;continue;}pc=0x110c9928;continue;
}
0x110c9928=>{
// 0x110c9928 and w21, w19, #1
{let a=((cpu.x[19] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[21]=(result) as u32 as u64;}
// 0x110c992c bl #0x10d34428
cpu.x[30]=0x110c9930;super::chunk13::f_10d34428(cpu,rt,0x10d34428)?;
pc=0x110c9930;continue;
}
0x110c9930=>{
// 0x110c9930 strb w21, [x0, #0x79]
{let address=(cpu.x[0]).wrapping_add(0x79u64);let v0=cpu.x[21] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
pc=0x110c9934;continue;
}
0x110c9934=>{
// 0x110c9934 ldr x0, [x20, #0x218]
{let address=(cpu.x[20]).wrapping_add(0x218u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9938 cbz x0, #0x110c9958
if (cpu.x[0]) == 0 {pc=0x110c9958;continue;}pc=0x110c993c;continue;
}
0x110c993c=>{
// 0x110c993c ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9940 ldr x2, [x8, #0x50]
{let address=(cpu.x[8]).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x110c9944 and w1, w19, #1
{let a=((cpu.x[19] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[1]=(result) as u32 as u64;}
// 0x110c9948 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x110c994c ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x110c9950 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x110c9954 br x2
{let target=cpu.x[2];if target>=0x110c9904 && target<0x110c9968{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
0x110c9958=>{
// 0x110c9958 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x110c995c ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x110c9960 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x110c9964 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x110c9904")),}}}
#[inline]
pub(super) unsafe fn f_110cba24(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x110cba24=>{
// 0x110cba24 stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x110cba28 str x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x110cba2c stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x110cba30 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x110cba34 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x110cba38 ldr x8, [x1]
{let address=(cpu.x[1]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110cba3c mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x110cba40 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x110cba44 mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x110cba48 mov w1, #0x2a
cpu.x[1]=(((0x2au64))) as u32 as u64;
// 0x110cba4c mov w20, w4
cpu.x[20]=(((cpu.x[4] as u32 as u64))) as u32 as u64;
// 0x110cba50 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110cba54 mov x22, x3
cpu.x[22]=(((cpu.x[3]))) as u64;
// 0x110cba58 mov w23, w2
cpu.x[23]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x110cba5c blr x8
{let target=cpu.x[8];cpu.x[30]=0x110cba60;super::dispatch(cpu,rt,target)?;}
pc=0x110cba60;continue;
}
0x110cba60=>{
// 0x110cba60 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110cba64 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x110cba68 mov w1, w20
cpu.x[1]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x110cba6c ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110cba70 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110cba74;super::dispatch(cpu,rt,target)?;}
pc=0x110cba74;continue;
}
0x110cba74=>{
// 0x110cba74 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110cba78 ldrb w1, [x21, #0x1d0]
{let address=(cpu.x[21]).wrapping_add(0x1d0u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x110cba7c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x110cba80 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110cba84 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110cba88;super::dispatch(cpu,rt,target)?;}
pc=0x110cba88;continue;
}
0x110cba88=>{
// 0x110cba88 ldr x0, [x21, #0xa8]
{let address=(cpu.x[21]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110cba8c cbz x0, #0x110cbaa0
if (cpu.x[0]) == 0 {pc=0x110cbaa0;continue;}pc=0x110cba90;continue;
}
0x110cba90=>{
// 0x110cba90 and w2, w23, #1
{let a=((cpu.x[23] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[2]=(result) as u32 as u64;}
// 0x110cba94 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x110cba98 mov x3, x22
cpu.x[3]=(((cpu.x[22]))) as u64;
// 0x110cba9c bl #0x10d398e8
cpu.x[30]=0x110cbaa0;super::chunk8::f_10d398e8(cpu,rt,0x10d398e8)?;
pc=0x110cbaa0;continue;
}
0x110cbaa0=>{
// 0x110cbaa0 ldr x0, [x21, #0x218]
{let address=(cpu.x[21]).wrapping_add(0x218u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110cbaa4 cbz x0, #0x110cbad8
if (cpu.x[0]) == 0 {pc=0x110cbad8;continue;}pc=0x110cbaa8;continue;
}
0x110cbaa8=>{
// 0x110cbaa8 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110cbaac and w2, w23, #1
{let a=((cpu.x[23] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[2]=(result) as u32 as u64;}
// 0x110cbab0 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x110cbab4 mov x3, x22
cpu.x[3]=(((cpu.x[22]))) as u64;
// 0x110cbab8 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110cbabc blr x8
{let target=cpu.x[8];cpu.x[30]=0x110cbac0;super::dispatch(cpu,rt,target)?;}
// 0x110cbac0 ldr x8, [x21, #0x218]
{let address=(cpu.x[21]).wrapping_add(0x218u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110cbac4 cmp x8, #0
{let a=((cpu.x[8]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x110cbac8 cset w8, eq
cpu.x[8]=((cpu.condition(0)) as u64) as u32 as u64;
// 0x110cbacc ldr x9, [x21, #0xa8]
{let address=(cpu.x[21]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x110cbad0 cbz x9, #0x110cbae4
if (cpu.x[9]) == 0 {pc=0x110cbae4;continue;}pc=0x110cbad4;continue;
}
0x110cbad4=>{
// 0x110cbad4 b #0x110cbae8
pc=0x110cbae8;continue;
}
0x110cbad8=>{
// 0x110cbad8 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x110cbadc ldr x9, [x21, #0xa8]
{let address=(cpu.x[21]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x110cbae0 cbnz x9, #0x110cbae8
if (cpu.x[9]) != 0 {pc=0x110cbae8;continue;}pc=0x110cbae4;continue;
}
0x110cbae4=>{
// 0x110cbae4 tbnz w8, #0, #0x110cbc08
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x110cbc08;continue;}pc=0x110cbae8;continue;
}
0x110cbae8=>{
// 0x110cbae8 ldr x8, [x21, #0xd8]
{let address=(cpu.x[21]).wrapping_add(0xd8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110cbaec ldr x9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x110cbaf0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x110cbaf4 cmp x8, #0
{let a=((cpu.x[8]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x110cbaf8 ldr x8, [x9, #0x40]
{let address=(cpu.x[9]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110cbafc cset w1, ne
cpu.x[1]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x110cbb00 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110cbb04;super::dispatch(cpu,rt,target)?;}
pc=0x110cbb04;continue;
}
0x110cbb04=>{
// 0x110cbb04 ldr x0, [x21, #0xd8]
{let address=(cpu.x[21]).wrapping_add(0xd8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110cbb08 cbz x0, #0x110cbb18
if (cpu.x[0]) == 0 {pc=0x110cbb18;continue;}pc=0x110cbb0c;continue;
}
0x110cbb0c=>{
// 0x110cbb0c and w2, w20, #1
{let a=((cpu.x[20] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[2]=(result) as u32 as u64;}
// 0x110cbb10 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x110cbb14 bl #0x10d245d8
cpu.x[30]=0x110cbb18;super::chunk12::f_10d245d8(cpu,rt,0x10d245d8)?;
pc=0x110cbb18;continue;
}
0x110cbb18=>{
// 0x110cbb18 ldr x8, [x21, #0xe0]
{let address=(cpu.x[21]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110cbb1c ldr x9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x110cbb20 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x110cbb24 cmp x8, #0
{let a=((cpu.x[8]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x110cbb28 ldr x8, [x9, #0x40]
{let address=(cpu.x[9]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110cbb2c cset w1, ne
cpu.x[1]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x110cbb30 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110cbb34;super::dispatch(cpu,rt,target)?;}
pc=0x110cbb34;continue;
}
0x110cbb34=>{
// 0x110cbb34 ldr x0, [x21, #0xe0]
{let address=(cpu.x[21]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110cbb38 cbz x0, #0x110cbb48
if (cpu.x[0]) == 0 {pc=0x110cbb48;continue;}pc=0x110cbb3c;continue;
}
0x110cbb3c=>{
// 0x110cbb3c and w2, w20, #1
{let a=((cpu.x[20] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[2]=(result) as u32 as u64;}
// 0x110cbb40 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x110cbb44 bl #0x10d245d8
cpu.x[30]=0x110cbb48;super::chunk12::f_10d245d8(cpu,rt,0x10d245d8)?;
pc=0x110cbb48;continue;
}
0x110cbb48=>{
// 0x110cbb48 ldr x8, [x21, #0xe8]
{let address=(cpu.x[21]).wrapping_add(0xe8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110cbb4c ldr x9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x110cbb50 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x110cbb54 cmp x8, #0
{let a=((cpu.x[8]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x110cbb58 ldr x8, [x9, #0x40]
{let address=(cpu.x[9]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110cbb5c cset w1, ne
cpu.x[1]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x110cbb60 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110cbb64;super::dispatch(cpu,rt,target)?;}
pc=0x110cbb64;continue;
}
0x110cbb64=>{
// 0x110cbb64 ldr x0, [x21, #0xe8]
{let address=(cpu.x[21]).wrapping_add(0xe8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110cbb68 cbz x0, #0x110cbb78
if (cpu.x[0]) == 0 {pc=0x110cbb78;continue;}pc=0x110cbb6c;continue;
}
0x110cbb6c=>{
// 0x110cbb6c and w2, w20, #1
{let a=((cpu.x[20] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[2]=(result) as u32 as u64;}
// 0x110cbb70 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x110cbb74 bl #0x10d245d8
cpu.x[30]=0x110cbb78;super::chunk12::f_10d245d8(cpu,rt,0x10d245d8)?;
pc=0x110cbb78;continue;
}
0x110cbb78=>{
// 0x110cbb78 ldr x8, [x21, #0xf0]
{let address=(cpu.x[21]).wrapping_add(0xf0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110cbb7c ldr x9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x110cbb80 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x110cbb84 cmp x8, #0
{let a=((cpu.x[8]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x110cbb88 ldr x8, [x9, #0x40]
{let address=(cpu.x[9]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110cbb8c cset w1, ne
cpu.x[1]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x110cbb90 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110cbb94;super::dispatch(cpu,rt,target)?;}
pc=0x110cbb94;continue;
}
0x110cbb94=>{
// 0x110cbb94 ldr x0, [x21, #0xf0]
{let address=(cpu.x[21]).wrapping_add(0xf0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110cbb98 cbz x0, #0x110cbba8
if (cpu.x[0]) == 0 {pc=0x110cbba8;continue;}pc=0x110cbb9c;continue;
}
0x110cbb9c=>{
// 0x110cbb9c and w2, w20, #1
{let a=((cpu.x[20] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[2]=(result) as u32 as u64;}
// 0x110cbba0 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x110cbba4 bl #0x10d245d8
cpu.x[30]=0x110cbba8;super::chunk12::f_10d245d8(cpu,rt,0x10d245d8)?;
pc=0x110cbba8;continue;
}
0x110cbba8=>{
// 0x110cbba8 ldr x8, [x21, #0xf8]
{let address=(cpu.x[21]).wrapping_add(0xf8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110cbbac ldr x9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x110cbbb0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x110cbbb4 cmp x8, #0
{let a=((cpu.x[8]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x110cbbb8 ldr x8, [x9, #0x40]
{let address=(cpu.x[9]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110cbbbc cset w1, ne
cpu.x[1]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x110cbbc0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110cbbc4;super::dispatch(cpu,rt,target)?;}
pc=0x110cbbc4;continue;
}
0x110cbbc4=>{
// 0x110cbbc4 ldr x0, [x21, #0xf8]
{let address=(cpu.x[21]).wrapping_add(0xf8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110cbbc8 cbz x0, #0x110cbbd8
if (cpu.x[0]) == 0 {pc=0x110cbbd8;continue;}pc=0x110cbbcc;continue;
}
0x110cbbcc=>{
// 0x110cbbcc and w2, w20, #1
{let a=((cpu.x[20] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[2]=(result) as u32 as u64;}
// 0x110cbbd0 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x110cbbd4 bl #0x10d245d8
cpu.x[30]=0x110cbbd8;super::chunk12::f_10d245d8(cpu,rt,0x10d245d8)?;
pc=0x110cbbd8;continue;
}
0x110cbbd8=>{
// 0x110cbbd8 ldr x8, [x21, #0x100]
{let address=(cpu.x[21]).wrapping_add(0x100u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110cbbdc ldr x9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x110cbbe0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x110cbbe4 cmp x8, #0
{let a=((cpu.x[8]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x110cbbe8 ldr x8, [x9, #0x40]
{let address=(cpu.x[9]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110cbbec cset w1, ne
cpu.x[1]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x110cbbf0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110cbbf4;super::dispatch(cpu,rt,target)?;}
pc=0x110cbbf4;continue;
}
0x110cbbf4=>{
// 0x110cbbf4 ldr x0, [x21, #0x100]
{let address=(cpu.x[21]).wrapping_add(0x100u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110cbbf8 cbz x0, #0x110cbc08
if (cpu.x[0]) == 0 {pc=0x110cbc08;continue;}pc=0x110cbbfc;continue;
}
0x110cbbfc=>{
// 0x110cbbfc and w2, w20, #1
{let a=((cpu.x[20] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[2]=(result) as u32 as u64;}
// 0x110cbc00 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x110cbc04 bl #0x10d245d8
cpu.x[30]=0x110cbc08;super::chunk12::f_10d245d8(cpu,rt,0x10d245d8)?;
pc=0x110cbc08;continue;
}
0x110cbc08=>{
// 0x110cbc08 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110cbc0c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x110cbc10 mov w1, #0x2b
cpu.x[1]=(((0x2bu64))) as u32 as u64;
// 0x110cbc14 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110cbc18 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110cbc1c;super::dispatch(cpu,rt,target)?;}
pc=0x110cbc1c;continue;
}
0x110cbc1c=>{
// 0x110cbc1c mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x110cbc20 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x110cbc24 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x110cbc28 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x110cbc2c ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x110cbc30 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x110cba24")),}}}
#[inline]
pub(super) unsafe fn f_110ce314(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x110ce314=>{
// 0x110ce314 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x110ce318 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x110ce31c stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x110ce320 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x110ce324 mov w19, w1
cpu.x[19]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x110ce328 cmp w1, #0
{let a=((cpu.x[1] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x110ce32c mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x110ce330 ubfiz x8, x19, #3, #0x20
cpu.x[8]=(((((cpu.x[19])) & 0xffffffffu64) << 3)) as u64;
// 0x110ce334 csinv x0, x8, xzr, ge
cpu.x[0]=(if cpu.condition(10){((cpu.x[8]))}else{!(((0u64)))}) as u64;
// 0x110ce338 bl #0x1187d520
cpu.x[30]=0x110ce33c;super::chunk8::f_1187d520(cpu,rt,0x1187d520)?;
pc=0x110ce33c;continue;
}
0x110ce33c=>{
// 0x110ce33c ldr w9, [x20, #0xc]
{let address=(cpu.x[20]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x110ce340 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110ce344 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x110ce348 cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x110ce34c b.lt #0x110ce3d4
if cpu.condition(11){pc=0x110ce3d4;continue;}pc=0x110ce350;continue;
}
0x110ce350=>{
// 0x110ce350 cmp w9, #4
{let a=((cpu.x[9] as u32 as u64));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x110ce354 b.lo #0x110ce3b4
if cpu.condition(3){pc=0x110ce3b4;continue;}pc=0x110ce358;continue;
}
0x110ce358=>{
// 0x110ce358 sub x10, x21, x8
{let a=((cpu.x[21]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.x[10]=(result) as u64;}
// 0x110ce35c mov x11, x21
cpu.x[11]=(((cpu.x[21]))) as u64;
// 0x110ce360 cmp x10, #0x20
{let a=((cpu.x[10]));let b=((0x20u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x110ce364 mov x10, x8
cpu.x[10]=(((cpu.x[8]))) as u64;
// 0x110ce368 b.lo #0x110ce3bc
if cpu.condition(3){pc=0x110ce3bc;continue;}pc=0x110ce36c;continue;
}
0x110ce36c=>{
// 0x110ce36c mov w12, w9
cpu.x[12]=(((cpu.x[9] as u32 as u64))) as u32 as u64;
// 0x110ce370 add x14, x21, #0x10
{let a=((cpu.x[21]));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[14]=(result) as u64;}
// 0x110ce374 add x15, x8, #0x10
{let a=((cpu.x[8]));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[15]=(result) as u64;}
// 0x110ce378 and x13, x12, #0x7ffffffc
{let a=((cpu.x[12]));let b=((0x7ffffffcu64));let result=a & b;cpu.x[13]=(result) as u64;}
// 0x110ce37c lsl x11, x13, #3
cpu.x[11]=(((((cpu.x[13]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x110ce380 sub w9, w9, w13
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[13] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x110ce384 mov x16, x13
cpu.x[16]=(((cpu.x[13]))) as u64;
// 0x110ce388 add x10, x8, x11
{let a=((cpu.x[8]));let b=((cpu.x[11]));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x110ce38c add x11, x21, x11
{let a=((cpu.x[21]));let b=((cpu.x[11]));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
pc=0x110ce390;continue;
}
0x110ce390=>{
// 0x110ce390 ldp q0, q1, [x15, #-0x10]
{let address=(cpu.x[15]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));let v1=rt.memory.read::<u128>(address.wrapping_add(16));cpu.q[0]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;cpu.q[1]=(v1) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x110ce394 subs x16, x16, #4
{let a=((cpu.x[16]));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.x[16]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x110ce398 add x15, x15, #0x20
{let a=((cpu.x[15]));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[15]=(result) as u64;}
// 0x110ce39c stp q0, q1, [x14, #-0x10]
{let address=(cpu.x[14]).wrapping_add(0xfffffffffffffff0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[1] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x110ce3a0 add x14, x14, #0x20
{let a=((cpu.x[14]));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[14]=(result) as u64;}
// 0x110ce3a4 b.ne #0x110ce390
if cpu.condition(1){pc=0x110ce390;continue;}pc=0x110ce3a8;continue;
}
0x110ce3a8=>{
// 0x110ce3a8 cmp x13, x12
{let a=((cpu.x[13]));let b=((cpu.x[12]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x110ce3ac b.ne #0x110ce3bc
if cpu.condition(1){pc=0x110ce3bc;continue;}pc=0x110ce3b0;continue;
}
0x110ce3b0=>{
// 0x110ce3b0 b #0x110ce3d4
pc=0x110ce3d4;continue;
}
0x110ce3b4=>{
// 0x110ce3b4 mov x10, x8
cpu.x[10]=(((cpu.x[8]))) as u64;
// 0x110ce3b8 mov x11, x21
cpu.x[11]=(((cpu.x[21]))) as u64;
pc=0x110ce3bc;continue;
}
0x110ce3bc=>{
// 0x110ce3bc add w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
pc=0x110ce3c0;continue;
}
0x110ce3c0=>{
// 0x110ce3c0 ldr x12, [x10], #8
{let address=(cpu.x[10]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[12]=(v0) as u64;cpu.x[10]=((cpu.x[10]).wrapping_add(((0x8u64)))) as u64;}
// 0x110ce3c4 sub w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x110ce3c8 cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x110ce3cc str x12, [x11], #8
{let address=(cpu.x[11]).wrapping_add(0x0u64);let v0=cpu.x[12];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);cpu.x[11]=((cpu.x[11]).wrapping_add(((0x8u64)))) as u64;}
// 0x110ce3d0 b.hi #0x110ce3c0
if cpu.condition(8){pc=0x110ce3c0;continue;}pc=0x110ce3d4;continue;
}
0x110ce3d4=>{
// 0x110ce3d4 cbz x8, #0x110ce3e0
if (cpu.x[8]) == 0 {pc=0x110ce3e0;continue;}pc=0x110ce3d8;continue;
}
0x110ce3d8=>{
// 0x110ce3d8 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x110ce3dc bl #0x1187d360
cpu.x[30]=0x110ce3e0;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x110ce3e0;continue;
}
0x110ce3e0=>{
// 0x110ce3e0 str x21, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x110ce3e4 str w19, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=cpu.x[19] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x110ce3e8 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x110ce3ec ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x110ce3f0 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x110ce3f4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x110ce314")),}}}
#[inline]
pub(super) unsafe fn f_111d9674(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111d9674=>{
// 0x111d9674 orr w8, w2, w1
{let a=((cpu.x[2] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x111d9678 tbnz w8, #0x1f, #0x111d96b8
if ((cpu.x[8] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x111d96b8;continue;}pc=0x111d967c;continue;
}
0x111d967c=>{
// 0x111d967c ldrb w8, [x0, #0x70]
{let address=(cpu.x[0]).wrapping_add(0x70u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d9680 cbz w8, #0x111d96b8
if (cpu.x[8] as u32 as u64) == 0 {pc=0x111d96b8;continue;}pc=0x111d9684;continue;
}
0x111d9684=>{
// 0x111d9684 ldr w8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d9688 cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d968c b.le #0x111d96b8
if cpu.condition(13){pc=0x111d96b8;continue;}pc=0x111d9690;continue;
}
0x111d9690=>{
// 0x111d9690 ldr w9, [x0, #4]
{let address=(cpu.x[0]).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x111d9694 cmp w9, w2
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[2] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d9698 b.le #0x111d96b8
if cpu.condition(13){pc=0x111d96b8;continue;}pc=0x111d969c;continue;
}
0x111d969c=>{
// 0x111d969c ldr x9, [x0, #0x28]
{let address=(cpu.x[0]).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111d96a0 madd w8, w8, w2, w1
cpu.x[8]=((((cpu.x[1] as u32 as u64))).wrapping_add((((cpu.x[8] as u32 as u64))).wrapping_mul(((cpu.x[2] as u32 as u64))))) as u32 as u64;
// 0x111d96a4 ldr x9, [x9]
{let address=(cpu.x[9]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111d96a8 ldr w8, [x9, w8, sxtw #2]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d96ac ldr w9, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x111d96b0 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d96b4 b.ge #0x111d96c0
if cpu.condition(10){pc=0x111d96c0;continue;}pc=0x111d96b8;continue;
}
0x111d96b8=>{
// 0x111d96b8 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x111d96bc ret
cpu.pc=cpu.x[30];return Ok(());
}
0x111d96c0=>{
// 0x111d96c0 mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
// 0x111d96c4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111d9674")),}}}
#[inline]
pub(super) unsafe fn f_111db894(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111db894=>{
// 0x111db894 str xzr, [x0, #0x18]
{let address=(cpu.x[0]).wrapping_add(0x18u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111db898 stp xzr, xzr, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111db89c str wzr, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111db8a0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111db894")),}}}
#[inline]
pub(super) unsafe fn f_111e4f3c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111e4f3c=>{
// 0x111e4f3c str xzr, [x0, #0x18]
{let address=(cpu.x[0]).wrapping_add(0x18u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111e4f40 str xzr, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111e4f44 str wzr, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111e4f48 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111e4f3c")),}}}
#[inline]
pub(super) unsafe fn f_111fc0c4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111fc0c4=>{
// 0x111fc0c4 stp xzr, xzr, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111fc0c8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111fc0c4")),}}}
#[inline]
pub(super) unsafe fn f_1128a854(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x1128a854=>{
// 0x1128a854 tbnz w1, #0x1f, #0x1128a874
if ((cpu.x[1] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x1128a874;continue;}pc=0x1128a858;continue;
}
0x1128a858=>{
// 0x1128a858 ldr x8, [x0, #0x38]
{let address=(cpu.x[0]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x1128a85c ldr x8, [x8, w1, uxtw #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[1] as u32 as u64) as u32 as u64)) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x1128a860 ldr x8, [x8, #0x28]
{let address=(cpu.x[8]).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x1128a864 ldrb w8, [x8, w2, sxtw]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[2] as u32 as u64) as i32 as i64 as u64)));let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x1128a868 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x1128a86c cset w0, eq
cpu.x[0]=((cpu.condition(0)) as u64) as u32 as u64;
// 0x1128a870 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x1128a874=>{
// 0x1128a874 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x1128a878 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x1128a854")),}}}
#[inline]
pub(super) unsafe fn f_1128beac(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x1128beac=>{
// 0x1128beac stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x1128beb0 str x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x1128beb4 stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x1128beb8 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x1128bebc mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x1128bec0 mov x21, x1
cpu.x[21]=(((cpu.x[1]))) as u64;
// 0x1128bec4 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x1128bec8 bl #0x1128db60
cpu.x[30]=0x1128becc;super::chunk11::f_1128db60(cpu,rt,0x1128db60)?;
pc=0x1128becc;continue;
}
0x1128becc=>{
// 0x1128becc cbz x21, #0x1128bef4
if (cpu.x[21]) == 0 {pc=0x1128bef4;continue;}pc=0x1128bed0;continue;
}
0x1128bed0=>{
// 0x1128bed0 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x1128bed4 bl #0x112ada18
cpu.x[30]=0x1128bed8;super::chunk7::f_112ada18(cpu,rt,0x112ada18)?;
// 0x1128bed8 mov x1, x21
cpu.x[1]=(((cpu.x[21]))) as u64;
// 0x1128bedc mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x1128bee0 bl #0x112aef98
cpu.x[30]=0x1128bee4;super::chunk15::f_112aef98(cpu,rt,0x112aef98)?;
// 0x1128bee4 mov w8, #0xbba1
cpu.x[8]=(((0xbba1u64))) as u32 as u64;
// 0x1128bee8 movk w8, #0xd, lsl #16
cpu.x[8]=((cpu.x[8] as u32 as u64 & !(0xffff0000u64)) | 0xd0000u64) as u32 as u64;
// 0x1128beec cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x1128bef0 b.lt #0x1128bf10
if cpu.condition(11){pc=0x1128bf10;continue;}pc=0x1128bef4;continue;
}
0x1128bef4=>{
// 0x1128bef4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x1128bef8 mov w1, #-1
cpu.x[1]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x1128befc ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x1128bf00 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x1128bf04 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x1128bf08 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x1128bf0c b #0x1128bd30
super::chunk10::f_1128bd30(cpu,rt,0x1128bd30)?;return Ok(());
}
0x1128bf10=>{
// 0x1128bf10 ldr w22, [x19, #0x14]
{let address=(cpu.x[19]).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[22]=(v0) as u32 as u64;}
// 0x1128bf14 add w8, w0, #4
{let a=((cpu.x[0] as u32 as u64));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x1128bf18 ldr w9, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x1128bf1c mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x1128bf20 add w10, w22, w8
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x1128bf24 cmp w10, w9
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x1128bf28 b.le #0x1128bf78
if cpu.condition(13){pc=0x1128bf78;continue;}pc=0x1128bf2c;continue;
}
0x1128bf2c=>{
// 0x1128bf2c add w10, w9, w9, lsr #1
{let a=((cpu.x[9] as u32 as u64));let b=(((cpu.x[9] as u32 as u64)) as u32).wrapping_shr(1) as u64;let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x1128bf30 cmp w9, #0x100
{let a=((cpu.x[9] as u32 as u64));let b=((0x100u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x1128bf34 mov w11, #0x64
cpu.x[11]=(((0x64u64))) as u32 as u64;
// 0x1128bf38 add w8, w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x1128bf3c ldr x23, [x19, #0x30]
{let address=(cpu.x[19]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x1128bf40 csel w10, w11, w10, lt
cpu.x[10]=(if cpu.condition(11){((cpu.x[11] as u32 as u64))}else{((cpu.x[10] as u32 as u64))}) as u32 as u64;
// 0x1128bf44 add w8, w8, w10
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x1128bf48 sxtw x0, w8
cpu.x[0]=((((cpu.x[8] as u32 as u64))) as i32 as i64 as u64) as u64;
// 0x1128bf4c str w8, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128bf50 bl #0x1187d520
cpu.x[30]=0x1128bf54;super::chunk8::f_1187d520(cpu,rt,0x1187d520)?;
// 0x1128bf54 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x1128bf58 mov x2, x23
cpu.x[2]=(((cpu.x[23]))) as u64;
// 0x1128bf5c mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x1128bf60 mov w4, w22
cpu.x[4]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x1128bf64 stp x0, x0, [x19, #0x30]
{let address=(cpu.x[19]).wrapping_add(0x30u64);let v0=cpu.x[0];let v1=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x1128bf68 bl #0x112aef80
cpu.x[30]=0x1128bf6c;super::chunk11::f_112aef80(cpu,rt,0x112aef80)?;
// 0x1128bf6c cbz x23, #0x1128bf78
if (cpu.x[23]) == 0 {pc=0x1128bf78;continue;}pc=0x1128bf70;continue;
}
0x1128bf70=>{
// 0x1128bf70 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x1128bf74 bl #0x1187d360
cpu.x[30]=0x1128bf78;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x1128bf78;continue;
}
0x1128bf78=>{
// 0x1128bf78 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x1128bf7c mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x1128bf80 bl #0x1128bd30
cpu.x[30]=0x1128bf84;super::chunk10::f_1128bd30(cpu,rt,0x1128bd30)?;
// 0x1128bf84 ldr x0, [x19, #0x38]
{let address=(cpu.x[19]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x1128bf88 ldr w1, [x19, #0x14]
{let address=(cpu.x[19]).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x1128bf8c mov x2, x20
cpu.x[2]=(((cpu.x[20]))) as u64;
// 0x1128bf90 mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x1128bf94 mov w4, w21
cpu.x[4]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x1128bf98 bl #0x112aef80
cpu.x[30]=0x1128bf9c;super::chunk11::f_112aef80(cpu,rt,0x112aef80)?;
// 0x1128bf9c ldr w8, [x19, #0x14]
{let address=(cpu.x[19]).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x1128bfa0 add w8, w8, w21
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[21] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x1128bfa4 str w8, [x19, #0x14]
{let address=(cpu.x[19]).wrapping_add(0x14u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128bfa8 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x1128bfac ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x1128bfb0 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x1128bfb4 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x1128bfb8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x1128beac")),}}}
#[inline]
pub(super) unsafe fn f_1128ca70(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x1128ca70=>{
// 0x1128ca70 ldrsw x8, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x1128ca74 ldr x9, [x0, #0x38]
{let address=(cpu.x[0]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x1128ca78 str wzr, [x0, #0x1c]
{let address=(cpu.x[0]).wrapping_add(0x1cu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128ca7c add x10, x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x1128ca80 add x11, x8, #2
{let a=((cpu.x[8]));let b=((0x2u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
// 0x1128ca84 add x12, x8, #3
{let a=((cpu.x[8]));let b=((0x3u64));let result=a.wrapping_add(b);cpu.x[12]=(result) as u64;}
// 0x1128ca88 str w10, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128ca8c add x14, x8, #4
{let a=((cpu.x[8]));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[14]=(result) as u64;}
// 0x1128ca90 add x15, x8, #5
{let a=((cpu.x[8]));let b=((0x5u64));let result=a.wrapping_add(b);cpu.x[15]=(result) as u64;}
// 0x1128ca94 ldrb w13, [x9, x8]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[8])));let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[13]=(v0) as u32 as u64;}
// 0x1128ca98 str w11, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[11] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128ca9c add x16, x8, #6
{let a=((cpu.x[8]));let b=((0x6u64));let result=a.wrapping_add(b);cpu.x[16]=(result) as u64;}
// 0x1128caa0 ldrb w10, [x9, x10]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[10])));let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x1128caa4 str w12, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[12] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128caa8 add x17, x8, #7
{let a=((cpu.x[8]));let b=((0x7u64));let result=a.wrapping_add(b);cpu.x[17]=(result) as u64;}
// 0x1128caac ldrb w11, [x9, x11]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[11])));let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
// 0x1128cab0 str w14, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[14] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128cab4 lsl w13, w13, #0x18
cpu.x[13]=(((((cpu.x[13] as u32 as u64))) as u32).wrapping_shl(((((0x18u64))) & 31) as u32) as u64) as u32 as u64;
// 0x1128cab8 ldrb w12, [x9, x12]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[12])));let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[12]=(v0) as u32 as u64;}
// 0x1128cabc str wzr, [x0, #0x1c]
{let address=(cpu.x[0]).wrapping_add(0x1cu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128cac0 add w8, w8, #8
{let a=((cpu.x[8] as u32 as u64));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x1128cac4 str w15, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[15] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128cac8 orr w10, w13, w10, lsl #16
{let a=((cpu.x[13] as u32 as u64));let b=(((cpu.x[10] as u32 as u64)) as u32).wrapping_shl(16) as u64;let result=a | b;cpu.x[10]=(result) as u32 as u64;}
// 0x1128cacc ldrb w14, [x9, x14]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[14])));let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[14]=(v0) as u32 as u64;}
// 0x1128cad0 str w16, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[16] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128cad4 ldrb w15, [x9, x15]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[15])));let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[15]=(v0) as u32 as u64;}
// 0x1128cad8 str w17, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[17] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128cadc lsl x14, x14, #0x18
cpu.x[14]=(((((cpu.x[14]))) as u64).wrapping_shl(((((0x18u64))) & 63) as u32) as u64) as u64;
// 0x1128cae0 orr x13, x14, x15, lsl #16
{let a=((cpu.x[14]));let b=(((cpu.x[15])) as u64).wrapping_shl(16) as u64;let result=a | b;cpu.x[13]=(result) as u64;}
// 0x1128cae4 ldrb w14, [x9, x16]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[16])));let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[14]=(v0) as u32 as u64;}
// 0x1128cae8 str w8, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128caec orr w8, w10, w11, lsl #8
{let a=((cpu.x[10] as u32 as u64));let b=(((cpu.x[11] as u32 as u64)) as u32).wrapping_shl(8) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x1128caf0 ldrb w9, [x9, x17]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[17])));let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x1128caf4 orr x10, x13, x14, lsl #8
{let a=((cpu.x[13]));let b=(((cpu.x[14])) as u64).wrapping_shl(8) as u64;let result=a | b;cpu.x[10]=(result) as u64;}
// 0x1128caf8 orr w8, w8, w12
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[12] as u32 as u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x1128cafc orr x9, x10, x9
{let a=((cpu.x[10]));let b=((cpu.x[9]));let result=a | b;cpu.x[9]=(result) as u64;}
// 0x1128cb00 orr x0, x9, x8, lsl #32
{let a=((cpu.x[9]));let b=(((cpu.x[8])) as u64).wrapping_shl(32) as u64;let result=a | b;cpu.x[0]=(result) as u64;}
// 0x1128cb04 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x1128ca70")),}}}
#[inline]
pub(super) unsafe fn f_11294e74(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x11294e74=>{
// 0x11294e74 ldr w8, [x1]
{let address=(cpu.x[1]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x11294e78 ldr w10, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x11294e7c mov w9, #-0xb505
cpu.x[9]=(((0xffffffffffff4afbu64))) as u32 as u64;
// 0x11294e80 mov w11, #0x95f7
cpu.x[11]=(((0x95f7u64))) as u32 as u64;
// 0x11294e84 sub w8, w8, w10
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x11294e88 movk w11, #0xfffe, lsl #16
cpu.x[11]=((cpu.x[11] as u32 as u64 & !(0xffff0000u64)) | 0xfffe0000u64) as u32 as u64;
// 0x11294e8c add w10, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x11294e90 cmp w10, w11
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[11] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x11294e94 b.hs #0x11294ea0
if cpu.condition(2){pc=0x11294ea0;continue;}pc=0x11294e98;continue;
}
0x11294e98=>{
// 0x11294e98 mov w0, #0x7fffffff
cpu.x[0]=(((0x7fffffffu64))) as u32 as u64;
// 0x11294e9c ret
cpu.pc=cpu.x[30];return Ok(());
}
0x11294ea0=>{
// 0x11294ea0 ldr w10, [x1, #4]
{let address=(cpu.x[1]).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x11294ea4 ldr w12, [x0, #4]
{let address=(cpu.x[0]).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[12]=(v0) as u32 as u64;}
// 0x11294ea8 mov w0, #0x7fffffff
cpu.x[0]=(((0x7fffffffu64))) as u32 as u64;
// 0x11294eac sub w10, w10, w12
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[12] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[10]=(result) as u32 as u64;}
// 0x11294eb0 add w9, w10, w9
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x11294eb4 cmp w9, w11
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[11] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x11294eb8 b.lo #0x11294ed4
if cpu.condition(3){pc=0x11294ed4;continue;}pc=0x11294ebc;continue;
}
0x11294ebc=>{
// 0x11294ebc mul w8, w8, w8
cpu.x[8]=((((cpu.x[8] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64)))) as u32 as u64;
// 0x11294ec0 mul w9, w10, w10
cpu.x[9]=((((cpu.x[10] as u32 as u64))).wrapping_mul(((cpu.x[10] as u32 as u64)))) as u32 as u64;
// 0x11294ec4 eor w10, w8, #0x7fffffff
{let a=((cpu.x[8] as u32 as u64));let b=((0x7fffffffu64));let result=a ^ b;cpu.x[10]=(result) as u32 as u64;}
// 0x11294ec8 add w8, w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x11294ecc cmp w9, w10
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x11294ed0 csel w0, w8, w0, lo
cpu.x[0]=(if cpu.condition(3){((cpu.x[8] as u32 as u64))}else{((cpu.x[0] as u32 as u64))}) as u32 as u64;
pc=0x11294ed4;continue;
}
0x11294ed4=>{
// 0x11294ed4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x11294e74")),}}}
#[inline]
pub(super) unsafe fn f_11295174(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x11295174=>{
// 0x11295174 stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x11295178 stp x24, x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x1129517c stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x11295180 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x11295184 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x11295188 ldp w22, w23, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[22]=(v0) as u32 as u64;cpu.x[23]=(v1) as u32 as u64;}
// 0x1129518c mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x11295190 add w0, w1, #0x5a
{let a=((cpu.x[1] as u32 as u64));let b=((0x5au64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u32 as u64;}
// 0x11295194 mov w20, w1
cpu.x[20]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x11295198 bl #0x11294440
cpu.x[30]=0x1129519c;super::chunk0::f_11294440(cpu,rt,0x11294440)?;
pc=0x1129519c;continue;
}
0x1129519c=>{
// 0x1129519c mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x112951a0 mov w0, w20
cpu.x[0]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x112951a4 bl #0x11294440
cpu.x[30]=0x112951a8;super::chunk0::f_11294440(cpu,rt,0x11294440)?;
pc=0x112951a8;continue;
}
0x112951a8=>{
// 0x112951a8 mul w8, w0, w23
cpu.x[8]=((((cpu.x[0] as u32 as u64))).wrapping_mul(((cpu.x[23] as u32 as u64)))) as u32 as u64;
// 0x112951ac mov w0, w20
cpu.x[0]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x112951b0 neg w8, w8
cpu.x[8]=(0u64.wrapping_sub(((cpu.x[8] as u32 as u64)))) as u32 as u64;
// 0x112951b4 madd w8, w21, w22, w8
cpu.x[8]=((((cpu.x[8] as u32 as u64))).wrapping_add((((cpu.x[21] as u32 as u64))).wrapping_mul(((cpu.x[22] as u32 as u64))))) as u32 as u64;
// 0x112951b8 ldp w24, w22, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[24]=(v0) as u32 as u64;cpu.x[22]=(v1) as u32 as u64;}
// 0x112951bc add w9, w8, #0x3ff
{let a=((cpu.x[8] as u32 as u64));let b=((0x3ffu64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x112951c0 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x112951c4 csel w8, w9, w8, lt
cpu.x[8]=(if cpu.condition(11){((cpu.x[9] as u32 as u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x112951c8 asr w21, w8, #0xa
cpu.x[21]=(((((cpu.x[8] as u32 as u64))) as i32).wrapping_shr(((((0xau64))) & 31) as u32) as u64) as u32 as u64;
// 0x112951cc bl #0x11294440
cpu.x[30]=0x112951d0;super::chunk0::f_11294440(cpu,rt,0x11294440)?;
pc=0x112951d0;continue;
}
0x112951d0=>{
// 0x112951d0 mul w23, w0, w24
cpu.x[23]=((((cpu.x[0] as u32 as u64))).wrapping_mul(((cpu.x[24] as u32 as u64)))) as u32 as u64;
// 0x112951d4 add w0, w20, #0x5a
{let a=((cpu.x[20] as u32 as u64));let b=((0x5au64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u32 as u64;}
// 0x112951d8 bl #0x11294440
cpu.x[30]=0x112951dc;super::chunk0::f_11294440(cpu,rt,0x11294440)?;
pc=0x112951dc;continue;
}
0x112951dc=>{
// 0x112951dc madd w8, w0, w22, w23
cpu.x[8]=((((cpu.x[23] as u32 as u64))).wrapping_add((((cpu.x[0] as u32 as u64))).wrapping_mul(((cpu.x[22] as u32 as u64))))) as u32 as u64;
// 0x112951e0 add w9, w8, #0x3ff
{let a=((cpu.x[8] as u32 as u64));let b=((0x3ffu64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x112951e4 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x112951e8 csel w8, w9, w8, lt
cpu.x[8]=(if cpu.condition(11){((cpu.x[9] as u32 as u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x112951ec asr w8, w8, #0xa
cpu.x[8]=(((((cpu.x[8] as u32 as u64))) as i32).wrapping_shr(((((0xau64))) & 31) as u32) as u64) as u32 as u64;
// 0x112951f0 stp w21, w8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=cpu.x[21] as u32 as u64;let v1=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x112951f4 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x112951f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x112951fc ldp x24, x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x11295200 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x11295204 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x11295174")),}}}
#[inline]
pub(super) unsafe fn f_11379154(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x11379154=>{
// 0x11379154 mov x8, x0
cpu.x[8]=(((cpu.x[0]))) as u64;
// 0x11379158 mov x0, #0x2325
cpu.x[0]=(((0x2325u64))) as u64;
// 0x1137915c movk x0, #0x8422, lsl #16
cpu.x[0]=((cpu.x[0] & !(0xffff0000u64)) | 0x84220000u64) as u64;
// 0x11379160 movk x0, #0x9ce4, lsl #32
cpu.x[0]=((cpu.x[0] & !(0xffff00000000u64)) | 0x9ce400000000u64) as u64;
// 0x11379164 movk x0, #0xcbf2, lsl #48
cpu.x[0]=((cpu.x[0] & !(0xffff000000000000u64)) | 0xcbf2000000000000u64) as u64;
// 0x11379168 cbz x1, #0x11379188
if (cpu.x[1]) == 0 {pc=0x11379188;continue;}pc=0x1137916c;continue;
}
0x1137916c=>{
// 0x1137916c mov x9, #0x1b3
cpu.x[9]=(((0x1b3u64))) as u64;
// 0x11379170 movk x9, #0x100, lsl #32
cpu.x[9]=((cpu.x[9] & !(0xffff00000000u64)) | 0x10000000000u64) as u64;
pc=0x11379174;continue;
}
0x11379174=>{
// 0x11379174 ldrb w10, [x8], #1
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;cpu.x[8]=((cpu.x[8]).wrapping_add(((0x1u64)))) as u64;}
// 0x11379178 subs x1, x1, #1
{let a=((cpu.x[1]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[1]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x1137917c eor x10, x0, x10
{let a=((cpu.x[0]));let b=((cpu.x[10]));let result=a ^ b;cpu.x[10]=(result) as u64;}
// 0x11379180 mul x0, x10, x9
cpu.x[0]=((((cpu.x[10]))).wrapping_mul(((cpu.x[9])))) as u64;
// 0x11379184 b.ne #0x11379174
if cpu.condition(1){pc=0x11379174;continue;}pc=0x11379188;continue;
}
0x11379188=>{
// 0x11379188 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x11379154")),}}}
#[inline]
pub(super) unsafe fn f_1187d520(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x1187d520=>{
// 0x1187d520 adrp x16, #0x11981000
cpu.x[16]=(0x11981000u64) as u64;
// 0x1187d524 ldr x17, [x16, #0xaa0]
{let address=(cpu.x[16]).wrapping_add(0xaa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[17]=(v0) as u64;}
// 0x1187d528 add x16, x16, #0xaa0
{let a=((cpu.x[16]));let b=((0xaa0u64));let result=a.wrapping_add(b);cpu.x[16]=(result) as u64;}
// 0x1187d52c br x17
{let target=cpu.x[17];if target>=0x1187d520 && target<0x1187d530{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
_=>return Err(format!("untranslated block {pc:#x} in 0x1187d520")),}}}
#[inline]
pub(super) unsafe fn f_30132dd0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x30132dd0=>{
// 0x30132dd0 adrp x16, #0x30142000
cpu.x[16]=(0x30142000u64) as u64;
// 0x30132dd4 ldr x17, [x16, #0x1e0]
{let address=(cpu.x[16]).wrapping_add(0x1e0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[17]=(v0) as u64;}
// 0x30132dd8 add x16, x16, #0x1e0
{let a=((cpu.x[16]));let b=((0x1e0u64));let result=a.wrapping_add(b);cpu.x[16]=(result) as u64;}
// 0x30132ddc br x17
{let target=cpu.x[17];if target>=0x30132dd0 && target<0x30132de0{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
_=>return Err(format!("untranslated block {pc:#x} in 0x30132dd0")),}}}
