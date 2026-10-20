#![allow(unused_parens,unreachable_code,unused_variables,unused_mut)]
use crate::{cpu::Cpu,runtime::Runtime};
use super::helpers::*;
#[inline]
pub(super) unsafe fn f_10834730(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10834730=>{
// 0x10834730 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10834730")),}}}
#[inline]
pub(super) unsafe fn f_10834758(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10834758=>{
// 0x10834758 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10834758")),}}}
#[inline]
pub(super) unsafe fn f_1085aa6c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x1085aa6c=>{
// 0x1085aa6c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x1085aa6c")),}}}
#[inline]
pub(super) unsafe fn f_10d19304(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d19304=>{
// 0x10d19304 ldr w9, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10d19308 cmp w9, #0x14
{let a=((cpu.x[9] as u32 as u64));let b=((0x14u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d1930c b.hi #0x10d1933c
if cpu.condition(8){pc=0x10d1933c;continue;}pc=0x10d19310;continue;
}
0x10d19310=>{
// 0x10d19310 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10d19314 mov w11, #0x2102
cpu.x[11]=(((0x2102u64))) as u32 as u64;
// 0x10d19318 lsl w10, w8, w9
cpu.x[10]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((cpu.x[9] as u32 as u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10d1931c movk w11, #0x11, lsl #16
cpu.x[11]=((cpu.x[11] as u32 as u64 & !(0xffff0000u64)) | 0x110000u64) as u32 as u64;
// 0x10d19320 tst w10, w11
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[11] as u32 as u64));let result=a & b;cpu.logical_flags(result,32);}
// 0x10d19324 b.eq #0x10d19330
if cpu.condition(0){pc=0x10d19330;continue;}pc=0x10d19328;continue;
}
0x10d19328=>{
// 0x10d19328 mov w0, w8
cpu.x[0]=(((cpu.x[8] as u32 as u64))) as u32 as u64;
// 0x10d1932c ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10d19330=>{
// 0x10d19330 cmp w9, #7
{let a=((cpu.x[9] as u32 as u64));let b=((0x7u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d19334 b.eq #0x10d19348
if cpu.condition(0){pc=0x10d19348;continue;}pc=0x10d19338;continue;
}
0x10d19338=>{
// 0x10d19338 cmp w9, #0xa
{let a=((cpu.x[9] as u32 as u64));let b=((0xau64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
pc=0x10d1933c;continue;
}
0x10d1933c=>{
// 0x10d1933c mov w8, wzr
cpu.x[8]=(((0u64))) as u32 as u64;
// 0x10d19340 mov w0, w8
cpu.x[0]=(((cpu.x[8] as u32 as u64))) as u32 as u64;
// 0x10d19344 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10d19348=>{
// 0x10d19348 stp x29, x30, [sp, #-0x10]!
{let address=(cpu.sp).wrapping_add(0xfffffffffffffff0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d1934c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d19350 ldr x0, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d19354 bl #0x10e16208
cpu.x[30]=0x10d19358;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
// 0x10d19358 ldrb w8, [x0, #0x501]
{let address=(cpu.x[0]).wrapping_add(0x501u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d1935c cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d19360 cset w0, ne
cpu.x[0]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x10d19364 ldp x29, x30, [sp], #0x10
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x10u64)))) as u64;}
// 0x10d19368 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d19304")),}}}
#[inline]
pub(super) unsafe fn f_10d19abc(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d19abc=>{
// 0x10d19abc mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10d19ac0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d19abc")),}}}
#[inline]
pub(super) unsafe fn f_10d19ae8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d19ae8=>{
// 0x10d19ae8 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10d19aec ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d19ae8")),}}}
#[inline]
pub(super) unsafe fn f_10d22ea4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d22ea4=>{
// 0x10d22ea4 sub sp, sp, #0x70
{let a=((cpu.sp));let b=((0x70u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x10d22ea8 stp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d22eac stp x28, x27, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[28];let v1=cpu.x[27];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d22eb0 stp x26, x25, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[26];let v1=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d22eb4 stp x24, x23, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d22eb8 stp x22, x21, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d22ebc stp x20, x19, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d22ec0 add x29, sp, #0x10
{let a=((cpu.sp));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x10d22ec4 mov x23, x1
cpu.x[23]=(((cpu.x[1]))) as u64;
// 0x10d22ec8 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10d22ecc add x1, x0, #0x18
{let a=((cpu.x[0]));let b=((0x18u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x10d22ed0 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d22ed4 mov w19, w3
cpu.x[19]=(((cpu.x[3] as u32 as u64))) as u32 as u64;
// 0x10d22ed8 str w2, [sp, #4]
{let address=(cpu.sp).wrapping_add(0x4u64);let v0=cpu.x[2] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d22edc bl #0x111dd3d0
cpu.x[30]=0x10d22ee0;super::chunk5::f_111dd3d0(cpu,rt,0x111dd3d0)?;
pc=0x10d22ee0;continue;
}
0x10d22ee0=>{
// 0x10d22ee0 add x1, x21, #0x20
{let a=((cpu.x[21]));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x10d22ee4 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d22ee8 bl #0x111dd3d0
cpu.x[30]=0x10d22eec;super::chunk5::f_111dd3d0(cpu,rt,0x111dd3d0)?;
pc=0x10d22eec;continue;
}
0x10d22eec=>{
// 0x10d22eec add x1, x21, #0x28
{let a=((cpu.x[21]));let b=((0x28u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x10d22ef0 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d22ef4 bl #0x111dd3d0
cpu.x[30]=0x10d22ef8;super::chunk5::f_111dd3d0(cpu,rt,0x111dd3d0)?;
pc=0x10d22ef8;continue;
}
0x10d22ef8=>{
// 0x10d22ef8 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d22efc add x1, x21, #0x78
{let a=((cpu.x[21]));let b=((0x78u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x10d22f00 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d22f04 ldr x8, [x8, #0x28]
{let address=(cpu.x[8]).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d22f08 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d22f0c;super::dispatch(cpu,rt,target)?;}
pc=0x10d22f0c;continue;
}
0x10d22f0c=>{
// 0x10d22f0c ldr x1, [x21, #0x160]
{let address=(cpu.x[21]).wrapping_add(0x160u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10d22f10 ldr x3, [x21, #0x10]
{let address=(cpu.x[21]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[3]=(v0) as u64;}
// 0x10d22f14 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d22f18 mov w2, #0x36
cpu.x[2]=(((0x36u64))) as u32 as u64;
// 0x10d22f1c bl #0x111dd77c
cpu.x[30]=0x10d22f20;super::chunk15::f_111dd77c(cpu,rt,0x111dd77c)?;
pc=0x10d22f20;continue;
}
0x10d22f20=>{
// 0x10d22f20 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d22f24 ldr w1, [x21, #0x88]
{let address=(cpu.x[21]).wrapping_add(0x88u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d22f28 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d22f2c ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d22f30 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d22f34;super::dispatch(cpu,rt,target)?;}
pc=0x10d22f34;continue;
}
0x10d22f34=>{
// 0x10d22f34 tbnz w19, #0, #0x10d23490
if ((cpu.x[19] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10d23490;continue;}pc=0x10d22f38;continue;
}
0x10d22f38=>{
// 0x10d22f38 ldr w8, [sp, #4]
{let address=(cpu.sp).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d22f3c tbnz w8, #0, #0x10d22f54
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10d22f54;continue;}pc=0x10d22f40;continue;
}
0x10d22f40=>{
// 0x10d22f40 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d22f44 ldr w1, [x21, #0x158]
{let address=(cpu.x[21]).wrapping_add(0x158u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d22f48 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d22f4c ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d22f50 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d22f54;super::dispatch(cpu,rt,target)?;}
pc=0x10d22f54;continue;
}
0x10d22f54=>{
// 0x10d22f54 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d22f58 ldr w1, [x21, #0x8c]
{let address=(cpu.x[21]).wrapping_add(0x8cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d22f5c mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d22f60 str w19, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=cpu.x[19] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d22f64 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d22f68 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d22f6c;super::dispatch(cpu,rt,target)?;}
pc=0x10d22f6c;continue;
}
0x10d22f6c=>{
// 0x10d22f6c ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d22f70 ldr w1, [x21, #0x90]
{let address=(cpu.x[21]).wrapping_add(0x90u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d22f74 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d22f78 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d22f7c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d22f80;super::dispatch(cpu,rt,target)?;}
pc=0x10d22f80;continue;
}
0x10d22f80=>{
// 0x10d22f80 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d22f84 ldr w1, [x21, #0x94]
{let address=(cpu.x[21]).wrapping_add(0x94u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d22f88 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d22f8c ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d22f90 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d22f94;super::dispatch(cpu,rt,target)?;}
pc=0x10d22f94;continue;
}
0x10d22f94=>{
// 0x10d22f94 ldr w8, [sp, #4]
{let address=(cpu.sp).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d22f98 tbnz w8, #0, #0x10d22fb0
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10d22fb0;continue;}pc=0x10d22f9c;continue;
}
0x10d22f9c=>{
// 0x10d22f9c ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d22fa0 ldr w1, [x21, #0x98]
{let address=(cpu.x[21]).wrapping_add(0x98u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d22fa4 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d22fa8 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d22fac blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d22fb0;super::dispatch(cpu,rt,target)?;}
pc=0x10d22fb0;continue;
}
0x10d22fb0=>{
// 0x10d22fb0 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d22fb4 ldr w1, [x21, #0x3c]
{let address=(cpu.x[21]).wrapping_add(0x3cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d22fb8 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d22fbc ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d22fc0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d22fc4;super::dispatch(cpu,rt,target)?;}
pc=0x10d22fc4;continue;
}
0x10d22fc4=>{
// 0x10d22fc4 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d22fc8 ldr w1, [x21, #0x40]
{let address=(cpu.x[21]).wrapping_add(0x40u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d22fcc mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d22fd0 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d22fd4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d22fd8;super::dispatch(cpu,rt,target)?;}
pc=0x10d22fd8;continue;
}
0x10d22fd8=>{
// 0x10d22fd8 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d22fdc ldr w1, [x21, #0x44]
{let address=(cpu.x[21]).wrapping_add(0x44u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d22fe0 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d22fe4 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d22fe8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d22fec;super::dispatch(cpu,rt,target)?;}
pc=0x10d22fec;continue;
}
0x10d22fec=>{
// 0x10d22fec ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d22ff0 ldr w1, [x21, #0x48]
{let address=(cpu.x[21]).wrapping_add(0x48u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d22ff4 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d22ff8 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d22ffc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d23000;super::dispatch(cpu,rt,target)?;}
pc=0x10d23000;continue;
}
0x10d23000=>{
// 0x10d23000 ldr x1, [x21, #0x50]
{let address=(cpu.x[21]).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10d23004 ldr x3, [x21, #0x10]
{let address=(cpu.x[21]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[3]=(v0) as u64;}
// 0x10d23008 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d2300c mov w2, #0x36
cpu.x[2]=(((0x36u64))) as u32 as u64;
// 0x10d23010 bl #0x111dd77c
cpu.x[30]=0x10d23014;super::chunk15::f_111dd77c(cpu,rt,0x111dd77c)?;
pc=0x10d23014;continue;
}
0x10d23014=>{
// 0x10d23014 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23018 ldr w1, [x21, #0x5c]
{let address=(cpu.x[21]).wrapping_add(0x5cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d2301c mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d23020 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23024 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d23028;super::dispatch(cpu,rt,target)?;}
pc=0x10d23028;continue;
}
0x10d23028=>{
// 0x10d23028 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d2302c ldr w1, [x21, #0x60]
{let address=(cpu.x[21]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d23030 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d23034 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23038 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d2303c;super::dispatch(cpu,rt,target)?;}
pc=0x10d2303c;continue;
}
0x10d2303c=>{
// 0x10d2303c ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23040 ldr w1, [x21, #0x64]
{let address=(cpu.x[21]).wrapping_add(0x64u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d23044 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d23048 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d2304c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d23050;super::dispatch(cpu,rt,target)?;}
pc=0x10d23050;continue;
}
0x10d23050=>{
// 0x10d23050 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23054 ldr w1, [x21, #0x68]
{let address=(cpu.x[21]).wrapping_add(0x68u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d23058 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d2305c ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23060 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d23064;super::dispatch(cpu,rt,target)?;}
pc=0x10d23064;continue;
}
0x10d23064=>{
// 0x10d23064 ldr x1, [x21, #0x70]
{let address=(cpu.x[21]).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10d23068 ldr x3, [x21, #0x10]
{let address=(cpu.x[21]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[3]=(v0) as u64;}
// 0x10d2306c mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d23070 mov w2, #0x36
cpu.x[2]=(((0x36u64))) as u32 as u64;
// 0x10d23074 bl #0x111dd77c
cpu.x[30]=0x10d23078;super::chunk15::f_111dd77c(cpu,rt,0x111dd77c)?;
pc=0x10d23078;continue;
}
0x10d23078=>{
// 0x10d23078 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d2307c mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d23080 mov w1, #9
cpu.x[1]=(((0x9u64))) as u32 as u64;
// 0x10d23084 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23088 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d2308c;super::dispatch(cpu,rt,target)?;}
pc=0x10d2308c;continue;
}
0x10d2308c=>{
// 0x10d2308c mov x8, xzr
cpu.x[8]=(((0u64))) as u64;
// 0x10d23090 mov x19, x21
cpu.x[19]=(((cpu.x[21]))) as u64;
// 0x10d23094 mov x26, x23
cpu.x[26]=(((cpu.x[23]))) as u64;
// 0x10d23098 b #0x10d230b0
pc=0x10d230b0;continue;
}
0x10d2309c=>{
// 0x10d2309c ldr x8, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d230a0 mov x21, x19
cpu.x[21]=(((cpu.x[19]))) as u64;
// 0x10d230a4 add x8, x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d230a8 cmp x8, #9
{let a=((cpu.x[8]));let b=((0x9u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d230ac b.eq #0x10d233e4
if cpu.condition(0){pc=0x10d233e4;continue;}pc=0x10d230b0;continue;
}
0x10d230b0=>{
// 0x10d230b0 str x8, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d230b4 add x8, x21, x8, lsl #3
{let a=((cpu.x[21]));let b=(((cpu.x[8])) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d230b8 ldr x28, [x8, #0xa0]
{let address=(cpu.x[8]).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[28]=(v0) as u64;}
// 0x10d230bc ldr w8, [sp, #4]
{let address=(cpu.sp).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d230c0 ldr w22, [x28, #0xc]
{let address=(cpu.x[28]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[22]=(v0) as u32 as u64;}
// 0x10d230c4 tbz w8, #0, #0x10d230e4
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d230e4;continue;}pc=0x10d230c8;continue;
}
0x10d230c8=>{
// 0x10d230c8 ldr x0, [x21, #0x10]
{let address=(cpu.x[21]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d230cc cbz x0, #0x10d23278
if (cpu.x[0]) == 0 {pc=0x10d23278;continue;}pc=0x10d230d0;continue;
}
0x10d230d0=>{
// 0x10d230d0 bl #0x10e16208
cpu.x[30]=0x10d230d4;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
pc=0x10d230d4;continue;
}
0x10d230d4=>{
// 0x10d230d4 ldrb w8, [x0, #0x43b]
{let address=(cpu.x[0]).wrapping_add(0x43bu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d230d8 mov w24, w22
cpu.x[24]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10d230dc cbnz w8, #0x10d23290
if (cpu.x[8] as u32 as u64) != 0 {pc=0x10d23290;continue;}pc=0x10d230e0;continue;
}
0x10d230e0=>{
// 0x10d230e0 b #0x10d230e8
pc=0x10d230e8;continue;
}
0x10d230e4=>{
// 0x10d230e4 mov w24, w22
cpu.x[24]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
pc=0x10d230e8;continue;
}
0x10d230e8=>{
// 0x10d230e8 ldr x8, [x26]
{let address=(cpu.x[26]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d230ec mov x0, x26
cpu.x[0]=(((cpu.x[26]))) as u64;
// 0x10d230f0 mov w1, w24
cpu.x[1]=(((cpu.x[24] as u32 as u64))) as u32 as u64;
// 0x10d230f4 mov x23, x26
cpu.x[23]=(((cpu.x[26]))) as u64;
// 0x10d230f8 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d230fc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d23100;super::dispatch(cpu,rt,target)?;}
pc=0x10d23100;continue;
}
0x10d23100=>{
// 0x10d23100 cmp w22, #1
{let a=((cpu.x[22] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d23104 b.lt #0x10d2309c
if cpu.condition(11){pc=0x10d2309c;continue;}pc=0x10d23108;continue;
}
0x10d23108=>{
// 0x10d23108 ldr x9, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d2310c and x8, x9, #0xe
{let a=((cpu.x[9]));let b=((0xeu64));let result=a & b;cpu.x[8]=(result) as u64;}
// 0x10d23110 cmp x9, #8
{let a=((cpu.x[9]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d23114 ccmp x8, #2, #4, ne
if cpu.condition(1){{let a=((cpu.x[8]));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}}else{cpu.nzcv=4u64 << 28;}
// 0x10d23118 ldr w8, [sp, #4]
{let address=(cpu.sp).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d2311c cset w25, eq
cpu.x[25]=((cpu.condition(0)) as u64) as u32 as u64;
// 0x10d23120 tbz w8, #0, #0x10d23248
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d23248;continue;}pc=0x10d23124;continue;
}
0x10d23124=>{
// 0x10d23124 mov x27, xzr
cpu.x[27]=(((0u64))) as u64;
// 0x10d23128 b #0x10d23154
pc=0x10d23154;continue;
}
0x10d2312c=>{
// 0x10d2312c ldr x8, [x28]
{let address=(cpu.x[28]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23130 mov x1, x26
cpu.x[1]=(((cpu.x[26]))) as u64;
// 0x10d23134 ldr x0, [x8, x27, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[27])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d23138 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d2313c ldr x8, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23140 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d23144;super::dispatch(cpu,rt,target)?;}
pc=0x10d23144;continue;
}
0x10d23144=>{
// 0x10d23144 add x27, x27, #1
{let a=((cpu.x[27]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[27]=(result) as u64;}
// 0x10d23148 mov x23, x26
cpu.x[23]=(((cpu.x[26]))) as u64;
// 0x10d2314c cmp x27, x22
{let a=((cpu.x[27]));let b=((cpu.x[22]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d23150 b.eq #0x10d2309c
if cpu.condition(0){pc=0x10d2309c;continue;}pc=0x10d23154;continue;
}
0x10d23154=>{
// 0x10d23154 ldr x0, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d23158 cbz x0, #0x10d2316c
if (cpu.x[0]) == 0 {pc=0x10d2316c;continue;}pc=0x10d2315c;continue;
}
0x10d2315c=>{
// 0x10d2315c bl #0x10e16208
cpu.x[30]=0x10d23160;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
pc=0x10d23160;continue;
}
0x10d23160=>{
// 0x10d23160 ldrb w8, [x0, #0x43b]
{let address=(cpu.x[0]).wrapping_add(0x43bu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d23164 cbnz w8, #0x10d23180
if (cpu.x[8] as u32 as u64) != 0 {pc=0x10d23180;continue;}pc=0x10d23168;continue;
}
0x10d23168=>{
// 0x10d23168 b #0x10d2312c
pc=0x10d2312c;continue;
}
0x10d2316c=>{
// 0x10d2316c ldr x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d23170 bl #0x110cbf10
cpu.x[30]=0x10d23174;super::chunk14::f_110cbf10(cpu,rt,0x110cbf10)?;
// 0x10d23174 bl #0x10e16208
cpu.x[30]=0x10d23178;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
// 0x10d23178 ldrb w8, [x0, #0x43b]
{let address=(cpu.x[0]).wrapping_add(0x43bu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d2317c cbz w8, #0x10d2312c
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10d2312c;continue;}pc=0x10d23180;continue;
}
0x10d23180=>{
// 0x10d23180 tbnz w25, #0, #0x10d23144
if ((cpu.x[25] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10d23144;continue;}pc=0x10d23184;continue;
}
0x10d23184=>{
// 0x10d23184 ldr x8, [x28]
{let address=(cpu.x[28]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23188 ldr x8, [x8, x27, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[27])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d2318c ldr x24, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[24]=(v0) as u64;}
// 0x10d23190 ldr x8, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23194 cmp x8, #5
{let a=((cpu.x[8]));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d23198 b.ne #0x10d231b4
if cpu.condition(1){pc=0x10d231b4;continue;}pc=0x10d2319c;continue;
}
0x10d2319c=>{
// 0x10d2319c ldr x23, [x19, #0xc8]
{let address=(cpu.x[19]).wrapping_add(0xc8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10d231a0 cbz x23, #0x10d231d4
if (cpu.x[23]) == 0 {pc=0x10d231d4;continue;}pc=0x10d231a4;continue;
}
0x10d231a4=>{
// 0x10d231a4 ldr w8, [x23, #0xc]
{let address=(cpu.x[23]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d231a8 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d231ac b.ge #0x10d231f0
if cpu.condition(10){pc=0x10d231f0;continue;}pc=0x10d231b0;continue;
}
0x10d231b0=>{
// 0x10d231b0 b #0x10d23144
pc=0x10d23144;continue;
}
0x10d231b4=>{
// 0x10d231b4 ldr x8, [x24]
{let address=(cpu.x[24]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d231b8 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10d231bc ldr x8, [x8, #0xa0]
{let address=(cpu.x[8]).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d231c0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d231c4;super::dispatch(cpu,rt,target)?;}
pc=0x10d231c4;continue;
}
0x10d231c4=>{
// 0x10d231c4 tbz w0, #0, #0x10d2312c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d2312c;continue;}pc=0x10d231c8;continue;
}
0x10d231c8=>{
// 0x10d231c8 ldrb w8, [x24, #0x61]
{let address=(cpu.x[24]).wrapping_add(0x61u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d231cc cbnz w8, #0x10d2312c
if (cpu.x[8] as u32 as u64) != 0 {pc=0x10d2312c;continue;}pc=0x10d231d0;continue;
}
0x10d231d0=>{
// 0x10d231d0 b #0x10d23144
pc=0x10d23144;continue;
}
0x10d231d4=>{
// 0x10d231d4 adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x10d231d8 add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d231dc mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d231e0 bl #0x1128df18
cpu.x[30]=0x10d231e4;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10d231e4 ldr w8, [x23, #0xc]
{let address=(cpu.x[23]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d231e8 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d231ec b.lt #0x10d23144
if cpu.condition(11){pc=0x10d23144;continue;}pc=0x10d231f0;continue;
}
0x10d231f0=>{
// 0x10d231f0 lsl x20, x8, #3
cpu.x[20]=(((((cpu.x[8]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10d231f4 mov x21, xzr
cpu.x[21]=(((0u64))) as u64;
// 0x10d231f8 b #0x10d23208
pc=0x10d23208;continue;
}
0x10d231fc=>{
// 0x10d231fc add x21, x21, #8
{let a=((cpu.x[21]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[21]=(result) as u64;}
// 0x10d23200 cmp x20, x21
{let a=((cpu.x[20]));let b=((cpu.x[21]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d23204 b.eq #0x10d23144
if cpu.condition(0){pc=0x10d23144;continue;}pc=0x10d23208;continue;
}
0x10d23208=>{
// 0x10d23208 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d2320c ldr x8, [x8, x21]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[21])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23210 cbz x8, #0x10d231fc
if (cpu.x[8]) == 0 {pc=0x10d231fc;continue;}pc=0x10d23214;continue;
}
0x10d23214=>{
// 0x10d23214 ldr x0, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d23218 cbz x0, #0x10d231fc
if (cpu.x[0]) == 0 {pc=0x10d231fc;continue;}pc=0x10d2321c;continue;
}
0x10d2321c=>{
// 0x10d2321c mov x1, x24
cpu.x[1]=(((cpu.x[24]))) as u64;
// 0x10d23220 bl #0x10e0bc20
cpu.x[30]=0x10d23224;super::chunk1::f_10e0bc20(cpu,rt,0x10e0bc20)?;
pc=0x10d23224;continue;
}
0x10d23224=>{
// 0x10d23224 tbz w0, #0, #0x10d231fc
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d231fc;continue;}pc=0x10d23228;continue;
}
0x10d23228=>{
// 0x10d23228 mov x8, #0x7fffffff8
cpu.x[8]=(((0x7fffffff8u64))) as u64;
// 0x10d2322c cmp x21, x8
{let a=((cpu.x[21]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d23230 b.eq #0x10d23144
if cpu.condition(0){pc=0x10d23144;continue;}pc=0x10d23234;continue;
}
0x10d23234=>{
// 0x10d23234 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23238 ldr x8, [x8, x21]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[21])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d2323c ldr w8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d23240 cbnz w8, #0x10d2312c
if (cpu.x[8] as u32 as u64) != 0 {pc=0x10d2312c;continue;}pc=0x10d23244;continue;
}
0x10d23244=>{
// 0x10d23244 b #0x10d23144
pc=0x10d23144;continue;
}
0x10d23248=>{
// 0x10d23248 lsl x20, x22, #3
cpu.x[20]=(((((cpu.x[22]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10d2324c mov x21, xzr
cpu.x[21]=(((0u64))) as u64;
pc=0x10d23250;continue;
}
0x10d23250=>{
// 0x10d23250 ldr x8, [x28]
{let address=(cpu.x[28]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23254 mov x1, x23
cpu.x[1]=(((cpu.x[23]))) as u64;
// 0x10d23258 ldr x0, [x8, x21]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[21])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d2325c ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23260 ldr x8, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23264 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d23268;super::dispatch(cpu,rt,target)?;}
// 0x10d23268 add x21, x21, #8
{let a=((cpu.x[21]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[21]=(result) as u64;}
// 0x10d2326c cmp x20, x21
{let a=((cpu.x[20]));let b=((cpu.x[21]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d23270 b.ne #0x10d23250
if cpu.condition(1){pc=0x10d23250;continue;}pc=0x10d23274;continue;
}
0x10d23274=>{
// 0x10d23274 b #0x10d2309c
pc=0x10d2309c;continue;
}
0x10d23278=>{
// 0x10d23278 ldr x0, [x21, #8]
{let address=(cpu.x[21]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d2327c bl #0x110cbf10
cpu.x[30]=0x10d23280;super::chunk14::f_110cbf10(cpu,rt,0x110cbf10)?;
// 0x10d23280 bl #0x10e16208
cpu.x[30]=0x10d23284;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
// 0x10d23284 ldrb w8, [x0, #0x43b]
{let address=(cpu.x[0]).wrapping_add(0x43bu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d23288 mov w24, w22
cpu.x[24]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10d2328c cbz w8, #0x10d230e8
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10d230e8;continue;}pc=0x10d23290;continue;
}
0x10d23290=>{
// 0x10d23290 ldr x8, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23294 mov w24, wzr
cpu.x[24]=(((0u64))) as u32 as u64;
// 0x10d23298 cmp x8, #8
{let a=((cpu.x[8]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d2329c b.eq #0x10d230e8
if cpu.condition(0){pc=0x10d230e8;continue;}pc=0x10d232a0;continue;
}
0x10d232a0=>{
// 0x10d232a0 and x8, x8, #0xe
{let a=((cpu.x[8]));let b=((0xeu64));let result=a & b;cpu.x[8]=(result) as u64;}
// 0x10d232a4 cmp x8, #2
{let a=((cpu.x[8]));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d232a8 b.eq #0x10d230e8
if cpu.condition(0){pc=0x10d230e8;continue;}pc=0x10d232ac;continue;
}
0x10d232ac=>{
// 0x10d232ac ldr x8, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d232b0 cmp x8, #5
{let a=((cpu.x[8]));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d232b4 b.ne #0x10d23378
if cpu.condition(1){pc=0x10d23378;continue;}pc=0x10d232b8;continue;
}
0x10d232b8=>{
// 0x10d232b8 cmp w22, #1
{let a=((cpu.x[22] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d232bc b.lt #0x10d233dc
if cpu.condition(11){pc=0x10d233dc;continue;}pc=0x10d232c0;continue;
}
0x10d232c0=>{
// 0x10d232c0 mov x27, xzr
cpu.x[27]=(((0u64))) as u64;
// 0x10d232c4 mov w24, wzr
cpu.x[24]=(((0u64))) as u32 as u64;
// 0x10d232c8 b #0x10d232dc
pc=0x10d232dc;continue;
}
0x10d232cc=>{
// 0x10d232cc mov x21, x19
cpu.x[21]=(((cpu.x[19]))) as u64;
pc=0x10d232d0;continue;
}
0x10d232d0=>{
// 0x10d232d0 add x27, x27, #1
{let a=((cpu.x[27]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[27]=(result) as u64;}
// 0x10d232d4 cmp x27, x22
{let a=((cpu.x[27]));let b=((cpu.x[22]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d232d8 b.eq #0x10d230e8
if cpu.condition(0){pc=0x10d230e8;continue;}pc=0x10d232dc;continue;
}
0x10d232dc=>{
// 0x10d232dc ldr x8, [x28]
{let address=(cpu.x[28]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d232e0 ldr x23, [x21, #0xc8]
{let address=(cpu.x[21]).wrapping_add(0xc8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10d232e4 ldr x8, [x8, x27, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[27])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d232e8 ldr x25, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[25]=(v0) as u64;}
// 0x10d232ec cbz x23, #0x10d23300
if (cpu.x[23]) == 0 {pc=0x10d23300;continue;}pc=0x10d232f0;continue;
}
0x10d232f0=>{
// 0x10d232f0 ldr w8, [x23, #0xc]
{let address=(cpu.x[23]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d232f4 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d232f8 b.ge #0x10d2331c
if cpu.condition(10){pc=0x10d2331c;continue;}pc=0x10d232fc;continue;
}
0x10d232fc=>{
// 0x10d232fc b #0x10d232d0
pc=0x10d232d0;continue;
}
0x10d23300=>{
// 0x10d23300 adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x10d23304 add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d23308 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d2330c bl #0x1128df18
cpu.x[30]=0x10d23310;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10d23310 ldr w8, [x23, #0xc]
{let address=(cpu.x[23]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d23314 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d23318 b.lt #0x10d232d0
if cpu.condition(11){pc=0x10d232d0;continue;}pc=0x10d2331c;continue;
}
0x10d2331c=>{
// 0x10d2331c lsl x20, x8, #3
cpu.x[20]=(((((cpu.x[8]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10d23320 mov x21, xzr
cpu.x[21]=(((0u64))) as u64;
// 0x10d23324 b #0x10d23334
pc=0x10d23334;continue;
}
0x10d23328=>{
// 0x10d23328 add x21, x21, #8
{let a=((cpu.x[21]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[21]=(result) as u64;}
// 0x10d2332c cmp x20, x21
{let a=((cpu.x[20]));let b=((cpu.x[21]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d23330 b.eq #0x10d232cc
if cpu.condition(0){pc=0x10d232cc;continue;}pc=0x10d23334;continue;
}
0x10d23334=>{
// 0x10d23334 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23338 ldr x8, [x8, x21]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[21])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d2333c cbz x8, #0x10d23328
if (cpu.x[8]) == 0 {pc=0x10d23328;continue;}pc=0x10d23340;continue;
}
0x10d23340=>{
// 0x10d23340 ldr x0, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d23344 cbz x0, #0x10d23328
if (cpu.x[0]) == 0 {pc=0x10d23328;continue;}pc=0x10d23348;continue;
}
0x10d23348=>{
// 0x10d23348 mov x1, x25
cpu.x[1]=(((cpu.x[25]))) as u64;
// 0x10d2334c bl #0x10e0bc20
cpu.x[30]=0x10d23350;super::chunk1::f_10e0bc20(cpu,rt,0x10e0bc20)?;
pc=0x10d23350;continue;
}
0x10d23350=>{
// 0x10d23350 tbz w0, #0, #0x10d23328
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d23328;continue;}pc=0x10d23354;continue;
}
0x10d23354=>{
// 0x10d23354 mov x8, #0x7fffffff8
cpu.x[8]=(((0x7fffffff8u64))) as u64;
// 0x10d23358 cmp x21, x8
{let a=((cpu.x[21]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d2335c b.eq #0x10d232cc
if cpu.condition(0){pc=0x10d232cc;continue;}pc=0x10d23360;continue;
}
0x10d23360=>{
// 0x10d23360 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23364 ldr x8, [x8, x21]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[21])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23368 ldr w8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d2336c cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d23370 cinc w24, w24, gt
cpu.x[24]=(if cpu.condition(12){(((cpu.x[24] as u32 as u64))).wrapping_add(1)}else{((cpu.x[24] as u32 as u64))}) as u32 as u64;
// 0x10d23374 b #0x10d232cc
pc=0x10d232cc;continue;
}
0x10d23378=>{
// 0x10d23378 cmp w22, #1
{let a=((cpu.x[22] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d2337c b.lt #0x10d233dc
if cpu.condition(11){pc=0x10d233dc;continue;}pc=0x10d23380;continue;
}
0x10d23380=>{
// 0x10d23380 lsl x27, x22, #3
cpu.x[27]=(((((cpu.x[22]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10d23384 mov x23, xzr
cpu.x[23]=(((0u64))) as u64;
// 0x10d23388 mov w24, wzr
cpu.x[24]=(((0u64))) as u32 as u64;
// 0x10d2338c b #0x10d233a4
pc=0x10d233a4;continue;
}
0x10d23390=>{
// 0x10d23390 ldrb w8, [x25, #0x61]
{let address=(cpu.x[25]).wrapping_add(0x61u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d23394 add x23, x23, #8
{let a=((cpu.x[23]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[23]=(result) as u64;}
// 0x10d23398 add w24, w24, w8
{let a=((cpu.x[24] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_add(b);cpu.x[24]=(result) as u32 as u64;}
// 0x10d2339c cmp x27, x23
{let a=((cpu.x[27]));let b=((cpu.x[23]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d233a0 b.eq #0x10d230e8
if cpu.condition(0){pc=0x10d230e8;continue;}pc=0x10d233a4;continue;
}
0x10d233a4=>{
// 0x10d233a4 ldr x8, [x28]
{let address=(cpu.x[28]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d233a8 ldr x8, [x8, x23]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[23])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d233ac ldr x25, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[25]=(v0) as u64;}
// 0x10d233b0 ldr x8, [x25]
{let address=(cpu.x[25]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d233b4 mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10d233b8 ldr x8, [x8, #0xa0]
{let address=(cpu.x[8]).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d233bc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d233c0;super::dispatch(cpu,rt,target)?;}
pc=0x10d233c0;continue;
}
0x10d233c0=>{
// 0x10d233c0 tbnz w0, #0, #0x10d23390
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10d23390;continue;}pc=0x10d233c4;continue;
}
0x10d233c4=>{
// 0x10d233c4 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10d233c8 add x23, x23, #8
{let a=((cpu.x[23]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[23]=(result) as u64;}
// 0x10d233cc add w24, w24, w8
{let a=((cpu.x[24] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_add(b);cpu.x[24]=(result) as u32 as u64;}
// 0x10d233d0 cmp x27, x23
{let a=((cpu.x[27]));let b=((cpu.x[23]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d233d4 b.ne #0x10d233a4
if cpu.condition(1){pc=0x10d233a4;continue;}pc=0x10d233d8;continue;
}
0x10d233d8=>{
// 0x10d233d8 b #0x10d230e8
pc=0x10d230e8;continue;
}
0x10d233dc=>{
// 0x10d233dc mov w24, wzr
cpu.x[24]=(((0u64))) as u32 as u64;
// 0x10d233e0 b #0x10d230e8
pc=0x10d230e8;continue;
}
0x10d233e4=>{
// 0x10d233e4 ldr w8, [sp, #4]
{let address=(cpu.sp).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d233e8 tbnz w8, #0, #0x10d23478
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10d23478;continue;}pc=0x10d233ec;continue;
}
0x10d233ec=>{
// 0x10d233ec ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d233f0 ldr w1, [x21, #0xf4]
{let address=(cpu.x[21]).wrapping_add(0xf4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d233f4 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d233f8 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d233fc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d23400;super::dispatch(cpu,rt,target)?;}
// 0x10d23400 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23404 ldr w1, [x21, #0xf8]
{let address=(cpu.x[21]).wrapping_add(0xf8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d23408 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d2340c ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23410 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d23414;super::dispatch(cpu,rt,target)?;}
// 0x10d23414 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23418 ldr w1, [x21, #0xec]
{let address=(cpu.x[21]).wrapping_add(0xecu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d2341c mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d23420 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23424 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d23428;super::dispatch(cpu,rt,target)?;}
// 0x10d23428 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d2342c ldr w1, [x21, #0xf0]
{let address=(cpu.x[21]).wrapping_add(0xf0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d23430 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d23434 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23438 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d2343c;super::dispatch(cpu,rt,target)?;}
// 0x10d2343c ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23440 ldr w1, [x21, #0xfc]
{let address=(cpu.x[21]).wrapping_add(0xfcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d23444 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d23448 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d2344c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d23450;super::dispatch(cpu,rt,target)?;}
// 0x10d23450 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23454 ldrb w1, [x21, #0x155]
{let address=(cpu.x[21]).wrapping_add(0x155u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d23458 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d2345c ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23460 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d23464;super::dispatch(cpu,rt,target)?;}
// 0x10d23464 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23468 ldrb w1, [x21, #0x154]
{let address=(cpu.x[21]).wrapping_add(0x154u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d2346c mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d23470 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23474 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d23478;super::dispatch(cpu,rt,target)?;}
pc=0x10d23478;continue;
}
0x10d23478=>{
// 0x10d23478 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d2347c ldrb w1, [x21, #0x156]
{let address=(cpu.x[21]).wrapping_add(0x156u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d23480 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d23484 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23488 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d2348c;super::dispatch(cpu,rt,target)?;}
pc=0x10d2348c;continue;
}
0x10d2348c=>{
// 0x10d2348c ldr w19, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[19]=(v0) as u32 as u64;}
pc=0x10d23490;continue;
}
0x10d23490=>{
// 0x10d23490 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23494 ldr w1, [x21, #0xe8]
{let address=(cpu.x[21]).wrapping_add(0xe8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d23498 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d2349c ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d234a0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d234a4;super::dispatch(cpu,rt,target)?;}
pc=0x10d234a4;continue;
}
0x10d234a4=>{
// 0x10d234a4 ldr x8, [x21, #0x108]
{let address=(cpu.x[21]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d234a8 ldr x9, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d234ac mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d234b0 cmp x8, #0
{let a=((cpu.x[8]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d234b4 ldr x8, [x9, #0x40]
{let address=(cpu.x[9]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d234b8 cset w1, ne
cpu.x[1]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x10d234bc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d234c0;super::dispatch(cpu,rt,target)?;}
pc=0x10d234c0;continue;
}
0x10d234c0=>{
// 0x10d234c0 tbz w0, #0, #0x10d23528
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d23528;continue;}pc=0x10d234c4;continue;
}
0x10d234c4=>{
// 0x10d234c4 ldr x1, [x21, #0x108]
{let address=(cpu.x[21]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10d234c8 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d234cc bl #0x111dd3d0
cpu.x[30]=0x10d234d0;super::chunk5::f_111dd3d0(cpu,rt,0x111dd3d0)?;
// 0x10d234d0 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d234d4 add x1, x21, #0x120
{let a=((cpu.x[21]));let b=((0x120u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x10d234d8 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d234dc ldr x8, [x8, #0x28]
{let address=(cpu.x[8]).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d234e0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d234e4;super::dispatch(cpu,rt,target)?;}
// 0x10d234e4 ldr x1, [x21, #0x110]
{let address=(cpu.x[21]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10d234e8 ldr x3, [x21, #0x10]
{let address=(cpu.x[21]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[3]=(v0) as u64;}
// 0x10d234ec mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d234f0 mov w2, #0x10
cpu.x[2]=(((0x10u64))) as u32 as u64;
// 0x10d234f4 bl #0x111dd77c
cpu.x[30]=0x10d234f8;super::chunk15::f_111dd77c(cpu,rt,0x111dd77c)?;
// 0x10d234f8 ldr w8, [sp, #4]
{let address=(cpu.sp).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d234fc tbnz w8, #0, #0x10d23514
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10d23514;continue;}pc=0x10d23500;continue;
}
0x10d23500=>{
// 0x10d23500 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23504 ldr w1, [x21, #0x118]
{let address=(cpu.x[21]).wrapping_add(0x118u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d23508 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d2350c ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23510 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d23514;super::dispatch(cpu,rt,target)?;}
pc=0x10d23514;continue;
}
0x10d23514=>{
// 0x10d23514 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23518 ldr w1, [x21, #0x130]
{let address=(cpu.x[21]).wrapping_add(0x130u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d2351c mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d23520 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23524 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d23528;super::dispatch(cpu,rt,target)?;}
pc=0x10d23528;continue;
}
0x10d23528=>{
// 0x10d23528 ldr x8, [x21, #0x180]
{let address=(cpu.x[21]).wrapping_add(0x180u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d2352c ldr x9, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d23530 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d23534 cmp x8, #0
{let a=((cpu.x[8]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d23538 ldr x8, [x9, #0x40]
{let address=(cpu.x[9]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d2353c cset w1, ne
cpu.x[1]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x10d23540 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d23544;super::dispatch(cpu,rt,target)?;}
pc=0x10d23544;continue;
}
0x10d23544=>{
// 0x10d23544 tbz w0, #0, #0x10d23554
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d23554;continue;}pc=0x10d23548;continue;
}
0x10d23548=>{
// 0x10d23548 ldr x0, [x21, #0x180]
{let address=(cpu.x[21]).wrapping_add(0x180u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d2354c mov x1, x23
cpu.x[1]=(((cpu.x[23]))) as u64;
// 0x10d23550 bl #0x111d4910
cpu.x[30]=0x10d23554;super::dispatch(cpu,rt,0x111d4910)?;
pc=0x10d23554;continue;
}
0x10d23554=>{
// 0x10d23554 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23558 ldrb w1, [x21, #0x157]
{let address=(cpu.x[21]).wrapping_add(0x157u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d2355c mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d23560 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23564 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d23568;super::dispatch(cpu,rt,target)?;}
pc=0x10d23568;continue;
}
0x10d23568=>{
// 0x10d23568 tbnz w19, #0, #0x10d23680
if ((cpu.x[19] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10d23680;continue;}pc=0x10d2356c;continue;
}
0x10d2356c=>{
// 0x10d2356c ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23570 ldr w1, [x21, #0x134]
{let address=(cpu.x[21]).wrapping_add(0x134u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d23574 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d23578 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d2357c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d23580;super::dispatch(cpu,rt,target)?;}
pc=0x10d23580;continue;
}
0x10d23580=>{
// 0x10d23580 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23584 ldr w1, [x21, #0x138]
{let address=(cpu.x[21]).wrapping_add(0x138u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d23588 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d2358c ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23590 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d23594;super::dispatch(cpu,rt,target)?;}
pc=0x10d23594;continue;
}
0x10d23594=>{
// 0x10d23594 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23598 ldr w1, [x21, #0x13c]
{let address=(cpu.x[21]).wrapping_add(0x13cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d2359c mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d235a0 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d235a4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d235a8;super::dispatch(cpu,rt,target)?;}
pc=0x10d235a8;continue;
}
0x10d235a8=>{
// 0x10d235a8 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d235ac ldr w1, [x21, #0x140]
{let address=(cpu.x[21]).wrapping_add(0x140u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d235b0 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d235b4 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d235b8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d235bc;super::dispatch(cpu,rt,target)?;}
pc=0x10d235bc;continue;
}
0x10d235bc=>{
// 0x10d235bc ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d235c0 ldr w1, [x21, #0x144]
{let address=(cpu.x[21]).wrapping_add(0x144u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d235c4 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d235c8 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d235cc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d235d0;super::dispatch(cpu,rt,target)?;}
pc=0x10d235d0;continue;
}
0x10d235d0=>{
// 0x10d235d0 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d235d4 ldr w1, [x21, #0x148]
{let address=(cpu.x[21]).wrapping_add(0x148u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d235d8 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d235dc ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d235e0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d235e4;super::dispatch(cpu,rt,target)?;}
pc=0x10d235e4;continue;
}
0x10d235e4=>{
// 0x10d235e4 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d235e8 ldr w1, [x21, #0x14c]
{let address=(cpu.x[21]).wrapping_add(0x14cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d235ec mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d235f0 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d235f4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d235f8;super::dispatch(cpu,rt,target)?;}
pc=0x10d235f8;continue;
}
0x10d235f8=>{
// 0x10d235f8 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d235fc ldr w1, [x21, #0x150]
{let address=(cpu.x[21]).wrapping_add(0x150u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d23600 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d23604 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23608 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d2360c;super::dispatch(cpu,rt,target)?;}
pc=0x10d2360c;continue;
}
0x10d2360c=>{
// 0x10d2360c ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23610 ldr w1, [x21, #0x17c]
{let address=(cpu.x[21]).wrapping_add(0x17cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d23614 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d23618 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d2361c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d23620;super::dispatch(cpu,rt,target)?;}
pc=0x10d23620;continue;
}
0x10d23620=>{
// 0x10d23620 ldr w8, [x21, #0x17c]
{let address=(cpu.x[21]).wrapping_add(0x17cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d23624 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d23628 b.lt #0x10d23650
if cpu.condition(11){pc=0x10d23650;continue;}pc=0x10d2362c;continue;
}
0x10d2362c=>{
// 0x10d2362c mov x20, xzr
cpu.x[20]=(((0u64))) as u64;
pc=0x10d23630;continue;
}
0x10d23630=>{
// 0x10d23630 ldr x8, [x21, #0x170]
{let address=(cpu.x[21]).wrapping_add(0x170u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23634 mov x1, x23
cpu.x[1]=(((cpu.x[23]))) as u64;
// 0x10d23638 ldr x0, [x8, x20, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[20])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d2363c bl #0x10fdb4b4
cpu.x[30]=0x10d23640;super::dispatch(cpu,rt,0x10fdb4b4)?;
// 0x10d23640 ldrsw x8, [x21, #0x17c]
{let address=(cpu.x[21]).wrapping_add(0x17cu64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10d23644 add x20, x20, #1
{let a=((cpu.x[20]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[20]=(result) as u64;}
// 0x10d23648 cmp x20, x8
{let a=((cpu.x[20]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d2364c b.lt #0x10d23630
if cpu.condition(11){pc=0x10d23630;continue;}pc=0x10d23650;continue;
}
0x10d23650=>{
// 0x10d23650 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23654 ldr w1, [x21, #0x188]
{let address=(cpu.x[21]).wrapping_add(0x188u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d23658 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d2365c ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23660 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d23664;super::dispatch(cpu,rt,target)?;}
pc=0x10d23664;continue;
}
0x10d23664=>{
// 0x10d23664 ldr w8, [sp, #4]
{let address=(cpu.sp).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d23668 tbnz w8, #0, #0x10d23680
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10d23680;continue;}pc=0x10d2366c;continue;
}
0x10d2366c=>{
// 0x10d2366c ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23670 ldr w1, [x21, #0x190]
{let address=(cpu.x[21]).wrapping_add(0x190u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d23674 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d23678 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d2367c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d23680;super::dispatch(cpu,rt,target)?;}
pc=0x10d23680;continue;
}
0x10d23680=>{
// 0x10d23680 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23684 ldr w1, [x21, #0x194]
{let address=(cpu.x[21]).wrapping_add(0x194u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d23688 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d2368c ldr x8, [x8, #0x50]
{let address=(cpu.x[8]).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23690 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d23694;super::dispatch(cpu,rt,target)?;}
pc=0x10d23694;continue;
}
0x10d23694=>{
// 0x10d23694 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23698 ldr w1, [x21, #0x58]
{let address=(cpu.x[21]).wrapping_add(0x58u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d2369c mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d236a0 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d236a4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d236a8;super::dispatch(cpu,rt,target)?;}
pc=0x10d236a8;continue;
}
0x10d236a8=>{
// 0x10d236a8 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d236ac ldrb w1, [x21, #0x19c]
{let address=(cpu.x[21]).wrapping_add(0x19cu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d236b0 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d236b4 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d236b8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d236bc;super::dispatch(cpu,rt,target)?;}
pc=0x10d236bc;continue;
}
0x10d236bc=>{
// 0x10d236bc ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d236c0 ldrb w1, [x21, #0x18c]
{let address=(cpu.x[21]).wrapping_add(0x18cu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d236c4 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d236c8 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d236cc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d236d0;super::dispatch(cpu,rt,target)?;}
pc=0x10d236d0;continue;
}
0x10d236d0=>{
// 0x10d236d0 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d236d4 ldr w1, [x21, #0x1a0]
{let address=(cpu.x[21]).wrapping_add(0x1a0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d236d8 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d236dc ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d236e0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d236e4;super::dispatch(cpu,rt,target)?;}
pc=0x10d236e4;continue;
}
0x10d236e4=>{
// 0x10d236e4 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d236e8 ldr w1, [x21, #0x1a4]
{let address=(cpu.x[21]).wrapping_add(0x1a4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d236ec mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d236f0 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d236f4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d236f8;super::dispatch(cpu,rt,target)?;}
pc=0x10d236f8;continue;
}
0x10d236f8=>{
// 0x10d236f8 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d236fc ldr x1, [x21, #0x30]
{let address=(cpu.x[21]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10d23700 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d23704 ldr x8, [x8, #0x38]
{let address=(cpu.x[8]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23708 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d2370c;super::dispatch(cpu,rt,target)?;}
pc=0x10d2370c;continue;
}
0x10d2370c=>{
// 0x10d2370c ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23710 ldr w1, [x21, #0x38]
{let address=(cpu.x[21]).wrapping_add(0x38u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d23714 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d23718 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d2371c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d23720;super::dispatch(cpu,rt,target)?;}
pc=0x10d23720;continue;
}
0x10d23720=>{
// 0x10d23720 ldr x0, [x21, #0x1a8]
{let address=(cpu.x[21]).wrapping_add(0x1a8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d23724 mov x1, x23
cpu.x[1]=(((cpu.x[23]))) as u64;
// 0x10d23728 bl #0x10d22840
cpu.x[30]=0x10d2372c;super::chunk2::f_10d22840(cpu,rt,0x10d22840)?;
pc=0x10d2372c;continue;
}
0x10d2372c=>{
// 0x10d2372c ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23730 ldrb w1, [x21, #0x1b0]
{let address=(cpu.x[21]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d23734 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d23738 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d2373c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d23740;super::dispatch(cpu,rt,target)?;}
pc=0x10d23740;continue;
}
0x10d23740=>{
// 0x10d23740 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23744 ldr w1, [x21, #0x1b4]
{let address=(cpu.x[21]).wrapping_add(0x1b4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d23748 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d2374c ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23750 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d23754;super::dispatch(cpu,rt,target)?;}
pc=0x10d23754;continue;
}
0x10d23754=>{
// 0x10d23754 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23758 ldr w1, [x21, #0x1b8]
{let address=(cpu.x[21]).wrapping_add(0x1b8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d2375c mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d23760 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23764 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d23768;super::dispatch(cpu,rt,target)?;}
pc=0x10d23768;continue;
}
0x10d23768=>{
// 0x10d23768 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d2376c ldr w1, [x21, #0x1bc]
{let address=(cpu.x[21]).wrapping_add(0x1bcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d23770 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d23774 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23778 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d2377c;super::dispatch(cpu,rt,target)?;}
pc=0x10d2377c;continue;
}
0x10d2377c=>{
// 0x10d2377c ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23780 ldr w1, [x21, #0x1c0]
{let address=(cpu.x[21]).wrapping_add(0x1c0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d23784 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d23788 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d2378c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d23790;super::dispatch(cpu,rt,target)?;}
pc=0x10d23790;continue;
}
0x10d23790=>{
// 0x10d23790 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23794 ldr w1, [x21, #0x1c4]
{let address=(cpu.x[21]).wrapping_add(0x1c4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d23798 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d2379c ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d237a0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d237a4;super::dispatch(cpu,rt,target)?;}
pc=0x10d237a4;continue;
}
0x10d237a4=>{
// 0x10d237a4 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d237a8 ldr w1, [x21, #0x1c8]
{let address=(cpu.x[21]).wrapping_add(0x1c8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d237ac mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d237b0 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d237b4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d237b8;super::dispatch(cpu,rt,target)?;}
pc=0x10d237b8;continue;
}
0x10d237b8=>{
// 0x10d237b8 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d237bc ldr w1, [x21, #0x1cc]
{let address=(cpu.x[21]).wrapping_add(0x1ccu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d237c0 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d237c4 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d237c8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d237cc;super::dispatch(cpu,rt,target)?;}
pc=0x10d237cc;continue;
}
0x10d237cc=>{
// 0x10d237cc ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d237d0 ldr w1, [x21, #0x1d0]
{let address=(cpu.x[21]).wrapping_add(0x1d0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d237d4 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d237d8 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d237dc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d237e0;super::dispatch(cpu,rt,target)?;}
pc=0x10d237e0;continue;
}
0x10d237e0=>{
// 0x10d237e0 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d237e4 ldr w1, [x21, #0x1d4]
{let address=(cpu.x[21]).wrapping_add(0x1d4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d237e8 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d237ec ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d237f0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d237f4;super::dispatch(cpu,rt,target)?;}
pc=0x10d237f4;continue;
}
0x10d237f4=>{
// 0x10d237f4 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d237f8 ldr w1, [x21, #0x1d8]
{let address=(cpu.x[21]).wrapping_add(0x1d8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d237fc mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
pc=0x10d23800;continue;
}
0x10d23800=>{
// 0x10d23800 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23804 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d23808;super::dispatch(cpu,rt,target)?;}
pc=0x10d23808;continue;
}
0x10d23808=>{
// 0x10d23808 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d2380c ldr w1, [x21, #0x1dc]
{let address=(cpu.x[21]).wrapping_add(0x1dcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d23810 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d23814 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23818 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d2381c;super::dispatch(cpu,rt,target)?;}
pc=0x10d2381c;continue;
}
0x10d2381c=>{
// 0x10d2381c ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23820 ldr w1, [x21, #0x1e0]
{let address=(cpu.x[21]).wrapping_add(0x1e0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d23824 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d23828 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d2382c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d23830;super::dispatch(cpu,rt,target)?;}
pc=0x10d23830;continue;
}
0x10d23830=>{
// 0x10d23830 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23834 ldr w1, [x21, #0x1e4]
{let address=(cpu.x[21]).wrapping_add(0x1e4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d23838 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d2383c ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23840 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d23844;super::dispatch(cpu,rt,target)?;}
pc=0x10d23844;continue;
}
0x10d23844=>{
// 0x10d23844 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23848 ldr w1, [x21, #0x1e8]
{let address=(cpu.x[21]).wrapping_add(0x1e8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d2384c mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d23850 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23854 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d23858;super::dispatch(cpu,rt,target)?;}
pc=0x10d23858;continue;
}
0x10d23858=>{
// 0x10d23858 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d2385c ldr w1, [x21, #0x1ec]
{let address=(cpu.x[21]).wrapping_add(0x1ecu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d23860 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d23864 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23868 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d2386c;super::dispatch(cpu,rt,target)?;}
pc=0x10d2386c;continue;
}
0x10d2386c=>{
// 0x10d2386c ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23870 ldr w1, [x21, #0x1f0]
{let address=(cpu.x[21]).wrapping_add(0x1f0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d23874 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d23878 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d2387c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d23880;super::dispatch(cpu,rt,target)?;}
pc=0x10d23880;continue;
}
0x10d23880=>{
// 0x10d23880 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23884 ldr w1, [x21, #0x1f4]
{let address=(cpu.x[21]).wrapping_add(0x1f4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d23888 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d2388c ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23890 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d23894;super::dispatch(cpu,rt,target)?;}
pc=0x10d23894;continue;
}
0x10d23894=>{
// 0x10d23894 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d23898 ldr w1, [x21, #0x1f8]
{let address=(cpu.x[21]).wrapping_add(0x1f8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d2389c mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d238a0 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d238a4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d238a8;super::dispatch(cpu,rt,target)?;}
pc=0x10d238a8;continue;
}
0x10d238a8=>{
// 0x10d238a8 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d238ac ldr w1, [x21, #0x1fc]
{let address=(cpu.x[21]).wrapping_add(0x1fcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d238b0 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d238b4 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d238b8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d238bc;super::dispatch(cpu,rt,target)?;}
pc=0x10d238bc;continue;
}
0x10d238bc=>{
// 0x10d238bc ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d238c0 ldr w1, [x21, #0x200]
{let address=(cpu.x[21]).wrapping_add(0x200u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d238c4 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d238c8 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d238cc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d238d0;super::dispatch(cpu,rt,target)?;}
pc=0x10d238d0;continue;
}
0x10d238d0=>{
// 0x10d238d0 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d238d4 ldr w1, [x21, #0x198]
{let address=(cpu.x[21]).wrapping_add(0x198u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d238d8 ldr x2, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10d238dc mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d238e0 ldp x20, x19, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d238e4 ldp x22, x21, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d238e8 ldp x24, x23, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10d238ec ldp x26, x25, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10d238f0 ldp x28, x27, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10d238f4 ldp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10d238f8 add sp, sp, #0x70
{let a=((cpu.sp));let b=((0x70u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10d238fc br x2
{let target=cpu.x[2];if target>=0x10d22ea4 && target<0x10d23900{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d22ea4")),}}}
#[inline]
pub(super) unsafe fn f_10d24c34(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d24c34=>{
// 0x10d24c34 sub sp, sp, #0x70
{let a=((cpu.sp));let b=((0x70u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x10d24c38 stp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d24c3c stp x28, x27, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[28];let v1=cpu.x[27];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d24c40 stp x26, x25, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[26];let v1=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d24c44 stp x24, x23, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d24c48 stp x22, x21, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d24c4c stp x20, x19, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d24c50 add x29, sp, #0x10
{let a=((cpu.sp));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x10d24c54 cmp w1, #8
{let a=((cpu.x[1] as u32 as u64));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d24c58 mov w22, wzr
cpu.x[22]=(((0u64))) as u32 as u64;
// 0x10d24c5c b.hi #0x10d24fd0
if cpu.condition(8){pc=0x10d24fd0;continue;}pc=0x10d24c60;continue;
}
0x10d24c60=>{
// 0x10d24c60 mov w26, w3
cpu.x[26]=(((cpu.x[3] as u32 as u64))) as u32 as u64;
// 0x10d24c64 cbz w3, #0x10d24fd0
if (cpu.x[3] as u32 as u64) == 0 {pc=0x10d24fd0;continue;}pc=0x10d24c68;continue;
}
0x10d24c68=>{
// 0x10d24c68 mov w21, w5
cpu.x[21]=(((cpu.x[5] as u32 as u64))) as u32 as u64;
// 0x10d24c6c mov w25, w4
cpu.x[25]=(((cpu.x[4] as u32 as u64))) as u32 as u64;
// 0x10d24c70 mov x24, x2
cpu.x[24]=(((cpu.x[2]))) as u64;
// 0x10d24c74 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10d24c78 mov w8, wzr
cpu.x[8]=(((0u64))) as u32 as u64;
// 0x10d24c7c str w5, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=cpu.x[5] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d24c80 b #0x10d24ca8
pc=0x10d24ca8;continue;
}
0x10d24c84=>{
// 0x10d24c84 bl #0x10e1afec
cpu.x[30]=0x10d24c88;super::chunk3::f_10e1afec(cpu,rt,0x10e1afec)?;
pc=0x10d24c88;continue;
}
0x10d24c88=>{
// 0x10d24c88 ldur w8, [x29, #-4]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffffcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d24c8c mov x24, x0
cpu.x[24]=(((cpu.x[0]))) as u64;
// 0x10d24c90 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10d24c94 mov w25, #0xbc
cpu.x[25]=(((0xbcu64))) as u32 as u64;
// 0x10d24c98 tst w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.logical_flags(result,32);}
// 0x10d24c9c mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10d24ca0 csel w22, w22, w26, ne
cpu.x[22]=(if cpu.condition(1){((cpu.x[22] as u32 as u64))}else{((cpu.x[26] as u32 as u64))}) as u32 as u64;
// 0x10d24ca4 cbz w26, #0x10d24fd0
if (cpu.x[26] as u32 as u64) == 0 {pc=0x10d24fd0;continue;}pc=0x10d24ca8;continue;
}
0x10d24ca8=>{
// 0x10d24ca8 stur w8, [x29, #-4]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffffcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d24cac add x8, x20, w1, uxtw #3
{let a=((cpu.x[20]));let b=((((cpu.x[1] as u32 as u64) as u32 as u64)) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d24cb0 str w1, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=cpu.x[1] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d24cb4 ldr x28, [x8, #0xa0]
{let address=(cpu.x[8]).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[28]=(v0) as u64;}
// 0x10d24cb8 ldr w19, [x28, #0xc]
{let address=(cpu.x[28]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[19]=(v0) as u32 as u64;}
// 0x10d24cbc cmp w19, #1
{let a=((cpu.x[19] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d24cc0 b.lt #0x10d24d1c
if cpu.condition(11){pc=0x10d24d1c;continue;}pc=0x10d24cc4;continue;
}
0x10d24cc4=>{
// 0x10d24cc4 mov x23, xzr
cpu.x[23]=(((0u64))) as u64;
// 0x10d24cc8 b #0x10d24cd8
pc=0x10d24cd8;continue;
}
0x10d24ccc=>{
// 0x10d24ccc add x23, x23, #1
{let a=((cpu.x[23]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[23]=(result) as u64;}
// 0x10d24cd0 cmp x19, x23
{let a=((cpu.x[19]));let b=((cpu.x[23]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d24cd4 b.eq #0x10d24d1c
if cpu.condition(0){pc=0x10d24d1c;continue;}pc=0x10d24cd8;continue;
}
0x10d24cd8=>{
// 0x10d24cd8 ldr x8, [x28]
{let address=(cpu.x[28]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d24cdc ldr x8, [x8, x23, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[23])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d24ce0 cbz x8, #0x10d24ccc
if (cpu.x[8]) == 0 {pc=0x10d24ccc;continue;}pc=0x10d24ce4;continue;
}
0x10d24ce4=>{
// 0x10d24ce4 ldr x0, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d24ce8 cbz x0, #0x10d24ccc
if (cpu.x[0]) == 0 {pc=0x10d24ccc;continue;}pc=0x10d24cec;continue;
}
0x10d24cec=>{
// 0x10d24cec mov x1, x24
cpu.x[1]=(((cpu.x[24]))) as u64;
// 0x10d24cf0 bl #0x10e0bc20
cpu.x[30]=0x10d24cf4;super::chunk1::f_10e0bc20(cpu,rt,0x10e0bc20)?;
pc=0x10d24cf4;continue;
}
0x10d24cf4=>{
// 0x10d24cf4 tbz w0, #0, #0x10d24ccc
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d24ccc;continue;}pc=0x10d24cf8;continue;
}
0x10d24cf8=>{
// 0x10d24cf8 tbnz w23, #0x1f, #0x10d24d1c
if ((cpu.x[23] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x10d24d1c;continue;}pc=0x10d24cfc;continue;
}
0x10d24cfc=>{
// 0x10d24cfc ldr x8, [x28]
{let address=(cpu.x[28]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d24d00 ldr x27, [x8, x23, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[23])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[27]=(v0) as u64;}
// 0x10d24d04 ldr w28, [x27, #0x10]
{let address=(cpu.x[27]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[28]=(v0) as u32 as u64;}
// 0x10d24d08 ldr x0, [x20, #0x10]
{let address=(cpu.x[20]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d24d0c add w8, w28, w26
{let a=((cpu.x[28] as u32 as u64));let b=((cpu.x[26] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10d24d10 str w8, [x27, #0x10]
{let address=(cpu.x[27]).wrapping_add(0x10u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d24d14 cbnz x0, #0x10d24d8c
if (cpu.x[0]) != 0 {pc=0x10d24d8c;continue;}pc=0x10d24d18;continue;
}
0x10d24d18=>{
// 0x10d24d18 b #0x10d24d9c
pc=0x10d24d9c;continue;
}
0x10d24d1c=>{
// 0x10d24d1c mov w0, #0x20
cpu.x[0]=(((0x20u64))) as u32 as u64;
// 0x10d24d20 bl #0x1187d2b0
cpu.x[30]=0x10d24d24;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
pc=0x10d24d24;continue;
}
0x10d24d24=>{
// 0x10d24d24 ldr x1, [x20, #0x10]
{let address=(cpu.x[20]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10d24d28 mov x2, x24
cpu.x[2]=(((cpu.x[24]))) as u64;
// 0x10d24d2c mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10d24d30 mov x27, x0
cpu.x[27]=(((cpu.x[0]))) as u64;
// 0x10d24d34 bl #0x111e4f24
cpu.x[30]=0x10d24d38;super::chunk12::f_111e4f24(cpu,rt,0x111e4f24)?;
pc=0x10d24d38;continue;
}
0x10d24d38=>{
// 0x10d24d38 ldp w9, w8, [x28, #8]
{let address=(cpu.x[28]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10d24d3c cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d24d40 b.ne #0x10d24d68
if cpu.condition(1){pc=0x10d24d68;continue;}pc=0x10d24d44;continue;
}
0x10d24d44=>{
// 0x10d24d44 lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10d24d48 mov w10, #5
cpu.x[10]=(((0x5u64))) as u32 as u64;
// 0x10d24d4c cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d24d50 csel w1, w10, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[10] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10d24d54 cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d24d58 b.ge #0x10d24d68
if cpu.condition(10){pc=0x10d24d68;continue;}pc=0x10d24d5c;continue;
}
0x10d24d5c=>{
// 0x10d24d5c mov x0, x28
cpu.x[0]=(((cpu.x[28]))) as u64;
// 0x10d24d60 bl #0x10d2d0f8
cpu.x[30]=0x10d24d64;super::chunk4::f_10d2d0f8(cpu,rt,0x10d2d0f8)?;
// 0x10d24d64 ldr w8, [x28, #0xc]
{let address=(cpu.x[28]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
pc=0x10d24d68;continue;
}
0x10d24d68=>{
// 0x10d24d68 ldr x9, [x28]
{let address=(cpu.x[28]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d24d6c add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10d24d70 str w10, [x28, #0xc]
{let address=(cpu.x[28]).wrapping_add(0xcu64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d24d74 str x27, [x9, w8, sxtw #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64);let v0=cpu.x[27];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d24d78 ldr w28, [x27, #0x10]
{let address=(cpu.x[27]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[28]=(v0) as u32 as u64;}
// 0x10d24d7c ldr x0, [x20, #0x10]
{let address=(cpu.x[20]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d24d80 add w8, w28, w26
{let a=((cpu.x[28] as u32 as u64));let b=((cpu.x[26] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10d24d84 str w8, [x27, #0x10]
{let address=(cpu.x[27]).wrapping_add(0x10u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d24d88 cbz x0, #0x10d24d9c
if (cpu.x[0]) == 0 {pc=0x10d24d9c;continue;}pc=0x10d24d8c;continue;
}
0x10d24d8c=>{
// 0x10d24d8c bl #0x10e167a4
cpu.x[30]=0x10d24d90;super::chunk15::f_10e167a4(cpu,rt,0x10e167a4)?;
pc=0x10d24d90;continue;
}
0x10d24d90=>{
// 0x10d24d90 cmp x0, x24
{let a=((cpu.x[0]));let b=((cpu.x[24]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d24d94 b.ne #0x10d24e84
if cpu.condition(1){pc=0x10d24e84;continue;}pc=0x10d24d98;continue;
}
0x10d24d98=>{
// 0x10d24d98 b #0x10d24db0
pc=0x10d24db0;continue;
}
0x10d24d9c=>{
// 0x10d24d9c ldr x0, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d24da0 bl #0x110cbf10
cpu.x[30]=0x10d24da4;super::chunk14::f_110cbf10(cpu,rt,0x110cbf10)?;
// 0x10d24da4 bl #0x10e167a4
cpu.x[30]=0x10d24da8;super::chunk15::f_10e167a4(cpu,rt,0x10e167a4)?;
// 0x10d24da8 cmp x0, x24
{let a=((cpu.x[0]));let b=((cpu.x[24]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d24dac b.ne #0x10d24e84
if cpu.condition(1){pc=0x10d24e84;continue;}pc=0x10d24db0;continue;
}
0x10d24db0=>{
// 0x10d24db0 ldr x0, [x20, #0x10]
{let address=(cpu.x[20]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d24db4 cbz x0, #0x10d24e40
if (cpu.x[0]) == 0 {pc=0x10d24e40;continue;}pc=0x10d24db8;continue;
}
0x10d24db8=>{
// 0x10d24db8 bl #0x10e167cc
cpu.x[30]=0x10d24dbc;super::chunk14::f_10e167cc(cpu,rt,0x10e167cc)?;
pc=0x10d24dbc;continue;
}
0x10d24dbc=>{
// 0x10d24dbc ldr x19, [x20, #0xa0]
{let address=(cpu.x[20]).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10d24dc0 mov x23, x0
cpu.x[23]=(((cpu.x[0]))) as u64;
// 0x10d24dc4 cbz x19, #0x10d24e58
if (cpu.x[19]) == 0 {pc=0x10d24e58;continue;}pc=0x10d24dc8;continue;
}
0x10d24dc8=>{
// 0x10d24dc8 ldr w8, [x19, #0xc]
{let address=(cpu.x[19]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d24dcc cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d24dd0 b.lt #0x10d24e74
if cpu.condition(11){pc=0x10d24e74;continue;}pc=0x10d24dd4;continue;
}
0x10d24dd4=>{
// 0x10d24dd4 str w26, [sp, #4]
{let address=(cpu.sp).wrapping_add(0x4u64);let v0=cpu.x[26] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d24dd8 lsl x26, x8, #3
cpu.x[26]=(((((cpu.x[8]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10d24ddc mov x21, xzr
cpu.x[21]=(((0u64))) as u64;
// 0x10d24de0 b #0x10d24df0
pc=0x10d24df0;continue;
}
0x10d24de4=>{
// 0x10d24de4 add x21, x21, #8
{let a=((cpu.x[21]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[21]=(result) as u64;}
// 0x10d24de8 cmp x26, x21
{let a=((cpu.x[26]));let b=((cpu.x[21]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d24dec b.eq #0x10d24e28
if cpu.condition(0){pc=0x10d24e28;continue;}pc=0x10d24df0;continue;
}
0x10d24df0=>{
// 0x10d24df0 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d24df4 ldr x8, [x8, x21]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[21])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d24df8 cbz x8, #0x10d24de4
if (cpu.x[8]) == 0 {pc=0x10d24de4;continue;}pc=0x10d24dfc;continue;
}
0x10d24dfc=>{
// 0x10d24dfc ldr x0, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d24e00 cbz x0, #0x10d24de4
if (cpu.x[0]) == 0 {pc=0x10d24de4;continue;}pc=0x10d24e04;continue;
}
0x10d24e04=>{
// 0x10d24e04 mov x1, x23
cpu.x[1]=(((cpu.x[23]))) as u64;
// 0x10d24e08 bl #0x10e0bc20
cpu.x[30]=0x10d24e0c;super::chunk1::f_10e0bc20(cpu,rt,0x10e0bc20)?;
pc=0x10d24e0c;continue;
}
0x10d24e0c=>{
// 0x10d24e0c tbz w0, #0, #0x10d24de4
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d24de4;continue;}pc=0x10d24e10;continue;
}
0x10d24e10=>{
// 0x10d24e10 mov x8, #0x7fffffff8
cpu.x[8]=(((0x7fffffff8u64))) as u64;
// 0x10d24e14 ldr w26, [sp, #4]
{let address=(cpu.sp).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[26]=(v0) as u32 as u64;}
// 0x10d24e18 cmp x21, x8
{let a=((cpu.x[21]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d24e1c b.ne #0x10d24f00
if cpu.condition(1){pc=0x10d24f00;continue;}pc=0x10d24e20;continue;
}
0x10d24e20=>{
// 0x10d24e20 mov w8, wzr
cpu.x[8]=(((0u64))) as u32 as u64;
// 0x10d24e24 b #0x10d24f0c
pc=0x10d24f0c;continue;
}
0x10d24e28=>{
// 0x10d24e28 ldp w21, w26, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[21]=(v0) as u32 as u64;cpu.x[26]=(v1) as u32 as u64;}
// 0x10d24e2c mov w8, wzr
cpu.x[8]=(((0u64))) as u32 as u64;
// 0x10d24e30 ldr w9, [x27, #0x10]
{let address=(cpu.x[27]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10d24e34 cmp w9, wzr
{let a=((cpu.x[9] as u32 as u64));let b=((0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d24e38 b.ge #0x10d24e84
if cpu.condition(10){pc=0x10d24e84;continue;}pc=0x10d24e3c;continue;
}
0x10d24e3c=>{
// 0x10d24e3c b #0x10d24f1c
pc=0x10d24f1c;continue;
}
0x10d24e40=>{
// 0x10d24e40 ldr x0, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d24e44 bl #0x110cbf10
cpu.x[30]=0x10d24e48;super::chunk14::f_110cbf10(cpu,rt,0x110cbf10)?;
// 0x10d24e48 bl #0x10e167cc
cpu.x[30]=0x10d24e4c;super::chunk14::f_10e167cc(cpu,rt,0x10e167cc)?;
// 0x10d24e4c ldr x19, [x20, #0xa0]
{let address=(cpu.x[20]).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10d24e50 mov x23, x0
cpu.x[23]=(((cpu.x[0]))) as u64;
// 0x10d24e54 cbnz x19, #0x10d24dc8
if (cpu.x[19]) != 0 {pc=0x10d24dc8;continue;}pc=0x10d24e58;continue;
}
0x10d24e58=>{
// 0x10d24e58 adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x10d24e5c add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d24e60 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d24e64 bl #0x1128df18
cpu.x[30]=0x10d24e68;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10d24e68 ldr w8, [x19, #0xc]
{let address=(cpu.x[19]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d24e6c cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d24e70 b.ge #0x10d24dd4
if cpu.condition(10){pc=0x10d24dd4;continue;}pc=0x10d24e74;continue;
}
0x10d24e74=>{
// 0x10d24e74 mov w8, wzr
cpu.x[8]=(((0u64))) as u32 as u64;
// 0x10d24e78 ldr w9, [x27, #0x10]
{let address=(cpu.x[27]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10d24e7c cmp w9, wzr
{let a=((cpu.x[9] as u32 as u64));let b=((0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d24e80 b.lt #0x10d24f1c
if cpu.condition(11){pc=0x10d24f1c;continue;}pc=0x10d24e84;continue;
}
0x10d24e84=>{
// 0x10d24e84 cmp w26, #0
{let a=((cpu.x[26] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d24e88 b.le #0x10d24f50
if cpu.condition(13){pc=0x10d24f50;continue;}pc=0x10d24e8c;continue;
}
0x10d24e8c=>{
// 0x10d24e8c mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10d24e90 bl #0x10e0b614
cpu.x[30]=0x10d24e94;super::chunk0::f_10e0b614(cpu,rt,0x10e0b614)?;
pc=0x10d24e94;continue;
}
0x10d24e94=>{
// 0x10d24e94 cmp w0, #5
{let a=((cpu.x[0] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d24e98 b.ne #0x10d24ed0
if cpu.condition(1){pc=0x10d24ed0;continue;}pc=0x10d24e9c;continue;
}
0x10d24e9c=>{
// 0x10d24e9c tbz w21, #0, #0x10d24ed0
if ((cpu.x[21] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d24ed0;continue;}pc=0x10d24ea0;continue;
}
0x10d24ea0=>{
// 0x10d24ea0 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10d24ea4 bl #0x10e692bc
cpu.x[30]=0x10d24ea8;super::chunk1::f_10e692bc(cpu,rt,0x10e692bc)?;
pc=0x10d24ea8;continue;
}
0x10d24ea8=>{
// 0x10d24ea8 ldr w3, [x27, #0x10]
{let address=(cpu.x[27]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10d24eac cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d24eb0 b.lt #0x10d24ed4
if cpu.condition(11){pc=0x10d24ed4;continue;}pc=0x10d24eb4;continue;
}
0x10d24eb4=>{
// 0x10d24eb4 cmp w28, w0
{let a=((cpu.x[28] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d24eb8 csel w8, w28, w0, gt
cpu.x[8]=(if cpu.condition(12){((cpu.x[28] as u32 as u64))}else{((cpu.x[0] as u32 as u64))}) as u32 as u64;
// 0x10d24ebc cmp w3, w8
{let a=((cpu.x[3] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d24ec0 csel w3, w3, w8, lt
cpu.x[3]=(if cpu.condition(11){((cpu.x[3] as u32 as u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10d24ec4 sub w26, w3, w28
{let a=((cpu.x[3] as u32 as u64));let b=((cpu.x[28] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[26]=(result) as u32 as u64;}
// 0x10d24ec8 str w3, [x27, #0x10]
{let address=(cpu.x[27]).wrapping_add(0x10u64);let v0=cpu.x[3] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d24ecc b #0x10d24ed4
pc=0x10d24ed4;continue;
}
0x10d24ed0=>{
// 0x10d24ed0 ldr w3, [x27, #0x10]
{let address=(cpu.x[27]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
pc=0x10d24ed4;continue;
}
0x10d24ed4=>{
// 0x10d24ed4 cmp w26, #0
{let a=((cpu.x[26] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d24ed8 cset w23, gt
cpu.x[23]=((cpu.condition(12)) as u64) as u32 as u64;
// 0x10d24edc cmp w26, #1
{let a=((cpu.x[26] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d24ee0 b.lt #0x10d24f58
if cpu.condition(11){pc=0x10d24f58;continue;}pc=0x10d24ee4;continue;
}
0x10d24ee4=>{
// 0x10d24ee4 cmp w28, #1
{let a=((cpu.x[28] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d24ee8 b.lt #0x10d24f58
if cpu.condition(11){pc=0x10d24f58;continue;}pc=0x10d24eec;continue;
}
0x10d24eec=>{
// 0x10d24eec tbz w3, #0x1f, #0x10d24ef8
if ((cpu.x[3] as u32 as u64) & (1u64 << 31)) == 0 {pc=0x10d24ef8;continue;}pc=0x10d24ef0;continue;
}
0x10d24ef0=>{
// 0x10d24ef0 mov w3, #0x7fffffff
cpu.x[3]=(((0x7fffffffu64))) as u32 as u64;
// 0x10d24ef4 str w3, [x27, #0x10]
{let address=(cpu.x[27]).wrapping_add(0x10u64);let v0=cpu.x[3] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10d24ef8;continue;
}
0x10d24ef8=>{
// 0x10d24ef8 mov w23, #1
cpu.x[23]=(((0x1u64))) as u32 as u64;
// 0x10d24efc b #0x10d24f58
pc=0x10d24f58;continue;
}
0x10d24f00=>{
// 0x10d24f00 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d24f04 ldr x8, [x8, x21]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[21])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d24f08 ldr w8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
pc=0x10d24f0c;continue;
}
0x10d24f0c=>{
// 0x10d24f0c ldr w21, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[21]=(v0) as u32 as u64;}
// 0x10d24f10 ldr w9, [x27, #0x10]
{let address=(cpu.x[27]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10d24f14 cmp w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d24f18 b.ge #0x10d24e84
if cpu.condition(10){pc=0x10d24e84;continue;}pc=0x10d24f1c;continue;
}
0x10d24f1c=>{
// 0x10d24f1c ldr x0, [x20, #0x10]
{let address=(cpu.x[20]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d24f20 cbnz x0, #0x10d24f2c
if (cpu.x[0]) != 0 {pc=0x10d24f2c;continue;}pc=0x10d24f24;continue;
}
0x10d24f24=>{
// 0x10d24f24 ldr x0, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d24f28 bl #0x110cbf10
cpu.x[30]=0x10d24f2c;super::chunk14::f_110cbf10(cpu,rt,0x110cbf10)?;
pc=0x10d24f2c;continue;
}
0x10d24f2c=>{
// 0x10d24f2c bl #0x10e167cc
cpu.x[30]=0x10d24f30;super::chunk14::f_10e167cc(cpu,rt,0x10e167cc)?;
// 0x10d24f30 ldr w3, [x27, #0x10]
{let address=(cpu.x[27]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10d24f34 mov x2, x0
cpu.x[2]=(((cpu.x[0]))) as u64;
// 0x10d24f38 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d24f3c mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10d24f40 mov w4, #1
cpu.x[4]=(((0x1u64))) as u32 as u64;
// 0x10d24f44 bl #0x10d24ff4
cpu.x[30]=0x10d24f48;super::chunk15::f_10d24ff4(cpu,rt,0x10d24ff4)?;
// 0x10d24f48 cmp w26, #0
{let a=((cpu.x[26] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d24f4c b.gt #0x10d24e8c
if cpu.condition(12){pc=0x10d24e8c;continue;}pc=0x10d24f50;continue;
}
0x10d24f50=>{
// 0x10d24f50 ldr w3, [x27, #0x10]
{let address=(cpu.x[27]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10d24f54 mov w23, wzr
cpu.x[23]=(((0u64))) as u32 as u64;
pc=0x10d24f58;continue;
}
0x10d24f58=>{
// 0x10d24f58 ldr x0, [x20, #0x100]
{let address=(cpu.x[20]).wrapping_add(0x100u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d24f5c ldr w1, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d24f60 mov x2, x24
cpu.x[2]=(((cpu.x[24]))) as u64;
// 0x10d24f64 mov w4, w25
cpu.x[4]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10d24f68 mov w5, w28
cpu.x[5]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10d24f6c ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d24f70 ldr x8, [x8, #0x50]
{let address=(cpu.x[8]).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d24f74 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d24f78;super::dispatch(cpu,rt,target)?;}
pc=0x10d24f78;continue;
}
0x10d24f78=>{
// 0x10d24f78 ldr x0, [x20, #0x10]
{let address=(cpu.x[20]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d24f7c cbnz x0, #0x10d24f88
if (cpu.x[0]) != 0 {pc=0x10d24f88;continue;}pc=0x10d24f80;continue;
}
0x10d24f80=>{
// 0x10d24f80 ldr x0, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d24f84 bl #0x110cbf10
cpu.x[30]=0x10d24f88;super::chunk14::f_110cbf10(cpu,rt,0x110cbf10)?;
pc=0x10d24f88;continue;
}
0x10d24f88=>{
// 0x10d24f88 bl #0x10e167a4
cpu.x[30]=0x10d24f8c;super::chunk15::f_10e167a4(cpu,rt,0x10e167a4)?;
pc=0x10d24f8c;continue;
}
0x10d24f8c=>{
// 0x10d24f8c cmp w25, #0xbc
{let a=((cpu.x[25] as u32 as u64));let b=((0xbcu64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d24f90 mov w8, #0x72
cpu.x[8]=(((0x72u64))) as u32 as u64;
// 0x10d24f94 ccmp w25, w8, #4, ne
if cpu.condition(1){{let a=((cpu.x[25] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}}else{cpu.nzcv=4u64 << 28;}
// 0x10d24f98 mov w8, #0x97
cpu.x[8]=(((0x97u64))) as u32 as u64;
// 0x10d24f9c ccmp x0, x24, #0, ne
if cpu.condition(1){{let a=((cpu.x[0]));let b=((cpu.x[24]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}}else{cpu.nzcv=0u64 << 28;}
// 0x10d24fa0 ccmp w25, w8, #4, eq
if cpu.condition(0){{let a=((cpu.x[25] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}}else{cpu.nzcv=4u64 << 28;}
// 0x10d24fa4 csel w8, wzr, w23, eq
cpu.x[8]=(if cpu.condition(0){((0u64))}else{((cpu.x[23] as u32 as u64))}) as u32 as u64;
// 0x10d24fa8 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d24fac b.ne #0x10d24fc4
if cpu.condition(1){pc=0x10d24fc4;continue;}pc=0x10d24fb0;continue;
}
0x10d24fb0=>{
// 0x10d24fb0 ldr x0, [x20, #0x10]
{let address=(cpu.x[20]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d24fb4 cbnz x0, #0x10d24c84
if (cpu.x[0]) != 0 {pc=0x10d24c84;continue;}pc=0x10d24fb8;continue;
}
0x10d24fb8=>{
// 0x10d24fb8 ldr x0, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d24fbc bl #0x110cbf10
cpu.x[30]=0x10d24fc0;super::chunk14::f_110cbf10(cpu,rt,0x110cbf10)?;
// 0x10d24fc0 b #0x10d24c84
pc=0x10d24c84;continue;
}
0x10d24fc4=>{
// 0x10d24fc4 ldur w8, [x29, #-4]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffffcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d24fc8 tst w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.logical_flags(result,32);}
// 0x10d24fcc csel w22, w22, w26, ne
cpu.x[22]=(if cpu.condition(1){((cpu.x[22] as u32 as u64))}else{((cpu.x[26] as u32 as u64))}) as u32 as u64;
pc=0x10d24fd0;continue;
}
0x10d24fd0=>{
// 0x10d24fd0 mov w0, w22
cpu.x[0]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10d24fd4 ldp x20, x19, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d24fd8 ldp x22, x21, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d24fdc ldp x24, x23, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10d24fe0 ldp x26, x25, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10d24fe4 ldp x28, x27, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10d24fe8 ldp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10d24fec add sp, sp, #0x70
{let a=((cpu.sp));let b=((0x70u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10d24ff0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d24c34")),}}}
#[inline]
pub(super) unsafe fn f_10d2801c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d2801c=>{
// 0x10d2801c tbnz w1, #0x1f, #0x10d28068
if ((cpu.x[1] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x10d28068;continue;}pc=0x10d28020;continue;
}
0x10d28020=>{
// 0x10d28020 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d28024 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d28028 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d2802c mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10d28030 ldr x0, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d28034 mov w19, w1
cpu.x[19]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x10d28038 cbnz x0, #0x10d28044
if (cpu.x[0]) != 0 {pc=0x10d28044;continue;}pc=0x10d2803c;continue;
}
0x10d2803c=>{
// 0x10d2803c ldr x0, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d28040 bl #0x110cbf10
cpu.x[30]=0x10d28044;super::chunk14::f_110cbf10(cpu,rt,0x110cbf10)?;
pc=0x10d28044;continue;
}
0x10d28044=>{
// 0x10d28044 bl #0x10e1af9c
cpu.x[30]=0x10d28048;super::chunk5::f_10e1af9c(cpu,rt,0x10e1af9c)?;
pc=0x10d28048;continue;
}
0x10d28048=>{
// 0x10d28048 mov x2, x0
cpu.x[2]=(((cpu.x[0]))) as u64;
// 0x10d2804c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d28050 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10d28054 mov w3, w19
cpu.x[3]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10d28058 mov w4, #1
cpu.x[4]=(((0x1u64))) as u32 as u64;
// 0x10d2805c ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d28060 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10d28064 b #0x10d24ff4
super::chunk15::f_10d24ff4(cpu,rt,0x10d24ff4)?;return Ok(());
}
0x10d28068=>{
// 0x10d28068 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d2801c")),}}}
#[inline]
pub(super) unsafe fn f_10d2db64(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d2db64=>{
// 0x10d2db64 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d2db64")),}}}
#[inline]
pub(super) unsafe fn f_10d32d10(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d32d10=>{
// 0x10d32d10 stp x29, x30, [sp, #-0x50]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffb0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d32d14 stp x26, x25, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[26];let v1=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d32d18 stp x24, x23, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d32d1c stp x22, x21, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d32d20 stp x20, x19, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d32d24 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d32d28 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10d32d2c ldr x0, [x0, #0x1f0]
{let address=(cpu.x[0]).wrapping_add(0x1f0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d32d30 cbz x0, #0x10d32d38
if (cpu.x[0]) == 0 {pc=0x10d32d38;continue;}pc=0x10d32d34;continue;
}
0x10d32d34=>{
// 0x10d32d34 bl #0x1187d2c0
cpu.x[30]=0x10d32d38;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10d32d38;continue;
}
0x10d32d38=>{
// 0x10d32d38 ldr x0, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d32d3c str xzr, [x19, #0x1f0]
{let address=(cpu.x[19]).wrapping_add(0x1f0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d32d40 cbz x0, #0x10d32d90
if (cpu.x[0]) == 0 {pc=0x10d32d90;continue;}pc=0x10d32d44;continue;
}
0x10d32d44=>{
// 0x10d32d44 bl #0x111d8a78
cpu.x[30]=0x10d32d48;super::chunk1::f_111d8a78(cpu,rt,0x111d8a78)?;
pc=0x10d32d48;continue;
}
0x10d32d48=>{
// 0x10d32d48 ldr x20, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10d32d4c cbz x20, #0x10d32d8c
if (cpu.x[20]) == 0 {pc=0x10d32d8c;continue;}pc=0x10d32d50;continue;
}
0x10d32d50=>{
// 0x10d32d50 mov x22, x20
cpu.x[22]=(((cpu.x[20]))) as u64;
// 0x10d32d54 ldr x0, [x22, #0x80]!
{let address=(cpu.x[22]).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;cpu.x[22]=(address) as u64;}
// 0x10d32d58 cbz x0, #0x10d32d60
if (cpu.x[0]) == 0 {pc=0x10d32d60;continue;}pc=0x10d32d5c;continue;
}
0x10d32d5c=>{
// 0x10d32d5c bl #0x1187d360
cpu.x[30]=0x10d32d60;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x10d32d60;continue;
}
0x10d32d60=>{
// 0x10d32d60 mov x21, x20
cpu.x[21]=(((cpu.x[20]))) as u64;
// 0x10d32d64 stp xzr, xzr, [x22]
{let address=(cpu.x[22]).wrapping_add(0x0u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d32d68 ldr x0, [x21, #0x60]!
{let address=(cpu.x[21]).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;cpu.x[21]=(address) as u64;}
// 0x10d32d6c cbz x0, #0x10d32d74
if (cpu.x[0]) == 0 {pc=0x10d32d74;continue;}pc=0x10d32d70;continue;
}
0x10d32d70=>{
// 0x10d32d70 bl #0x1187d360
cpu.x[30]=0x10d32d74;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x10d32d74;continue;
}
0x10d32d74=>{
// 0x10d32d74 ldr x0, [x20, #0x50]
{let address=(cpu.x[20]).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d32d78 stp xzr, xzr, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d32d7c cbz x0, #0x10d32d84
if (cpu.x[0]) == 0 {pc=0x10d32d84;continue;}pc=0x10d32d80;continue;
}
0x10d32d80=>{
// 0x10d32d80 bl #0x1187d360
cpu.x[30]=0x10d32d84;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x10d32d84;continue;
}
0x10d32d84=>{
// 0x10d32d84 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d32d88 bl #0x1187d2c0
cpu.x[30]=0x10d32d8c;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10d32d8c;continue;
}
0x10d32d8c=>{
// 0x10d32d8c str xzr, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10d32d90;continue;
}
0x10d32d90=>{
// 0x10d32d90 ldr x0, [x19, #0x190]
{let address=(cpu.x[19]).wrapping_add(0x190u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d32d94 cbz x0, #0x10d32dbc
if (cpu.x[0]) == 0 {pc=0x10d32dbc;continue;}pc=0x10d32d98;continue;
}
0x10d32d98=>{
// 0x10d32d98 bl #0x10d55a08
cpu.x[30]=0x10d32d9c;super::chunk1::f_10d55a08(cpu,rt,0x10d55a08)?;
pc=0x10d32d9c;continue;
}
0x10d32d9c=>{
// 0x10d32d9c ldr x20, [x19, #0x190]
{let address=(cpu.x[19]).wrapping_add(0x190u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10d32da0 cbz x20, #0x10d32db8
if (cpu.x[20]) == 0 {pc=0x10d32db8;continue;}pc=0x10d32da4;continue;
}
0x10d32da4=>{
// 0x10d32da4 ldr x0, [x20, #0x68]
{let address=(cpu.x[20]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d32da8 cbz x0, #0x10d32db0
if (cpu.x[0]) == 0 {pc=0x10d32db0;continue;}pc=0x10d32dac;continue;
}
0x10d32dac=>{
// 0x10d32dac bl #0x1187d360
cpu.x[30]=0x10d32db0;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x10d32db0;continue;
}
0x10d32db0=>{
// 0x10d32db0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d32db4 bl #0x1187d2c0
cpu.x[30]=0x10d32db8;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10d32db8;continue;
}
0x10d32db8=>{
// 0x10d32db8 str xzr, [x19, #0x190]
{let address=(cpu.x[19]).wrapping_add(0x190u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10d32dbc;continue;
}
0x10d32dbc=>{
// 0x10d32dbc ldr x0, [x19, #0x1d8]
{let address=(cpu.x[19]).wrapping_add(0x1d8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d32dc0 add x21, x19, #0x1d8
{let a=((cpu.x[19]));let b=((0x1d8u64));let result=a.wrapping_add(b);cpu.x[21]=(result) as u64;}
// 0x10d32dc4 cbz x0, #0x10d32e64
if (cpu.x[0]) == 0 {pc=0x10d32e64;continue;}pc=0x10d32dc8;continue;
}
0x10d32dc8=>{
// 0x10d32dc8 bl #0x10d4fecc
cpu.x[30]=0x10d32dcc;super::dispatch(cpu,rt,0x10d4fecc)?;
// 0x10d32dcc ldr x20, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10d32dd0 cbz x20, #0x10d32e60
if (cpu.x[20]) == 0 {pc=0x10d32e60;continue;}pc=0x10d32dd4;continue;
}
0x10d32dd4=>{
// 0x10d32dd4 ldr x8, [x20, #0x80]
{let address=(cpu.x[20]).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d32dd8 cbz x8, #0x10d32e1c
if (cpu.x[8]) == 0 {pc=0x10d32e1c;continue;}pc=0x10d32ddc;continue;
}
0x10d32ddc=>{
// 0x10d32ddc ldr x0, [x20, #0x78]
{let address=(cpu.x[20]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d32de0 cbz x0, #0x10d32df4
if (cpu.x[0]) == 0 {pc=0x10d32df4;continue;}pc=0x10d32de4;continue;
}
0x10d32de4=>{
// 0x10d32de4 ldr x22, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[22]=(v0) as u64;}
// 0x10d32de8 bl #0x1187d2c0
cpu.x[30]=0x10d32dec;super::dispatch(cpu,rt,0x1187d2c0)?;
// 0x10d32dec mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d32df0 cbnz x22, #0x10d32de4
if (cpu.x[22]) != 0 {pc=0x10d32de4;continue;}pc=0x10d32df4;continue;
}
0x10d32df4=>{
// 0x10d32df4 ldr x8, [x20, #0x70]
{let address=(cpu.x[20]).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d32df8 str xzr, [x20, #0x78]
{let address=(cpu.x[20]).wrapping_add(0x78u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d32dfc cbz x8, #0x10d32e18
if (cpu.x[8]) == 0 {pc=0x10d32e18;continue;}pc=0x10d32e00;continue;
}
0x10d32e00=>{
// 0x10d32e00 mov x9, xzr
cpu.x[9]=(((0u64))) as u64;
pc=0x10d32e04;continue;
}
0x10d32e04=>{
// 0x10d32e04 ldr x10, [x20, #0x68]
{let address=(cpu.x[20]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10d32e08 str xzr, [x10, x9, lsl #3]
{let address=((cpu.x[10]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[9])) as u64).wrapping_shl(3) as u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d32e0c add x9, x9, #1
{let a=((cpu.x[9]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
// 0x10d32e10 cmp x8, x9
{let a=((cpu.x[8]));let b=((cpu.x[9]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d32e14 b.ne #0x10d32e04
if cpu.condition(1){pc=0x10d32e04;continue;}pc=0x10d32e18;continue;
}
0x10d32e18=>{
// 0x10d32e18 str xzr, [x20, #0x80]
{let address=(cpu.x[20]).wrapping_add(0x80u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10d32e1c;continue;
}
0x10d32e1c=>{
// 0x10d32e1c ldr x0, [x20, #0x78]
{let address=(cpu.x[20]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d32e20 cbz x0, #0x10d32e34
if (cpu.x[0]) == 0 {pc=0x10d32e34;continue;}pc=0x10d32e24;continue;
}
0x10d32e24=>{
// 0x10d32e24 ldr x22, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[22]=(v0) as u64;}
// 0x10d32e28 bl #0x1187d2c0
cpu.x[30]=0x10d32e2c;super::dispatch(cpu,rt,0x1187d2c0)?;
// 0x10d32e2c mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d32e30 cbnz x22, #0x10d32e24
if (cpu.x[22]) != 0 {pc=0x10d32e24;continue;}pc=0x10d32e34;continue;
}
0x10d32e34=>{
// 0x10d32e34 ldr x0, [x20, #0x68]
{let address=(cpu.x[20]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d32e38 str xzr, [x20, #0x68]
{let address=(cpu.x[20]).wrapping_add(0x68u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d32e3c cbz x0, #0x10d32e44
if (cpu.x[0]) == 0 {pc=0x10d32e44;continue;}pc=0x10d32e40;continue;
}
0x10d32e40=>{
// 0x10d32e40 bl #0x1187d2c0
cpu.x[30]=0x10d32e44;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10d32e44;continue;
}
0x10d32e44=>{
// 0x10d32e44 add x0, x20, #0x40
{let a=((cpu.x[20]));let b=((0x40u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d32e48 bl #0x1142fcc4
cpu.x[30]=0x10d32e4c;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
// 0x10d32e4c ldr x0, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d32e50 cbz x0, #0x10d32e58
if (cpu.x[0]) == 0 {pc=0x10d32e58;continue;}pc=0x10d32e54;continue;
}
0x10d32e54=>{
// 0x10d32e54 bl #0x1187d360
cpu.x[30]=0x10d32e58;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x10d32e58;continue;
}
0x10d32e58=>{
// 0x10d32e58 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d32e5c bl #0x1187d2c0
cpu.x[30]=0x10d32e60;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10d32e60;continue;
}
0x10d32e60=>{
// 0x10d32e60 str xzr, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10d32e64;continue;
}
0x10d32e64=>{
// 0x10d32e64 ldr x0, [x19, #0x2a8]
{let address=(cpu.x[19]).wrapping_add(0x2a8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d32e68 cbz x0, #0x10d32ea4
if (cpu.x[0]) == 0 {pc=0x10d32ea4;continue;}pc=0x10d32e6c;continue;
}
0x10d32e6c=>{
// 0x10d32e6c bl #0x10f1928c
cpu.x[30]=0x10d32e70;super::dispatch(cpu,rt,0x10f1928c)?;
// 0x10d32e70 ldr x20, [x19, #0x2a8]
{let address=(cpu.x[19]).wrapping_add(0x2a8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10d32e74 cbz x20, #0x10d32ea0
if (cpu.x[20]) == 0 {pc=0x10d32ea0;continue;}pc=0x10d32e78;continue;
}
0x10d32e78=>{
// 0x10d32e78 add x0, x20, #0x58
{let a=((cpu.x[20]));let b=((0x58u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d32e7c bl #0x1142fcc4
cpu.x[30]=0x10d32e80;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
// 0x10d32e80 add x0, x20, #0x48
{let a=((cpu.x[20]));let b=((0x48u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d32e84 bl #0x1142fcc4
cpu.x[30]=0x10d32e88;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
// 0x10d32e88 add x0, x20, #0x38
{let a=((cpu.x[20]));let b=((0x38u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d32e8c bl #0x1142fcc4
cpu.x[30]=0x10d32e90;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
// 0x10d32e90 add x0, x20, #0x10
{let a=((cpu.x[20]));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d32e94 bl #0x1142fcc4
cpu.x[30]=0x10d32e98;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
// 0x10d32e98 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d32e9c bl #0x1187d2c0
cpu.x[30]=0x10d32ea0;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10d32ea0;continue;
}
0x10d32ea0=>{
// 0x10d32ea0 str xzr, [x19, #0x2a8]
{let address=(cpu.x[19]).wrapping_add(0x2a8u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10d32ea4;continue;
}
0x10d32ea4=>{
// 0x10d32ea4 mov x23, x19
cpu.x[23]=(((cpu.x[19]))) as u64;
// 0x10d32ea8 ldr x0, [x23, #0x30]!
{let address=(cpu.x[23]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;cpu.x[23]=(address) as u64;}
// 0x10d32eac cbz x0, #0x10d32eb4
if (cpu.x[0]) == 0 {pc=0x10d32eb4;continue;}pc=0x10d32eb0;continue;
}
0x10d32eb0=>{
// 0x10d32eb0 bl #0x1187d2c0
cpu.x[30]=0x10d32eb4;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10d32eb4;continue;
}
0x10d32eb4=>{
// 0x10d32eb4 mov x22, x19
cpu.x[22]=(((cpu.x[19]))) as u64;
// 0x10d32eb8 ldr x0, [x22, #0x88]!
{let address=(cpu.x[22]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;cpu.x[22]=(address) as u64;}
// 0x10d32ebc stur xzr, [x22, #-0x58]
{let address=(cpu.x[22]).wrapping_add(0xffffffffffffffa8u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d32ec0 cbz x0, #0x10d32ee8
if (cpu.x[0]) == 0 {pc=0x10d32ee8;continue;}pc=0x10d32ec4;continue;
}
0x10d32ec4=>{
// 0x10d32ec4 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d32ec8 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d32ecc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d32ed0;super::dispatch(cpu,rt,target)?;}
pc=0x10d32ed0;continue;
}
0x10d32ed0=>{
// 0x10d32ed0 ldr x0, [x22]
{let address=(cpu.x[22]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d32ed4 cbz x0, #0x10d32ee4
if (cpu.x[0]) == 0 {pc=0x10d32ee4;continue;}pc=0x10d32ed8;continue;
}
0x10d32ed8=>{
// 0x10d32ed8 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d32edc ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d32ee0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d32ee4;super::dispatch(cpu,rt,target)?;}
pc=0x10d32ee4;continue;
}
0x10d32ee4=>{
// 0x10d32ee4 str xzr, [x22]
{let address=(cpu.x[22]).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10d32ee8;continue;
}
0x10d32ee8=>{
// 0x10d32ee8 ldr x0, [x19, #0x38]
{let address=(cpu.x[19]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d32eec cbz x0, #0x10d32ef4
if (cpu.x[0]) == 0 {pc=0x10d32ef4;continue;}pc=0x10d32ef0;continue;
}
0x10d32ef0=>{
// 0x10d32ef0 bl #0x1187d2c0
cpu.x[30]=0x10d32ef4;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10d32ef4;continue;
}
0x10d32ef4=>{
// 0x10d32ef4 ldr x0, [x19, #0x90]
{let address=(cpu.x[19]).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d32ef8 str xzr, [x19, #0x38]
{let address=(cpu.x[19]).wrapping_add(0x38u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d32efc cbz x0, #0x10d32f24
if (cpu.x[0]) == 0 {pc=0x10d32f24;continue;}pc=0x10d32f00;continue;
}
0x10d32f00=>{
// 0x10d32f00 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d32f04 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d32f08 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d32f0c;super::dispatch(cpu,rt,target)?;}
pc=0x10d32f0c;continue;
}
0x10d32f0c=>{
// 0x10d32f0c ldr x0, [x19, #0x90]
{let address=(cpu.x[19]).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d32f10 cbz x0, #0x10d32f20
if (cpu.x[0]) == 0 {pc=0x10d32f20;continue;}pc=0x10d32f14;continue;
}
0x10d32f14=>{
// 0x10d32f14 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d32f18 ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d32f1c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d32f20;super::dispatch(cpu,rt,target)?;}
pc=0x10d32f20;continue;
}
0x10d32f20=>{
// 0x10d32f20 str xzr, [x19, #0x90]
{let address=(cpu.x[19]).wrapping_add(0x90u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10d32f24;continue;
}
0x10d32f24=>{
// 0x10d32f24 ldr x0, [x19, #0x40]
{let address=(cpu.x[19]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d32f28 cbz x0, #0x10d32f30
if (cpu.x[0]) == 0 {pc=0x10d32f30;continue;}pc=0x10d32f2c;continue;
}
0x10d32f2c=>{
// 0x10d32f2c bl #0x1187d2c0
cpu.x[30]=0x10d32f30;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10d32f30;continue;
}
0x10d32f30=>{
// 0x10d32f30 ldr x0, [x19, #0x98]
{let address=(cpu.x[19]).wrapping_add(0x98u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d32f34 str xzr, [x19, #0x40]
{let address=(cpu.x[19]).wrapping_add(0x40u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d32f38 cbz x0, #0x10d32f60
if (cpu.x[0]) == 0 {pc=0x10d32f60;continue;}pc=0x10d32f3c;continue;
}
0x10d32f3c=>{
// 0x10d32f3c ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d32f40 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d32f44 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d32f48;super::dispatch(cpu,rt,target)?;}
pc=0x10d32f48;continue;
}
0x10d32f48=>{
// 0x10d32f48 ldr x0, [x19, #0x98]
{let address=(cpu.x[19]).wrapping_add(0x98u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d32f4c cbz x0, #0x10d32f5c
if (cpu.x[0]) == 0 {pc=0x10d32f5c;continue;}pc=0x10d32f50;continue;
}
0x10d32f50=>{
// 0x10d32f50 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d32f54 ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d32f58 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d32f5c;super::dispatch(cpu,rt,target)?;}
pc=0x10d32f5c;continue;
}
0x10d32f5c=>{
// 0x10d32f5c str xzr, [x19, #0x98]
{let address=(cpu.x[19]).wrapping_add(0x98u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10d32f60;continue;
}
0x10d32f60=>{
// 0x10d32f60 ldr x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d32f64 cbz x0, #0x10d32f6c
if (cpu.x[0]) == 0 {pc=0x10d32f6c;continue;}pc=0x10d32f68;continue;
}
0x10d32f68=>{
// 0x10d32f68 bl #0x1187d2c0
cpu.x[30]=0x10d32f6c;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10d32f6c;continue;
}
0x10d32f6c=>{
// 0x10d32f6c ldr x0, [x19, #0xa0]
{let address=(cpu.x[19]).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d32f70 str xzr, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d32f74 cbz x0, #0x10d32f9c
if (cpu.x[0]) == 0 {pc=0x10d32f9c;continue;}pc=0x10d32f78;continue;
}
0x10d32f78=>{
// 0x10d32f78 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d32f7c ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d32f80 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d32f84;super::dispatch(cpu,rt,target)?;}
pc=0x10d32f84;continue;
}
0x10d32f84=>{
// 0x10d32f84 ldr x0, [x19, #0xa0]
{let address=(cpu.x[19]).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d32f88 cbz x0, #0x10d32f98
if (cpu.x[0]) == 0 {pc=0x10d32f98;continue;}pc=0x10d32f8c;continue;
}
0x10d32f8c=>{
// 0x10d32f8c ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d32f90 ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d32f94 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d32f98;super::dispatch(cpu,rt,target)?;}
pc=0x10d32f98;continue;
}
0x10d32f98=>{
// 0x10d32f98 str xzr, [x19, #0xa0]
{let address=(cpu.x[19]).wrapping_add(0xa0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10d32f9c;continue;
}
0x10d32f9c=>{
// 0x10d32f9c ldr x0, [x19, #0x50]
{let address=(cpu.x[19]).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d32fa0 cbz x0, #0x10d32fa8
if (cpu.x[0]) == 0 {pc=0x10d32fa8;continue;}pc=0x10d32fa4;continue;
}
0x10d32fa4=>{
// 0x10d32fa4 bl #0x1187d2c0
cpu.x[30]=0x10d32fa8;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10d32fa8;continue;
}
0x10d32fa8=>{
// 0x10d32fa8 ldr x0, [x19, #0xa8]
{let address=(cpu.x[19]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d32fac str xzr, [x19, #0x50]
{let address=(cpu.x[19]).wrapping_add(0x50u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d32fb0 cbz x0, #0x10d32fd8
if (cpu.x[0]) == 0 {pc=0x10d32fd8;continue;}pc=0x10d32fb4;continue;
}
0x10d32fb4=>{
// 0x10d32fb4 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d32fb8 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d32fbc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d32fc0;super::dispatch(cpu,rt,target)?;}
pc=0x10d32fc0;continue;
}
0x10d32fc0=>{
// 0x10d32fc0 ldr x0, [x19, #0xa8]
{let address=(cpu.x[19]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d32fc4 cbz x0, #0x10d32fd4
if (cpu.x[0]) == 0 {pc=0x10d32fd4;continue;}pc=0x10d32fc8;continue;
}
0x10d32fc8=>{
// 0x10d32fc8 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d32fcc ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d32fd0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d32fd4;super::dispatch(cpu,rt,target)?;}
pc=0x10d32fd4;continue;
}
0x10d32fd4=>{
// 0x10d32fd4 str xzr, [x19, #0xa8]
{let address=(cpu.x[19]).wrapping_add(0xa8u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10d32fd8;continue;
}
0x10d32fd8=>{
// 0x10d32fd8 ldr x0, [x19, #0x58]
{let address=(cpu.x[19]).wrapping_add(0x58u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d32fdc cbz x0, #0x10d32fe4
if (cpu.x[0]) == 0 {pc=0x10d32fe4;continue;}pc=0x10d32fe0;continue;
}
0x10d32fe0=>{
// 0x10d32fe0 bl #0x1187d2c0
cpu.x[30]=0x10d32fe4;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10d32fe4;continue;
}
0x10d32fe4=>{
// 0x10d32fe4 ldr x0, [x19, #0xb0]
{let address=(cpu.x[19]).wrapping_add(0xb0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d32fe8 str xzr, [x19, #0x58]
{let address=(cpu.x[19]).wrapping_add(0x58u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d32fec cbz x0, #0x10d33014
if (cpu.x[0]) == 0 {pc=0x10d33014;continue;}pc=0x10d32ff0;continue;
}
0x10d32ff0=>{
// 0x10d32ff0 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d32ff4 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d32ff8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d32ffc;super::dispatch(cpu,rt,target)?;}
pc=0x10d32ffc;continue;
}
0x10d32ffc=>{
// 0x10d32ffc ldr x0, [x19, #0xb0]
{let address=(cpu.x[19]).wrapping_add(0xb0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
pc=0x10d33000;continue;
}
0x10d33000=>{
// 0x10d33000 cbz x0, #0x10d33010
if (cpu.x[0]) == 0 {pc=0x10d33010;continue;}pc=0x10d33004;continue;
}
0x10d33004=>{
// 0x10d33004 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d33008 ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3300c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d33010;super::dispatch(cpu,rt,target)?;}
pc=0x10d33010;continue;
}
0x10d33010=>{
// 0x10d33010 str xzr, [x19, #0xb0]
{let address=(cpu.x[19]).wrapping_add(0xb0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10d33014;continue;
}
0x10d33014=>{
// 0x10d33014 ldr x20, [x19, #0x110]
{let address=(cpu.x[19]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10d33018 cbz x20, #0x10d33030
if (cpu.x[20]) == 0 {pc=0x10d33030;continue;}pc=0x10d3301c;continue;
}
0x10d3301c=>{
// 0x10d3301c ldr x0, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d33020 cbz x0, #0x10d33028
if (cpu.x[0]) == 0 {pc=0x10d33028;continue;}pc=0x10d33024;continue;
}
0x10d33024=>{
// 0x10d33024 bl #0x1187d360
cpu.x[30]=0x10d33028;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x10d33028;continue;
}
0x10d33028=>{
// 0x10d33028 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3302c bl #0x1187d2c0
cpu.x[30]=0x10d33030;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10d33030;continue;
}
0x10d33030=>{
// 0x10d33030 ldr x20, [x19, #0x118]
{let address=(cpu.x[19]).wrapping_add(0x118u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10d33034 str xzr, [x19, #0x110]
{let address=(cpu.x[19]).wrapping_add(0x110u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d33038 cbz x20, #0x10d33050
if (cpu.x[20]) == 0 {pc=0x10d33050;continue;}pc=0x10d3303c;continue;
}
0x10d3303c=>{
// 0x10d3303c ldr x0, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d33040 cbz x0, #0x10d33048
if (cpu.x[0]) == 0 {pc=0x10d33048;continue;}pc=0x10d33044;continue;
}
0x10d33044=>{
// 0x10d33044 bl #0x1187d360
cpu.x[30]=0x10d33048;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x10d33048;continue;
}
0x10d33048=>{
// 0x10d33048 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3304c bl #0x1187d2c0
cpu.x[30]=0x10d33050;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10d33050;continue;
}
0x10d33050=>{
// 0x10d33050 mov x24, xzr
cpu.x[24]=(((0u64))) as u64;
// 0x10d33054 str xzr, [x19, #0x118]
{let address=(cpu.x[19]).wrapping_add(0x118u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d33058 b #0x10d33074
pc=0x10d33074;continue;
}
0x10d3305c=>{
// 0x10d3305c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d33060 bl #0x1187d2c0
cpu.x[30]=0x10d33064;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10d33064;continue;
}
0x10d33064=>{
// 0x10d33064 str xzr, [x25, #0x128]
{let address=(cpu.x[25]).wrapping_add(0x128u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10d33068;continue;
}
0x10d33068=>{
// 0x10d33068 add x24, x24, #1
{let a=((cpu.x[24]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[24]=(result) as u64;}
// 0x10d3306c cmp x24, #6
{let a=((cpu.x[24]));let b=((0x6u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d33070 b.eq #0x10d3317c
if cpu.condition(0){pc=0x10d3317c;continue;}pc=0x10d33074;continue;
}
0x10d33074=>{
// 0x10d33074 add x25, x19, x24, lsl #3
{let a=((cpu.x[19]));let b=(((cpu.x[24])) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[25]=(result) as u64;}
// 0x10d33078 ldr x0, [x25, #0x208]
{let address=(cpu.x[25]).wrapping_add(0x208u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3307c cbz x0, #0x10d330a4
if (cpu.x[0]) == 0 {pc=0x10d330a4;continue;}pc=0x10d33080;continue;
}
0x10d33080=>{
// 0x10d33080 add x26, x25, #0x208
{let a=((cpu.x[25]));let b=((0x208u64));let result=a.wrapping_add(b);cpu.x[26]=(result) as u64;}
// 0x10d33084 bl #0x10d497f4
cpu.x[30]=0x10d33088;super::chunk3::f_10d497f4(cpu,rt,0x10d497f4)?;
pc=0x10d33088;continue;
}
0x10d33088=>{
// 0x10d33088 ldr x20, [x26]
{let address=(cpu.x[26]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10d3308c cbz x20, #0x10d330a0
if (cpu.x[20]) == 0 {pc=0x10d330a0;continue;}pc=0x10d33090;continue;
}
0x10d33090=>{
// 0x10d33090 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d33094 bl #0x10d3345c
cpu.x[30]=0x10d33098;super::chunk4::f_10d3345c(cpu,rt,0x10d3345c)?;
pc=0x10d33098;continue;
}
0x10d33098=>{
// 0x10d33098 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3309c bl #0x1187d2c0
cpu.x[30]=0x10d330a0;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10d330a0;continue;
}
0x10d330a0=>{
// 0x10d330a0 str xzr, [x26]
{let address=(cpu.x[26]).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10d330a4;continue;
}
0x10d330a4=>{
// 0x10d330a4 ldr x0, [x25, #0x270]
{let address=(cpu.x[25]).wrapping_add(0x270u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d330a8 cbz x0, #0x10d330c4
if (cpu.x[0]) == 0 {pc=0x10d330c4;continue;}pc=0x10d330ac;continue;
}
0x10d330ac=>{
// 0x10d330ac add x20, x25, #0x270
{let a=((cpu.x[25]));let b=((0x270u64));let result=a.wrapping_add(b);cpu.x[20]=(result) as u64;}
// 0x10d330b0 bl #0x10d48f20
cpu.x[30]=0x10d330b4;super::chunk11::f_10d48f20(cpu,rt,0x10d48f20)?;
pc=0x10d330b4;continue;
}
0x10d330b4=>{
// 0x10d330b4 ldr x0, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d330b8 cbz x0, #0x10d330c0
if (cpu.x[0]) == 0 {pc=0x10d330c0;continue;}pc=0x10d330bc;continue;
}
0x10d330bc=>{
// 0x10d330bc bl #0x1187d2c0
cpu.x[30]=0x10d330c0;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10d330c0;continue;
}
0x10d330c0=>{
// 0x10d330c0 str xzr, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10d330c4;continue;
}
0x10d330c4=>{
// 0x10d330c4 ldr x0, [x25, #0x240]
{let address=(cpu.x[25]).wrapping_add(0x240u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d330c8 cbz x0, #0x10d330f4
if (cpu.x[0]) == 0 {pc=0x10d330f4;continue;}pc=0x10d330cc;continue;
}
0x10d330cc=>{
// 0x10d330cc add x26, x25, #0x240
{let a=((cpu.x[25]));let b=((0x240u64));let result=a.wrapping_add(b);cpu.x[26]=(result) as u64;}
// 0x10d330d0 bl #0x10d48870
cpu.x[30]=0x10d330d4;super::chunk15::f_10d48870(cpu,rt,0x10d48870)?;
pc=0x10d330d4;continue;
}
0x10d330d4=>{
// 0x10d330d4 ldr x20, [x26]
{let address=(cpu.x[26]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10d330d8 cbz x20, #0x10d330f0
if (cpu.x[20]) == 0 {pc=0x10d330f0;continue;}pc=0x10d330dc;continue;
}
0x10d330dc=>{
// 0x10d330dc ldr x0, [x20, #0x20]
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d330e0 cbz x0, #0x10d330e8
if (cpu.x[0]) == 0 {pc=0x10d330e8;continue;}pc=0x10d330e4;continue;
}
0x10d330e4=>{
// 0x10d330e4 bl #0x1187d360
cpu.x[30]=0x10d330e8;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x10d330e8;continue;
}
0x10d330e8=>{
// 0x10d330e8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d330ec bl #0x1187d2c0
cpu.x[30]=0x10d330f0;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10d330f0;continue;
}
0x10d330f0=>{
// 0x10d330f0 str xzr, [x26]
{let address=(cpu.x[26]).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10d330f4;continue;
}
0x10d330f4=>{
// 0x10d330f4 ldr x0, [x25, #0x128]
{let address=(cpu.x[25]).wrapping_add(0x128u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d330f8 cbz x0, #0x10d33068
if (cpu.x[0]) == 0 {pc=0x10d33068;continue;}pc=0x10d330fc;continue;
}
0x10d330fc=>{
// 0x10d330fc bl #0x10d58d00
cpu.x[30]=0x10d33100;super::dispatch(cpu,rt,0x10d58d00)?;
// 0x10d33100 ldr x20, [x25, #0x128]
{let address=(cpu.x[25]).wrapping_add(0x128u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10d33104 cbz x20, #0x10d33064
if (cpu.x[20]) == 0 {pc=0x10d33064;continue;}pc=0x10d33108;continue;
}
0x10d33108=>{
// 0x10d33108 ldr x8, [x20, #0x18]
{let address=(cpu.x[20]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3310c cbz x8, #0x10d33150
if (cpu.x[8]) == 0 {pc=0x10d33150;continue;}pc=0x10d33110;continue;
}
0x10d33110=>{
// 0x10d33110 ldr x0, [x20, #0x10]
{let address=(cpu.x[20]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d33114 cbz x0, #0x10d33128
if (cpu.x[0]) == 0 {pc=0x10d33128;continue;}pc=0x10d33118;continue;
}
0x10d33118=>{
// 0x10d33118 ldr x26, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[26]=(v0) as u64;}
// 0x10d3311c bl #0x1187d2c0
cpu.x[30]=0x10d33120;super::dispatch(cpu,rt,0x1187d2c0)?;
// 0x10d33120 mov x0, x26
cpu.x[0]=(((cpu.x[26]))) as u64;
// 0x10d33124 cbnz x26, #0x10d33118
if (cpu.x[26]) != 0 {pc=0x10d33118;continue;}pc=0x10d33128;continue;
}
0x10d33128=>{
// 0x10d33128 ldr x8, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3312c str xzr, [x20, #0x10]
{let address=(cpu.x[20]).wrapping_add(0x10u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d33130 cbz x8, #0x10d3314c
if (cpu.x[8]) == 0 {pc=0x10d3314c;continue;}pc=0x10d33134;continue;
}
0x10d33134=>{
// 0x10d33134 mov x9, xzr
cpu.x[9]=(((0u64))) as u64;
pc=0x10d33138;continue;
}
0x10d33138=>{
// 0x10d33138 ldr x10, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10d3313c str xzr, [x10, x9, lsl #3]
{let address=((cpu.x[10]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[9])) as u64).wrapping_shl(3) as u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d33140 add x9, x9, #1
{let a=((cpu.x[9]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
// 0x10d33144 cmp x8, x9
{let a=((cpu.x[8]));let b=((cpu.x[9]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d33148 b.ne #0x10d33138
if cpu.condition(1){pc=0x10d33138;continue;}pc=0x10d3314c;continue;
}
0x10d3314c=>{
// 0x10d3314c str xzr, [x20, #0x18]
{let address=(cpu.x[20]).wrapping_add(0x18u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10d33150;continue;
}
0x10d33150=>{
// 0x10d33150 ldr x0, [x20, #0x10]
{let address=(cpu.x[20]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d33154 cbz x0, #0x10d33168
if (cpu.x[0]) == 0 {pc=0x10d33168;continue;}pc=0x10d33158;continue;
}
0x10d33158=>{
// 0x10d33158 ldr x26, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[26]=(v0) as u64;}
// 0x10d3315c bl #0x1187d2c0
cpu.x[30]=0x10d33160;super::dispatch(cpu,rt,0x1187d2c0)?;
// 0x10d33160 mov x0, x26
cpu.x[0]=(((cpu.x[26]))) as u64;
// 0x10d33164 cbnz x26, #0x10d33158
if (cpu.x[26]) != 0 {pc=0x10d33158;continue;}pc=0x10d33168;continue;
}
0x10d33168=>{
// 0x10d33168 ldr x0, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3316c str xzr, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d33170 cbz x0, #0x10d3305c
if (cpu.x[0]) == 0 {pc=0x10d3305c;continue;}pc=0x10d33174;continue;
}
0x10d33174=>{
// 0x10d33174 bl #0x1187d2c0
cpu.x[30]=0x10d33178;super::dispatch(cpu,rt,0x1187d2c0)?;
// 0x10d33178 b #0x10d3305c
pc=0x10d3305c;continue;
}
0x10d3317c=>{
// 0x10d3317c ldr x0, [x19, #0x238]
{let address=(cpu.x[19]).wrapping_add(0x238u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d33180 cbz x0, #0x10d331a8
if (cpu.x[0]) == 0 {pc=0x10d331a8;continue;}pc=0x10d33184;continue;
}
0x10d33184=>{
// 0x10d33184 bl #0x10d525a8
cpu.x[30]=0x10d33188;super::chunk0::f_10d525a8(cpu,rt,0x10d525a8)?;
pc=0x10d33188;continue;
}
0x10d33188=>{
// 0x10d33188 ldr x20, [x19, #0x238]
{let address=(cpu.x[19]).wrapping_add(0x238u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10d3318c cbz x20, #0x10d331a4
if (cpu.x[20]) == 0 {pc=0x10d331a4;continue;}pc=0x10d33190;continue;
}
0x10d33190=>{
// 0x10d33190 ldr x0, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d33194 cbz x0, #0x10d3319c
if (cpu.x[0]) == 0 {pc=0x10d3319c;continue;}pc=0x10d33198;continue;
}
0x10d33198=>{
// 0x10d33198 bl #0x1187d360
cpu.x[30]=0x10d3319c;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x10d3319c;continue;
}
0x10d3319c=>{
// 0x10d3319c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d331a0 bl #0x1187d2c0
cpu.x[30]=0x10d331a4;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10d331a4;continue;
}
0x10d331a4=>{
// 0x10d331a4 str xzr, [x19, #0x238]
{let address=(cpu.x[19]).wrapping_add(0x238u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10d331a8;continue;
}
0x10d331a8=>{
// 0x10d331a8 ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d331ac cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d331b0 b.lt #0x10d331fc
if cpu.condition(11){pc=0x10d331fc;continue;}pc=0x10d331b4;continue;
}
0x10d331b4=>{
// 0x10d331b4 mov x24, xzr
cpu.x[24]=(((0u64))) as u64;
// 0x10d331b8 b #0x10d331c8
pc=0x10d331c8;continue;
}
0x10d331bc=>{
// 0x10d331bc add x24, x24, #1
{let a=((cpu.x[24]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[24]=(result) as u64;}
// 0x10d331c0 cmp x24, w8, sxtw
{let a=((cpu.x[24]));let b=(((cpu.x[8] as u32 as u64) as i32 as i64 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d331c4 b.ge #0x10d331fc
if cpu.condition(10){pc=0x10d331fc;continue;}pc=0x10d331c8;continue;
}
0x10d331c8=>{
// 0x10d331c8 ldr x9, [x19, #0x2e8]
{let address=(cpu.x[19]).wrapping_add(0x2e8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d331cc ldrsw x10, [x9, #0xc]
{let address=(cpu.x[9]).wrapping_add(0xcu64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[10]=(v0) as u64;}
// 0x10d331d0 cmp x24, x10
{let a=((cpu.x[24]));let b=((cpu.x[10]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d331d4 b.ge #0x10d331bc
if cpu.condition(10){pc=0x10d331bc;continue;}pc=0x10d331d8;continue;
}
0x10d331d8=>{
// 0x10d331d8 ldr x9, [x9]
{let address=(cpu.x[9]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d331dc ldr x20, [x9, x24, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[24])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10d331e0 cbz x20, #0x10d331bc
if (cpu.x[20]) == 0 {pc=0x10d331bc;continue;}pc=0x10d331e4;continue;
}
0x10d331e4=>{
// 0x10d331e4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d331e8 bl #0x11027538
cpu.x[30]=0x10d331ec;super::chunk6::f_11027538(cpu,rt,0x11027538)?;
pc=0x10d331ec;continue;
}
0x10d331ec=>{
// 0x10d331ec mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d331f0 bl #0x1187d2c0
cpu.x[30]=0x10d331f4;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10d331f4;continue;
}
0x10d331f4=>{
// 0x10d331f4 ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d331f8 b #0x10d331bc
pc=0x10d331bc;continue;
}
0x10d331fc=>{
// 0x10d331fc ldr x20, [x19, #0x2e8]
{let address=(cpu.x[19]).wrapping_add(0x2e8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10d33200 cbz x20, #0x10d33218
if (cpu.x[20]) == 0 {pc=0x10d33218;continue;}pc=0x10d33204;continue;
}
0x10d33204=>{
// 0x10d33204 ldr x0, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d33208 cbz x0, #0x10d33210
if (cpu.x[0]) == 0 {pc=0x10d33210;continue;}pc=0x10d3320c;continue;
}
0x10d3320c=>{
// 0x10d3320c bl #0x1187d360
cpu.x[30]=0x10d33210;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x10d33210;continue;
}
0x10d33210=>{
// 0x10d33210 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d33214 bl #0x1187d2c0
cpu.x[30]=0x10d33218;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10d33218;continue;
}
0x10d33218=>{
// 0x10d33218 ldr x0, [x19, #0x1b0]
{let address=(cpu.x[19]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3321c str xzr, [x19, #0x2e8]
{let address=(cpu.x[19]).wrapping_add(0x2e8u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d33220 cbz x0, #0x10d33238
if (cpu.x[0]) == 0 {pc=0x10d33238;continue;}pc=0x10d33224;continue;
}
0x10d33224=>{
// 0x10d33224 bl #0x10d17d20
cpu.x[30]=0x10d33228;super::chunk1::f_10d17d20(cpu,rt,0x10d17d20)?;
pc=0x10d33228;continue;
}
0x10d33228=>{
// 0x10d33228 ldr x0, [x19, #0x1b0]
{let address=(cpu.x[19]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3322c cbz x0, #0x10d33234
if (cpu.x[0]) == 0 {pc=0x10d33234;continue;}pc=0x10d33230;continue;
}
0x10d33230=>{
// 0x10d33230 bl #0x1187d2c0
cpu.x[30]=0x10d33234;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10d33234;continue;
}
0x10d33234=>{
// 0x10d33234 str xzr, [x19, #0x1b0]
{let address=(cpu.x[19]).wrapping_add(0x1b0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10d33238;continue;
}
0x10d33238=>{
// 0x10d33238 ldr x0, [x19, #0x2b0]
{let address=(cpu.x[19]).wrapping_add(0x2b0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3323c add x24, x19, #0x2b0
{let a=((cpu.x[19]));let b=((0x2b0u64));let result=a.wrapping_add(b);cpu.x[24]=(result) as u64;}
// 0x10d33240 cbz x0, #0x10d33264
if (cpu.x[0]) == 0 {pc=0x10d33264;continue;}pc=0x10d33244;continue;
}
0x10d33244=>{
// 0x10d33244 bl #0x10f07810
cpu.x[30]=0x10d33248;super::chunk12::f_10f07810(cpu,rt,0x10f07810)?;
pc=0x10d33248;continue;
}
0x10d33248=>{
// 0x10d33248 ldr x20, [x24]
{let address=(cpu.x[24]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10d3324c cbz x20, #0x10d33260
if (cpu.x[20]) == 0 {pc=0x10d33260;continue;}pc=0x10d33250;continue;
}
0x10d33250=>{
// 0x10d33250 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d33254 bl #0x10d3352c
cpu.x[30]=0x10d33258;super::chunk1::f_10d3352c(cpu,rt,0x10d3352c)?;
pc=0x10d33258;continue;
}
0x10d33258=>{
// 0x10d33258 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3325c bl #0x1187d2c0
cpu.x[30]=0x10d33260;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10d33260;continue;
}
0x10d33260=>{
// 0x10d33260 str xzr, [x24]
{let address=(cpu.x[24]).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10d33264;continue;
}
0x10d33264=>{
// 0x10d33264 ldr x0, [x19, #0x2f8]
{let address=(cpu.x[19]).wrapping_add(0x2f8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d33268 cbz x0, #0x10d33280
if (cpu.x[0]) == 0 {pc=0x10d33280;continue;}pc=0x10d3326c;continue;
}
0x10d3326c=>{
// 0x10d3326c bl #0x10d44cbc
cpu.x[30]=0x10d33270;super::chunk2::f_10d44cbc(cpu,rt,0x10d44cbc)?;
pc=0x10d33270;continue;
}
0x10d33270=>{
// 0x10d33270 ldr x0, [x19, #0x2f8]
{let address=(cpu.x[19]).wrapping_add(0x2f8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d33274 cbz x0, #0x10d3327c
if (cpu.x[0]) == 0 {pc=0x10d3327c;continue;}pc=0x10d33278;continue;
}
0x10d33278=>{
// 0x10d33278 bl #0x1187d2c0
cpu.x[30]=0x10d3327c;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10d3327c;continue;
}
0x10d3327c=>{
// 0x10d3327c str xzr, [x19, #0x2f8]
{let address=(cpu.x[19]).wrapping_add(0x2f8u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10d33280;continue;
}
0x10d33280=>{
// 0x10d33280 ldr x0, [x19, #0x328]
{let address=(cpu.x[19]).wrapping_add(0x328u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d33284 cbz x0, #0x10d332ac
if (cpu.x[0]) == 0 {pc=0x10d332ac;continue;}pc=0x10d33288;continue;
}
0x10d33288=>{
// 0x10d33288 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3328c ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d33290 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d33294;super::dispatch(cpu,rt,target)?;}
pc=0x10d33294;continue;
}
0x10d33294=>{
// 0x10d33294 ldr x0, [x19, #0x328]
{let address=(cpu.x[19]).wrapping_add(0x328u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d33298 cbz x0, #0x10d332a8
if (cpu.x[0]) == 0 {pc=0x10d332a8;continue;}pc=0x10d3329c;continue;
}
0x10d3329c=>{
// 0x10d3329c ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d332a0 ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d332a4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d332a8;super::dispatch(cpu,rt,target)?;}
pc=0x10d332a8;continue;
}
0x10d332a8=>{
// 0x10d332a8 str xzr, [x19, #0x328]
{let address=(cpu.x[19]).wrapping_add(0x328u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10d332ac;continue;
}
0x10d332ac=>{
// 0x10d332ac ldr x0, [x19, #0x330]
{let address=(cpu.x[19]).wrapping_add(0x330u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d332b0 cbz x0, #0x10d332d8
if (cpu.x[0]) == 0 {pc=0x10d332d8;continue;}pc=0x10d332b4;continue;
}
0x10d332b4=>{
// 0x10d332b4 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d332b8 ldr x8, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d332bc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d332c0;super::dispatch(cpu,rt,target)?;}
pc=0x10d332c0;continue;
}
0x10d332c0=>{
// 0x10d332c0 ldr x0, [x19, #0x330]
{let address=(cpu.x[19]).wrapping_add(0x330u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d332c4 cbz x0, #0x10d332d4
if (cpu.x[0]) == 0 {pc=0x10d332d4;continue;}pc=0x10d332c8;continue;
}
0x10d332c8=>{
// 0x10d332c8 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d332cc ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d332d0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d332d4;super::dispatch(cpu,rt,target)?;}
pc=0x10d332d4;continue;
}
0x10d332d4=>{
// 0x10d332d4 str xzr, [x19, #0x330]
{let address=(cpu.x[19]).wrapping_add(0x330u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10d332d8;continue;
}
0x10d332d8=>{
// 0x10d332d8 ldr x0, [x19, #0x338]
{let address=(cpu.x[19]).wrapping_add(0x338u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d332dc cbz x0, #0x10d332f4
if (cpu.x[0]) == 0 {pc=0x10d332f4;continue;}pc=0x10d332e0;continue;
}
0x10d332e0=>{
// 0x10d332e0 bl #0x10d569e0
cpu.x[30]=0x10d332e4;super::chunk11::f_10d569e0(cpu,rt,0x10d569e0)?;
pc=0x10d332e4;continue;
}
0x10d332e4=>{
// 0x10d332e4 ldr x0, [x19, #0x338]
{let address=(cpu.x[19]).wrapping_add(0x338u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d332e8 cbz x0, #0x10d332f0
if (cpu.x[0]) == 0 {pc=0x10d332f0;continue;}pc=0x10d332ec;continue;
}
0x10d332ec=>{
// 0x10d332ec bl #0x1187d2c0
cpu.x[30]=0x10d332f0;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10d332f0;continue;
}
0x10d332f0=>{
// 0x10d332f0 str xzr, [x19, #0x338]
{let address=(cpu.x[19]).wrapping_add(0x338u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10d332f4;continue;
}
0x10d332f4=>{
// 0x10d332f4 ldrsw x8, [x19, #0x36c]
{let address=(cpu.x[19]).wrapping_add(0x36cu64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10d332f8 cbz w8, #0x10d33340
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10d33340;continue;}pc=0x10d332fc;continue;
}
0x10d332fc=>{
// 0x10d332fc ldr x25, [x19, #0x360]
{let address=(cpu.x[19]).wrapping_add(0x360u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[25]=(v0) as u64;}
// 0x10d33300 lsl x26, x8, #3
cpu.x[26]=(((((cpu.x[8]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10d33304 b #0x10d33314
pc=0x10d33314;continue;
}
0x10d33308=>{
// 0x10d33308 subs x26, x26, #8
{let a=((cpu.x[26]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.x[26]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x10d3330c add x25, x25, #8
{let a=((cpu.x[25]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[25]=(result) as u64;}
// 0x10d33310 b.eq #0x10d33340
if cpu.condition(0){pc=0x10d33340;continue;}pc=0x10d33314;continue;
}
0x10d33314=>{
// 0x10d33314 ldr x20, [x25]
{let address=(cpu.x[25]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10d33318 cbz x20, #0x10d33308
if (cpu.x[20]) == 0 {pc=0x10d33308;continue;}pc=0x10d3331c;continue;
}
0x10d3331c=>{
// 0x10d3331c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d33320 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d33324 ldr x8, [x8, #0x28]
{let address=(cpu.x[8]).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d33328 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3332c;super::dispatch(cpu,rt,target)?;}
pc=0x10d3332c;continue;
}
0x10d3332c=>{
// 0x10d3332c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d33330 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d33334 ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d33338 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3333c;super::dispatch(cpu,rt,target)?;}
pc=0x10d3333c;continue;
}
0x10d3333c=>{
// 0x10d3333c b #0x10d33308
pc=0x10d33308;continue;
}
0x10d33340=>{
// 0x10d33340 movi v0.2d, #0000000000000000
cpu.q[0]=(lane_dup(0u128,64,2)) & 0xffffffffffffffffffffffffffffffffu128;
// 0x10d33344 add x8, x19, #0x1bc
{let a=((cpu.x[19]));let b=((0x1bcu64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d33348 movi v1.2d, #0xffffffffffffffff
cpu.q[1]=(lane_dup(18446744073709551615u128,64,2)) & 0xffffffffffffffffffffffffffffffffu128;
// 0x10d3334c str xzr, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d33350 mov x9, #-0x100000000
cpu.x[9]=(((0xffffffff00000000u64))) as u64;
// 0x10d33354 str wzr, [x19, #0x36c]
{let address=(cpu.x[19]).wrapping_add(0x36cu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d33358 strb wzr, [x19, #0x31c]
{let address=(cpu.x[19]).wrapping_add(0x31cu64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10d3335c stur q0, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d33360 mov x8, #-1
cpu.x[8]=(((0xffffffffffffffffu64))) as u64;
// 0x10d33364 str x8, [x19, #0x120]
{let address=(cpu.x[19]).wrapping_add(0x120u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d33368 add x8, x19, #0x1e4
{let a=((cpu.x[19]));let b=((0x1e4u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d3336c str xzr, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d33370 add x8, x19, #0x128
{let a=((cpu.x[19]));let b=((0x128u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d33374 stp q0, q0, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10d33378 stp q0, q0, [x8, #0x20]
{let address=(cpu.x[8]).wrapping_add(0x20u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10d3337c stp q0, q0, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10d33380 stp q0, q0, [x8, #0x60]
{let address=(cpu.x[8]).wrapping_add(0x60u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10d33384 add x8, x19, #0x208
{let a=((cpu.x[19]));let b=((0x208u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d33388 stp q0, q0, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10d3338c stp q0, q0, [x8, #0x30]
{let address=(cpu.x[8]).wrapping_add(0x30u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10d33390 stp q0, q0, [x8, #0x50]
{let address=(cpu.x[8]).wrapping_add(0x50u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10d33394 stp q0, q0, [x8, #0x70]
{let address=(cpu.x[8]).wrapping_add(0x70u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10d33398 str q0, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d3339c mov w8, #2
cpu.x[8]=(((0x2u64))) as u32 as u64;
// 0x10d333a0 str d1, [x19, #0x2e0]
{let address=(cpu.x[19]).wrapping_add(0x2e0u64);let v0=(cpu.q[1] & 0xffffffffffffffffu128) as u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d333a4 mvni v1.2s, #0x63
cpu.q[1]=(lane_dup(99u128,32,2) ^ 0xffffffffffffffffu128) & 0xffffffffffffffffu128;
// 0x10d333a8 str w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d333ac add x8, x19, #0x1fc
{let a=((cpu.x[19]));let b=((0x1fcu64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d333b0 str x9, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=cpu.x[9];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d333b4 add x8, x19, #0x2c8
{let a=((cpu.x[19]));let b=((0x2c8u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d333b8 str xzr, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d333bc str q0, [x19, #0x110]
{let address=(cpu.x[19]).wrapping_add(0x110u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d333c0 stp q0, q0, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10d333c4 str q0, [x23, #0x20]
{let address=(cpu.x[23]).wrapping_add(0x20u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d333c8 str wzr, [x19, #0x1b8]
{let address=(cpu.x[19]).wrapping_add(0x1b8u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d333cc str xzr, [x19, #0x1f0]
{let address=(cpu.x[19]).wrapping_add(0x1f0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d333d0 str xzr, [x19, #0x1b0]
{let address=(cpu.x[19]).wrapping_add(0x1b0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d333d4 stur q0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d333d8 str wzr, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d333dc stur xzr, [x19, #0x74]
{let address=(cpu.x[19]).wrapping_add(0x74u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d333e0 stur xzr, [x19, #0x7a]
{let address=(cpu.x[19]).wrapping_add(0x7au64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d333e4 stur q0, [x19, #0x64]
{let address=(cpu.x[19]).wrapping_add(0x64u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d333e8 str xzr, [x22, #0x80]
{let address=(cpu.x[22]).wrapping_add(0x80u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d333ec stp q0, q0, [x22]
{let address=(cpu.x[22]).wrapping_add(0x0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10d333f0 stp q0, q0, [x22, #0x20]
{let address=(cpu.x[22]).wrapping_add(0x20u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10d333f4 stp q0, q0, [x22, #0x40]
{let address=(cpu.x[22]).wrapping_add(0x40u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10d333f8 stp q0, q0, [x22, #0x60]
{let address=(cpu.x[22]).wrapping_add(0x60u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10d333fc str xzr, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10d33400;continue;
}
0x10d33400=>{
// 0x10d33400 stur wzr, [x21, #7]
{let address=(cpu.x[21]).wrapping_add(0x7u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d33404 str xzr, [x19, #0x298]
{let address=(cpu.x[19]).wrapping_add(0x298u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d33408 str xzr, [x19, #0x2f8]
{let address=(cpu.x[19]).wrapping_add(0x2f8u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d3340c str q0, [x24]
{let address=(cpu.x[24]).wrapping_add(0x0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d33410 strb wzr, [x19, #0x1f8]
{let address=(cpu.x[19]).wrapping_add(0x1f8u64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10d33414 str wzr, [x19, #0x2c0]
{let address=(cpu.x[19]).wrapping_add(0x2c0u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d33418 strb wzr, [x19, #0x2c4]
{let address=(cpu.x[19]).wrapping_add(0x2c4u64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10d3341c str q0, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d33420 str xzr, [x19, #0x2d8]
{let address=(cpu.x[19]).wrapping_add(0x2d8u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d33424 str d1, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=(cpu.q[1] & 0xffffffffffffffffu128) as u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d33428 str xzr, [x19, #0x2e8]
{let address=(cpu.x[19]).wrapping_add(0x2e8u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d3342c str q0, [x19, #0x2a0]
{let address=(cpu.x[19]).wrapping_add(0x2a0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d33430 strb wzr, [x19, #0x2f0]
{let address=(cpu.x[19]).wrapping_add(0x2f0u64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10d33434 str wzr, [x19, #0x1a8]
{let address=(cpu.x[19]).wrapping_add(0x1a8u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d33438 str wzr, [x19, #0x310]
{let address=(cpu.x[19]).wrapping_add(0x310u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d3343c str q0, [x19, #0x300]
{let address=(cpu.x[19]).wrapping_add(0x300u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d33440 stp q0, q0, [x19, #0x340]
{let address=(cpu.x[19]).wrapping_add(0x340u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10d33444 ldp x20, x19, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d33448 ldp x22, x21, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d3344c ldp x24, x23, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10d33450 ldp x26, x25, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10d33454 ldp x29, x30, [sp], #0x50
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x50u64)))) as u64;}
// 0x10d33458 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d32d10")),}}}
#[inline]
pub(super) unsafe fn f_10d34434(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d34434=>{
// 0x10d34434 ldr w8, [x0, #0x18]
{let address=(cpu.x[0]).wrapping_add(0x18u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d34438 ldr x9, [x0, #0xc8]
{let address=(cpu.x[0]).wrapping_add(0xc8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d3443c cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d34440 ccmp x9, #0, #4, eq
if cpu.condition(0){{let a=((cpu.x[9]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}}else{cpu.nzcv=4u64 << 28;}
// 0x10d34444 cset w0, ne
cpu.x[0]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x10d34448 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d34434")),}}}
#[inline]
pub(super) unsafe fn f_10d344f0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d344f0=>{
// 0x10d344f0 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d344f4 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d344f8 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d344fc ldr x20, [x0, #0xe0]
{let address=(cpu.x[0]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10d34500 mov x8, x0
cpu.x[8]=(((cpu.x[0]))) as u64;
// 0x10d34504 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10d34508 cbz x20, #0x10d34548
if (cpu.x[20]) == 0 {pc=0x10d34548;continue;}pc=0x10d3450c;continue;
}
0x10d3450c=>{
// 0x10d3450c ldr x19, [x8, #0xe8]
{let address=(cpu.x[8]).wrapping_add(0xe8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10d34510 cbz x19, #0x10d34548
if (cpu.x[19]) == 0 {pc=0x10d34548;continue;}pc=0x10d34514;continue;
}
0x10d34514=>{
// 0x10d34514 ldr x8, [x8, #0x1b0]
{let address=(cpu.x[8]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d34518 ldr x0, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3451c cbz x0, #0x10d34554
if (cpu.x[0]) == 0 {pc=0x10d34554;continue;}pc=0x10d34520;continue;
}
0x10d34520=>{
// 0x10d34520 bl #0x10e2d5a4
cpu.x[30]=0x10d34524;super::chunk4::f_10e2d5a4(cpu,rt,0x10e2d5a4)?;
pc=0x10d34524;continue;
}
0x10d34524=>{
// 0x10d34524 tbz w0, #0, #0x10d34554
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d34554;continue;}pc=0x10d34528;continue;
}
0x10d34528=>{
// 0x10d34528 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d3452c bl #0x10f9c6b8
cpu.x[30]=0x10d34530;super::dispatch(cpu,rt,0x10f9c6b8)?;
// 0x10d34530 mov w20, w0
cpu.x[20]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d34534 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d34538 bl #0x10f9c6b8
cpu.x[30]=0x10d3453c;super::dispatch(cpu,rt,0x10f9c6b8)?;
// 0x10d3453c cmp w20, w0
{let a=((cpu.x[20] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d34540 b.ne #0x10d34588
if cpu.condition(1){pc=0x10d34588;continue;}pc=0x10d34544;continue;
}
0x10d34544=>{
// 0x10d34544 mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
pc=0x10d34548;continue;
}
0x10d34548=>{
// 0x10d34548 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d3454c ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10d34550 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10d34554=>{
// 0x10d34554 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d34558 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d3455c bl #0x10fa4038
cpu.x[30]=0x10d34560;super::chunk15::f_10fa4038(cpu,rt,0x10fa4038)?;
pc=0x10d34560;continue;
}
0x10d34560=>{
// 0x10d34560 mov w20, w0
cpu.x[20]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d34564 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d34568 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10d3456c bl #0x10fa4038
cpu.x[30]=0x10d34570;super::chunk15::f_10fa4038(cpu,rt,0x10fa4038)?;
pc=0x10d34570;continue;
}
0x10d34570=>{
// 0x10d34570 cmp w20, w0
{let a=((cpu.x[20] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d34574 b.ne #0x10d34588
if cpu.condition(1){pc=0x10d34588;continue;}pc=0x10d34578;continue;
}
0x10d34578=>{
// 0x10d34578 mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
// 0x10d3457c ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d34580 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10d34584 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10d34588=>{
// 0x10d34588 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10d3458c ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d34590 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10d34594 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d344f0")),}}}
#[inline]
pub(super) unsafe fn f_10d388b4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d388b4=>{
// 0x10d388b4 stp x29, x30, [sp, #-0x50]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffb0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d388b8 stp x26, x25, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[26];let v1=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d388bc stp x24, x23, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d388c0 stp x22, x21, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d388c4 stp x20, x19, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d388c8 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d388cc mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10d388d0 mov x0, x2
cpu.x[0]=(((cpu.x[2]))) as u64;
// 0x10d388d4 mov w23, w7
cpu.x[23]=(((cpu.x[7] as u32 as u64))) as u32 as u64;
// 0x10d388d8 mov w22, w6
cpu.x[22]=(((cpu.x[6] as u32 as u64))) as u32 as u64;
// 0x10d388dc mov w24, w5
cpu.x[24]=(((cpu.x[5] as u32 as u64))) as u32 as u64;
// 0x10d388e0 mov w25, w4
cpu.x[25]=(((cpu.x[4] as u32 as u64))) as u32 as u64;
// 0x10d388e4 mov w26, w3
cpu.x[26]=(((cpu.x[3] as u32 as u64))) as u32 as u64;
// 0x10d388e8 mov w21, w1
cpu.x[21]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x10d388ec bl #0x10f8a168
cpu.x[30]=0x10d388f0;super::chunk13::f_10f8a168(cpu,rt,0x10f8a168)?;
pc=0x10d388f0;continue;
}
0x10d388f0=>{
// 0x10d388f0 mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10d388f4 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10d388f8 bl #0x10f94050
cpu.x[30]=0x10d388fc;super::chunk10::f_10f94050(cpu,rt,0x10f94050)?;
pc=0x10d388fc;continue;
}
0x10d388fc=>{
// 0x10d388fc cmp w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d38900 and w26, w21, w26
{let a=((cpu.x[21] as u32 as u64));let b=((cpu.x[26] as u32 as u64));let result=a & b;cpu.x[26]=(result) as u32 as u64;}
// 0x10d38904 b.gt #0x10d38924
if cpu.condition(12){pc=0x10d38924;continue;}pc=0x10d38908;continue;
}
0x10d38908=>{
// 0x10d38908 ldr w8, [x20, #0x60]
{let address=(cpu.x[20]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3890c cmp w8, #3
{let a=((cpu.x[8] as u32 as u64));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d38910 b.lt #0x10d38924
if cpu.condition(11){pc=0x10d38924;continue;}pc=0x10d38914;continue;
}
0x10d38914=>{
// 0x10d38914 ldr x0, [x20, #0xc0]
{let address=(cpu.x[20]).wrapping_add(0xc0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d38918 cbz x0, #0x10d38924
if (cpu.x[0]) == 0 {pc=0x10d38924;continue;}pc=0x10d3891c;continue;
}
0x10d3891c=>{
// 0x10d3891c bl #0x10e8b6a4
cpu.x[30]=0x10d38920;super::chunk10::f_10e8b6a4(cpu,rt,0x10e8b6a4)?;
// 0x10d38920 sub w25, w0, w25
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[25] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[25]=(result) as u32 as u64;}
pc=0x10d38924;continue;
}
0x10d38924=>{
// 0x10d38924 ldr w8, [x20, #0x18c]
{let address=(cpu.x[20]).wrapping_add(0x18cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d38928 tst w26, #1
{let a=((cpu.x[26] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.logical_flags(result,32);}
// 0x10d3892c mov w9, #0x1f4
cpu.x[9]=(((0x1f4u64))) as u32 as u64;
// 0x10d38930 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d38934 mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10d38938 sub w8, w8, w24
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[24] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10d3893c csel w8, w8, w24, ne
cpu.x[8]=(if cpu.condition(1){((cpu.x[8] as u32 as u64))}else{((cpu.x[24] as u32 as u64))}) as u32 as u64;
// 0x10d38940 mul w24, w25, w9
cpu.x[24]=((((cpu.x[25] as u32 as u64))).wrapping_mul(((cpu.x[9] as u32 as u64)))) as u32 as u64;
// 0x10d38944 mul w25, w8, w9
cpu.x[25]=((((cpu.x[8] as u32 as u64))).wrapping_mul(((cpu.x[9] as u32 as u64)))) as u32 as u64;
// 0x10d38948 mov w1, w24
cpu.x[1]=(((cpu.x[24] as u32 as u64))) as u32 as u64;
// 0x10d3894c mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10d38950 bl #0x10f94250
cpu.x[30]=0x10d38954;super::chunk3::f_10f94250(cpu,rt,0x10f94250)?;
pc=0x10d38954;continue;
}
0x10d38954=>{
// 0x10d38954 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d38958 mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10d3895c mov w2, w23
cpu.x[2]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10d38960 mov x3, xzr
cpu.x[3]=(((0u64))) as u64;
// 0x10d38964 bl #0x10f9406c
cpu.x[30]=0x10d38968;super::chunk11::f_10f9406c(cpu,rt,0x10f9406c)?;
pc=0x10d38968;continue;
}
0x10d38968=>{
// 0x10d38968 ldr x8, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3896c ldrb w9, [x20, #0x370]
{let address=(cpu.x[20]).wrapping_add(0x370u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10d38970 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10d38974 mov w2, #1
cpu.x[2]=(((0x1u64))) as u32 as u64;
// 0x10d38978 ldr x0, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3897c cbz w9, #0x10d389cc
if (cpu.x[9] as u32 as u64) == 0 {pc=0x10d389cc;continue;}pc=0x10d38980;continue;
}
0x10d38980=>{
// 0x10d38980 bl #0x10f8c284
cpu.x[30]=0x10d38984;super::dispatch(cpu,rt,0x10f8c284)?;
// 0x10d38984 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d38988 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d3898c ldr x8, [x8, #0x190]
{let address=(cpu.x[8]).wrapping_add(0x190u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d38990 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d38994;super::dispatch(cpu,rt,target)?;}
// 0x10d38994 tbnz w0, #0, #0x10d389e4
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10d389e4;continue;}pc=0x10d38998;continue;
}
0x10d38998=>{
// 0x10d38998 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3899c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d389a0 ldr x8, [x8, #0x170]
{let address=(cpu.x[8]).wrapping_add(0x170u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d389a4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d389a8;super::dispatch(cpu,rt,target)?;}
// 0x10d389a8 tbz w0, #0, #0x10d38ad0
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d38ad0;continue;}pc=0x10d389ac;continue;
}
0x10d389ac=>{
// 0x10d389ac add x21, x20, w21, sxtw #3
{let a=((cpu.x[20]));let b=((((cpu.x[21] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[21]=(result) as u64;}
// 0x10d389b0 ldr x8, [x21, #0xe0]!
{let address=(cpu.x[21]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;cpu.x[21]=(address) as u64;}
// 0x10d389b4 cbz x8, #0x10d38b68
if (cpu.x[8]) == 0 {pc=0x10d38b68;continue;}pc=0x10d389b8;continue;
}
0x10d389b8=>{
// 0x10d389b8 adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x10d389bc add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d389c0 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d389c4 bl #0x1128df18
cpu.x[30]=0x10d389c8;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10d389c8 b #0x10d38b68
pc=0x10d38b68;continue;
}
0x10d389cc=>{
// 0x10d389cc bl #0x10f8c274
cpu.x[30]=0x10d389d0;super::chunk5::f_10f8c274(cpu,rt,0x10f8c274)?;
pc=0x10d389d0;continue;
}
0x10d389d0=>{
// 0x10d389d0 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d389d4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d389d8 ldr x8, [x8, #0x190]
{let address=(cpu.x[8]).wrapping_add(0x190u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d389dc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d389e0;super::dispatch(cpu,rt,target)?;}
pc=0x10d389e0;continue;
}
0x10d389e0=>{
// 0x10d389e0 tbz w0, #0, #0x10d38998
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d38998;continue;}pc=0x10d389e4;continue;
}
0x10d389e4=>{
// 0x10d389e4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d389e8 bl #0x10f7e050
cpu.x[30]=0x10d389ec;super::chunk10::f_10f7e050(cpu,rt,0x10f7e050)?;
pc=0x10d389ec;continue;
}
0x10d389ec=>{
// 0x10d389ec mov x23, x0
cpu.x[23]=(((cpu.x[0]))) as u64;
// 0x10d389f0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d389f4 mov w1, w24
cpu.x[1]=(((cpu.x[24] as u32 as u64))) as u32 as u64;
// 0x10d389f8 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10d389fc mov w3, #-1
cpu.x[3]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10d38a00 mov w4, wzr
cpu.x[4]=(((0u64))) as u32 as u64;
// 0x10d38a04 bl #0x10d38d9c
cpu.x[30]=0x10d38a08;super::chunk6::f_10d38d9c(cpu,rt,0x10d38d9c)?;
pc=0x10d38a08;continue;
}
0x10d38a08=>{
// 0x10d38a08 mov w1, w0
cpu.x[1]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d38a0c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d38a10 bl #0x10f85114
cpu.x[30]=0x10d38a14;super::chunk10::f_10f85114(cpu,rt,0x10f85114)?;
pc=0x10d38a14;continue;
}
0x10d38a14=>{
// 0x10d38a14 tbnz w22, #0x1f, #0x10d38a2c
if ((cpu.x[22] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x10d38a2c;continue;}pc=0x10d38a18;continue;
}
0x10d38a18=>{
// 0x10d38a18 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d38a1c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d38a20 ldr x8, [x8, #0x170]
{let address=(cpu.x[8]).wrapping_add(0x170u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d38a24 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d38a28;super::dispatch(cpu,rt,target)?;}
pc=0x10d38a28;continue;
}
0x10d38a28=>{
// 0x10d38a28 tbz w0, #0, #0x10d38a40
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d38a40;continue;}pc=0x10d38a2c;continue;
}
0x10d38a2c=>{
// 0x10d38a2c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d38a30 mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10d38a34 mov x2, x23
cpu.x[2]=(((cpu.x[23]))) as u64;
// 0x10d38a38 bl #0x10d3910c
cpu.x[30]=0x10d38a3c;super::chunk1::f_10d3910c(cpu,rt,0x10d3910c)?;
pc=0x10d38a3c;continue;
}
0x10d38a3c=>{
// 0x10d38a3c b #0x10d38a60
pc=0x10d38a60;continue;
}
0x10d38a40=>{
// 0x10d38a40 ldr x0, [x23, #0x270]
{let address=(cpu.x[23]).wrapping_add(0x270u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d38a44 and w22, w22, #0xff
{let a=((cpu.x[22] as u32 as u64));let b=((0xffu64));let result=a & b;cpu.x[22]=(result) as u32 as u64;}
// 0x10d38a48 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d38a4c ldr x8, [x8, #0x118]
{let address=(cpu.x[8]).wrapping_add(0x118u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d38a50 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d38a54;super::dispatch(cpu,rt,target)?;}
pc=0x10d38a54;continue;
}
0x10d38a54=>{
// 0x10d38a54 mov w8, w0
cpu.x[8]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d38a58 mov w0, w22
cpu.x[0]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10d38a5c bfi w0, w8, #8, #8
cpu.x[0]=((cpu.x[0] as u32 as u64 & !0xff00u64) | ((((cpu.x[8] as u32 as u64)) & 0xffu64) << 8)) as u32 as u64;
pc=0x10d38a60;continue;
}
0x10d38a60=>{
// 0x10d38a60 mov w1, w0
cpu.x[1]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d38a64 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d38a68 bl #0x10f7ed38
cpu.x[30]=0x10d38a6c;super::chunk11::f_10f7ed38(cpu,rt,0x10f7ed38)?;
pc=0x10d38a6c;continue;
}
0x10d38a6c=>{
// 0x10d38a6c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d38a70 bl #0x10f93d3c
cpu.x[30]=0x10d38a74;super::chunk3::f_10f93d3c(cpu,rt,0x10f93d3c)?;
pc=0x10d38a74;continue;
}
0x10d38a74=>{
// 0x10d38a74 cmp w0, #0x64
{let a=((cpu.x[0] as u32 as u64));let b=((0x64u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d38a78 b.ne #0x10d38a8c
if cpu.condition(1){pc=0x10d38a8c;continue;}pc=0x10d38a7c;continue;
}
0x10d38a7c=>{
// 0x10d38a7c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d38a80 bl #0x10f81b34
cpu.x[30]=0x10d38a84;super::chunk11::f_10f81b34(cpu,rt,0x10f81b34)?;
// 0x10d38a84 mov w8, #-0x100
cpu.x[8]=(((0xffffffffffffff00u64))) as u32 as u64;
// 0x10d38a88 b #0x10d38ab8
pc=0x10d38ab8;continue;
}
0x10d38a8c=>{
// 0x10d38a8c mov w1, w0
cpu.x[1]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d38a90 ldr x0, [x20, #0x348]
{let address=(cpu.x[20]).wrapping_add(0x348u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d38a94 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d38a98 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d38a9c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d38aa0;super::dispatch(cpu,rt,target)?;}
pc=0x10d38aa0;continue;
}
0x10d38aa0=>{
// 0x10d38aa0 mov w22, w0
cpu.x[22]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d38aa4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d38aa8 bl #0x10f81b34
cpu.x[30]=0x10d38aac;super::chunk11::f_10f81b34(cpu,rt,0x10f81b34)?;
pc=0x10d38aac;continue;
}
0x10d38aac=>{
// 0x10d38aac cmp w22, #0
{let a=((cpu.x[22] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d38ab0 mov w8, #0x100
cpu.x[8]=(((0x100u64))) as u32 as u64;
// 0x10d38ab4 cneg w8, w8, ne
cpu.x[8]=(if cpu.condition(1){0u64.wrapping_sub(((cpu.x[8] as u32 as u64)))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
pc=0x10d38ab8;continue;
}
0x10d38ab8=>{
// 0x10d38ab8 str w8, [x0, #4]
{let address=(cpu.x[0]).wrapping_add(0x4u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d38abc ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d38ac0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d38ac4 ldr x8, [x8, #0x170]
{let address=(cpu.x[8]).wrapping_add(0x170u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d38ac8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d38acc;super::dispatch(cpu,rt,target)?;}
pc=0x10d38acc;continue;
}
0x10d38acc=>{
// 0x10d38acc tbnz w0, #0, #0x10d389ac
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10d389ac;continue;}pc=0x10d38ad0;continue;
}
0x10d38ad0=>{
// 0x10d38ad0 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d38ad4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d38ad8 ldr x8, [x8, #0x188]
{let address=(cpu.x[8]).wrapping_add(0x188u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d38adc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d38ae0;super::dispatch(cpu,rt,target)?;}
pc=0x10d38ae0;continue;
}
0x10d38ae0=>{
// 0x10d38ae0 tbz w0, #0, #0x10d38b6c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d38b6c;continue;}pc=0x10d38ae4;continue;
}
0x10d38ae4=>{
// 0x10d38ae4 cmp w21, #0x64
{let a=((cpu.x[21] as u32 as u64));let b=((0x64u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d38ae8 b.ne #0x10d38b08
if cpu.condition(1){pc=0x10d38b08;continue;}pc=0x10d38aec;continue;
}
0x10d38aec=>{
// 0x10d38aec mov w8, #2
cpu.x[8]=(((0x2u64))) as u32 as u64;
// 0x10d38af0 add x8, x20, x8, lsl #3
{let a=((cpu.x[20]));let b=(((cpu.x[8])) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d38af4 ldr x20, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10d38af8 ldp w9, w8, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10d38afc cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d38b00 b.eq #0x10d38b34
if cpu.condition(0){pc=0x10d38b34;continue;}pc=0x10d38b04;continue;
}
0x10d38b04=>{
// 0x10d38b04 b #0x10d38b58
pc=0x10d38b58;continue;
}
0x10d38b08=>{
// 0x10d38b08 ldr x0, [x20, #0x348]
{let address=(cpu.x[20]).wrapping_add(0x348u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d38b0c mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10d38b10 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d38b14 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d38b18 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d38b1c;super::dispatch(cpu,rt,target)?;}
pc=0x10d38b1c;continue;
}
0x10d38b1c=>{
// 0x10d38b1c sxtw x8, w0
cpu.x[8]=((((cpu.x[0] as u32 as u64))) as i32 as i64 as u64) as u64;
// 0x10d38b20 add x8, x20, x8, lsl #3
{let a=((cpu.x[20]));let b=(((cpu.x[8])) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d38b24 ldr x20, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10d38b28 ldp w9, w8, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10d38b2c cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d38b30 b.ne #0x10d38b58
if cpu.condition(1){pc=0x10d38b58;continue;}pc=0x10d38b34;continue;
}
0x10d38b34=>{
// 0x10d38b34 lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10d38b38 mov w10, #5
cpu.x[10]=(((0x5u64))) as u32 as u64;
// 0x10d38b3c cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d38b40 csel w1, w10, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[10] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10d38b44 cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d38b48 b.ge #0x10d38b58
if cpu.condition(10){pc=0x10d38b58;continue;}pc=0x10d38b4c;continue;
}
0x10d38b4c=>{
// 0x10d38b4c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d38b50 bl #0x10d449dc
cpu.x[30]=0x10d38b54;super::dispatch(cpu,rt,0x10d449dc)?;
// 0x10d38b54 ldr w8, [x20, #0xc]
{let address=(cpu.x[20]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
pc=0x10d38b58;continue;
}
0x10d38b58=>{
// 0x10d38b58 ldr x9, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d38b5c add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10d38b60 str w10, [x20, #0xc]
{let address=(cpu.x[20]).wrapping_add(0xcu64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d38b64 add x21, x9, w8, sxtw #3
{let a=((cpu.x[9]));let b=((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[21]=(result) as u64;}
pc=0x10d38b68;continue;
}
0x10d38b68=>{
// 0x10d38b68 str x19, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10d38b6c;continue;
}
0x10d38b6c=>{
// 0x10d38b6c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d38b70 ldp x20, x19, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d38b74 ldp x22, x21, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d38b78 ldp x24, x23, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10d38b7c ldp x26, x25, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10d38b80 ldp x29, x30, [sp], #0x50
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x50u64)))) as u64;}
// 0x10d38b84 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d388b4")),}}}
#[inline]
pub(super) unsafe fn f_10d497f4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d497f4=>{
// 0x10d497f4 movi v0.2d, #0000000000000000
cpu.q[0]=(lane_dup(0u128,64,2)) & 0xffffffffffffffffffffffffffffffffu128;
// 0x10d497f8 adrp x8, #0x102b2000
cpu.x[8]=(0x102b2000u64) as u64;
// 0x10d497fc str wzr, [x0, #0x5c]
{let address=(cpu.x[0]).wrapping_add(0x5cu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10d49800;continue;
}
0x10d49800=>{
// 0x10d49800 ldr q1, [x8, #0xaf0]
{let address=(cpu.x[8]).wrapping_add(0xaf0u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));cpu.q[1]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x10d49804 ldr x8, [x0, #0x98]
{let address=(cpu.x[0]).wrapping_add(0x98u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d49808 str xzr, [x0, #0x60]
{let address=(cpu.x[0]).wrapping_add(0x60u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d4980c str wzr, [x0, #0x74]
{let address=(cpu.x[0]).wrapping_add(0x74u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d49810 strb wzr, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10d49814 stp q1, q0, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=(cpu.q[1] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10d49818 stp q0, q0, [x0, #0x20]
{let address=(cpu.x[0]).wrapping_add(0x20u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10d4981c str q0, [x0, #0x40]
{let address=(cpu.x[0]).wrapping_add(0x40u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d49820 cbz x8, #0x10d49890
if (cpu.x[8]) == 0 {pc=0x10d49890;continue;}pc=0x10d49824;continue;
}
0x10d49824=>{
// 0x10d49824 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d49828 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d4982c stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d49830 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d49834 ldr x20, [x0, #0x90]
{let address=(cpu.x[0]).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10d49838 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10d4983c cbz x20, #0x10d4985c
if (cpu.x[20]) == 0 {pc=0x10d4985c;continue;}pc=0x10d49840;continue;
}
0x10d49840=>{
// 0x10d49840 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d49844 ldr x21, [x0], #0x10
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;cpu.x[0]=((cpu.x[0]).wrapping_add(((0x10u64)))) as u64;}
// 0x10d49848 bl #0x1142fcc4
cpu.x[30]=0x10d4984c;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
// 0x10d4984c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d49850 bl #0x1187d2c0
cpu.x[30]=0x10d49854;super::dispatch(cpu,rt,0x1187d2c0)?;
// 0x10d49854 mov x20, x21
cpu.x[20]=(((cpu.x[21]))) as u64;
// 0x10d49858 cbnz x21, #0x10d49840
if (cpu.x[21]) != 0 {pc=0x10d49840;continue;}pc=0x10d4985c;continue;
}
0x10d4985c=>{
// 0x10d4985c ldr x8, [x19, #0x88]
{let address=(cpu.x[19]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d49860 str xzr, [x19, #0x90]
{let address=(cpu.x[19]).wrapping_add(0x90u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d49864 cbz x8, #0x10d49880
if (cpu.x[8]) == 0 {pc=0x10d49880;continue;}pc=0x10d49868;continue;
}
0x10d49868=>{
// 0x10d49868 mov x9, xzr
cpu.x[9]=(((0u64))) as u64;
pc=0x10d4986c;continue;
}
0x10d4986c=>{
// 0x10d4986c ldr x10, [x19, #0x80]
{let address=(cpu.x[19]).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10d49870 str xzr, [x10, x9, lsl #3]
{let address=((cpu.x[10]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[9])) as u64).wrapping_shl(3) as u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d49874 add x9, x9, #1
{let a=((cpu.x[9]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
// 0x10d49878 cmp x8, x9
{let a=((cpu.x[8]));let b=((cpu.x[9]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d4987c b.ne #0x10d4986c
if cpu.condition(1){pc=0x10d4986c;continue;}pc=0x10d49880;continue;
}
0x10d49880=>{
// 0x10d49880 str xzr, [x19, #0x98]
{let address=(cpu.x[19]).wrapping_add(0x98u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d49884 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d49888 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10d4988c ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
pc=0x10d49890;continue;
}
0x10d49890=>{
// 0x10d49890 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d497f4")),}}}
#[inline]
pub(super) unsafe fn f_10d52520(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d52520=>{
// 0x10d52520 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d52524 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d52528 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d5252c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d52530 movi v0.2d, #0000000000000000
cpu.q[0]=(lane_dup(0u128,64,2)) & 0xffffffffffffffffffffffffffffffffu128;
// 0x10d52534 mov w8, #0x10
cpu.x[8]=(((0x10u64))) as u32 as u64;
// 0x10d52538 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10d5253c str xzr, [x0, #0x40]
{let address=(cpu.x[0]).wrapping_add(0x40u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d52540 str w8, [x0, #0x44]
{let address=(cpu.x[0]).wrapping_add(0x44u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d52544 stp q0, q0, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10d52548 stp q0, q0, [x0, #0x20]
{let address=(cpu.x[0]).wrapping_add(0x20u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10d5254c str x1, [x0, #0x20]
{let address=(cpu.x[0]).wrapping_add(0x20u64);let v0=cpu.x[1];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d52550 mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x10d52554 bl #0x110cbf10
cpu.x[30]=0x10d52558;super::chunk14::f_110cbf10(cpu,rt,0x110cbf10)?;
pc=0x10d52558;continue;
}
0x10d52558=>{
// 0x10d52558 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10d5255c mov w0, #0x58
cpu.x[0]=(((0x58u64))) as u32 as u64;
// 0x10d52560 bl #0x1187d2b0
cpu.x[30]=0x10d52564;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
pc=0x10d52564;continue;
}
0x10d52564=>{
// 0x10d52564 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d52568 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10d5256c bl #0x10fea43c
cpu.x[30]=0x10d52570;super::chunk2::f_10fea43c(cpu,rt,0x10fea43c)?;
pc=0x10d52570;continue;
}
0x10d52570=>{
// 0x10d52570 ldr x0, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d52574 str x21, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d52578 bl #0x110cbf10
cpu.x[30]=0x10d5257c;super::chunk14::f_110cbf10(cpu,rt,0x110cbf10)?;
pc=0x10d5257c;continue;
}
0x10d5257c=>{
// 0x10d5257c mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10d52580 mov w0, #0x58
cpu.x[0]=(((0x58u64))) as u32 as u64;
// 0x10d52584 bl #0x1187d2b0
cpu.x[30]=0x10d52588;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
pc=0x10d52588;continue;
}
0x10d52588=>{
// 0x10d52588 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d5258c mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10d52590 bl #0x10fea43c
cpu.x[30]=0x10d52594;super::chunk2::f_10fea43c(cpu,rt,0x10fea43c)?;
pc=0x10d52594;continue;
}
0x10d52594=>{
// 0x10d52594 str x21, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d52598 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d5259c ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10d525a0 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10d525a4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d52520")),}}}
#[inline]
pub(super) unsafe fn f_10d549d0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d549d0=>{
// 0x10d549d0 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d549d4 stp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d549d8 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d549dc mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d549e0 mov w5, w4
cpu.x[5]=(((cpu.x[4] as u32 as u64))) as u32 as u64;
// 0x10d549e4 mov w4, #0x18
cpu.x[4]=(((0x18u64))) as u32 as u64;
// 0x10d549e8 mov w19, w3
cpu.x[19]=(((cpu.x[3] as u32 as u64))) as u32 as u64;
// 0x10d549ec mov w20, w2
cpu.x[20]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x10d549f0 mov x21, x1
cpu.x[21]=(((cpu.x[1]))) as u64;
// 0x10d549f4 mov x22, x0
cpu.x[22]=(((cpu.x[0]))) as u64;
// 0x10d549f8 bl #0x10d54530
cpu.x[30]=0x10d549fc;super::chunk14::f_10d54530(cpu,rt,0x10d54530)?;
pc=0x10d549fc;continue;
}
0x10d549fc=>{
// 0x10d549fc mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d54a00 mov x1, x21
cpu.x[1]=(((cpu.x[21]))) as u64;
// 0x10d54a04 mov w2, w20
cpu.x[2]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x10d54a08 mov w3, w19
cpu.x[3]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10d54a0c mov w4, #0x19
cpu.x[4]=(((0x19u64))) as u32 as u64;
// 0x10d54a10 mov w5, #1
cpu.x[5]=(((0x1u64))) as u32 as u64;
// 0x10d54a14 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d54a18 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d54a1c ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10d54a20 b #0x10d54530
super::chunk14::f_10d54530(cpu,rt,0x10d54530)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d549d0")),}}}
#[inline]
pub(super) unsafe fn f_10d55e70(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d55e70=>{
// 0x10d55e70 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d55e74 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d55e78 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d55e7c ldr x2, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10d55e80 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10d55e84 mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x10d55e88 mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x10d55e8c mov w1, #0x6c
cpu.x[1]=(((0x6cu64))) as u32 as u64;
// 0x10d55e90 bl #0x111dd80c
cpu.x[30]=0x10d55e94;super::chunk6::f_111dd80c(cpu,rt,0x111dd80c)?;
pc=0x10d55e94;continue;
}
0x10d55e94=>{
// 0x10d55e94 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d55e98 str x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d55e9c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d55ea0 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d55ea4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d55ea8;super::dispatch(cpu,rt,target)?;}
pc=0x10d55ea8;continue;
}
0x10d55ea8=>{
// 0x10d55ea8 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d55eac str w0, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d55eb0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d55eb4 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d55eb8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d55ebc;super::dispatch(cpu,rt,target)?;}
pc=0x10d55ebc;continue;
}
0x10d55ebc=>{
// 0x10d55ebc ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d55ec0 str w0, [x19, #0x14]
{let address=(cpu.x[19]).wrapping_add(0x14u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d55ec4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d55ec8 ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d55ecc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d55ed0;super::dispatch(cpu,rt,target)?;}
pc=0x10d55ed0;continue;
}
0x10d55ed0=>{
// 0x10d55ed0 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d55ed4 and w9, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[9]=(result) as u32 as u64;}
// 0x10d55ed8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d55edc strb w9, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10d55ee0 ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d55ee4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d55ee8;super::dispatch(cpu,rt,target)?;}
pc=0x10d55ee8;continue;
}
0x10d55ee8=>{
// 0x10d55ee8 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d55eec and w9, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[9]=(result) as u32 as u64;}
// 0x10d55ef0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d55ef4 strb w9, [x19, #0x19]
{let address=(cpu.x[19]).wrapping_add(0x19u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10d55ef8 ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d55efc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d55f00;super::dispatch(cpu,rt,target)?;}
pc=0x10d55f00;continue;
}
0x10d55f00=>{
// 0x10d55f00 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d55f04 and w9, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[9]=(result) as u32 as u64;}
// 0x10d55f08 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d55f0c strb w9, [x19, #0x1a]
{let address=(cpu.x[19]).wrapping_add(0x1au64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10d55f10 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d55f14 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d55f18;super::dispatch(cpu,rt,target)?;}
pc=0x10d55f18;continue;
}
0x10d55f18=>{
// 0x10d55f18 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d55f1c str w0, [x19, #0x1c]
{let address=(cpu.x[19]).wrapping_add(0x1cu64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d55f20 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d55f24 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d55f28 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d55f2c;super::dispatch(cpu,rt,target)?;}
pc=0x10d55f2c;continue;
}
0x10d55f2c=>{
// 0x10d55f2c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d55f30 str w0, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d55f34 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d55f38 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d55f3c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d55f40;super::dispatch(cpu,rt,target)?;}
pc=0x10d55f40;continue;
}
0x10d55f40=>{
// 0x10d55f40 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d55f44 str w0, [x19, #0x24]
{let address=(cpu.x[19]).wrapping_add(0x24u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d55f48 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d55f4c ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d55f50 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d55f54;super::dispatch(cpu,rt,target)?;}
pc=0x10d55f54;continue;
}
0x10d55f54=>{
// 0x10d55f54 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d55f58 str w0, [x19, #0x34]
{let address=(cpu.x[19]).wrapping_add(0x34u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d55f5c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d55f60 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d55f64 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d55f68;super::dispatch(cpu,rt,target)?;}
pc=0x10d55f68;continue;
}
0x10d55f68=>{
// 0x10d55f68 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d55f6c str w0, [x19, #0x38]
{let address=(cpu.x[19]).wrapping_add(0x38u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d55f70 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d55f74 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d55f78 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d55f7c;super::dispatch(cpu,rt,target)?;}
pc=0x10d55f7c;continue;
}
0x10d55f7c=>{
// 0x10d55f7c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d55f80 str w0, [x19, #0x4c]
{let address=(cpu.x[19]).wrapping_add(0x4cu64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d55f84 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d55f88 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d55f8c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d55f90;super::dispatch(cpu,rt,target)?;}
pc=0x10d55f90;continue;
}
0x10d55f90=>{
// 0x10d55f90 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d55f94 str w0, [x19, #0x50]
{let address=(cpu.x[19]).wrapping_add(0x50u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d55f98 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d55f9c ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d55fa0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d55fa4;super::dispatch(cpu,rt,target)?;}
pc=0x10d55fa4;continue;
}
0x10d55fa4=>{
// 0x10d55fa4 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d55fa8 str w0, [x19, #0x5c]
{let address=(cpu.x[19]).wrapping_add(0x5cu64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d55fac mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d55fb0 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d55fb4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d55fb8;super::dispatch(cpu,rt,target)?;}
pc=0x10d55fb8;continue;
}
0x10d55fb8=>{
// 0x10d55fb8 str w0, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d55fbc mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d55fc0 bl #0x10d55b64
cpu.x[30]=0x10d55fc4;super::chunk0::f_10d55b64(cpu,rt,0x10d55b64)?;
pc=0x10d55fc4;continue;
}
0x10d55fc4=>{
// 0x10d55fc4 ldr x8, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d55fc8 ldrsw x9, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[9]=(v0) as u64;}
// 0x10d55fcc mov w11, #0x3e8
cpu.x[11]=(((0x3e8u64))) as u32 as u64;
// 0x10d55fd0 ldr x10, [x8, #0x68]
{let address=(cpu.x[8]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10d55fd4 lsl x9, x9, #2
cpu.x[9]=(((((cpu.x[9]))) as u64).wrapping_shl(((((0x2u64))) & 63) as u32) as u64) as u64;
// 0x10d55fd8 ldr x12, [x8, #0x78]
{let address=(cpu.x[8]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[12]=(v0) as u64;}
// 0x10d55fdc ldr x13, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[13]=(v0) as u64;}
// 0x10d55fe0 ldr w10, [x10, x9]
{let address=((cpu.x[10]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[9])));let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10d55fe4 mul w10, w10, w11
cpu.x[10]=((((cpu.x[10] as u32 as u64))).wrapping_mul(((cpu.x[11] as u32 as u64)))) as u32 as u64;
// 0x10d55fe8 str w10, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d55fec ldr w10, [x12, x9]
{let address=((cpu.x[12]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[9])));let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10d55ff0 ldr x12, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[12]=(v0) as u64;}
// 0x10d55ff4 str w10, [x19, #0x2c]
{let address=(cpu.x[19]).wrapping_add(0x2cu64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d55ff8 ldrsw x10, [x19, #0x34]
{let address=(cpu.x[19]).wrapping_add(0x34u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[10]=(v0) as u64;}
// 0x10d55ffc ldr w9, [x12, x9]
{let address=((cpu.x[12]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[9])));let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
pc=0x10d56000;continue;
}
0x10d56000=>{
// 0x10d56000 ldr x12, [x8, #0xa0]
{let address=(cpu.x[8]).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[12]=(v0) as u64;}
// 0x10d56004 lsl x10, x10, #2
cpu.x[10]=(((((cpu.x[10]))) as u64).wrapping_shl(((((0x2u64))) & 63) as u32) as u64) as u64;
// 0x10d56008 str w9, [x19, #0x30]
{let address=(cpu.x[19]).wrapping_add(0x30u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d5600c ldr w9, [x12, x10]
{let address=((cpu.x[12]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[10])));let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10d56010 ldr x12, [x8, #0xb0]
{let address=(cpu.x[8]).wrapping_add(0xb0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[12]=(v0) as u64;}
// 0x10d56014 mul w9, w9, w11
cpu.x[9]=((((cpu.x[9] as u32 as u64))).wrapping_mul(((cpu.x[11] as u32 as u64)))) as u32 as u64;
// 0x10d56018 str w9, [x19, #0x3c]
{let address=(cpu.x[19]).wrapping_add(0x3cu64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d5601c ldr w9, [x12, x10]
{let address=((cpu.x[12]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[10])));let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10d56020 ldr x12, [x8, #0xc0]
{let address=(cpu.x[8]).wrapping_add(0xc0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[12]=(v0) as u64;}
// 0x10d56024 str w9, [x19, #0x40]
{let address=(cpu.x[19]).wrapping_add(0x40u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d56028 ldrsw x9, [x19, #0x4c]
{let address=(cpu.x[19]).wrapping_add(0x4cu64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[9]=(v0) as u64;}
// 0x10d5602c ldr w12, [x12, x10]
{let address=((cpu.x[12]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[10])));let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[12]=(v0) as u32 as u64;}
// 0x10d56030 lsl x9, x9, #2
cpu.x[9]=(((((cpu.x[9]))) as u64).wrapping_shl(((((0x2u64))) & 63) as u32) as u64) as u64;
// 0x10d56034 str w12, [x19, #0x44]
{let address=(cpu.x[19]).wrapping_add(0x44u64);let v0=cpu.x[12] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d56038 ldr w12, [x13, x9]
{let address=((cpu.x[13]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[9])));let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[12]=(v0) as u32 as u64;}
// 0x10d5603c ldr x13, [x8, #0xd0]
{let address=(cpu.x[8]).wrapping_add(0xd0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[13]=(v0) as u64;}
// 0x10d56040 ldr x8, [x8, #0xf0]
{let address=(cpu.x[8]).wrapping_add(0xf0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d56044 mul w11, w12, w11
cpu.x[11]=((((cpu.x[12] as u32 as u64))).wrapping_mul(((cpu.x[11] as u32 as u64)))) as u32 as u64;
// 0x10d56048 ldr w10, [x13, x10]
{let address=((cpu.x[13]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[10])));let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10d5604c cmp w10, #0
{let a=((cpu.x[10] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d56050 str w11, [x19, #0x54]
{let address=(cpu.x[19]).wrapping_add(0x54u64);let v0=cpu.x[11] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d56054 cset w10, ne
cpu.x[10]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x10d56058 ldr w8, [x8, x9]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[9])));let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d5605c strb w10, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10d56060 str w8, [x19, #0x58]
{let address=(cpu.x[19]).wrapping_add(0x58u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d56064 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d56068 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10d5606c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d55e70")),}}}
#[inline]
pub(super) unsafe fn f_10d58bf8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d58bf8=>{
// 0x10d58bf8 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d58bfc stp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
pc=0x10d58c00;continue;
}
0x10d58c00=>{
// 0x10d58c00 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d58c04 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d58c08 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10d58c0c mov x19, x8
cpu.x[19]=(((cpu.x[8]))) as u64;
// 0x10d58c10 bl #0x10d36084
cpu.x[30]=0x10d58c14;super::chunk6::f_10d36084(cpu,rt,0x10d36084)?;
pc=0x10d58c14;continue;
}
0x10d58c14=>{
// 0x10d58c14 tbz w0, #0, #0x10d58c40
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d58c40;continue;}pc=0x10d58c18;continue;
}
0x10d58c18=>{
// 0x10d58c18 mov w0, #0x18
cpu.x[0]=(((0x18u64))) as u32 as u64;
// 0x10d58c1c bl #0x1187d2b0
cpu.x[30]=0x10d58c20;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
// 0x10d58c20 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d58c24 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10d58c28 bl #0x10d55950
cpu.x[30]=0x10d58c2c;super::dispatch(cpu,rt,0x10d55950)?;
// 0x10d58c2c str x21, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d58c30 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d58c34 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d58c38 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10d58c3c ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10d58c40=>{
// 0x10d58c40 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d58c44 bl #0x10d43840
cpu.x[30]=0x10d58c48;super::chunk15::f_10d43840(cpu,rt,0x10d43840)?;
pc=0x10d58c48;continue;
}
0x10d58c48=>{
// 0x10d58c48 tbz w0, #0, #0x10d58c74
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d58c74;continue;}pc=0x10d58c4c;continue;
}
0x10d58c4c=>{
// 0x10d58c4c mov w0, #0x18
cpu.x[0]=(((0x18u64))) as u32 as u64;
// 0x10d58c50 bl #0x1187d2b0
cpu.x[30]=0x10d58c54;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
// 0x10d58c54 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d58c58 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10d58c5c bl #0x10d558d8
cpu.x[30]=0x10d58c60;super::dispatch(cpu,rt,0x10d558d8)?;
// 0x10d58c60 str x21, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d58c64 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d58c68 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d58c6c ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10d58c70 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10d58c74=>{
// 0x10d58c74 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d58c78 bl #0x10d374d4
cpu.x[30]=0x10d58c7c;super::chunk1::f_10d374d4(cpu,rt,0x10d374d4)?;
pc=0x10d58c7c;continue;
}
0x10d58c7c=>{
// 0x10d58c7c mov w22, w0
cpu.x[22]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d58c80 mov w0, #0x18
cpu.x[0]=(((0x18u64))) as u32 as u64;
// 0x10d58c84 bl #0x1187d2b0
cpu.x[30]=0x10d58c88;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
pc=0x10d58c88;continue;
}
0x10d58c88=>{
// 0x10d58c88 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10d58c8c mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d58c90 tbz w22, #0, #0x10d58cac
if ((cpu.x[22] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d58cac;continue;}pc=0x10d58c94;continue;
}
0x10d58c94=>{
// 0x10d58c94 bl #0x10d557a4
cpu.x[30]=0x10d58c98;super::dispatch(cpu,rt,0x10d557a4)?;
// 0x10d58c98 str x21, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d58c9c ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d58ca0 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d58ca4 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10d58ca8 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10d58cac=>{
// 0x10d58cac bl #0x10d55838
cpu.x[30]=0x10d58cb0;super::chunk12::f_10d55838(cpu,rt,0x10d55838)?;
pc=0x10d58cb0;continue;
}
0x10d58cb0=>{
// 0x10d58cb0 str x21, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d58cb4 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d58cb8 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d58cbc ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10d58cc0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d58bf8")),}}}
#[inline]
pub(super) unsafe fn f_10d77cac(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d77cac=>{
// 0x10d77cac stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d77cb0 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d77cb4 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d77cb8 mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10d77cbc mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10d77cc0 bl #0x10d72784
cpu.x[30]=0x10d77cc4;super::chunk12::f_10d72784(cpu,rt,0x10d72784)?;
pc=0x10d77cc4;continue;
}
0x10d77cc4=>{
// 0x10d77cc4 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d77cc8 ldr w1, [x20, #0x30]
{let address=(cpu.x[20]).wrapping_add(0x30u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d77ccc mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d77cd0 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d77cd4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d77cd8;super::dispatch(cpu,rt,target)?;}
pc=0x10d77cd8;continue;
}
0x10d77cd8=>{
// 0x10d77cd8 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d77cdc ldr w1, [x20, #0x28]
{let address=(cpu.x[20]).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d77ce0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d77ce4 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d77ce8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d77cec;super::dispatch(cpu,rt,target)?;}
pc=0x10d77cec;continue;
}
0x10d77cec=>{
// 0x10d77cec ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d77cf0 ldr w1, [x20, #0x50]
{let address=(cpu.x[20]).wrapping_add(0x50u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d77cf4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d77cf8 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d77cfc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d77d00;super::dispatch(cpu,rt,target)?;}
pc=0x10d77d00;continue;
}
0x10d77d00=>{
// 0x10d77d00 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d77d04 ldr w1, [x20, #0x2c]
{let address=(cpu.x[20]).wrapping_add(0x2cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d77d08 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d77d0c ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d77d10 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d77d14;super::dispatch(cpu,rt,target)?;}
pc=0x10d77d14;continue;
}
0x10d77d14=>{
// 0x10d77d14 add x0, x20, #0x38
{let a=((cpu.x[20]));let b=((0x38u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d77d18 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10d77d1c ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d77d20 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10d77d24 b #0x10e806c0
super::chunk3::f_10e806c0(cpu,rt,0x10e806c0)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d77cac")),}}}
#[inline]
pub(super) unsafe fn f_10ddacf0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10ddacf0=>{
// 0x10ddacf0 ldrb w0, [x0, #0x108]
{let address=(cpu.x[0]).wrapping_add(0x108u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10ddacf4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10ddacf0")),}}}
#[inline]
pub(super) unsafe fn f_10ddddf0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10ddddf0=>{
// 0x10ddddf0 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10ddddf4 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10ddddf8 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10ddddfc mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10ddde00 mov w0, #0x208
cpu.x[0]=(((0x208u64))) as u32 as u64;
// 0x10ddde04 bl #0x1187d2b0
cpu.x[30]=0x10ddde08;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
pc=0x10ddde08;continue;
}
0x10ddde08=>{
// 0x10ddde08 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10ddde0c mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10ddde10 bl #0x10f7df40
cpu.x[30]=0x10ddde14;super::chunk10::f_10f7df40(cpu,rt,0x10f7df40)?;
pc=0x10ddde14;continue;
}
0x10ddde14=>{
// 0x10ddde14 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10ddde18 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10ddde1c ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10ddde20 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10ddddf0")),}}}
#[inline]
pub(super) unsafe fn f_10de0580(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10de0580=>{
// 0x10de0580 ldr w8, [x0, #0x334]
{let address=(cpu.x[0]).wrapping_add(0x334u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10de0584 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10de0588 cset w0, gt
cpu.x[0]=((cpu.condition(12)) as u64) as u32 as u64;
// 0x10de058c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10de0580")),}}}
#[inline]
pub(super) unsafe fn f_10de0720(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10de0720=>{
// 0x10de0720 ldr x0, [x0, #0x238]
{let address=(cpu.x[0]).wrapping_add(0x238u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10de0724 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10de0720")),}}}
#[inline]
pub(super) unsafe fn f_10de08c0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10de08c0=>{
// 0x10de08c0 ldr w0, [x0, #0x598]
{let address=(cpu.x[0]).wrapping_add(0x598u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10de08c4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10de08c0")),}}}
#[inline]
pub(super) unsafe fn f_10de0bb0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10de0bb0=>{
// 0x10de0bb0 ldr w0, [x0, #0x6a8]
{let address=(cpu.x[0]).wrapping_add(0x6a8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10de0bb4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10de0bb0")),}}}
#[inline]
pub(super) unsafe fn f_10e19a70(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e19a70=>{
// 0x10e19a70 sub w8, w1, #1
{let a=((cpu.x[1] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10e19a74 cmp w8, #0xdd
{let a=((cpu.x[8] as u32 as u64));let b=((0xddu64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e19a78 b.hi #0x10e19a94
if cpu.condition(8){pc=0x10e19a94;continue;}pc=0x10e19a7c;continue;
}
0x10e19a7c=>{
// 0x10e19a7c ldr x0, [x0, w1, uxtw #3]
{let address=((cpu.x[0]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[1] as u32 as u64) as u32 as u64)) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10e19a80 cbz x0, #0x10e19a94
if (cpu.x[0]) == 0 {pc=0x10e19a94;continue;}pc=0x10e19a84;continue;
}
0x10e19a84=>{
// 0x10e19a84 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e19a88 mov w1, w2
cpu.x[1]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x10e19a8c ldr x3, [x8, #0x30]
{let address=(cpu.x[8]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[3]=(v0) as u64;}
// 0x10e19a90 br x3
{let target=cpu.x[3];if target>=0x10e19a70 && target<0x10e19a9c{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
0x10e19a94=>{
// 0x10e19a94 mov x0, xzr
cpu.x[0]=(((0u64))) as u64;
// 0x10e19a98 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e19a70")),}}}
#[inline]
pub(super) unsafe fn f_10e19ce8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e19ce8=>{
// 0x10e19ce8 ldr x0, [x0, #0xe88]
{let address=(cpu.x[0]).wrapping_add(0xe88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10e19cec ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e19ce8")),}}}
#[inline]
pub(super) unsafe fn f_10e1a324(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e1a324=>{
// 0x10e1a324 ldr x0, [x0, #0x170]
{let address=(cpu.x[0]).wrapping_add(0x170u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10e1a328 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e1a32c ldr x1, [x8, #0x20]
{let address=(cpu.x[8]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10e1a330 br x1
{let target=cpu.x[1];if target>=0x10e1a324 && target<0x10e1a334{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e1a324")),}}}
#[inline]
pub(super) unsafe fn f_10e1afec(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e1afec=>{
// 0x10e1afec ldr x0, [x0, #0x830]
{let address=(cpu.x[0]).wrapping_add(0x830u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10e1aff0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e1afec")),}}}
#[inline]
pub(super) unsafe fn f_10e20f30(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e20f30=>{
// 0x10e20f30 ldr w0, [x0, #0xc4]
{let address=(cpu.x[0]).wrapping_add(0xc4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10e20f34 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e20f30")),}}}
#[inline]
pub(super) unsafe fn f_10e31d8c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e31d8c=>{
// 0x10e31d8c stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10e31d90 stp x24, x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10e31d94 stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10e31d98 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10e31d9c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10e31da0 mov w21, w4
cpu.x[21]=(((cpu.x[4] as u32 as u64))) as u32 as u64;
// 0x10e31da4 mov w19, w3
cpu.x[19]=(((cpu.x[3] as u32 as u64))) as u32 as u64;
// 0x10e31da8 mov w23, w2
cpu.x[23]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x10e31dac mov x22, x1
cpu.x[22]=(((cpu.x[1]))) as u64;
// 0x10e31db0 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10e31db4 bl #0x10e3202c
cpu.x[30]=0x10e31db8;super::chunk3::f_10e3202c(cpu,rt,0x10e3202c)?;
pc=0x10e31db8;continue;
}
0x10e31db8=>{
// 0x10e31db8 tbz w0, #0, #0x10e31f98
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10e31f98;continue;}pc=0x10e31dbc;continue;
}
0x10e31dbc=>{
// 0x10e31dbc ldrb w8, [x20, #0x81]
{let address=(cpu.x[20]).wrapping_add(0x81u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e31dc0 cbz w8, #0x10e31dd0
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10e31dd0;continue;}pc=0x10e31dc4;continue;
}
0x10e31dc4=>{
// 0x10e31dc4 ldr w8, [x22, #8]
{let address=(cpu.x[22]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e31dc8 cmp w8, w19
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e31dcc b.ne #0x10e31f98
if cpu.condition(1){pc=0x10e31f98;continue;}pc=0x10e31dd0;continue;
}
0x10e31dd0=>{
// 0x10e31dd0 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10e31dd4 bl #0x10f94124
cpu.x[30]=0x10e31dd8;super::chunk5::f_10f94124(cpu,rt,0x10f94124)?;
pc=0x10e31dd8;continue;
}
0x10e31dd8=>{
// 0x10e31dd8 cmp w0, w23
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[23] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e31ddc b.ne #0x10e31e00
if cpu.condition(1){pc=0x10e31e00;continue;}pc=0x10e31de0;continue;
}
0x10e31de0=>{
// 0x10e31de0 ldrb w8, [x20, #0x78]
{let address=(cpu.x[20]).wrapping_add(0x78u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e31de4 cbnz w8, #0x10e31e08
if (cpu.x[8] as u32 as u64) != 0 {pc=0x10e31e08;continue;}pc=0x10e31de8;continue;
}
0x10e31de8=>{
// 0x10e31de8 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
pc=0x10e31dec;continue;
}
0x10e31dec=>{
// 0x10e31dec ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10e31df0 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10e31df4 ldp x24, x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10e31df8 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10e31dfc ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10e31e00=>{
// 0x10e31e00 ldrb w8, [x20, #0x79]
{let address=(cpu.x[20]).wrapping_add(0x79u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e31e04 cbz w8, #0x10e31f98
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10e31f98;continue;}pc=0x10e31e08;continue;
}
0x10e31e08=>{
// 0x10e31e08 ldr w8, [x20, #0x74]
{let address=(cpu.x[20]).wrapping_add(0x74u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e31e0c cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e31e10 b.lt #0x10e31e64
if cpu.condition(11){pc=0x10e31e64;continue;}pc=0x10e31e14;continue;
}
0x10e31e14=>{
// 0x10e31e14 lsl x23, x8, #2
cpu.x[23]=(((((cpu.x[8]))) as u64).wrapping_shl(((((0x2u64))) & 63) as u32) as u64) as u64;
// 0x10e31e18 mov x24, xzr
cpu.x[24]=(((0u64))) as u64;
// 0x10e31e1c b #0x10e31e3c
pc=0x10e31e3c;continue;
}
0x10e31e20=>{
// 0x10e31e20 ldr x0, [x22, #0x48]
{let address=(cpu.x[22]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10e31e24 bl #0x10e0b624
cpu.x[30]=0x10e31e28;super::chunk10::f_10e0b624(cpu,rt,0x10e0b624)?;
// 0x10e31e28 cmp w0, w21
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[21] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e31e2c b.eq #0x10e31f98
if cpu.condition(0){pc=0x10e31f98;continue;}pc=0x10e31e30;continue;
}
0x10e31e30=>{
// 0x10e31e30 add x24, x24, #4
{let a=((cpu.x[24]));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[24]=(result) as u64;}
// 0x10e31e34 cmp x23, x24
{let a=((cpu.x[23]));let b=((cpu.x[24]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10e31e38 b.eq #0x10e31e64
if cpu.condition(0){pc=0x10e31e64;continue;}pc=0x10e31e3c;continue;
}
0x10e31e3c=>{
// 0x10e31e3c ldr x8, [x20, #0x68]
{let address=(cpu.x[20]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e31e40 ldr w8, [x8, x24]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[24])));let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e31e44 cmp w8, #0xe
{let a=((cpu.x[8] as u32 as u64));let b=((0xeu64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e31e48 b.eq #0x10e31e20
if cpu.condition(0){pc=0x10e31e20;continue;}pc=0x10e31e4c;continue;
}
0x10e31e4c=>{
// 0x10e31e4c cmp w8, #0xf
{let a=((cpu.x[8] as u32 as u64));let b=((0xfu64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e31e50 b.ne #0x10e31e30
if cpu.condition(1){pc=0x10e31e30;continue;}pc=0x10e31e54;continue;
}
0x10e31e54=>{
// 0x10e31e54 ldr w8, [x22, #8]
{let address=(cpu.x[22]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e31e58 cmp w8, w19
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e31e5c b.ne #0x10e31e30
if cpu.condition(1){pc=0x10e31e30;continue;}pc=0x10e31e60;continue;
}
0x10e31e60=>{
// 0x10e31e60 b #0x10e31f98
pc=0x10e31f98;continue;
}
0x10e31e64=>{
// 0x10e31e64 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10e31e68 bl #0x10f940fc
cpu.x[30]=0x10e31e6c;super::chunk9::f_10f940fc(cpu,rt,0x10f940fc)?;
pc=0x10e31e6c;continue;
}
0x10e31e6c=>{
// 0x10e31e6c cbz x0, #0x10e31f8c
if (cpu.x[0]) == 0 {pc=0x10e31f8c;continue;}pc=0x10e31e70;continue;
}
0x10e31e70=>{
// 0x10e31e70 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10e31e74 bl #0x10fc22f0
cpu.x[30]=0x10e31e78;super::chunk3::f_10fc22f0(cpu,rt,0x10fc22f0)?;
pc=0x10e31e78;continue;
}
0x10e31e78=>{
// 0x10e31e78 ldr w8, [x20, #0xd4]
{let address=(cpu.x[20]).wrapping_add(0xd4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e31e7c mov w22, w0
cpu.x[22]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10e31e80 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e31e84 b.lt #0x10e31f04
if cpu.condition(11){pc=0x10e31f04;continue;}pc=0x10e31e88;continue;
}
0x10e31e88=>{
// 0x10e31e88 cmp w22, #1
{let a=((cpu.x[22] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e31e8c b.lt #0x10e31f98
if cpu.condition(11){pc=0x10e31f98;continue;}pc=0x10e31e90;continue;
}
0x10e31e90=>{
// 0x10e31e90 mov w23, wzr
cpu.x[23]=(((0u64))) as u32 as u64;
// 0x10e31e94 b #0x10e31ea8
pc=0x10e31ea8;continue;
}
0x10e31e98=>{
// 0x10e31e98 add w23, w23, #1
{let a=((cpu.x[23] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[23]=(result) as u32 as u64;}
// 0x10e31e9c mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10e31ea0 cmp w23, w22
{let a=((cpu.x[23] as u32 as u64));let b=((cpu.x[22] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e31ea4 b.eq #0x10e31dec
if cpu.condition(0){pc=0x10e31dec;continue;}pc=0x10e31ea8;continue;
}
0x10e31ea8=>{
// 0x10e31ea8 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10e31eac mov w1, w23
cpu.x[1]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10e31eb0 bl #0x10fc23e4
cpu.x[30]=0x10e31eb4;super::chunk11::f_10fc23e4(cpu,rt,0x10fc23e4)?;
// 0x10e31eb4 ldr x9, [x20, #0xc8]
{let address=(cpu.x[20]).wrapping_add(0xc8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10e31eb8 ldr x8, [x0, #0x18]
{let address=(cpu.x[0]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e31ebc ldr w10, [x20, #0xd4]
{let address=(cpu.x[20]).wrapping_add(0xd4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10e31ec0 sub x9, x9, #8
{let a=((cpu.x[9]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u64;}
pc=0x10e31ec4;continue;
}
0x10e31ec4=>{
// 0x10e31ec4 cmp w10, #1
{let a=((cpu.x[10] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e31ec8 b.lt #0x10e31e98
if cpu.condition(11){pc=0x10e31e98;continue;}pc=0x10e31ecc;continue;
}
0x10e31ecc=>{
// 0x10e31ecc ldr x11, [x9, x10, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[10])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[11]=(v0) as u64;}
// 0x10e31ed0 sub x10, x10, #1
{let a=((cpu.x[10]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[10]=(result) as u64;}
// 0x10e31ed4 cmp x11, x8
{let a=((cpu.x[11]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10e31ed8 b.ne #0x10e31ec4
if cpu.condition(1){pc=0x10e31ec4;continue;}pc=0x10e31edc;continue;
}
0x10e31edc=>{
// 0x10e31edc ldr x8, [x0, #0x38]
{let address=(cpu.x[0]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e31ee0 cbz x8, #0x10e31ef0
if (cpu.x[8]) == 0 {pc=0x10e31ef0;continue;}pc=0x10e31ee4;continue;
}
0x10e31ee4=>{
// 0x10e31ee4 ldr w8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e31ee8 cmp w8, w19
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e31eec b.eq #0x10e31f04
if cpu.condition(0){pc=0x10e31f04;continue;}pc=0x10e31ef0;continue;
}
0x10e31ef0=>{
// 0x10e31ef0 ldr x8, [x0, #0x40]
{let address=(cpu.x[0]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e31ef4 cbz x8, #0x10e31e98
if (cpu.x[8]) == 0 {pc=0x10e31e98;continue;}pc=0x10e31ef8;continue;
}
0x10e31ef8=>{
// 0x10e31ef8 ldr w8, [x8, #0x4c]
{let address=(cpu.x[8]).wrapping_add(0x4cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e31efc cmp w8, w19
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e31f00 b.ne #0x10e31e98
if cpu.condition(1){pc=0x10e31e98;continue;}pc=0x10e31f04;continue;
}
0x10e31f04=>{
// 0x10e31f04 ldr w8, [x20, #0xe4]
{let address=(cpu.x[20]).wrapping_add(0xe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e31f08 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e31f0c b.lt #0x10e31fb0
if cpu.condition(11){pc=0x10e31fb0;continue;}pc=0x10e31f10;continue;
}
0x10e31f10=>{
// 0x10e31f10 cmp w22, #1
{let a=((cpu.x[22] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e31f14 b.lt #0x10e31fb0
if cpu.condition(11){pc=0x10e31fb0;continue;}pc=0x10e31f18;continue;
}
0x10e31f18=>{
// 0x10e31f18 mov w23, wzr
cpu.x[23]=(((0u64))) as u32 as u64;
// 0x10e31f1c b #0x10e31f2c
pc=0x10e31f2c;continue;
}
0x10e31f20=>{
// 0x10e31f20 add w23, w23, #1
{let a=((cpu.x[23] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[23]=(result) as u32 as u64;}
// 0x10e31f24 cmp w23, w22
{let a=((cpu.x[23] as u32 as u64));let b=((cpu.x[22] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e31f28 b.eq #0x10e31fb0
if cpu.condition(0){pc=0x10e31fb0;continue;}pc=0x10e31f2c;continue;
}
0x10e31f2c=>{
// 0x10e31f2c mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10e31f30 mov w1, w23
cpu.x[1]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10e31f34 bl #0x10fc23e4
cpu.x[30]=0x10e31f38;super::chunk11::f_10fc23e4(cpu,rt,0x10fc23e4)?;
// 0x10e31f38 ldr x9, [x20, #0xd8]
{let address=(cpu.x[20]).wrapping_add(0xd8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10e31f3c ldr x8, [x0, #0x18]
{let address=(cpu.x[0]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e31f40 ldr w10, [x20, #0xe4]
{let address=(cpu.x[20]).wrapping_add(0xe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10e31f44 sub x9, x9, #8
{let a=((cpu.x[9]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u64;}
pc=0x10e31f48;continue;
}
0x10e31f48=>{
// 0x10e31f48 cmp w10, #1
{let a=((cpu.x[10] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e31f4c b.lt #0x10e31f20
if cpu.condition(11){pc=0x10e31f20;continue;}pc=0x10e31f50;continue;
}
0x10e31f50=>{
// 0x10e31f50 ldr x11, [x9, x10, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[10])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[11]=(v0) as u64;}
// 0x10e31f54 sub x10, x10, #1
{let a=((cpu.x[10]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[10]=(result) as u64;}
// 0x10e31f58 cmp x11, x8
{let a=((cpu.x[11]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10e31f5c b.ne #0x10e31f48
if cpu.condition(1){pc=0x10e31f48;continue;}pc=0x10e31f60;continue;
}
0x10e31f60=>{
// 0x10e31f60 ldr x8, [x0, #0x38]
{let address=(cpu.x[0]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e31f64 cbz x8, #0x10e31f74
if (cpu.x[8]) == 0 {pc=0x10e31f74;continue;}pc=0x10e31f68;continue;
}
0x10e31f68=>{
// 0x10e31f68 ldr w8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e31f6c cmp w8, w19
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e31f70 b.eq #0x10e31f98
if cpu.condition(0){pc=0x10e31f98;continue;}pc=0x10e31f74;continue;
}
0x10e31f74=>{
// 0x10e31f74 ldr x8, [x0, #0x40]
{let address=(cpu.x[0]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e31f78 cbz x8, #0x10e31f20
if (cpu.x[8]) == 0 {pc=0x10e31f20;continue;}pc=0x10e31f7c;continue;
}
0x10e31f7c=>{
// 0x10e31f7c ldr w8, [x8, #0x4c]
{let address=(cpu.x[8]).wrapping_add(0x4cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e31f80 cmp w8, w19
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e31f84 b.ne #0x10e31f20
if cpu.condition(1){pc=0x10e31f20;continue;}pc=0x10e31f88;continue;
}
0x10e31f88=>{
// 0x10e31f88 b #0x10e31f98
pc=0x10e31f98;continue;
}
0x10e31f8c=>{
// 0x10e31f8c ldr w8, [x20, #0xd4]
{let address=(cpu.x[20]).wrapping_add(0xd4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e31f90 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e31f94 b.le #0x10e31fb0
if cpu.condition(13){pc=0x10e31fb0;continue;}pc=0x10e31f98;continue;
}
0x10e31f98=>{
// 0x10e31f98 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10e31f9c ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10e31fa0 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10e31fa4 ldp x24, x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10e31fa8 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10e31fac ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10e31fb0=>{
// 0x10e31fb0 mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
// 0x10e31fb4 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10e31fb8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10e31fbc ldp x24, x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10e31fc0 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10e31fc4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e31d8c")),}}}
#[inline]
pub(super) unsafe fn f_10e3202c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e3202c=>{
// 0x10e3202c stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10e32030 stp x24, x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10e32034 stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10e32038 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10e3203c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10e32040 ldr x8, [x1]
{let address=(cpu.x[1]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e32044 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10e32048 mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x10e3204c mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x10e32050 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e32054 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10e32058;super::dispatch(cpu,rt,target)?;}
pc=0x10e32058;continue;
}
0x10e32058=>{
// 0x10e32058 mov w8, w0
cpu.x[8]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10e3205c cmp w0, #4
{let a=((cpu.x[0] as u32 as u64));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e32060 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10e32064 b.gt #0x10e32098
if cpu.condition(12){pc=0x10e32098;continue;}pc=0x10e32068;continue;
}
0x10e32068=>{
// 0x10e32068 cmp w8, #3
{let a=((cpu.x[8] as u32 as u64));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e3206c b.eq #0x10e320ec
if cpu.condition(0){pc=0x10e320ec;continue;}pc=0x10e32070;continue;
}
0x10e32070=>{
// 0x10e32070 cmp w8, #4
{let a=((cpu.x[8] as u32 as u64));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e32074 b.ne #0x10e323d0
if cpu.condition(1){pc=0x10e323d0;continue;}pc=0x10e32078;continue;
}
0x10e32078=>{
// 0x10e32078 ldrb w8, [x19, #0x7c]
{let address=(cpu.x[19]).wrapping_add(0x7cu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e3207c cbnz w8, #0x10e32100
if (cpu.x[8] as u32 as u64) != 0 {pc=0x10e32100;continue;}pc=0x10e32080;continue;
}
0x10e32080=>{
// 0x10e32080 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10e32084 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10e32088 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10e3208c ldp x24, x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10e32090 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10e32094 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10e32098=>{
// 0x10e32098 cmp w8, #6
{let a=((cpu.x[8] as u32 as u64));let b=((0x6u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e3209c b.eq #0x10e320f8
if cpu.condition(0){pc=0x10e320f8;continue;}pc=0x10e320a0;continue;
}
0x10e320a0=>{
// 0x10e320a0 cmp w8, #5
{let a=((cpu.x[8] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e320a4 b.ne #0x10e323d0
if cpu.condition(1){pc=0x10e323d0;continue;}pc=0x10e320a8;continue;
}
0x10e320a8=>{
// 0x10e320a8 ldrb w8, [x19, #0x7f]
{let address=(cpu.x[19]).wrapping_add(0x7fu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e320ac cbz w8, #0x10e320c4
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10e320c4;continue;}pc=0x10e320b0;continue;
}
0x10e320b0=>{
// 0x10e320b0 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e320b4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10e320b8 ldr x8, [x8, #0x178]
{let address=(cpu.x[8]).wrapping_add(0x178u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e320bc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10e320c0;super::dispatch(cpu,rt,target)?;}
// 0x10e320c0 tbnz w0, #0, #0x10e32100
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10e32100;continue;}pc=0x10e320c4;continue;
}
0x10e320c4=>{
// 0x10e320c4 ldrb w8, [x19, #0x7b]
{let address=(cpu.x[19]).wrapping_add(0x7bu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e320c8 cbz w8, #0x10e320e0
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10e320e0;continue;}pc=0x10e320cc;continue;
}
0x10e320cc=>{
// 0x10e320cc ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e320d0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10e320d4 ldr x8, [x8, #0x180]
{let address=(cpu.x[8]).wrapping_add(0x180u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e320d8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10e320dc;super::dispatch(cpu,rt,target)?;}
// 0x10e320dc tbnz w0, #0, #0x10e32100
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10e32100;continue;}pc=0x10e320e0;continue;
}
0x10e320e0=>{
// 0x10e320e0 ldrb w8, [x19, #0x7a]
{let address=(cpu.x[19]).wrapping_add(0x7au64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e320e4 cbz w8, #0x10e32110
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10e32110;continue;}pc=0x10e320e8;continue;
}
0x10e320e8=>{
// 0x10e320e8 b #0x10e32100
pc=0x10e32100;continue;
}
0x10e320ec=>{
// 0x10e320ec ldrb w8, [x19, #0x7d]
{let address=(cpu.x[19]).wrapping_add(0x7du64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e320f0 cbnz w8, #0x10e32100
if (cpu.x[8] as u32 as u64) != 0 {pc=0x10e32100;continue;}pc=0x10e320f4;continue;
}
0x10e320f4=>{
// 0x10e320f4 b #0x10e32110
pc=0x10e32110;continue;
}
0x10e320f8=>{
// 0x10e320f8 ldrb w8, [x19, #0x7e]
{let address=(cpu.x[19]).wrapping_add(0x7eu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e320fc cbz w8, #0x10e32110
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10e32110;continue;}pc=0x10e32100;continue;
}
0x10e32100=>{
// 0x10e32100 ldr x8, [x19, #0x88]
{let address=(cpu.x[19]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e32104 ldr x9, [x20, #0xb0]
{let address=(cpu.x[20]).wrapping_add(0xb0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10e32108 tst x9, x8
{let a=((cpu.x[9]));let b=((cpu.x[8]));let result=a & b;cpu.logical_flags(result,64);}
// 0x10e3210c b.eq #0x10e32128
if cpu.condition(0){pc=0x10e32128;continue;}pc=0x10e32110;continue;
}
0x10e32110=>{
// 0x10e32110 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10e32114 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10e32118 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10e3211c ldp x24, x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10e32120 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10e32124 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10e32128=>{
// 0x10e32128 ldrb w8, [x19, #0xa0]
{let address=(cpu.x[19]).wrapping_add(0xa0u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e3212c cbz w8, #0x10e32144
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10e32144;continue;}pc=0x10e32130;continue;
}
0x10e32130=>{
// 0x10e32130 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e32134 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10e32138 ldr x8, [x8, #0x138]
{let address=(cpu.x[8]).wrapping_add(0x138u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e3213c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10e32140;super::dispatch(cpu,rt,target)?;}
pc=0x10e32140;continue;
}
0x10e32140=>{
// 0x10e32140 tbz w0, #0, #0x10e32110
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10e32110;continue;}pc=0x10e32144;continue;
}
0x10e32144=>{
// 0x10e32144 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e32148 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10e3214c ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e32150 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10e32154;super::dispatch(cpu,rt,target)?;}
pc=0x10e32154;continue;
}
0x10e32154=>{
// 0x10e32154 ldr w8, [x19, #0x74]
{let address=(cpu.x[19]).wrapping_add(0x74u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e32158 cmp w0, #5
{let a=((cpu.x[0] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e3215c csel x21, x20, xzr, eq
cpu.x[21]=(if cpu.condition(0){((cpu.x[20]))}else{((0u64))}) as u64;
// 0x10e32160 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e32164 b.lt #0x10e3235c
if cpu.condition(11){pc=0x10e3235c;continue;}pc=0x10e32168;continue;
}
0x10e32168=>{
// 0x10e32168 lsl x23, x8, #2
cpu.x[23]=(((((cpu.x[8]))) as u64).wrapping_shl(((((0x2u64))) & 63) as u32) as u64) as u64;
// 0x10e3216c mov x22, xzr
cpu.x[22]=(((0u64))) as u64;
// 0x10e32170 adrp x24, #0x1033f000
cpu.x[24]=(0x1033f000u64) as u64;
// 0x10e32174 add x24, x24, #0x2b8
{let a=((cpu.x[24]));let b=((0x2b8u64));let result=a.wrapping_add(b);cpu.x[24]=(result) as u64;}
// 0x10e32178 b #0x10e3219c
pc=0x10e3219c;continue;
}
0x10e3217c=>{
// 0x10e3217c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e32180 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10e32184 ldr x8, [x8, #0x188]
{let address=(cpu.x[8]).wrapping_add(0x188u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e32188 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10e3218c;super::dispatch(cpu,rt,target)?;}
// 0x10e3218c tbnz w0, #0, #0x10e32110
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10e32110;continue;}pc=0x10e32190;continue;
}
0x10e32190=>{
// 0x10e32190 add x22, x22, #4
{let a=((cpu.x[22]));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[22]=(result) as u64;}
// 0x10e32194 cmp x23, x22
{let a=((cpu.x[23]));let b=((cpu.x[22]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10e32198 b.eq #0x10e3235c
if cpu.condition(0){pc=0x10e3235c;continue;}pc=0x10e3219c;continue;
}
0x10e3219c=>{
// 0x10e3219c ldr x8, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e321a0 ldr w8, [x8, x22]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[22])));let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e321a4 sub w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10e321a8 cmp w8, #0x12
{let a=((cpu.x[8] as u32 as u64));let b=((0x12u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e321ac b.hi #0x10e32190
if cpu.condition(8){pc=0x10e32190;continue;}pc=0x10e321b0;continue;
}
0x10e321b0=>{
// 0x10e321b0 adr x9, #0x10e3217c
cpu.x[9]=(0x10e3217cu64) as u64;
// 0x10e321b4 ldrb w10, [x24, x8]
{let address=((cpu.x[24]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[8])));let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10e321b8 add x9, x9, x10, lsl #2
{let a=((cpu.x[9]));let b=(((cpu.x[10])) as u64).wrapping_shl(2) as u64;let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
// 0x10e321bc br x9
{let target=cpu.x[9];if target>=0x10e3202c && target<0x10e32408{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
0x10e321c0=>{
// 0x10e321c0 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e321c4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10e321c8 ldr x8, [x8, #0xf8]
{let address=(cpu.x[8]).wrapping_add(0xf8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e321cc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10e321d0;super::dispatch(cpu,rt,target)?;}
pc=0x10e321d0;continue;
}
0x10e321d0=>{
// 0x10e321d0 tbz w0, #0, #0x10e32190
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10e32190;continue;}pc=0x10e321d4;continue;
}
0x10e321d4=>{
// 0x10e321d4 b #0x10e32110
pc=0x10e32110;continue;
}
0x10e321d8=>{
// 0x10e321d8 cbz x21, #0x10e32190
if (cpu.x[21]) == 0 {pc=0x10e32190;continue;}pc=0x10e321dc;continue;
}
0x10e321dc=>{
// 0x10e321dc mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10e321e0 bl #0x10f817a8
cpu.x[30]=0x10e321e4;super::chunk8::f_10f817a8(cpu,rt,0x10f817a8)?;
// 0x10e321e4 cmp w0, #8
{let a=((cpu.x[0] as u32 as u64));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e321e8 b.ne #0x10e32190
if cpu.condition(1){pc=0x10e32190;continue;}pc=0x10e321ec;continue;
}
0x10e321ec=>{
// 0x10e321ec b #0x10e32110
pc=0x10e32110;continue;
}
0x10e321f0=>{
// 0x10e321f0 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e321f4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10e321f8 ldr x8, [x8, #0x178]
{let address=(cpu.x[8]).wrapping_add(0x178u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e321fc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10e32200;super::dispatch(cpu,rt,target)?;}
// 0x10e32200 tbz w0, #0, #0x10e32190
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10e32190;continue;}pc=0x10e32204;continue;
}
0x10e32204=>{
// 0x10e32204 b #0x10e32110
pc=0x10e32110;continue;
}
0x10e32208=>{
// 0x10e32208 cbz x21, #0x10e32190
if (cpu.x[21]) == 0 {pc=0x10e32190;continue;}pc=0x10e3220c;continue;
}
0x10e3220c=>{
// 0x10e3220c mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10e32210 bl #0x10f7e050
cpu.x[30]=0x10e32214;super::chunk10::f_10f7e050(cpu,rt,0x10f7e050)?;
// 0x10e32214 bl #0x10de049c
cpu.x[30]=0x10e32218;super::chunk14::f_10de049c(cpu,rt,0x10de049c)?;
// 0x10e32218 tbnz w0, #0, #0x10e32110
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10e32110;continue;}pc=0x10e3221c;continue;
}
0x10e3221c=>{
// 0x10e3221c mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10e32220 bl #0x10f940fc
cpu.x[30]=0x10e32224;super::chunk9::f_10f940fc(cpu,rt,0x10f940fc)?;
// 0x10e32224 bl #0x10fc2c4c
cpu.x[30]=0x10e32228;super::chunk15::f_10fc2c4c(cpu,rt,0x10fc2c4c)?;
// 0x10e32228 tbz w0, #0, #0x10e32190
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10e32190;continue;}pc=0x10e3222c;continue;
}
0x10e3222c=>{
// 0x10e3222c b #0x10e32110
pc=0x10e32110;continue;
}
0x10e32230=>{
// 0x10e32230 cbz x21, #0x10e32190
if (cpu.x[21]) == 0 {pc=0x10e32190;continue;}pc=0x10e32234;continue;
}
0x10e32234=>{
// 0x10e32234 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10e32238 bl #0x10f83288
cpu.x[30]=0x10e3223c;super::dispatch(cpu,rt,0x10f83288)?;
// 0x10e3223c tbz w0, #0, #0x10e32190
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10e32190;continue;}pc=0x10e32240;continue;
}
0x10e32240=>{
// 0x10e32240 b #0x10e32110
pc=0x10e32110;continue;
}
0x10e32244=>{
// 0x10e32244 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e32248 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10e3224c ldr x8, [x8, #0x170]
{let address=(cpu.x[8]).wrapping_add(0x170u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e32250 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10e32254;super::dispatch(cpu,rt,target)?;}
// 0x10e32254 tbz w0, #0, #0x10e32190
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10e32190;continue;}pc=0x10e32258;continue;
}
0x10e32258=>{
// 0x10e32258 b #0x10e32110
pc=0x10e32110;continue;
}
0x10e3225c=>{
// 0x10e3225c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e32260 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10e32264 ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e32268 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10e3226c;super::dispatch(cpu,rt,target)?;}
pc=0x10e3226c;continue;
}
0x10e3226c=>{
// 0x10e3226c cbnz x0, #0x10e32190
if (cpu.x[0]) != 0 {pc=0x10e32190;continue;}pc=0x10e32270;continue;
}
0x10e32270=>{
// 0x10e32270 b #0x10e323d0
pc=0x10e323d0;continue;
}
0x10e32274=>{
// 0x10e32274 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e32278 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10e3227c ldr x8, [x8, #0x180]
{let address=(cpu.x[8]).wrapping_add(0x180u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e32280 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10e32284;super::dispatch(cpu,rt,target)?;}
pc=0x10e32284;continue;
}
0x10e32284=>{
// 0x10e32284 tbz w0, #0, #0x10e32190
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10e32190;continue;}pc=0x10e32288;continue;
}
0x10e32288=>{
// 0x10e32288 b #0x10e32110
pc=0x10e32110;continue;
}
0x10e3228c=>{
// 0x10e3228c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e32290 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10e32294 ldr x8, [x8, #0x100]
{let address=(cpu.x[8]).wrapping_add(0x100u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e32298 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10e3229c;super::dispatch(cpu,rt,target)?;}
pc=0x10e3229c;continue;
}
0x10e3229c=>{
// 0x10e3229c tbz w0, #0, #0x10e32190
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10e32190;continue;}pc=0x10e322a0;continue;
}
0x10e322a0=>{
// 0x10e322a0 b #0x10e32110
pc=0x10e32110;continue;
}
0x10e322a4=>{
// 0x10e322a4 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e322a8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10e322ac ldr x8, [x8, #0x108]
{let address=(cpu.x[8]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e322b0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10e322b4;super::dispatch(cpu,rt,target)?;}
// 0x10e322b4 tbz w0, #0, #0x10e32190
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10e32190;continue;}pc=0x10e322b8;continue;
}
0x10e322b8=>{
// 0x10e322b8 b #0x10e32110
pc=0x10e32110;continue;
}
0x10e322bc=>{
// 0x10e322bc cbz x21, #0x10e32190
if (cpu.x[21]) == 0 {pc=0x10e32190;continue;}pc=0x10e322c0;continue;
}
0x10e322c0=>{
// 0x10e322c0 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10e322c4 bl #0x10f817a8
cpu.x[30]=0x10e322c8;super::chunk8::f_10f817a8(cpu,rt,0x10f817a8)?;
pc=0x10e322c8;continue;
}
0x10e322c8=>{
// 0x10e322c8 cmp w0, #5
{let a=((cpu.x[0] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e322cc b.ne #0x10e32190
if cpu.condition(1){pc=0x10e32190;continue;}pc=0x10e322d0;continue;
}
0x10e322d0=>{
// 0x10e322d0 b #0x10e32110
pc=0x10e32110;continue;
}
0x10e322d4=>{
// 0x10e322d4 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e322d8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10e322dc ldr x8, [x8, #0x140]
{let address=(cpu.x[8]).wrapping_add(0x140u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e322e0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10e322e4;super::dispatch(cpu,rt,target)?;}
pc=0x10e322e4;continue;
}
0x10e322e4=>{
// 0x10e322e4 tbz w0, #0, #0x10e32190
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10e32190;continue;}pc=0x10e322e8;continue;
}
0x10e322e8=>{
// 0x10e322e8 b #0x10e32110
pc=0x10e32110;continue;
}
0x10e322ec=>{
// 0x10e322ec cbz x21, #0x10e32190
if (cpu.x[21]) == 0 {pc=0x10e32190;continue;}pc=0x10e322f0;continue;
}
0x10e322f0=>{
// 0x10e322f0 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10e322f4 bl #0x10f7e050
cpu.x[30]=0x10e322f8;super::chunk10::f_10f7e050(cpu,rt,0x10f7e050)?;
// 0x10e322f8 bl #0x10de06a0
cpu.x[30]=0x10e322fc;super::chunk9::f_10de06a0(cpu,rt,0x10de06a0)?;
// 0x10e322fc tbz w0, #0, #0x10e32190
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10e32190;continue;}pc=0x10e32300;continue;
}
0x10e32300=>{
// 0x10e32300 b #0x10e32110
pc=0x10e32110;continue;
}
0x10e32304=>{
// 0x10e32304 cbz x21, #0x10e32190
if (cpu.x[21]) == 0 {pc=0x10e32190;continue;}pc=0x10e32308;continue;
}
0x10e32308=>{
// 0x10e32308 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10e3230c bl #0x10f817a8
cpu.x[30]=0x10e32310;super::chunk8::f_10f817a8(cpu,rt,0x10f817a8)?;
// 0x10e32310 cmp w0, #0xc
{let a=((cpu.x[0] as u32 as u64));let b=((0xcu64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e32314 b.eq #0x10e32110
if cpu.condition(0){pc=0x10e32110;continue;}pc=0x10e32318;continue;
}
0x10e32318=>{
// 0x10e32318 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10e3231c bl #0x10f817a8
cpu.x[30]=0x10e32320;super::chunk8::f_10f817a8(cpu,rt,0x10f817a8)?;
// 0x10e32320 cmp w0, #0xd
{let a=((cpu.x[0] as u32 as u64));let b=((0xdu64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e32324 b.ne #0x10e32190
if cpu.condition(1){pc=0x10e32190;continue;}pc=0x10e32328;continue;
}
0x10e32328=>{
// 0x10e32328 b #0x10e32110
pc=0x10e32110;continue;
}
0x10e3232c=>{
// 0x10e3232c cbz x21, #0x10e32190
if (cpu.x[21]) == 0 {pc=0x10e32190;continue;}pc=0x10e32330;continue;
}
0x10e32330=>{
// 0x10e32330 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10e32334 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10e32338 bl #0x10f82058
cpu.x[30]=0x10e3233c;super::chunk2::f_10f82058(cpu,rt,0x10f82058)?;
pc=0x10e3233c;continue;
}
0x10e3233c=>{
// 0x10e3233c tbz w0, #0, #0x10e32190
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10e32190;continue;}pc=0x10e32340;continue;
}
0x10e32340=>{
// 0x10e32340 b #0x10e32110
pc=0x10e32110;continue;
}
0x10e32344=>{
// 0x10e32344 cbz x21, #0x10e32190
if (cpu.x[21]) == 0 {pc=0x10e32190;continue;}pc=0x10e32348;continue;
}
0x10e32348=>{
// 0x10e32348 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10e3234c bl #0x10f7e050
cpu.x[30]=0x10e32350;super::chunk10::f_10f7e050(cpu,rt,0x10f7e050)?;
pc=0x10e32350;continue;
}
0x10e32350=>{
// 0x10e32350 ldrb w8, [x0, #0x6f9]
{let address=(cpu.x[0]).wrapping_add(0x6f9u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e32354 cbz w8, #0x10e32190
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10e32190;continue;}pc=0x10e32358;continue;
}
0x10e32358=>{
// 0x10e32358 b #0x10e32110
pc=0x10e32110;continue;
}
0x10e3235c=>{
// 0x10e3235c cbz x21, #0x10e323e4
if (cpu.x[21]) == 0 {pc=0x10e323e4;continue;}pc=0x10e32360;continue;
}
0x10e32360=>{
// 0x10e32360 ldrb w8, [x19, #0x80]
{let address=(cpu.x[19]).wrapping_add(0x80u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e32364 cbz w8, #0x10e323fc
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10e323fc;continue;}pc=0x10e32368;continue;
}
0x10e32368=>{
// 0x10e32368 ldr w9, [x19, #0xb4]
{let address=(cpu.x[19]).wrapping_add(0xb4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10e3236c ldr x8, [x20, #0x48]
{let address=(cpu.x[20]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e32370 cbz w9, #0x10e323a0
if (cpu.x[9] as u32 as u64) == 0 {pc=0x10e323a0;continue;}pc=0x10e32374;continue;
}
0x10e32374=>{
// 0x10e32374 cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e32378 b.lt #0x10e32110
if cpu.condition(11){pc=0x10e32110;continue;}pc=0x10e3237c;continue;
}
0x10e3237c=>{
// 0x10e3237c ldr x10, [x19, #0xa8]
{let address=(cpu.x[19]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
pc=0x10e32380;continue;
}
0x10e32380=>{
// 0x10e32380 ldr x11, [x10]
{let address=(cpu.x[10]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[11]=(v0) as u64;}
// 0x10e32384 cmp x8, x11
{let a=((cpu.x[8]));let b=((cpu.x[11]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10e32388 b.eq #0x10e323a0
if cpu.condition(0){pc=0x10e323a0;continue;}pc=0x10e3238c;continue;
}
0x10e3238c=>{
// 0x10e3238c mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10e32390 subs x9, x9, #1
{let a=((cpu.x[9]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x10e32394 add x10, x10, #8
{let a=((cpu.x[10]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x10e32398 b.ne #0x10e32380
if cpu.condition(1){pc=0x10e32380;continue;}pc=0x10e3239c;continue;
}
0x10e3239c=>{
// 0x10e3239c b #0x10e323d0
pc=0x10e323d0;continue;
}
0x10e323a0=>{
// 0x10e323a0 ldr w10, [x19, #0xc4]
{let address=(cpu.x[19]).wrapping_add(0xc4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10e323a4 cmp w10, #1
{let a=((cpu.x[10] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e323a8 b.lt #0x10e323e4
if cpu.condition(11){pc=0x10e323e4;continue;}pc=0x10e323ac;continue;
}
0x10e323ac=>{
// 0x10e323ac ldr x9, [x19, #0xb8]
{let address=(cpu.x[19]).wrapping_add(0xb8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10e323b0 sub x10, x10, #1
{let a=((cpu.x[10]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[10]=(result) as u64;}
pc=0x10e323b4;continue;
}
0x10e323b4=>{
// 0x10e323b4 ldr x12, [x9], #8
{let address=(cpu.x[9]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[12]=(v0) as u64;cpu.x[9]=((cpu.x[9]).wrapping_add(((0x8u64)))) as u64;}
// 0x10e323b8 subs x10, x10, #1
{let a=((cpu.x[10]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[10]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x10e323bc cset w11, hs
cpu.x[11]=((cpu.condition(2)) as u64) as u32 as u64;
// 0x10e323c0 cmp x8, x12
{let a=((cpu.x[8]));let b=((cpu.x[12]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10e323c4 cset w0, ne
cpu.x[0]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x10e323c8 b.eq #0x10e323d0
if cpu.condition(0){pc=0x10e323d0;continue;}pc=0x10e323cc;continue;
}
0x10e323cc=>{
// 0x10e323cc tbnz w11, #0, #0x10e323b4
if ((cpu.x[11] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10e323b4;continue;}pc=0x10e323d0;continue;
}
0x10e323d0=>{
// 0x10e323d0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10e323d4 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10e323d8 ldp x24, x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10e323dc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10e323e0 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10e323e4=>{
// 0x10e323e4 mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
// 0x10e323e8 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10e323ec ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10e323f0 ldp x24, x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10e323f4 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10e323f8 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10e323fc=>{
// 0x10e323fc ldr x8, [x21, #0x180]
{let address=(cpu.x[21]).wrapping_add(0x180u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
pc=0x10e32400;continue;
}
0x10e32400=>{
// 0x10e32400 cbnz x8, #0x10e32110
if (cpu.x[8]) != 0 {pc=0x10e32110;continue;}pc=0x10e32404;continue;
}
0x10e32404=>{
// 0x10e32404 b #0x10e32368
pc=0x10e32368;continue;
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e3202c")),}}}
#[inline]
pub(super) unsafe fn f_10e5b064(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e5b064=>{
// 0x10e5b064 sxtb w8, w1
cpu.x[8]=((((cpu.x[1] as u32 as u64))) as i8 as i64 as u64) as u32 as u64;
// 0x10e5b068 ldr w9, [x0, #0x18c]
{let address=(cpu.x[0]).wrapping_add(0x18cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10e5b06c ldr w10, [x0, #0x238]
{let address=(cpu.x[0]).wrapping_add(0x238u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10e5b070 madd w0, w9, w8, w10
cpu.x[0]=((((cpu.x[10] as u32 as u64))).wrapping_add((((cpu.x[9] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x10e5b074 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e5b064")),}}}
#[inline]
pub(super) unsafe fn f_10e5b090(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e5b090=>{
// 0x10e5b090 ldr w0, [x0, #0x234]
{let address=(cpu.x[0]).wrapping_add(0x234u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10e5b094 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e5b090")),}}}
#[inline]
pub(super) unsafe fn f_10e806c0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e806c0=>{
// 0x10e806c0 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10e806c4 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10e806c8 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10e806cc ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e806d0 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10e806d4 mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x10e806d8 mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x10e806dc mov x1, x8
cpu.x[1]=(((cpu.x[8]))) as u64;
// 0x10e806e0 bl #0x111dd6a4
cpu.x[30]=0x10e806e4;super::chunk3::f_111dd6a4(cpu,rt,0x111dd6a4)?;
pc=0x10e806e4;continue;
}
0x10e806e4=>{
// 0x10e806e4 ldr x1, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10e806e8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10e806ec bl #0x111dd6a4
cpu.x[30]=0x10e806f0;super::chunk3::f_111dd6a4(cpu,rt,0x111dd6a4)?;
pc=0x10e806f0;continue;
}
0x10e806f0=>{
// 0x10e806f0 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e806f4 ldr w1, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10e806f8 ldr x2, [x8, #0x50]
{let address=(cpu.x[8]).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10e806fc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10e80700 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10e80704 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10e80708 br x2
{let target=cpu.x[2];if target>=0x10e806c0 && target<0x10e8070c{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e806c0")),}}}
#[inline]
pub(super) unsafe fn f_10ec7454(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10ec7454=>{
// 0x10ec7454 mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
// 0x10ec7458 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10ec7454")),}}}
#[inline]
pub(super) unsafe fn f_10ed7264(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10ed7264=>{
// 0x10ed7264 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10ed7268 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10ed726c stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10ed7270 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10ed7274 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10ed7278 mov w0, #0x60
cpu.x[0]=(((0x60u64))) as u32 as u64;
// 0x10ed727c mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10ed7280 bl #0x1187d2b0
cpu.x[30]=0x10ed7284;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
pc=0x10ed7284;continue;
}
0x10ed7284=>{
// 0x10ed7284 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10ed7288 mov x2, x20
cpu.x[2]=(((cpu.x[20]))) as u64;
// 0x10ed728c mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10ed7290 bl #0x10fbf4cc
cpu.x[30]=0x10ed7294;super::chunk4::f_10fbf4cc(cpu,rt,0x10fbf4cc)?;
pc=0x10ed7294;continue;
}
0x10ed7294=>{
// 0x10ed7294 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10ed7298 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10ed729c ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10ed72a0 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10ed72a4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10ed7264")),}}}
#[inline]
pub(super) unsafe fn f_10edcfa0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10edcfa0=>{
// 0x10edcfa0 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10edcfa4 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10edcfa8 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10edcfac mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10edcfb0 ldr x0, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10edcfb4 mov x19, x2
cpu.x[19]=(((cpu.x[2]))) as u64;
// 0x10edcfb8 bl #0x11289eac
cpu.x[30]=0x10edcfbc;super::chunk14::f_11289eac(cpu,rt,0x11289eac)?;
pc=0x10edcfbc;continue;
}
0x10edcfbc=>{
// 0x10edcfbc tbnz w0, #0x1f, #0x10edcfe0
if ((cpu.x[0] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x10edcfe0;continue;}pc=0x10edcfc0;continue;
}
0x10edcfc0=>{
// 0x10edcfc0 ldr x8, [x20, #0x78]
{let address=(cpu.x[20]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10edcfc4 ldr x9, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10edcfc8 ldrsw x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10edcfcc ldr x9, [x9, #0x38]
{let address=(cpu.x[9]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10edcfd0 ldr x9, [x9, w0, uxtw #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[0] as u32 as u64) as u32 as u64)) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10edcfd4 ldr x9, [x9, #0x18]
{let address=(cpu.x[9]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10edcfd8 ldr w8, [x9, x8, lsl #2]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[8])) as u64).wrapping_shl(2) as u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10edcfdc str w8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10edcfe0;continue;
}
0x10edcfe0=>{
// 0x10edcfe0 mvn w8, w0
cpu.x[8]=(!(((cpu.x[0] as u32 as u64)))) as u32 as u64;
// 0x10edcfe4 lsr w0, w8, #0x1f
cpu.x[0]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shr(((((0x1fu64))) & 31) as u32) as u64) as u32 as u64;
// 0x10edcfe8 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10edcfec ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10edcff0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10edcfa0")),}}}
#[inline]
pub(super) unsafe fn f_10f74158(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f74158=>{
// 0x10f74158 mov w0, w1
cpu.x[0]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x10f7415c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f74158")),}}}
#[inline]
pub(super) unsafe fn f_10f74170(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f74170=>{
// 0x10f74170 mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x10f74174 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f74170")),}}}
#[inline]
pub(super) unsafe fn f_10f75100(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f75100=>{
// 0x10f75100 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f75104 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f75108 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f7510c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f75110 mov w19, w1
cpu.x[19]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x10f75114 cmp w1, #0
{let a=((cpu.x[1] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f75118 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10f7511c ubfiz x8, x19, #3, #0x20
cpu.x[8]=(((((cpu.x[19])) & 0xffffffffu64) << 3)) as u64;
// 0x10f75120 csinv x0, x8, xzr, ge
cpu.x[0]=(if cpu.condition(10){((cpu.x[8]))}else{!(((0u64)))}) as u64;
// 0x10f75124 bl #0x1187d520
cpu.x[30]=0x10f75128;super::chunk8::f_1187d520(cpu,rt,0x1187d520)?;
pc=0x10f75128;continue;
}
0x10f75128=>{
// 0x10f75128 ldr w9, [x20, #0xc]
{let address=(cpu.x[20]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f7512c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f75130 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10f75134 cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f75138 b.lt #0x10f751c0
if cpu.condition(11){pc=0x10f751c0;continue;}pc=0x10f7513c;continue;
}
0x10f7513c=>{
// 0x10f7513c cmp w9, #4
{let a=((cpu.x[9] as u32 as u64));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f75140 b.lo #0x10f751a0
if cpu.condition(3){pc=0x10f751a0;continue;}pc=0x10f75144;continue;
}
0x10f75144=>{
// 0x10f75144 sub x10, x21, x8
{let a=((cpu.x[21]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.x[10]=(result) as u64;}
// 0x10f75148 mov x11, x21
cpu.x[11]=(((cpu.x[21]))) as u64;
// 0x10f7514c cmp x10, #0x20
{let a=((cpu.x[10]));let b=((0x20u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f75150 mov x10, x8
cpu.x[10]=(((cpu.x[8]))) as u64;
// 0x10f75154 b.lo #0x10f751a8
if cpu.condition(3){pc=0x10f751a8;continue;}pc=0x10f75158;continue;
}
0x10f75158=>{
// 0x10f75158 mov w12, w9
cpu.x[12]=(((cpu.x[9] as u32 as u64))) as u32 as u64;
// 0x10f7515c add x14, x21, #0x10
{let a=((cpu.x[21]));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[14]=(result) as u64;}
// 0x10f75160 add x15, x8, #0x10
{let a=((cpu.x[8]));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[15]=(result) as u64;}
// 0x10f75164 and x13, x12, #0x7ffffffc
{let a=((cpu.x[12]));let b=((0x7ffffffcu64));let result=a & b;cpu.x[13]=(result) as u64;}
// 0x10f75168 lsl x11, x13, #3
cpu.x[11]=(((((cpu.x[13]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10f7516c sub w9, w9, w13
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[13] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10f75170 mov x16, x13
cpu.x[16]=(((cpu.x[13]))) as u64;
// 0x10f75174 add x10, x8, x11
{let a=((cpu.x[8]));let b=((cpu.x[11]));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x10f75178 add x11, x21, x11
{let a=((cpu.x[21]));let b=((cpu.x[11]));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
pc=0x10f7517c;continue;
}
0x10f7517c=>{
// 0x10f7517c ldp q0, q1, [x15, #-0x10]
{let address=(cpu.x[15]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));let v1=rt.memory.read::<u128>(address.wrapping_add(16));cpu.q[0]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;cpu.q[1]=(v1) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x10f75180 subs x16, x16, #4
{let a=((cpu.x[16]));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.x[16]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x10f75184 add x15, x15, #0x20
{let a=((cpu.x[15]));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[15]=(result) as u64;}
// 0x10f75188 stp q0, q1, [x14, #-0x10]
{let address=(cpu.x[14]).wrapping_add(0xfffffffffffffff0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[1] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10f7518c add x14, x14, #0x20
{let a=((cpu.x[14]));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[14]=(result) as u64;}
// 0x10f75190 b.ne #0x10f7517c
if cpu.condition(1){pc=0x10f7517c;continue;}pc=0x10f75194;continue;
}
0x10f75194=>{
// 0x10f75194 cmp x13, x12
{let a=((cpu.x[13]));let b=((cpu.x[12]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f75198 b.ne #0x10f751a8
if cpu.condition(1){pc=0x10f751a8;continue;}pc=0x10f7519c;continue;
}
0x10f7519c=>{
// 0x10f7519c b #0x10f751c0
pc=0x10f751c0;continue;
}
0x10f751a0=>{
// 0x10f751a0 mov x10, x8
cpu.x[10]=(((cpu.x[8]))) as u64;
// 0x10f751a4 mov x11, x21
cpu.x[11]=(((cpu.x[21]))) as u64;
pc=0x10f751a8;continue;
}
0x10f751a8=>{
// 0x10f751a8 add w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
pc=0x10f751ac;continue;
}
0x10f751ac=>{
// 0x10f751ac ldr x12, [x10], #8
{let address=(cpu.x[10]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[12]=(v0) as u64;cpu.x[10]=((cpu.x[10]).wrapping_add(((0x8u64)))) as u64;}
// 0x10f751b0 sub w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10f751b4 cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f751b8 str x12, [x11], #8
{let address=(cpu.x[11]).wrapping_add(0x0u64);let v0=cpu.x[12];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);cpu.x[11]=((cpu.x[11]).wrapping_add(((0x8u64)))) as u64;}
// 0x10f751bc b.hi #0x10f751ac
if cpu.condition(8){pc=0x10f751ac;continue;}pc=0x10f751c0;continue;
}
0x10f751c0=>{
// 0x10f751c0 cbz x8, #0x10f751cc
if (cpu.x[8]) == 0 {pc=0x10f751cc;continue;}pc=0x10f751c4;continue;
}
0x10f751c4=>{
// 0x10f751c4 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10f751c8 bl #0x1187d360
cpu.x[30]=0x10f751cc;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x10f751cc;continue;
}
0x10f751cc=>{
// 0x10f751cc str x21, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f751d0 str w19, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=cpu.x[19] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f751d4 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f751d8 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f751dc ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f751e0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f75100")),}}}
#[inline]
pub(super) unsafe fn f_10f77f18(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f77f18=>{
// 0x10f77f18 ldr w8, [x0, #0x5c]
{let address=(cpu.x[0]).wrapping_add(0x5cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f77f1c cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f77f20 b.lt #0x10f77f74
if cpu.condition(11){pc=0x10f77f74;continue;}pc=0x10f77f24;continue;
}
0x10f77f24=>{
// 0x10f77f24 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f77f28 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f77f2c stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f77f30 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f77f34 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f77f38 mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x10f77f3c sub x21, x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[21]=(result) as u64;}
pc=0x10f77f40;continue;
}
0x10f77f40=>{
// 0x10f77f40 ldr x8, [x19, #0x50]
{let address=(cpu.x[19]).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f77f44 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10f77f48 ldr x0, [x8, x21, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[21])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f77f4c ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f77f50 ldr x8, [x8, #0x50]
{let address=(cpu.x[8]).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f77f54 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f77f58;super::dispatch(cpu,rt,target)?;}
pc=0x10f77f58;continue;
}
0x10f77f58=>{
// 0x10f77f58 add x8, x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10f77f5c sub x21, x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[21]=(result) as u64;}
// 0x10f77f60 cmp x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f77f64 b.hi #0x10f77f40
if cpu.condition(8){pc=0x10f77f40;continue;}pc=0x10f77f68;continue;
}
0x10f77f68=>{
// 0x10f77f68 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f77f6c ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f77f70 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
pc=0x10f77f74;continue;
}
0x10f77f74=>{
// 0x10f77f74 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f77f18")),}}}
#[inline]
pub(super) unsafe fn f_10f79a48(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f79a48=>{
// 0x10f79a48 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f79a4c str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f79a50 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f79a54 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f79a58 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10f79a5c mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f79a60 adrp x8, #0x11935000
cpu.x[8]=(0x11935000u64) as u64;
// 0x10f79a64 add x8, x8, #0x6e8
{let a=((cpu.x[8]));let b=((0x6e8u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10f79a68 ldr x0, [x21, #0x50]!
{let address=(cpu.x[21]).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;cpu.x[21]=(address) as u64;}
// 0x10f79a6c str x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f79a70 cbz x0, #0x10f79a78
if (cpu.x[0]) == 0 {pc=0x10f79a78;continue;}pc=0x10f79a74;continue;
}
0x10f79a74=>{
// 0x10f79a74 bl #0x1187d360
cpu.x[30]=0x10f79a78;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x10f79a78;continue;
}
0x10f79a78=>{
// 0x10f79a78 mov x20, x19
cpu.x[20]=(((cpu.x[19]))) as u64;
// 0x10f79a7c stp xzr, xzr, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f79a80 ldr x0, [x20, #0x40]!
{let address=(cpu.x[20]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;cpu.x[20]=(address) as u64;}
// 0x10f79a84 cbz x0, #0x10f79a8c
if (cpu.x[0]) == 0 {pc=0x10f79a8c;continue;}pc=0x10f79a88;continue;
}
0x10f79a88=>{
// 0x10f79a88 bl #0x1187d360
cpu.x[30]=0x10f79a8c;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x10f79a8c;continue;
}
0x10f79a8c=>{
// 0x10f79a8c mov x21, x19
cpu.x[21]=(((cpu.x[19]))) as u64;
// 0x10f79a90 stp xzr, xzr, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f79a94 ldr x0, [x21, #0x30]!
{let address=(cpu.x[21]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;cpu.x[21]=(address) as u64;}
// 0x10f79a98 cbz x0, #0x10f79aa0
if (cpu.x[0]) == 0 {pc=0x10f79aa0;continue;}pc=0x10f79a9c;continue;
}
0x10f79a9c=>{
// 0x10f79a9c bl #0x1187d360
cpu.x[30]=0x10f79aa0;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x10f79aa0;continue;
}
0x10f79aa0=>{
// 0x10f79aa0 mov x20, x19
cpu.x[20]=(((cpu.x[19]))) as u64;
// 0x10f79aa4 stp xzr, xzr, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f79aa8 ldr x0, [x20, #0x20]!
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;cpu.x[20]=(address) as u64;}
// 0x10f79aac cbz x0, #0x10f79ab4
if (cpu.x[0]) == 0 {pc=0x10f79ab4;continue;}pc=0x10f79ab0;continue;
}
0x10f79ab0=>{
// 0x10f79ab0 bl #0x1187d360
cpu.x[30]=0x10f79ab4;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x10f79ab4;continue;
}
0x10f79ab4=>{
// 0x10f79ab4 ldr x0, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f79ab8 stp xzr, xzr, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f79abc cbz x0, #0x10f79ac4
if (cpu.x[0]) == 0 {pc=0x10f79ac4;continue;}pc=0x10f79ac0;continue;
}
0x10f79ac0=>{
// 0x10f79ac0 bl #0x1187d360
cpu.x[30]=0x10f79ac4;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x10f79ac4;continue;
}
0x10f79ac4=>{
// 0x10f79ac4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f79ac8 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f79acc ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f79ad0 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f79ad4 b #0x1187d2c0
super::dispatch(cpu,rt,0x1187d2c0)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f79a48")),}}}
#[inline]
pub(super) unsafe fn f_10f7ba2c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f7ba2c=>{
// 0x10f7ba2c stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f7ba30 str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f7ba34 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f7ba38 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f7ba3c ldr x0, [x0, #0x120]
{let address=(cpu.x[0]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f7ba40 adrp x8, #0x11935000
cpu.x[8]=(0x11935000u64) as u64;
// 0x10f7ba44 add x8, x8, #0x708
{let a=((cpu.x[8]));let b=((0x708u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10f7ba48 str x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f7ba4c cbz x0, #0x10f7ba54
if (cpu.x[0]) == 0 {pc=0x10f7ba54;continue;}pc=0x10f7ba50;continue;
}
0x10f7ba50=>{
// 0x10f7ba50 bl #0x1187d360
cpu.x[30]=0x10f7ba54;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x10f7ba54;continue;
}
0x10f7ba54=>{
// 0x10f7ba54 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7ba58 stp xzr, xzr, [x19, #0x120]
{let address=(cpu.x[19]).wrapping_add(0x120u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f7ba5c bl #0x10f7bb1c
cpu.x[30]=0x10f7ba60;super::chunk15::f_10f7bb1c(cpu,rt,0x10f7bb1c)?;
pc=0x10f7ba60;continue;
}
0x10f7ba60=>{
// 0x10f7ba60 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7ba64 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10f7ba68 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f7ba6c b #0x1187d2c0
super::dispatch(cpu,rt,0x1187d2c0)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f7ba2c")),}}}
#[inline]
pub(super) unsafe fn f_10f7bac4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f7bac4=>{
// 0x10f7bac4 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10f7bac8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f7bac4")),}}}
#[inline]
pub(super) unsafe fn f_10f7e13c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f7e13c=>{
// 0x10f7e13c stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f7e140 stp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f7e144 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f7e148 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f7e14c ldr w22, [x0, #0x120]
{let address=(cpu.x[0]).wrapping_add(0x120u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[22]=(v0) as u32 as u64;}
// 0x10f7e150 ldr x21, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f7e154 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f7e158 mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x10f7e15c cmn w22, #1
{let a=((cpu.x[22] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.flags(a,b,result,32,false);}
// 0x10f7e160 b.ne #0x10f7e174
if cpu.condition(1){pc=0x10f7e174;continue;}pc=0x10f7e164;continue;
}
0x10f7e164=>{
// 0x10f7e164 adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x10f7e168 add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10f7e16c mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10f7e170 bl #0x1128df18
cpu.x[30]=0x10f7e174;super::dispatch(cpu,rt,0x1128df18)?;
pc=0x10f7e174;continue;
}
0x10f7e174=>{
// 0x10f7e174 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e178 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e17c mov w1, w22
cpu.x[1]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10f7e180 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e184 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e188;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e188;continue;
}
0x10f7e188=>{
// 0x10f7e188 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e18c ldr w1, [x19, #0x124]
{let address=(cpu.x[19]).wrapping_add(0x124u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e190 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e194 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e198 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e19c;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e19c;continue;
}
0x10f7e19c=>{
// 0x10f7e19c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7e1a0 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10f7e1a4 bl #0x10f93bd8
cpu.x[30]=0x10f7e1a8;super::chunk14::f_10f93bd8(cpu,rt,0x10f93bd8)?;
pc=0x10f7e1a8;continue;
}
0x10f7e1a8=>{
// 0x10f7e1a8 add x1, x19, #0xf8
{let a=((cpu.x[19]));let b=((0xf8u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x10f7e1ac mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e1b0 bl #0x111dd2a0
cpu.x[30]=0x10f7e1b4;super::chunk12::f_111dd2a0(cpu,rt,0x111dd2a0)?;
pc=0x10f7e1b4;continue;
}
0x10f7e1b4=>{
// 0x10f7e1b4 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e1b8 ldr w1, [x19, #0x128]
{let address=(cpu.x[19]).wrapping_add(0x128u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e1bc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e1c0 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e1c4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e1c8;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e1c8;continue;
}
0x10f7e1c8=>{
// 0x10f7e1c8 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e1cc ldr w1, [x19, #0x12c]
{let address=(cpu.x[19]).wrapping_add(0x12cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e1d0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e1d4 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e1d8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e1dc;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e1dc;continue;
}
0x10f7e1dc=>{
// 0x10f7e1dc ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e1e0 ldr w1, [x19, #0x11c]
{let address=(cpu.x[19]).wrapping_add(0x11cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e1e4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e1e8 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e1ec blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e1f0;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e1f0;continue;
}
0x10f7e1f0=>{
// 0x10f7e1f0 ldr x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f7e1f4 bl #0x10de05c4
cpu.x[30]=0x10f7e1f8;super::chunk5::f_10de05c4(cpu,rt,0x10de05c4)?;
pc=0x10f7e1f8;continue;
}
0x10f7e1f8=>{
// 0x10f7e1f8 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7e1fc b.lt #0x10f7e228
if cpu.condition(11){pc=0x10f7e228;continue;}pc=0x10f7e200;continue;
}
0x10f7e200=>{
// 0x10f7e200 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e204 ldr w1, [x19, #0x13c]
{let address=(cpu.x[19]).wrapping_add(0x13cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e208 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e20c ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e210 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e214;super::dispatch(cpu,rt,target)?;}
// 0x10f7e214 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e218 ldr w1, [x19, #0x140]
{let address=(cpu.x[19]).wrapping_add(0x140u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e21c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e220 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e224 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e228;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e228;continue;
}
0x10f7e228=>{
// 0x10f7e228 ldr x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f7e22c bl #0x10de0720
cpu.x[30]=0x10f7e230;super::chunk3::f_10de0720(cpu,rt,0x10de0720)?;
pc=0x10f7e230;continue;
}
0x10f7e230=>{
// 0x10f7e230 cbnz x0, #0x10f7e248
if (cpu.x[0]) != 0 {pc=0x10f7e248;continue;}pc=0x10f7e234;continue;
}
0x10f7e234=>{
// 0x10f7e234 ldr x8, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e238 ldr x9, [x8, #0x258]
{let address=(cpu.x[8]).wrapping_add(0x258u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f7e23c cbnz x9, #0x10f7e248
if (cpu.x[9]) != 0 {pc=0x10f7e248;continue;}pc=0x10f7e240;continue;
}
0x10f7e240=>{
// 0x10f7e240 ldr x8, [x8, #0x360]
{let address=(cpu.x[8]).wrapping_add(0x360u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e244 cbz x8, #0x10f7e2a4
if (cpu.x[8]) == 0 {pc=0x10f7e2a4;continue;}pc=0x10f7e248;continue;
}
0x10f7e248=>{
// 0x10f7e248 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e24c ldr w1, [x19, #0x144]
{let address=(cpu.x[19]).wrapping_add(0x144u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e250 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e254 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e258 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e25c;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e25c;continue;
}
0x10f7e25c=>{
// 0x10f7e25c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e260 ldr w1, [x19, #0x148]
{let address=(cpu.x[19]).wrapping_add(0x148u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e264 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e268 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e26c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e270;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e270;continue;
}
0x10f7e270=>{
// 0x10f7e270 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e274 ldr w1, [x19, #0x14c]
{let address=(cpu.x[19]).wrapping_add(0x14cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e278 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e27c ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e280 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e284;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e284;continue;
}
0x10f7e284=>{
// 0x10f7e284 ldr x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f7e288 bl #0x10de0728
cpu.x[30]=0x10f7e28c;super::chunk13::f_10de0728(cpu,rt,0x10de0728)?;
pc=0x10f7e28c;continue;
}
0x10f7e28c=>{
// 0x10f7e28c cbz x0, #0x10f7e2a4
if (cpu.x[0]) == 0 {pc=0x10f7e2a4;continue;}pc=0x10f7e290;continue;
}
0x10f7e290=>{
// 0x10f7e290 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e294 ldr w1, [x19, #0x150]
{let address=(cpu.x[19]).wrapping_add(0x150u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e298 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e29c ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e2a0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e2a4;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e2a4;continue;
}
0x10f7e2a4=>{
// 0x10f7e2a4 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e2a8 ldr w1, [x19, #0x158]
{let address=(cpu.x[19]).wrapping_add(0x158u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e2ac mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e2b0 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e2b4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e2b8;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e2b8;continue;
}
0x10f7e2b8=>{
// 0x10f7e2b8 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e2bc ldr w1, [x19, #0x15c]
{let address=(cpu.x[19]).wrapping_add(0x15cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e2c0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e2c4 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e2c8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e2cc;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e2cc;continue;
}
0x10f7e2cc=>{
// 0x10f7e2cc ldr x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f7e2d0 bl #0x10de1240
cpu.x[30]=0x10f7e2d4;super::chunk12::f_10de1240(cpu,rt,0x10de1240)?;
pc=0x10f7e2d4;continue;
}
0x10f7e2d4=>{
// 0x10f7e2d4 tbnz w0, #0, #0x10f7e2f8
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10f7e2f8;continue;}pc=0x10f7e2d8;continue;
}
0x10f7e2d8=>{
// 0x10f7e2d8 ldr x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f7e2dc bl #0x10de1248
cpu.x[30]=0x10f7e2e0;super::chunk8::f_10de1248(cpu,rt,0x10de1248)?;
pc=0x10f7e2e0;continue;
}
0x10f7e2e0=>{
// 0x10f7e2e0 tbnz w0, #0, #0x10f7e2f8
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10f7e2f8;continue;}pc=0x10f7e2e4;continue;
}
0x10f7e2e4=>{
// 0x10f7e2e4 ldr x8, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e2e8 ldr x0, [x8, #0x298]
{let address=(cpu.x[8]).wrapping_add(0x298u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f7e2ec cbz x0, #0x10f7e30c
if (cpu.x[0]) == 0 {pc=0x10f7e30c;continue;}pc=0x10f7e2f0;continue;
}
0x10f7e2f0=>{
// 0x10f7e2f0 bl #0x10ddacf0
cpu.x[30]=0x10f7e2f4;super::chunk3::f_10ddacf0(cpu,rt,0x10ddacf0)?;
// 0x10f7e2f4 tbz w0, #0, #0x10f7e30c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f7e30c;continue;}pc=0x10f7e2f8;continue;
}
0x10f7e2f8=>{
// 0x10f7e2f8 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e2fc ldr w1, [x19, #0x138]
{let address=(cpu.x[19]).wrapping_add(0x138u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e300 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e304 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e308 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e30c;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e30c;continue;
}
0x10f7e30c=>{
// 0x10f7e30c ldr x8, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e310 ldrb w9, [x8, #0x3a4]
{let address=(cpu.x[8]).wrapping_add(0x3a4u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f7e314 cbnz w9, #0x10f7e320
if (cpu.x[9] as u32 as u64) != 0 {pc=0x10f7e320;continue;}pc=0x10f7e318;continue;
}
0x10f7e318=>{
// 0x10f7e318 ldrb w8, [x8, #0x98]
{let address=(cpu.x[8]).wrapping_add(0x98u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f7e31c tbz w8, #0, #0x10f7e354
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f7e354;continue;}pc=0x10f7e320;continue;
}
0x10f7e320=>{
// 0x10f7e320 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e324 ldr w1, [x19, #0x130]
{let address=(cpu.x[19]).wrapping_add(0x130u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e328 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e32c ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e330 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e334;super::dispatch(cpu,rt,target)?;}
// 0x10f7e334 ldr x8, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e338 ldrb w8, [x8, #0x98]
{let address=(cpu.x[8]).wrapping_add(0x98u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f7e33c tbz w8, #0, #0x10f7e354
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f7e354;continue;}pc=0x10f7e340;continue;
}
0x10f7e340=>{
// 0x10f7e340 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e344 ldr w1, [x19, #0x134]
{let address=(cpu.x[19]).wrapping_add(0x134u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e348 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e34c ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e350 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e354;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e354;continue;
}
0x10f7e354=>{
// 0x10f7e354 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e358 ldr w1, [x19, #0x114]
{let address=(cpu.x[19]).wrapping_add(0x114u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e35c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e360 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e364 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e368;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e368;continue;
}
0x10f7e368=>{
// 0x10f7e368 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e36c ldr w1, [x19, #0x118]
{let address=(cpu.x[19]).wrapping_add(0x118u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e370 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e374 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e378 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e37c;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e37c;continue;
}
0x10f7e37c=>{
// 0x10f7e37c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e380 ldr w1, [x19, #0x1d8]
{let address=(cpu.x[19]).wrapping_add(0x1d8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e384 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e388 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e38c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e390;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e390;continue;
}
0x10f7e390=>{
// 0x10f7e390 ldr x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f7e394 ldr w8, [x0, #0x3ac]
{let address=(cpu.x[0]).wrapping_add(0x3acu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f7e398 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7e39c b.lt #0x10f7e3b8
if cpu.condition(11){pc=0x10f7e3b8;continue;}pc=0x10f7e3a0;continue;
}
0x10f7e3a0=>{
// 0x10f7e3a0 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e3a4 ldr w1, [x19, #0x154]
{let address=(cpu.x[19]).wrapping_add(0x154u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e3a8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e3ac ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e3b0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e3b4;super::dispatch(cpu,rt,target)?;}
// 0x10f7e3b4 ldr x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
pc=0x10f7e3b8;continue;
}
0x10f7e3b8=>{
// 0x10f7e3b8 bl #0x10de06a0
cpu.x[30]=0x10f7e3bc;super::chunk9::f_10de06a0(cpu,rt,0x10de06a0)?;
pc=0x10f7e3bc;continue;
}
0x10f7e3bc=>{
// 0x10f7e3bc tbz w0, #0, #0x10f7e3d4
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f7e3d4;continue;}pc=0x10f7e3c0;continue;
}
0x10f7e3c0=>{
// 0x10f7e3c0 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e3c4 ldrb w1, [x19, #0x171]
{let address=(cpu.x[19]).wrapping_add(0x171u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e3c8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e3cc ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e3d0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e3d4;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e3d4;continue;
}
0x10f7e3d4=>{
// 0x10f7e3d4 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e3d8 ldrb w1, [x19, #0x172]
{let address=(cpu.x[19]).wrapping_add(0x172u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e3dc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e3e0 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e3e4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e3e8;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e3e8;continue;
}
0x10f7e3e8=>{
// 0x10f7e3e8 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e3ec ldrb w1, [x19, #0x1c4]
{let address=(cpu.x[19]).wrapping_add(0x1c4u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e3f0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e3f4 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e3f8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e3fc;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e3fc;continue;
}
0x10f7e3fc=>{
// 0x10f7e3fc ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
pc=0x10f7e400;continue;
}
0x10f7e400=>{
// 0x10f7e400 ldrb w1, [x19, #0x174]
{let address=(cpu.x[19]).wrapping_add(0x174u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e404 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e408 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e40c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e410;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e410;continue;
}
0x10f7e410=>{
// 0x10f7e410 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e414 ldrb w1, [x19, #0x175]
{let address=(cpu.x[19]).wrapping_add(0x175u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e418 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e41c ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e420 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e424;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e424;continue;
}
0x10f7e424=>{
// 0x10f7e424 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e428 ldrb w1, [x19, #0x110]
{let address=(cpu.x[19]).wrapping_add(0x110u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e42c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e430 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e434 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e438;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e438;continue;
}
0x10f7e438=>{
// 0x10f7e438 ldrb w8, [x19, #0x174]
{let address=(cpu.x[19]).wrapping_add(0x174u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f7e43c cbz w8, #0x10f7e454
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f7e454;continue;}pc=0x10f7e440;continue;
}
0x10f7e440=>{
// 0x10f7e440 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e444 ldr w1, [x19, #0x178]
{let address=(cpu.x[19]).wrapping_add(0x178u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e448 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e44c ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e450 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e454;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e454;continue;
}
0x10f7e454=>{
// 0x10f7e454 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e458 ldr w1, [x19, #0x1a0]
{let address=(cpu.x[19]).wrapping_add(0x1a0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e45c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e460 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e464 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e468;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e468;continue;
}
0x10f7e468=>{
// 0x10f7e468 ldr x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f7e46c bl #0x10de08c0
cpu.x[30]=0x10f7e470;super::chunk3::f_10de08c0(cpu,rt,0x10de08c0)?;
pc=0x10f7e470;continue;
}
0x10f7e470=>{
// 0x10f7e470 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7e474 b.lt #0x10f7e48c
if cpu.condition(11){pc=0x10f7e48c;continue;}pc=0x10f7e478;continue;
}
0x10f7e478=>{
// 0x10f7e478 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e47c ldr w1, [x19, #0x1a4]
{let address=(cpu.x[19]).wrapping_add(0x1a4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e480 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e484 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e488 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e48c;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e48c;continue;
}
0x10f7e48c=>{
// 0x10f7e48c ldr x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f7e490 bl #0x10de08d0
cpu.x[30]=0x10f7e494;super::chunk15::f_10de08d0(cpu,rt,0x10de08d0)?;
pc=0x10f7e494;continue;
}
0x10f7e494=>{
// 0x10f7e494 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7e498 b.lt #0x10f7e4b0
if cpu.condition(11){pc=0x10f7e4b0;continue;}pc=0x10f7e49c;continue;
}
0x10f7e49c=>{
// 0x10f7e49c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e4a0 ldr w1, [x19, #0x1a8]
{let address=(cpu.x[19]).wrapping_add(0x1a8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e4a4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e4a8 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e4ac blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e4b0;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e4b0;continue;
}
0x10f7e4b0=>{
// 0x10f7e4b0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7e4b4 bl #0x10f93d2c
cpu.x[30]=0x10f7e4b8;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
pc=0x10f7e4b8;continue;
}
0x10f7e4b8=>{
// 0x10f7e4b8 bl #0x10d36084
cpu.x[30]=0x10f7e4bc;super::chunk6::f_10d36084(cpu,rt,0x10d36084)?;
pc=0x10f7e4bc;continue;
}
0x10f7e4bc=>{
// 0x10f7e4bc tbz w0, #0, #0x10f7e4d4
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f7e4d4;continue;}pc=0x10f7e4c0;continue;
}
0x10f7e4c0=>{
// 0x10f7e4c0 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e4c4 ldrb w1, [x19, #0x170]
{let address=(cpu.x[19]).wrapping_add(0x170u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e4c8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e4cc ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e4d0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e4d4;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e4d4;continue;
}
0x10f7e4d4=>{
// 0x10f7e4d4 add x0, x19, #0x198
{let a=((cpu.x[19]));let b=((0x198u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10f7e4d8 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10f7e4dc bl #0x112942c8
cpu.x[30]=0x10f7e4e0;super::chunk0::f_112942c8(cpu,rt,0x112942c8)?;
pc=0x10f7e4e0;continue;
}
0x10f7e4e0=>{
// 0x10f7e4e0 ldr x8, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e4e4 ldr w9, [x8, #0x5a8]
{let address=(cpu.x[8]).wrapping_add(0x5a8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f7e4e8 cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7e4ec b.lt #0x10f7e508
if cpu.condition(11){pc=0x10f7e508;continue;}pc=0x10f7e4f0;continue;
}
0x10f7e4f0=>{
// 0x10f7e4f0 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e4f4 ldr w1, [x19, #0x1ac]
{let address=(cpu.x[19]).wrapping_add(0x1acu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e4f8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e4fc ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e500 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e504;super::dispatch(cpu,rt,target)?;}
// 0x10f7e504 ldr x8, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
pc=0x10f7e508;continue;
}
0x10f7e508=>{
// 0x10f7e508 ldr w9, [x8, #0x684]
{let address=(cpu.x[8]).wrapping_add(0x684u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f7e50c cbnz w9, #0x10f7e51c
if (cpu.x[9] as u32 as u64) != 0 {pc=0x10f7e51c;continue;}pc=0x10f7e510;continue;
}
0x10f7e510=>{
// 0x10f7e510 ldr w8, [x8, #0x4c8]
{let address=(cpu.x[8]).wrapping_add(0x4c8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f7e514 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7e518 b.lt #0x10f7e530
if cpu.condition(11){pc=0x10f7e530;continue;}pc=0x10f7e51c;continue;
}
0x10f7e51c=>{
// 0x10f7e51c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e520 ldr w1, [x19, #0x1b4]
{let address=(cpu.x[19]).wrapping_add(0x1b4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e524 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e528 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e52c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e530;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e530;continue;
}
0x10f7e530=>{
// 0x10f7e530 ldr x8, [x19, #0x180]
{let address=(cpu.x[19]).wrapping_add(0x180u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e534 ldr x9, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f7e538 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e53c cmp x8, #0
{let a=((cpu.x[8]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f7e540 ldr x8, [x9, #0x40]
{let address=(cpu.x[9]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e544 cset w1, ne
cpu.x[1]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x10f7e548 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e54c;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e54c;continue;
}
0x10f7e54c=>{
// 0x10f7e54c ldr x2, [x19, #0x180]
{let address=(cpu.x[19]).wrapping_add(0x180u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10f7e550 cbz x2, #0x10f7e564
if (cpu.x[2]) == 0 {pc=0x10f7e564;continue;}pc=0x10f7e554;continue;
}
0x10f7e554=>{
// 0x10f7e554 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f7e558 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10f7e55c mov w3, #5
cpu.x[3]=(((0x5u64))) as u32 as u64;
// 0x10f7e560 bl #0x10f8bfc0
cpu.x[30]=0x10f7e564;super::chunk11::f_10f8bfc0(cpu,rt,0x10f8bfc0)?;
pc=0x10f7e564;continue;
}
0x10f7e564=>{
// 0x10f7e564 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e568 ldr w1, [x19, #0x190]
{let address=(cpu.x[19]).wrapping_add(0x190u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e56c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e570 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e574 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e578;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e578;continue;
}
0x10f7e578=>{
// 0x10f7e578 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e57c ldr w1, [x19, #0x194]
{let address=(cpu.x[19]).wrapping_add(0x194u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e580 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e584 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e588 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e58c;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e58c;continue;
}
0x10f7e58c=>{
// 0x10f7e58c ldr x2, [x19, #0x188]
{let address=(cpu.x[19]).wrapping_add(0x188u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10f7e590 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f7e594 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10f7e598 bl #0x10f8c0ac
cpu.x[30]=0x10f7e59c;super::chunk1::f_10f8c0ac(cpu,rt,0x10f8c0ac)?;
pc=0x10f7e59c;continue;
}
0x10f7e59c=>{
// 0x10f7e59c ldr x0, [x19, #0x1b8]
{let address=(cpu.x[19]).wrapping_add(0x1b8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f7e5a0 cbz x0, #0x10f7e5b0
if (cpu.x[0]) == 0 {pc=0x10f7e5b0;continue;}pc=0x10f7e5a4;continue;
}
0x10f7e5a4=>{
// 0x10f7e5a4 bl #0x10e0b624
cpu.x[30]=0x10f7e5a8;super::chunk10::f_10e0b624(cpu,rt,0x10e0b624)?;
// 0x10f7e5a8 mov w1, w0
cpu.x[1]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f7e5ac b #0x10f7e5b4
pc=0x10f7e5b4;continue;
}
0x10f7e5b0=>{
// 0x10f7e5b0 mov w1, #-1
cpu.x[1]=(((0xffffffffffffffffu64))) as u32 as u64;
pc=0x10f7e5b4;continue;
}
0x10f7e5b4=>{
// 0x10f7e5b4 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e5b8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e5bc ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e5c0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e5c4;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e5c4;continue;
}
0x10f7e5c4=>{
// 0x10f7e5c4 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e5c8 ldr w1, [x19, #0x1c0]
{let address=(cpu.x[19]).wrapping_add(0x1c0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e5cc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e5d0 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e5d4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e5d8;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e5d8;continue;
}
0x10f7e5d8=>{
// 0x10f7e5d8 ldr x8, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e5dc ldr x8, [x8, #0x710]
{let address=(cpu.x[8]).wrapping_add(0x710u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e5e0 cbz x8, #0x10f7e640
if (cpu.x[8]) == 0 {pc=0x10f7e640;continue;}pc=0x10f7e5e4;continue;
}
0x10f7e5e4=>{
// 0x10f7e5e4 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e5e8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7e5ec ldr x8, [x8, #0x140]
{let address=(cpu.x[8]).wrapping_add(0x140u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e5f0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e5f4;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e5f4;continue;
}
0x10f7e5f4=>{
// 0x10f7e5f4 tbz w0, #0, #0x10f7e604
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f7e604;continue;}pc=0x10f7e5f8;continue;
}
0x10f7e5f8=>{
// 0x10f7e5f8 ldr x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f7e5fc bl #0x10de1648
cpu.x[30]=0x10f7e600;super::chunk8::f_10de1648(cpu,rt,0x10de1648)?;
// 0x10f7e600 tbnz w0, #0, #0x10f7e640
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10f7e640;continue;}pc=0x10f7e604;continue;
}
0x10f7e604=>{
// 0x10f7e604 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e608 ldrb w1, [x19, #0x1e0]
{let address=(cpu.x[19]).wrapping_add(0x1e0u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e60c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e610 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e614 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e618;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e618;continue;
}
0x10f7e618=>{
// 0x10f7e618 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e61c ldr w1, [x19, #0x1e4]
{let address=(cpu.x[19]).wrapping_add(0x1e4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e620 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e624 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e628 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e62c;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e62c;continue;
}
0x10f7e62c=>{
// 0x10f7e62c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e630 ldr w1, [x19, #0x1e8]
{let address=(cpu.x[19]).wrapping_add(0x1e8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e634 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e638 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e63c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e640;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e640;continue;
}
0x10f7e640=>{
// 0x10f7e640 ldr x1, [x19, #0x1d0]
{let address=(cpu.x[19]).wrapping_add(0x1d0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10f7e644 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e648 bl #0x111dd6a4
cpu.x[30]=0x10f7e64c;super::chunk3::f_111dd6a4(cpu,rt,0x111dd6a4)?;
pc=0x10f7e64c;continue;
}
0x10f7e64c=>{
// 0x10f7e64c ldrb w8, [x19, #0x110]
{let address=(cpu.x[19]).wrapping_add(0x110u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f7e650 cbz w8, #0x10f7e674
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f7e674;continue;}pc=0x10f7e654;continue;
}
0x10f7e654=>{
// 0x10f7e654 ldr x2, [x19, #0x100]
{let address=(cpu.x[19]).wrapping_add(0x100u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10f7e658 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f7e65c mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10f7e660 bl #0x10f8c0ac
cpu.x[30]=0x10f7e664;super::chunk1::f_10f8c0ac(cpu,rt,0x10f8c0ac)?;
// 0x10f7e664 ldr x2, [x19, #0x108]
{let address=(cpu.x[19]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10f7e668 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f7e66c mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10f7e670 bl #0x10f8c0ac
cpu.x[30]=0x10f7e674;super::chunk1::f_10f8c0ac(cpu,rt,0x10f8c0ac)?;
pc=0x10f7e674;continue;
}
0x10f7e674=>{
// 0x10f7e674 ldr x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f7e678 bl #0x10de0bb0
cpu.x[30]=0x10f7e67c;super::chunk3::f_10de0bb0(cpu,rt,0x10de0bb0)?;
pc=0x10f7e67c;continue;
}
0x10f7e67c=>{
// 0x10f7e67c cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7e680 b.lt #0x10f7e698
if cpu.condition(11){pc=0x10f7e698;continue;}pc=0x10f7e684;continue;
}
0x10f7e684=>{
// 0x10f7e684 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e688 ldr w1, [x19, #0x1f0]
{let address=(cpu.x[19]).wrapping_add(0x1f0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e68c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e690 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e694 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e698;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e698;continue;
}
0x10f7e698=>{
// 0x10f7e698 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e69c ldr w1, [x19, #0x1f8]
{let address=(cpu.x[19]).wrapping_add(0x1f8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e6a0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e6a4 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e6a8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e6ac;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e6ac;continue;
}
0x10f7e6ac=>{
// 0x10f7e6ac ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e6b0 ldrb w1, [x19, #0x1fc]
{let address=(cpu.x[19]).wrapping_add(0x1fcu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e6b4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e6b8 ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e6bc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e6c0;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e6c0;continue;
}
0x10f7e6c0=>{
// 0x10f7e6c0 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e6c4 ldrb w1, [x19, #0x1ec]
{let address=(cpu.x[19]).wrapping_add(0x1ecu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e6c8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e6cc ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e6d0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f7e6d4;super::dispatch(cpu,rt,target)?;}
pc=0x10f7e6d4;continue;
}
0x10f7e6d4=>{
// 0x10f7e6d4 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7e6d8 ldr w1, [x19, #0x200]
{let address=(cpu.x[19]).wrapping_add(0x200u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7e6dc ldr x2, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10f7e6e0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7e6e4 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f7e6e8 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f7e6ec ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f7e6f0 br x2
{let target=cpu.x[2];if target>=0x10f7e13c && target<0x10f7e6f4{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f7e13c")),}}}
#[inline]
pub(super) unsafe fn f_10f80080(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f80080=>{
// 0x10f80080 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f80084 str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f80088 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f8008c mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f80090 bl #0x10f96040
cpu.x[30]=0x10f80094;super::chunk14::f_10f96040(cpu,rt,0x10f96040)?;
pc=0x10f80094;continue;
}
0x10f80094=>{
// 0x10f80094 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f80098 strb wzr, [x19, #0x1fc]
{let address=(cpu.x[19]).wrapping_add(0x1fcu64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f8009c bl #0x10f9433c
cpu.x[30]=0x10f800a0;super::chunk3::f_10f9433c(cpu,rt,0x10f9433c)?;
pc=0x10f800a0;continue;
}
0x10f800a0=>{
// 0x10f800a0 cbz x0, #0x10f800b0
if (cpu.x[0]) == 0 {pc=0x10f800b0;continue;}pc=0x10f800a4;continue;
}
0x10f800a4=>{
// 0x10f800a4 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10f800a8 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f800ac b #0x10fc4240
super::chunk15::f_10fc4240(cpu,rt,0x10fc4240)?;return Ok(());
}
0x10f800b0=>{
// 0x10f800b0 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10f800b4 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f800b8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f80080")),}}}
#[inline]
pub(super) unsafe fn f_10f83114(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f83114=>{
// 0x10f83114 strb w1, [x0, #0x172]
{let address=(cpu.x[0]).wrapping_add(0x172u64);let v0=cpu.x[1] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f83118 tbz w1, #0, #0x10f83284
if ((cpu.x[1] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f83284;continue;}pc=0x10f8311c;continue;
}
0x10f8311c=>{
// 0x10f8311c stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f83120 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f83124 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f83128 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f8312c ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f83130 mov x20, x2
cpu.x[20]=(((cpu.x[2]))) as u64;
// 0x10f83134 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f83138 ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8313c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f83140;super::dispatch(cpu,rt,target)?;}
pc=0x10f83140;continue;
}
0x10f83140=>{
// 0x10f83140 cbz x0, #0x10f83278
if (cpu.x[0]) == 0 {pc=0x10f83278;continue;}pc=0x10f83144;continue;
}
0x10f83144=>{
// 0x10f83144 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f83148 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8314c ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f83150 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f83154;super::dispatch(cpu,rt,target)?;}
pc=0x10f83154;continue;
}
0x10f83154=>{
// 0x10f83154 mov w21, #1
cpu.x[21]=(((0x1u64))) as u32 as u64;
// 0x10f83158 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8315c str w21, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=cpu.x[21] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f83160 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f83164 ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f83168 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8316c;super::dispatch(cpu,rt,target)?;}
pc=0x10f8316c;continue;
}
0x10f8316c=>{
// 0x10f8316c str w21, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[21] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f83170 cbz x20, #0x10f831a0
if (cpu.x[20]) == 0 {pc=0x10f831a0;continue;}pc=0x10f83174;continue;
}
0x10f83174=>{
// 0x10f83174 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f83178 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f8317c ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f83180 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f83184;super::dispatch(cpu,rt,target)?;}
// 0x10f83184 cbz x0, #0x10f831a0
if (cpu.x[0]) == 0 {pc=0x10f831a0;continue;}pc=0x10f83188;continue;
}
0x10f83188=>{
// 0x10f83188 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8318c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f83190 ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f83194 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f83198;super::dispatch(cpu,rt,target)?;}
// 0x10f83198 ldr w8, [x0, #0x2c]
{let address=(cpu.x[0]).wrapping_add(0x2cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8319c cbz w8, #0x10f83228
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f83228;continue;}pc=0x10f831a0;continue;
}
0x10f831a0=>{
// 0x10f831a0 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f831a4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f831a8 ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f831ac blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f831b0;super::dispatch(cpu,rt,target)?;}
pc=0x10f831b0;continue;
}
0x10f831b0=>{
// 0x10f831b0 ldr w8, [x0, #0x2c]
{let address=(cpu.x[0]).wrapping_add(0x2cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f831b4 cbz w8, #0x10f831fc
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f831fc;continue;}pc=0x10f831b8;continue;
}
0x10f831b8=>{
// 0x10f831b8 ldr x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f831bc bl #0x10e0b6b4
cpu.x[30]=0x10f831c0;super::chunk8::f_10e0b6b4(cpu,rt,0x10e0b6b4)?;
// 0x10f831c0 ldr x0, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f831c4 bl #0x10e16208
cpu.x[30]=0x10f831c8;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
// 0x10f831c8 ldrb w8, [x0, #0x400]
{let address=(cpu.x[0]).wrapping_add(0x400u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f831cc cbz w8, #0x10f831fc
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f831fc;continue;}pc=0x10f831d0;continue;
}
0x10f831d0=>{
// 0x10f831d0 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f831d4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f831d8 ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f831dc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f831e0;super::dispatch(cpu,rt,target)?;}
// 0x10f831e0 mov w20, #1
cpu.x[20]=(((0x1u64))) as u32 as u64;
// 0x10f831e4 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f831e8 str w20, [x0, #0x2c]
{let address=(cpu.x[0]).wrapping_add(0x2cu64);let v0=cpu.x[20] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f831ec mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f831f0 ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f831f4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f831f8;super::dispatch(cpu,rt,target)?;}
// 0x10f831f8 b #0x10f83274
pc=0x10f83274;continue;
}
0x10f831fc=>{
// 0x10f831fc ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f83200 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f83204 ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f83208 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8320c;super::dispatch(cpu,rt,target)?;}
pc=0x10f8320c;continue;
}
0x10f8320c=>{
// 0x10f8320c ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f83210 str wzr, [x0, #0x2c]
{let address=(cpu.x[0]).wrapping_add(0x2cu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f83214 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f83218 ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8321c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f83220;super::dispatch(cpu,rt,target)?;}
pc=0x10f83220;continue;
}
0x10f83220=>{
// 0x10f83220 mov w20, wzr
cpu.x[20]=(((0u64))) as u32 as u64;
// 0x10f83224 b #0x10f83274
pc=0x10f83274;continue;
}
0x10f83228=>{
// 0x10f83228 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8322c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f83230 ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f83234 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f83238;super::dispatch(cpu,rt,target)?;}
// 0x10f83238 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8323c str wzr, [x0, #0x2c]
{let address=(cpu.x[0]).wrapping_add(0x2cu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f83240 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f83244 ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f83248 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8324c;super::dispatch(cpu,rt,target)?;}
// 0x10f8324c ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f83250 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10f83254 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f83258 ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8325c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f83260;super::dispatch(cpu,rt,target)?;}
// 0x10f83260 mov x8, x0
cpu.x[8]=(((cpu.x[0]))) as u64;
// 0x10f83264 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f83268 ldr w8, [x8, #0x30]
{let address=(cpu.x[8]).wrapping_add(0x30u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8326c cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f83270 cset w20, gt
cpu.x[20]=((cpu.condition(12)) as u64) as u32 as u64;
pc=0x10f83274;continue;
}
0x10f83274=>{
// 0x10f83274 str w20, [x0, #0x30]
{let address=(cpu.x[0]).wrapping_add(0x30u64);let v0=cpu.x[20] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10f83278;continue;
}
0x10f83278=>{
// 0x10f83278 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f8327c ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f83280 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
pc=0x10f83284;continue;
}
0x10f83284=>{
// 0x10f83284 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f83114")),}}}
#[inline]
pub(super) unsafe fn f_10f85730(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f85730=>{
// 0x10f85730 sub sp, sp, #0x50
{let a=((cpu.sp));let b=((0x50u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x10f85734 stp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f85738 stp x24, x23, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f8573c stp x22, x21, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f85740 stp x20, x19, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f85744 add x29, sp, #0x10
{let a=((cpu.sp));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x10f85748 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f8574c ldr x21, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f85750 ldr x0, [x0, #0x48]
{let address=(cpu.x[0]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f85754 mov x22, x2
cpu.x[22]=(((cpu.x[2]))) as u64;
// 0x10f85758 mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x10f8575c bl #0x10de02d4
cpu.x[30]=0x10f85760;super::chunk12::f_10de02d4(cpu,rt,0x10de02d4)?;
pc=0x10f85760;continue;
}
0x10f85760=>{
// 0x10f85760 cbz x22, #0x10f857e4
if (cpu.x[22]) == 0 {pc=0x10f857e4;continue;}pc=0x10f85764;continue;
}
0x10f85764=>{
// 0x10f85764 cbz w0, #0x10f857e4
if (cpu.x[0] as u32 as u64) == 0 {pc=0x10f857e4;continue;}pc=0x10f85768;continue;
}
0x10f85768=>{
// 0x10f85768 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8576c bl #0x10f942c0
cpu.x[30]=0x10f85770;super::chunk2::f_10f942c0(cpu,rt,0x10f942c0)?;
// 0x10f85770 ldr w8, [x22]
{let address=(cpu.x[22]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f85774 ldr w9, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f85778 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8577c sub w23, w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[23]=(result) as u32 as u64;}
// 0x10f85780 bl #0x10f942c0
cpu.x[30]=0x10f85784;super::chunk2::f_10f942c0(cpu,rt,0x10f942c0)?;
// 0x10f85784 ldp w9, w8, [x22]
{let address=(cpu.x[22]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10f85788 ldr w10, [x0, #4]
{let address=(cpu.x[0]).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10f8578c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f85790 orr w9, w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a | b;cpu.x[9]=(result) as u32 as u64;}
// 0x10f85794 sub w8, w10, w8
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10f85798 cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8579c cset w9, eq
cpu.x[9]=((cpu.condition(0)) as u64) as u32 as u64;
// 0x10f857a0 sub w22, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[22]=(result) as u32 as u64;}
// 0x10f857a4 bl #0x10f942c8
cpu.x[30]=0x10f857a8;super::chunk1::f_10f942c8(cpu,rt,0x10f942c8)?;
// 0x10f857a8 ldr x8, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f857ac mov x24, x0
cpu.x[24]=(((cpu.x[0]))) as u64;
// 0x10f857b0 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10f857b4 bl #0x10de02d4
cpu.x[30]=0x10f857b8;super::chunk12::f_10de02d4(cpu,rt,0x10de02d4)?;
// 0x10f857b8 mov w3, w0
cpu.x[3]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f857bc mov x4, xzr
cpu.x[4]=(((0u64))) as u64;
// 0x10f857c0 mov w5, wzr
cpu.x[5]=(((0u64))) as u32 as u64;
// 0x10f857c4 mov w6, wzr
cpu.x[6]=(((0u64))) as u32 as u64;
// 0x10f857c8 mov w7, wzr
cpu.x[7]=(((0u64))) as u32 as u64;
// 0x10f857cc mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10f857d0 mov w1, w23
cpu.x[1]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10f857d4 mov w2, w22
cpu.x[2]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10f857d8 strb wzr, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f857dc strb wzr, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f857e0 bl #0x10fcfac4
cpu.x[30]=0x10f857e4;super::chunk6::f_10fcfac4(cpu,rt,0x10fcfac4)?;
pc=0x10f857e4;continue;
}
0x10f857e4=>{
// 0x10f857e4 ldr x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f857e8 bl #0x10e0b6b4
cpu.x[30]=0x10f857ec;super::chunk8::f_10e0b6b4(cpu,rt,0x10e0b6b4)?;
pc=0x10f857ec;continue;
}
0x10f857ec=>{
// 0x10f857ec ldr x0, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f857f0 bl #0x10e16208
cpu.x[30]=0x10f857f4;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
pc=0x10f857f4;continue;
}
0x10f857f4=>{
// 0x10f857f4 ldrb w8, [x0, #0x437]
{let address=(cpu.x[0]).wrapping_add(0x437u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f857f8 cbz w8, #0x10f85810
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f85810;continue;}pc=0x10f857fc;continue;
}
0x10f857fc=>{
// 0x10f857fc ldr x1, [x20, #0x20]
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
pc=0x10f85800;continue;
}
0x10f85800=>{
// 0x10f85800 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f85804 mov x2, x19
cpu.x[2]=(((cpu.x[19]))) as u64;
// 0x10f85808 mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10f8580c bl #0x10f96a84
cpu.x[30]=0x10f85810;super::chunk0::f_10f96a84(cpu,rt,0x10f96a84)?;
pc=0x10f85810;continue;
}
0x10f85810=>{
// 0x10f85810 ldr x8, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f85814 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f85818 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10f8581c ldr x2, [x8, #0x168]
{let address=(cpu.x[8]).wrapping_add(0x168u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10f85820 bl #0x10f895a0
cpu.x[30]=0x10f85824;super::chunk9::f_10f895a0(cpu,rt,0x10f895a0)?;
pc=0x10f85824;continue;
}
0x10f85824=>{
// 0x10f85824 ldr x0, [x19, #0x58]
{let address=(cpu.x[19]).wrapping_add(0x58u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f85828 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8582c ldr x1, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10f85830 ldp x20, x19, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f85834 ldp x22, x21, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f85838 ldp x24, x23, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10f8583c ldp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10f85840 add sp, sp, #0x50
{let a=((cpu.sp));let b=((0x50u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10f85844 br x1
{let target=cpu.x[1];if target>=0x10f85730 && target<0x10f85848{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f85730")),}}}
#[inline]
pub(super) unsafe fn f_10f85a60(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f85a60=>{
// 0x10f85a60 sub sp, sp, #0x40
{let a=((cpu.sp));let b=((0x40u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x10f85a64 stp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f85a68 stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f85a6c stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f85a70 add x29, sp, #0x10
{let a=((cpu.sp));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x10f85a74 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f85a78 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f85a7c ldr x8, [x8, #0x140]
{let address=(cpu.x[8]).wrapping_add(0x140u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f85a80 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f85a84;super::dispatch(cpu,rt,target)?;}
pc=0x10f85a84;continue;
}
0x10f85a84=>{
// 0x10f85a84 tbz w0, #0, #0x10f85a94
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f85a94;continue;}pc=0x10f85a88;continue;
}
0x10f85a88=>{
// 0x10f85a88 ldr x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f85a8c bl #0x10de1648
cpu.x[30]=0x10f85a90;super::chunk8::f_10de1648(cpu,rt,0x10de1648)?;
// 0x10f85a90 tbnz w0, #0, #0x10f85b80
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10f85b80;continue;}pc=0x10f85a94;continue;
}
0x10f85a94=>{
// 0x10f85a94 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f85a98 bl #0x10f818ac
cpu.x[30]=0x10f85a9c;super::chunk12::f_10f818ac(cpu,rt,0x10f818ac)?;
pc=0x10f85a9c;continue;
}
0x10f85a9c=>{
// 0x10f85a9c tbz w0, #0, #0x10f85ac8
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f85ac8;continue;}pc=0x10f85aa0;continue;
}
0x10f85aa0=>{
// 0x10f85aa0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f85aa4 mov w1, #0xa
cpu.x[1]=(((0xau64))) as u32 as u64;
// 0x10f85aa8 bl #0x10f7f2c8
cpu.x[30]=0x10f85aac;super::chunk8::f_10f7f2c8(cpu,rt,0x10f7f2c8)?;
pc=0x10f85aac;continue;
}
0x10f85aac=>{
// 0x10f85aac adrp x8, #0x11a66000
cpu.x[8]=(0x11a66000u64) as u64;
// 0x10f85ab0 add x8, x8, #0xb0
{let a=((cpu.x[8]));let b=((0xb0u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10f85ab4 ldr x9, [x19, #0xb8]
{let address=(cpu.x[19]).wrapping_add(0xb8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f85ab8 ldr x8, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f85abc orr x8, x9, x8
{let a=((cpu.x[9]));let b=((cpu.x[8]));let result=a | b;cpu.x[8]=(result) as u64;}
// 0x10f85ac0 str x8, [x19, #0xb8]
{let address=(cpu.x[19]).wrapping_add(0xb8u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f85ac4 b #0x10f85b80
pc=0x10f85b80;continue;
}
0x10f85ac8=>{
// 0x10f85ac8 ldr x8, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f85acc ldr x8, [x8, #0x710]
{let address=(cpu.x[8]).wrapping_add(0x710u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f85ad0 cbz x8, #0x10f85b80
if (cpu.x[8]) == 0 {pc=0x10f85b80;continue;}pc=0x10f85ad4;continue;
}
0x10f85ad4=>{
// 0x10f85ad4 adrp x8, #0x11a66000
cpu.x[8]=(0x11a66000u64) as u64;
// 0x10f85ad8 add x8, x8, #0xd8
{let a=((cpu.x[8]));let b=((0xd8u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10f85adc ldr x9, [x19, #0xb8]
{let address=(cpu.x[19]).wrapping_add(0xb8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f85ae0 ldr x0, [x19, #0x58]
{let address=(cpu.x[19]).wrapping_add(0x58u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f85ae4 ldr x8, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f85ae8 orr x8, x9, x8
{let a=((cpu.x[9]));let b=((cpu.x[8]));let result=a | b;cpu.x[8]=(result) as u64;}
// 0x10f85aec str x8, [x19, #0xb8]
{let address=(cpu.x[19]).wrapping_add(0xb8u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f85af0 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10f85af4 ldr x9, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f85af8 strb w8, [x19, #0x1e0]
{let address=(cpu.x[19]).wrapping_add(0x1e0u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f85afc ldr x8, [x9, #0x1e0]
{let address=(cpu.x[9]).wrapping_add(0x1e0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f85b00 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f85b04;super::dispatch(cpu,rt,target)?;}
// 0x10f85b04 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f85b08 bl #0x10f940fc
cpu.x[30]=0x10f85b0c;super::chunk9::f_10f940fc(cpu,rt,0x10f940fc)?;
// 0x10f85b0c cbz x0, #0x10f85b80
if (cpu.x[0]) == 0 {pc=0x10f85b80;continue;}pc=0x10f85b10;continue;
}
0x10f85b10=>{
// 0x10f85b10 ldr x8, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f85b14 ldr x8, [x8, #0x710]
{let address=(cpu.x[8]).wrapping_add(0x710u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f85b18 ldr x8, [x8, #0xc0]
{let address=(cpu.x[8]).wrapping_add(0xc0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f85b1c cbz x8, #0x10f85b80
if (cpu.x[8]) == 0 {pc=0x10f85b80;continue;}pc=0x10f85b20;continue;
}
0x10f85b20=>{
// 0x10f85b20 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f85b24 bl #0x10f940fc
cpu.x[30]=0x10f85b28;super::chunk9::f_10f940fc(cpu,rt,0x10f940fc)?;
// 0x10f85b28 ldr x8, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f85b2c ldr w21, [x19, #0x120]
{let address=(cpu.x[19]).wrapping_add(0x120u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[21]=(v0) as u32 as u64;}
// 0x10f85b30 mov x22, x0
cpu.x[22]=(((cpu.x[0]))) as u64;
// 0x10f85b34 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f85b38 ldr x8, [x8, #0x710]
{let address=(cpu.x[8]).wrapping_add(0x710u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f85b3c ldr x20, [x8, #0xc0]
{let address=(cpu.x[8]).wrapping_add(0xc0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10f85b40 bl #0x10f93d3c
cpu.x[30]=0x10f85b44;super::chunk3::f_10f93d3c(cpu,rt,0x10f93d3c)?;
// 0x10f85b44 mov w6, w0
cpu.x[6]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f85b48 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f85b4c mov x1, xzr
cpu.x[1]=(((0u64))) as u64;
// 0x10f85b50 mov x2, x20
cpu.x[2]=(((cpu.x[20]))) as u64;
// 0x10f85b54 mov w3, #0x7fffffff
cpu.x[3]=(((0x7fffffffu64))) as u32 as u64;
// 0x10f85b58 mov x4, x21
cpu.x[4]=(((cpu.x[21]))) as u64;
// 0x10f85b5c mov x5, x19
cpu.x[5]=(((cpu.x[19]))) as u64;
// 0x10f85b60 mov w7, wzr
cpu.x[7]=(((0u64))) as u32 as u64;
// 0x10f85b64 str xzr, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f85b68 bl #0x10fc0e08
cpu.x[30]=0x10f85b6c;super::chunk14::f_10fc0e08(cpu,rt,0x10fc0e08)?;
// 0x10f85b6c ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f85b70 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f85b74 ldp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10f85b78 add sp, sp, #0x40
{let a=((cpu.sp));let b=((0x40u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10f85b7c ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f85b80=>{
// 0x10f85b80 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f85b84 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f85b88 ldp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10f85b8c add sp, sp, #0x40
{let a=((cpu.sp));let b=((0x40u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10f85b90 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f85a60")),}}}
#[inline]
pub(super) unsafe fn f_10f86c6c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f86c6c=>{
// 0x10f86c6c stp w1, w1, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=cpu.x[1] as u32 as u64;let v1=cpu.x[1] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10f86c70 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f86c6c")),}}}
#[inline]
pub(super) unsafe fn f_10f86e9c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f86e9c=>{
// 0x10f86e9c stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f86ea0 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f86ea4 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f86ea8 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f86eac mov x21, x1
cpu.x[21]=(((cpu.x[1]))) as u64;
// 0x10f86eb0 ldr x1, [x1, #0x148]
{let address=(cpu.x[1]).wrapping_add(0x148u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10f86eb4 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f86eb8 str x21, [x0, #0x18]
{let address=(cpu.x[0]).wrapping_add(0x18u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f86ebc and x0, x2, #0xffffffff
{let a=((cpu.x[2]));let b=((0xffffffffu64));let result=a & b;cpu.x[0]=(result) as u64;}
// 0x10f86ec0 mov x20, x3
cpu.x[20]=(((cpu.x[3]))) as u64;
// 0x10f86ec4 bl #0x10fa5c64
cpu.x[30]=0x10f86ec8;super::chunk6::f_10fa5c64(cpu,rt,0x10fa5c64)?;
pc=0x10f86ec8;continue;
}
0x10f86ec8=>{
// 0x10f86ec8 str w0, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f86ecc ldr x8, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f86ed0 ldrb w9, [x21, #0x129]
{let address=(cpu.x[21]).wrapping_add(0x129u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f86ed4 cbnz w9, #0x10f86ee0
if (cpu.x[9] as u32 as u64) != 0 {pc=0x10f86ee0;continue;}pc=0x10f86ed8;continue;
}
0x10f86ed8=>{
// 0x10f86ed8 ldrb w9, [x8, #0x12b]
{let address=(cpu.x[8]).wrapping_add(0x12bu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f86edc cbz w9, #0x10f86ee4
if (cpu.x[9] as u32 as u64) == 0 {pc=0x10f86ee4;continue;}pc=0x10f86ee0;continue;
}
0x10f86ee0=>{
// 0x10f86ee0 str x20, [x19, #0x30]
{let address=(cpu.x[19]).wrapping_add(0x30u64);let v0=cpu.x[20];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10f86ee4;continue;
}
0x10f86ee4=>{
// 0x10f86ee4 ldr w9, [x8, #0x188]
{let address=(cpu.x[8]).wrapping_add(0x188u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f86ee8 ldr w10, [x8, #0x190]
{let address=(cpu.x[8]).wrapping_add(0x190u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10f86eec cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f86ef0 str w10, [x19, #0x58]
{let address=(cpu.x[19]).wrapping_add(0x58u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f86ef4 b.lt #0x10f86efc
if cpu.condition(11){pc=0x10f86efc;continue;}pc=0x10f86ef8;continue;
}
0x10f86ef8=>{
// 0x10f86ef8 str w9, [x19, #0x50]
{let address=(cpu.x[19]).wrapping_add(0x50u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10f86efc;continue;
}
0x10f86efc=>{
// 0x10f86efc ldr w8, [x8, #0x140]
{let address=(cpu.x[8]).wrapping_add(0x140u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f86f00 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f86f04 b.lt #0x10f86f0c
if cpu.condition(11){pc=0x10f86f0c;continue;}pc=0x10f86f08;continue;
}
0x10f86f08=>{
// 0x10f86f08 str w8, [x19, #0x5c]
{let address=(cpu.x[19]).wrapping_add(0x5cu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10f86f0c;continue;
}
0x10f86f0c=>{
// 0x10f86f0c ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f86f10 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f86f14 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f86f18 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f86e9c")),}}}
#[inline]
pub(super) unsafe fn f_10f87920(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f87920=>{
// 0x10f87920 sub sp, sp, #0x70
{let a=((cpu.sp));let b=((0x70u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x10f87924 stp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f87928 stp x28, x27, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[28];let v1=cpu.x[27];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f8792c stp x26, x25, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[26];let v1=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f87930 stp x24, x23, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f87934 stp x22, x21, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f87938 stp x20, x19, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f8793c add x29, sp, #0x10
{let a=((cpu.sp));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x10f87940 ldr w8, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f87944 ldp x21, x11, [x29, #0x60]
{let address=(cpu.x[29]).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[21]=(v0) as u64;cpu.x[11]=(v1) as u64;}
// 0x10f87948 ldr w23, [x29, #0x78]
{let address=(cpu.x[29]).wrapping_add(0x78u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[23]=(v0) as u32 as u64;}
// 0x10f8794c mov w27, w7
cpu.x[27]=(((cpu.x[7] as u32 as u64))) as u32 as u64;
// 0x10f87950 mov w28, w5
cpu.x[28]=(((cpu.x[5] as u32 as u64))) as u32 as u64;
// 0x10f87954 ldr w20, [x29, #0x70]
{let address=(cpu.x[29]).wrapping_add(0x70u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[20]=(v0) as u32 as u64;}
// 0x10f87958 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f8795c mov w24, w4
cpu.x[24]=(((cpu.x[4] as u32 as u64))) as u32 as u64;
// 0x10f87960 mov w25, w3
cpu.x[25]=(((cpu.x[3] as u32 as u64))) as u32 as u64;
// 0x10f87964 mov w22, w2
cpu.x[22]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x10f87968 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8796c mov x26, x1
cpu.x[26]=(((cpu.x[1]))) as u64;
// 0x10f87970 b.eq #0x10f87994
if cpu.condition(0){pc=0x10f87994;continue;}pc=0x10f87974;continue;
}
0x10f87974=>{
// 0x10f87974 adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x10f87978 add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10f8797c mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10f87980 stur w6, [x29, #-4]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffffcu64);let v0=cpu.x[6] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f87984 str x11, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=cpu.x[11];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f87988 bl #0x1128df18
cpu.x[30]=0x10f8798c;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10f8798c ldr x11, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[11]=(v0) as u64;}
// 0x10f87990 ldur w6, [x29, #-4]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffffcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[6]=(v0) as u32 as u64;}
pc=0x10f87994;continue;
}
0x10f87994=>{
// 0x10f87994 stp w22, w25, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=cpu.x[22] as u32 as u64;let v1=cpu.x[25] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10f87998 and w8, w6, #1
{let a=((cpu.x[6] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[8]=(result) as u32 as u64;}
// 0x10f8799c stp w24, w28, [x19, #0x30]
{let address=(cpu.x[19]).wrapping_add(0x30u64);let v0=cpu.x[24] as u32 as u64;let v1=cpu.x[28] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10f879a0 stp w27, w21, [x19, #0x3c]
{let address=(cpu.x[19]).wrapping_add(0x3cu64);let v0=cpu.x[27] as u32 as u64;let v1=cpu.x[21] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10f879a4 ldr d0, [x11]
{let address=(cpu.x[11]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.q[0]=(v0) as u128 & 0xffffffffffffffffu128;}
// 0x10f879a8 strb w8, [x19, #0x38]
{let address=(cpu.x[19]).wrapping_add(0x38u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f879ac mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10f879b0 str x26, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=cpu.x[26];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f879b4 strb w8, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f879b8 stur d0, [x19, #0x44]
{let address=(cpu.x[19]).wrapping_add(0x44u64);let v0=(cpu.q[0] & 0xffffffffffffffffu128) as u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f879bc stp w20, w23, [x19, #0x4c]
{let address=(cpu.x[19]).wrapping_add(0x4cu64);let v0=cpu.x[20] as u32 as u64;let v1=cpu.x[23] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10f879c0 ldp x20, x19, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f879c4 ldp x22, x21, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f879c8 ldp x24, x23, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10f879cc ldp x26, x25, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10f879d0 ldp x28, x27, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10f879d4 ldp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10f879d8 add sp, sp, #0x70
{let a=((cpu.sp));let b=((0x70u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10f879dc ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f87920")),}}}
#[inline]
pub(super) unsafe fn f_10f892bc(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f892bc=>{
// 0x10f892bc stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f892c0 str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f892c4 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f892c8 ldr w8, [x0, #0x24]
{let address=(cpu.x[0]).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f892cc cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f892d0 b.le #0x10f89310
if cpu.condition(13){pc=0x10f89310;continue;}pc=0x10f892d4;continue;
}
0x10f892d4=>{
// 0x10f892d4 ldr x8, [x0, #0x18]
{let address=(cpu.x[0]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f892d8 sbfiz x9, x1, #3, #0x20
cpu.x[9]=((((((cpu.x[1])) << 32) as i64 >> 32) as u64).wrapping_shl(3)) as u64;
// 0x10f892dc ldr x19, [x8, x9]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[9])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10f892e0 str xzr, [x8, x9]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[9])));let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f892e4 cbz x19, #0x10f89310
if (cpu.x[19]) == 0 {pc=0x10f89310;continue;}pc=0x10f892e8;continue;
}
0x10f892e8=>{
// 0x10f892e8 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f892ec mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f892f0 ldr x8, [x8, #0x50]
{let address=(cpu.x[8]).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f892f4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f892f8;super::dispatch(cpu,rt,target)?;}
pc=0x10f892f8;continue;
}
0x10f892f8=>{
// 0x10f892f8 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f892fc ldr x1, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10f89300 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f89304 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10f89308 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f8930c br x1
{let target=cpu.x[1];if target>=0x10f892bc && target<0x10f8931c{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
0x10f89310=>{
// 0x10f89310 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10f89314 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f89318 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f892bc")),}}}
#[inline]
pub(super) unsafe fn f_10f89cf8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f89cf8=>{
// 0x10f89cf8 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f89cfc str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f89d00 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f89d04 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f89d08 ldr x8, [x1]
{let address=(cpu.x[1]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f89d0c mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f89d10 mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x10f89d14 mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x10f89d18 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f89d1c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f89d20;super::dispatch(cpu,rt,target)?;}
pc=0x10f89d20;continue;
}
0x10f89d20=>{
// 0x10f89d20 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f89d24 str w0, [x19, #0x30]
{let address=(cpu.x[19]).wrapping_add(0x30u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f89d28 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f89d2c ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f89d30 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f89d34;super::dispatch(cpu,rt,target)?;}
pc=0x10f89d34;continue;
}
0x10f89d34=>{
// 0x10f89d34 mov w8, w0
cpu.x[8]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f89d38 ldr x0, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f89d3c tbz w8, #0, #0x10f89d58
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f89d58;continue;}pc=0x10f89d40;continue;
}
0x10f89d40=>{
// 0x10f89d40 cbz x0, #0x10f89d88
if (cpu.x[0]) == 0 {pc=0x10f89d88;continue;}pc=0x10f89d44;continue;
}
0x10f89d44=>{
// 0x10f89d44 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10f89d48 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f89d4c ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f89d50 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f89d54 b #0x10f785d4
super::chunk11::f_10f785d4(cpu,rt,0x10f785d4)?;return Ok(());
}
0x10f89d58=>{
// 0x10f89d58 cbz x0, #0x10f89d78
if (cpu.x[0]) == 0 {pc=0x10f89d78;continue;}pc=0x10f89d5c;continue;
}
0x10f89d5c=>{
// 0x10f89d5c bl #0x10f798dc
cpu.x[30]=0x10f89d60;super::chunk6::f_10f798dc(cpu,rt,0x10f798dc)?;
// 0x10f89d60 ldr x0, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f89d64 cbz x0, #0x10f89d74
if (cpu.x[0]) == 0 {pc=0x10f89d74;continue;}pc=0x10f89d68;continue;
}
0x10f89d68=>{
// 0x10f89d68 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f89d6c ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f89d70 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f89d74;super::dispatch(cpu,rt,target)?;}
pc=0x10f89d74;continue;
}
0x10f89d74=>{
// 0x10f89d74 str xzr, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10f89d78;continue;
}
0x10f89d78=>{
// 0x10f89d78 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f89d7c ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f89d80 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f89d84 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f89d88=>{
// 0x10f89d88 mov w0, #0x68
cpu.x[0]=(((0x68u64))) as u32 as u64;
// 0x10f89d8c bl #0x1187d2b0
cpu.x[30]=0x10f89d90;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
pc=0x10f89d90;continue;
}
0x10f89d90=>{
// 0x10f89d90 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f89d94 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10f89d98 bl #0x10f77db8
cpu.x[30]=0x10f89d9c;super::chunk10::f_10f77db8(cpu,rt,0x10f77db8)?;
pc=0x10f89d9c;continue;
}
0x10f89d9c=>{
// 0x10f89d9c mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f89da0 str x21, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f89da4 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10f89da8 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f89dac ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f89db0 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f89db4 b #0x10f785d4
super::chunk11::f_10f785d4(cpu,rt,0x10f785d4)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f89cf8")),}}}
#[inline]
pub(super) unsafe fn f_10f8c3b8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f8c3b8=>{
// 0x10f8c3b8 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f8c3bc str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f8c3c0 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f8c3c4 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f8c3c8 ldr w8, [x0, #0x54]
{let address=(cpu.x[0]).wrapping_add(0x54u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8c3cc ldr w9, [x0, #0x34]
{let address=(cpu.x[0]).wrapping_add(0x34u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f8c3d0 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f8c3d4 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8c3d8 b.eq #0x10f8c3f0
if cpu.condition(0){pc=0x10f8c3f0;continue;}pc=0x10f8c3dc;continue;
}
0x10f8c3dc=>{
// 0x10f8c3dc adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x10f8c3e0 add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10f8c3e4 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10f8c3e8 bl #0x1128df18
cpu.x[30]=0x10f8c3ec;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10f8c3ec ldr w8, [x19, #0x34]
{let address=(cpu.x[19]).wrapping_add(0x34u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
pc=0x10f8c3f0;continue;
}
0x10f8c3f0=>{
// 0x10f8c3f0 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8c3f4 b.lt #0x10f8c4f4
if cpu.condition(11){pc=0x10f8c4f4;continue;}pc=0x10f8c3f8;continue;
}
0x10f8c3f8=>{
// 0x10f8c3f8 mov x21, xzr
cpu.x[21]=(((0u64))) as u64;
// 0x10f8c3fc b #0x10f8c42c
pc=0x10f8c42c;continue;
}
0x10f8c400=>{
// 0x10f8c400 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8c404 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8c408 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10f8c40c mov w2, wzr
cpu.x[2]=(((0u64))) as u32 as u64;
// 0x10f8c410 mov w3, #1
cpu.x[3]=(((0x1u64))) as u32 as u64;
// 0x10f8c414 ldr x8, [x8, #0x48]
{let address=(cpu.x[8]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8c418 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8c41c;super::dispatch(cpu,rt,target)?;}
pc=0x10f8c41c;continue;
}
0x10f8c41c=>{
// 0x10f8c41c ldrsw x8, [x19, #0x34]
{let address=(cpu.x[19]).wrapping_add(0x34u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10f8c420 add x21, x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[21]=(result) as u64;}
// 0x10f8c424 cmp x21, x8
{let a=((cpu.x[21]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f8c428 b.ge #0x10f8c4f4
if cpu.condition(10){pc=0x10f8c4f4;continue;}pc=0x10f8c42c;continue;
}
0x10f8c42c=>{
// 0x10f8c42c ldr x8, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8c430 ldr x9, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f8c434 ldr w8, [x8, x21, lsl #2]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[21])) as u64).wrapping_shl(2) as u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8c438 ldr x20, [x9, x21, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[21])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10f8c43c cbz w8, #0x10f8c400
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f8c400;continue;}pc=0x10f8c440;continue;
}
0x10f8c440=>{
// 0x10f8c440 ldr w11, [x19, #0x14]
{let address=(cpu.x[19]).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
// 0x10f8c444 cbz w11, #0x10f8c4cc
if (cpu.x[11] as u32 as u64) == 0 {pc=0x10f8c4cc;continue;}pc=0x10f8c448;continue;
}
0x10f8c448=>{
// 0x10f8c448 ldr x9, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f8c44c cmp w11, #2
{let a=((cpu.x[11] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8c450 b.lt #0x10f8c490
if cpu.condition(11){pc=0x10f8c490;continue;}pc=0x10f8c454;continue;
}
0x10f8c454=>{
// 0x10f8c454 mov w10, wzr
cpu.x[10]=(((0u64))) as u32 as u64;
// 0x10f8c458 sub w11, w11, #1
{let a=((cpu.x[11] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[11]=(result) as u32 as u64;}
// 0x10f8c45c b #0x10f8c46c
pc=0x10f8c46c;continue;
}
0x10f8c460=>{
// 0x10f8c460 cmp w12, w10
{let a=((cpu.x[12] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8c464 mov w11, w12
cpu.x[11]=(((cpu.x[12] as u32 as u64))) as u32 as u64;
// 0x10f8c468 b.le #0x10f8c4a4
if cpu.condition(13){pc=0x10f8c4a4;continue;}pc=0x10f8c46c;continue;
}
0x10f8c46c=>{
// 0x10f8c46c add w12, w10, w11
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[11] as u32 as u64));let result=a.wrapping_add(b);cpu.x[12]=(result) as u32 as u64;}
// 0x10f8c470 asr w12, w12, #1
cpu.x[12]=(((((cpu.x[12] as u32 as u64))) as i32).wrapping_shr(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10f8c474 ldr x13, [x9, w12, sxtw #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[12] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[13]=(v0) as u64;}
// 0x10f8c478 ldr w13, [x13, #8]
{let address=(cpu.x[13]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[13]=(v0) as u32 as u64;}
// 0x10f8c47c cmp w13, w8
{let a=((cpu.x[13] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8c480 b.ge #0x10f8c460
if cpu.condition(10){pc=0x10f8c460;continue;}pc=0x10f8c484;continue;
}
0x10f8c484=>{
// 0x10f8c484 add w10, w12, #1
{let a=((cpu.x[12] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10f8c488 mov w12, w11
cpu.x[12]=(((cpu.x[11] as u32 as u64))) as u32 as u64;
// 0x10f8c48c b #0x10f8c460
pc=0x10f8c460;continue;
}
0x10f8c490=>{
// 0x10f8c490 ldr x0, [x9]
{let address=(cpu.x[9]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f8c494 ldr w9, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f8c498 cmp w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8c49c b.eq #0x10f8c4bc
if cpu.condition(0){pc=0x10f8c4bc;continue;}pc=0x10f8c4a0;continue;
}
0x10f8c4a0=>{
// 0x10f8c4a0 b #0x10f8c4cc
pc=0x10f8c4cc;continue;
}
0x10f8c4a4=>{
// 0x10f8c4a4 cmn w10, #1
{let a=((cpu.x[10] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.flags(a,b,result,32,false);}
// 0x10f8c4a8 b.eq #0x10f8c4cc
if cpu.condition(0){pc=0x10f8c4cc;continue;}pc=0x10f8c4ac;continue;
}
0x10f8c4ac=>{
// 0x10f8c4ac ldr x0, [x9, w10, sxtw #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[10] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f8c4b0 ldr w9, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f8c4b4 cmp w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8c4b8 b.ne #0x10f8c4cc
if cpu.condition(1){pc=0x10f8c4cc;continue;}pc=0x10f8c4bc;continue;
}
0x10f8c4bc=>{
// 0x10f8c4bc ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8c4c0 ldr x8, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8c4c4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8c4c8;super::dispatch(cpu,rt,target)?;}
// 0x10f8c4c8 tbz w0, #0, #0x10f8c400
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f8c400;continue;}pc=0x10f8c4cc;continue;
}
0x10f8c4cc=>{
// 0x10f8c4cc cbz x20, #0x10f8c41c
if (cpu.x[20]) == 0 {pc=0x10f8c41c;continue;}pc=0x10f8c4d0;continue;
}
0x10f8c4d0=>{
// 0x10f8c4d0 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8c4d4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f8c4d8 ldr x8, [x8, #0x68]
{let address=(cpu.x[8]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8c4dc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8c4e0;super::dispatch(cpu,rt,target)?;}
// 0x10f8c4e0 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8c4e4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f8c4e8 ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8c4ec blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8c4f0;super::dispatch(cpu,rt,target)?;}
// 0x10f8c4f0 b #0x10f8c41c
pc=0x10f8c41c;continue;
}
0x10f8c4f4=>{
// 0x10f8c4f4 str wzr, [x19, #0x34]
{let address=(cpu.x[19]).wrapping_add(0x34u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8c4f8 str wzr, [x19, #0x54]
{let address=(cpu.x[19]).wrapping_add(0x54u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8c4fc ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f8c500 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f8c504 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f8c508 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f8c3b8")),}}}
#[inline]
pub(super) unsafe fn f_10f8e028(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f8e028=>{
// 0x10f8e028 stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f8e02c str x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f8e030 stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f8e034 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f8e038 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f8e03c mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10f8e040 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10f8e044 bl #0x10f93d44
cpu.x[30]=0x10f8e048;super::chunk8::f_10f93d44(cpu,rt,0x10f93d44)?;
pc=0x10f8e048;continue;
}
0x10f8e048=>{
// 0x10f8e048 ldr x21, [x20, #0x10]
{let address=(cpu.x[20]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f8e04c add x1, x20, #0x120
{let a=((cpu.x[20]));let b=((0x120u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x10f8e050 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8e054 bl #0x111dd2e0
cpu.x[30]=0x10f8e058;super::chunk9::f_111dd2e0(cpu,rt,0x111dd2e0)?;
pc=0x10f8e058;continue;
}
0x10f8e058=>{
// 0x10f8e058 add x1, x20, #0x12c
{let a=((cpu.x[20]));let b=((0x12cu64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x10f8e05c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8e060 bl #0x111dd2e0
cpu.x[30]=0x10f8e064;super::chunk9::f_111dd2e0(cpu,rt,0x111dd2e0)?;
pc=0x10f8e064;continue;
}
0x10f8e064=>{
// 0x10f8e064 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f8e068 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f8e06c bl #0x10f992f0
cpu.x[30]=0x10f8e070;super::chunk11::f_10f992f0(cpu,rt,0x10f992f0)?;
pc=0x10f8e070;continue;
}
0x10f8e070=>{
// 0x10f8e070 str x0, [x20, #0x108]
{let address=(cpu.x[20]).wrapping_add(0x108u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f8e074 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f8e078 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f8e07c bl #0x10f992f0
cpu.x[30]=0x10f8e080;super::chunk11::f_10f992f0(cpu,rt,0x10f992f0)?;
pc=0x10f8e080;continue;
}
0x10f8e080=>{
// 0x10f8e080 ldr x8, [x20, #0x48]
{let address=(cpu.x[20]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e084 str x0, [x20, #0x100]
{let address=(cpu.x[20]).wrapping_add(0x100u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f8e088 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10f8e08c bl #0x10e0b6b4
cpu.x[30]=0x10f8e090;super::chunk8::f_10e0b6b4(cpu,rt,0x10e0b6b4)?;
pc=0x10f8e090;continue;
}
0x10f8e090=>{
// 0x10f8e090 ldr x2, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10f8e094 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8e098 mov w1, #0xb
cpu.x[1]=(((0xbu64))) as u32 as u64;
// 0x10f8e09c bl #0x111dd80c
cpu.x[30]=0x10f8e0a0;super::chunk6::f_111dd80c(cpu,rt,0x111dd80c)?;
pc=0x10f8e0a0;continue;
}
0x10f8e0a0=>{
// 0x10f8e0a0 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e0a4 str x0, [x20, #0x190]
{let address=(cpu.x[20]).wrapping_add(0x190u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f8e0a8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8e0ac ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e0b0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8e0b4;super::dispatch(cpu,rt,target)?;}
pc=0x10f8e0b4;continue;
}
0x10f8e0b4=>{
// 0x10f8e0b4 str w0, [x20, #0x198]
{let address=(cpu.x[20]).wrapping_add(0x198u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8e0b8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8e0bc ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e0c0 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e0c4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8e0c8;super::dispatch(cpu,rt,target)?;}
pc=0x10f8e0c8;continue;
}
0x10f8e0c8=>{
// 0x10f8e0c8 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e0cc str w0, [x20, #0x128]
{let address=(cpu.x[20]).wrapping_add(0x128u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8e0d0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8e0d4 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e0d8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8e0dc;super::dispatch(cpu,rt,target)?;}
pc=0x10f8e0dc;continue;
}
0x10f8e0dc=>{
// 0x10f8e0dc ldr x8, [x20, #0x48]
{let address=(cpu.x[20]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e0e0 str w0, [x20, #0x134]
{let address=(cpu.x[20]).wrapping_add(0x134u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8e0e4 ldr w8, [x8, #0x1c0]
{let address=(cpu.x[8]).wrapping_add(0x1c0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8e0e8 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8e0ec b.lt #0x10f8e12c
if cpu.condition(11){pc=0x10f8e12c;continue;}pc=0x10f8e0f0;continue;
}
0x10f8e0f0=>{
// 0x10f8e0f0 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e0f4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8e0f8 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e0fc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8e100;super::dispatch(cpu,rt,target)?;}
// 0x10f8e100 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e104 str w0, [x20, #0x138]
{let address=(cpu.x[20]).wrapping_add(0x138u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8e108 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8e10c ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e110 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8e114;super::dispatch(cpu,rt,target)?;}
// 0x10f8e114 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e118 str w0, [x20, #0x13c]
{let address=(cpu.x[20]).wrapping_add(0x13cu64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8e11c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8e120 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e124 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8e128;super::dispatch(cpu,rt,target)?;}
// 0x10f8e128 str w0, [x20, #0x140]
{let address=(cpu.x[20]).wrapping_add(0x140u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10f8e12c;continue;
}
0x10f8e12c=>{
// 0x10f8e12c ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e130 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8e134 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e138 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8e13c;super::dispatch(cpu,rt,target)?;}
pc=0x10f8e13c;continue;
}
0x10f8e13c=>{
// 0x10f8e13c cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8e140 str w0, [x20, #0x148]
{let address=(cpu.x[20]).wrapping_add(0x148u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8e144 b.lt #0x10f8e164
if cpu.condition(11){pc=0x10f8e164;continue;}pc=0x10f8e148;continue;
}
0x10f8e148=>{
// 0x10f8e148 ldr x0, [x20, #0x48]
{let address=(cpu.x[20]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f8e14c bl #0x10e0b6b4
cpu.x[30]=0x10f8e150;super::chunk8::f_10e0b6b4(cpu,rt,0x10e0b6b4)?;
// 0x10f8e150 ldr x2, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10f8e154 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8e158 mov w1, #0xb
cpu.x[1]=(((0xbu64))) as u32 as u64;
// 0x10f8e15c bl #0x111dd80c
cpu.x[30]=0x10f8e160;super::chunk6::f_111dd80c(cpu,rt,0x111dd80c)?;
// 0x10f8e160 str x0, [x20, #0x1a8]
{let address=(cpu.x[20]).wrapping_add(0x1a8u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10f8e164;continue;
}
0x10f8e164=>{
// 0x10f8e164 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e168 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8e16c ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e170 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8e174;super::dispatch(cpu,rt,target)?;}
pc=0x10f8e174;continue;
}
0x10f8e174=>{
// 0x10f8e174 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e178 and w9, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[9]=(result) as u32 as u64;}
// 0x10f8e17c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8e180 strb w9, [x20, #0x1dd]
{let address=(cpu.x[20]).wrapping_add(0x1ddu64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f8e184 ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e188 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8e18c;super::dispatch(cpu,rt,target)?;}
pc=0x10f8e18c;continue;
}
0x10f8e18c=>{
// 0x10f8e18c ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e190 and w9, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[9]=(result) as u32 as u64;}
// 0x10f8e194 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8e198 strb w9, [x20, #0x188]
{let address=(cpu.x[20]).wrapping_add(0x188u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f8e19c ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e1a0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8e1a4;super::dispatch(cpu,rt,target)?;}
pc=0x10f8e1a4;continue;
}
0x10f8e1a4=>{
// 0x10f8e1a4 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e1a8 and w9, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[9]=(result) as u32 as u64;}
// 0x10f8e1ac mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8e1b0 strb w9, [x20, #0x189]
{let address=(cpu.x[20]).wrapping_add(0x189u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f8e1b4 ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e1b8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8e1bc;super::dispatch(cpu,rt,target)?;}
pc=0x10f8e1bc;continue;
}
0x10f8e1bc=>{
// 0x10f8e1bc ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e1c0 and w9, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[9]=(result) as u32 as u64;}
// 0x10f8e1c4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8e1c8 strb w9, [x20, #0x170]
{let address=(cpu.x[20]).wrapping_add(0x170u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f8e1cc ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e1d0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8e1d4;super::dispatch(cpu,rt,target)?;}
pc=0x10f8e1d4;continue;
}
0x10f8e1d4=>{
// 0x10f8e1d4 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e1d8 and w9, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[9]=(result) as u32 as u64;}
// 0x10f8e1dc mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8e1e0 strb w9, [x20, #0x1dc]
{let address=(cpu.x[20]).wrapping_add(0x1dcu64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f8e1e4 ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e1e8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8e1ec;super::dispatch(cpu,rt,target)?;}
pc=0x10f8e1ec;continue;
}
0x10f8e1ec=>{
// 0x10f8e1ec ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e1f0 and w9, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[9]=(result) as u32 as u64;}
// 0x10f8e1f4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8e1f8 strb w9, [x20, #0x14c]
{let address=(cpu.x[20]).wrapping_add(0x14cu64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f8e1fc ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e200 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8e204;super::dispatch(cpu,rt,target)?;}
pc=0x10f8e204;continue;
}
0x10f8e204=>{
// 0x10f8e204 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e208 mov w22, w0
cpu.x[22]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f8e20c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8e210 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e214 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8e218;super::dispatch(cpu,rt,target)?;}
pc=0x10f8e218;continue;
}
0x10f8e218=>{
// 0x10f8e218 str w0, [x20, #0x144]
{let address=(cpu.x[20]).wrapping_add(0x144u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8e21c mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f8e220 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f8e224 bl #0x10f992f0
cpu.x[30]=0x10f8e228;super::chunk11::f_10f992f0(cpu,rt,0x10f992f0)?;
pc=0x10f8e228;continue;
}
0x10f8e228=>{
// 0x10f8e228 mov x8, x0
cpu.x[8]=(((cpu.x[0]))) as u64;
// 0x10f8e22c ldr x0, [x20, #0x48]
{let address=(cpu.x[20]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f8e230 str x8, [x20, #0x118]
{let address=(cpu.x[20]).wrapping_add(0x118u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f8e234 ldr w9, [x0, #0x1b0]
{let address=(cpu.x[0]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f8e238 cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8e23c b.lt #0x10f8e25c
if cpu.condition(11){pc=0x10f8e25c;continue;}pc=0x10f8e240;continue;
}
0x10f8e240=>{
// 0x10f8e240 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e244 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8e248 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e24c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8e250;super::dispatch(cpu,rt,target)?;}
// 0x10f8e250 mov w8, w0
cpu.x[8]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f8e254 ldr x0, [x20, #0x48]
{let address=(cpu.x[20]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f8e258 str w8, [x20, #0x1b8]
{let address=(cpu.x[20]).wrapping_add(0x1b8u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10f8e25c;continue;
}
0x10f8e25c=>{
// 0x10f8e25c ldr w8, [x0, #0x200]
{let address=(cpu.x[0]).wrapping_add(0x200u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8e260 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8e264 b.gt #0x10f8e280
if cpu.condition(12){pc=0x10f8e280;continue;}pc=0x10f8e268;continue;
}
0x10f8e268=>{
// 0x10f8e268 ldr w8, [x0, #0x1b0]
{let address=(cpu.x[0]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8e26c cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8e270 b.gt #0x10f8e280
if cpu.condition(12){pc=0x10f8e280;continue;}pc=0x10f8e274;continue;
}
0x10f8e274=>{
// 0x10f8e274 ldr w8, [x0, #0x204]
{let address=(cpu.x[0]).wrapping_add(0x204u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8e278 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8e27c b.lt #0x10f8e29c
if cpu.condition(11){pc=0x10f8e29c;continue;}pc=0x10f8e280;continue;
}
0x10f8e280=>{
// 0x10f8e280 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e284 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8e288 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e28c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8e290;super::dispatch(cpu,rt,target)?;}
// 0x10f8e290 mov w8, w0
cpu.x[8]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f8e294 ldr x0, [x20, #0x48]
{let address=(cpu.x[20]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f8e298 str w8, [x20, #0x1b0]
{let address=(cpu.x[20]).wrapping_add(0x1b0u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10f8e29c;continue;
}
0x10f8e29c=>{
// 0x10f8e29c bl #0x10e5ae3c
cpu.x[30]=0x10f8e2a0;super::chunk11::f_10e5ae3c(cpu,rt,0x10e5ae3c)?;
pc=0x10f8e2a0;continue;
}
0x10f8e2a0=>{
// 0x10f8e2a0 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8e2a4 b.lt #0x10f8e2cc
if cpu.condition(11){pc=0x10f8e2cc;continue;}pc=0x10f8e2a8;continue;
}
0x10f8e2a8=>{
// 0x10f8e2a8 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e2ac mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8e2b0 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e2b4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8e2b8;super::dispatch(cpu,rt,target)?;}
// 0x10f8e2b8 str w0, [x20, #0x1bc]
{let address=(cpu.x[20]).wrapping_add(0x1bcu64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8e2bc mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f8e2c0 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f8e2c4 bl #0x10f992f0
cpu.x[30]=0x10f8e2c8;super::chunk11::f_10f992f0(cpu,rt,0x10f992f0)?;
// 0x10f8e2c8 str x0, [x20, #0x110]
{let address=(cpu.x[20]).wrapping_add(0x110u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10f8e2cc;continue;
}
0x10f8e2cc=>{
// 0x10f8e2cc ldr x8, [x20, #0x48]
{let address=(cpu.x[20]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e2d0 ldr w8, [x8, #0x1dc]
{let address=(cpu.x[8]).wrapping_add(0x1dcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8e2d4 cmp w8, #3
{let a=((cpu.x[8] as u32 as u64));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8e2d8 b.ne #0x10f8e2f0
if cpu.condition(1){pc=0x10f8e2f0;continue;}pc=0x10f8e2dc;continue;
}
0x10f8e2dc=>{
// 0x10f8e2dc ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e2e0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8e2e4 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e2e8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8e2ec;super::dispatch(cpu,rt,target)?;}
// 0x10f8e2ec str w0, [x20, #0x1c0]
{let address=(cpu.x[20]).wrapping_add(0x1c0u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10f8e2f0;continue;
}
0x10f8e2f0=>{
// 0x10f8e2f0 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e2f4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8e2f8 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e2fc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8e300;super::dispatch(cpu,rt,target)?;}
pc=0x10f8e300;continue;
}
0x10f8e300=>{
// 0x10f8e300 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e304 str w0, [x20, #0x1b4]
{let address=(cpu.x[20]).wrapping_add(0x1b4u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8e308 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8e30c ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e310 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8e314;super::dispatch(cpu,rt,target)?;}
pc=0x10f8e314;continue;
}
0x10f8e314=>{
// 0x10f8e314 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e318 str w0, [x20, #0x1c4]
{let address=(cpu.x[20]).wrapping_add(0x1c4u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8e31c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8e320 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e324 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8e328;super::dispatch(cpu,rt,target)?;}
pc=0x10f8e328;continue;
}
0x10f8e328=>{
// 0x10f8e328 str xzr, [x20, #0x1d0]
{let address=(cpu.x[20]).wrapping_add(0x1d0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f8e32c str w0, [x20, #0x1c8]
{let address=(cpu.x[20]).wrapping_add(0x1c8u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8e330 tbz w22, #0, #0x10f8e34c
if ((cpu.x[22] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f8e34c;continue;}pc=0x10f8e334;continue;
}
0x10f8e334=>{
// 0x10f8e334 ldr x0, [x20, #0x48]
{let address=(cpu.x[20]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f8e338 bl #0x10e0b6b4
cpu.x[30]=0x10f8e33c;super::chunk8::f_10e0b6b4(cpu,rt,0x10e0b6b4)?;
pc=0x10f8e33c;continue;
}
0x10f8e33c=>{
// 0x10f8e33c ldr x1, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10f8e340 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8e344 bl #0x111dd1b0
cpu.x[30]=0x10f8e348;super::chunk14::f_111dd1b0(cpu,rt,0x111dd1b0)?;
pc=0x10f8e348;continue;
}
0x10f8e348=>{
// 0x10f8e348 str x0, [x20, #0x1d0]
{let address=(cpu.x[20]).wrapping_add(0x1d0u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10f8e34c;continue;
}
0x10f8e34c=>{
// 0x10f8e34c ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e350 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8e354 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e358 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8e35c;super::dispatch(cpu,rt,target)?;}
pc=0x10f8e35c;continue;
}
0x10f8e35c=>{
// 0x10f8e35c ldrb w8, [x20, #0x14c]
{let address=(cpu.x[20]).wrapping_add(0x14cu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8e360 str w0, [x20, #0x1d8]
{let address=(cpu.x[20]).wrapping_add(0x1d8u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8e364 cbz w8, #0x10f8e3f8
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f8e3f8;continue;}pc=0x10f8e368;continue;
}
0x10f8e368=>{
// 0x10f8e368 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f8e36c mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f8e370 bl #0x10f8c0c4
cpu.x[30]=0x10f8e374;super::chunk6::f_10f8c0c4(cpu,rt,0x10f8c0c4)?;
// 0x10f8e374 str x0, [x20, #0x150]
{let address=(cpu.x[20]).wrapping_add(0x150u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f8e378 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f8e37c mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f8e380 bl #0x10f8c0c4
cpu.x[30]=0x10f8e384;super::chunk6::f_10f8c0c4(cpu,rt,0x10f8c0c4)?;
// 0x10f8e384 str x0, [x20, #0x158]
{let address=(cpu.x[20]).wrapping_add(0x158u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f8e388 add x1, x20, #0x160
{let a=((cpu.x[20]));let b=((0x160u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x10f8e38c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8e390 bl #0x111dd2e0
cpu.x[30]=0x10f8e394;super::chunk9::f_111dd2e0(cpu,rt,0x111dd2e0)?;
// 0x10f8e394 ldr x8, [x20, #0x150]
{let address=(cpu.x[20]).wrapping_add(0x150u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e398 ldr x21, [x20, #0x168]
{let address=(cpu.x[20]).wrapping_add(0x168u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f8e39c cbz x8, #0x10f8e3d4
if (cpu.x[8]) == 0 {pc=0x10f8e3d4;continue;}pc=0x10f8e3a0;continue;
}
0x10f8e3a0=>{
// 0x10f8e3a0 cbz x21, #0x10f8e3f8
if (cpu.x[21]) == 0 {pc=0x10f8e3f8;continue;}pc=0x10f8e3a4;continue;
}
0x10f8e3a4=>{
// 0x10f8e3a4 mov x22, x21
cpu.x[22]=(((cpu.x[21]))) as u64;
// 0x10f8e3a8 ldr x0, [x22, #0x20]!
{let address=(cpu.x[22]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;cpu.x[22]=(address) as u64;}
// 0x10f8e3ac cbz x0, #0x10f8e3b4
if (cpu.x[0]) == 0 {pc=0x10f8e3b4;continue;}pc=0x10f8e3b0;continue;
}
0x10f8e3b0=>{
// 0x10f8e3b0 bl #0x1187d360
cpu.x[30]=0x10f8e3b4;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x10f8e3b4;continue;
}
0x10f8e3b4=>{
// 0x10f8e3b4 ldr x0, [x21, #0x10]
{let address=(cpu.x[21]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f8e3b8 stp xzr, xzr, [x22]
{let address=(cpu.x[22]).wrapping_add(0x0u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f8e3bc cbz x0, #0x10f8e3c4
if (cpu.x[0]) == 0 {pc=0x10f8e3c4;continue;}pc=0x10f8e3c0;continue;
}
0x10f8e3c0=>{
// 0x10f8e3c0 bl #0x1187d360
cpu.x[30]=0x10f8e3c4;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x10f8e3c4;continue;
}
0x10f8e3c4=>{
// 0x10f8e3c4 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f8e3c8 bl #0x1187d2c0
cpu.x[30]=0x10f8e3cc;super::dispatch(cpu,rt,0x1187d2c0)?;
// 0x10f8e3cc str xzr, [x20, #0x168]
{let address=(cpu.x[20]).wrapping_add(0x168u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f8e3d0 b #0x10f8e3f8
pc=0x10f8e3f8;continue;
}
0x10f8e3d4=>{
// 0x10f8e3d4 cbnz x21, #0x10f8e3ec
if (cpu.x[21]) != 0 {pc=0x10f8e3ec;continue;}pc=0x10f8e3d8;continue;
}
0x10f8e3d8=>{
// 0x10f8e3d8 mov w0, #0x30
cpu.x[0]=(((0x30u64))) as u32 as u64;
// 0x10f8e3dc bl #0x1187d2b0
cpu.x[30]=0x10f8e3e0;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
// 0x10f8e3e0 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10f8e3e4 bl #0x10f93770
cpu.x[30]=0x10f8e3e8;super::chunk1::f_10f93770(cpu,rt,0x10f93770)?;
// 0x10f8e3e8 str x21, [x20, #0x168]
{let address=(cpu.x[20]).wrapping_add(0x168u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10f8e3ec;continue;
}
0x10f8e3ec=>{
// 0x10f8e3ec mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f8e3f0 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f8e3f4 bl #0x10f93828
cpu.x[30]=0x10f8e3f8;super::dispatch(cpu,rt,0x10f93828)?;
pc=0x10f8e3f8;continue;
}
0x10f8e3f8=>{
// 0x10f8e3f8 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e3fc mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
pc=0x10f8e400;continue;
}
0x10f8e400=>{
// 0x10f8e400 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e404 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8e408;super::dispatch(cpu,rt,target)?;}
pc=0x10f8e408;continue;
}
0x10f8e408=>{
// 0x10f8e408 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e40c str w0, [x20, #0x200]
{let address=(cpu.x[20]).wrapping_add(0x200u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8e410 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8e414 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e418 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8e41c;super::dispatch(cpu,rt,target)?;}
pc=0x10f8e41c;continue;
}
0x10f8e41c=>{
// 0x10f8e41c ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e420 str w0, [x20, #0xf8]
{let address=(cpu.x[20]).wrapping_add(0xf8u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8e424 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8e428 str wzr, [x20, #0x214]
{let address=(cpu.x[20]).wrapping_add(0x214u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8e42c ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8e430 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8e434;super::dispatch(cpu,rt,target)?;}
pc=0x10f8e434;continue;
}
0x10f8e434=>{
// 0x10f8e434 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8e438 b.lt #0x10f8e4a4
if cpu.condition(11){pc=0x10f8e4a4;continue;}pc=0x10f8e43c;continue;
}
0x10f8e43c=>{
// 0x10f8e43c mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f8e440 mov w22, #5
cpu.x[22]=(((0x5u64))) as u32 as u64;
// 0x10f8e444 b #0x10f8e460
pc=0x10f8e460;continue;
}
0x10f8e448=>{
// 0x10f8e448 ldr x9, [x20, #0x208]
{let address=(cpu.x[20]).wrapping_add(0x208u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f8e44c add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10f8e450 subs w21, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[21]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10f8e454 str w10, [x20, #0x214]
{let address=(cpu.x[20]).wrapping_add(0x214u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8e458 str w0, [x9, w8, sxtw #2]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8e45c b.eq #0x10f8e4a4
if cpu.condition(0){pc=0x10f8e4a4;continue;}pc=0x10f8e460;continue;
}
0x10f8e460=>{
// 0x10f8e460 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8e464 bl #0x111dd1e0
cpu.x[30]=0x10f8e468;super::chunk10::f_111dd1e0(cpu,rt,0x111dd1e0)?;
// 0x10f8e468 ldr w8, [x20, #0x214]
{let address=(cpu.x[20]).wrapping_add(0x214u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8e46c ldr w9, [x20, #0x210]
{let address=(cpu.x[20]).wrapping_add(0x210u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f8e470 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8e474 b.ne #0x10f8e448
if cpu.condition(1){pc=0x10f8e448;continue;}pc=0x10f8e478;continue;
}
0x10f8e478=>{
// 0x10f8e478 lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10f8e47c cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8e480 csel w1, w22, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[22] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10f8e484 cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8e488 b.ge #0x10f8e448
if cpu.condition(10){pc=0x10f8e448;continue;}pc=0x10f8e48c;continue;
}
0x10f8e48c=>{
// 0x10f8e48c mov w23, w0
cpu.x[23]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f8e490 add x0, x20, #0x208
{let a=((cpu.x[20]));let b=((0x208u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10f8e494 bl #0x1078f4d4
cpu.x[30]=0x10f8e498;super::chunk2::f_1078f4d4(cpu,rt,0x1078f4d4)?;
// 0x10f8e498 ldr w8, [x20, #0x214]
{let address=(cpu.x[20]).wrapping_add(0x214u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8e49c mov w0, w23
cpu.x[0]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10f8e4a0 b #0x10f8e448
pc=0x10f8e448;continue;
}
0x10f8e4a4=>{
// 0x10f8e4a4 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f8e4a8 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10f8e4ac ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f8e4b0 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10f8e4b4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f8e028")),}}}
#[inline]
pub(super) unsafe fn f_10f8f890(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f8f890=>{
// 0x10f8f890 sub sp, sp, #0x50
{let a=((cpu.sp));let b=((0x50u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x10f8f894 stp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f8f898 str x23, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f8f89c stp x22, x21, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f8f8a0 stp x20, x19, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f8f8a4 add x29, sp, #0x10
{let a=((cpu.sp));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x10f8f8a8 cbz x2, #0x10f8f8f8
if (cpu.x[2]) == 0 {pc=0x10f8f8f8;continue;}pc=0x10f8f8ac;continue;
}
0x10f8f8ac=>{
// 0x10f8f8ac mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f8f8b0 mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x10f8f8b4 mov x20, x2
cpu.x[20]=(((cpu.x[2]))) as u64;
// 0x10f8f8b8 mov x21, x1
cpu.x[21]=(((cpu.x[1]))) as u64;
// 0x10f8f8bc bl #0x10f940fc
cpu.x[30]=0x10f8f8c0;super::chunk9::f_10f940fc(cpu,rt,0x10f940fc)?;
// 0x10f8f8c0 cbz x0, #0x10f8f8f8
if (cpu.x[0]) == 0 {pc=0x10f8f8f8;continue;}pc=0x10f8f8c4;continue;
}
0x10f8f8c4=>{
// 0x10f8f8c4 ldr x8, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8f8c8 ldrb w8, [x8, #0x2d9]
{let address=(cpu.x[8]).wrapping_add(0x2d9u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8f8cc cbnz w8, #0x10f8f910
if (cpu.x[8] as u32 as u64) != 0 {pc=0x10f8f910;continue;}pc=0x10f8f8d0;continue;
}
0x10f8f8d0=>{
// 0x10f8f8d0 ldr x8, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8f8d4 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f8f8d8 ldr x8, [x8, #0x190]
{let address=(cpu.x[8]).wrapping_add(0x190u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8f8dc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8f8e0;super::dispatch(cpu,rt,target)?;}
// 0x10f8f8e0 tbz w0, #0, #0x10f8f910
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f8f910;continue;}pc=0x10f8f8e4;continue;
}
0x10f8f8e4=>{
// 0x10f8f8e4 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f8f8e8 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10f8f8ec mov w2, wzr
cpu.x[2]=(((0u64))) as u32 as u64;
// 0x10f8f8f0 bl #0x10f817b0
cpu.x[30]=0x10f8f8f4;super::chunk12::f_10f817b0(cpu,rt,0x10f817b0)?;
// 0x10f8f8f4 tbz w0, #0, #0x10f8f910
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f8f910;continue;}pc=0x10f8f8f8;continue;
}
0x10f8f8f8=>{
// 0x10f8f8f8 ldp x20, x19, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f8f8fc ldr x23, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10f8f900 ldp x22, x21, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f8f904 ldp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10f8f908 add sp, sp, #0x50
{let a=((cpu.sp));let b=((0x50u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10f8f90c ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f8f910=>{
// 0x10f8f910 ldr x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f8f914 ldr w1, [x19, #0x198]
{let address=(cpu.x[19]).wrapping_add(0x198u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f8f918 bl #0x10e5b064
cpu.x[30]=0x10f8f91c;super::chunk3::f_10e5b064(cpu,rt,0x10e5b064)?;
// 0x10f8f91c mov w22, w0
cpu.x[22]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f8f920 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f8f924 bl #0x10f940fc
cpu.x[30]=0x10f8f928;super::chunk9::f_10f940fc(cpu,rt,0x10f940fc)?;
// 0x10f8f928 ldr w21, [x19, #0x198]
{let address=(cpu.x[19]).wrapping_add(0x198u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[21]=(v0) as u32 as u64;}
// 0x10f8f92c mov x23, x0
cpu.x[23]=(((cpu.x[0]))) as u64;
// 0x10f8f930 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8f934 bl #0x10f93d3c
cpu.x[30]=0x10f8f938;super::chunk3::f_10f93d3c(cpu,rt,0x10f93d3c)?;
// 0x10f8f938 mov w6, w0
cpu.x[6]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f8f93c mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10f8f940 mov x1, xzr
cpu.x[1]=(((0u64))) as u64;
// 0x10f8f944 mov x2, x20
cpu.x[2]=(((cpu.x[20]))) as u64;
// 0x10f8f948 mov w3, w22
cpu.x[3]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10f8f94c mov x4, x21
cpu.x[4]=(((cpu.x[21]))) as u64;
// 0x10f8f950 mov x5, x19
cpu.x[5]=(((cpu.x[19]))) as u64;
// 0x10f8f954 mov w7, wzr
cpu.x[7]=(((0u64))) as u32 as u64;
// 0x10f8f958 str x19, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f8f95c bl #0x10fc0e08
cpu.x[30]=0x10f8f960;super::chunk14::f_10fc0e08(cpu,rt,0x10fc0e08)?;
// 0x10f8f960 ldp x20, x19, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f8f964 ldr x23, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10f8f968 ldp x22, x21, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f8f96c ldp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10f8f970 add sp, sp, #0x50
{let a=((cpu.sp));let b=((0x50u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10f8f974 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f8f890")),}}}
#[inline]
pub(super) unsafe fn f_10f934bc(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f934bc=>{
// 0x10f934bc stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f934c0 stp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f934c4 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f934c8 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f934cc mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f934d0 bl #0x10f8915c
cpu.x[30]=0x10f934d4;super::chunk9::f_10f8915c(cpu,rt,0x10f8915c)?;
pc=0x10f934d4;continue;
}
0x10f934d4=>{
// 0x10f934d4 ldr x8, [x19, #0x100]
{let address=(cpu.x[19]).wrapping_add(0x100u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f934d8 cbz x8, #0x10f934ec
if (cpu.x[8]) == 0 {pc=0x10f934ec;continue;}pc=0x10f934dc;continue;
}
0x10f934dc=>{
// 0x10f934dc ldr x0, [x8, #0x28]
{let address=(cpu.x[8]).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f934e0 cbz x0, #0x10f934ec
if (cpu.x[0]) == 0 {pc=0x10f934ec;continue;}pc=0x10f934e4;continue;
}
0x10f934e4=>{
// 0x10f934e4 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f934e8 bl #0x10f77f18
cpu.x[30]=0x10f934ec;super::chunk3::f_10f77f18(cpu,rt,0x10f77f18)?;
pc=0x10f934ec;continue;
}
0x10f934ec=>{
// 0x10f934ec mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f934f0 bl #0x10f93d2c
cpu.x[30]=0x10f934f4;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
pc=0x10f934f4;continue;
}
0x10f934f4=>{
// 0x10f934f4 ldrb w8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f934f8 cbnz w8, #0x10f93574
if (cpu.x[8] as u32 as u64) != 0 {pc=0x10f93574;continue;}pc=0x10f934fc;continue;
}
0x10f934fc=>{
// 0x10f934fc ldr x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f93500 bl #0x10e5a908
cpu.x[30]=0x10f93504;super::chunk5::f_10e5a908(cpu,rt,0x10e5a908)?;
pc=0x10f93504;continue;
}
0x10f93504=>{
// 0x10f93504 tbz w0, #0, #0x10f93574
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f93574;continue;}pc=0x10f93508;continue;
}
0x10f93508=>{
// 0x10f93508 ldr w8, [x19, #0x148]
{let address=(cpu.x[19]).wrapping_add(0x148u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f9350c cbz w8, #0x10f93564
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f93564;continue;}pc=0x10f93510;continue;
}
0x10f93510=>{
// 0x10f93510 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f93514 bl #0x10f94298
cpu.x[30]=0x10f93518;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
pc=0x10f93518;continue;
}
0x10f93518=>{
// 0x10f93518 mov w20, w0
cpu.x[20]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f9351c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f93520 bl #0x10f942a0
cpu.x[30]=0x10f93524;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
pc=0x10f93524;continue;
}
0x10f93524=>{
// 0x10f93524 mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f93528 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f9352c bl #0x10f942a8
cpu.x[30]=0x10f93530;super::chunk11::f_10f942a8(cpu,rt,0x10f942a8)?;
pc=0x10f93530;continue;
}
0x10f93530=>{
// 0x10f93530 ldr x8, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f93534 mov w22, w0
cpu.x[22]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f93538 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10f9353c bl #0x10e5ae4c
cpu.x[30]=0x10f93540;super::chunk2::f_10e5ae4c(cpu,rt,0x10e5ae4c)?;
pc=0x10f93540;continue;
}
0x10f93540=>{
// 0x10f93540 mov w4, w0
cpu.x[4]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f93544 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f93548 mov w1, w20
cpu.x[1]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x10f9354c mov w2, w21
cpu.x[2]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10f93550 mov w3, w22
cpu.x[3]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10f93554 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f93558 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f9355c ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f93560 b #0x10f8f2d0
super::chunk2::f_10f8f2d0(cpu,rt,0x10f8f2d0)?;return Ok(());
}
0x10f93564=>{
// 0x10f93564 ldr x8, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f93568 ldr w8, [x8, #0x1c0]
{let address=(cpu.x[8]).wrapping_add(0x1c0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f9356c cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f93570 b.le #0x10f93510
if cpu.condition(13){pc=0x10f93510;continue;}pc=0x10f93574;continue;
}
0x10f93574=>{
// 0x10f93574 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f93578 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f9357c ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f93580 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f934bc")),}}}
#[inline]
pub(super) unsafe fn f_10f93d3c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f93d3c=>{
// 0x10f93d3c ldr w0, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10f93d40 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f93d3c")),}}}
#[inline]
pub(super) unsafe fn f_10f93f94(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f93f94=>{
// 0x10f93f94 ldr x0, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f93f98 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f93f94")),}}}
#[inline]
pub(super) unsafe fn f_10f9414c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f9414c=>{
// 0x10f9414c stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f94150 str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f94154 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f94158 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f9415c ldr x0, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f94160 bl #0x10f966a8
cpu.x[30]=0x10f94164;super::chunk2::f_10f966a8(cpu,rt,0x10f966a8)?;
pc=0x10f94164;continue;
}
0x10f94164=>{
// 0x10f94164 ldr w1, [x19, #0x78]
{let address=(cpu.x[19]).wrapping_add(0x78u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f94168 bl #0x10d39264
cpu.x[30]=0x10f9416c;super::chunk4::f_10d39264(cpu,rt,0x10d39264)?;
pc=0x10f9416c;continue;
}
0x10f9416c=>{
// 0x10f9416c cmp w0, #0
{let a=((cpu.x[0] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f94170 cset w0, eq
cpu.x[0]=((cpu.condition(0)) as u64) as u32 as u64;
// 0x10f94174 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10f94178 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f9417c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f9414c")),}}}
#[inline]
pub(super) unsafe fn f_10f94250(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f94250=>{
// 0x10f94250 ldr w8, [x0, #0x7c]
{let address=(cpu.x[0]).wrapping_add(0x7cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f94254 mov w9, #0x7fffffff
cpu.x[9]=(((0x7fffffffu64))) as u32 as u64;
// 0x10f94258 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9425c b.ne #0x10f94268
if cpu.condition(1){pc=0x10f94268;continue;}pc=0x10f94260;continue;
}
0x10f94260=>{
// 0x10f94260 stp w1, w2, [x0, #0x84]
{let address=(cpu.x[0]).wrapping_add(0x84u64);let v0=cpu.x[1] as u32 as u64;let v1=cpu.x[2] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10f94264 str w3, [x0, #0x90]
{let address=(cpu.x[0]).wrapping_add(0x90u64);let v0=cpu.x[3] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10f94268;continue;
}
0x10f94268=>{
// 0x10f94268 stp w1, w2, [x0, #0x7c]
{let address=(cpu.x[0]).wrapping_add(0x7cu64);let v0=cpu.x[1] as u32 as u64;let v1=cpu.x[2] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10f9426c str w3, [x0, #0x8c]
{let address=(cpu.x[0]).wrapping_add(0x8cu64);let v0=cpu.x[3] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f94270 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f94250")),}}}
#[inline]
pub(super) unsafe fn f_10f9433c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f9433c=>{
// 0x10f9433c ldrb w9, [x0, #0x30]
{let address=(cpu.x[0]).wrapping_add(0x30u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f94340 mov x8, x0
cpu.x[8]=(((cpu.x[0]))) as u64;
// 0x10f94344 mov x0, xzr
cpu.x[0]=(((0u64))) as u64;
// 0x10f94348 tbz w9, #0, #0x10f94360
if ((cpu.x[9] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f94360;continue;}pc=0x10f9434c;continue;
}
0x10f9434c=>{
// 0x10f9434c ldr w9, [x8, #0x24]
{let address=(cpu.x[8]).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f94350 cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f94354 b.lt #0x10f94360
if cpu.condition(11){pc=0x10f94360;continue;}pc=0x10f94358;continue;
}
0x10f94358=>{
// 0x10f94358 ldr x8, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f9435c ldr x0, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
pc=0x10f94360;continue;
}
0x10f94360=>{
// 0x10f94360 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f9433c")),}}}
#[inline]
pub(super) unsafe fn f_10f94600(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f94600=>{
// 0x10f94600 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10f94604 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f94600")),}}}
#[inline]
pub(super) unsafe fn f_10f94674(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f94674=>{
// 0x10f94674 ldr w8, [x0, #0x24]
{let address=(cpu.x[0]).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f94678 cmp w8, #2
{let a=((cpu.x[8] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9467c b.lt #0x10f94694
if cpu.condition(11){pc=0x10f94694;continue;}pc=0x10f94680;continue;
}
0x10f94680=>{
// 0x10f94680 ldr x8, [x0, #0x18]
{let address=(cpu.x[0]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f94684 ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f94688 cmp x8, #0
{let a=((cpu.x[8]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f9468c cset w0, ne
cpu.x[0]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x10f94690 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f94694=>{
// 0x10f94694 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10f94698 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f94674")),}}}
#[inline]
pub(super) unsafe fn f_10f95b5c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f95b5c=>{
// 0x10f95b5c mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10f95b60 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f95b5c")),}}}
#[inline]
pub(super) unsafe fn f_10f95ef0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f95ef0=>{
// 0x10f95ef0 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f95ef4 str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f95ef8 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f95efc ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f95f00 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f95f04 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f95f08 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f95f0c;super::dispatch(cpu,rt,target)?;}
pc=0x10f95f0c;continue;
}
0x10f95f0c=>{
// 0x10f95f0c sub w8, w0, #5
{let a=((cpu.x[0] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10f95f10 cmp w8, #2
{let a=((cpu.x[8] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f95f14 b.lo #0x10f95f44
if cpu.condition(3){pc=0x10f95f44;continue;}pc=0x10f95f18;continue;
}
0x10f95f18=>{
// 0x10f95f18 cmp w0, #4
{let a=((cpu.x[0] as u32 as u64));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f95f1c b.eq #0x10f95f60
if cpu.condition(0){pc=0x10f95f60;continue;}pc=0x10f95f20;continue;
}
0x10f95f20=>{
// 0x10f95f20 cmp w0, #3
{let a=((cpu.x[0] as u32 as u64));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f95f24 b.ne #0x10f95f7c
if cpu.condition(1){pc=0x10f95f7c;continue;}pc=0x10f95f28;continue;
}
0x10f95f28=>{
// 0x10f95f28 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f95f2c bl #0x10f79d48
cpu.x[30]=0x10f95f30;super::chunk0::f_10f79d48(cpu,rt,0x10f79d48)?;
pc=0x10f95f30;continue;
}
0x10f95f30=>{
// 0x10f95f30 add x8, x0, #0x168
{let a=((cpu.x[0]));let b=((0x168u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10f95f34 ldr x0, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f95f38 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10f95f3c ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f95f40 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f95f44=>{
// 0x10f95f44 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f95f48 bl #0x10f7e050
cpu.x[30]=0x10f95f4c;super::chunk10::f_10f7e050(cpu,rt,0x10f7e050)?;
// 0x10f95f4c add x8, x0, #0x270
{let a=((cpu.x[0]));let b=((0x270u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10f95f50 ldr x0, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f95f54 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10f95f58 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f95f5c ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f95f60=>{
// 0x10f95f60 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f95f64 bl #0x10f8e020
cpu.x[30]=0x10f95f68;super::chunk1::f_10f8e020(cpu,rt,0x10f8e020)?;
// 0x10f95f68 add x8, x0, #0x1a8
{let a=((cpu.x[0]));let b=((0x1a8u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10f95f6c ldr x0, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f95f70 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10f95f74 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f95f78 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f95f7c=>{
// 0x10f95f7c mov x0, xzr
cpu.x[0]=(((0u64))) as u64;
// 0x10f95f80 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10f95f84 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f95f88 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f95ef0")),}}}
#[inline]
pub(super) unsafe fn f_10f966b0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f966b0=>{
// 0x10f966b0 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f966b4 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f966b8 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f966bc ldr x8, [x1]
{let address=(cpu.x[1]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f966c0 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f966c4 mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x10f966c8 mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x10f966cc ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f966d0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f966d4;super::dispatch(cpu,rt,target)?;}
pc=0x10f966d4;continue;
}
0x10f966d4=>{
// 0x10f966d4 cmp w0, #3
{let a=((cpu.x[0] as u32 as u64));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f966d8 b.ne #0x10f96798
if cpu.condition(1){pc=0x10f96798;continue;}pc=0x10f966dc;continue;
}
0x10f966dc=>{
// 0x10f966dc ldr w10, [x19, #0x8c]
{let address=(cpu.x[19]).wrapping_add(0x8cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10f966e0 ldr x9, [x19, #0x80]
{let address=(cpu.x[19]).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f966e4 cmp w10, #1
{let a=((cpu.x[10] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f966e8 b.lt #0x10f96708
if cpu.condition(11){pc=0x10f96708;continue;}pc=0x10f966ec;continue;
}
0x10f966ec=>{
// 0x10f966ec mov x11, xzr
cpu.x[11]=(((0u64))) as u64;
pc=0x10f966f0;continue;
}
0x10f966f0=>{
// 0x10f966f0 ldr x8, [x9, x11, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[11])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f966f4 cmp x8, x20
{let a=((cpu.x[8]));let b=((cpu.x[20]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f966f8 b.eq #0x10f9670c
if cpu.condition(0){pc=0x10f9670c;continue;}pc=0x10f966fc;continue;
}
0x10f966fc=>{
// 0x10f966fc add x11, x11, #1
{let a=((cpu.x[11]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
// 0x10f96700 cmp x10, x11
{let a=((cpu.x[10]));let b=((cpu.x[11]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f96704 b.ne #0x10f966f0
if cpu.condition(1){pc=0x10f966f0;continue;}pc=0x10f96708;continue;
}
0x10f96708=>{
// 0x10f96708 mov w11, #-1
cpu.x[11]=(((0xffffffffffffffffu64))) as u32 as u64;
pc=0x10f9670c;continue;
}
0x10f9670c=>{
// 0x10f9670c sub w12, w10, #1
{let a=((cpu.x[10] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[12]=(result) as u32 as u64;}
// 0x10f96710 subs w8, w12, w11
{let a=((cpu.x[12] as u32 as u64));let b=((cpu.x[11] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10f96714 str w12, [x19, #0x8c]
{let address=(cpu.x[19]).wrapping_add(0x8cu64);let v0=cpu.x[12] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f96718 b.le #0x10f96798
if cpu.condition(13){pc=0x10f96798;continue;}pc=0x10f9671c;continue;
}
0x10f9671c=>{
// 0x10f9671c cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f96720 b.lt #0x10f96798
if cpu.condition(11){pc=0x10f96798;continue;}pc=0x10f96724;continue;
}
0x10f96724=>{
// 0x10f96724 sub w10, w10, w11
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[11] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10f96728 sub w12, w10, #2
{let a=((cpu.x[10] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.x[12]=(result) as u32 as u64;}
// 0x10f9672c add x10, x9, w11, sxtw #3
{let a=((cpu.x[9]));let b=((((cpu.x[11] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x10f96730 cmp w12, #3
{let a=((cpu.x[12] as u32 as u64));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f96734 b.lo #0x10f96778
if cpu.condition(3){pc=0x10f96778;continue;}pc=0x10f96738;continue;
}
0x10f96738=>{
// 0x10f96738 sxtw x13, w11
cpu.x[13]=((((cpu.x[11] as u32 as u64))) as i32 as i64 as u64) as u64;
// 0x10f9673c add x11, x12, #1
{let a=((cpu.x[12]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
// 0x10f96740 and x12, x11, #0x1fffffffc
{let a=((cpu.x[11]));let b=((0x1fffffffcu64));let result=a & b;cpu.x[12]=(result) as u64;}
// 0x10f96744 add x9, x9, x13, lsl #3
{let a=((cpu.x[9]));let b=(((cpu.x[13])) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
// 0x10f96748 add x10, x10, x12, lsl #3
{let a=((cpu.x[10]));let b=(((cpu.x[12])) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x10f9674c sub w8, w8, w12
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[12] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10f96750 mov x13, x12
cpu.x[13]=(((cpu.x[12]))) as u64;
// 0x10f96754 add x9, x9, #0x10
{let a=((cpu.x[9]));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
pc=0x10f96758;continue;
}
0x10f96758=>{
// 0x10f96758 ldur q0, [x9, #-8]
{let address=(cpu.x[9]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));cpu.q[0]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x10f9675c ldur q1, [x9, #8]
{let address=(cpu.x[9]).wrapping_add(0x8u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));cpu.q[1]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x10f96760 subs x13, x13, #4
{let a=((cpu.x[13]));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.x[13]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x10f96764 stp q0, q1, [x9, #-0x10]
{let address=(cpu.x[9]).wrapping_add(0xfffffffffffffff0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[1] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10f96768 add x9, x9, #0x20
{let a=((cpu.x[9]));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
// 0x10f9676c b.ne #0x10f96758
if cpu.condition(1){pc=0x10f96758;continue;}pc=0x10f96770;continue;
}
0x10f96770=>{
// 0x10f96770 cmp x11, x12
{let a=((cpu.x[11]));let b=((cpu.x[12]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f96774 b.eq #0x10f96798
if cpu.condition(0){pc=0x10f96798;continue;}pc=0x10f96778;continue;
}
0x10f96778=>{
// 0x10f96778 add w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10f9677c add x9, x10, #8
{let a=((cpu.x[10]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
pc=0x10f96780;continue;
}
0x10f96780=>{
// 0x10f96780 ldr x10, [x9]
{let address=(cpu.x[9]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10f96784 sub w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10f96788 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9678c stur x10, [x9, #-8]
{let address=(cpu.x[9]).wrapping_add(0xfffffffffffffff8u64);let v0=cpu.x[10];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f96790 add x9, x9, #8
{let a=((cpu.x[9]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
// 0x10f96794 b.hi #0x10f96780
if cpu.condition(8){pc=0x10f96780;continue;}pc=0x10f96798;continue;
}
0x10f96798=>{
// 0x10f96798 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f9679c ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f967a0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f966b0")),}}}
#[inline]
pub(super) unsafe fn f_10f99b84(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f99b84=>{
// 0x10f99b84 stp x29, x30, [sp, #-0x50]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffb0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f99b88 stp x26, x25, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[26];let v1=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f99b8c stp x24, x23, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f99b90 stp x22, x21, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f99b94 stp x20, x19, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f99b98 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f99b9c ldr w21, [x0, #0x1c]
{let address=(cpu.x[0]).wrapping_add(0x1cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[21]=(v0) as u32 as u64;}
// 0x10f99ba0 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f99ba4 cmp w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f99ba8 b.lt #0x10f99c98
if cpu.condition(11){pc=0x10f99c98;continue;}pc=0x10f99bac;continue;
}
0x10f99bac=>{
// 0x10f99bac lsl x8, x21, #3
cpu.x[8]=(((((cpu.x[21]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10f99bb0 mvn w23, w21
cpu.x[23]=(!(((cpu.x[21] as u32 as u64)))) as u32 as u64;
// 0x10f99bb4 mov w20, w1
cpu.x[20]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x10f99bb8 mov w22, wzr
cpu.x[22]=(((0u64))) as u32 as u64;
// 0x10f99bbc mov w25, w23
cpu.x[25]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10f99bc0 add x24, x8, #8
{let a=((cpu.x[8]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[24]=(result) as u64;}
// 0x10f99bc4 b #0x10f99be4
pc=0x10f99be4;continue;
}
0x10f99bc8=>{
// 0x10f99bc8 bl #0x1187d2c0
cpu.x[30]=0x10f99bcc;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10f99bcc;continue;
}
0x10f99bcc=>{
// 0x10f99bcc add w22, w22, #1
{let a=((cpu.x[22] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[22]=(result) as u32 as u64;}
// 0x10f99bd0 cmp x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f99bd4 add w25, w25, #1
{let a=((cpu.x[25] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[25]=(result) as u32 as u64;}
// 0x10f99bd8 sub x24, x24, #8
{let a=((cpu.x[24]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.x[24]=(result) as u64;}
// 0x10f99bdc mov x21, x26
cpu.x[21]=(((cpu.x[26]))) as u64;
// 0x10f99be0 b.le #0x10f99c98
if cpu.condition(13){pc=0x10f99c98;continue;}pc=0x10f99be4;continue;
}
0x10f99be4=>{
// 0x10f99be4 ldur x10, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10f99be8 sub x26, x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[26]=(result) as u64;}
// 0x10f99bec add x8, x10, x26, lsl #3
{let a=((cpu.x[10]));let b=(((cpu.x[26])) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10f99bf0 ldr x0, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f99bf4 ldr w9, [x0, #0x18]
{let address=(cpu.x[0]).wrapping_add(0x18u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f99bf8 cmp w9, w20
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[20] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f99bfc b.gt #0x10f99bcc
if cpu.condition(12){pc=0x10f99bcc;continue;}pc=0x10f99c00;continue;
}
0x10f99c00=>{
// 0x10f99c00 ldrsw x13, [x19, #0x1c]
{let address=(cpu.x[19]).wrapping_add(0x1cu64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[13]=(v0) as u64;}
// 0x10f99c04 sub w9, w13, #1
{let a=((cpu.x[13] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10f99c08 cmp x21, x13
{let a=((cpu.x[21]));let b=((cpu.x[13]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f99c0c str w9, [x19, #0x1c]
{let address=(cpu.x[19]).wrapping_add(0x1cu64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f99c10 b.ge #0x10f99bc8
if cpu.condition(10){pc=0x10f99bc8;continue;}pc=0x10f99c14;continue;
}
0x10f99c14=>{
// 0x10f99c14 sub w9, w13, w21
{let a=((cpu.x[13] as u32 as u64));let b=((cpu.x[21] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10f99c18 cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f99c1c b.lt #0x10f99bc8
if cpu.condition(11){pc=0x10f99bc8;continue;}pc=0x10f99c20;continue;
}
0x10f99c20=>{
// 0x10f99c20 add w11, w22, w23
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[23] as u32 as u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u32 as u64;}
// 0x10f99c24 add w11, w13, w11
{let a=((cpu.x[13] as u32 as u64));let b=((cpu.x[11] as u32 as u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u32 as u64;}
// 0x10f99c28 cmp w11, #3
{let a=((cpu.x[11] as u32 as u64));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f99c2c b.lo #0x10f99c70
if cpu.condition(3){pc=0x10f99c70;continue;}pc=0x10f99c30;continue;
}
0x10f99c30=>{
// 0x10f99c30 add x11, x11, #1
{let a=((cpu.x[11]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
// 0x10f99c34 add w13, w13, w25
{let a=((cpu.x[13] as u32 as u64));let b=((cpu.x[25] as u32 as u64));let result=a.wrapping_add(b);cpu.x[13]=(result) as u32 as u64;}
// 0x10f99c38 add x10, x10, x24
{let a=((cpu.x[10]));let b=((cpu.x[24]));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x10f99c3c and x12, x11, #0x1fffffffc
{let a=((cpu.x[11]));let b=((0x1fffffffcu64));let result=a & b;cpu.x[12]=(result) as u64;}
// 0x10f99c40 add x13, x13, #1
{let a=((cpu.x[13]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[13]=(result) as u64;}
// 0x10f99c44 add x8, x8, x12, lsl #3
{let a=((cpu.x[8]));let b=(((cpu.x[12])) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10f99c48 sub w9, w9, w12
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[12] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10f99c4c and x13, x13, #0x1fffffffc
{let a=((cpu.x[13]));let b=((0x1fffffffcu64));let result=a & b;cpu.x[13]=(result) as u64;}
pc=0x10f99c50;continue;
}
0x10f99c50=>{
// 0x10f99c50 ldur q0, [x10, #-8]
{let address=(cpu.x[10]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));cpu.q[0]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x10f99c54 ldur q1, [x10, #8]
{let address=(cpu.x[10]).wrapping_add(0x8u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));cpu.q[1]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x10f99c58 subs x13, x13, #4
{let a=((cpu.x[13]));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.x[13]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x10f99c5c stp q0, q1, [x10, #-0x10]
{let address=(cpu.x[10]).wrapping_add(0xfffffffffffffff0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[1] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10f99c60 add x10, x10, #0x20
{let a=((cpu.x[10]));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x10f99c64 b.ne #0x10f99c50
if cpu.condition(1){pc=0x10f99c50;continue;}pc=0x10f99c68;continue;
}
0x10f99c68=>{
// 0x10f99c68 cmp x11, x12
{let a=((cpu.x[11]));let b=((cpu.x[12]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f99c6c b.eq #0x10f99c90
if cpu.condition(0){pc=0x10f99c90;continue;}pc=0x10f99c70;continue;
}
0x10f99c70=>{
// 0x10f99c70 add w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10f99c74 add x8, x8, #8
{let a=((cpu.x[8]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
pc=0x10f99c78;continue;
}
0x10f99c78=>{
// 0x10f99c78 ldr x10, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10f99c7c sub w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10f99c80 cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f99c84 stur x10, [x8, #-8]
{let address=(cpu.x[8]).wrapping_add(0xfffffffffffffff8u64);let v0=cpu.x[10];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f99c88 add x8, x8, #8
{let a=((cpu.x[8]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10f99c8c b.hi #0x10f99c78
if cpu.condition(8){pc=0x10f99c78;continue;}pc=0x10f99c90;continue;
}
0x10f99c90=>{
// 0x10f99c90 cbnz x0, #0x10f99bc8
if (cpu.x[0]) != 0 {pc=0x10f99bc8;continue;}pc=0x10f99c94;continue;
}
0x10f99c94=>{
// 0x10f99c94 b #0x10f99bcc
pc=0x10f99bcc;continue;
}
0x10f99c98=>{
// 0x10f99c98 ldrb w8, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f99c9c cbz w8, #0x10f99d60
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f99d60;continue;}pc=0x10f99ca0;continue;
}
0x10f99ca0=>{
// 0x10f99ca0 ldp w8, w10, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[8]=(v0) as u32 as u64;cpu.x[10]=(v1) as u32 as u64;}
// 0x10f99ca4 ldr w21, [x19, #0x2c]
{let address=(cpu.x[19]).wrapping_add(0x2cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[21]=(v0) as u32 as u64;}
// 0x10f99ca8 add w20, w10, w21
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[21] as u32 as u64));let result=a.wrapping_add(b);cpu.x[20]=(result) as u32 as u64;}
// 0x10f99cac cmp w8, w20
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[20] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f99cb0 b.ge #0x10f99cc4
if cpu.condition(10){pc=0x10f99cc4;continue;}pc=0x10f99cb4;continue;
}
0x10f99cb4=>{
// 0x10f99cb4 add x0, x19, #0x10
{let a=((cpu.x[19]));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10f99cb8 mov w1, w20
cpu.x[1]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x10f99cbc bl #0x10f99e10
cpu.x[30]=0x10f99cc0;super::chunk13::f_10f99e10(cpu,rt,0x10f99e10)?;
pc=0x10f99cc0;continue;
}
0x10f99cc0=>{
// 0x10f99cc0 ldr w10, [x19, #0x1c]
{let address=(cpu.x[19]).wrapping_add(0x1cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
pc=0x10f99cc4;continue;
}
0x10f99cc4=>{
// 0x10f99cc4 cmp w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f99cc8 b.lt #0x10f99d58
if cpu.condition(11){pc=0x10f99d58;continue;}pc=0x10f99ccc;continue;
}
0x10f99ccc=>{
// 0x10f99ccc ldr x11, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[11]=(v0) as u64;}
// 0x10f99cd0 ldr x9, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f99cd4 cmp w21, #8
{let a=((cpu.x[21] as u32 as u64));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f99cd8 add x8, x11, w10, sxtw #3
{let a=((cpu.x[11]));let b=((((cpu.x[10] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10f99cdc b.lo #0x10f99d40
if cpu.condition(3){pc=0x10f99d40;continue;}pc=0x10f99ce0;continue;
}
0x10f99ce0=>{
// 0x10f99ce0 sxtw x10, w10
cpu.x[10]=((((cpu.x[10] as u32 as u64))) as i32 as i64 as u64) as u64;
// 0x10f99ce4 lsl x10, x10, #3
cpu.x[10]=(((((cpu.x[10]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10f99ce8 add x13, x10, x11
{let a=((cpu.x[10]));let b=((cpu.x[11]));let result=a.wrapping_add(b);cpu.x[13]=(result) as u64;}
// 0x10f99cec sub x10, x13, x9
{let a=((cpu.x[13]));let b=((cpu.x[9]));let result=a.wrapping_sub(b);cpu.x[10]=(result) as u64;}
// 0x10f99cf0 cmp x10, #0x20
{let a=((cpu.x[10]));let b=((0x20u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f99cf4 b.lo #0x10f99d40
if cpu.condition(3){pc=0x10f99d40;continue;}pc=0x10f99cf8;continue;
}
0x10f99cf8=>{
// 0x10f99cf8 mov w10, w21
cpu.x[10]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10f99cfc add x13, x13, #0x10
{let a=((cpu.x[13]));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[13]=(result) as u64;}
// 0x10f99d00 and x11, x10, #0x7ffffffc
{let a=((cpu.x[10]));let b=((0x7ffffffcu64));let result=a & b;cpu.x[11]=(result) as u64;}
// 0x10f99d04 lsl x14, x11, #3
cpu.x[14]=(((((cpu.x[11]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10f99d08 sub w21, w21, w11
{let a=((cpu.x[21] as u32 as u64));let b=((cpu.x[11] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[21]=(result) as u32 as u64;}
// 0x10f99d0c add x12, x9, x14
{let a=((cpu.x[9]));let b=((cpu.x[14]));let result=a.wrapping_add(b);cpu.x[12]=(result) as u64;}
// 0x10f99d10 add x8, x8, x14
{let a=((cpu.x[8]));let b=((cpu.x[14]));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10f99d14 add x9, x9, #0x10
{let a=((cpu.x[9]));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
// 0x10f99d18 mov x14, x11
cpu.x[14]=(((cpu.x[11]))) as u64;
pc=0x10f99d1c;continue;
}
0x10f99d1c=>{
// 0x10f99d1c ldp q0, q1, [x9, #-0x10]
{let address=(cpu.x[9]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));let v1=rt.memory.read::<u128>(address.wrapping_add(16));cpu.q[0]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;cpu.q[1]=(v1) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x10f99d20 subs x14, x14, #4
{let a=((cpu.x[14]));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.x[14]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x10f99d24 add x9, x9, #0x20
{let a=((cpu.x[9]));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
// 0x10f99d28 stp q0, q1, [x13, #-0x10]
{let address=(cpu.x[13]).wrapping_add(0xfffffffffffffff0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[1] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10f99d2c add x13, x13, #0x20
{let a=((cpu.x[13]));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[13]=(result) as u64;}
// 0x10f99d30 b.ne #0x10f99d1c
if cpu.condition(1){pc=0x10f99d1c;continue;}pc=0x10f99d34;continue;
}
0x10f99d34=>{
// 0x10f99d34 cmp x11, x10
{let a=((cpu.x[11]));let b=((cpu.x[10]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f99d38 mov x9, x12
cpu.x[9]=(((cpu.x[12]))) as u64;
// 0x10f99d3c b.eq #0x10f99d58
if cpu.condition(0){pc=0x10f99d58;continue;}pc=0x10f99d40;continue;
}
0x10f99d40=>{
// 0x10f99d40 add w10, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
pc=0x10f99d44;continue;
}
0x10f99d44=>{
// 0x10f99d44 ldr x11, [x9], #8
{let address=(cpu.x[9]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[11]=(v0) as u64;cpu.x[9]=((cpu.x[9]).wrapping_add(((0x8u64)))) as u64;}
// 0x10f99d48 sub w10, w10, #1
{let a=((cpu.x[10] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10f99d4c cmp w10, #1
{let a=((cpu.x[10] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f99d50 str x11, [x8], #8
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=cpu.x[11];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);cpu.x[8]=((cpu.x[8]).wrapping_add(((0x8u64)))) as u64;}
// 0x10f99d54 b.hi #0x10f99d44
if cpu.condition(8){pc=0x10f99d44;continue;}pc=0x10f99d58;continue;
}
0x10f99d58=>{
// 0x10f99d58 str w20, [x19, #0x1c]
{let address=(cpu.x[19]).wrapping_add(0x1cu64);let v0=cpu.x[20] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f99d5c str wzr, [x19, #0x2c]
{let address=(cpu.x[19]).wrapping_add(0x2cu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10f99d60;continue;
}
0x10f99d60=>{
// 0x10f99d60 ldp x20, x19, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f99d64 ldp x22, x21, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f99d68 ldp x24, x23, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10f99d6c ldp x26, x25, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10f99d70 ldp x29, x30, [sp], #0x50
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x50u64)))) as u64;}
// 0x10f99d74 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f99b84")),}}}
#[inline]
pub(super) unsafe fn f_10f9d820(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f9d820=>{
// 0x10f9d820 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f9d824 stp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f9d828 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f9d82c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f9d830 mov x20, x3
cpu.x[20]=(((cpu.x[3]))) as u64;
// 0x10f9d834 mov w21, w2
cpu.x[21]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x10f9d838 mov x22, x1
cpu.x[22]=(((cpu.x[1]))) as u64;
// 0x10f9d83c mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f9d840 bl #0x10f93d2c
cpu.x[30]=0x10f9d844;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
pc=0x10f9d844;continue;
}
0x10f9d844=>{
// 0x10f9d844 mov x3, x0
cpu.x[3]=(((cpu.x[0]))) as u64;
// 0x10f9d848 add x0, x19, #0x208
{let a=((cpu.x[19]));let b=((0x208u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10f9d84c mov x1, x22
cpu.x[1]=(((cpu.x[22]))) as u64;
// 0x10f9d850 mov w2, w21
cpu.x[2]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10f9d854 mov x4, x20
cpu.x[4]=(((cpu.x[20]))) as u64;
// 0x10f9d858 mov x5, x19
cpu.x[5]=(((cpu.x[19]))) as u64;
// 0x10f9d85c bl #0x10fa49a8
cpu.x[30]=0x10f9d860;super::chunk14::f_10fa49a8(cpu,rt,0x10fa49a8)?;
pc=0x10f9d860;continue;
}
0x10f9d860=>{
// 0x10f9d860 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f9d864 bl #0x10f9d8ec
cpu.x[30]=0x10f9d868;super::chunk2::f_10f9d8ec(cpu,rt,0x10f9d8ec)?;
pc=0x10f9d868;continue;
}
0x10f9d868=>{
// 0x10f9d868 ldr x8, [x22]
{let address=(cpu.x[22]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f9d86c mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f9d870 ldr x8, [x8, #0x58]
{let address=(cpu.x[8]).wrapping_add(0x58u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f9d874 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f9d878;super::dispatch(cpu,rt,target)?;}
pc=0x10f9d878;continue;
}
0x10f9d878=>{
// 0x10f9d878 ldr w8, [x19, #0x2f4]
{let address=(cpu.x[19]).wrapping_add(0x2f4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f9d87c mov w20, w0
cpu.x[20]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f9d880 cmp w8, w0
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9d884 b.ge #0x10f9d8d0
if cpu.condition(10){pc=0x10f9d8d0;continue;}pc=0x10f9d888;continue;
}
0x10f9d888=>{
// 0x10f9d888 ldr w9, [x19, #0x2f0]
{let address=(cpu.x[19]).wrapping_add(0x2f0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f9d88c add x21, x19, #0x2e8
{let a=((cpu.x[19]));let b=((0x2e8u64));let result=a.wrapping_add(b);cpu.x[21]=(result) as u64;}
// 0x10f9d890 cmp w9, w20
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[20] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9d894 b.ge #0x10f9d8a8
if cpu.condition(10){pc=0x10f9d8a8;continue;}pc=0x10f9d898;continue;
}
0x10f9d898=>{
// 0x10f9d898 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f9d89c mov w1, w20
cpu.x[1]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x10f9d8a0 bl #0x1078f4d4
cpu.x[30]=0x10f9d8a4;super::chunk2::f_1078f4d4(cpu,rt,0x1078f4d4)?;
pc=0x10f9d8a4;continue;
}
0x10f9d8a4=>{
// 0x10f9d8a4 ldr w8, [x19, #0x2f4]
{let address=(cpu.x[19]).wrapping_add(0x2f4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
pc=0x10f9d8a8;continue;
}
0x10f9d8a8=>{
// 0x10f9d8a8 cmp w8, w20
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[20] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9d8ac b.ge #0x10f9d8d4
if cpu.condition(10){pc=0x10f9d8d4;continue;}pc=0x10f9d8b0;continue;
}
0x10f9d8b0=>{
// 0x10f9d8b0 sxtw x9, w20
cpu.x[9]=((((cpu.x[20] as u32 as u64))) as i32 as i64 as u64) as u64;
// 0x10f9d8b4 ldr x10, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10f9d8b8 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10f9d8bc sub x9, x9, w8, sxtw
{let a=((cpu.x[9]));let b=(((cpu.x[8] as u32 as u64) as i32 as i64 as u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u64;}
// 0x10f9d8c0 add x0, x10, w8, sxtw #2
{let a=((cpu.x[10]));let b=((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64;let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10f9d8c4 lsl x2, x9, #2
cpu.x[2]=(((((cpu.x[9]))) as u64).wrapping_shl(((((0x2u64))) & 63) as u32) as u64) as u64;
// 0x10f9d8c8 bl #0x1187d480
cpu.x[30]=0x10f9d8cc;super::chunk15::f_1187d480(cpu,rt,0x1187d480)?;
pc=0x10f9d8cc;continue;
}
0x10f9d8cc=>{
// 0x10f9d8cc b #0x10f9d8d4
pc=0x10f9d8d4;continue;
}
0x10f9d8d0=>{
// 0x10f9d8d0 b.le #0x10f9d8d8
if cpu.condition(13){pc=0x10f9d8d8;continue;}pc=0x10f9d8d4;continue;
}
0x10f9d8d4=>{
// 0x10f9d8d4 str w20, [x19, #0x2f4]
{let address=(cpu.x[19]).wrapping_add(0x2f4u64);let v0=cpu.x[20] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10f9d8d8;continue;
}
0x10f9d8d8=>{
// 0x10f9d8d8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f9d8dc ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f9d8e0 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f9d8e4 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f9d8e8 b #0x10f9dbac
super::chunk4::f_10f9dbac(cpu,rt,0x10f9dbac)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f9d820")),}}}
#[inline]
pub(super) unsafe fn f_10fa3758(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fa3758=>{
// 0x10fa3758 ldr x0, [x0, #0x248]
{let address=(cpu.x[0]).wrapping_add(0x248u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fa375c cbz x0, #0x10fa3764
if (cpu.x[0]) == 0 {pc=0x10fa3764;continue;}pc=0x10fa3760;continue;
}
0x10fa3760=>{
// 0x10fa3760 b #0x10f996f4
super::chunk9::f_10f996f4(cpu,rt,0x10f996f4)?;return Ok(());
}
0x10fa3764=>{
// 0x10fa3764 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fa3758")),}}}
#[inline]
pub(super) unsafe fn f_10fa3d08(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fa3d08=>{
// 0x10fa3d08 sub sp, sp, #0x50
{let a=((cpu.sp));let b=((0x50u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x10fa3d0c stp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fa3d10 str x23, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fa3d14 stp x22, x21, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fa3d18 stp x20, x19, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fa3d1c add x29, sp, #0x10
{let a=((cpu.sp));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x10fa3d20 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fa3d24 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10fa3d28 ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fa3d2c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fa3d30;super::dispatch(cpu,rt,target)?;}
pc=0x10fa3d30;continue;
}
0x10fa3d30=>{
// 0x10fa3d30 bl #0x10fca234
cpu.x[30]=0x10fa3d34;super::chunk2::f_10fca234(cpu,rt,0x10fca234)?;
pc=0x10fa3d34;continue;
}
0x10fa3d34=>{
// 0x10fa3d34 ldr x20, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10fa3d38 str w0, [x19, #0x310]
{let address=(cpu.x[19]).wrapping_add(0x310u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fa3d3c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fa3d40 bl #0x10f93d2c
cpu.x[30]=0x10fa3d44;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
pc=0x10fa3d44;continue;
}
0x10fa3d44=>{
// 0x10fa3d44 bl #0x10d33f6c
cpu.x[30]=0x10fa3d48;super::chunk12::f_10d33f6c(cpu,rt,0x10d33f6c)?;
pc=0x10fa3d48;continue;
}
0x10fa3d48=>{
// 0x10fa3d48 cmp w0, #3
{let a=((cpu.x[0] as u32 as u64));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa3d4c b.gt #0x10fa3d78
if cpu.condition(12){pc=0x10fa3d78;continue;}pc=0x10fa3d50;continue;
}
0x10fa3d50=>{
// 0x10fa3d50 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fa3d54 bl #0x10f93d2c
cpu.x[30]=0x10fa3d58;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
pc=0x10fa3d58;continue;
}
0x10fa3d58=>{
// 0x10fa3d58 bl #0x10d33f6c
cpu.x[30]=0x10fa3d5c;super::chunk12::f_10d33f6c(cpu,rt,0x10d33f6c)?;
pc=0x10fa3d5c;continue;
}
0x10fa3d5c=>{
// 0x10fa3d5c cmp w0, #3
{let a=((cpu.x[0] as u32 as u64));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa3d60 b.lt #0x10fa3dd8
if cpu.condition(11){pc=0x10fa3dd8;continue;}pc=0x10fa3d64;continue;
}
0x10fa3d64=>{
// 0x10fa3d64 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fa3d68 bl #0x10f93d2c
cpu.x[30]=0x10fa3d6c;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
// 0x10fa3d6c mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10fa3d70 bl #0x10d3c2ac
cpu.x[30]=0x10fa3d74;super::chunk1::f_10d3c2ac(cpu,rt,0x10d3c2ac)?;
// 0x10fa3d74 cbz x0, #0x10fa3dd8
if (cpu.x[0]) == 0 {pc=0x10fa3dd8;continue;}pc=0x10fa3d78;continue;
}
0x10fa3d78=>{
// 0x10fa3d78 ldr x20, [x20, #0x90]
{let address=(cpu.x[20]).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10fa3d7c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fa3d80 bl #0x10f94298
cpu.x[30]=0x10fa3d84;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
// 0x10fa3d84 mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fa3d88 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fa3d8c bl #0x10f942a0
cpu.x[30]=0x10fa3d90;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
// 0x10fa3d90 mov w22, w0
cpu.x[22]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fa3d94 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fa3d98 bl #0x10f942a8
cpu.x[30]=0x10fa3d9c;super::chunk11::f_10f942a8(cpu,rt,0x10f942a8)?;
// 0x10fa3d9c mov w23, w0
cpu.x[23]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fa3da0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fa3da4 bl #0x10f93fc0
cpu.x[30]=0x10fa3da8;super::chunk5::f_10f93fc0(cpu,rt,0x10f93fc0)?;
// 0x10fa3da8 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fa3dac mov x6, x0
cpu.x[6]=(((cpu.x[0]))) as u64;
// 0x10fa3db0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fa3db4 mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10fa3db8 mov w2, w22
cpu.x[2]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10fa3dbc mov w3, w23
cpu.x[3]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10fa3dc0 ldr x8, [x8, #0x60]
{let address=(cpu.x[8]).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fa3dc4 mov x4, x19
cpu.x[4]=(((cpu.x[19]))) as u64;
// 0x10fa3dc8 mov x5, xzr
cpu.x[5]=(((0u64))) as u64;
// 0x10fa3dcc mov w7, wzr
cpu.x[7]=(((0u64))) as u32 as u64;
// 0x10fa3dd0 str wzr, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fa3dd4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fa3dd8;super::dispatch(cpu,rt,target)?;}
pc=0x10fa3dd8;continue;
}
0x10fa3dd8=>{
// 0x10fa3dd8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fa3ddc ldp x20, x19, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fa3de0 ldr x23, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10fa3de4 ldp x22, x21, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fa3de8 ldp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fa3dec add sp, sp, #0x50
{let a=((cpu.sp));let b=((0x50u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fa3df0 b #0x10f83990
super::chunk14::f_10f83990(cpu,rt,0x10f83990)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fa3d08")),}}}
#[inline]
pub(super) unsafe fn f_10fa492c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fa492c=>{
// 0x10fa492c stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fa4930 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fa4934 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fa4938 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fa493c movi v0.2d, #0000000000000000
cpu.q[0]=(lane_dup(0u128,64,2)) & 0xffffffffffffffffffffffffffffffffu128;
// 0x10fa4940 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10fa4944 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10fa4948 str wzr, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fa494c mov w1, #4
cpu.x[1]=(((0x4u64))) as u32 as u64;
// 0x10fa4950 mov w21, #4
cpu.x[21]=(((0x4u64))) as u32 as u64;
// 0x10fa4954 str wzr, [x0, #0x38]
{let address=(cpu.x[0]).wrapping_add(0x38u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fa4958 str q0, [x20, #8]!
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);cpu.x[20]=(address) as u64;}
// 0x10fa495c stur q0, [x0, #0x18]
{let address=(cpu.x[0]).wrapping_add(0x18u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10fa4960 stur q0, [x0, #0x28]
{let address=(cpu.x[0]).wrapping_add(0x28u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10fa4964 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fa4968 bl #0x1078f4d4
cpu.x[30]=0x10fa496c;super::chunk2::f_1078f4d4(cpu,rt,0x1078f4d4)?;
pc=0x10fa496c;continue;
}
0x10fa496c=>{
// 0x10fa496c ldrsw x8, [x19, #0x14]
{let address=(cpu.x[19]).wrapping_add(0x14u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10fa4970 cmp w8, #3
{let a=((cpu.x[8] as u32 as u64));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa4974 b.gt #0x10fa4994
if cpu.condition(12){pc=0x10fa4994;continue;}pc=0x10fa4978;continue;
}
0x10fa4978=>{
// 0x10fa4978 lsl x8, x8, #2
cpu.x[8]=(((((cpu.x[8]))) as u64).wrapping_shl(((((0x2u64))) & 63) as u32) as u64) as u64;
// 0x10fa497c ldr x9, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fa4980 mov w10, #0x10
cpu.x[10]=(((0x10u64))) as u32 as u64;
// 0x10fa4984 mov w1, #0xff
cpu.x[1]=(((0xffu64))) as u32 as u64;
// 0x10fa4988 sub x2, x10, x8
{let a=((cpu.x[10]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.x[2]=(result) as u64;}
// 0x10fa498c add x0, x9, x8
{let a=((cpu.x[9]));let b=((cpu.x[8]));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fa4990 bl #0x1187d480
cpu.x[30]=0x10fa4994;super::chunk15::f_1187d480(cpu,rt,0x1187d480)?;
pc=0x10fa4994;continue;
}
0x10fa4994=>{
// 0x10fa4994 str w21, [x19, #0x14]
{let address=(cpu.x[19]).wrapping_add(0x14u64);let v0=cpu.x[21] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fa4998 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fa499c ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10fa49a0 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10fa49a4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fa492c")),}}}
#[inline]
pub(super) unsafe fn f_10fa87bc(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fa87bc=>{
// 0x10fa87bc stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fa87c0 str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fa87c4 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fa87c8 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10fa87cc bl #0x10f73c60
cpu.x[30]=0x10fa87d0;super::chunk7::f_10f73c60(cpu,rt,0x10f73c60)?;
pc=0x10fa87d0;continue;
}
0x10fa87d0=>{
// 0x10fa87d0 ldr x0, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fa87d4 bl #0x10f94608
cpu.x[30]=0x10fa87d8;super::chunk12::f_10f94608(cpu,rt,0x10f94608)?;
pc=0x10fa87d8;continue;
}
0x10fa87d8=>{
// 0x10fa87d8 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10fa87dc ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10fa87e0 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10fa87e4 b #0x10fc9410
super::chunk12::f_10fc9410(cpu,rt,0x10fc9410)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fa87bc")),}}}
#[inline]
pub(super) unsafe fn f_10fad0ac(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fad0ac=>{
// 0x10fad0ac stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fad0b0 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fad0b4 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fad0b8 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fad0bc mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10fad0c0 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10fad0c4 cbz x1, #0x10fad11c
if (cpu.x[1]) == 0 {pc=0x10fad11c;continue;}pc=0x10fad0c8;continue;
}
0x10fad0c8=>{
// 0x10fad0c8 ldr x8, [x20, #0x90]
{let address=(cpu.x[20]).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fad0cc cbz x8, #0x10fad11c
if (cpu.x[8]) == 0 {pc=0x10fad11c;continue;}pc=0x10fad0d0;continue;
}
0x10fad0d0=>{
// 0x10fad0d0 ldr x0, [x20, #0x20]
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fad0d4 mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10fad0d8 bl #0x10f93d3c
cpu.x[30]=0x10fad0dc;super::chunk3::f_10f93d3c(cpu,rt,0x10f93d3c)?;
pc=0x10fad0dc;continue;
}
0x10fad0dc=>{
// 0x10fad0dc mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fad0e0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fad0e4 bl #0x10f93d3c
cpu.x[30]=0x10fad0e8;super::chunk3::f_10f93d3c(cpu,rt,0x10f93d3c)?;
pc=0x10fad0e8;continue;
}
0x10fad0e8=>{
// 0x10fad0e8 cmp w21, w0
{let a=((cpu.x[21] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fad0ec b.ne #0x10fad12c
if cpu.condition(1){pc=0x10fad12c;continue;}pc=0x10fad0f0;continue;
}
0x10fad0f0=>{
// 0x10fad0f0 ldr w21, [x20, #0x68]
{let address=(cpu.x[20]).wrapping_add(0x68u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[21]=(v0) as u32 as u64;}
// 0x10fad0f4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fad0f8 bl #0x10f95c08
cpu.x[30]=0x10fad0fc;super::chunk13::f_10f95c08(cpu,rt,0x10f95c08)?;
pc=0x10fad0fc;continue;
}
0x10fad0fc=>{
// 0x10fad0fc cmp w21, w0
{let a=((cpu.x[21] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fad100 b.ne #0x10fad12c
if cpu.condition(1){pc=0x10fad12c;continue;}pc=0x10fad104;continue;
}
0x10fad104=>{
// 0x10fad104 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fad108 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fad10c ldr x8, [x8, #0x140]
{let address=(cpu.x[8]).wrapping_add(0x140u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fad110 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fad114;super::dispatch(cpu,rt,target)?;}
pc=0x10fad114;continue;
}
0x10fad114=>{
// 0x10fad114 tbz w0, #0, #0x10fad140
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fad140;continue;}pc=0x10fad118;continue;
}
0x10fad118=>{
// 0x10fad118 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
pc=0x10fad11c;continue;
}
0x10fad11c=>{
// 0x10fad11c ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fad120 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10fad124 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10fad128 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10fad12c=>{
// 0x10fad12c mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10fad130 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fad134 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10fad138 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10fad13c ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10fad140=>{
// 0x10fad140 ldr x8, [x20, #0x90]
{let address=(cpu.x[20]).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fad144 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fad148 ldr x20, [x8, #0x710]
{let address=(cpu.x[8]).wrapping_add(0x710u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10fad14c bl #0x10f7e050
cpu.x[30]=0x10fad150;super::chunk10::f_10f7e050(cpu,rt,0x10f7e050)?;
pc=0x10fad150;continue;
}
0x10fad150=>{
// 0x10fad150 ldr x8, [x0, #0x710]
{let address=(cpu.x[0]).wrapping_add(0x710u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fad154 cmp x20, x8
{let a=((cpu.x[20]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fad158 cset w0, eq
cpu.x[0]=((cpu.condition(0)) as u64) as u32 as u64;
// 0x10fad15c ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fad160 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10fad164 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10fad168 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fad0ac")),}}}
#[inline]
pub(super) unsafe fn f_10fc06b4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fc06b4=>{
// 0x10fc06b4 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fc06b8 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fc06bc stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc06c0 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fc06c4 ldr w8, [x0, #0x24]
{let address=(cpu.x[0]).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc06c8 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10fc06cc cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc06d0 b.lt #0x10fc070c
if cpu.condition(11){pc=0x10fc070c;continue;}pc=0x10fc06d4;continue;
}
0x10fc06d4=>{
// 0x10fc06d4 mov x21, xzr
cpu.x[21]=(((0u64))) as u64;
// 0x10fc06d8 b #0x10fc06e8
pc=0x10fc06e8;continue;
}
0x10fc06dc=>{
// 0x10fc06dc add x21, x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[21]=(result) as u64;}
// 0x10fc06e0 cmp x21, w8, sxtw
{let a=((cpu.x[21]));let b=(((cpu.x[8] as u32 as u64) as i32 as i64 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fc06e4 b.ge #0x10fc070c
if cpu.condition(10){pc=0x10fc070c;continue;}pc=0x10fc06e8;continue;
}
0x10fc06e8=>{
// 0x10fc06e8 ldr x9, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fc06ec ldr x20, [x9, x21, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[21])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10fc06f0 cbz x20, #0x10fc06dc
if (cpu.x[20]) == 0 {pc=0x10fc06dc;continue;}pc=0x10fc06f4;continue;
}
0x10fc06f4=>{
// 0x10fc06f4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fc06f8 bl #0x10f86100
cpu.x[30]=0x10fc06fc;super::chunk4::f_10f86100(cpu,rt,0x10f86100)?;
pc=0x10fc06fc;continue;
}
0x10fc06fc=>{
// 0x10fc06fc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fc0700 bl #0x1187d2c0
cpu.x[30]=0x10fc0704;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10fc0704;continue;
}
0x10fc0704=>{
// 0x10fc0704 ldr w8, [x19, #0x24]
{let address=(cpu.x[19]).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc0708 b #0x10fc06dc
pc=0x10fc06dc;continue;
}
0x10fc070c=>{
// 0x10fc070c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc0710 bl #0x10fc96e0
cpu.x[30]=0x10fc0714;super::chunk14::f_10fc96e0(cpu,rt,0x10fc96e0)?;
pc=0x10fc0714;continue;
}
0x10fc0714=>{
// 0x10fc0714 adrp x8, #0x102b3000
cpu.x[8]=(0x102b3000u64) as u64;
// 0x10fc0718 str xzr, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fc071c ldr q0, [x8, #0xd50]
{let address=(cpu.x[8]).wrapping_add(0xd50u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));cpu.q[0]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x10fc0720 strb wzr, [x19, #0x4c]
{let address=(cpu.x[19]).wrapping_add(0x4cu64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10fc0724 str wzr, [x19, #0x24]
{let address=(cpu.x[19]).wrapping_add(0x24u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc0728 str xzr, [x19, #0x40]
{let address=(cpu.x[19]).wrapping_add(0x40u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fc072c str xzr, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fc0730 str q0, [x19, #0x30]
{let address=(cpu.x[19]).wrapping_add(0x30u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10fc0734 str wzr, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc0738 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fc073c ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10fc0740 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10fc0744 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fc06b4")),}}}
#[inline]
pub(super) unsafe fn f_10fc1e00(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fc1e00=>{
// 0x10fc1e00 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fc1e04 stp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc1e08 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc1e0c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fc1e10 ldr w22, [x0, #0x24]
{let address=(cpu.x[0]).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[22]=(v0) as u32 as u64;}
// 0x10fc1e14 cmp w22, #1
{let a=((cpu.x[22] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc1e18 b.lt #0x10fc1e70
if cpu.condition(11){pc=0x10fc1e70;continue;}pc=0x10fc1e1c;continue;
}
0x10fc1e1c=>{
// 0x10fc1e1c mov x19, x2
cpu.x[19]=(((cpu.x[2]))) as u64;
// 0x10fc1e20 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10fc1e24 mov x21, x1
cpu.x[21]=(((cpu.x[1]))) as u64;
// 0x10fc1e28 b #0x10fc1e3c
pc=0x10fc1e3c;continue;
}
0x10fc1e2c=>{
// 0x10fc1e2c sub x22, x22, #1
{let a=((cpu.x[22]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[22]=(result) as u64;}
// 0x10fc1e30 add x8, x22, #1
{let a=((cpu.x[22]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fc1e34 cmp x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fc1e38 b.ls #0x10fc1e70
if cpu.condition(9){pc=0x10fc1e70;continue;}pc=0x10fc1e3c;continue;
}
0x10fc1e3c=>{
// 0x10fc1e3c ldr x8, [x20, #0x18]
{let address=(cpu.x[20]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc1e40 add x8, x8, x22, lsl #3
{let a=((cpu.x[8]));let b=(((cpu.x[22])) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fc1e44 ldur x8, [x8, #-8]
{let address=(cpu.x[8]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc1e48 ldr x9, [x8, #0x30]
{let address=(cpu.x[8]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fc1e4c cmp x9, x21
{let a=((cpu.x[9]));let b=((cpu.x[21]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fc1e50 b.ne #0x10fc1e2c
if cpu.condition(1){pc=0x10fc1e2c;continue;}pc=0x10fc1e54;continue;
}
0x10fc1e54=>{
// 0x10fc1e54 ldr x8, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc1e58 cmp x8, x19
{let a=((cpu.x[8]));let b=((cpu.x[19]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fc1e5c b.ne #0x10fc1e2c
if cpu.condition(1){pc=0x10fc1e2c;continue;}pc=0x10fc1e60;continue;
}
0x10fc1e60=>{
// 0x10fc1e60 sub w1, w22, #1
{let a=((cpu.x[22] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[1]=(result) as u32 as u64;}
// 0x10fc1e64 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fc1e68 bl #0x10fc0558
cpu.x[30]=0x10fc1e6c;super::chunk5::f_10fc0558(cpu,rt,0x10fc0558)?;
pc=0x10fc1e6c;continue;
}
0x10fc1e6c=>{
// 0x10fc1e6c b #0x10fc1e2c
pc=0x10fc1e2c;continue;
}
0x10fc1e70=>{
// 0x10fc1e70 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fc1e74 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fc1e78 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10fc1e7c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fc1e00")),}}}
#[inline]
pub(super) unsafe fn f_10fc2098(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fc2098=>{
// 0x10fc2098 ldr w8, [x0, #0x24]
{let address=(cpu.x[0]).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc209c cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc20a0 b.lt #0x10fc20c4
if cpu.condition(11){pc=0x10fc20c4;continue;}pc=0x10fc20a4;continue;
}
0x10fc20a4=>{
// 0x10fc20a4 ldr x9, [x0, #0x18]
{let address=(cpu.x[0]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fc20a8 ldr x10, [x9]
{let address=(cpu.x[9]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10fc20ac ldr x10, [x10, #0x18]
{let address=(cpu.x[10]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10fc20b0 ldr w10, [x10, #0x140]
{let address=(cpu.x[10]).wrapping_add(0x140u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10fc20b4 cmp w10, #0
{let a=((cpu.x[10] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc20b8 b.le #0x10fc20cc
if cpu.condition(13){pc=0x10fc20cc;continue;}pc=0x10fc20bc;continue;
}
0x10fc20bc=>{
// 0x10fc20bc mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
// 0x10fc20c0 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10fc20c4=>{
// 0x10fc20c4 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10fc20c8 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10fc20cc=>{
// 0x10fc20cc mov w11, #1
cpu.x[11]=(((0x1u64))) as u32 as u64;
pc=0x10fc20d0;continue;
}
0x10fc20d0=>{
// 0x10fc20d0 mov x10, x11
cpu.x[10]=(((cpu.x[11]))) as u64;
// 0x10fc20d4 cmp x8, x11
{let a=((cpu.x[8]));let b=((cpu.x[11]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fc20d8 b.eq #0x10fc20f4
if cpu.condition(0){pc=0x10fc20f4;continue;}pc=0x10fc20dc;continue;
}
0x10fc20dc=>{
// 0x10fc20dc ldr x11, [x9, x10, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[10])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[11]=(v0) as u64;}
// 0x10fc20e0 ldr x11, [x11, #0x18]
{let address=(cpu.x[11]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[11]=(v0) as u64;}
// 0x10fc20e4 ldr w11, [x11, #0x140]
{let address=(cpu.x[11]).wrapping_add(0x140u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
// 0x10fc20e8 cmp w11, #1
{let a=((cpu.x[11] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc20ec add x11, x10, #1
{let a=((cpu.x[10]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
// 0x10fc20f0 b.lt #0x10fc20d0
if cpu.condition(11){pc=0x10fc20d0;continue;}pc=0x10fc20f4;continue;
}
0x10fc20f4=>{
// 0x10fc20f4 cmp x10, x8
{let a=((cpu.x[10]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fc20f8 cset w0, lo
cpu.x[0]=((cpu.condition(3)) as u64) as u32 as u64;
// 0x10fc20fc ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fc2098")),}}}
#[inline]
pub(super) unsafe fn f_10fc22f0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fc22f0=>{
// 0x10fc22f0 ldr w0, [x0, #0x24]
{let address=(cpu.x[0]).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10fc22f4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fc22f0")),}}}
#[inline]
pub(super) unsafe fn f_10fc3648(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fc3648=>{
// 0x10fc3648 stp x29, x30, [sp, #-0x50]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffb0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fc364c stp x26, x25, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[26];let v1=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc3650 stp x24, x23, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc3654 stp x22, x21, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc3658 stp x20, x19, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc365c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fc3660 mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10fc3664 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10fc3668 bl #0x10fc96e8
cpu.x[30]=0x10fc366c;super::chunk15::f_10fc96e8(cpu,rt,0x10fc96e8)?;
pc=0x10fc366c;continue;
}
0x10fc366c=>{
// 0x10fc366c ldr x0, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fc3670 bl #0x10f7e050
cpu.x[30]=0x10fc3674;super::chunk10::f_10f7e050(cpu,rt,0x10f7e050)?;
pc=0x10fc3674;continue;
}
0x10fc3674=>{
// 0x10fc3674 mov x22, x0
cpu.x[22]=(((cpu.x[0]))) as u64;
// 0x10fc3678 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fc367c bl #0x10fd049c
cpu.x[30]=0x10fc3680;super::chunk6::f_10fd049c(cpu,rt,0x10fd049c)?;
pc=0x10fc3680;continue;
}
0x10fc3680=>{
// 0x10fc3680 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3684 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10fc3688 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc368c ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3690 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc3694;super::dispatch(cpu,rt,target)?;}
pc=0x10fc3694;continue;
}
0x10fc3694=>{
// 0x10fc3694 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3698 and w9, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[9]=(result) as u32 as u64;}
// 0x10fc369c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc36a0 strb w9, [x20, #0x19]
{let address=(cpu.x[20]).wrapping_add(0x19u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10fc36a4 ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc36a8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc36ac;super::dispatch(cpu,rt,target)?;}
pc=0x10fc36ac;continue;
}
0x10fc36ac=>{
// 0x10fc36ac ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc36b0 and w9, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[9]=(result) as u32 as u64;}
// 0x10fc36b4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc36b8 strb w9, [x20, #0x18]
{let address=(cpu.x[20]).wrapping_add(0x18u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10fc36bc ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc36c0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc36c4;super::dispatch(cpu,rt,target)?;}
pc=0x10fc36c4;continue;
}
0x10fc36c4=>{
// 0x10fc36c4 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc36c8 and w9, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[9]=(result) as u32 as u64;}
// 0x10fc36cc mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc36d0 strb w9, [x20, #0x3f]
{let address=(cpu.x[20]).wrapping_add(0x3fu64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10fc36d4 ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc36d8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc36dc;super::dispatch(cpu,rt,target)?;}
pc=0x10fc36dc;continue;
}
0x10fc36dc=>{
// 0x10fc36dc ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc36e0 and w9, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[9]=(result) as u32 as u64;}
// 0x10fc36e4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc36e8 strb w9, [x20, #0x3e]
{let address=(cpu.x[20]).wrapping_add(0x3eu64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10fc36ec ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc36f0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc36f4;super::dispatch(cpu,rt,target)?;}
pc=0x10fc36f4;continue;
}
0x10fc36f4=>{
// 0x10fc36f4 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc36f8 and w9, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[9]=(result) as u32 as u64;}
// 0x10fc36fc mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc3700 strb w9, [x20, #0x3d]
{let address=(cpu.x[20]).wrapping_add(0x3du64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10fc3704 ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3708 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc370c;super::dispatch(cpu,rt,target)?;}
pc=0x10fc370c;continue;
}
0x10fc370c=>{
// 0x10fc370c ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3710 mov w23, w0
cpu.x[23]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fc3714 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc3718 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc371c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc3720;super::dispatch(cpu,rt,target)?;}
pc=0x10fc3720;continue;
}
0x10fc3720=>{
// 0x10fc3720 str w0, [x20, #0x70]
{let address=(cpu.x[20]).wrapping_add(0x70u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc3724 add x1, x20, #0x68
{let a=((cpu.x[20]));let b=((0x68u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x10fc3728 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc372c bl #0x111dd2e0
cpu.x[30]=0x10fc3730;super::chunk9::f_111dd2e0(cpu,rt,0x111dd2e0)?;
pc=0x10fc3730;continue;
}
0x10fc3730=>{
// 0x10fc3730 ldr w8, [x22, #0x3d4]
{let address=(cpu.x[22]).wrapping_add(0x3d4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc3734 cmp w8, #2
{let a=((cpu.x[8] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc3738 b.lt #0x10fc3750
if cpu.condition(11){pc=0x10fc3750;continue;}pc=0x10fc373c;continue;
}
0x10fc373c=>{
// 0x10fc373c ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3740 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc3744 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3748 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc374c;super::dispatch(cpu,rt,target)?;}
pc=0x10fc374c;continue;
}
0x10fc374c=>{
// 0x10fc374c str w0, [x20, #0x20]
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10fc3750;continue;
}
0x10fc3750=>{
// 0x10fc3750 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3754 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc3758 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc375c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc3760;super::dispatch(cpu,rt,target)?;}
pc=0x10fc3760;continue;
}
0x10fc3760=>{
// 0x10fc3760 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3764 str w0, [x20, #0x24]
{let address=(cpu.x[20]).wrapping_add(0x24u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc3768 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc376c ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3770 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc3774;super::dispatch(cpu,rt,target)?;}
pc=0x10fc3774;continue;
}
0x10fc3774=>{
// 0x10fc3774 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3778 str w0, [x20, #0x28]
{let address=(cpu.x[20]).wrapping_add(0x28u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc377c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc3780 ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3784 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc3788;super::dispatch(cpu,rt,target)?;}
pc=0x10fc3788;continue;
}
0x10fc3788=>{
// 0x10fc3788 and w8, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fc378c mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10fc3790 strb w8, [x20, #0x30]
{let address=(cpu.x[20]).wrapping_add(0x30u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10fc3794 bl #0x10de108c
cpu.x[30]=0x10fc3798;super::chunk11::f_10de108c(cpu,rt,0x10de108c)?;
pc=0x10fc3798;continue;
}
0x10fc3798=>{
// 0x10fc3798 tbnz w0, #0, #0x10fc3af4
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10fc3af4;continue;}pc=0x10fc379c;continue;
}
0x10fc379c=>{
// 0x10fc379c mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10fc37a0 bl #0x10de05ec
cpu.x[30]=0x10fc37a4;super::chunk5::f_10de05ec(cpu,rt,0x10de05ec)?;
pc=0x10fc37a4;continue;
}
0x10fc37a4=>{
// 0x10fc37a4 tbnz w0, #0, #0x10fc3b14
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10fc3b14;continue;}pc=0x10fc37a8;continue;
}
0x10fc37a8=>{
// 0x10fc37a8 ldr w8, [x22, #0x738]
{let address=(cpu.x[22]).wrapping_add(0x738u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc37ac cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc37b0 b.lt #0x10fc37d4
if cpu.condition(11){pc=0x10fc37d4;continue;}pc=0x10fc37b4;continue;
}
0x10fc37b4=>{
// 0x10fc37b4 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc37b8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc37bc ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc37c0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc37c4;super::dispatch(cpu,rt,target)?;}
// 0x10fc37c4 str w0, [x20, #0x78]
{let address=(cpu.x[20]).wrapping_add(0x78u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc37c8 add x1, x20, #0x7c
{let a=((cpu.x[20]));let b=((0x7cu64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x10fc37cc mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc37d0 bl #0x111dd2e0
cpu.x[30]=0x10fc37d4;super::chunk9::f_111dd2e0(cpu,rt,0x111dd2e0)?;
pc=0x10fc37d4;continue;
}
0x10fc37d4=>{
// 0x10fc37d4 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10fc37d8 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10fc37dc bl #0x10ddde2c
cpu.x[30]=0x10fc37e0;super::chunk11::f_10ddde2c(cpu,rt,0x10ddde2c)?;
pc=0x10fc37e0;continue;
}
0x10fc37e0=>{
// 0x10fc37e0 cmp w0, #0
{let a=((cpu.x[0] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc37e4 b.gt #0x10fc37fc
if cpu.condition(12){pc=0x10fc37fc;continue;}pc=0x10fc37e8;continue;
}
0x10fc37e8=>{
// 0x10fc37e8 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10fc37ec mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10fc37f0 bl #0x10ddde2c
cpu.x[30]=0x10fc37f4;super::chunk11::f_10ddde2c(cpu,rt,0x10ddde2c)?;
pc=0x10fc37f4;continue;
}
0x10fc37f4=>{
// 0x10fc37f4 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc37f8 b.lt #0x10fc3810
if cpu.condition(11){pc=0x10fc3810;continue;}pc=0x10fc37fc;continue;
}
0x10fc37fc=>{
// 0x10fc37fc ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3800 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc3804 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3808 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc380c;super::dispatch(cpu,rt,target)?;}
// 0x10fc380c str w0, [x20, #0x54]
{let address=(cpu.x[20]).wrapping_add(0x54u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10fc3810;continue;
}
0x10fc3810=>{
// 0x10fc3810 ldr x0, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fc3814 bl #0x10f7e050
cpu.x[30]=0x10fc3818;super::chunk10::f_10f7e050(cpu,rt,0x10f7e050)?;
pc=0x10fc3818;continue;
}
0x10fc3818=>{
// 0x10fc3818 mov x24, x0
cpu.x[24]=(((cpu.x[0]))) as u64;
// 0x10fc381c bl #0x10de065c
cpu.x[30]=0x10fc3820;super::chunk4::f_10de065c(cpu,rt,0x10de065c)?;
pc=0x10fc3820;continue;
}
0x10fc3820=>{
// 0x10fc3820 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc3824 b.gt #0x10fc3840
if cpu.condition(12){pc=0x10fc3840;continue;}pc=0x10fc3828;continue;
}
0x10fc3828=>{
// 0x10fc3828 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10fc382c bl #0x10de1250
cpu.x[30]=0x10fc3830;super::chunk9::f_10de1250(cpu,rt,0x10de1250)?;
pc=0x10fc3830;continue;
}
0x10fc3830=>{
// 0x10fc3830 tbnz w0, #0, #0x10fc3840
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10fc3840;continue;}pc=0x10fc3834;continue;
}
0x10fc3834=>{
// 0x10fc3834 ldr w8, [x24, #0x7a4]
{let address=(cpu.x[24]).wrapping_add(0x7a4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc3838 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc383c b.lt #0x10fc3854
if cpu.condition(11){pc=0x10fc3854;continue;}pc=0x10fc3840;continue;
}
0x10fc3840=>{
// 0x10fc3840 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3844 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc3848 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc384c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc3850;super::dispatch(cpu,rt,target)?;}
// 0x10fc3850 str w0, [x20, #0x4c]
{let address=(cpu.x[20]).wrapping_add(0x4cu64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10fc3854;continue;
}
0x10fc3854=>{
// 0x10fc3854 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3858 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc385c ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3860 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc3864;super::dispatch(cpu,rt,target)?;}
pc=0x10fc3864;continue;
}
0x10fc3864=>{
// 0x10fc3864 str w0, [x20, #0x50]
{let address=(cpu.x[20]).wrapping_add(0x50u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc3868 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10fc386c mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10fc3870 bl #0x10de02a0
cpu.x[30]=0x10fc3874;super::chunk7::f_10de02a0(cpu,rt,0x10de02a0)?;
pc=0x10fc3874;continue;
}
0x10fc3874=>{
// 0x10fc3874 cbz x0, #0x10fc38a8
if (cpu.x[0]) == 0 {pc=0x10fc38a8;continue;}pc=0x10fc3878;continue;
}
0x10fc3878=>{
// 0x10fc3878 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10fc387c mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10fc3880 bl #0x10de02a0
cpu.x[30]=0x10fc3884;super::chunk7::f_10de02a0(cpu,rt,0x10de02a0)?;
pc=0x10fc3884;continue;
}
0x10fc3884=>{
// 0x10fc3884 ldr w8, [x0, #0x1c0]
{let address=(cpu.x[0]).wrapping_add(0x1c0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc3888 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc388c b.gt #0x10fc38e8
if cpu.condition(12){pc=0x10fc38e8;continue;}pc=0x10fc3890;continue;
}
0x10fc3890=>{
// 0x10fc3890 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10fc3894 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10fc3898 bl #0x10de02a0
cpu.x[30]=0x10fc389c;super::chunk7::f_10de02a0(cpu,rt,0x10de02a0)?;
pc=0x10fc389c;continue;
}
0x10fc389c=>{
// 0x10fc389c ldr w8, [x0, #0x1e8]
{let address=(cpu.x[0]).wrapping_add(0x1e8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc38a0 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc38a4 b.gt #0x10fc38e8
if cpu.condition(12){pc=0x10fc38e8;continue;}pc=0x10fc38a8;continue;
}
0x10fc38a8=>{
// 0x10fc38a8 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10fc38ac mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10fc38b0 bl #0x10de02a0
cpu.x[30]=0x10fc38b4;super::chunk7::f_10de02a0(cpu,rt,0x10de02a0)?;
pc=0x10fc38b4;continue;
}
0x10fc38b4=>{
// 0x10fc38b4 cbz x0, #0x10fc38fc
if (cpu.x[0]) == 0 {pc=0x10fc38fc;continue;}pc=0x10fc38b8;continue;
}
0x10fc38b8=>{
// 0x10fc38b8 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10fc38bc mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10fc38c0 bl #0x10de02a0
cpu.x[30]=0x10fc38c4;super::chunk7::f_10de02a0(cpu,rt,0x10de02a0)?;
pc=0x10fc38c4;continue;
}
0x10fc38c4=>{
// 0x10fc38c4 ldr w8, [x0, #0x1c0]
{let address=(cpu.x[0]).wrapping_add(0x1c0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc38c8 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc38cc b.gt #0x10fc38e8
if cpu.condition(12){pc=0x10fc38e8;continue;}pc=0x10fc38d0;continue;
}
0x10fc38d0=>{
// 0x10fc38d0 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10fc38d4 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10fc38d8 bl #0x10de02a0
cpu.x[30]=0x10fc38dc;super::chunk7::f_10de02a0(cpu,rt,0x10de02a0)?;
pc=0x10fc38dc;continue;
}
0x10fc38dc=>{
// 0x10fc38dc ldr w8, [x0, #0x1e8]
{let address=(cpu.x[0]).wrapping_add(0x1e8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc38e0 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc38e4 b.lt #0x10fc38fc
if cpu.condition(11){pc=0x10fc38fc;continue;}pc=0x10fc38e8;continue;
}
0x10fc38e8=>{
// 0x10fc38e8 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fc38ec mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10fc38f0 mov w2, #4
cpu.x[2]=(((0x4u64))) as u32 as u64;
// 0x10fc38f4 bl #0x10f8bfe0
cpu.x[30]=0x10fc38f8;super::chunk2::f_10f8bfe0(cpu,rt,0x10f8bfe0)?;
// 0x10fc38f8 str x0, [x20, #0x58]
{let address=(cpu.x[20]).wrapping_add(0x58u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10fc38fc;continue;
}
0x10fc38fc=>{
// 0x10fc38fc ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3900 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc3904 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3908 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc390c;super::dispatch(cpu,rt,target)?;}
pc=0x10fc390c;continue;
}
0x10fc390c=>{
// 0x10fc390c str w0, [x20, #0x1c]
{let address=(cpu.x[20]).wrapping_add(0x1cu64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc3910 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fc3914 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10fc3918 bl #0x10f992f0
cpu.x[30]=0x10fc391c;super::chunk11::f_10f992f0(cpu,rt,0x10f992f0)?;
pc=0x10fc391c;continue;
}
0x10fc391c=>{
// 0x10fc391c ldr w8, [x22, #0x69c]
{let address=(cpu.x[22]).wrapping_add(0x69cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc3920 str x0, [x20, #0x10]
{let address=(cpu.x[20]).wrapping_add(0x10u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fc3924 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc3928 b.lt #0x10fc3940
if cpu.condition(11){pc=0x10fc3940;continue;}pc=0x10fc392c;continue;
}
0x10fc392c=>{
// 0x10fc392c ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3930 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc3934 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3938 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc393c;super::dispatch(cpu,rt,target)?;}
// 0x10fc393c str w0, [x20, #0x74]
{let address=(cpu.x[20]).wrapping_add(0x74u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10fc3940;continue;
}
0x10fc3940=>{
// 0x10fc3940 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10fc3944 bl #0x10de0384
cpu.x[30]=0x10fc3948;super::chunk4::f_10de0384(cpu,rt,0x10de0384)?;
pc=0x10fc3948;continue;
}
0x10fc3948=>{
// 0x10fc3948 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc394c b.lt #0x10fc397c
if cpu.condition(11){pc=0x10fc397c;continue;}pc=0x10fc3950;continue;
}
0x10fc3950=>{
// 0x10fc3950 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3954 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc3958 ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc395c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc3960;super::dispatch(cpu,rt,target)?;}
// 0x10fc3960 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3964 and w9, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[9]=(result) as u32 as u64;}
// 0x10fc3968 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc396c strb w9, [x20, #0x41]
{let address=(cpu.x[20]).wrapping_add(0x41u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10fc3970 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3974 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc3978;super::dispatch(cpu,rt,target)?;}
// 0x10fc3978 str w0, [x20, #0x44]
{let address=(cpu.x[20]).wrapping_add(0x44u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10fc397c;continue;
}
0x10fc397c=>{
// 0x10fc397c mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10fc3980 mov x1, xzr
cpu.x[1]=(((0u64))) as u64;
// 0x10fc3984 bl #0x10de0b2c
cpu.x[30]=0x10fc3988;super::chunk13::f_10de0b2c(cpu,rt,0x10de0b2c)?;
pc=0x10fc3988;continue;
}
0x10fc3988=>{
// 0x10fc3988 cmp w0, #0
{let a=((cpu.x[0] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc398c b.gt #0x10fc39ac
if cpu.condition(12){pc=0x10fc39ac;continue;}pc=0x10fc3990;continue;
}
0x10fc3990=>{
// 0x10fc3990 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10fc3994 bl #0x10de0384
cpu.x[30]=0x10fc3998;super::chunk4::f_10de0384(cpu,rt,0x10de0384)?;
pc=0x10fc3998;continue;
}
0x10fc3998=>{
// 0x10fc3998 cmp w0, #0
{let a=((cpu.x[0] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc399c b.gt #0x10fc39ac
if cpu.condition(12){pc=0x10fc39ac;continue;}pc=0x10fc39a0;continue;
}
0x10fc39a0=>{
// 0x10fc39a0 ldr w8, [x22, #0x738]
{let address=(cpu.x[22]).wrapping_add(0x738u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc39a4 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc39a8 b.lt #0x10fc3a2c
if cpu.condition(11){pc=0x10fc3a2c;continue;}pc=0x10fc39ac;continue;
}
0x10fc39ac=>{
// 0x10fc39ac ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc39b0 str wzr, [x20, #0x94]
{let address=(cpu.x[20]).wrapping_add(0x94u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc39b4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc39b8 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc39bc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc39c0;super::dispatch(cpu,rt,target)?;}
// 0x10fc39c0 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc39c4 b.lt #0x10fc3a2c
if cpu.condition(11){pc=0x10fc3a2c;continue;}pc=0x10fc39c8;continue;
}
0x10fc39c8=>{
// 0x10fc39c8 mov w24, w0
cpu.x[24]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fc39cc mov w25, #5
cpu.x[25]=(((0x5u64))) as u32 as u64;
// 0x10fc39d0 b #0x10fc39ec
pc=0x10fc39ec;continue;
}
0x10fc39d4=>{
// 0x10fc39d4 ldr x9, [x20, #0x88]
{let address=(cpu.x[20]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fc39d8 add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10fc39dc subs w24, w24, #1
{let a=((cpu.x[24] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[24]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10fc39e0 str w10, [x20, #0x94]
{let address=(cpu.x[20]).wrapping_add(0x94u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc39e4 str w0, [x9, w8, sxtw #2]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc39e8 b.eq #0x10fc3a2c
if cpu.condition(0){pc=0x10fc3a2c;continue;}pc=0x10fc39ec;continue;
}
0x10fc39ec=>{
// 0x10fc39ec mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc39f0 bl #0x111dd1e0
cpu.x[30]=0x10fc39f4;super::chunk10::f_111dd1e0(cpu,rt,0x111dd1e0)?;
// 0x10fc39f4 ldp w9, w8, [x20, #0x90]
{let address=(cpu.x[20]).wrapping_add(0x90u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10fc39f8 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc39fc b.ne #0x10fc39d4
if cpu.condition(1){pc=0x10fc39d4;continue;}pc=0x10fc3a00;continue;
}
0x10fc3a00=>{
// 0x10fc3a00 lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10fc3a04 cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc3a08 csel w1, w25, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[25] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10fc3a0c cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc3a10 b.ge #0x10fc39d4
if cpu.condition(10){pc=0x10fc39d4;continue;}pc=0x10fc3a14;continue;
}
0x10fc3a14=>{
// 0x10fc3a14 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fc3a18 add x0, x20, #0x88
{let a=((cpu.x[20]));let b=((0x88u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fc3a1c bl #0x1078f4d4
cpu.x[30]=0x10fc3a20;super::chunk2::f_1078f4d4(cpu,rt,0x1078f4d4)?;
// 0x10fc3a20 ldr w8, [x20, #0x94]
{let address=(cpu.x[20]).wrapping_add(0x94u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc3a24 mov w0, w26
cpu.x[0]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fc3a28 b #0x10fc39d4
pc=0x10fc39d4;continue;
}
0x10fc3a2c=>{
// 0x10fc3a2c mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10fc3a30 bl #0x10de0f80
cpu.x[30]=0x10fc3a34;super::chunk5::f_10de0f80(cpu,rt,0x10de0f80)?;
pc=0x10fc3a34;continue;
}
0x10fc3a34=>{
// 0x10fc3a34 tbz w0, #0, #0x10fc3ab8
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fc3ab8;continue;}pc=0x10fc3a38;continue;
}
0x10fc3a38=>{
// 0x10fc3a38 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3a3c str wzr, [x20, #0xa4]
{let address=(cpu.x[20]).wrapping_add(0xa4u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc3a40 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc3a44 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3a48 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc3a4c;super::dispatch(cpu,rt,target)?;}
// 0x10fc3a4c cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc3a50 b.lt #0x10fc3ab8
if cpu.condition(11){pc=0x10fc3ab8;continue;}pc=0x10fc3a54;continue;
}
0x10fc3a54=>{
// 0x10fc3a54 mov w22, w0
cpu.x[22]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fc3a58 mov w24, #5
cpu.x[24]=(((0x5u64))) as u32 as u64;
// 0x10fc3a5c b #0x10fc3a78
pc=0x10fc3a78;continue;
}
0x10fc3a60=>{
// 0x10fc3a60 ldr x9, [x20, #0x98]
{let address=(cpu.x[20]).wrapping_add(0x98u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fc3a64 add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10fc3a68 subs w22, w22, #1
{let a=((cpu.x[22] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[22]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10fc3a6c str w10, [x20, #0xa4]
{let address=(cpu.x[20]).wrapping_add(0xa4u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc3a70 str w0, [x9, w8, sxtw #2]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc3a74 b.eq #0x10fc3ab8
if cpu.condition(0){pc=0x10fc3ab8;continue;}pc=0x10fc3a78;continue;
}
0x10fc3a78=>{
// 0x10fc3a78 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc3a7c bl #0x111dd1e0
cpu.x[30]=0x10fc3a80;super::chunk10::f_111dd1e0(cpu,rt,0x111dd1e0)?;
// 0x10fc3a80 ldp w9, w8, [x20, #0xa0]
{let address=(cpu.x[20]).wrapping_add(0xa0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10fc3a84 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc3a88 b.ne #0x10fc3a60
if cpu.condition(1){pc=0x10fc3a60;continue;}pc=0x10fc3a8c;continue;
}
0x10fc3a8c=>{
// 0x10fc3a8c lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10fc3a90 cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc3a94 csel w1, w24, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[24] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10fc3a98 cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc3a9c b.ge #0x10fc3a60
if cpu.condition(10){pc=0x10fc3a60;continue;}pc=0x10fc3aa0;continue;
}
0x10fc3aa0=>{
// 0x10fc3aa0 mov w25, w0
cpu.x[25]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fc3aa4 add x0, x20, #0x98
{let a=((cpu.x[20]));let b=((0x98u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fc3aa8 bl #0x1078f4d4
cpu.x[30]=0x10fc3aac;super::chunk2::f_1078f4d4(cpu,rt,0x1078f4d4)?;
// 0x10fc3aac ldr w8, [x20, #0xa4]
{let address=(cpu.x[20]).wrapping_add(0xa4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc3ab0 mov w0, w25
cpu.x[0]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fc3ab4 b #0x10fc3a60
pc=0x10fc3a60;continue;
}
0x10fc3ab8=>{
// 0x10fc3ab8 tbz w23, #0, #0x10fc3b38
if ((cpu.x[23] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fc3b38;continue;}pc=0x10fc3abc;continue;
}
0x10fc3abc=>{
// 0x10fc3abc mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fc3ac0 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10fc3ac4 bl #0x10f8c0c4
cpu.x[30]=0x10fc3ac8;super::chunk6::f_10f8c0c4(cpu,rt,0x10f8c0c4)?;
// 0x10fc3ac8 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3acc str x0, [x20, #0x58]
{let address=(cpu.x[20]).wrapping_add(0x58u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fc3ad0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc3ad4 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3ad8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc3adc;super::dispatch(cpu,rt,target)?;}
// 0x10fc3adc ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3ae0 str w0, [x20, #0x60]
{let address=(cpu.x[20]).wrapping_add(0x60u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc3ae4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc3ae8 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3aec blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc3af0;super::dispatch(cpu,rt,target)?;}
// 0x10fc3af0 b #0x10fc3b44
pc=0x10fc3b44;continue;
}
0x10fc3af4=>{
// 0x10fc3af4 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3af8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc3afc ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3b00 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc3b04;super::dispatch(cpu,rt,target)?;}
// 0x10fc3b04 str w0, [x20, #0x2c]
{let address=(cpu.x[20]).wrapping_add(0x2cu64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc3b08 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10fc3b0c bl #0x10de05ec
cpu.x[30]=0x10fc3b10;super::chunk5::f_10de05ec(cpu,rt,0x10de05ec)?;
// 0x10fc3b10 tbz w0, #0, #0x10fc37a8
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fc37a8;continue;}pc=0x10fc3b14;continue;
}
0x10fc3b14=>{
// 0x10fc3b14 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3b18 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc3b1c ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3b20 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc3b24;super::dispatch(cpu,rt,target)?;}
// 0x10fc3b24 str w0, [x20, #0x48]
{let address=(cpu.x[20]).wrapping_add(0x48u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc3b28 ldr w8, [x22, #0x738]
{let address=(cpu.x[22]).wrapping_add(0x738u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc3b2c cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc3b30 b.ge #0x10fc37b4
if cpu.condition(10){pc=0x10fc37b4;continue;}pc=0x10fc3b34;continue;
}
0x10fc3b34=>{
// 0x10fc3b34 b #0x10fc37d4
pc=0x10fc37d4;continue;
}
0x10fc3b38=>{
// 0x10fc3b38 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10fc3b3c str xzr, [x20, #0x58]
{let address=(cpu.x[20]).wrapping_add(0x58u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fc3b40 str wzr, [x20, #0x60]
{let address=(cpu.x[20]).wrapping_add(0x60u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10fc3b44;continue;
}
0x10fc3b44=>{
// 0x10fc3b44 str w0, [x20, #0x64]
{let address=(cpu.x[20]).wrapping_add(0x64u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc3b48 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fc3b4c mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10fc3b50 bl #0x10f992f0
cpu.x[30]=0x10fc3b54;super::chunk11::f_10f992f0(cpu,rt,0x10f992f0)?;
pc=0x10fc3b54;continue;
}
0x10fc3b54=>{
// 0x10fc3b54 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3b58 str x0, [x20, #0xb0]
{let address=(cpu.x[20]).wrapping_add(0xb0u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fc3b5c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc3b60 ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3b64 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc3b68;super::dispatch(cpu,rt,target)?;}
pc=0x10fc3b68;continue;
}
0x10fc3b68=>{
// 0x10fc3b68 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3b6c and w9, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[9]=(result) as u32 as u64;}
// 0x10fc3b70 str wzr, [x20, #0xc4]
{let address=(cpu.x[20]).wrapping_add(0xc4u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc3b74 strb w9, [x20, #0x40]
{let address=(cpu.x[20]).wrapping_add(0x40u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10fc3b78 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc3b7c ldr x8, [x8, #0xf0]
{let address=(cpu.x[8]).wrapping_add(0xf0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3b80 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc3b84;super::dispatch(cpu,rt,target)?;}
pc=0x10fc3b84;continue;
}
0x10fc3b84=>{
// 0x10fc3b84 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc3b88 b.lt #0x10fc3bf8
if cpu.condition(11){pc=0x10fc3bf8;continue;}pc=0x10fc3b8c;continue;
}
0x10fc3b8c=>{
// 0x10fc3b8c mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fc3b90 mov w22, #5
cpu.x[22]=(((0x5u64))) as u32 as u64;
// 0x10fc3b94 b #0x10fc3bb0
pc=0x10fc3bb0;continue;
}
0x10fc3b98=>{
// 0x10fc3b98 ldr x9, [x20, #0xb8]
{let address=(cpu.x[20]).wrapping_add(0xb8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fc3b9c add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10fc3ba0 subs w21, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[21]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10fc3ba4 str w10, [x20, #0xc4]
{let address=(cpu.x[20]).wrapping_add(0xc4u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc3ba8 str w0, [x9, w8, sxtw #2]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc3bac b.eq #0x10fc3bf8
if cpu.condition(0){pc=0x10fc3bf8;continue;}pc=0x10fc3bb0;continue;
}
0x10fc3bb0=>{
// 0x10fc3bb0 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3bb4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc3bb8 ldr x8, [x8, #0xf0]
{let address=(cpu.x[8]).wrapping_add(0xf0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3bbc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc3bc0;super::dispatch(cpu,rt,target)?;}
// 0x10fc3bc0 ldp w9, w8, [x20, #0xc0]
{let address=(cpu.x[20]).wrapping_add(0xc0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10fc3bc4 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc3bc8 b.ne #0x10fc3b98
if cpu.condition(1){pc=0x10fc3b98;continue;}pc=0x10fc3bcc;continue;
}
0x10fc3bcc=>{
// 0x10fc3bcc lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10fc3bd0 cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc3bd4 csel w1, w22, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[22] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10fc3bd8 cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc3bdc b.ge #0x10fc3b98
if cpu.condition(10){pc=0x10fc3b98;continue;}pc=0x10fc3be0;continue;
}
0x10fc3be0=>{
// 0x10fc3be0 mov w23, w0
cpu.x[23]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fc3be4 add x0, x20, #0xb8
{let a=((cpu.x[20]));let b=((0xb8u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fc3be8 bl #0x1078f4d4
cpu.x[30]=0x10fc3bec;super::chunk2::f_1078f4d4(cpu,rt,0x1078f4d4)?;
// 0x10fc3bec ldr w8, [x20, #0xc4]
{let address=(cpu.x[20]).wrapping_add(0xc4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc3bf0 mov w0, w23
cpu.x[0]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10fc3bf4 b #0x10fc3b98
pc=0x10fc3b98;continue;
}
0x10fc3bf8=>{
// 0x10fc3bf8 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3bfc str wzr, [x20, #0xd4]
{let address=(cpu.x[20]).wrapping_add(0xd4u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10fc3c00;continue;
}
0x10fc3c00=>{
// 0x10fc3c00 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc3c04 ldr x8, [x8, #0xf0]
{let address=(cpu.x[8]).wrapping_add(0xf0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3c08 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc3c0c;super::dispatch(cpu,rt,target)?;}
pc=0x10fc3c0c;continue;
}
0x10fc3c0c=>{
// 0x10fc3c0c cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc3c10 b.lt #0x10fc3c80
if cpu.condition(11){pc=0x10fc3c80;continue;}pc=0x10fc3c14;continue;
}
0x10fc3c14=>{
// 0x10fc3c14 mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fc3c18 mov w22, #5
cpu.x[22]=(((0x5u64))) as u32 as u64;
// 0x10fc3c1c b #0x10fc3c38
pc=0x10fc3c38;continue;
}
0x10fc3c20=>{
// 0x10fc3c20 ldr x9, [x20, #0xc8]
{let address=(cpu.x[20]).wrapping_add(0xc8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fc3c24 add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10fc3c28 subs w21, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[21]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10fc3c2c str w10, [x20, #0xd4]
{let address=(cpu.x[20]).wrapping_add(0xd4u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc3c30 str w0, [x9, w8, sxtw #2]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc3c34 b.eq #0x10fc3c80
if cpu.condition(0){pc=0x10fc3c80;continue;}pc=0x10fc3c38;continue;
}
0x10fc3c38=>{
// 0x10fc3c38 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3c3c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc3c40 ldr x8, [x8, #0xf0]
{let address=(cpu.x[8]).wrapping_add(0xf0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3c44 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc3c48;super::dispatch(cpu,rt,target)?;}
// 0x10fc3c48 ldp w9, w8, [x20, #0xd0]
{let address=(cpu.x[20]).wrapping_add(0xd0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10fc3c4c cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc3c50 b.ne #0x10fc3c20
if cpu.condition(1){pc=0x10fc3c20;continue;}pc=0x10fc3c54;continue;
}
0x10fc3c54=>{
// 0x10fc3c54 lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10fc3c58 cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc3c5c csel w1, w22, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[22] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10fc3c60 cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc3c64 b.ge #0x10fc3c20
if cpu.condition(10){pc=0x10fc3c20;continue;}pc=0x10fc3c68;continue;
}
0x10fc3c68=>{
// 0x10fc3c68 mov w23, w0
cpu.x[23]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fc3c6c add x0, x20, #0xc8
{let a=((cpu.x[20]));let b=((0xc8u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fc3c70 bl #0x1078f4d4
cpu.x[30]=0x10fc3c74;super::chunk2::f_1078f4d4(cpu,rt,0x1078f4d4)?;
// 0x10fc3c74 ldr w8, [x20, #0xd4]
{let address=(cpu.x[20]).wrapping_add(0xd4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc3c78 mov w0, w23
cpu.x[0]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10fc3c7c b #0x10fc3c20
pc=0x10fc3c20;continue;
}
0x10fc3c80=>{
// 0x10fc3c80 ldp x20, x19, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fc3c84 ldp x22, x21, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fc3c88 ldp x24, x23, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fc3c8c ldp x26, x25, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fc3c90 ldp x29, x30, [sp], #0x50
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x50u64)))) as u64;}
// 0x10fc3c94 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fc3648")),}}}
#[inline]
pub(super) unsafe fn f_10fc4afc(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fc4afc=>{
// 0x10fc4afc stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fc4b00 stp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc4b04 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc4b08 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fc4b0c mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10fc4b10 ldr x0, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fc4b14 cbz x0, #0x10fc4b44
if (cpu.x[0]) == 0 {pc=0x10fc4b44;continue;}pc=0x10fc4b18;continue;
}
0x10fc4b18=>{
// 0x10fc4b18 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc4b1c ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc4b20 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc4b24;super::dispatch(cpu,rt,target)?;}
pc=0x10fc4b24;continue;
}
0x10fc4b24=>{
// 0x10fc4b24 cmp w0, #3
{let a=((cpu.x[0] as u32 as u64));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc4b28 b.ne #0x10fc4b44
if cpu.condition(1){pc=0x10fc4b44;continue;}pc=0x10fc4b2c;continue;
}
0x10fc4b2c=>{
// 0x10fc4b2c mov w19, #0xfa
cpu.x[19]=(((0xfau64))) as u32 as u64;
pc=0x10fc4b30;continue;
}
0x10fc4b30=>{
// 0x10fc4b30 mov w0, w19
cpu.x[0]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10fc4b34 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fc4b38 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fc4b3c ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10fc4b40 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10fc4b44=>{
// 0x10fc4b44 ldr x21, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10fc4b48 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fc4b4c bl #0x10f7e050
cpu.x[30]=0x10fc4b50;super::chunk10::f_10f7e050(cpu,rt,0x10f7e050)?;
pc=0x10fc4b50;continue;
}
0x10fc4b50=>{
// 0x10fc4b50 ldrb w8, [x20, #0x41]
{let address=(cpu.x[20]).wrapping_add(0x41u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc4b54 mov x22, x0
cpu.x[22]=(((cpu.x[0]))) as u64;
// 0x10fc4b58 cbz w8, #0x10fc4c04
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10fc4c04;continue;}pc=0x10fc4b5c;continue;
}
0x10fc4b5c=>{
// 0x10fc4b5c mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10fc4b60 bl #0x10de0384
cpu.x[30]=0x10fc4b64;super::chunk4::f_10de0384(cpu,rt,0x10de0384)?;
// 0x10fc4b64 mov w19, w0
cpu.x[19]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
pc=0x10fc4b68;continue;
}
0x10fc4b68=>{
// 0x10fc4b68 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fc4b6c bl #0x10fd04a8
cpu.x[30]=0x10fc4b70;super::chunk2::f_10fd04a8(cpu,rt,0x10fd04a8)?;
pc=0x10fc4b70;continue;
}
0x10fc4b70=>{
// 0x10fc4b70 bl #0x10e0b6b4
cpu.x[30]=0x10fc4b74;super::chunk8::f_10e0b6b4(cpu,rt,0x10e0b6b4)?;
pc=0x10fc4b74;continue;
}
0x10fc4b74=>{
// 0x10fc4b74 ldr x0, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fc4b78 bl #0x10e16208
cpu.x[30]=0x10fc4b7c;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
pc=0x10fc4b7c;continue;
}
0x10fc4b7c=>{
// 0x10fc4b7c ldrb w8, [x0, #0x360]
{let address=(cpu.x[0]).wrapping_add(0x360u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc4b80 cbz w8, #0x10fc4b90
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10fc4b90;continue;}pc=0x10fc4b84;continue;
}
0x10fc4b84=>{
// 0x10fc4b84 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10fc4b88 bl #0x10de0430
cpu.x[30]=0x10fc4b8c;super::chunk11::f_10de0430(cpu,rt,0x10de0430)?;
pc=0x10fc4b8c;continue;
}
0x10fc4b8c=>{
// 0x10fc4b8c add w19, w0, w19
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_add(b);cpu.x[19]=(result) as u32 as u64;}
pc=0x10fc4b90;continue;
}
0x10fc4b90=>{
// 0x10fc4b90 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fc4b94 bl #0x10fd04a8
cpu.x[30]=0x10fc4b98;super::chunk2::f_10fd04a8(cpu,rt,0x10fd04a8)?;
pc=0x10fc4b98;continue;
}
0x10fc4b98=>{
// 0x10fc4b98 bl #0x10e0b6b4
cpu.x[30]=0x10fc4b9c;super::chunk8::f_10e0b6b4(cpu,rt,0x10e0b6b4)?;
pc=0x10fc4b9c;continue;
}
0x10fc4b9c=>{
// 0x10fc4b9c ldr x0, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fc4ba0 bl #0x10e16208
cpu.x[30]=0x10fc4ba4;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
pc=0x10fc4ba4;continue;
}
0x10fc4ba4=>{
// 0x10fc4ba4 ldr w8, [x0, #0x410]
{let address=(cpu.x[0]).wrapping_add(0x410u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc4ba8 cbz w8, #0x10fc4b30
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10fc4b30;continue;}pc=0x10fc4bac;continue;
}
0x10fc4bac=>{
// 0x10fc4bac mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fc4bb0 bl #0x10f942c8
cpu.x[30]=0x10fc4bb4;super::chunk1::f_10f942c8(cpu,rt,0x10f942c8)?;
pc=0x10fc4bb4;continue;
}
0x10fc4bb4=>{
// 0x10fc4bb4 cbz x0, #0x10fc4b30
if (cpu.x[0]) == 0 {pc=0x10fc4b30;continue;}pc=0x10fc4bb8;continue;
}
0x10fc4bb8=>{
// 0x10fc4bb8 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10fc4bbc bl #0x10de0be8
cpu.x[30]=0x10fc4bc0;super::chunk6::f_10de0be8(cpu,rt,0x10de0be8)?;
pc=0x10fc4bc0;continue;
}
0x10fc4bc0=>{
// 0x10fc4bc0 tbz w0, #0, #0x10fc4b30
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fc4b30;continue;}pc=0x10fc4bc4;continue;
}
0x10fc4bc4=>{
// 0x10fc4bc4 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fc4bc8 bl #0x10f817a8
cpu.x[30]=0x10fc4bcc;super::chunk8::f_10f817a8(cpu,rt,0x10f817a8)?;
// 0x10fc4bcc cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc4bd0 b.ne #0x10fc4b30
if cpu.condition(1){pc=0x10fc4b30;continue;}pc=0x10fc4bd4;continue;
}
0x10fc4bd4=>{
// 0x10fc4bd4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fc4bd8 bl #0x10fd04a8
cpu.x[30]=0x10fc4bdc;super::chunk2::f_10fd04a8(cpu,rt,0x10fd04a8)?;
// 0x10fc4bdc bl #0x10e0b6b4
cpu.x[30]=0x10fc4be0;super::chunk8::f_10e0b6b4(cpu,rt,0x10e0b6b4)?;
// 0x10fc4be0 ldr x0, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fc4be4 bl #0x10e16208
cpu.x[30]=0x10fc4be8;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
// 0x10fc4be8 ldr w8, [x0, #0x410]
{let address=(cpu.x[0]).wrapping_add(0x410u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc4bec sub w19, w19, w8
{let a=((cpu.x[19] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[19]=(result) as u32 as u64;}
// 0x10fc4bf0 mov w0, w19
cpu.x[0]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10fc4bf4 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fc4bf8 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fc4bfc ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10fc4c00 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10fc4c04=>{
// 0x10fc4c04 ldr w1, [x20, #0x20]
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10fc4c08 ldr w19, [x22, #0x420]
{let address=(cpu.x[22]).wrapping_add(0x420u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[19]=(v0) as u32 as u64;}
// 0x10fc4c0c cmn w1, #1
{let a=((cpu.x[1] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.flags(a,b,result,32,false);}
// 0x10fc4c10 b.eq #0x10fc4b68
if cpu.condition(0){pc=0x10fc4b68;continue;}pc=0x10fc4c14;continue;
}
0x10fc4c14=>{
// 0x10fc4c14 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10fc4c18 bl #0x10de0eec
cpu.x[30]=0x10fc4c1c;super::chunk13::f_10de0eec(cpu,rt,0x10de0eec)?;
pc=0x10fc4c1c;continue;
}
0x10fc4c1c=>{
// 0x10fc4c1c ldr x8, [x22, #0x2a8]
{let address=(cpu.x[22]).wrapping_add(0x2a8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc4c20 ldr x8, [x8, w0, sxtw #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[0] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc4c24 cbz x8, #0x10fc4b68
if (cpu.x[8]) == 0 {pc=0x10fc4b68;continue;}pc=0x10fc4c28;continue;
}
0x10fc4c28=>{
// 0x10fc4c28 ldr w8, [x8, #0xf8]
{let address=(cpu.x[8]).wrapping_add(0xf8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc4c2c cmn w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.flags(a,b,result,32,false);}
// 0x10fc4c30 csel w19, w19, w8, eq
cpu.x[19]=(if cpu.condition(0){((cpu.x[19] as u32 as u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10fc4c34 b #0x10fc4b68
pc=0x10fc4b68;continue;
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fc4afc")),}}}
#[inline]
pub(super) unsafe fn f_10fca24c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fca24c=>{
// 0x10fca24c stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fca250 stp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fca254 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fca258 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fca25c ldr x21, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10fca260 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10fca264 mov w19, w2
cpu.x[19]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x10fca268 mov x22, x1
cpu.x[22]=(((cpu.x[1]))) as u64;
// 0x10fca26c mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fca270 bl #0x10f93f94
cpu.x[30]=0x10fca274;super::chunk3::f_10f93f94(cpu,rt,0x10f93f94)?;
pc=0x10fca274;continue;
}
0x10fca274=>{
// 0x10fca274 bl #0x10f9922c
cpu.x[30]=0x10fca278;super::chunk0::f_10f9922c(cpu,rt,0x10f9922c)?;
pc=0x10fca278;continue;
}
0x10fca278=>{
// 0x10fca278 ldr w2, [x20, #0x10]
{let address=(cpu.x[20]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[2]=(v0) as u32 as u64;}
// 0x10fca27c mov x1, x21
cpu.x[1]=(((cpu.x[21]))) as u64;
// 0x10fca280 mov x3, xzr
cpu.x[3]=(((0u64))) as u64;
// 0x10fca284 bl #0x10fd0738
cpu.x[30]=0x10fca288;super::chunk14::f_10fd0738(cpu,rt,0x10fd0738)?;
pc=0x10fca288;continue;
}
0x10fca288=>{
// 0x10fca288 mov x1, x22
cpu.x[1]=(((cpu.x[22]))) as u64;
// 0x10fca28c mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10fca290 bl #0x10fd11f4
cpu.x[30]=0x10fca294;super::chunk9::f_10fd11f4(cpu,rt,0x10fd11f4)?;
pc=0x10fca294;continue;
}
0x10fca294=>{
// 0x10fca294 ldr w8, [x20, #0x20]
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fca298 str w19, [x20, #0xc]
{let address=(cpu.x[20]).wrapping_add(0xcu64);let v0=cpu.x[19] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fca29c orr w8, w8, #8
{let a=((cpu.x[8] as u32 as u64));let b=((0x8u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fca2a0 str w8, [x20, #0x20]
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fca2a4 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fca2a8 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fca2ac ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10fca2b0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fca24c")),}}}
#[inline]
pub(super) unsafe fn f_10fcc030(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fcc030=>{
// 0x10fcc030 ldp w8, w9, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[8]=(v0) as u32 as u64;cpu.x[9]=(v1) as u32 as u64;}
// 0x10fcc034 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcc038 cset w0, ge
cpu.x[0]=((cpu.condition(10)) as u64) as u32 as u64;
// 0x10fcc03c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fcc030")),}}}
#[inline]
pub(super) unsafe fn f_10fcc054(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fcc054=>{
// 0x10fcc054 ldr w8, [x0, #0x2c]
{let address=(cpu.x[0]).wrapping_add(0x2cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcc058 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcc05c b.le #0x10fcc068
if cpu.condition(13){pc=0x10fcc068;continue;}pc=0x10fcc060;continue;
}
0x10fcc060=>{
// 0x10fcc060 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10fcc064 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10fcc068=>{
// 0x10fcc068 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fcc06c str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fcc070 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fcc074 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fcc078 ldr x20, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10fcc07c mov w19, w1
cpu.x[19]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x10fcc080 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10fcc084 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10fcc088 mov w2, wzr
cpu.x[2]=(((0u64))) as u32 as u64;
// 0x10fcc08c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcc090 bl #0x10f817b0
cpu.x[30]=0x10fcc094;super::chunk12::f_10f817b0(cpu,rt,0x10f817b0)?;
pc=0x10fcc094;continue;
}
0x10fcc094=>{
// 0x10fcc094 mov w8, w0
cpu.x[8]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fcc098 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10fcc09c tbnz w8, #0, #0x10fcc0ec
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10fcc0ec;continue;}pc=0x10fcc0a0;continue;
}
0x10fcc0a0=>{
// 0x10fcc0a0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcc0a4 bl #0x10f940fc
cpu.x[30]=0x10fcc0a8;super::chunk9::f_10f940fc(cpu,rt,0x10f940fc)?;
pc=0x10fcc0a8;continue;
}
0x10fcc0a8=>{
// 0x10fcc0a8 cbz x0, #0x10fcc0e0
if (cpu.x[0]) == 0 {pc=0x10fcc0e0;continue;}pc=0x10fcc0ac;continue;
}
0x10fcc0ac=>{
// 0x10fcc0ac mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcc0b0 bl #0x10f940fc
cpu.x[30]=0x10fcc0b4;super::chunk9::f_10f940fc(cpu,rt,0x10f940fc)?;
pc=0x10fcc0b4;continue;
}
0x10fcc0b4=>{
// 0x10fcc0b4 mov w1, w19
cpu.x[1]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10fcc0b8 mov x2, xzr
cpu.x[2]=(((0u64))) as u64;
// 0x10fcc0bc bl #0x10fc1e84
cpu.x[30]=0x10fcc0c0;super::chunk4::f_10fc1e84(cpu,rt,0x10fc1e84)?;
pc=0x10fcc0c0;continue;
}
0x10fcc0c0=>{
// 0x10fcc0c0 mov w19, w0
cpu.x[19]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fcc0c4 tbz w0, #0x1f, #0x10fcc0d8
if ((cpu.x[0] as u32 as u64) & (1u64 << 31)) == 0 {pc=0x10fcc0d8;continue;}pc=0x10fcc0c8;continue;
}
0x10fcc0c8=>{
// 0x10fcc0c8 adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x10fcc0cc add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fcc0d0 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10fcc0d4 bl #0x1128df18
cpu.x[30]=0x10fcc0d8;super::dispatch(cpu,rt,0x1128df18)?;
pc=0x10fcc0d8;continue;
}
0x10fcc0d8=>{
// 0x10fcc0d8 cmp w19, #1
{let a=((cpu.x[19] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcc0dc csinc w19, w19, wzr, gt
cpu.x[19]=(if cpu.condition(12){((cpu.x[19] as u32 as u64))}else{(((0u64))).wrapping_add(1)}) as u32 as u64;
pc=0x10fcc0e0;continue;
}
0x10fcc0e0=>{
// 0x10fcc0e0 ldr w8, [x21, #0x10]
{let address=(cpu.x[21]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcc0e4 cmp w19, w8
{let a=((cpu.x[19] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcc0e8 cset w0, ge
cpu.x[0]=((cpu.condition(10)) as u64) as u32 as u64;
pc=0x10fcc0ec;continue;
}
0x10fcc0ec=>{
// 0x10fcc0ec ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fcc0f0 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10fcc0f4 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10fcc0f8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fcc054")),}}}
#[inline]
pub(super) unsafe fn f_10fce900(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fce900=>{
// 0x10fce900 sub sp, sp, #0xc0
{let a=((cpu.sp));let b=((0xc0u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x10fce904 stp x29, x30, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fce908 stp x28, x27, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=cpu.x[28];let v1=cpu.x[27];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fce90c stp x26, x25, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=cpu.x[26];let v1=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fce910 stp x24, x23, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fce914 stp x22, x21, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fce918 stp x20, x19, [sp, #0xb0]
{let address=(cpu.sp).wrapping_add(0xb0u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fce91c add x29, sp, #0x60
{let a=((cpu.sp));let b=((0x60u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x10fce920 ldr x20, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10fce924 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10fce928 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fce92c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fce930 ldr x8, [x8, #0xa0]
{let address=(cpu.x[8]).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fce934 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fce938;super::dispatch(cpu,rt,target)?;}
pc=0x10fce938;continue;
}
0x10fce938=>{
// 0x10fce938 cbz w0, #0x10fce988
if (cpu.x[0] as u32 as u64) == 0 {pc=0x10fce988;continue;}pc=0x10fce93c;continue;
}
0x10fce93c=>{
// 0x10fce93c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fce940 mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fce944 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fce948 mov x1, xzr
cpu.x[1]=(((0u64))) as u64;
// 0x10fce94c mov w2, #1
cpu.x[2]=(((0x1u64))) as u32 as u64;
// 0x10fce950 ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fce954 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fce958;super::dispatch(cpu,rt,target)?;}
pc=0x10fce958;continue;
}
0x10fce958=>{
// 0x10fce958 tbz w0, #0, #0x10fce988
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce988;continue;}pc=0x10fce95c;continue;
}
0x10fce95c=>{
// 0x10fce95c adrp x22, #0x11980000
cpu.x[22]=(0x11980000u64) as u64;
// 0x10fce960 ldr x22, [x22, #0x550]
{let address=(cpu.x[22]).wrapping_add(0x550u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[22]=(v0) as u64;}
// 0x10fce964 ldr x8, [x20, #0xb0]
{let address=(cpu.x[20]).wrapping_add(0xb0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fce968 ldr x9, [x22]
{let address=(cpu.x[22]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fce96c tst x8, x9
{let a=((cpu.x[8]));let b=((cpu.x[9]));let result=a & b;cpu.logical_flags(result,64);}
// 0x10fce970 b.ne #0x10fce988
if cpu.condition(1){pc=0x10fce988;continue;}pc=0x10fce974;continue;
}
0x10fce974=>{
// 0x10fce974 adrp x9, #0x11980000
cpu.x[9]=(0x11980000u64) as u64;
// 0x10fce978 ldr x9, [x9, #0x578]
{let address=(cpu.x[9]).wrapping_add(0x578u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fce97c ldr x9, [x9]
{let address=(cpu.x[9]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fce980 tst x9, x8
{let a=((cpu.x[9]));let b=((cpu.x[8]));let result=a & b;cpu.logical_flags(result,64);}
// 0x10fce984 b.eq #0x10fce9a8
if cpu.condition(0){pc=0x10fce9a8;continue;}pc=0x10fce988;continue;
}
0x10fce988=>{
// 0x10fce988 ldp x20, x19, [sp, #0xb0]
{let address=(cpu.sp).wrapping_add(0xb0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fce98c ldp x22, x21, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fce990 ldp x24, x23, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fce994 ldp x26, x25, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fce998 ldp x28, x27, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fce99c ldp x29, x30, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fce9a0 add sp, sp, #0xc0
{let a=((cpu.sp));let b=((0xc0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fce9a4 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10fce9a8=>{
// 0x10fce9a8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fce9ac bl #0x10f94298
cpu.x[30]=0x10fce9b0;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
pc=0x10fce9b0;continue;
}
0x10fce9b0=>{
// 0x10fce9b0 mov w24, w0
cpu.x[24]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fce9b4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fce9b8 bl #0x10f942a0
cpu.x[30]=0x10fce9bc;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
pc=0x10fce9bc;continue;
}
0x10fce9bc=>{
// 0x10fce9bc mov w28, w0
cpu.x[28]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fce9c0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fce9c4 bl #0x10f942a8
cpu.x[30]=0x10fce9c8;super::chunk11::f_10f942a8(cpu,rt,0x10f942a8)?;
pc=0x10fce9c8;continue;
}
0x10fce9c8=>{
// 0x10fce9c8 ldr x23, [x20, #0x10]
{let address=(cpu.x[20]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10fce9cc mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fce9d0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fce9d4 ldr x25, [x23, #0xa0]
{let address=(cpu.x[23]).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[25]=(v0) as u64;}
// 0x10fce9d8 bl #0x10f942c0
cpu.x[30]=0x10fce9dc;super::chunk2::f_10f942c0(cpu,rt,0x10f942c0)?;
pc=0x10fce9dc;continue;
}
0x10fce9dc=>{
// 0x10fce9dc mov x1, x0
cpu.x[1]=(((cpu.x[0]))) as u64;
// 0x10fce9e0 add w2, w21, #0x14
{let a=((cpu.x[21] as u32 as u64));let b=((0x14u64));let result=a.wrapping_add(b);cpu.x[2]=(result) as u32 as u64;}
// 0x10fce9e4 mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10fce9e8 mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10fce9ec mov w4, wzr
cpu.x[4]=(((0u64))) as u32 as u64;
// 0x10fce9f0 mov w5, wzr
cpu.x[5]=(((0u64))) as u32 as u64;
// 0x10fce9f4 mov w6, wzr
cpu.x[6]=(((0u64))) as u32 as u64;
// 0x10fce9f8 mov w7, #-1
cpu.x[7]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10fce9fc bl #0x10f9afc8
cpu.x[30]=0x10fcea00;super::chunk2::f_10f9afc8(cpu,rt,0x10f9afc8)?;
pc=0x10fcea00;continue;
}
0x10fcea00=>{
// 0x10fcea00 mov x27, x0
cpu.x[27]=(((cpu.x[0]))) as u64;
// 0x10fcea04 stur x0, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fcea08 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcea0c bl #0x10f94124
cpu.x[30]=0x10fcea10;super::chunk5::f_10f94124(cpu,rt,0x10f94124)?;
pc=0x10fcea10;continue;
}
0x10fcea10=>{
// 0x10fcea10 ldr w8, [x27, #0xc]
{let address=(cpu.x[27]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcea14 stur w0, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fcea18 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcea1c b.le #0x10fcea7c
if cpu.condition(13){pc=0x10fcea7c;continue;}pc=0x10fcea20;continue;
}
0x10fcea20=>{
// 0x10fcea20 str w28, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=cpu.x[28] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fcea24 adrp x10, #0x11980000
cpu.x[10]=(0x11980000u64) as u64;
// 0x10fcea28 adrp x11, #0x11980000
cpu.x[11]=(0x11980000u64) as u64;
// 0x10fcea2c stur x25, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fcea30 ldr x9, [x20, #0xb0]
{let address=(cpu.x[20]).wrapping_add(0xb0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fcea34 cmp w21, #0x1f4
{let a=((cpu.x[21] as u32 as u64));let b=((0x1f4u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcea38 str w24, [sp, #0x2c]
{let address=(cpu.sp).wrapping_add(0x2cu64);let v0=cpu.x[24] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fcea3c mov w12, #0x1f4
cpu.x[12]=(((0x1f4u64))) as u32 as u64;
// 0x10fcea40 lsl x24, x8, #3
cpu.x[24]=(((((cpu.x[8]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10fcea44 stur x23, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fcea48 mov x28, xzr
cpu.x[28]=(((0u64))) as u64;
// 0x10fcea4c csel w8, w21, w12, lt
cpu.x[8]=(if cpu.condition(11){((cpu.x[21] as u32 as u64))}else{((cpu.x[12] as u32 as u64))}) as u32 as u64;
// 0x10fcea50 stur wzr, [x29, #-0xc]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff4u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fcea54 ldr x10, [x10, #0x4e0]
{let address=(cpu.x[10]).wrapping_add(0x4e0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10fcea58 ldr x11, [x11, #0x570]
{let address=(cpu.x[11]).wrapping_add(0x570u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[11]=(v0) as u64;}
// 0x10fcea5c str w8, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fcea60 ldr x10, [x10]
{let address=(cpu.x[10]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10fcea64 ldr x11, [x11]
{let address=(cpu.x[11]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[11]=(v0) as u64;}
// 0x10fcea68 and x10, x9, x10
{let a=((cpu.x[9]));let b=((cpu.x[10]));let result=a & b;cpu.x[10]=(result) as u64;}
// 0x10fcea6c and x9, x11, x9
{let a=((cpu.x[11]));let b=((cpu.x[9]));let result=a & b;cpu.x[9]=(result) as u64;}
// 0x10fcea70 stur x10, [x29, #-0x28]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffd8u64);let v0=cpu.x[10];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fcea74 str x9, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[9];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fcea78 b #0x10fceaa0
pc=0x10fceaa0;continue;
}
0x10fcea7c=>{
// 0x10fcea7c sub x1, x29, #8
{let a=((cpu.x[29]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.x[1]=(result) as u64;}
// 0x10fcea80 mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10fcea84 bl #0x10f9abb8
cpu.x[30]=0x10fcea88;super::chunk9::f_10f9abb8(cpu,rt,0x10f9abb8)?;
pc=0x10fcea88;continue;
}
0x10fcea88=>{
// 0x10fcea88 b #0x10fce988
pc=0x10fce988;continue;
}
0x10fcea8c=>{
// 0x10fcea8c ldur x8, [x29, #-0x28]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffd8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcea90 cbz x8, #0x10fceb4c
if (cpu.x[8]) == 0 {pc=0x10fceb4c;continue;}pc=0x10fcea94;continue;
}
0x10fcea94=>{
// 0x10fcea94 add x28, x28, #8
{let a=((cpu.x[28]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[28]=(result) as u64;}
// 0x10fcea98 cmp x24, x28
{let a=((cpu.x[24]));let b=((cpu.x[28]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fcea9c b.eq #0x10fced60
if cpu.condition(0){pc=0x10fced60;continue;}pc=0x10fceaa0;continue;
}
0x10fceaa0=>{
// 0x10fceaa0 ldr x8, [x27]
{let address=(cpu.x[27]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fceaa4 ldr x23, [x8, x28]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[28])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10fceaa8 cmp x23, x20
{let a=((cpu.x[23]));let b=((cpu.x[20]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fceaac b.eq #0x10fcea94
if cpu.condition(0){pc=0x10fcea94;continue;}pc=0x10fceab0;continue;
}
0x10fceab0=>{
// 0x10fceab0 cmp w26, #0
{let a=((cpu.x[26] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fceab4 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fceab8 cset w25, gt
cpu.x[25]=((cpu.condition(12)) as u64) as u32 as u64;
// 0x10fceabc bl #0x10f942a8
cpu.x[30]=0x10fceac0;super::chunk11::f_10f942a8(cpu,rt,0x10f942a8)?;
pc=0x10fceac0;continue;
}
0x10fceac0=>{
// 0x10fceac0 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fceac4 cset w8, lt
cpu.x[8]=((cpu.condition(11)) as u64) as u32 as u64;
// 0x10fceac8 cmp w25, w8
{let a=((cpu.x[25] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fceacc b.eq #0x10fcea94
if cpu.condition(0){pc=0x10fcea94;continue;}pc=0x10fcead0;continue;
}
0x10fcead0=>{
// 0x10fcead0 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcead4 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fcead8 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10fceadc mov w2, #1
cpu.x[2]=(((0x1u64))) as u32 as u64;
// 0x10fceae0 ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fceae4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fceae8;super::dispatch(cpu,rt,target)?;}
pc=0x10fceae8;continue;
}
0x10fceae8=>{
// 0x10fceae8 tbz w0, #0, #0x10fcea94
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fcea94;continue;}pc=0x10fceaec;continue;
}
0x10fceaec=>{
// 0x10fceaec ldr x8, [x22]
{let address=(cpu.x[22]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fceaf0 ldr x9, [x23, #0xb0]
{let address=(cpu.x[23]).wrapping_add(0xb0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fceaf4 tst x9, x8
{let a=((cpu.x[9]));let b=((cpu.x[8]));let result=a & b;cpu.logical_flags(result,64);}
// 0x10fceaf8 b.ne #0x10fcea94
if cpu.condition(1){pc=0x10fcea94;continue;}pc=0x10fceafc;continue;
}
0x10fceafc=>{
// 0x10fceafc ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fceb00 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fceb04 ldr x8, [x8, #0x138]
{let address=(cpu.x[8]).wrapping_add(0x138u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fceb08 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fceb0c;super::dispatch(cpu,rt,target)?;}
pc=0x10fceb0c;continue;
}
0x10fceb0c=>{
// 0x10fceb0c tbz w0, #0, #0x10fcea94
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fcea94;continue;}pc=0x10fceb10;continue;
}
0x10fceb10=>{
// 0x10fceb10 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fceb14 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fceb18 ldr x8, [x8, #0xb0]
{let address=(cpu.x[8]).wrapping_add(0xb0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fceb1c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fceb20;super::dispatch(cpu,rt,target)?;}
pc=0x10fceb20;continue;
}
0x10fceb20=>{
// 0x10fceb20 mov w25, w0
cpu.x[25]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fceb24 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fceb28 bl #0x10f94124
cpu.x[30]=0x10fceb2c;super::chunk5::f_10f94124(cpu,rt,0x10f94124)?;
pc=0x10fceb2c;continue;
}
0x10fceb2c=>{
// 0x10fceb2c ldur w8, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fceb30 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fceb34 b.ne #0x10fcea8c
if cpu.condition(1){pc=0x10fcea8c;continue;}pc=0x10fceb38;continue;
}
0x10fceb38=>{
// 0x10fceb38 ldr x8, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fceb3c cbnz x8, #0x10fcea94
if (cpu.x[8]) != 0 {pc=0x10fcea94;continue;}pc=0x10fceb40;continue;
}
0x10fceb40=>{
// 0x10fceb40 adrp x8, #0x11980000
cpu.x[8]=(0x11980000u64) as u64;
// 0x10fceb44 ldr x8, [x8, #0x570]
{let address=(cpu.x[8]).wrapping_add(0x570u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fceb48 b #0x10fceb54
pc=0x10fceb54;continue;
}
0x10fceb4c=>{
// 0x10fceb4c adrp x8, #0x11980000
cpu.x[8]=(0x11980000u64) as u64;
// 0x10fceb50 ldr x8, [x8, #0x4e0]
{let address=(cpu.x[8]).wrapping_add(0x4e0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
pc=0x10fceb54;continue;
}
0x10fceb54=>{
// 0x10fceb54 ldr x8, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fceb58 ldr x9, [x23, #0xb0]
{let address=(cpu.x[23]).wrapping_add(0xb0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fceb5c mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fceb60 tst x9, x8
{let a=((cpu.x[9]));let b=((cpu.x[8]));let result=a & b;cpu.logical_flags(result,64);}
// 0x10fceb64 csinv w8, w25, wzr, eq
cpu.x[8]=(if cpu.condition(0){((cpu.x[25] as u32 as u64))}else{!(((0u64)))}) as u32 as u64;
// 0x10fceb68 str w8, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fceb6c bl #0x10f942f0
cpu.x[30]=0x10fceb70;super::chunk11::f_10f942f0(cpu,rt,0x10f942f0)?;
pc=0x10fceb70;continue;
}
0x10fceb70=>{
// 0x10fceb70 ldr w9, [sp, #0x24]
{let address=(cpu.sp).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fceb74 cmp x0, #0
{let a=((cpu.x[0]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fceb78 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fceb7c mov x25, x0
cpu.x[25]=(((cpu.x[0]))) as u64;
// 0x10fceb80 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fceb84 csel w9, w9, w21, eq
cpu.x[9]=(if cpu.condition(0){((cpu.x[9] as u32 as u64))}else{((cpu.x[21] as u32 as u64))}) as u32 as u64;
// 0x10fceb88 ldr x8, [x8, #0xa0]
{let address=(cpu.x[8]).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fceb8c str w9, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fceb90 ldur w9, [x29, #-0xc]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fceb94 cinc w9, w9, eq
cpu.x[9]=(if cpu.condition(0){(((cpu.x[9] as u32 as u64))).wrapping_add(1)}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10fceb98 stur w9, [x29, #-0xc]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff4u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fceb9c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fceba0;super::dispatch(cpu,rt,target)?;}
pc=0x10fceba0;continue;
}
0x10fceba0=>{
// 0x10fceba0 str w0, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fceba4 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fceba8 bl #0x10f94298
cpu.x[30]=0x10fcebac;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
pc=0x10fcebac;continue;
}
0x10fcebac=>{
// 0x10fcebac str w0, [sp, #0x14]
{let address=(cpu.sp).wrapping_add(0x14u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fcebb0 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fcebb4 bl #0x10f942a0
cpu.x[30]=0x10fcebb8;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
pc=0x10fcebb8;continue;
}
0x10fcebb8=>{
// 0x10fcebb8 str w0, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fcebbc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcebc0 bl #0x10f93d2c
cpu.x[30]=0x10fcebc4;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
pc=0x10fcebc4;continue;
}
0x10fcebc4=>{
// 0x10fcebc4 str x0, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fcebc8 bl #0x10d36084
cpu.x[30]=0x10fcebcc;super::chunk6::f_10d36084(cpu,rt,0x10d36084)?;
pc=0x10fcebcc;continue;
}
0x10fcebcc=>{
// 0x10fcebcc tbz w0, #0, #0x10fcec4c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fcec4c;continue;}pc=0x10fcebd0;continue;
}
0x10fcebd0=>{
// 0x10fcebd0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fcebd4 bl #0x10fd04a8
cpu.x[30]=0x10fcebd8;super::chunk2::f_10fd04a8(cpu,rt,0x10fd04a8)?;
// 0x10fcebd8 bl #0x10e0b6b4
cpu.x[30]=0x10fcebdc;super::chunk8::f_10e0b6b4(cpu,rt,0x10e0b6b4)?;
// 0x10fcebdc ldr x0, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fcebe0 bl #0x10e16208
cpu.x[30]=0x10fcebe4;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
// 0x10fcebe4 cbnz x25, #0x10fcec4c
if (cpu.x[25]) != 0 {pc=0x10fcec4c;continue;}pc=0x10fcebe8;continue;
}
0x10fcebe8=>{
// 0x10fcebe8 ldrb w8, [x0, #0x3c4]
{let address=(cpu.x[0]).wrapping_add(0x3c4u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcebec cbz w8, #0x10fcec4c
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10fcec4c;continue;}pc=0x10fcebf0;continue;
}
0x10fcebf0=>{
// 0x10fcebf0 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fcebf4 bl #0x10f94010
cpu.x[30]=0x10fcebf8;super::chunk6::f_10f94010(cpu,rt,0x10f94010)?;
// 0x10fcebf8 cbnz w0, #0x10fcec14
if (cpu.x[0] as u32 as u64) != 0 {pc=0x10fcec14;continue;}pc=0x10fcebfc;continue;
}
0x10fcebfc=>{
// 0x10fcebfc ldr w8, [x19, #0x1d4]
{let address=(cpu.x[19]).wrapping_add(0x1d4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcec00 ldr w9, [x19, #0x1dc]
{let address=(cpu.x[19]).wrapping_add(0x1dcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fcec04 add w8, w8, #0x100
{let a=((cpu.x[8] as u32 as u64));let b=((0x100u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fcec08 add w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fcec0c str w8, [x19, #0x1d4]
{let address=(cpu.x[19]).wrapping_add(0x1d4u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fcec10 str w9, [x19, #0x1dc]
{let address=(cpu.x[19]).wrapping_add(0x1dcu64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10fcec14;continue;
}
0x10fcec14=>{
// 0x10fcec14 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fcec18 bl #0x10f94010
cpu.x[30]=0x10fcec1c;super::chunk6::f_10f94010(cpu,rt,0x10f94010)?;
// 0x10fcec1c mov w23, w0
cpu.x[23]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fcec20 ldr x0, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fcec24 bl #0x10d358a0
cpu.x[30]=0x10fcec28;super::chunk0::f_10d358a0(cpu,rt,0x10d358a0)?;
// 0x10fcec28 sub w8, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fcec2c cmp w23, w8
{let a=((cpu.x[23] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcec30 b.ne #0x10fcec4c
if cpu.condition(1){pc=0x10fcec4c;continue;}pc=0x10fcec34;continue;
}
0x10fcec34=>{
// 0x10fcec34 ldr w8, [x19, #0x1d4]
{let address=(cpu.x[19]).wrapping_add(0x1d4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcec38 ldr w9, [x19, #0x1dc]
{let address=(cpu.x[19]).wrapping_add(0x1dcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fcec3c sub w8, w8, #0x100
{let a=((cpu.x[8] as u32 as u64));let b=((0x100u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fcec40 add w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fcec44 str w8, [x19, #0x1d4]
{let address=(cpu.x[19]).wrapping_add(0x1d4u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fcec48 str w9, [x19, #0x1dc]
{let address=(cpu.x[19]).wrapping_add(0x1dcu64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10fcec4c;continue;
}
0x10fcec4c=>{
// 0x10fcec4c ldp w8, w11, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[8]=(v0) as u32 as u64;cpu.x[11]=(v1) as u32 as u64;}
// 0x10fcec50 ldr w9, [sp, #0x14]
{let address=(cpu.sp).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fcec54 subs w10, w11, w9
{let a=((cpu.x[11] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[10]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10fcec58 ldr w9, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fcec5c sub w23, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[23]=(result) as u32 as u64;}
// 0x10fcec60 ldr w8, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcec64 ldr w9, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fcec68 add w25, w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_add(b);cpu.x[25]=(result) as u32 as u64;}
// 0x10fcec6c cneg w9, w10, mi
cpu.x[9]=(if cpu.condition(4){0u64.wrapping_sub(((cpu.x[10] as u32 as u64)))}else{((cpu.x[10] as u32 as u64))}) as u32 as u64;
// 0x10fcec70 cmp w23, #0
{let a=((cpu.x[23] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcec74 cneg w8, w23, mi
cpu.x[8]=(if cpu.condition(4){0u64.wrapping_sub(((cpu.x[23] as u32 as u64)))}else{((cpu.x[23] as u32 as u64))}) as u32 as u64;
// 0x10fcec78 cmp w9, w25
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[25] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcec7c b.gt #0x10fcea94
if cpu.condition(12){pc=0x10fcea94;continue;}pc=0x10fcec80;continue;
}
0x10fcec80=>{
// 0x10fcec80 cmp w8, w25
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[25] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcec84 b.gt #0x10fcea94
if cpu.condition(12){pc=0x10fcea94;continue;}pc=0x10fcec88;continue;
}
0x10fcec88=>{
// 0x10fcec88 mul w8, w10, w10
cpu.x[8]=((((cpu.x[10] as u32 as u64))).wrapping_mul(((cpu.x[10] as u32 as u64)))) as u32 as u64;
// 0x10fcec8c str w10, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fcec90 madd w0, w23, w23, w8
cpu.x[0]=((((cpu.x[8] as u32 as u64))).wrapping_add((((cpu.x[23] as u32 as u64))).wrapping_mul(((cpu.x[23] as u32 as u64))))) as u32 as u64;
// 0x10fcec94 ldr x8, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcec98 str x8, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fcec9c cbnz w0, #0x10fcecbc
if (cpu.x[0] as u32 as u64) != 0 {pc=0x10fcecbc;continue;}pc=0x10fceca0;continue;
}
0x10fceca0=>{
// 0x10fceca0 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10fceca4 bl #0x10f9414c
cpu.x[30]=0x10fceca8;super::chunk3::f_10f9414c(cpu,rt,0x10f9414c)?;
pc=0x10fceca8;continue;
}
0x10fceca8=>{
// 0x10fceca8 mov w8, w0
cpu.x[8]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fcecac mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
// 0x10fcecb0 tst w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.logical_flags(result,32);}
// 0x10fcecb4 mov w8, #-1
cpu.x[8]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10fcecb8 cneg w23, w8, eq
cpu.x[23]=(if cpu.condition(0){0u64.wrapping_sub(((cpu.x[8] as u32 as u64)))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
pc=0x10fcecbc;continue;
}
0x10fcecbc=>{
// 0x10fcecbc mul w8, w25, w25
cpu.x[8]=((((cpu.x[25] as u32 as u64))).wrapping_mul(((cpu.x[25] as u32 as u64)))) as u32 as u64;
// 0x10fcecc0 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcecc4 b.gt #0x10fcea94
if cpu.condition(12){pc=0x10fcea94;continue;}pc=0x10fcecc8;continue;
}
0x10fcecc8=>{
// 0x10fcecc8 bl #0x112944a4
cpu.x[30]=0x10fceccc;super::chunk0::f_112944a4(cpu,rt,0x112944a4)?;
pc=0x10fceccc;continue;
}
0x10fceccc=>{
// 0x10fceccc ldr x9, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fcecd0 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcecd4 csinc w10, w0, wzr, hi
cpu.x[10]=(if cpu.condition(8){((cpu.x[0] as u32 as u64))}else{(((0u64))).wrapping_add(1)}) as u32 as u64;
// 0x10fcecd8 ldr x8, [x9]
{let address=(cpu.x[9]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcecdc mov x0, x9
cpu.x[0]=(((cpu.x[9]))) as u64;
// 0x10fcece0 str w10, [sp, #0x14]
{let address=(cpu.sp).wrapping_add(0x14u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fcece4 ldr x8, [x8, #0xb0]
{let address=(cpu.x[8]).wrapping_add(0xb0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcece8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fcecec;super::dispatch(cpu,rt,target)?;}
pc=0x10fcecec;continue;
}
0x10fcecec=>{
// 0x10fcecec ldr w11, [sp, #0x14]
{let address=(cpu.sp).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
// 0x10fcecf0 mov w9, #0x12c
cpu.x[9]=(((0x12cu64))) as u32 as u64;
// 0x10fcecf4 sub w8, w25, w11
{let a=((cpu.x[25] as u32 as u64));let b=((cpu.x[11] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fcecf8 cmp w8, #0x12c
{let a=((cpu.x[8] as u32 as u64));let b=((0x12cu64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcecfc csel w8, w8, w9, lt
cpu.x[8]=(if cpu.condition(11){((cpu.x[8] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10fced00 ldr w9, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fced04 bic w8, w8, w8, asr #31
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[8] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fced08 cmn w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.flags(a,b,result,32,false);}
// 0x10fced0c b.eq #0x10fced18
if cpu.condition(0){pc=0x10fced18;continue;}pc=0x10fced10;continue;
}
0x10fced10=>{
// 0x10fced10 mul w8, w8, w9
cpu.x[8]=((((cpu.x[8] as u32 as u64))).wrapping_mul(((cpu.x[9] as u32 as u64)))) as u32 as u64;
// 0x10fced14 sdiv w8, w8, w0
{let a=((cpu.x[8] as u32 as u64)) as i32;let b=((cpu.x[0] as u32 as u64)) as i32;cpu.x[8]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u32 as u64;}
pc=0x10fced18;continue;
}
0x10fced18=>{
// 0x10fced18 cmp w8, #0x12b
{let a=((cpu.x[8] as u32 as u64));let b=((0x12bu64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fced1c mov w9, #0x12b
cpu.x[9]=(((0x12bu64))) as u32 as u64;
// 0x10fced20 ldr w10, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10fced24 csel w8, w8, w9, lt
cpu.x[8]=(if cpu.condition(11){((cpu.x[8] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10fced28 movi v0.2s, #1
cpu.q[0]=(lane_dup(1u128,32,2)) & 0xffffffffffffffffu128;
// 0x10fced2c ldr d1, [x19, #0x1d8]
{let address=(cpu.x[19]).wrapping_add(0x1d8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.q[1]=(v0) as u128 & 0xffffffffffffffffu128;}
// 0x10fced30 add w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fced34 mul w9, w8, w23
cpu.x[9]=((((cpu.x[8] as u32 as u64))).wrapping_mul(((cpu.x[23] as u32 as u64)))) as u32 as u64;
// 0x10fced38 mul w8, w8, w10
cpu.x[8]=((((cpu.x[8] as u32 as u64))).wrapping_mul(((cpu.x[10] as u32 as u64)))) as u32 as u64;
// 0x10fced3c sdiv w9, w9, w11
{let a=((cpu.x[9] as u32 as u64)) as i32;let b=((cpu.x[11] as u32 as u64)) as i32;cpu.x[9]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u32 as u64;}
// 0x10fced40 sdiv w8, w8, w11
{let a=((cpu.x[8] as u32 as u64)) as i32;let b=((cpu.x[11] as u32 as u64)) as i32;cpu.x[8]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u32 as u64;}
// 0x10fced44 mov v0.s[0], w9
cpu.q[0]=(cpu.q[0] & !(0xffffffffu128 << 0)) | ((((cpu.x[9] as u32 as u64) as u128) as u128 & 0xffffffffu128) << 0);
// 0x10fced48 ldr w9, [x19, #0x1d4]
{let address=(cpu.x[19]).wrapping_add(0x1d4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fced4c add v0.2s, v1.2s, v0.2s
cpu.q[0]=(lane_binary((cpu.q[1] & 0xffffffffffffffffu128),(cpu.q[0] & 0xffffffffffffffffu128),32,2,"add")) & 0xffffffffffffffffu128;
// 0x10fced50 str d0, [x19, #0x1d8]
{let address=(cpu.x[19]).wrapping_add(0x1d8u64);let v0=(cpu.q[0] & 0xffffffffffffffffu128) as u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fced54 add w8, w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fced58 str w8, [x19, #0x1d4]
{let address=(cpu.x[19]).wrapping_add(0x1d4u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fced5c b #0x10fcea94
pc=0x10fcea94;continue;
}
0x10fced60=>{
// 0x10fced60 ldur x0, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fced64 sub x1, x29, #8
{let a=((cpu.x[29]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.x[1]=(result) as u64;}
// 0x10fced68 bl #0x10f9abb8
cpu.x[30]=0x10fced6c;super::chunk9::f_10f9abb8(cpu,rt,0x10f9abb8)?;
pc=0x10fced6c;continue;
}
0x10fced6c=>{
// 0x10fced6c ldur w8, [x29, #-0xc]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fced70 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fced74 b.le #0x10fce988
if cpu.condition(13){pc=0x10fce988;continue;}pc=0x10fced78;continue;
}
0x10fced78=>{
// 0x10fced78 ldur x0, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fced7c mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10fced80 bl #0x10f98fa4
cpu.x[30]=0x10fced84;super::chunk14::f_10f98fa4(cpu,rt,0x10f98fa4)?;
pc=0x10fced84;continue;
}
0x10fced84=>{
// 0x10fced84 tbz w0, #0, #0x10fce988
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fce988;continue;}pc=0x10fced88;continue;
}
0x10fced88=>{
// 0x10fced88 ldr w8, [x19, #0x1d4]
{let address=(cpu.x[19]).wrapping_add(0x1d4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fced8c ldr w9, [x19, #0x1d8]
{let address=(cpu.x[19]).wrapping_add(0x1d8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fced90 cbz w8, #0x10fced98
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10fced98;continue;}pc=0x10fced94;continue;
}
0x10fced94=>{
// 0x10fced94 cbnz w9, #0x10fce988
if (cpu.x[9] as u32 as u64) != 0 {pc=0x10fce988;continue;}pc=0x10fced98;continue;
}
0x10fced98=>{
// 0x10fced98 cbz w8, #0x10fceda8
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10fceda8;continue;}pc=0x10fced9c;continue;
}
0x10fced9c=>{
// 0x10fced9c cbnz w9, #0x10fce988
if (cpu.x[9] as u32 as u64) != 0 {pc=0x10fce988;continue;}pc=0x10fceda0;continue;
}
0x10fceda0=>{
// 0x10fceda0 str w8, [x19, #0x1d8]
{let address=(cpu.x[19]).wrapping_add(0x1d8u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fceda4 b #0x10fce988
pc=0x10fce988;continue;
}
0x10fceda8=>{
// 0x10fceda8 str w9, [x19, #0x1d4]
{let address=(cpu.x[19]).wrapping_add(0x1d4u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fcedac b #0x10fce988
pc=0x10fce988;continue;
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fce900")),}}}
#[inline]
pub(super) unsafe fn f_10fcf8d4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fcf8d4=>{
// 0x10fcf8d4 stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fcf8d8 str x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fcf8dc stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fcf8e0 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fcf8e4 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fcf8e8 ldr x19, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10fcf8ec mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10fcf8f0 mov w23, w2
cpu.x[23]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x10fcf8f4 mov w22, w1
cpu.x[22]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x10fcf8f8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fcf8fc bl #0x10f7e050
cpu.x[30]=0x10fcf900;super::chunk10::f_10f7e050(cpu,rt,0x10f7e050)?;
pc=0x10fcf900;continue;
}
0x10fcf900=>{
// 0x10fcf900 cmp w22, #1
{let a=((cpu.x[22] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcf904 b.lt #0x10fcf94c
if cpu.condition(11){pc=0x10fcf94c;continue;}pc=0x10fcf908;continue;
}
0x10fcf908=>{
// 0x10fcf908 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10fcf90c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fcf910 bl #0x10f817a8
cpu.x[30]=0x10fcf914;super::chunk8::f_10f817a8(cpu,rt,0x10f817a8)?;
pc=0x10fcf914;continue;
}
0x10fcf914=>{
// 0x10fcf914 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcf918 b.ne #0x10fcf94c
if cpu.condition(1){pc=0x10fcf94c;continue;}pc=0x10fcf91c;continue;
}
0x10fcf91c=>{
// 0x10fcf91c ldr w8, [x20, #0x1e0]
{let address=(cpu.x[20]).wrapping_add(0x1e0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcf920 mov w9, #0x270f
cpu.x[9]=(((0x270fu64))) as u32 as u64;
// 0x10fcf924 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fcf928 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcf92c b.le #0x10fcf990
if cpu.condition(13){pc=0x10fcf990;continue;}pc=0x10fcf930;continue;
}
0x10fcf930=>{
// 0x10fcf930 bl #0x10f9433c
cpu.x[30]=0x10fcf934;super::chunk3::f_10f9433c(cpu,rt,0x10f9433c)?;
pc=0x10fcf934;continue;
}
0x10fcf934=>{
// 0x10fcf934 cbz x0, #0x10fcfa54
if (cpu.x[0]) == 0 {pc=0x10fcfa54;continue;}pc=0x10fcf938;continue;
}
0x10fcf938=>{
// 0x10fcf938 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fcf93c bl #0x10f9433c
cpu.x[30]=0x10fcf940;super::chunk3::f_10f9433c(cpu,rt,0x10f9433c)?;
pc=0x10fcf940;continue;
}
0x10fcf940=>{
// 0x10fcf940 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fcf944 strb w8, [x0, #0x3f]
{let address=(cpu.x[0]).wrapping_add(0x3fu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10fcf948 b #0x10fcfa54
pc=0x10fcfa54;continue;
}
0x10fcf94c=>{
// 0x10fcf94c tbnz w23, #0, #0x10fcfa54
if ((cpu.x[23] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10fcfa54;continue;}pc=0x10fcf950;continue;
}
0x10fcf950=>{
// 0x10fcf950 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fcf954 bl #0x10f817a8
cpu.x[30]=0x10fcf958;super::chunk8::f_10f817a8(cpu,rt,0x10f817a8)?;
pc=0x10fcf958;continue;
}
0x10fcf958=>{
// 0x10fcf958 cmp w0, #5
{let a=((cpu.x[0] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcf95c b.eq #0x10fcfa54
if cpu.condition(0){pc=0x10fcfa54;continue;}pc=0x10fcf960;continue;
}
0x10fcf960=>{
// 0x10fcf960 cmp w0, #8
{let a=((cpu.x[0] as u32 as u64));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcf964 b.eq #0x10fcfa54
if cpu.condition(0){pc=0x10fcfa54;continue;}pc=0x10fcf968;continue;
}
0x10fcf968=>{
// 0x10fcf968 ldr x21, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10fcf96c mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fcf970 bl #0x10f7e050
cpu.x[30]=0x10fcf974;super::chunk10::f_10f7e050(cpu,rt,0x10f7e050)?;
pc=0x10fcf974;continue;
}
0x10fcf974=>{
// 0x10fcf974 bl #0x10de0494
cpu.x[30]=0x10fcf978;super::chunk2::f_10de0494(cpu,rt,0x10de0494)?;
pc=0x10fcf978;continue;
}
0x10fcf978=>{
// 0x10fcf978 cbz w0, #0x10fcfa28
if (cpu.x[0] as u32 as u64) == 0 {pc=0x10fcfa28;continue;}pc=0x10fcf97c;continue;
}
0x10fcf97c=>{
// 0x10fcf97c mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fcf980 str wzr, [x20, #0x1e0]
{let address=(cpu.x[20]).wrapping_add(0x1e0u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fcf984 bl #0x10f94608
cpu.x[30]=0x10fcf988;super::chunk12::f_10f94608(cpu,rt,0x10f94608)?;
pc=0x10fcf988;continue;
}
0x10fcf988=>{
// 0x10fcf988 cbnz x0, #0x10fcfa50
if (cpu.x[0]) != 0 {pc=0x10fcfa50;continue;}pc=0x10fcf98c;continue;
}
0x10fcf98c=>{
// 0x10fcf98c b #0x10fcfa54
pc=0x10fcfa54;continue;
}
0x10fcf990=>{
// 0x10fcf990 bl #0x10f7e050
cpu.x[30]=0x10fcf994;super::chunk10::f_10f7e050(cpu,rt,0x10f7e050)?;
pc=0x10fcf994;continue;
}
0x10fcf994=>{
// 0x10fcf994 bl #0x10de0494
cpu.x[30]=0x10fcf998;super::chunk2::f_10de0494(cpu,rt,0x10de0494)?;
pc=0x10fcf998;continue;
}
0x10fcf998=>{
// 0x10fcf998 cbz w0, #0x10fcf9c8
if (cpu.x[0] as u32 as u64) == 0 {pc=0x10fcf9c8;continue;}pc=0x10fcf99c;continue;
}
0x10fcf99c=>{
// 0x10fcf99c bic w8, w22, w22, asr #31
{let a=((cpu.x[22] as u32 as u64));let b=(((cpu.x[22] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fcf9a0 mov w9, #0x3e8
cpu.x[9]=(((0x3e8u64))) as u32 as u64;
// 0x10fcf9a4 mul w8, w8, w9
cpu.x[8]=((((cpu.x[8] as u32 as u64))).wrapping_mul(((cpu.x[9] as u32 as u64)))) as u32 as u64;
// 0x10fcf9a8 ldr w9, [x20, #0x1e0]
{let address=(cpu.x[20]).wrapping_add(0x1e0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fcf9ac sdiv w8, w8, w0
{let a=((cpu.x[8] as u32 as u64)) as i32;let b=((cpu.x[0] as u32 as u64)) as i32;cpu.x[8]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u32 as u64;}
// 0x10fcf9b0 add w8, w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fcf9b4 str w8, [x20, #0x1e0]
{let address=(cpu.x[20]).wrapping_add(0x1e0u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fcf9b8 mov w9, #0x2710
cpu.x[9]=(((0x2710u64))) as u32 as u64;
// 0x10fcf9bc cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcf9c0 b.ge #0x10fcfa14
if cpu.condition(10){pc=0x10fcfa14;continue;}pc=0x10fcf9c4;continue;
}
0x10fcf9c4=>{
// 0x10fcf9c4 b #0x10fcfa54
pc=0x10fcfa54;continue;
}
0x10fcf9c8=>{
// 0x10fcf9c8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fcf9cc bl #0x10f940fc
cpu.x[30]=0x10fcf9d0;super::chunk9::f_10f940fc(cpu,rt,0x10f940fc)?;
// 0x10fcf9d0 cbz x0, #0x10fcf9dc
if (cpu.x[0]) == 0 {pc=0x10fcf9dc;continue;}pc=0x10fcf9d4;continue;
}
0x10fcf9d4=>{
// 0x10fcf9d4 bl #0x10fc2980
cpu.x[30]=0x10fcf9d8;super::chunk4::f_10fc2980(cpu,rt,0x10fc2980)?;
// 0x10fcf9d8 cbnz w0, #0x10fcf99c
if (cpu.x[0] as u32 as u64) != 0 {pc=0x10fcf99c;continue;}pc=0x10fcf9dc;continue;
}
0x10fcf9dc=>{
// 0x10fcf9dc ldr x22, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[22]=(v0) as u64;}
// 0x10fcf9e0 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10fcf9e4 bl #0x10f7e050
cpu.x[30]=0x10fcf9e8;super::chunk10::f_10f7e050(cpu,rt,0x10f7e050)?;
// 0x10fcf9e8 bl #0x10de0494
cpu.x[30]=0x10fcf9ec;super::chunk2::f_10de0494(cpu,rt,0x10de0494)?;
// 0x10fcf9ec cbz w0, #0x10fcfa90
if (cpu.x[0] as u32 as u64) == 0 {pc=0x10fcfa90;continue;}pc=0x10fcf9f0;continue;
}
0x10fcf9f0=>{
// 0x10fcf9f0 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10fcf9f4 str wzr, [x20, #0x1e0]
{let address=(cpu.x[20]).wrapping_add(0x1e0u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fcf9f8 bl #0x10f94608
cpu.x[30]=0x10fcf9fc;super::chunk12::f_10f94608(cpu,rt,0x10f94608)?;
// 0x10fcf9fc cbz x0, #0x10fcfa04
if (cpu.x[0]) == 0 {pc=0x10fcfa04;continue;}pc=0x10fcfa00;continue;
}
0x10fcfa00=>{
// 0x10fcfa00 strb wzr, [x0, #0x3f]
{let address=(cpu.x[0]).wrapping_add(0x3fu64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
pc=0x10fcfa04;continue;
}
0x10fcfa04=>{
// 0x10fcfa04 ldr w8, [x20, #0x1e0]
{let address=(cpu.x[20]).wrapping_add(0x1e0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcfa08 mov w9, #0x2710
cpu.x[9]=(((0x2710u64))) as u32 as u64;
// 0x10fcfa0c cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcfa10 b.lt #0x10fcfa54
if cpu.condition(11){pc=0x10fcfa54;continue;}pc=0x10fcfa14;continue;
}
0x10fcfa14=>{
// 0x10fcfa14 ldr x2, [x21, #0x188]
{let address=(cpu.x[21]).wrapping_add(0x188u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10fcfa18 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fcfa1c mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10fcfa20 bl #0x10f89720
cpu.x[30]=0x10fcfa24;super::chunk13::f_10f89720(cpu,rt,0x10f89720)?;
pc=0x10fcfa24;continue;
}
0x10fcfa24=>{
// 0x10fcfa24 b #0x10fcfa54
pc=0x10fcfa54;continue;
}
0x10fcfa28=>{
// 0x10fcfa28 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fcfa2c bl #0x10f940fc
cpu.x[30]=0x10fcfa30;super::chunk9::f_10f940fc(cpu,rt,0x10f940fc)?;
// 0x10fcfa30 cbz x0, #0x10fcfa3c
if (cpu.x[0]) == 0 {pc=0x10fcfa3c;continue;}pc=0x10fcfa34;continue;
}
0x10fcfa34=>{
// 0x10fcfa34 bl #0x10fc2980
cpu.x[30]=0x10fcfa38;super::chunk4::f_10fc2980(cpu,rt,0x10fc2980)?;
// 0x10fcfa38 cbnz w0, #0x10fcf97c
if (cpu.x[0] as u32 as u64) != 0 {pc=0x10fcf97c;continue;}pc=0x10fcfa3c;continue;
}
0x10fcfa3c=>{
// 0x10fcfa3c mov w8, #-1
cpu.x[8]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10fcfa40 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fcfa44 str w8, [x20, #0x1e0]
{let address=(cpu.x[20]).wrapping_add(0x1e0u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fcfa48 bl #0x10f94608
cpu.x[30]=0x10fcfa4c;super::chunk12::f_10f94608(cpu,rt,0x10f94608)?;
// 0x10fcfa4c cbz x0, #0x10fcfa54
if (cpu.x[0]) == 0 {pc=0x10fcfa54;continue;}pc=0x10fcfa50;continue;
}
0x10fcfa50=>{
// 0x10fcfa50 strb wzr, [x0, #0x3f]
{let address=(cpu.x[0]).wrapping_add(0x3fu64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
pc=0x10fcfa54;continue;
}
0x10fcfa54=>{
// 0x10fcfa54 ldr w8, [x20, #0x1e0]
{let address=(cpu.x[20]).wrapping_add(0x1e0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcfa58 mov w9, #0x2710
cpu.x[9]=(((0x2710u64))) as u32 as u64;
// 0x10fcfa5c cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcfa60 b.lt #0x10fcfa7c
if cpu.condition(11){pc=0x10fcfa7c;continue;}pc=0x10fcfa64;continue;
}
0x10fcfa64=>{
// 0x10fcfa64 adrp x8, #0x11a65000
cpu.x[8]=(0x11a65000u64) as u64;
// 0x10fcfa68 add x8, x8, #0xf80
{let a=((cpu.x[8]));let b=((0xf80u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fcfa6c ldr x9, [x19, #0xb8]
{let address=(cpu.x[19]).wrapping_add(0xb8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fcfa70 ldr x8, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcfa74 orr x8, x9, x8
{let a=((cpu.x[9]));let b=((cpu.x[8]));let result=a | b;cpu.x[8]=(result) as u64;}
// 0x10fcfa78 str x8, [x19, #0xb8]
{let address=(cpu.x[19]).wrapping_add(0xb8u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10fcfa7c;continue;
}
0x10fcfa7c=>{
// 0x10fcfa7c ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fcfa80 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10fcfa84 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fcfa88 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10fcfa8c ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10fcfa90=>{
// 0x10fcfa90 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10fcfa94 bl #0x10f940fc
cpu.x[30]=0x10fcfa98;super::chunk9::f_10f940fc(cpu,rt,0x10f940fc)?;
// 0x10fcfa98 cbz x0, #0x10fcfaa4
if (cpu.x[0]) == 0 {pc=0x10fcfaa4;continue;}pc=0x10fcfa9c;continue;
}
0x10fcfa9c=>{
// 0x10fcfa9c bl #0x10fc2980
cpu.x[30]=0x10fcfaa0;super::chunk4::f_10fc2980(cpu,rt,0x10fc2980)?;
// 0x10fcfaa0 cbnz w0, #0x10fcf9f0
if (cpu.x[0] as u32 as u64) != 0 {pc=0x10fcf9f0;continue;}pc=0x10fcfaa4;continue;
}
0x10fcfaa4=>{
// 0x10fcfaa4 mov w8, #-1
cpu.x[8]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10fcfaa8 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10fcfaac str w8, [x20, #0x1e0]
{let address=(cpu.x[20]).wrapping_add(0x1e0u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fcfab0 bl #0x10f94608
cpu.x[30]=0x10fcfab4;super::chunk12::f_10f94608(cpu,rt,0x10f94608)?;
// 0x10fcfab4 cbnz x0, #0x10fcfa00
if (cpu.x[0]) != 0 {pc=0x10fcfa00;continue;}pc=0x10fcfab8;continue;
}
0x10fcfab8=>{
// 0x10fcfab8 b #0x10fcfa04
pc=0x10fcfa04;continue;
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fcf8d4")),}}}
#[inline]
pub(super) unsafe fn f_10fd1174(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fd1174=>{
// 0x10fd1174 ldr x0, [x0, #0x28]
{let address=(cpu.x[0]).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fd1178 cbz x0, #0x10fd1180
if (cpu.x[0]) == 0 {pc=0x10fd1180;continue;}pc=0x10fd117c;continue;
}
0x10fd117c=>{
// 0x10fd117c b #0x10f87f00
super::chunk12::f_10f87f00(cpu,rt,0x10f87f00)?;return Ok(());
}
0x10fd1180=>{
// 0x10fd1180 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fd1174")),}}}
#[inline]
pub(super) unsafe fn f_10fdb6e0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fdb6e0=>{
// 0x10fdb6e0 mov w8, #0xde83
cpu.x[8]=(((0xde83u64))) as u32 as u64;
// 0x10fdb6e4 movk w8, #0x431b, lsl #16
cpu.x[8]=((cpu.x[8] as u32 as u64 & !(0xffff0000u64)) | 0x431b0000u64) as u32 as u64;
// 0x10fdb6e8 smull x8, w0, w8
cpu.x[8]=(((((cpu.x[0] as u32 as u64))) as i32 as i64).wrapping_mul((((cpu.x[8] as u32 as u64))) as i32 as i64) as u64) as u64;
// 0x10fdb6ec lsr x9, x8, #0x3f
cpu.x[9]=(((((cpu.x[8]))) as u64).wrapping_shr(((((0x3fu64))) & 63) as u32) as u64) as u64;
// 0x10fdb6f0 asr x8, x8, #0x32
cpu.x[8]=(((((cpu.x[8]))) as i64).wrapping_shr(((((0x32u64))) & 63) as u32) as u64) as u64;
// 0x10fdb6f4 add w0, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u32 as u64;}
// 0x10fdb6f8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fdb6e0")),}}}
#[inline]
pub(super) unsafe fn f_10fe6e00(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fe6e00=>{
// 0x10fe6e00 movi v0.2d, #0000000000000000
cpu.q[0]=(lane_dup(0u128,64,2)) & 0xffffffffffffffffffffffffffffffffu128;
// 0x10fe6e04 stp q0, q0, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10fe6e08 stp q0, q0, [x0, #0x20]
{let address=(cpu.x[0]).wrapping_add(0x20u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10fe6e0c stp q0, q0, [x0, #0x40]
{let address=(cpu.x[0]).wrapping_add(0x40u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10fe6e10 str x1, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=cpu.x[1];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fe6e14 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fe6e00")),}}}
#[inline]
pub(super) unsafe fn f_10fecfd4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fecfd4=>{
// 0x10fecfd4 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fecfd8 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fecfdc stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fecfe0 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fecfe4 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fecfe8 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10fecfec mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x10fecff0 ldr x8, [x8, #0x58]
{let address=(cpu.x[8]).wrapping_add(0x58u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fecff4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fecff8;super::dispatch(cpu,rt,target)?;}
pc=0x10fecff8;continue;
}
0x10fecff8=>{
// 0x10fecff8 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fecffc b.lt #0x10fed038
if cpu.condition(11){pc=0x10fed038;continue;}pc=0x10fed000;continue;
}
0x10fed000=>{
// 0x10fed000 mov w21, wzr
cpu.x[21]=(((0u64))) as u32 as u64;
// 0x10fed004 cbnz x20, #0x10fed06c
if (cpu.x[20]) != 0 {pc=0x10fed06c;continue;}pc=0x10fed008;continue;
}
0x10fed008=>{
// 0x10fed008 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fed00c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fed010 mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10fed014 ldr x8, [x8, #0x38]
{let address=(cpu.x[8]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fed018 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fed01c;super::dispatch(cpu,rt,target)?;}
// 0x10fed01c ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fed020 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fed024 add w21, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[21]=(result) as u32 as u64;}
// 0x10fed028 ldr x8, [x8, #0x58]
{let address=(cpu.x[8]).wrapping_add(0x58u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fed02c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fed030;super::dispatch(cpu,rt,target)?;}
// 0x10fed030 cmp w21, w0
{let a=((cpu.x[21] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fed034 b.lt #0x10fed008
if cpu.condition(11){pc=0x10fed008;continue;}pc=0x10fed038;continue;
}
0x10fed038=>{
// 0x10fed038 ldr w8, [x19, #0x3c]
{let address=(cpu.x[19]).wrapping_add(0x3cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fed03c cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fed040 b.lt #0x10fed0f4
if cpu.condition(11){pc=0x10fed0f4;continue;}pc=0x10fed044;continue;
}
0x10fed044=>{
// 0x10fed044 cbz x20, #0x10fed0f4
if (cpu.x[20]) == 0 {pc=0x10fed0f4;continue;}pc=0x10fed048;continue;
}
0x10fed048=>{
// 0x10fed048 mov x21, xzr
cpu.x[21]=(((0u64))) as u64;
// 0x10fed04c b #0x10fed0c0
pc=0x10fed0c0;continue;
}
0x10fed050=>{
// 0x10fed050 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fed054 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fed058 add w21, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[21]=(result) as u32 as u64;}
// 0x10fed05c ldr x8, [x8, #0x58]
{let address=(cpu.x[8]).wrapping_add(0x58u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fed060 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fed064;super::dispatch(cpu,rt,target)?;}
pc=0x10fed064;continue;
}
0x10fed064=>{
// 0x10fed064 cmp w21, w0
{let a=((cpu.x[21] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fed068 b.ge #0x10fed038
if cpu.condition(10){pc=0x10fed038;continue;}pc=0x10fed06c;continue;
}
0x10fed06c=>{
// 0x10fed06c ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fed070 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fed074 mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10fed078 ldr x8, [x8, #0x38]
{let address=(cpu.x[8]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fed07c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fed080;super::dispatch(cpu,rt,target)?;}
pc=0x10fed080;continue;
}
0x10fed080=>{
// 0x10fed080 cbz x0, #0x10fed050
if (cpu.x[0]) == 0 {pc=0x10fed050;continue;}pc=0x10fed084;continue;
}
0x10fed084=>{
// 0x10fed084 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fed088 ldr x8, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fed08c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fed090;super::dispatch(cpu,rt,target)?;}
pc=0x10fed090;continue;
}
0x10fed090=>{
// 0x10fed090 mov x1, x0
cpu.x[1]=(((cpu.x[0]))) as u64;
// 0x10fed094 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fed098 bl #0x10e0bc20
cpu.x[30]=0x10fed09c;super::chunk1::f_10e0bc20(cpu,rt,0x10e0bc20)?;
pc=0x10fed09c;continue;
}
0x10fed09c=>{
// 0x10fed09c tbz w0, #0, #0x10fed050
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fed050;continue;}pc=0x10fed0a0;continue;
}
0x10fed0a0=>{
// 0x10fed0a0 mov w0, w21
cpu.x[0]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10fed0a4 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fed0a8 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10fed0ac ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10fed0b0 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10fed0b4=>{
// 0x10fed0b4 add x21, x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[21]=(result) as u64;}
// 0x10fed0b8 cmp x21, w8, sxtw
{let a=((cpu.x[21]));let b=(((cpu.x[8] as u32 as u64) as i32 as i64 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fed0bc b.ge #0x10fed0f4
if cpu.condition(10){pc=0x10fed0f4;continue;}pc=0x10fed0c0;continue;
}
0x10fed0c0=>{
// 0x10fed0c0 ldr x9, [x19, #0x30]
{let address=(cpu.x[19]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fed0c4 ldr x9, [x9, x21, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[21])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fed0c8 ldr x0, [x9]
{let address=(cpu.x[9]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fed0cc cbz x0, #0x10fed0b4
if (cpu.x[0]) == 0 {pc=0x10fed0b4;continue;}pc=0x10fed0d0;continue;
}
0x10fed0d0=>{
// 0x10fed0d0 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fed0d4 ldr x8, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fed0d8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fed0dc;super::dispatch(cpu,rt,target)?;}
// 0x10fed0dc mov x1, x0
cpu.x[1]=(((cpu.x[0]))) as u64;
// 0x10fed0e0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fed0e4 bl #0x10e0bc20
cpu.x[30]=0x10fed0e8;super::chunk1::f_10e0bc20(cpu,rt,0x10e0bc20)?;
// 0x10fed0e8 tbnz w0, #0, #0x10fed10c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10fed10c;continue;}pc=0x10fed0ec;continue;
}
0x10fed0ec=>{
// 0x10fed0ec ldr w8, [x19, #0x3c]
{let address=(cpu.x[19]).wrapping_add(0x3cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fed0f0 b #0x10fed0b4
pc=0x10fed0b4;continue;
}
0x10fed0f4=>{
// 0x10fed0f4 mov w21, #-1
cpu.x[21]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10fed0f8 mov w0, w21
cpu.x[0]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10fed0fc ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fed100 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10fed104 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10fed108 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10fed10c=>{
// 0x10fed10c ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fed110 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fed114 ldr x8, [x8, #0x58]
{let address=(cpu.x[8]).wrapping_add(0x58u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fed118 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fed11c;super::dispatch(cpu,rt,target)?;}
// 0x10fed11c add w21, w0, w21
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[21] as u32 as u64));let result=a.wrapping_add(b);cpu.x[21]=(result) as u32 as u64;}
// 0x10fed120 mov w0, w21
cpu.x[0]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10fed124 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fed128 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10fed12c ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10fed130 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fecfd4")),}}}
#[inline]
pub(super) unsafe fn f_1101b380(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x1101b380=>{
// 0x1101b380 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x1101b384 stp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x1101b388 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x1101b38c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x1101b390 ldr w8, [x0, #0x3c]
{let address=(cpu.x[0]).wrapping_add(0x3cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x1101b394 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x1101b398 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x1101b39c b.lt #0x1101b3cc
if cpu.condition(11){pc=0x1101b3cc;continue;}pc=0x1101b3a0;continue;
}
0x1101b3a0=>{
// 0x1101b3a0 mov x20, xzr
cpu.x[20]=(((0u64))) as u64;
// 0x1101b3a4 b #0x1101b3b4
pc=0x1101b3b4;continue;
}
0x1101b3a8=>{
// 0x1101b3a8 add x20, x20, #1
{let a=((cpu.x[20]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[20]=(result) as u64;}
// 0x1101b3ac cmp x20, w8, sxtw
{let a=((cpu.x[20]));let b=(((cpu.x[8] as u32 as u64) as i32 as i64 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x1101b3b0 b.ge #0x1101b3cc
if cpu.condition(10){pc=0x1101b3cc;continue;}pc=0x1101b3b4;continue;
}
0x1101b3b4=>{
// 0x1101b3b4 ldr x9, [x19, #0x30]
{let address=(cpu.x[19]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x1101b3b8 ldr x0, [x9, x20, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[20])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x1101b3bc cbz x0, #0x1101b3a8
if (cpu.x[0]) == 0 {pc=0x1101b3a8;continue;}pc=0x1101b3c0;continue;
}
0x1101b3c0=>{
// 0x1101b3c0 bl #0x1187d2c0
cpu.x[30]=0x1101b3c4;super::dispatch(cpu,rt,0x1187d2c0)?;
// 0x1101b3c4 ldr w8, [x19, #0x3c]
{let address=(cpu.x[19]).wrapping_add(0x3cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x1101b3c8 b #0x1101b3a8
pc=0x1101b3a8;continue;
}
0x1101b3cc=>{
// 0x1101b3cc mov x21, x19
cpu.x[21]=(((cpu.x[19]))) as u64;
// 0x1101b3d0 ldr w8, [x21, #0x4c]!
{let address=(cpu.x[21]).wrapping_add(0x4cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;cpu.x[21]=(address) as u64;}
// 0x1101b3d4 stur wzr, [x21, #-0x10]
{let address=(cpu.x[21]).wrapping_add(0xfffffffffffffff0u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1101b3d8 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x1101b3dc b.lt #0x1101b40c
if cpu.condition(11){pc=0x1101b40c;continue;}pc=0x1101b3e0;continue;
}
0x1101b3e0=>{
// 0x1101b3e0 mov x20, xzr
cpu.x[20]=(((0u64))) as u64;
// 0x1101b3e4 b #0x1101b3f4
pc=0x1101b3f4;continue;
}
0x1101b3e8=>{
// 0x1101b3e8 add x20, x20, #1
{let a=((cpu.x[20]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[20]=(result) as u64;}
// 0x1101b3ec cmp x20, w8, sxtw
{let a=((cpu.x[20]));let b=(((cpu.x[8] as u32 as u64) as i32 as i64 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x1101b3f0 b.ge #0x1101b40c
if cpu.condition(10){pc=0x1101b40c;continue;}pc=0x1101b3f4;continue;
}
0x1101b3f4=>{
// 0x1101b3f4 ldr x9, [x19, #0x40]
{let address=(cpu.x[19]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x1101b3f8 ldr x0, [x9, x20, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[20])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x1101b3fc cbz x0, #0x1101b3e8
if (cpu.x[0]) == 0 {pc=0x1101b3e8;continue;}pc=0x1101b400;continue;
}
0x1101b400=>{
// 0x1101b400 bl #0x1187d2c0
cpu.x[30]=0x1101b404;super::dispatch(cpu,rt,0x1187d2c0)?;
// 0x1101b404 ldr w8, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x1101b408 b #0x1101b3e8
pc=0x1101b3e8;continue;
}
0x1101b40c=>{
// 0x1101b40c ldr x0, [x19, #0x50]
{let address=(cpu.x[19]).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x1101b410 str wzr, [x19, #0x4c]
{let address=(cpu.x[19]).wrapping_add(0x4cu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1101b414 cbz x0, #0x1101b41c
if (cpu.x[0]) == 0 {pc=0x1101b41c;continue;}pc=0x1101b418;continue;
}
0x1101b418=>{
// 0x1101b418 bl #0x1187d2c0
cpu.x[30]=0x1101b41c;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x1101b41c;continue;
}
0x1101b41c=>{
// 0x1101b41c ldr x0, [x19, #0x58]
{let address=(cpu.x[19]).wrapping_add(0x58u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x1101b420 str xzr, [x19, #0x50]
{let address=(cpu.x[19]).wrapping_add(0x50u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x1101b424 cbz x0, #0x1101b42c
if (cpu.x[0]) == 0 {pc=0x1101b42c;continue;}pc=0x1101b428;continue;
}
0x1101b428=>{
// 0x1101b428 bl #0x1187d2c0
cpu.x[30]=0x1101b42c;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x1101b42c;continue;
}
0x1101b42c=>{
// 0x1101b42c ldr x0, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x1101b430 str xzr, [x19, #0x58]
{let address=(cpu.x[19]).wrapping_add(0x58u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x1101b434 cbz x0, #0x1101b43c
if (cpu.x[0]) == 0 {pc=0x1101b43c;continue;}pc=0x1101b438;continue;
}
0x1101b438=>{
// 0x1101b438 bl #0x1187d2c0
cpu.x[30]=0x1101b43c;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x1101b43c;continue;
}
0x1101b43c=>{
// 0x1101b43c ldr x0, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x1101b440 str xzr, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x1101b444 cbz x0, #0x1101b44c
if (cpu.x[0]) == 0 {pc=0x1101b44c;continue;}pc=0x1101b448;continue;
}
0x1101b448=>{
// 0x1101b448 bl #0x1187d2c0
cpu.x[30]=0x1101b44c;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x1101b44c;continue;
}
0x1101b44c=>{
// 0x1101b44c ldr x0, [x19, #0x70]
{let address=(cpu.x[19]).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x1101b450 str xzr, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x1101b454 cbz x0, #0x1101b45c
if (cpu.x[0]) == 0 {pc=0x1101b45c;continue;}pc=0x1101b458;continue;
}
0x1101b458=>{
// 0x1101b458 bl #0x1187d2c0
cpu.x[30]=0x1101b45c;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x1101b45c;continue;
}
0x1101b45c=>{
// 0x1101b45c ldr x0, [x19, #0x78]
{let address=(cpu.x[19]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x1101b460 str xzr, [x19, #0x70]
{let address=(cpu.x[19]).wrapping_add(0x70u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x1101b464 cbz x0, #0x1101b46c
if (cpu.x[0]) == 0 {pc=0x1101b46c;continue;}pc=0x1101b468;continue;
}
0x1101b468=>{
// 0x1101b468 bl #0x1187d2c0
cpu.x[30]=0x1101b46c;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x1101b46c;continue;
}
0x1101b46c=>{
// 0x1101b46c ldr x0, [x19, #0x80]
{let address=(cpu.x[19]).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x1101b470 str xzr, [x19, #0x78]
{let address=(cpu.x[19]).wrapping_add(0x78u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x1101b474 cbz x0, #0x1101b47c
if (cpu.x[0]) == 0 {pc=0x1101b47c;continue;}pc=0x1101b478;continue;
}
0x1101b478=>{
// 0x1101b478 bl #0x1187d2c0
cpu.x[30]=0x1101b47c;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x1101b47c;continue;
}
0x1101b47c=>{
// 0x1101b47c ldr x0, [x19, #0x88]
{let address=(cpu.x[19]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x1101b480 str xzr, [x19, #0x80]
{let address=(cpu.x[19]).wrapping_add(0x80u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x1101b484 cbz x0, #0x1101b48c
if (cpu.x[0]) == 0 {pc=0x1101b48c;continue;}pc=0x1101b488;continue;
}
0x1101b488=>{
// 0x1101b488 bl #0x1187d2c0
cpu.x[30]=0x1101b48c;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x1101b48c;continue;
}
0x1101b48c=>{
// 0x1101b48c ldr w8, [x19, #0x9c]
{let address=(cpu.x[19]).wrapping_add(0x9cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x1101b490 str xzr, [x19, #0x88]
{let address=(cpu.x[19]).wrapping_add(0x88u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x1101b494 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x1101b498 b.le #0x1101b4d4
if cpu.condition(13){pc=0x1101b4d4;continue;}pc=0x1101b49c;continue;
}
0x1101b49c=>{
// 0x1101b49c mov x22, xzr
cpu.x[22]=(((0u64))) as u64;
// 0x1101b4a0 b #0x1101b4b0
pc=0x1101b4b0;continue;
}
0x1101b4a4=>{
// 0x1101b4a4 add x22, x22, #1
{let a=((cpu.x[22]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[22]=(result) as u64;}
// 0x1101b4a8 cmp x22, w8, sxtw
{let a=((cpu.x[22]));let b=(((cpu.x[8] as u32 as u64) as i32 as i64 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x1101b4ac b.ge #0x1101b4d4
if cpu.condition(10){pc=0x1101b4d4;continue;}pc=0x1101b4b0;continue;
}
0x1101b4b0=>{
// 0x1101b4b0 ldr x9, [x19, #0x90]
{let address=(cpu.x[19]).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x1101b4b4 ldr x20, [x9, x22, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[22])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x1101b4b8 cbz x20, #0x1101b4a4
if (cpu.x[20]) == 0 {pc=0x1101b4a4;continue;}pc=0x1101b4bc;continue;
}
0x1101b4bc=>{
// 0x1101b4bc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x1101b4c0 bl #0x1101b010
cpu.x[30]=0x1101b4c4;super::dispatch(cpu,rt,0x1101b010)?;
// 0x1101b4c4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x1101b4c8 bl #0x1187d2c0
cpu.x[30]=0x1101b4cc;super::dispatch(cpu,rt,0x1187d2c0)?;
// 0x1101b4cc ldr w8, [x19, #0x9c]
{let address=(cpu.x[19]).wrapping_add(0x9cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x1101b4d0 b #0x1101b4a4
pc=0x1101b4a4;continue;
}
0x1101b4d4=>{
// 0x1101b4d4 mov x22, x19
cpu.x[22]=(((cpu.x[19]))) as u64;
// 0x1101b4d8 ldr w8, [x22, #0xac]!
{let address=(cpu.x[22]).wrapping_add(0xacu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;cpu.x[22]=(address) as u64;}
// 0x1101b4dc stur wzr, [x22, #-0x10]
{let address=(cpu.x[22]).wrapping_add(0xfffffffffffffff0u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1101b4e0 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x1101b4e4 b.lt #0x1101b514
if cpu.condition(11){pc=0x1101b514;continue;}pc=0x1101b4e8;continue;
}
0x1101b4e8=>{
// 0x1101b4e8 mov x20, xzr
cpu.x[20]=(((0u64))) as u64;
// 0x1101b4ec b #0x1101b4fc
pc=0x1101b4fc;continue;
}
0x1101b4f0=>{
// 0x1101b4f0 add x20, x20, #1
{let a=((cpu.x[20]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[20]=(result) as u64;}
// 0x1101b4f4 cmp x20, w8, sxtw
{let a=((cpu.x[20]));let b=(((cpu.x[8] as u32 as u64) as i32 as i64 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x1101b4f8 b.ge #0x1101b514
if cpu.condition(10){pc=0x1101b514;continue;}pc=0x1101b4fc;continue;
}
0x1101b4fc=>{
// 0x1101b4fc ldr x9, [x19, #0xa0]
{let address=(cpu.x[19]).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x1101b500 ldr x0, [x9, x20, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[20])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x1101b504 cbz x0, #0x1101b4f0
if (cpu.x[0]) == 0 {pc=0x1101b4f0;continue;}pc=0x1101b508;continue;
}
0x1101b508=>{
// 0x1101b508 bl #0x1187d2c0
cpu.x[30]=0x1101b50c;super::dispatch(cpu,rt,0x1187d2c0)?;
// 0x1101b50c ldr w8, [x22]
{let address=(cpu.x[22]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x1101b510 b #0x1101b4f0
pc=0x1101b4f0;continue;
}
0x1101b514=>{
// 0x1101b514 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x1101b518 bl #0x1101b5ac
cpu.x[30]=0x1101b51c;super::chunk0::f_1101b5ac(cpu,rt,0x1101b5ac)?;
pc=0x1101b51c;continue;
}
0x1101b51c=>{
// 0x1101b51c ldr x20, [x19, #0xb8]
{let address=(cpu.x[19]).wrapping_add(0xb8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x1101b520 cbz x20, #0x1101b538
if (cpu.x[20]) == 0 {pc=0x1101b538;continue;}pc=0x1101b524;continue;
}
0x1101b524=>{
// 0x1101b524 ldr x0, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x1101b528 cbz x0, #0x1101b530
if (cpu.x[0]) == 0 {pc=0x1101b530;continue;}pc=0x1101b52c;continue;
}
0x1101b52c=>{
// 0x1101b52c bl #0x1187d360
cpu.x[30]=0x1101b530;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x1101b530;continue;
}
0x1101b530=>{
// 0x1101b530 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x1101b534 bl #0x1187d2c0
cpu.x[30]=0x1101b538;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x1101b538;continue;
}
0x1101b538=>{
// 0x1101b538 ldr w8, [x19, #0xcc]
{let address=(cpu.x[19]).wrapping_add(0xccu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x1101b53c str xzr, [x19, #0xb8]
{let address=(cpu.x[19]).wrapping_add(0xb8u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x1101b540 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x1101b544 b.lt #0x1101b574
if cpu.condition(11){pc=0x1101b574;continue;}pc=0x1101b548;continue;
}
0x1101b548=>{
// 0x1101b548 mov x20, xzr
cpu.x[20]=(((0u64))) as u64;
// 0x1101b54c b #0x1101b55c
pc=0x1101b55c;continue;
}
0x1101b550=>{
// 0x1101b550 add x20, x20, #1
{let a=((cpu.x[20]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[20]=(result) as u64;}
// 0x1101b554 cmp x20, w8, sxtw
{let a=((cpu.x[20]));let b=(((cpu.x[8] as u32 as u64) as i32 as i64 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x1101b558 b.ge #0x1101b574
if cpu.condition(10){pc=0x1101b574;continue;}pc=0x1101b55c;continue;
}
0x1101b55c=>{
// 0x1101b55c ldr x9, [x19, #0xc0]
{let address=(cpu.x[19]).wrapping_add(0xc0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x1101b560 ldr x0, [x9, x20, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[20])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x1101b564 cbz x0, #0x1101b550
if (cpu.x[0]) == 0 {pc=0x1101b550;continue;}pc=0x1101b568;continue;
}
0x1101b568=>{
// 0x1101b568 bl #0x1187d2c0
cpu.x[30]=0x1101b56c;super::dispatch(cpu,rt,0x1187d2c0)?;
// 0x1101b56c ldr w8, [x19, #0xcc]
{let address=(cpu.x[19]).wrapping_add(0xccu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x1101b570 b #0x1101b550
pc=0x1101b550;continue;
}
0x1101b574=>{
// 0x1101b574 movi v0.2d, #0000000000000000
cpu.q[0]=(lane_dup(0u128,64,2)) & 0xffffffffffffffffffffffffffffffffu128;
// 0x1101b578 str wzr, [x19, #0xcc]
{let address=(cpu.x[19]).wrapping_add(0xccu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1101b57c str wzr, [x19, #0x3c]
{let address=(cpu.x[19]).wrapping_add(0x3cu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1101b580 str wzr, [x19, #0x9c]
{let address=(cpu.x[19]).wrapping_add(0x9cu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1101b584 stp xzr, xzr, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x1101b588 stp q0, q0, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x1101b58c stp q0, q0, [x21, #0x20]
{let address=(cpu.x[21]).wrapping_add(0x20u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x1101b590 str wzr, [x21, #0x40]
{let address=(cpu.x[21]).wrapping_add(0x40u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1101b594 stp xzr, xzr, [x22]
{let address=(cpu.x[22]).wrapping_add(0x0u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x1101b598 str wzr, [x22, #0x10]
{let address=(cpu.x[22]).wrapping_add(0x10u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1101b59c ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x1101b5a0 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x1101b5a4 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x1101b5a8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x1101b380")),}}}
#[inline]
pub(super) unsafe fn f_110ca464(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x110ca464=>{
// 0x110ca464 ldr x0, [x0, #0x1d8]
{let address=(cpu.x[0]).wrapping_add(0x1d8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110ca468 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x110ca464")),}}}
#[inline]
pub(super) unsafe fn f_110ca998(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x110ca998=>{
// 0x110ca998 ldr w0, [x0, #0x60]
{let address=(cpu.x[0]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x110ca99c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x110ca998")),}}}
#[inline]
pub(super) unsafe fn f_111d6364(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111d6364=>{
// 0x111d6364 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x111d6368 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d636c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x111d6370 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x111d6374 mov x20, x8
cpu.x[20]=(((cpu.x[8]))) as u64;
pc=0x111d6378;continue;
}
0x111d6378=>{
// 0x111d6378 mov x8, x20
cpu.x[8]=(((cpu.x[20]))) as u64;
// 0x111d637c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x111d6380 bl #0x111d6398
cpu.x[30]=0x111d6384;super::chunk15::f_111d6398(cpu,rt,0x111d6398)?;
pc=0x111d6384;continue;
}
0x111d6384=>{
// 0x111d6384 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d6388 cbz x8, #0x111d6378
if (cpu.x[8]) == 0 {pc=0x111d6378;continue;}pc=0x111d638c;continue;
}
0x111d638c=>{
// 0x111d638c ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d6390 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x111d6394 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111d6364")),}}}
#[inline]
pub(super) unsafe fn f_111d6ab4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111d6ab4=>{
// 0x111d6ab4 stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x111d6ab8 stp x24, x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d6abc stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d6ac0 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d6ac4 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x111d6ac8 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x111d6acc ldr x0, [x0, #0x20]
{let address=(cpu.x[0]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6ad0 bl #0x110ca998
cpu.x[30]=0x111d6ad4;super::chunk3::f_110ca998(cpu,rt,0x110ca998)?;
pc=0x111d6ad4;continue;
}
0x111d6ad4=>{
// 0x111d6ad4 ldr x8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d6ad8 mov w20, w0
cpu.x[20]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x111d6adc mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x111d6ae0 bl #0x1128bba8
cpu.x[30]=0x111d6ae4;super::chunk15::f_1128bba8(cpu,rt,0x1128bba8)?;
pc=0x111d6ae4;continue;
}
0x111d6ae4=>{
// 0x111d6ae4 ldr x0, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6ae8 bl #0x1128db58
cpu.x[30]=0x111d6aec;super::chunk6::f_1128db58(cpu,rt,0x1128db58)?;
pc=0x111d6aec;continue;
}
0x111d6aec=>{
// 0x111d6aec ldr x0, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6af0 ldr x1, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x111d6af4 mov w2, #1
cpu.x[2]=(((0x1u64))) as u32 as u64;
// 0x111d6af8 mov x3, xzr
cpu.x[3]=(((0u64))) as u64;
// 0x111d6afc mov w4, wzr
cpu.x[4]=(((0u64))) as u32 as u64;
// 0x111d6b00 mov w24, #1
cpu.x[24]=(((0x1u64))) as u32 as u64;
// 0x111d6b04 bl #0x110cba24
cpu.x[30]=0x111d6b08;super::chunk8::f_110cba24(cpu,rt,0x110cba24)?;
pc=0x111d6b08;continue;
}
0x111d6b08=>{
// 0x111d6b08 ldr x0, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6b0c ldr x1, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x111d6b10 mov w2, #1
cpu.x[2]=(((0x1u64))) as u32 as u64;
// 0x111d6b14 mov x3, xzr
cpu.x[3]=(((0u64))) as u64;
// 0x111d6b18 mov w4, wzr
cpu.x[4]=(((0u64))) as u32 as u64;
// 0x111d6b1c mov w5, wzr
cpu.x[5]=(((0u64))) as u32 as u64;
// 0x111d6b20 mov w6, wzr
cpu.x[6]=(((0u64))) as u32 as u64;
// 0x111d6b24 bl #0x110cb384
cpu.x[30]=0x111d6b28;super::chunk10::f_110cb384(cpu,rt,0x110cb384)?;
pc=0x111d6b28;continue;
}
0x111d6b28=>{
// 0x111d6b28 ldr x0, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6b2c bl #0x1128bc0c
cpu.x[30]=0x111d6b30;super::chunk12::f_1128bc0c(cpu,rt,0x1128bc0c)?;
pc=0x111d6b30;continue;
}
0x111d6b30=>{
// 0x111d6b30 ldr x0, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6b34 bl #0x1128dde8
cpu.x[30]=0x111d6b38;super::chunk11::f_1128dde8(cpu,rt,0x1128dde8)?;
pc=0x111d6b38;continue;
}
0x111d6b38=>{
// 0x111d6b38 ldr x8, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d6b3c ldr x8, [x8, #0xa8]
{let address=(cpu.x[8]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d6b40 cbz x8, #0x111d6cb4
if (cpu.x[8]) == 0 {pc=0x111d6cb4;continue;}pc=0x111d6b44;continue;
}
0x111d6b44=>{
// 0x111d6b44 ldr w8, [x19, #0x58]
{let address=(cpu.x[19]).wrapping_add(0x58u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d6b48 ldr x0, [x19, #0x50]
{let address=(cpu.x[19]).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6b4c cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d6b50 b.le #0x111d6b5c
if cpu.condition(13){pc=0x111d6b5c;continue;}pc=0x111d6b54;continue;
}
0x111d6b54=>{
// 0x111d6b54 cbnz x0, #0x111d6c1c
if (cpu.x[0]) != 0 {pc=0x111d6c1c;continue;}pc=0x111d6b58;continue;
}
0x111d6b58=>{
// 0x111d6b58 b #0x111d6b80
pc=0x111d6b80;continue;
}
0x111d6b5c=>{
// 0x111d6b5c cbz x0, #0x111d6b80
if (cpu.x[0]) == 0 {pc=0x111d6b80;continue;}pc=0x111d6b60;continue;
}
0x111d6b60=>{
// 0x111d6b60 bl #0x110c9a30
cpu.x[30]=0x111d6b64;super::chunk15::f_110c9a30(cpu,rt,0x110c9a30)?;
pc=0x111d6b64;continue;
}
0x111d6b64=>{
// 0x111d6b64 ldr x21, [x19, #0x50]
{let address=(cpu.x[19]).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x111d6b68 cbz x21, #0x111d6b7c
if (cpu.x[21]) == 0 {pc=0x111d6b7c;continue;}pc=0x111d6b6c;continue;
}
0x111d6b6c=>{
// 0x111d6b6c mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x111d6b70 bl #0x1099b50c
cpu.x[30]=0x111d6b74;super::chunk13::f_1099b50c(cpu,rt,0x1099b50c)?;
pc=0x111d6b74;continue;
}
0x111d6b74=>{
// 0x111d6b74 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x111d6b78 bl #0x1187d2c0
cpu.x[30]=0x111d6b7c;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x111d6b7c;continue;
}
0x111d6b7c=>{
// 0x111d6b7c str xzr, [x19, #0x50]
{let address=(cpu.x[19]).wrapping_add(0x50u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x111d6b80;continue;
}
0x111d6b80=>{
// 0x111d6b80 ldr x0, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6b84 bl #0x1128bba8
cpu.x[30]=0x111d6b88;super::chunk15::f_1128bba8(cpu,rt,0x1128bba8)?;
pc=0x111d6b88;continue;
}
0x111d6b88=>{
// 0x111d6b88 ldr x0, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6b8c bl #0x1128db58
cpu.x[30]=0x111d6b90;super::chunk6::f_1128db58(cpu,rt,0x1128db58)?;
pc=0x111d6b90;continue;
}
0x111d6b90=>{
// 0x111d6b90 bl #0x10f07778
cpu.x[30]=0x111d6b94;super::chunk9::f_10f07778(cpu,rt,0x10f07778)?;
pc=0x111d6b94;continue;
}
0x111d6b94=>{
// 0x111d6b94 ldr x21, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x111d6b98 mov x22, x0
cpu.x[22]=(((cpu.x[0]))) as u64;
// 0x111d6b9c mov w0, #0x238
cpu.x[0]=(((0x238u64))) as u32 as u64;
// 0x111d6ba0 bl #0x1187d2b0
cpu.x[30]=0x111d6ba4;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
pc=0x111d6ba4;continue;
}
0x111d6ba4=>{
// 0x111d6ba4 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x111d6ba8 mov x2, x21
cpu.x[2]=(((cpu.x[21]))) as u64;
// 0x111d6bac mov x3, x22
cpu.x[3]=(((cpu.x[22]))) as u64;
// 0x111d6bb0 mov w4, wzr
cpu.x[4]=(((0u64))) as u32 as u64;
// 0x111d6bb4 mov x23, x0
cpu.x[23]=(((cpu.x[0]))) as u64;
// 0x111d6bb8 bl #0x110c9514
cpu.x[30]=0x111d6bbc;super::chunk5::f_110c9514(cpu,rt,0x110c9514)?;
pc=0x111d6bbc;continue;
}
0x111d6bbc=>{
// 0x111d6bbc ldr x1, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x111d6bc0 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x111d6bc4 str x23, [x19, #0x50]
{let address=(cpu.x[19]).wrapping_add(0x50u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111d6bc8 bl #0x110cbc34
cpu.x[30]=0x111d6bcc;super::chunk12::f_110cbc34(cpu,rt,0x110cbc34)?;
pc=0x111d6bcc;continue;
}
0x111d6bcc=>{
// 0x111d6bcc ldr x0, [x19, #0x50]
{let address=(cpu.x[19]).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6bd0 ldr x1, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x111d6bd4 mov w2, #1
cpu.x[2]=(((0x1u64))) as u32 as u64;
// 0x111d6bd8 mov x3, xzr
cpu.x[3]=(((0u64))) as u64;
// 0x111d6bdc bl #0x110cbf18
cpu.x[30]=0x111d6be0;super::chunk6::f_110cbf18(cpu,rt,0x110cbf18)?;
pc=0x111d6be0;continue;
}
0x111d6be0=>{
// 0x111d6be0 ldr x0, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6be4 bl #0x1128bc0c
cpu.x[30]=0x111d6be8;super::chunk12::f_1128bc0c(cpu,rt,0x1128bc0c)?;
pc=0x111d6be8;continue;
}
0x111d6be8=>{
// 0x111d6be8 ldr x0, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6bec mov w1, #0x51
cpu.x[1]=(((0x51u64))) as u32 as u64;
// 0x111d6bf0 bl #0x11294bb4
cpu.x[30]=0x111d6bf4;super::chunk13::f_11294bb4(cpu,rt,0x11294bb4)?;
pc=0x111d6bf4;continue;
}
0x111d6bf4=>{
// 0x111d6bf4 mov w8, w0
cpu.x[8]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x111d6bf8 ldr x0, [x19, #0x50]
{let address=(cpu.x[19]).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6bfc add w8, w8, #0x14
{let a=((cpu.x[8] as u32 as u64));let b=((0x14u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
pc=0x111d6c00;continue;
}
0x111d6c00=>{
// 0x111d6c00 b #0x111d6c18
pc=0x111d6c18;continue;
}
0x111d6c04=>{
// 0x111d6c04 ldr x0, [x19, #0x50]
{let address=(cpu.x[19]).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6c08 bl #0x110ca81c
cpu.x[30]=0x111d6c0c;super::chunk13::f_110ca81c(cpu,rt,0x110ca81c)?;
pc=0x111d6c0c;continue;
}
0x111d6c0c=>{
// 0x111d6c0c ldr w8, [x19, #0x58]
{let address=(cpu.x[19]).wrapping_add(0x58u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d6c10 ldr x0, [x19, #0x50]
{let address=(cpu.x[19]).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6c14 sub w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
pc=0x111d6c18;continue;
}
0x111d6c18=>{
// 0x111d6c18 str w8, [x19, #0x58]
{let address=(cpu.x[19]).wrapping_add(0x58u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x111d6c1c;continue;
}
0x111d6c1c=>{
// 0x111d6c1c bl #0x110ca998
cpu.x[30]=0x111d6c20;super::chunk3::f_110ca998(cpu,rt,0x110ca998)?;
pc=0x111d6c20;continue;
}
0x111d6c20=>{
// 0x111d6c20 cmp w0, w20
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[20] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d6c24 b.lt #0x111d6c04
if cpu.condition(11){pc=0x111d6c04;continue;}pc=0x111d6c28;continue;
}
0x111d6c28=>{
// 0x111d6c28 ldr x0, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6c2c bl #0x1128bba8
cpu.x[30]=0x111d6c30;super::chunk15::f_1128bba8(cpu,rt,0x1128bba8)?;
pc=0x111d6c30;continue;
}
0x111d6c30=>{
// 0x111d6c30 ldr x0, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6c34 bl #0x1128db58
cpu.x[30]=0x111d6c38;super::chunk6::f_1128db58(cpu,rt,0x1128db58)?;
pc=0x111d6c38;continue;
}
0x111d6c38=>{
// 0x111d6c38 ldr x0, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6c3c bl #0x1128bba8
cpu.x[30]=0x111d6c40;super::chunk15::f_1128bba8(cpu,rt,0x1128bba8)?;
pc=0x111d6c40;continue;
}
0x111d6c40=>{
// 0x111d6c40 ldr x0, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6c44 bl #0x1128db58
cpu.x[30]=0x111d6c48;super::chunk6::f_1128db58(cpu,rt,0x1128db58)?;
pc=0x111d6c48;continue;
}
0x111d6c48=>{
// 0x111d6c48 ldr x0, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6c4c bl #0x111e7578
cpu.x[30]=0x111d6c50;super::chunk6::f_111e7578(cpu,rt,0x111e7578)?;
pc=0x111d6c50;continue;
}
0x111d6c50=>{
// 0x111d6c50 ldp x1, x0, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[1]=(v0) as u64;cpu.x[0]=(v1) as u64;}
// 0x111d6c54 bl #0x111e5850
cpu.x[30]=0x111d6c58;super::chunk3::f_111e5850(cpu,rt,0x111e5850)?;
pc=0x111d6c58;continue;
}
0x111d6c58=>{
// 0x111d6c58 ldr x1, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x111d6c5c ldr x0, [x19, #0x50]
{let address=(cpu.x[19]).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6c60 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x111d6c64 mov w2, #1
cpu.x[2]=(((0x1u64))) as u32 as u64;
// 0x111d6c68 mov x3, xzr
cpu.x[3]=(((0u64))) as u64;
// 0x111d6c6c mov w4, wzr
cpu.x[4]=(((0u64))) as u32 as u64;
// 0x111d6c70 strb w8, [x1, #0x80]
{let address=(cpu.x[1]).wrapping_add(0x80u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x111d6c74 bl #0x110cba24
cpu.x[30]=0x111d6c78;super::chunk8::f_110cba24(cpu,rt,0x110cba24)?;
pc=0x111d6c78;continue;
}
0x111d6c78=>{
// 0x111d6c78 ldr x0, [x19, #0x50]
{let address=(cpu.x[19]).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6c7c ldr x1, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x111d6c80 mov w2, #1
cpu.x[2]=(((0x1u64))) as u32 as u64;
// 0x111d6c84 mov x3, xzr
cpu.x[3]=(((0u64))) as u64;
// 0x111d6c88 mov w4, wzr
cpu.x[4]=(((0u64))) as u32 as u64;
// 0x111d6c8c mov w5, wzr
cpu.x[5]=(((0u64))) as u32 as u64;
// 0x111d6c90 mov w6, wzr
cpu.x[6]=(((0u64))) as u32 as u64;
// 0x111d6c94 bl #0x110cb384
cpu.x[30]=0x111d6c98;super::chunk10::f_110cb384(cpu,rt,0x110cb384)?;
pc=0x111d6c98;continue;
}
0x111d6c98=>{
// 0x111d6c98 ldr x0, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6c9c bl #0x1128bc0c
cpu.x[30]=0x111d6ca0;super::chunk12::f_1128bc0c(cpu,rt,0x1128bc0c)?;
pc=0x111d6ca0;continue;
}
0x111d6ca0=>{
// 0x111d6ca0 ldr x0, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6ca4 bl #0x1128bba8
cpu.x[30]=0x111d6ca8;super::chunk15::f_1128bba8(cpu,rt,0x1128bba8)?;
pc=0x111d6ca8;continue;
}
0x111d6ca8=>{
// 0x111d6ca8 ldr x0, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6cac mov x1, xzr
cpu.x[1]=(((0u64))) as u64;
// 0x111d6cb0 bl #0x111e5850
cpu.x[30]=0x111d6cb4;super::chunk3::f_111e5850(cpu,rt,0x111e5850)?;
pc=0x111d6cb4;continue;
}
0x111d6cb4=>{
// 0x111d6cb4 ldr x0, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6cb8 bl #0x1128bba8
cpu.x[30]=0x111d6cbc;super::chunk15::f_1128bba8(cpu,rt,0x1128bba8)?;
pc=0x111d6cbc;continue;
}
0x111d6cbc=>{
// 0x111d6cbc ldr x0, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6cc0 bl #0x1128db58
cpu.x[30]=0x111d6cc4;super::chunk6::f_1128db58(cpu,rt,0x1128db58)?;
pc=0x111d6cc4;continue;
}
0x111d6cc4=>{
// 0x111d6cc4 ldr x0, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6cc8 ldr x1, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x111d6ccc bl #0x110cbc34
cpu.x[30]=0x111d6cd0;super::chunk12::f_110cbc34(cpu,rt,0x110cbc34)?;
pc=0x111d6cd0;continue;
}
0x111d6cd0=>{
// 0x111d6cd0 ldr x0, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6cd4 ldr x1, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x111d6cd8 mov w2, #1
cpu.x[2]=(((0x1u64))) as u32 as u64;
// 0x111d6cdc mov x3, xzr
cpu.x[3]=(((0u64))) as u64;
// 0x111d6ce0 bl #0x110cbf18
cpu.x[30]=0x111d6ce4;super::chunk6::f_110cbf18(cpu,rt,0x110cbf18)?;
pc=0x111d6ce4;continue;
}
0x111d6ce4=>{
// 0x111d6ce4 ldr x0, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6ce8 bl #0x1128bc0c
cpu.x[30]=0x111d6cec;super::chunk12::f_1128bc0c(cpu,rt,0x1128bc0c)?;
pc=0x111d6cec;continue;
}
0x111d6cec=>{
// 0x111d6cec ldr x0, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6cf0 bl #0x1128bba8
cpu.x[30]=0x111d6cf4;super::chunk15::f_1128bba8(cpu,rt,0x1128bba8)?;
pc=0x111d6cf4;continue;
}
0x111d6cf4=>{
// 0x111d6cf4 ldr x0, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6cf8 bl #0x1128db58
cpu.x[30]=0x111d6cfc;super::chunk6::f_1128db58(cpu,rt,0x1128db58)?;
pc=0x111d6cfc;continue;
}
0x111d6cfc=>{
// 0x111d6cfc ldr x0, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6d00 bl #0x1128bba8
cpu.x[30]=0x111d6d04;super::chunk15::f_1128bba8(cpu,rt,0x1128bba8)?;
pc=0x111d6d04;continue;
}
0x111d6d04=>{
// 0x111d6d04 ldr x0, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6d08 bl #0x1128db58
cpu.x[30]=0x111d6d0c;super::chunk6::f_1128db58(cpu,rt,0x1128db58)?;
pc=0x111d6d0c;continue;
}
0x111d6d0c=>{
// 0x111d6d0c ldr x0, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6d10 bl #0x111e7578
cpu.x[30]=0x111d6d14;super::chunk6::f_111e7578(cpu,rt,0x111e7578)?;
pc=0x111d6d14;continue;
}
0x111d6d14=>{
// 0x111d6d14 ldp x1, x0, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[1]=(v0) as u64;cpu.x[0]=(v1) as u64;}
// 0x111d6d18 bl #0x111e5850
cpu.x[30]=0x111d6d1c;super::chunk3::f_111e5850(cpu,rt,0x111e5850)?;
pc=0x111d6d1c;continue;
}
0x111d6d1c=>{
// 0x111d6d1c ldr x1, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x111d6d20 ldr x0, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6d24 mov w2, #1
cpu.x[2]=(((0x1u64))) as u32 as u64;
// 0x111d6d28 mov x3, xzr
cpu.x[3]=(((0u64))) as u64;
// 0x111d6d2c mov w4, wzr
cpu.x[4]=(((0u64))) as u32 as u64;
// 0x111d6d30 strb w24, [x1, #0x80]
{let address=(cpu.x[1]).wrapping_add(0x80u64);let v0=cpu.x[24] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x111d6d34 bl #0x110cba24
cpu.x[30]=0x111d6d38;super::chunk8::f_110cba24(cpu,rt,0x110cba24)?;
pc=0x111d6d38;continue;
}
0x111d6d38=>{
// 0x111d6d38 ldr x0, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6d3c ldr x1, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x111d6d40 mov w2, #1
cpu.x[2]=(((0x1u64))) as u32 as u64;
// 0x111d6d44 mov x3, xzr
cpu.x[3]=(((0u64))) as u64;
// 0x111d6d48 mov w4, wzr
cpu.x[4]=(((0u64))) as u32 as u64;
// 0x111d6d4c mov w5, wzr
cpu.x[5]=(((0u64))) as u32 as u64;
// 0x111d6d50 mov w6, wzr
cpu.x[6]=(((0u64))) as u32 as u64;
// 0x111d6d54 bl #0x110cb384
cpu.x[30]=0x111d6d58;super::chunk10::f_110cb384(cpu,rt,0x110cb384)?;
pc=0x111d6d58;continue;
}
0x111d6d58=>{
// 0x111d6d58 ldr x0, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6d5c bl #0x1128bc0c
cpu.x[30]=0x111d6d60;super::chunk12::f_1128bc0c(cpu,rt,0x1128bc0c)?;
pc=0x111d6d60;continue;
}
0x111d6d60=>{
// 0x111d6d60 ldr x0, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6d64 bl #0x1128dde8
cpu.x[30]=0x111d6d68;super::chunk11::f_1128dde8(cpu,rt,0x1128dde8)?;
pc=0x111d6d68;continue;
}
0x111d6d68=>{
// 0x111d6d68 ldr x0, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6d6c bl #0x1128bba8
cpu.x[30]=0x111d6d70;super::chunk15::f_1128bba8(cpu,rt,0x1128bba8)?;
pc=0x111d6d70;continue;
}
0x111d6d70=>{
// 0x111d6d70 ldr x0, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6d74 mov x1, xzr
cpu.x[1]=(((0u64))) as u64;
// 0x111d6d78 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d6d7c ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d6d80 ldp x24, x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x111d6d84 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x111d6d88 b #0x111e5850
super::chunk3::f_111e5850(cpu,rt,0x111e5850)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111d6ab4")),}}}
#[inline]
pub(super) unsafe fn f_111d6f18(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111d6f18=>{
// 0x111d6f18 sub sp, sp, #0x60
{let a=((cpu.sp));let b=((0x60u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x111d6f1c stp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d6f20 str x25, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111d6f24 stp x24, x23, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d6f28 stp x22, x21, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d6f2c stp x20, x19, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d6f30 add x29, sp, #0x10
{let a=((cpu.sp));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x111d6f34 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x111d6f38 mov w21, #8
cpu.x[21]=(((0x8u64))) as u32 as u64;
// 0x111d6f3c mov w0, #0x40
cpu.x[0]=(((0x40u64))) as u32 as u64;
// 0x111d6f40 mov x20, x8
cpu.x[20]=(((cpu.x[8]))) as u64;
// 0x111d6f44 str x21, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111d6f48 bl #0x1187d520
cpu.x[30]=0x111d6f4c;super::chunk8::f_1187d520(cpu,rt,0x1187d520)?;
pc=0x111d6f4c;continue;
}
0x111d6f4c=>{
// 0x111d6f4c ldr x8, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d6f50 str x0, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111d6f54 ldr w8, [x8, #0x7c]
{let address=(cpu.x[8]).wrapping_add(0x7cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d6f58 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d6f5c b.lt #0x111d7000
if cpu.condition(11){pc=0x111d7000;continue;}pc=0x111d6f60;continue;
}
0x111d6f60=>{
// 0x111d6f60 lsl x23, x8, #3
cpu.x[23]=(((((cpu.x[8]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x111d6f64 mov x22, xzr
cpu.x[22]=(((0u64))) as u64;
// 0x111d6f68 mov w2, wzr
cpu.x[2]=(((0u64))) as u32 as u64;
// 0x111d6f6c mov w24, #5
cpu.x[24]=(((0x5u64))) as u32 as u64;
// 0x111d6f70 b #0x111d6f94
pc=0x111d6f94;continue;
}
0x111d6f74=>{
// 0x111d6f74 ldr x9, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111d6f78 add w2, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[2]=(result) as u32 as u64;}
// 0x111d6f7c ldr x10, [x29, #0x18]
{let address=(cpu.x[29]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x111d6f80 str w2, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=cpu.x[2] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111d6f84 str x10, [x9, w8, sxtw #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64);let v0=cpu.x[10];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x111d6f88;continue;
}
0x111d6f88=>{
// 0x111d6f88 add x22, x22, #8
{let a=((cpu.x[22]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[22]=(result) as u64;}
// 0x111d6f8c cmp x23, x22
{let a=((cpu.x[23]));let b=((cpu.x[22]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x111d6f90 b.eq #0x111d6ff4
if cpu.condition(0){pc=0x111d6ff4;continue;}pc=0x111d6f94;continue;
}
0x111d6f94=>{
// 0x111d6f94 ldr x8, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d6f98 ldr x8, [x8, #0x70]
{let address=(cpu.x[8]).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d6f9c ldr x1, [x8, x22]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[22])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x111d6fa0 cbz x1, #0x111d6f88
if (cpu.x[1]) == 0 {pc=0x111d6f88;continue;}pc=0x111d6fa4;continue;
}
0x111d6fa4=>{
// 0x111d6fa4 add x8, x29, #0x18
{let a=((cpu.x[29]));let b=((0x18u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x111d6fa8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x111d6fac bl #0x111d7250
cpu.x[30]=0x111d6fb0;super::chunk11::f_111d7250(cpu,rt,0x111d7250)?;
pc=0x111d6fb0;continue;
}
0x111d6fb0=>{
// 0x111d6fb0 ldp w9, w8, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x111d6fb4 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d6fb8 b.ne #0x111d6f74
if cpu.condition(1){pc=0x111d6f74;continue;}pc=0x111d6fbc;continue;
}
0x111d6fbc=>{
// 0x111d6fbc lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x111d6fc0 ldr x25, [x29, #0x18]
{let address=(cpu.x[29]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[25]=(v0) as u64;}
// 0x111d6fc4 cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d6fc8 csel w1, w24, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[24] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x111d6fcc cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d6fd0 b.ge #0x111d6fe0
if cpu.condition(10){pc=0x111d6fe0;continue;}pc=0x111d6fd4;continue;
}
0x111d6fd4=>{
// 0x111d6fd4 mov x0, sp
cpu.x[0]=(((cpu.sp))) as u64;
// 0x111d6fd8 bl #0x108afed8
cpu.x[30]=0x111d6fdc;super::dispatch(cpu,rt,0x108afed8)?;
// 0x111d6fdc ldr w8, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
pc=0x111d6fe0;continue;
}
0x111d6fe0=>{
// 0x111d6fe0 ldr x9, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111d6fe4 add w2, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[2]=(result) as u32 as u64;}
// 0x111d6fe8 str w2, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=cpu.x[2] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111d6fec str x25, [x9, w8, sxtw #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64);let v0=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111d6ff0 b #0x111d6f88
pc=0x111d6f88;continue;
}
0x111d6ff4=>{
// 0x111d6ff4 cmp w2, #8
{let a=((cpu.x[2] as u32 as u64));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d6ff8 b.lt #0x111d7004
if cpu.condition(11){pc=0x111d7004;continue;}pc=0x111d6ffc;continue;
}
0x111d6ffc=>{
// 0x111d6ffc b #0x111d702c
pc=0x111d702c;continue;
}
0x111d7000=>{
// 0x111d7000 mov w2, wzr
cpu.x[2]=(((0u64))) as u32 as u64;
pc=0x111d7004;continue;
}
0x111d7004=>{
// 0x111d7004 ldr x8, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d7008 sub w2, w21, w2
{let a=((cpu.x[21] as u32 as u64));let b=((cpu.x[2] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[2]=(result) as u32 as u64;}
// 0x111d700c mov x1, sp
cpu.x[1]=(((cpu.sp))) as u64;
// 0x111d7010 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x111d7014 ldr w8, [x8, #0x64]
{let address=(cpu.x[8]).wrapping_add(0x64u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d7018 cmp w8, #2
{let a=((cpu.x[8] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d701c b.ne #0x111d7028
if cpu.condition(1){pc=0x111d7028;continue;}pc=0x111d7020;continue;
}
0x111d7020=>{
// 0x111d7020 bl #0x111d74d4
cpu.x[30]=0x111d7024;super::dispatch(cpu,rt,0x111d74d4)?;
// 0x111d7024 b #0x111d702c
pc=0x111d702c;continue;
}
0x111d7028=>{
// 0x111d7028 bl #0x111d6d8c
cpu.x[30]=0x111d702c;super::dispatch(cpu,rt,0x111d6d8c)?;
pc=0x111d702c;continue;
}
0x111d702c=>{
// 0x111d702c mov w0, #0x10
cpu.x[0]=(((0x10u64))) as u32 as u64;
// 0x111d7030 bl #0x1187d2b0
cpu.x[30]=0x111d7034;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
pc=0x111d7034;continue;
}
0x111d7034=>{
// 0x111d7034 mov w8, #8
cpu.x[8]=(((0x8u64))) as u32 as u64;
// 0x111d7038 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x111d703c str x8, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111d7040 mov w0, #0x40
cpu.x[0]=(((0x40u64))) as u32 as u64;
// 0x111d7044 bl #0x1187d520
cpu.x[30]=0x111d7048;super::chunk8::f_1187d520(cpu,rt,0x1187d520)?;
pc=0x111d7048;continue;
}
0x111d7048=>{
// 0x111d7048 mov w22, #5
cpu.x[22]=(((0x5u64))) as u32 as u64;
// 0x111d704c str x0, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111d7050 b #0x111d706c
pc=0x111d706c;continue;
}
0x111d7054=>{
// 0x111d7054 ldr x9, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111d7058 add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x111d705c cmp w8, #7
{let a=((cpu.x[8] as u32 as u64));let b=((0x7u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d7060 str w10, [x21, #0xc]
{let address=(cpu.x[21]).wrapping_add(0xcu64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111d7064 str x23, [x9, w8, sxtw #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111d7068 b.ge #0x111d7140
if cpu.condition(10){pc=0x111d7140;continue;}pc=0x111d706c;continue;
}
0x111d706c=>{
// 0x111d706c ldr x0, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d7070 ldr w1, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x111d7074 bl #0x11294bb4
cpu.x[30]=0x111d7078;super::chunk13::f_11294bb4(cpu,rt,0x11294bb4)?;
pc=0x111d7078;continue;
}
0x111d7078=>{
// 0x111d7078 ldr x10, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x111d707c ldr w11, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
// 0x111d7080 add x8, x10, w0, sxtw #3
{let a=((cpu.x[10]));let b=((((cpu.x[0] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x111d7084 sub w12, w11, #1
{let a=((cpu.x[11] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[12]=(result) as u32 as u64;}
// 0x111d7088 subs w9, w12, w0
{let a=((cpu.x[12] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x111d708c str w12, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=cpu.x[12] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111d7090 ldr x23, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d7094 b.le #0x111d7110
if cpu.condition(13){pc=0x111d7110;continue;}pc=0x111d7098;continue;
}
0x111d7098=>{
// 0x111d7098 cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d709c b.lt #0x111d7110
if cpu.condition(11){pc=0x111d7110;continue;}pc=0x111d70a0;continue;
}
0x111d70a0=>{
// 0x111d70a0 sxtw x13, w0
cpu.x[13]=((((cpu.x[0] as u32 as u64))) as i32 as i64 as u64) as u64;
// 0x111d70a4 sub w11, w11, w13
{let a=((cpu.x[11] as u32 as u64));let b=((cpu.x[13] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[11]=(result) as u32 as u64;}
// 0x111d70a8 sub w11, w11, #2
{let a=((cpu.x[11] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.x[11]=(result) as u32 as u64;}
// 0x111d70ac cmp w11, #3
{let a=((cpu.x[11] as u32 as u64));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d70b0 b.lo #0x111d70f0
if cpu.condition(3){pc=0x111d70f0;continue;}pc=0x111d70b4;continue;
}
0x111d70b4=>{
// 0x111d70b4 add x11, x11, #1
{let a=((cpu.x[11]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
// 0x111d70b8 add x10, x10, x13, lsl #3
{let a=((cpu.x[10]));let b=(((cpu.x[13])) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x111d70bc and x12, x11, #0x1fffffffc
{let a=((cpu.x[11]));let b=((0x1fffffffcu64));let result=a & b;cpu.x[12]=(result) as u64;}
// 0x111d70c0 add x8, x8, x12, lsl #3
{let a=((cpu.x[8]));let b=(((cpu.x[12])) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x111d70c4 sub w9, w9, w12
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[12] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x111d70c8 add x10, x10, #0x10
{let a=((cpu.x[10]));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x111d70cc mov x13, x12
cpu.x[13]=(((cpu.x[12]))) as u64;
pc=0x111d70d0;continue;
}
0x111d70d0=>{
// 0x111d70d0 ldur q0, [x10, #-8]
{let address=(cpu.x[10]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));cpu.q[0]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x111d70d4 ldur q1, [x10, #8]
{let address=(cpu.x[10]).wrapping_add(0x8u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));cpu.q[1]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x111d70d8 subs x13, x13, #4
{let a=((cpu.x[13]));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.x[13]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x111d70dc stp q0, q1, [x10, #-0x10]
{let address=(cpu.x[10]).wrapping_add(0xfffffffffffffff0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[1] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x111d70e0 add x10, x10, #0x20
{let a=((cpu.x[10]));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x111d70e4 b.ne #0x111d70d0
if cpu.condition(1){pc=0x111d70d0;continue;}pc=0x111d70e8;continue;
}
0x111d70e8=>{
// 0x111d70e8 cmp x11, x12
{let a=((cpu.x[11]));let b=((cpu.x[12]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x111d70ec b.eq #0x111d7110
if cpu.condition(0){pc=0x111d7110;continue;}pc=0x111d70f0;continue;
}
0x111d70f0=>{
// 0x111d70f0 add w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x111d70f4 add x8, x8, #8
{let a=((cpu.x[8]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
pc=0x111d70f8;continue;
}
0x111d70f8=>{
// 0x111d70f8 ldr x10, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x111d70fc sub w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x111d7100 cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d7104 stur x10, [x8, #-8]
{let address=(cpu.x[8]).wrapping_add(0xfffffffffffffff8u64);let v0=cpu.x[10];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111d7108 add x8, x8, #8
{let a=((cpu.x[8]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x111d710c b.hi #0x111d70f8
if cpu.condition(8){pc=0x111d70f8;continue;}pc=0x111d7110;continue;
}
0x111d7110=>{
// 0x111d7110 ldp w9, w8, [x21, #8]
{let address=(cpu.x[21]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x111d7114 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d7118 b.ne #0x111d7054
if cpu.condition(1){pc=0x111d7054;continue;}pc=0x111d711c;continue;
}
0x111d711c=>{
// 0x111d711c lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x111d7120 cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d7124 csel w1, w22, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[22] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x111d7128 cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d712c b.ge #0x111d7054
if cpu.condition(10){pc=0x111d7054;continue;}pc=0x111d7130;continue;
}
0x111d7130=>{
// 0x111d7130 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x111d7134 bl #0x108c5574
cpu.x[30]=0x111d7138;super::chunk12::f_108c5574(cpu,rt,0x108c5574)?;
// 0x111d7138 ldr w8, [x21, #0xc]
{let address=(cpu.x[21]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d713c b #0x111d7054
pc=0x111d7054;continue;
}
0x111d7140=>{
// 0x111d7140 ldr w8, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d7144 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d7148 b.ge #0x111d7214
if cpu.condition(10){pc=0x111d7214;continue;}pc=0x111d714c;continue;
}
0x111d714c=>{
// 0x111d714c ldr x22, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[22]=(v0) as u64;}
// 0x111d7150 mov w0, #0x58
cpu.x[0]=(((0x58u64))) as u32 as u64;
// 0x111d7154 bl #0x1187d2b0
cpu.x[30]=0x111d7158;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
pc=0x111d7158;continue;
}
0x111d7158=>{
// 0x111d7158 mov x1, x22
cpu.x[1]=(((cpu.x[22]))) as u64;
// 0x111d715c mov x2, x21
cpu.x[2]=(((cpu.x[21]))) as u64;
// 0x111d7160 mov x23, x0
cpu.x[23]=(((cpu.x[0]))) as u64;
// 0x111d7164 bl #0x10fea548
cpu.x[30]=0x111d7168;super::chunk15::f_10fea548(cpu,rt,0x10fea548)?;
pc=0x111d7168;continue;
}
0x111d7168=>{
// 0x111d7168 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d716c str x23, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111d7170 ldr x24, [x8, #0x870]
{let address=(cpu.x[8]).wrapping_add(0x870u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[24]=(v0) as u64;}
// 0x111d7174 ldr x0, [x24, #0xa0]
{let address=(cpu.x[24]).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d7178 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d717c ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d7180 blr x8
{let target=cpu.x[8];cpu.x[30]=0x111d7184;super::dispatch(cpu,rt,target)?;}
pc=0x111d7184;continue;
}
0x111d7184=>{
// 0x111d7184 ldr x19, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x111d7188 mov w20, w0
cpu.x[20]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x111d718c mov w0, #0x30
cpu.x[0]=(((0x30u64))) as u32 as u64;
// 0x111d7190 bl #0x1187d2b0
cpu.x[30]=0x111d7194;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
pc=0x111d7194;continue;
}
0x111d7194=>{
// 0x111d7194 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x111d7198 mov x22, x0
cpu.x[22]=(((cpu.x[0]))) as u64;
// 0x111d719c bl #0x11058f84
cpu.x[30]=0x111d71a0;super::chunk4::f_11058f84(cpu,rt,0x11058f84)?;
pc=0x111d71a0;continue;
}
0x111d71a0=>{
// 0x111d71a0 ldr x8, [x22]
{let address=(cpu.x[22]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d71a4 str x24, [x22, #0x18]
{let address=(cpu.x[22]).wrapping_add(0x18u64);let v0=cpu.x[24];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111d71a8 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x111d71ac mov w1, w20
cpu.x[1]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x111d71b0 ldr x8, [x8, #0x38]
{let address=(cpu.x[8]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d71b4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x111d71b8;super::dispatch(cpu,rt,target)?;}
pc=0x111d71b8;continue;
}
0x111d71b8=>{
// 0x111d71b8 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d71bc mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x111d71c0 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x111d71c4 mov x2, x22
cpu.x[2]=(((cpu.x[22]))) as u64;
// 0x111d71c8 ldr x8, [x8, #0x130]
{let address=(cpu.x[8]).wrapping_add(0x130u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d71cc blr x8
{let target=cpu.x[8];cpu.x[30]=0x111d71d0;super::dispatch(cpu,rt,target)?;}
pc=0x111d71d0;continue;
}
0x111d71d0=>{
// 0x111d71d0 ldr x0, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d71d4 cbz x0, #0x111d71dc
if (cpu.x[0]) == 0 {pc=0x111d71dc;continue;}pc=0x111d71d8;continue;
}
0x111d71d8=>{
// 0x111d71d8 bl #0x1187d360
cpu.x[30]=0x111d71dc;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x111d71dc;continue;
}
0x111d71dc=>{
// 0x111d71dc mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x111d71e0 bl #0x1187d2c0
cpu.x[30]=0x111d71e4;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x111d71e4;continue;
}
0x111d71e4=>{
// 0x111d71e4 ldr x0, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d71e8 cbz x0, #0x111d71f0
if (cpu.x[0]) == 0 {pc=0x111d71f0;continue;}pc=0x111d71ec;continue;
}
0x111d71ec=>{
// 0x111d71ec bl #0x1187d360
cpu.x[30]=0x111d71f0;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x111d71f0;continue;
}
0x111d71f0=>{
// 0x111d71f0 ldp x20, x19, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d71f4 ldr x25, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[25]=(v0) as u64;}
// 0x111d71f8 ldp x22, x21, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d71fc ldp x24, x23, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x111d7200 ldp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x111d7204 add sp, sp, #0x60
{let a=((cpu.sp));let b=((0x60u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x111d7208 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x111d720c=>{
// 0x111d720c cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d7210 b.le #0x111d714c
if cpu.condition(13){pc=0x111d714c;continue;}pc=0x111d7214;continue;
}
0x111d7214=>{
// 0x111d7214 ldr x9, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111d7218 sub w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x111d721c str w8, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111d7220 ldr x22, [x9, w8, uxtw #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as u32 as u64)) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[22]=(v0) as u64;}
// 0x111d7224 cbz x22, #0x111d720c
if (cpu.x[22]) == 0 {pc=0x111d720c;continue;}pc=0x111d7228;continue;
}
0x111d7228=>{
// 0x111d7228 ldr x8, [x22]
{let address=(cpu.x[22]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d722c mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x111d7230 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d7234 blr x8
{let target=cpu.x[8];cpu.x[30]=0x111d7238;super::dispatch(cpu,rt,target)?;}
// 0x111d7238 ldr x8, [x22]
{let address=(cpu.x[22]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d723c mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x111d7240 ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d7244 blr x8
{let target=cpu.x[8];cpu.x[30]=0x111d7248;super::dispatch(cpu,rt,target)?;}
// 0x111d7248 ldr w8, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d724c b #0x111d720c
pc=0x111d720c;continue;
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111d6f18")),}}}
#[inline]
pub(super) unsafe fn f_111d7b58(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111d7b58=>{
// 0x111d7b58 stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x111d7b5c str x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111d7b60 stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d7b64 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d7b68 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x111d7b6c ldp w8, w9, [x0, #0x54]
{let address=(cpu.x[0]).wrapping_add(0x54u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[8]=(v0) as u32 as u64;cpu.x[9]=(v1) as u32 as u64;}
// 0x111d7b70 ldr w11, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
// 0x111d7b74 mov x21, x3
cpu.x[21]=(((cpu.x[3]))) as u64;
// 0x111d7b78 mov w20, w2
cpu.x[20]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x111d7b7c mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x111d7b80 str wzr, [x0, #0x70]
{let address=(cpu.x[0]).wrapping_add(0x70u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111d7b84 sdiv w10, w2, w8
{let a=((cpu.x[2] as u32 as u64)) as i32;let b=((cpu.x[8] as u32 as u64)) as i32;cpu.x[10]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u32 as u64;}
// 0x111d7b88 msub w12, w10, w8, w2
cpu.x[12]=((((cpu.x[2] as u32 as u64))).wrapping_sub((((cpu.x[10] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64))))) as u32 as u64;
// 0x111d7b8c mul w8, w9, w8
cpu.x[8]=((((cpu.x[9] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64)))) as u32 as u64;
// 0x111d7b90 add w9, w11, #1
{let a=((cpu.x[11] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x111d7b94 str w9, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111d7b98 stp w12, w10, [x0, #0x5c]
{let address=(cpu.x[0]).wrapping_add(0x5cu64);let v0=cpu.x[12] as u32 as u64;let v1=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x111d7b9c cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d7ba0 b.lt #0x111d7bf0
if cpu.condition(11){pc=0x111d7bf0;continue;}pc=0x111d7ba4;continue;
}
0x111d7ba4=>{
// 0x111d7ba4 ldp x10, x11, [x19, #0x38]
{let address=(cpu.x[19]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[10]=(v0) as u64;cpu.x[11]=(v1) as u64;}
// 0x111d7ba8 cmp w8, #0x14
{let a=((cpu.x[8] as u32 as u64));let b=((0x14u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d7bac ldr x9, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111d7bb0 ldr x12, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[12]=(v0) as u64;}
// 0x111d7bb4 b.hs #0x111d7d78
if cpu.condition(2){pc=0x111d7d78;continue;}pc=0x111d7bb8;continue;
}
0x111d7bb8=>{
// 0x111d7bb8 mov x13, xzr
cpu.x[13]=(((0u64))) as u64;
pc=0x111d7bbc;continue;
}
0x111d7bbc=>{
// 0x111d7bbc lsl x14, x13, #2
cpu.x[14]=(((((cpu.x[13]))) as u64).wrapping_shl(((((0x2u64))) & 63) as u32) as u64) as u64;
// 0x111d7bc0 sub x8, x8, x13
{let a=((cpu.x[8]));let b=((cpu.x[13]));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u64;}
// 0x111d7bc4 mov w13, #0x7ffffff
cpu.x[13]=(((0x7ffffffu64))) as u32 as u64;
// 0x111d7bc8 add x12, x12, x14
{let a=((cpu.x[12]));let b=((cpu.x[14]));let result=a.wrapping_add(b);cpu.x[12]=(result) as u64;}
// 0x111d7bcc add x11, x11, x14
{let a=((cpu.x[11]));let b=((cpu.x[14]));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
// 0x111d7bd0 add x10, x10, x14
{let a=((cpu.x[10]));let b=((cpu.x[14]));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x111d7bd4 add x9, x9, x14
{let a=((cpu.x[9]));let b=((cpu.x[14]));let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
pc=0x111d7bd8;continue;
}
0x111d7bd8=>{
// 0x111d7bd8 subs x8, x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x111d7bdc str wzr, [x9], #4
{let address=(cpu.x[9]).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);cpu.x[9]=((cpu.x[9]).wrapping_add(((0x4u64)))) as u64;}
// 0x111d7be0 str wzr, [x10], #4
{let address=(cpu.x[10]).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);cpu.x[10]=((cpu.x[10]).wrapping_add(((0x4u64)))) as u64;}
// 0x111d7be4 str wzr, [x11], #4
{let address=(cpu.x[11]).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);cpu.x[11]=((cpu.x[11]).wrapping_add(((0x4u64)))) as u64;}
// 0x111d7be8 str w13, [x12], #4
{let address=(cpu.x[12]).wrapping_add(0x0u64);let v0=cpu.x[13] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);cpu.x[12]=((cpu.x[12]).wrapping_add(((0x4u64)))) as u64;}
// 0x111d7bec b.ne #0x111d7bd8
if cpu.condition(1){pc=0x111d7bd8;continue;}pc=0x111d7bf0;continue;
}
0x111d7bf0=>{
// 0x111d7bf0 ldr x9, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111d7bf4 ldr x10, [x19, #0x30]
{let address=(cpu.x[19]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x111d7bf8 sbfiz x23, x1, #2, #0x20
cpu.x[23]=((((((cpu.x[1])) << 32) as i64 >> 32) as u64).wrapping_shl(2)) as u64;
// 0x111d7bfc mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
pc=0x111d7c00;continue;
}
0x111d7c00=>{
// 0x111d7c00 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x111d7c04 mov x2, x21
cpu.x[2]=(((cpu.x[21]))) as u64;
// 0x111d7c08 str w8, [x19, #0x50]
{let address=(cpu.x[19]).wrapping_add(0x50u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111d7c0c mov w8, #-1
cpu.x[8]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x111d7c10 str w1, [x9]
{let address=(cpu.x[9]).wrapping_add(0x0u64);let v0=cpu.x[1] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111d7c14 str w8, [x10, x23]
{let address=((cpu.x[10]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[23])));let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111d7c18 str w8, [x10, w20, sxtw #2]
{let address=((cpu.x[10]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[20] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111d7c1c bl #0x111d7e18
cpu.x[30]=0x111d7c20;super::chunk5::f_111d7e18(cpu,rt,0x111d7e18)?;
pc=0x111d7c20;continue;
}
0x111d7c20=>{
// 0x111d7c20 ldr x8, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d7c24 mov w22, #2
cpu.x[22]=(((0x2u64))) as u32 as u64;
// 0x111d7c28 str w22, [x8, x23]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[23])));let v0=cpu.x[22] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111d7c2c ldr w10, [x19, #0x70]
{let address=(cpu.x[19]).wrapping_add(0x70u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x111d7c30 cbz w10, #0x111d7d64
if (cpu.x[10] as u32 as u64) == 0 {pc=0x111d7d64;continue;}pc=0x111d7c34;continue;
}
0x111d7c34=>{
// 0x111d7c34 ldr w23, [x19, #0x14]
{let address=(cpu.x[19]).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[23]=(v0) as u32 as u64;}
pc=0x111d7c38;continue;
}
0x111d7c38=>{
// 0x111d7c38 ldr x9, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111d7c3c sub w10, w10, #1
{let a=((cpu.x[10] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[10]=(result) as u32 as u64;}
// 0x111d7c40 mov w12, wzr
cpu.x[12]=(((0u64))) as u32 as u64;
// 0x111d7c44 ldrsw x1, [x9]
{let address=(cpu.x[9]).wrapping_add(0x0u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[1]=(v0) as u64;}
// 0x111d7c48 str w10, [x19, #0x70]
{let address=(cpu.x[19]).wrapping_add(0x70u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111d7c4c ldr w10, [x9, w10, sxtw #2]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[10] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x111d7c50 str w10, [x9]
{let address=(cpu.x[9]).wrapping_add(0x0u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111d7c54 ldr x11, [x19, #0x40]
{let address=(cpu.x[19]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[11]=(v0) as u64;}
pc=0x111d7c58;continue;
}
0x111d7c58=>{
// 0x111d7c58 lsl w13, w12, #1
cpu.x[13]=(((((cpu.x[12] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x111d7c5c ldr w15, [x19, #0x70]
{let address=(cpu.x[19]).wrapping_add(0x70u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[15]=(v0) as u32 as u64;}
// 0x111d7c60 mov w14, w12
cpu.x[14]=(((cpu.x[12] as u32 as u64))) as u32 as u64;
// 0x111d7c64 sxtw x13, w13
cpu.x[13]=((((cpu.x[13] as u32 as u64))) as i32 as i64 as u64) as u64;
// 0x111d7c68 add x16, x13, #2
{let a=((cpu.x[13]));let b=((0x2u64));let result=a.wrapping_add(b);cpu.x[16]=(result) as u64;}
// 0x111d7c6c cmp w16, w15
{let a=((cpu.x[16] as u32 as u64));let b=((cpu.x[15] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d7c70 b.ge #0x111d7c90
if cpu.condition(10){pc=0x111d7c90;continue;}pc=0x111d7c74;continue;
}
0x111d7c74=>{
// 0x111d7c74 ldrsw x14, [x9, x16, lsl #2]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[16])) as u64).wrapping_shl(2) as u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[14]=(v0) as u64;}
// 0x111d7c78 ldr w17, [x11, w10, sxtw #2]
{let address=((cpu.x[11]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[10] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[17]=(v0) as u32 as u64;}
// 0x111d7c7c ldr w14, [x11, x14, lsl #2]
{let address=((cpu.x[11]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[14])) as u64).wrapping_shl(2) as u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[14]=(v0) as u32 as u64;}
// 0x111d7c80 cmp w17, w14
{let a=((cpu.x[17] as u32 as u64));let b=((cpu.x[14] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d7c84 mov w14, w16
cpu.x[14]=(((cpu.x[16] as u32 as u64))) as u32 as u64;
// 0x111d7c88 b.gt #0x111d7c90
if cpu.condition(12){pc=0x111d7c90;continue;}pc=0x111d7c8c;continue;
}
0x111d7c8c=>{
// 0x111d7c8c mov w14, w12
cpu.x[14]=(((cpu.x[12] as u32 as u64))) as u32 as u64;
pc=0x111d7c90;continue;
}
0x111d7c90=>{
// 0x111d7c90 orr x13, x13, #1
{let a=((cpu.x[13]));let b=((0x1u64));let result=a | b;cpu.x[13]=(result) as u64;}
// 0x111d7c94 cmp w13, w15
{let a=((cpu.x[13] as u32 as u64));let b=((cpu.x[15] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d7c98 b.ge #0x111d7cb4
if cpu.condition(10){pc=0x111d7cb4;continue;}pc=0x111d7c9c;continue;
}
0x111d7c9c=>{
// 0x111d7c9c ldrsw x15, [x9, w14, sxtw #2]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[14] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[15]=(v0) as u64;}
// 0x111d7ca0 ldrsw x16, [x9, x13, lsl #2]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[13])) as u64).wrapping_shl(2) as u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[16]=(v0) as u64;}
// 0x111d7ca4 ldr w15, [x11, x15, lsl #2]
{let address=((cpu.x[11]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[15])) as u64).wrapping_shl(2) as u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[15]=(v0) as u32 as u64;}
// 0x111d7ca8 ldr w16, [x11, x16, lsl #2]
{let address=((cpu.x[11]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[16])) as u64).wrapping_shl(2) as u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[16]=(v0) as u32 as u64;}
// 0x111d7cac cmp w15, w16
{let a=((cpu.x[15] as u32 as u64));let b=((cpu.x[16] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d7cb0 b.gt #0x111d7cb8
if cpu.condition(12){pc=0x111d7cb8;continue;}pc=0x111d7cb4;continue;
}
0x111d7cb4=>{
// 0x111d7cb4 mov w13, w14
cpu.x[13]=(((cpu.x[14] as u32 as u64))) as u32 as u64;
pc=0x111d7cb8;continue;
}
0x111d7cb8=>{
// 0x111d7cb8 cmp w13, w12
{let a=((cpu.x[13] as u32 as u64));let b=((cpu.x[12] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d7cbc b.eq #0x111d7cdc
if cpu.condition(0){pc=0x111d7cdc;continue;}pc=0x111d7cc0;continue;
}
0x111d7cc0=>{
// 0x111d7cc0 mov w14, w13
cpu.x[14]=(((cpu.x[13] as u32 as u64))) as u32 as u64;
// 0x111d7cc4 sbfiz x14, x14, #2, #0x20
cpu.x[14]=((((((cpu.x[14])) << 32) as i64 >> 32) as u64).wrapping_shl(2)) as u64;
// 0x111d7cc8 ldr w15, [x9, x14]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[14])));let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[15]=(v0) as u32 as u64;}
// 0x111d7ccc str w15, [x9, w12, sxtw #2]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[12] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=cpu.x[15] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111d7cd0 mov w12, w13
cpu.x[12]=(((cpu.x[13] as u32 as u64))) as u32 as u64;
// 0x111d7cd4 str w10, [x9, x14]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[14])));let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111d7cd8 b #0x111d7c58
pc=0x111d7c58;continue;
}
0x111d7cdc=>{
// 0x111d7cdc mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x111d7ce0 mov x2, x21
cpu.x[2]=(((cpu.x[21]))) as u64;
// 0x111d7ce4 str w22, [x8, x1, lsl #2]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[1])) as u64).wrapping_shl(2) as u64);let v0=cpu.x[22] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111d7ce8 bl #0x111d7e18
cpu.x[30]=0x111d7cec;super::chunk5::f_111d7e18(cpu,rt,0x111d7e18)?;
pc=0x111d7cec;continue;
}
0x111d7cec=>{
// 0x111d7cec cmp w23, #1
{let a=((cpu.x[23] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d7cf0 b.lt #0x111d7d04
if cpu.condition(11){pc=0x111d7d04;continue;}pc=0x111d7cf4;continue;
}
0x111d7cf4=>{
// 0x111d7cf4 ldr w8, [x19, #0x14]
{let address=(cpu.x[19]).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d7cf8 subs w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x111d7cfc str w8, [x19, #0x14]
{let address=(cpu.x[19]).wrapping_add(0x14u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111d7d00 b.eq #0x111d7d20
if cpu.condition(0){pc=0x111d7d20;continue;}pc=0x111d7d04;continue;
}
0x111d7d04=>{
// 0x111d7d04 ldr x8, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d7d08 ldr w9, [x8, w20, sxtw #2]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[20] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x111d7d0c cmp w9, #2
{let a=((cpu.x[9] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d7d10 b.eq #0x111d7d20
if cpu.condition(0){pc=0x111d7d20;continue;}pc=0x111d7d14;continue;
}
0x111d7d14=>{
// 0x111d7d14 ldr w10, [x19, #0x70]
{let address=(cpu.x[19]).wrapping_add(0x70u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x111d7d18 cmp w10, #0
{let a=((cpu.x[10] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d7d1c b.gt #0x111d7c38
if cpu.condition(12){pc=0x111d7c38;continue;}pc=0x111d7d20;continue;
}
0x111d7d20=>{
// 0x111d7d20 cmn w20, #1
{let a=((cpu.x[20] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.flags(a,b,result,32,false);}
// 0x111d7d24 str wzr, [x19, #0x50]
{let address=(cpu.x[19]).wrapping_add(0x50u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111d7d28 b.eq #0x111d7d64
if cpu.condition(0){pc=0x111d7d64;continue;}pc=0x111d7d2c;continue;
}
0x111d7d2c=>{
// 0x111d7d2c ldr x8, [x19, #0x30]
{let address=(cpu.x[19]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d7d30 ldr w9, [x8, w20, sxtw #2]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[20] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x111d7d34 cmn w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.flags(a,b,result,32,false);}
// 0x111d7d38 b.eq #0x111d7d64
if cpu.condition(0){pc=0x111d7d64;continue;}pc=0x111d7d3c;continue;
}
0x111d7d3c=>{
// 0x111d7d3c ldr x10, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
pc=0x111d7d40;continue;
}
0x111d7d40=>{
// 0x111d7d40 ldrsw x11, [x19, #0x50]
{let address=(cpu.x[19]).wrapping_add(0x50u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[11]=(v0) as u64;}
// 0x111d7d44 add w12, w11, #1
{let a=((cpu.x[11] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[12]=(result) as u32 as u64;}
// 0x111d7d48 str w12, [x19, #0x50]
{let address=(cpu.x[19]).wrapping_add(0x50u64);let v0=cpu.x[12] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111d7d4c mov w12, w9
cpu.x[12]=(((cpu.x[9] as u32 as u64))) as u32 as u64;
// 0x111d7d50 str w20, [x10, x11, lsl #2]
{let address=((cpu.x[10]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[11])) as u64).wrapping_shl(2) as u64);let v0=cpu.x[20] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111d7d54 mov w20, w12
cpu.x[20]=(((cpu.x[12] as u32 as u64))) as u32 as u64;
// 0x111d7d58 ldr w9, [x8, w9, sxtw #2]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[9] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x111d7d5c cmn w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.flags(a,b,result,32,false);}
// 0x111d7d60 b.ne #0x111d7d40
if cpu.condition(1){pc=0x111d7d40;continue;}pc=0x111d7d64;continue;
}
0x111d7d64=>{
// 0x111d7d64 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d7d68 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111d7d6c ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d7d70 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x111d7d74 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x111d7d78=>{
// 0x111d7d78 sub x13, x10, x9
{let a=((cpu.x[10]));let b=((cpu.x[9]));let result=a.wrapping_sub(b);cpu.x[13]=(result) as u64;}
// 0x111d7d7c cmp x13, #0x20
{let a=((cpu.x[13]));let b=((0x20u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x111d7d80 mov x13, xzr
cpu.x[13]=(((0u64))) as u64;
// 0x111d7d84 b.lo #0x111d7bbc
if cpu.condition(3){pc=0x111d7bbc;continue;}pc=0x111d7d88;continue;
}
0x111d7d88=>{
// 0x111d7d88 sub x14, x11, x9
{let a=((cpu.x[11]));let b=((cpu.x[9]));let result=a.wrapping_sub(b);cpu.x[14]=(result) as u64;}
// 0x111d7d8c cmp x14, #0x20
{let a=((cpu.x[14]));let b=((0x20u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x111d7d90 b.lo #0x111d7bbc
if cpu.condition(3){pc=0x111d7bbc;continue;}pc=0x111d7d94;continue;
}
0x111d7d94=>{
// 0x111d7d94 sub x14, x12, x9
{let a=((cpu.x[12]));let b=((cpu.x[9]));let result=a.wrapping_sub(b);cpu.x[14]=(result) as u64;}
// 0x111d7d98 cmp x14, #0x20
{let a=((cpu.x[14]));let b=((0x20u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x111d7d9c b.lo #0x111d7bbc
if cpu.condition(3){pc=0x111d7bbc;continue;}pc=0x111d7da0;continue;
}
0x111d7da0=>{
// 0x111d7da0 sub x14, x11, x10
{let a=((cpu.x[11]));let b=((cpu.x[10]));let result=a.wrapping_sub(b);cpu.x[14]=(result) as u64;}
// 0x111d7da4 cmp x14, #0x20
{let a=((cpu.x[14]));let b=((0x20u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x111d7da8 b.lo #0x111d7bbc
if cpu.condition(3){pc=0x111d7bbc;continue;}pc=0x111d7dac;continue;
}
0x111d7dac=>{
// 0x111d7dac sub x14, x12, x10
{let a=((cpu.x[12]));let b=((cpu.x[10]));let result=a.wrapping_sub(b);cpu.x[14]=(result) as u64;}
// 0x111d7db0 cmp x14, #0x20
{let a=((cpu.x[14]));let b=((0x20u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x111d7db4 b.lo #0x111d7bbc
if cpu.condition(3){pc=0x111d7bbc;continue;}pc=0x111d7db8;continue;
}
0x111d7db8=>{
// 0x111d7db8 sub x14, x12, x11
{let a=((cpu.x[12]));let b=((cpu.x[11]));let result=a.wrapping_sub(b);cpu.x[14]=(result) as u64;}
// 0x111d7dbc cmp x14, #0x20
{let a=((cpu.x[14]));let b=((0x20u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x111d7dc0 b.lo #0x111d7bbc
if cpu.condition(3){pc=0x111d7bbc;continue;}pc=0x111d7dc4;continue;
}
0x111d7dc4=>{
// 0x111d7dc4 movi v0.2d, #0000000000000000
cpu.q[0]=(lane_dup(0u128,64,2)) & 0xffffffffffffffffffffffffffffffffu128;
// 0x111d7dc8 mvni v1.4s, #0xf8, lsl #24
cpu.q[1]=(lane_dup(4160749568u128,32,4) ^ 0xffffffffffffffffffffffffffffffffu128) & 0xffffffffffffffffffffffffffffffffu128;
// 0x111d7dcc and x13, x8, #0x7ffffff8
{let a=((cpu.x[8]));let b=((0x7ffffff8u64));let result=a & b;cpu.x[13]=(result) as u64;}
// 0x111d7dd0 add x14, x9, #0x10
{let a=((cpu.x[9]));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[14]=(result) as u64;}
// 0x111d7dd4 add x15, x10, #0x10
{let a=((cpu.x[10]));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[15]=(result) as u64;}
// 0x111d7dd8 add x16, x11, #0x10
{let a=((cpu.x[11]));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[16]=(result) as u64;}
// 0x111d7ddc add x17, x12, #0x10
{let a=((cpu.x[12]));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[17]=(result) as u64;}
// 0x111d7de0 mov x0, x13
cpu.x[0]=(((cpu.x[13]))) as u64;
pc=0x111d7de4;continue;
}
0x111d7de4=>{
// 0x111d7de4 stp q0, q0, [x14, #-0x10]
{let address=(cpu.x[14]).wrapping_add(0xfffffffffffffff0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x111d7de8 add x14, x14, #0x20
{let a=((cpu.x[14]));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[14]=(result) as u64;}
// 0x111d7dec subs x0, x0, #8
{let a=((cpu.x[0]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.x[0]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x111d7df0 stp q0, q0, [x15, #-0x10]
{let address=(cpu.x[15]).wrapping_add(0xfffffffffffffff0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x111d7df4 add x15, x15, #0x20
{let a=((cpu.x[15]));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[15]=(result) as u64;}
// 0x111d7df8 stp q0, q0, [x16, #-0x10]
{let address=(cpu.x[16]).wrapping_add(0xfffffffffffffff0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x111d7dfc add x16, x16, #0x20
{let a=((cpu.x[16]));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[16]=(result) as u64;}
// 0x111d7e00 stp q1, q1, [x17, #-0x10]
{let address=(cpu.x[17]).wrapping_add(0xfffffffffffffff0u64);let v0=(cpu.q[1] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[1] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x111d7e04 add x17, x17, #0x20
{let a=((cpu.x[17]));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[17]=(result) as u64;}
// 0x111d7e08 b.ne #0x111d7de4
if cpu.condition(1){pc=0x111d7de4;continue;}pc=0x111d7e0c;continue;
}
0x111d7e0c=>{
// 0x111d7e0c cmp x13, x8
{let a=((cpu.x[13]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x111d7e10 b.ne #0x111d7bbc
if cpu.condition(1){pc=0x111d7bbc;continue;}pc=0x111d7e14;continue;
}
0x111d7e14=>{
// 0x111d7e14 b #0x111d7bf0
pc=0x111d7bf0;continue;
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111d7b58")),}}}
#[inline]
pub(super) unsafe fn f_111dc37c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111dc37c=>{
// 0x111dc37c stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x111dc380 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111dc384 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111dc388 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x111dc38c mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x111dc390 ldp x0, x21, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[0]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111dc394 mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x111dc398 bl #0x110cbf10
cpu.x[30]=0x111dc39c;super::chunk14::f_110cbf10(cpu,rt,0x110cbf10)?;
pc=0x111dc39c;continue;
}
0x111dc39c=>{
// 0x111dc39c mov x3, x0
cpu.x[3]=(((cpu.x[0]))) as u64;
// 0x111dc3a0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111dc3a4 mov x1, x21
cpu.x[1]=(((cpu.x[21]))) as u64;
// 0x111dc3a8 mov w2, #0x31
cpu.x[2]=(((0x31u64))) as u32 as u64;
// 0x111dc3ac bl #0x111dd77c
cpu.x[30]=0x111dc3b0;super::chunk15::f_111dd77c(cpu,rt,0x111dd77c)?;
pc=0x111dc3b0;continue;
}
0x111dc3b0=>{
// 0x111dc3b0 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111dc3b4 ldr w1, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x111dc3b8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111dc3bc ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111dc3c0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x111dc3c4;super::dispatch(cpu,rt,target)?;}
pc=0x111dc3c4;continue;
}
0x111dc3c4=>{
// 0x111dc3c4 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111dc3c8 ldr x9, [x8, #0xa8]
{let address=(cpu.x[8]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111dc3cc cbz x9, #0x111dc3ec
if (cpu.x[9]) == 0 {pc=0x111dc3ec;continue;}pc=0x111dc3d0;continue;
}
0x111dc3d0=>{
// 0x111dc3d0 ldr x0, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111dc3d4 ldr x2, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x111dc3d8 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x111dc3dc ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111dc3e0 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x111dc3e4 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x111dc3e8 b #0x10f8c0ac
super::chunk1::f_10f8c0ac(cpu,rt,0x10f8c0ac)?;return Ok(());
}
0x111dc3ec=>{
// 0x111dc3ec ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111dc3f0 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x111dc3f4 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x111dc3f8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111dc37c")),}}}
#[inline]
pub(super) unsafe fn f_111dc5e0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111dc5e0=>{
// 0x111dc5e0 ldr x0, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111dc5e4 cbz x0, #0x111dc5ec
if (cpu.x[0]) == 0 {pc=0x111dc5ec;continue;}pc=0x111dc5e8;continue;
}
0x111dc5e8=>{
// 0x111dc5e8 b #0x10e9dcdc
super::dispatch(cpu,rt,0x10e9dcdc)?;return Ok(());
}
0x111dc5ec=>{
// 0x111dc5ec ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111dc5e0")),}}}
#[inline]
pub(super) unsafe fn f_111dd1ec(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111dd1ec=>{
// 0x111dd1ec cmp w2, #1
{let a=((cpu.x[2] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111dd1f0 b.lt #0x111dd244
if cpu.condition(11){pc=0x111dd244;continue;}pc=0x111dd1f4;continue;
}
0x111dd1f4=>{
// 0x111dd1f4 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x111dd1f8 stp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111dd1fc stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111dd200 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x111dd204 mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x111dd208 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x111dd20c mov w8, wzr
cpu.x[8]=(((0u64))) as u32 as u64;
// 0x111dd210 mov w21, w2
cpu.x[21]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
pc=0x111dd214;continue;
}
0x111dd214=>{
// 0x111dd214 ldr x9, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111dd218 ldr w22, [x19], #4
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[22]=(v0) as u32 as u64;cpu.x[19]=((cpu.x[19]).wrapping_add(((0x4u64)))) as u64;}
// 0x111dd21c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111dd220 ldr x9, [x9, #0x88]
{let address=(cpu.x[9]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111dd224 sub w1, w22, w8
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[1]=(result) as u32 as u64;}
// 0x111dd228 blr x9
{let target=cpu.x[9];cpu.x[30]=0x111dd22c;super::dispatch(cpu,rt,target)?;}
pc=0x111dd22c;continue;
}
0x111dd22c=>{
// 0x111dd22c subs x21, x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[21]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x111dd230 mov w8, w22
cpu.x[8]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x111dd234 b.ne #0x111dd214
if cpu.condition(1){pc=0x111dd214;continue;}pc=0x111dd238;continue;
}
0x111dd238=>{
// 0x111dd238 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111dd23c ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111dd240 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
pc=0x111dd244;continue;
}
0x111dd244=>{
// 0x111dd244 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111dd1ec")),}}}
#[inline]
pub(super) unsafe fn f_111dd468(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111dd468=>{
// 0x111dd468 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x111dd46c stp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111dd470 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111dd474 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x111dd478 ldr w8, [x1, #0xc]
{let address=(cpu.x[1]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111dd47c ldr x9, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111dd480 mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x111dd484 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x111dd488 cmp w2, w8
{let a=((cpu.x[2] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111dd48c ldr x9, [x9, #0x88]
{let address=(cpu.x[9]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111dd490 csel w10, w2, w8, lt
cpu.x[10]=(if cpu.condition(11){((cpu.x[2] as u32 as u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x111dd494 cmn w2, #1
{let a=((cpu.x[2] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.flags(a,b,result,32,false);}
// 0x111dd498 csel w21, w8, w10, eq
cpu.x[21]=(if cpu.condition(0){((cpu.x[8] as u32 as u64))}else{((cpu.x[10] as u32 as u64))}) as u32 as u64;
// 0x111dd49c mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x111dd4a0 blr x9
{let target=cpu.x[9];cpu.x[30]=0x111dd4a4;super::dispatch(cpu,rt,target)?;}
pc=0x111dd4a4;continue;
}
0x111dd4a4=>{
// 0x111dd4a4 cmp w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111dd4a8 b.lt #0x111dd4d8
if cpu.condition(11){pc=0x111dd4d8;continue;}pc=0x111dd4ac;continue;
}
0x111dd4ac=>{
// 0x111dd4ac ubfiz x21, x21, #2, #0x20
cpu.x[21]=(((((cpu.x[21])) & 0xffffffffu64) << 2)) as u64;
// 0x111dd4b0 mov x22, xzr
cpu.x[22]=(((0u64))) as u64;
pc=0x111dd4b4;continue;
}
0x111dd4b4=>{
// 0x111dd4b4 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111dd4b8 ldr x9, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111dd4bc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111dd4c0 ldr w1, [x8, x22]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[22])));let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x111dd4c4 ldr x8, [x9, #0x88]
{let address=(cpu.x[9]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111dd4c8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x111dd4cc;super::dispatch(cpu,rt,target)?;}
pc=0x111dd4cc;continue;
}
0x111dd4cc=>{
// 0x111dd4cc add x22, x22, #4
{let a=((cpu.x[22]));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[22]=(result) as u64;}
// 0x111dd4d0 cmp x21, x22
{let a=((cpu.x[21]));let b=((cpu.x[22]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x111dd4d4 b.ne #0x111dd4b4
if cpu.condition(1){pc=0x111dd4b4;continue;}pc=0x111dd4d8;continue;
}
0x111dd4d8=>{
// 0x111dd4d8 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111dd4dc ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111dd4e0 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x111dd4e4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111dd468")),}}}
#[inline]
pub(super) unsafe fn f_111dd6a4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111dd6a4=>{
// 0x111dd6a4 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x111dd6a8 str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111dd6ac mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x111dd6b0 cbz x1, #0x111dd6c8
if (cpu.x[1]) == 0 {pc=0x111dd6c8;continue;}pc=0x111dd6b4;continue;
}
0x111dd6b4=>{
// 0x111dd6b4 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x111dd6b8 mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x111dd6bc bl #0x10e0b624
cpu.x[30]=0x111dd6c0;super::chunk10::f_10e0b624(cpu,rt,0x10e0b624)?;
pc=0x111dd6c0;continue;
}
0x111dd6c0=>{
// 0x111dd6c0 mov w1, w0
cpu.x[1]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x111dd6c4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
pc=0x111dd6c8;continue;
}
0x111dd6c8=>{
// 0x111dd6c8 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111dd6cc ldr x2, [x8, #0x50]
{let address=(cpu.x[8]).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x111dd6d0 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x111dd6d4 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x111dd6d8 br x2
{let target=cpu.x[2];if target>=0x111dd6a4 && target<0x111dd6dc{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111dd6a4")),}}}
#[inline]
pub(super) unsafe fn f_111e4f8c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111e4f8c=>{
// 0x111e4f8c stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x111e4f90 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111e4f94 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x111e4f98 ldr x8, [x0, #0x18]
{let address=(cpu.x[0]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111e4f9c mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x111e4fa0 mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x111e4fa4 mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x111e4fa8 mov x1, x8
cpu.x[1]=(((cpu.x[8]))) as u64;
// 0x111e4fac bl #0x111dd1b0
cpu.x[30]=0x111e4fb0;super::chunk14::f_111dd1b0(cpu,rt,0x111dd1b0)?;
pc=0x111e4fb0;continue;
}
0x111e4fb0=>{
// 0x111e4fb0 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111e4fb4 str x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111e4fb8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111e4fbc ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111e4fc0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x111e4fc4;super::dispatch(cpu,rt,target)?;}
pc=0x111e4fc4;continue;
}
0x111e4fc4=>{
// 0x111e4fc4 str w0, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111e4fc8 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111e4fcc ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x111e4fd0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111e4f8c")),}}}
#[inline]
pub(super) unsafe fn f_111e5850(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111e5850=>{
// 0x111e5850 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x111e5854 str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111e5858 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x111e585c mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x111e5860 str x1, [x0, #0x88]
{let address=(cpu.x[0]).wrapping_add(0x88u64);let v0=cpu.x[1];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111e5864 cbz x1, #0x111e5880
if (cpu.x[1]) == 0 {pc=0x111e5880;continue;}pc=0x111e5868;continue;
}
0x111e5868=>{
// 0x111e5868 mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x111e586c bl #0x1128bc0c
cpu.x[30]=0x111e5870;super::chunk12::f_1128bc0c(cpu,rt,0x1128bc0c)?;
pc=0x111e5870;continue;
}
0x111e5870=>{
// 0x111e5870 str w0, [x19, #0x90]
{let address=(cpu.x[19]).wrapping_add(0x90u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111e5874 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x111e5878 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x111e587c ret
cpu.pc=cpu.x[30];return Ok(());
}
0x111e5880=>{
// 0x111e5880 str wzr, [x19, #0x90]
{let address=(cpu.x[19]).wrapping_add(0x90u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111e5884 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x111e5888 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x111e588c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111e5850")),}}}
#[inline]
pub(super) unsafe fn f_111e6370(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111e6370=>{
// 0x111e6370 sub sp, sp, #0x40
{let a=((cpu.sp));let b=((0x40u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x111e6374 stp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111e6378 stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111e637c stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111e6380 add x29, sp, #0x10
{let a=((cpu.sp));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x111e6384 ldrb w8, [x0, #0x80]
{let address=(cpu.x[0]).wrapping_add(0x80u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111e6388 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x111e638c mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x111e6390 cbz w8, #0x111e63a4
if (cpu.x[8] as u32 as u64) == 0 {pc=0x111e63a4;continue;}pc=0x111e6394;continue;
}
0x111e6394=>{
// 0x111e6394 ldp w9, w8, [x19, #0xa0]
{let address=(cpu.x[19]).wrapping_add(0xa0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x111e6398 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111e639c b.eq #0x111e63e8
if cpu.condition(0){pc=0x111e63e8;continue;}pc=0x111e63a0;continue;
}
0x111e63a0=>{
// 0x111e63a0 b #0x111e640c
pc=0x111e640c;continue;
}
0x111e63a4=>{
// 0x111e63a4 adrp x1, #0x102bb000
cpu.x[1]=(0x102bb000u64) as u64;
// 0x111e63a8 add x1, x1, #0xb97
{let a=((cpu.x[1]));let b=((0xb97u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x111e63ac mov x0, sp
cpu.x[0]=(((cpu.sp))) as u64;
// 0x111e63b0 mov x21, sp
cpu.x[21]=(((cpu.sp))) as u64;
// 0x111e63b4 bl #0x1142fde4
cpu.x[30]=0x111e63b8;super::chunk6::f_1142fde4(cpu,rt,0x1142fde4)?;
// 0x111e63b8 ldr w8, [sp, #4]
{let address=(cpu.sp).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111e63bc ldr x9, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111e63c0 add x10, x21, #8
{let a=((cpu.x[21]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x111e63c4 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x111e63c8 cmp w8, #8
{let a=((cpu.x[8] as u32 as u64));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111e63cc csel x0, x10, x9, lt
cpu.x[0]=(if cpu.condition(11){((cpu.x[10]))}else{((cpu.x[9]))}) as u64;
// 0x111e63d0 bl #0x1128df18
cpu.x[30]=0x111e63d4;super::dispatch(cpu,rt,0x1128df18)?;
// 0x111e63d4 mov x0, sp
cpu.x[0]=(((cpu.sp))) as u64;
// 0x111e63d8 bl #0x1142fcc4
cpu.x[30]=0x111e63dc;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
// 0x111e63dc ldp w9, w8, [x19, #0xa0]
{let address=(cpu.x[19]).wrapping_add(0xa0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x111e63e0 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111e63e4 b.ne #0x111e640c
if cpu.condition(1){pc=0x111e640c;continue;}pc=0x111e63e8;continue;
}
0x111e63e8=>{
// 0x111e63e8 lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x111e63ec mov w10, #5
cpu.x[10]=(((0x5u64))) as u32 as u64;
// 0x111e63f0 cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111e63f4 csel w1, w10, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[10] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x111e63f8 cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111e63fc b.ge #0x111e640c
if cpu.condition(10){pc=0x111e640c;continue;}pc=0x111e6400;continue;
}
0x111e6400=>{
// 0x111e6400 add x0, x19, #0x98
{let a=((cpu.x[19]));let b=((0x98u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x111e6404 bl #0x1078f4d4
cpu.x[30]=0x111e6408;super::chunk2::f_1078f4d4(cpu,rt,0x1078f4d4)?;
// 0x111e6408 ldr w8, [x19, #0xa4]
{let address=(cpu.x[19]).wrapping_add(0xa4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
pc=0x111e640c;continue;
}
0x111e640c=>{
// 0x111e640c ldr x9, [x19, #0x98]
{let address=(cpu.x[19]).wrapping_add(0x98u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111e6410 add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x111e6414 str w10, [x19, #0xa4]
{let address=(cpu.x[19]).wrapping_add(0xa4u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111e6418 mov w10, #3
cpu.x[10]=(((0x3u64))) as u32 as u64;
// 0x111e641c str w10, [x9, w8, sxtw #2]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111e6420 ldr x0, [x19, #0x88]
{let address=(cpu.x[19]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111e6424 cbz x0, #0x111e6580
if (cpu.x[0]) == 0 {pc=0x111e6580;continue;}pc=0x111e6428;continue;
}
0x111e6428=>{
// 0x111e6428 bl #0x1128bc0c
cpu.x[30]=0x111e642c;super::chunk12::f_1128bc0c(cpu,rt,0x1128bc0c)?;
pc=0x111e642c;continue;
}
0x111e642c=>{
// 0x111e642c ldr w8, [x19, #0x90]
{let address=(cpu.x[19]).wrapping_add(0x90u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111e6430 sub w21, w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[21]=(result) as u32 as u64;}
// 0x111e6434 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x111e6438 bl #0x1128bc0c
cpu.x[30]=0x111e643c;super::chunk12::f_1128bc0c(cpu,rt,0x1128bc0c)?;
pc=0x111e643c;continue;
}
0x111e643c=>{
// 0x111e643c cmp w21, w0
{let a=((cpu.x[21] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111e6440 b.eq #0x111e649c
if cpu.condition(0){pc=0x111e649c;continue;}pc=0x111e6444;continue;
}
0x111e6444=>{
// 0x111e6444 ldr x0, [x19, #0x88]
{let address=(cpu.x[19]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111e6448 bl #0x1128bc0c
cpu.x[30]=0x111e644c;super::chunk12::f_1128bc0c(cpu,rt,0x1128bc0c)?;
// 0x111e644c ldr w8, [x19, #0x90]
{let address=(cpu.x[19]).wrapping_add(0x90u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111e6450 sub w21, w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[21]=(result) as u32 as u64;}
// 0x111e6454 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x111e6458 bl #0x1128bc0c
cpu.x[30]=0x111e645c;super::chunk12::f_1128bc0c(cpu,rt,0x1128bc0c)?;
// 0x111e645c mov w2, w0
cpu.x[2]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x111e6460 adrp x0, #0x10329000
cpu.x[0]=(0x10329000u64) as u64;
// 0x111e6464 add x0, x0, #0xc01
{let a=((cpu.x[0]));let b=((0xc01u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x111e6468 mov x8, sp
cpu.x[8]=(((cpu.sp))) as u64;
// 0x111e646c mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x111e6470 mov x21, sp
cpu.x[21]=(((cpu.sp))) as u64;
// 0x111e6474 bl #0x114328cc
cpu.x[30]=0x111e6478;super::chunk0::f_114328cc(cpu,rt,0x114328cc)?;
// 0x111e6478 ldr w8, [sp, #4]
{let address=(cpu.sp).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111e647c ldr x9, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111e6480 add x10, x21, #8
{let a=((cpu.x[21]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x111e6484 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x111e6488 cmp w8, #8
{let a=((cpu.x[8] as u32 as u64));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111e648c csel x0, x10, x9, lt
cpu.x[0]=(if cpu.condition(11){((cpu.x[10]))}else{((cpu.x[9]))}) as u64;
// 0x111e6490 bl #0x1128df18
cpu.x[30]=0x111e6494;super::dispatch(cpu,rt,0x1128df18)?;
// 0x111e6494 mov x0, sp
cpu.x[0]=(((cpu.sp))) as u64;
// 0x111e6498 bl #0x1142fcc4
cpu.x[30]=0x111e649c;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
pc=0x111e649c;continue;
}
0x111e649c=>{
// 0x111e649c cbz x20, #0x111e64dc
if (cpu.x[20]) == 0 {pc=0x111e64dc;continue;}pc=0x111e64a0;continue;
}
0x111e64a0=>{
// 0x111e64a0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111e64a4 bl #0x112aef94
cpu.x[30]=0x111e64a8;super::chunk7::f_112aef94(cpu,rt,0x112aef94)?;
// 0x111e64a8 mov w1, w0
cpu.x[1]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x111e64ac ldr x0, [x19, #0x88]
{let address=(cpu.x[19]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111e64b0 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111e64b4 ldr x8, [x8, #0xc0]
{let address=(cpu.x[8]).wrapping_add(0xc0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111e64b8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x111e64bc;super::dispatch(cpu,rt,target)?;}
// 0x111e64bc mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x111e64c0 cbz x20, #0x111e64f8
if (cpu.x[20]) == 0 {pc=0x111e64f8;continue;}pc=0x111e64c4;continue;
}
0x111e64c4=>{
// 0x111e64c4 cbz x21, #0x111e6548
if (cpu.x[21]) == 0 {pc=0x111e6548;continue;}pc=0x111e64c8;continue;
}
0x111e64c8=>{
// 0x111e64c8 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x111e64cc mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x111e64d0 bl #0x114307a8
cpu.x[30]=0x111e64d4;super::chunk15::f_114307a8(cpu,rt,0x114307a8)?;
// 0x111e64d4 tbz w0, #0, #0x111e64fc
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x111e64fc;continue;}pc=0x111e64d8;continue;
}
0x111e64d8=>{
// 0x111e64d8 b #0x111e6534
pc=0x111e6534;continue;
}
0x111e64dc=>{
// 0x111e64dc mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x111e64e0 ldr x0, [x19, #0x88]
{let address=(cpu.x[19]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111e64e4 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111e64e8 ldr x8, [x8, #0xc0]
{let address=(cpu.x[8]).wrapping_add(0xc0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111e64ec blr x8
{let target=cpu.x[8];cpu.x[30]=0x111e64f0;super::dispatch(cpu,rt,target)?;}
pc=0x111e64f0;continue;
}
0x111e64f0=>{
// 0x111e64f0 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x111e64f4 cbnz x20, #0x111e64c4
if (cpu.x[20]) != 0 {pc=0x111e64c4;continue;}pc=0x111e64f8;continue;
}
0x111e64f8=>{
// 0x111e64f8 cbz x21, #0x111e6580
if (cpu.x[21]) == 0 {pc=0x111e6580;continue;}pc=0x111e64fc;continue;
}
0x111e64fc=>{
// 0x111e64fc adrp x1, #0x1033a000
cpu.x[1]=(0x1033a000u64) as u64;
// 0x111e6500 add x1, x1, #0x82e
{let a=((cpu.x[1]));let b=((0x82eu64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x111e6504 mov x0, sp
cpu.x[0]=(((cpu.sp))) as u64;
// 0x111e6508 mov x22, sp
cpu.x[22]=(((cpu.sp))) as u64;
// 0x111e650c bl #0x1142fde4
cpu.x[30]=0x111e6510;super::chunk6::f_1142fde4(cpu,rt,0x1142fde4)?;
// 0x111e6510 ldr w8, [sp, #4]
{let address=(cpu.sp).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111e6514 ldr x9, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111e6518 add x10, x22, #8
{let a=((cpu.x[22]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x111e651c mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x111e6520 cmp w8, #8
{let a=((cpu.x[8] as u32 as u64));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111e6524 csel x0, x10, x9, lt
cpu.x[0]=(if cpu.condition(11){((cpu.x[10]))}else{((cpu.x[9]))}) as u64;
// 0x111e6528 bl #0x1128df18
cpu.x[30]=0x111e652c;super::dispatch(cpu,rt,0x1128df18)?;
// 0x111e652c mov x0, sp
cpu.x[0]=(((cpu.sp))) as u64;
// 0x111e6530 bl #0x1142fcc4
cpu.x[30]=0x111e6534;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
pc=0x111e6534;continue;
}
0x111e6534=>{
// 0x111e6534 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x111e6538 bl #0x1142fcc4
cpu.x[30]=0x111e653c;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
// 0x111e653c mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x111e6540 bl #0x1187d2c0
cpu.x[30]=0x111e6544;super::dispatch(cpu,rt,0x1187d2c0)?;
// 0x111e6544 b #0x111e6580
pc=0x111e6580;continue;
}
0x111e6548=>{
// 0x111e6548 adrp x1, #0x1033a000
cpu.x[1]=(0x1033a000u64) as u64;
// 0x111e654c add x1, x1, #0x82e
{let a=((cpu.x[1]));let b=((0x82eu64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x111e6550 mov x0, sp
cpu.x[0]=(((cpu.sp))) as u64;
// 0x111e6554 mov x21, sp
cpu.x[21]=(((cpu.sp))) as u64;
// 0x111e6558 bl #0x1142fde4
cpu.x[30]=0x111e655c;super::chunk6::f_1142fde4(cpu,rt,0x1142fde4)?;
// 0x111e655c ldr w8, [sp, #4]
{let address=(cpu.sp).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111e6560 ldr x9, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111e6564 add x10, x21, #8
{let a=((cpu.x[21]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x111e6568 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x111e656c cmp w8, #8
{let a=((cpu.x[8] as u32 as u64));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111e6570 csel x0, x10, x9, lt
cpu.x[0]=(if cpu.condition(11){((cpu.x[10]))}else{((cpu.x[9]))}) as u64;
// 0x111e6574 bl #0x1128df18
cpu.x[30]=0x111e6578;super::dispatch(cpu,rt,0x1128df18)?;
// 0x111e6578 mov x0, sp
cpu.x[0]=(((cpu.sp))) as u64;
// 0x111e657c bl #0x1142fcc4
cpu.x[30]=0x111e6580;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
pc=0x111e6580;continue;
}
0x111e6580=>{
// 0x111e6580 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x111e6584 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x111e6588 bl #0x1128beac
cpu.x[30]=0x111e658c;super::chunk8::f_1128beac(cpu,rt,0x1128beac)?;
pc=0x111e658c;continue;
}
0x111e658c=>{
// 0x111e658c ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111e6590 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111e6594 ldp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x111e6598 add sp, sp, #0x40
{let a=((cpu.sp));let b=((0x40u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x111e659c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111e6370")),}}}
#[inline]
pub(super) unsafe fn f_11289a0c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x11289a0c=>{
// 0x11289a0c stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x11289a10 stp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x11289a14 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x11289a18 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x11289a1c ldr w8, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x11289a20 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x11289a24 mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
pc=0x11289a28;continue;
}
0x11289a28=>{
// 0x11289a28 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x11289a2c b.lt #0x11289a68
if cpu.condition(11){pc=0x11289a68;continue;}pc=0x11289a30;continue;
}
0x11289a30=>{
// 0x11289a30 ldr x21, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x11289a34 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x11289a38 sub x22, x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[22]=(result) as u64;}
// 0x11289a3c add x9, x21, x8, lsl #3
{let a=((cpu.x[21]));let b=(((cpu.x[8])) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
// 0x11289a40 ldur x9, [x9, #-8]
{let address=(cpu.x[9]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x11289a44 add x0, x9, #0x20
{let a=((cpu.x[9]));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x11289a48 bl #0x114307a8
cpu.x[30]=0x11289a4c;super::chunk15::f_114307a8(cpu,rt,0x114307a8)?;
pc=0x11289a4c;continue;
}
0x11289a4c=>{
// 0x11289a4c mov x8, x22
cpu.x[8]=(((cpu.x[22]))) as u64;
// 0x11289a50 tbz w0, #0, #0x11289a28
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x11289a28;continue;}pc=0x11289a54;continue;
}
0x11289a54=>{
// 0x11289a54 ldr x0, [x21, x22, lsl #3]
{let address=((cpu.x[21]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[22])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x11289a58 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x11289a5c ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x11289a60 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x11289a64 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x11289a68=>{
// 0x11289a68 mov x0, xzr
cpu.x[0]=(((0u64))) as u64;
// 0x11289a6c ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x11289a70 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x11289a74 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x11289a78 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x11289a0c")),}}}
#[inline]
pub(super) unsafe fn f_1128a740(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x1128a740=>{
// 0x1128a740 tbnz w1, #0x1f, #0x1128a764
if ((cpu.x[1] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x1128a764;continue;}pc=0x1128a744;continue;
}
0x1128a744=>{
// 0x1128a744 ldr x8, [x0, #0x38]
{let address=(cpu.x[0]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x1128a748 mov w9, #0x7fffffff
cpu.x[9]=(((0x7fffffffu64))) as u32 as u64;
// 0x1128a74c ldr x8, [x8, w1, uxtw #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[1] as u32 as u64) as u32 as u64)) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x1128a750 ldr x8, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x1128a754 ldr w8, [x8, w2, sxtw #2]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[2] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x1128a758 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x1128a75c csel w0, wzr, w8, eq
cpu.x[0]=(if cpu.condition(0){((0u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x1128a760 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x1128a764=>{
// 0x1128a764 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x1128a768 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x1128a740")),}}}
#[inline]
pub(super) unsafe fn f_1128be18(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x1128be18=>{
// 0x1128be18 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x1128be1c str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x1128be20 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x1128be24 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x1128be28 ldr w19, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[19]=(v0) as u32 as u64;}
// 0x1128be2c ldr w8, [x0, #0x28]
{let address=(cpu.x[0]).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x1128be30 add w9, w19, w1
{let a=((cpu.x[19] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x1128be34 cmp w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x1128be38 b.le #0x1128be9c
if cpu.condition(13){pc=0x1128be9c;continue;}pc=0x1128be3c;continue;
}
0x1128be3c=>{
// 0x1128be3c add w9, w8, w8, lsr #1
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[8] as u32 as u64)) as u32).wrapping_shr(1) as u64;let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x1128be40 cmp w8, #0x100
{let a=((cpu.x[8] as u32 as u64));let b=((0x100u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x1128be44 mov w10, #0x64
cpu.x[10]=(((0x64u64))) as u32 as u64;
// 0x1128be48 add w8, w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x1128be4c ldr x20, [x0, #0x30]
{let address=(cpu.x[0]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x1128be50 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x1128be54 csel w9, w10, w9, lt
cpu.x[9]=(if cpu.condition(11){((cpu.x[10] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x1128be58 add w9, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x1128be5c sxtw x8, w9
cpu.x[8]=((((cpu.x[9] as u32 as u64))) as i32 as i64 as u64) as u64;
// 0x1128be60 str w9, [x0, #0x28]
{let address=(cpu.x[0]).wrapping_add(0x28u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128be64 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x1128be68 bl #0x1187d520
cpu.x[30]=0x1128be6c;super::chunk8::f_1187d520(cpu,rt,0x1187d520)?;
pc=0x1128be6c;continue;
}
0x1128be6c=>{
// 0x1128be6c mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x1128be70 mov x2, x20
cpu.x[2]=(((cpu.x[20]))) as u64;
// 0x1128be74 mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x1128be78 mov w4, w19
cpu.x[4]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x1128be7c stp x0, x0, [x21, #0x30]
{let address=(cpu.x[21]).wrapping_add(0x30u64);let v0=cpu.x[0];let v1=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x1128be80 bl #0x112aef80
cpu.x[30]=0x1128be84;super::chunk11::f_112aef80(cpu,rt,0x112aef80)?;
pc=0x1128be84;continue;
}
0x1128be84=>{
// 0x1128be84 cbz x20, #0x1128be9c
if (cpu.x[20]) == 0 {pc=0x1128be9c;continue;}pc=0x1128be88;continue;
}
0x1128be88=>{
// 0x1128be88 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x1128be8c ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x1128be90 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x1128be94 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x1128be98 b #0x1187d360
super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;return Ok(());
}
0x1128be9c=>{
// 0x1128be9c ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x1128bea0 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x1128bea4 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x1128bea8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x1128be18")),}}}
#[inline]
pub(super) unsafe fn f_1128dc00(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x1128dc00=>{
// 0x1128dc00 ldr w10, [x0, #0xc]
{let address=(cpu.x[0]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x1128dc04 mov x8, x0
cpu.x[8]=(((cpu.x[0]))) as u64;
// 0x1128dc08 mov w9, #7
cpu.x[9]=(((0x7u64))) as u32 as u64;
// 0x1128dc0c ands w0, w1, #1
{let a=((cpu.x[1] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[0]=(result) as u32 as u64;cpu.logical_flags(result,32);}
// 0x1128dc10 mov w11, #0xd
cpu.x[11]=(((0xdu64))) as u32 as u64;
// 0x1128dc14 ror w10, w10, #0x1f
cpu.x[10]=(((((cpu.x[10] as u32 as u64))) as u32).rotate_right(((((0x1fu64))) & 31) as u32) as u64) as u32 as u64;
// 0x1128dc18 csel w9, w11, w9, ne
cpu.x[9]=(if cpu.condition(1){((cpu.x[11] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x1128dc1c add w9, w10, w9
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x1128dc20 str w9, [x8, #0xc]
{let address=(cpu.x[8]).wrapping_add(0xcu64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128dc24 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x1128dc00")),}}}
#[inline]
pub(super) unsafe fn f_1142effc(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x1142effc=>{
// 0x1142effc ldr w0, [x0, #4]
{let address=(cpu.x[0]).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
pc=0x1142f000;continue;
}
0x1142f000=>{
// 0x1142f000 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x1142effc")),}}}
#[inline]
pub(super) unsafe fn f_1187d3e0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x1187d3e0=>{
// 0x1187d3e0 adrp x16, #0x11981000
cpu.x[16]=(0x11981000u64) as u64;
// 0x1187d3e4 ldr x17, [x16, #0xa00]
{let address=(cpu.x[16]).wrapping_add(0xa00u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[17]=(v0) as u64;}
// 0x1187d3e8 add x16, x16, #0xa00
{let a=((cpu.x[16]));let b=((0xa00u64));let result=a.wrapping_add(b);cpu.x[16]=(result) as u64;}
// 0x1187d3ec br x17
{let target=cpu.x[17];if target>=0x1187d3e0 && target<0x1187d400{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
0x1187d3f0=>{
// 0x1187d3f0 adrp x16, #0x11981000
cpu.x[16]=(0x11981000u64) as u64;
// 0x1187d3f4 ldr x17, [x16, #0xa08]
{let address=(cpu.x[16]).wrapping_add(0xa08u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[17]=(v0) as u64;}
// 0x1187d3f8 add x16, x16, #0xa08
{let a=((cpu.x[16]));let b=((0xa08u64));let result=a.wrapping_add(b);cpu.x[16]=(result) as u64;}
// 0x1187d3fc br x17
{let target=cpu.x[17];if target>=0x1187d3e0 && target<0x1187d400{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
_=>return Err(format!("untranslated block {pc:#x} in 0x1187d3e0")),}}}
#[inline]
pub(super) unsafe fn f_300cffbc(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x300cffbc=>{
// 0x300cffbc paciasp

// 0x300cffc0 stp x29, x30, [sp, #-0x10]!
{let address=(cpu.sp).wrapping_add(0xfffffffffffffff0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x300cffc4 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x300cffc8 cmp x0, #0xd3
{let a=((cpu.x[0]));let b=((0xd3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x300cffcc b.hi #0x300d0010
if cpu.condition(8){super::dispatch(cpu,rt,0x300d0010)?;return Ok(());}pc=0x300cffd0;continue;
}
0x300cffd0=>{
// 0x300cffd0 nop

// 0x300cffd4 adr x8, #0x30059c54
cpu.x[8]=(0x30059c54u64) as u64;
// 0x300cffd8 mov w9, #0x30
cpu.x[9]=(((0x30u64))) as u32 as u64;
pc=0x300cffdc;continue;
}
0x300cffdc=>{
// 0x300cffdc lsr x10, x9, #1
cpu.x[10]=(((((cpu.x[9]))) as u64).wrapping_shr(((((0x1u64))) & 63) as u32) as u64) as u64;
// 0x300cffe0 add x11, x8, x10, lsl #2
{let a=((cpu.x[8]));let b=(((cpu.x[10])) as u64).wrapping_shl(2) as u64;let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
// 0x300cffe4 mvn x13, x10
cpu.x[13]=(!(((cpu.x[10])))) as u64;
// 0x300cffe8 add x9, x9, x13
{let a=((cpu.x[9]));let b=((cpu.x[13]));let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
// 0x300cffec ldr w12, [x11], #4
{let address=(cpu.x[11]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[12]=(v0) as u32 as u64;cpu.x[11]=((cpu.x[11]).wrapping_add(((0x4u64)))) as u64;}
// 0x300cfff0 cmp x12, x0
{let a=((cpu.x[12]));let b=((cpu.x[0]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x300cfff4 csel x9, x9, x10, lo
cpu.x[9]=(if cpu.condition(3){((cpu.x[9]))}else{((cpu.x[10]))}) as u64;
// 0x300cfff8 csel x8, x11, x8, lo
cpu.x[8]=(if cpu.condition(3){((cpu.x[11]))}else{((cpu.x[8]))}) as u64;
// 0x300cfffc cbnz x9, #0x300cffdc
if (cpu.x[9]) != 0 {pc=0x300cffdc;continue;}pc=0x300d0000;continue;
}
0x300d0000=>{
// 0x300d0000 ldr w0, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x300d0004 ldp x29, x30, [sp], #0x10
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x10u64)))) as u64;}
// 0x300d0008 autiasp

// 0x300d000c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x300cffbc")),}}}
