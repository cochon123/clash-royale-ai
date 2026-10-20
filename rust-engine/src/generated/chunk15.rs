#![allow(unused_parens,unreachable_code,unused_variables,unused_mut)]
use crate::{cpu::Cpu,runtime::Runtime};
use super::helpers::*;
#[inline]
pub(super) unsafe fn f_108346f0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x108346f0=>{
// 0x108346f0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x108346f0")),}}}
#[inline]
pub(super) unsafe fn f_108346fc(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x108346fc=>{
// 0x108346fc ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x108346fc")),}}}
#[inline]
pub(super) unsafe fn f_10834708(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10834708=>{
// 0x10834708 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10834708")),}}}
#[inline]
pub(super) unsafe fn f_10834760(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10834760=>{
// 0x10834760 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10834760")),}}}
#[inline]
pub(super) unsafe fn f_1083479c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x1083479c=>{
// 0x1083479c mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x108347a0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x1083479c")),}}}
#[inline]
pub(super) unsafe fn f_10990ac8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10990ac8=>{
// 0x10990ac8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10990ac8")),}}}
#[inline]
pub(super) unsafe fn f_10990cb4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10990cb4=>{
// 0x10990cb4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10990cb4")),}}}
#[inline]
pub(super) unsafe fn f_10d22804(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d22804=>{
// 0x10d22804 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d22808 str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d2280c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d22810 mov w0, #4
cpu.x[0]=(((0x4u64))) as u32 as u64;
// 0x10d22814 mov x19, x8
cpu.x[19]=(((cpu.x[8]))) as u64;
// 0x10d22818 bl #0x1187d2b0
cpu.x[30]=0x10d2281c;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
pc=0x10d2281c;continue;
}
0x10d2281c=>{
// 0x10d2281c str x0, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d22820 str wzr, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d22824 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10d22828 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10d2282c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d22804")),}}}
#[inline]
pub(super) unsafe fn f_10d249d0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d249d0=>{
// 0x10d249d0 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d249d4 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d249d8 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d249dc mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10d249e0 ldr x0, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d249e4 mov w20, w1
cpu.x[20]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x10d249e8 cbnz x0, #0x10d249f4
if (cpu.x[0]) != 0 {pc=0x10d249f4;continue;}pc=0x10d249ec;continue;
}
0x10d249ec=>{
// 0x10d249ec ldr x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d249f0 bl #0x110cbf10
cpu.x[30]=0x10d249f4;super::chunk14::f_110cbf10(cpu,rt,0x110cbf10)?;
pc=0x10d249f4;continue;
}
0x10d249f4=>{
// 0x10d249f4 bl #0x10e1a324
cpu.x[30]=0x10d249f8;super::chunk3::f_10e1a324(cpu,rt,0x10e1a324)?;
pc=0x10d249f8;continue;
}
0x10d249f8=>{
// 0x10d249f8 cmp w20, w0
{let a=((cpu.x[20] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d249fc csel w8, w20, w0, lt
cpu.x[8]=(if cpu.condition(11){((cpu.x[20] as u32 as u64))}else{((cpu.x[0] as u32 as u64))}) as u32 as u64;
// 0x10d24a00 cmp w20, #1
{let a=((cpu.x[20] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d24a04 csinc w8, w8, wzr, gt
cpu.x[8]=(if cpu.condition(12){((cpu.x[8] as u32 as u64))}else{(((0u64))).wrapping_add(1)}) as u32 as u64;
// 0x10d24a08 str w8, [x19, #0xe8]
{let address=(cpu.x[19]).wrapping_add(0xe8u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d24a0c ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d24a10 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10d24a14 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d249d0")),}}}
#[inline]
pub(super) unsafe fn f_10d24ff4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d24ff4=>{
// 0x10d24ff4 stp x29, x30, [sp, #-0x60]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffa0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d24ff8 stp x28, x27, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[28];let v1=cpu.x[27];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d24ffc stp x26, x25, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[26];let v1=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
pc=0x10d25000;continue;
}
0x10d25000=>{
// 0x10d25000 stp x24, x23, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d25004 stp x22, x21, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d25008 stp x20, x19, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d2500c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d25010 add x27, x0, w1, sxtw #3
{let a=((cpu.x[0]));let b=((((cpu.x[1] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[27]=(result) as u64;}
// 0x10d25014 mov w21, w4
cpu.x[21]=(((cpu.x[4] as u32 as u64))) as u32 as u64;
// 0x10d25018 mov w24, w3
cpu.x[24]=(((cpu.x[3] as u32 as u64))) as u32 as u64;
// 0x10d2501c mov w19, w1
cpu.x[19]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x10d25020 mov x22, x0
cpu.x[22]=(((cpu.x[0]))) as u64;
// 0x10d25024 mov x20, x2
cpu.x[20]=(((cpu.x[2]))) as u64;
// 0x10d25028 ldr x23, [x27, #0xa0]!
{let address=(cpu.x[27]).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;cpu.x[27]=(address) as u64;}
// 0x10d2502c cbz x23, #0x10d25040
if (cpu.x[23]) == 0 {pc=0x10d25040;continue;}pc=0x10d25030;continue;
}
0x10d25030=>{
// 0x10d25030 ldr w8, [x23, #0xc]
{let address=(cpu.x[23]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d25034 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d25038 b.ge #0x10d2505c
if cpu.condition(10){pc=0x10d2505c;continue;}pc=0x10d2503c;continue;
}
0x10d2503c=>{
// 0x10d2503c b #0x10d250a0
pc=0x10d250a0;continue;
}
0x10d25040=>{
// 0x10d25040 adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x10d25044 add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d25048 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d2504c bl #0x1128df18
cpu.x[30]=0x10d25050;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10d25050 ldr w8, [x23, #0xc]
{let address=(cpu.x[23]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d25054 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d25058 b.lt #0x10d250a0
if cpu.condition(11){pc=0x10d250a0;continue;}pc=0x10d2505c;continue;
}
0x10d2505c=>{
// 0x10d2505c lsl x26, x8, #3
cpu.x[26]=(((((cpu.x[8]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10d25060 mov x25, xzr
cpu.x[25]=(((0u64))) as u64;
// 0x10d25064 b #0x10d25074
pc=0x10d25074;continue;
}
0x10d25068=>{
// 0x10d25068 add x25, x25, #8
{let a=((cpu.x[25]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[25]=(result) as u64;}
// 0x10d2506c cmp x26, x25
{let a=((cpu.x[26]));let b=((cpu.x[25]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d25070 b.eq #0x10d250a0
if cpu.condition(0){pc=0x10d250a0;continue;}pc=0x10d25074;continue;
}
0x10d25074=>{
// 0x10d25074 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d25078 ldr x8, [x8, x25]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[25])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d2507c cbz x8, #0x10d25068
if (cpu.x[8]) == 0 {pc=0x10d25068;continue;}pc=0x10d25080;continue;
}
0x10d25080=>{
// 0x10d25080 ldr x0, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d25084 cbz x0, #0x10d25068
if (cpu.x[0]) == 0 {pc=0x10d25068;continue;}pc=0x10d25088;continue;
}
0x10d25088=>{
// 0x10d25088 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d2508c bl #0x10e0bc20
cpu.x[30]=0x10d25090;super::chunk1::f_10e0bc20(cpu,rt,0x10e0bc20)?;
pc=0x10d25090;continue;
}
0x10d25090=>{
// 0x10d25090 tbz w0, #0, #0x10d25068
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d25068;continue;}pc=0x10d25094;continue;
}
0x10d25094=>{
// 0x10d25094 mov x8, #0x7fffffff8
cpu.x[8]=(((0x7fffffff8u64))) as u64;
// 0x10d25098 cmp x25, x8
{let a=((cpu.x[25]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d2509c b.ne #0x10d2512c
if cpu.condition(1){pc=0x10d2512c;continue;}pc=0x10d250a0;continue;
}
0x10d250a0=>{
// 0x10d250a0 mov w23, wzr
cpu.x[23]=(((0u64))) as u32 as u64;
// 0x10d250a4 ldr x25, [x27]
{let address=(cpu.x[27]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[25]=(v0) as u64;}
// 0x10d250a8 ldr w8, [x25, #0xc]
{let address=(cpu.x[25]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d250ac cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d250b0 b.lt #0x10d25148
if cpu.condition(11){pc=0x10d25148;continue;}pc=0x10d250b4;continue;
}
0x10d250b4=>{
// 0x10d250b4 lsl x28, x8, #3
cpu.x[28]=(((((cpu.x[8]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10d250b8 mov x26, xzr
cpu.x[26]=(((0u64))) as u64;
// 0x10d250bc b #0x10d250cc
pc=0x10d250cc;continue;
}
0x10d250c0=>{
// 0x10d250c0 add x26, x26, #8
{let a=((cpu.x[26]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[26]=(result) as u64;}
// 0x10d250c4 cmp x28, x26
{let a=((cpu.x[28]));let b=((cpu.x[26]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d250c8 b.eq #0x10d25148
if cpu.condition(0){pc=0x10d25148;continue;}pc=0x10d250cc;continue;
}
0x10d250cc=>{
// 0x10d250cc ldr x8, [x25]
{let address=(cpu.x[25]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d250d0 ldr x8, [x8, x26]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[26])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d250d4 cbz x8, #0x10d250c0
if (cpu.x[8]) == 0 {pc=0x10d250c0;continue;}pc=0x10d250d8;continue;
}
0x10d250d8=>{
// 0x10d250d8 ldr x0, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d250dc cbz x0, #0x10d250c0
if (cpu.x[0]) == 0 {pc=0x10d250c0;continue;}pc=0x10d250e0;continue;
}
0x10d250e0=>{
// 0x10d250e0 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d250e4 bl #0x10e0bc20
cpu.x[30]=0x10d250e8;super::chunk1::f_10e0bc20(cpu,rt,0x10e0bc20)?;
pc=0x10d250e8;continue;
}
0x10d250e8=>{
// 0x10d250e8 tbz w0, #0, #0x10d250c0
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d250c0;continue;}pc=0x10d250ec;continue;
}
0x10d250ec=>{
// 0x10d250ec mov x8, #0x7fffffff8
cpu.x[8]=(((0x7fffffff8u64))) as u64;
// 0x10d250f0 cmp x26, x8
{let a=((cpu.x[26]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d250f4 b.eq #0x10d25148
if cpu.condition(0){pc=0x10d25148;continue;}pc=0x10d250f8;continue;
}
0x10d250f8=>{
// 0x10d250f8 ldr x8, [x25]
{let address=(cpu.x[25]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d250fc ldr x8, [x8, x26]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[26])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d25100 str w24, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=cpu.x[24] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d25104 ldr x24, [x27]
{let address=(cpu.x[27]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[24]=(v0) as u64;}
// 0x10d25108 cbnz x24, #0x10d251ac
if (cpu.x[24]) != 0 {pc=0x10d251ac;continue;}pc=0x10d2510c;continue;
}
0x10d2510c=>{
// 0x10d2510c adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x10d25110 add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d25114 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d25118 bl #0x1128df18
cpu.x[30]=0x10d2511c;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10d2511c ldr w8, [x24, #0xc]
{let address=(cpu.x[24]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d25120 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d25124 b.ge #0x10d251b8
if cpu.condition(10){pc=0x10d251b8;continue;}pc=0x10d25128;continue;
}
0x10d25128=>{
// 0x10d25128 b #0x10d251fc
pc=0x10d251fc;continue;
}
0x10d2512c=>{
// 0x10d2512c ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d25130 ldr x8, [x8, x25]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[25])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d25134 ldr w23, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[23]=(v0) as u32 as u64;}
// 0x10d25138 ldr x25, [x27]
{let address=(cpu.x[27]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[25]=(v0) as u64;}
// 0x10d2513c ldr w8, [x25, #0xc]
{let address=(cpu.x[25]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d25140 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d25144 b.ge #0x10d250b4
if cpu.condition(10){pc=0x10d250b4;continue;}pc=0x10d25148;continue;
}
0x10d25148=>{
// 0x10d25148 mov w0, #0x20
cpu.x[0]=(((0x20u64))) as u32 as u64;
// 0x10d2514c bl #0x1187d2b0
cpu.x[30]=0x10d25150;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
pc=0x10d25150;continue;
}
0x10d25150=>{
// 0x10d25150 ldr x1, [x22, #0x10]
{let address=(cpu.x[22]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10d25154 mov x2, x20
cpu.x[2]=(((cpu.x[20]))) as u64;
// 0x10d25158 mov w3, w24
cpu.x[3]=(((cpu.x[24] as u32 as u64))) as u32 as u64;
// 0x10d2515c mov x26, x0
cpu.x[26]=(((cpu.x[0]))) as u64;
// 0x10d25160 bl #0x111e4f24
cpu.x[30]=0x10d25164;super::chunk12::f_111e4f24(cpu,rt,0x111e4f24)?;
pc=0x10d25164;continue;
}
0x10d25164=>{
// 0x10d25164 ldp w9, w8, [x25, #8]
{let address=(cpu.x[25]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10d25168 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d2516c b.ne #0x10d25194
if cpu.condition(1){pc=0x10d25194;continue;}pc=0x10d25170;continue;
}
0x10d25170=>{
// 0x10d25170 lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10d25174 mov w10, #5
cpu.x[10]=(((0x5u64))) as u32 as u64;
// 0x10d25178 cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d2517c csel w1, w10, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[10] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10d25180 cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d25184 b.ge #0x10d25194
if cpu.condition(10){pc=0x10d25194;continue;}pc=0x10d25188;continue;
}
0x10d25188=>{
// 0x10d25188 mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10d2518c bl #0x10d2d0f8
cpu.x[30]=0x10d25190;super::chunk4::f_10d2d0f8(cpu,rt,0x10d2d0f8)?;
pc=0x10d25190;continue;
}
0x10d25190=>{
// 0x10d25190 ldr w8, [x25, #0xc]
{let address=(cpu.x[25]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
pc=0x10d25194;continue;
}
0x10d25194=>{
// 0x10d25194 ldr x9, [x25]
{let address=(cpu.x[25]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d25198 add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10d2519c str w10, [x25, #0xc]
{let address=(cpu.x[25]).wrapping_add(0xcu64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d251a0 str x26, [x9, w8, sxtw #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64);let v0=cpu.x[26];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d251a4 ldr x24, [x27]
{let address=(cpu.x[27]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[24]=(v0) as u64;}
// 0x10d251a8 cbz x24, #0x10d2510c
if (cpu.x[24]) == 0 {pc=0x10d2510c;continue;}pc=0x10d251ac;continue;
}
0x10d251ac=>{
// 0x10d251ac ldr w8, [x24, #0xc]
{let address=(cpu.x[24]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d251b0 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d251b4 b.lt #0x10d251fc
if cpu.condition(11){pc=0x10d251fc;continue;}pc=0x10d251b8;continue;
}
0x10d251b8=>{
// 0x10d251b8 lsl x26, x8, #3
cpu.x[26]=(((((cpu.x[8]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10d251bc mov x25, xzr
cpu.x[25]=(((0u64))) as u64;
// 0x10d251c0 b #0x10d251d0
pc=0x10d251d0;continue;
}
0x10d251c4=>{
// 0x10d251c4 add x25, x25, #8
{let a=((cpu.x[25]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[25]=(result) as u64;}
// 0x10d251c8 cmp x26, x25
{let a=((cpu.x[26]));let b=((cpu.x[25]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d251cc b.eq #0x10d251fc
if cpu.condition(0){pc=0x10d251fc;continue;}pc=0x10d251d0;continue;
}
0x10d251d0=>{
// 0x10d251d0 ldr x8, [x24]
{let address=(cpu.x[24]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d251d4 ldr x8, [x8, x25]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[25])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d251d8 cbz x8, #0x10d251c4
if (cpu.x[8]) == 0 {pc=0x10d251c4;continue;}pc=0x10d251dc;continue;
}
0x10d251dc=>{
// 0x10d251dc ldr x0, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d251e0 cbz x0, #0x10d251c4
if (cpu.x[0]) == 0 {pc=0x10d251c4;continue;}pc=0x10d251e4;continue;
}
0x10d251e4=>{
// 0x10d251e4 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d251e8 bl #0x10e0bc20
cpu.x[30]=0x10d251ec;super::chunk1::f_10e0bc20(cpu,rt,0x10e0bc20)?;
pc=0x10d251ec;continue;
}
0x10d251ec=>{
// 0x10d251ec tbz w0, #0, #0x10d251c4
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d251c4;continue;}pc=0x10d251f0;continue;
}
0x10d251f0=>{
// 0x10d251f0 mov x8, #0x7fffffff8
cpu.x[8]=(((0x7fffffff8u64))) as u64;
// 0x10d251f4 cmp x25, x8
{let a=((cpu.x[25]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d251f8 b.ne #0x10d25244
if cpu.condition(1){pc=0x10d25244;continue;}pc=0x10d251fc;continue;
}
0x10d251fc=>{
// 0x10d251fc mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10d25200 cmp wzr, w23
{let a=((0u64));let b=((cpu.x[23] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d25204 b.eq #0x10d25258
if cpu.condition(0){pc=0x10d25258;continue;}pc=0x10d25208;continue;
}
0x10d25208=>{
// 0x10d25208 tbz w21, #0, #0x10d25258
if ((cpu.x[21] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d25258;continue;}pc=0x10d2520c;continue;
}
0x10d2520c=>{
// 0x10d2520c ldr x0, [x22, #0x100]
{let address=(cpu.x[22]).wrapping_add(0x100u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d25210 mov w1, w19
cpu.x[1]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10d25214 mov x2, x20
cpu.x[2]=(((cpu.x[20]))) as u64;
// 0x10d25218 mov w4, wzr
cpu.x[4]=(((0u64))) as u32 as u64;
// 0x10d2521c ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d25220 ldr x6, [x8, #0x50]
{let address=(cpu.x[8]).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[6]=(v0) as u64;}
// 0x10d25224 mov w5, w23
cpu.x[5]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10d25228 ldp x20, x19, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d2522c ldp x22, x21, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d25230 ldp x24, x23, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10d25234 ldp x26, x25, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10d25238 ldp x28, x27, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10d2523c ldp x29, x30, [sp], #0x60
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x60u64)))) as u64;}
// 0x10d25240 br x6
{let target=cpu.x[6];if target>=0x10d24ff4 && target<0x10d25274{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
0x10d25244=>{
// 0x10d25244 ldr x8, [x24]
{let address=(cpu.x[24]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d25248 ldr x8, [x8, x25]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[25])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d2524c ldr w3, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10d25250 cmp w3, w23
{let a=((cpu.x[3] as u32 as u64));let b=((cpu.x[23] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d25254 b.ne #0x10d25208
if cpu.condition(1){pc=0x10d25208;continue;}pc=0x10d25258;continue;
}
0x10d25258=>{
// 0x10d25258 ldp x20, x19, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d2525c ldp x22, x21, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d25260 ldp x24, x23, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10d25264 ldp x26, x25, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10d25268 ldp x28, x27, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10d2526c ldp x29, x30, [sp], #0x60
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x60u64)))) as u64;}
// 0x10d25270 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d24ff4")),}}}
#[inline]
pub(super) unsafe fn f_10d2d0ac(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d2d0ac=>{
// 0x10d2d0ac stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d2d0b0 str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d2d0b4 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d2d0b8 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10d2d0bc ldr x0, [x0, #0x170]
{let address=(cpu.x[0]).wrapping_add(0x170u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d2d0c0 adrp x8, #0x1190d000
cpu.x[8]=(0x1190d000u64) as u64;
// 0x10d2d0c4 add x8, x8, #0x4b8
{let a=((cpu.x[8]));let b=((0x4b8u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d2d0c8 str x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d2d0cc cbz x0, #0x10d2d0d4
if (cpu.x[0]) == 0 {pc=0x10d2d0d4;continue;}pc=0x10d2d0d0;continue;
}
0x10d2d0d0=>{
// 0x10d2d0d0 bl #0x1187d360
cpu.x[30]=0x10d2d0d4;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x10d2d0d4;continue;
}
0x10d2d0d4=>{
// 0x10d2d0d4 add x0, x19, #0x120
{let a=((cpu.x[19]));let b=((0x120u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d2d0d8 stp xzr, xzr, [x19, #0x170]
{let address=(cpu.x[19]).wrapping_add(0x170u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d2d0dc bl #0x1142fcc4
cpu.x[30]=0x10d2d0e0;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
pc=0x10d2d0e0;continue;
}
0x10d2d0e0=>{
// 0x10d2d0e0 add x0, x19, #0x78
{let a=((cpu.x[19]));let b=((0x78u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d2d0e4 bl #0x1142fcc4
cpu.x[30]=0x10d2d0e8;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
pc=0x10d2d0e8;continue;
}
0x10d2d0e8=>{
// 0x10d2d0e8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d2d0ec ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10d2d0f0 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10d2d0f4 b #0x1187d2c0
super::dispatch(cpu,rt,0x1187d2c0)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d2d0ac")),}}}
#[inline]
pub(super) unsafe fn f_10d2d56c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d2d56c=>{
// 0x10d2d56c b #0x1187d2c0
super::dispatch(cpu,rt,0x1187d2c0)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d2d56c")),}}}
#[inline]
pub(super) unsafe fn f_10d2d578(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d2d578=>{
// 0x10d2d578 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d2d578")),}}}
#[inline]
pub(super) unsafe fn f_10d2d58c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d2d58c=>{
// 0x10d2d58c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d2d58c")),}}}
#[inline]
pub(super) unsafe fn f_10d32c84(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d32c84=>{
// 0x10d32c84 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d32c88 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d32c8c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d32c90 mov w19, w1
cpu.x[19]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x10d32c94 cmp w1, #6
{let a=((cpu.x[1] as u32 as u64));let b=((0x6u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d32c98 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10d32c9c b.lo #0x10d32cb0
if cpu.condition(3){pc=0x10d32cb0;continue;}pc=0x10d32ca0;continue;
}
0x10d32ca0=>{
// 0x10d32ca0 adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x10d32ca4 add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d32ca8 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d32cac bl #0x1128df18
cpu.x[30]=0x10d32cb0;super::dispatch(cpu,rt,0x1128df18)?;
pc=0x10d32cb0;continue;
}
0x10d32cb0=>{
// 0x10d32cb0 add x8, x20, w19, sxtw #3
{let a=((cpu.x[20]));let b=((((cpu.x[19] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d32cb4 ldr x0, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d32cb8 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d32cbc ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10d32cc0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d32c84")),}}}
#[inline]
pub(super) unsafe fn f_10d359a4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d359a4=>{
// 0x10d359a4 ldr x0, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d359a8 b #0x110cb308
super::chunk15::f_110cb308(cpu,rt,0x110cb308)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d359a4")),}}}
#[inline]
pub(super) unsafe fn f_10d37038(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d37038=>{
// 0x10d37038 sub sp, sp, #0x80
{let a=((cpu.sp));let b=((0x80u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x10d3703c stp x29, x30, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d37040 stp x28, x27, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[28];let v1=cpu.x[27];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d37044 stp x26, x25, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[26];let v1=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d37048 stp x24, x23, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d3704c stp x22, x21, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d37050 stp x20, x19, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d37054 add x29, sp, #0x20
{let a=((cpu.sp));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x10d37058 mov w23, w1
cpu.x[23]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x10d3705c cmp w1, #6
{let a=((cpu.x[1] as u32 as u64));let b=((0x6u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d37060 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10d37064 b.hs #0x10d37074
if cpu.condition(2){pc=0x10d37074;continue;}pc=0x10d37068;continue;
}
0x10d37068=>{
// 0x10d37068 add x8, x19, w23, uxtw #3
{let a=((cpu.x[19]));let b=((((cpu.x[23] as u32 as u64) as u32 as u64)) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d3706c ldr x25, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[25]=(v0) as u64;}
// 0x10d37070 b #0x10d370f4
pc=0x10d370f4;continue;
}
0x10d37074=>{
// 0x10d37074 adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x10d37078 add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d3707c mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d37080 bl #0x1128df18
cpu.x[30]=0x10d37084;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10d37084 cmp w23, #0x64
{let a=((cpu.x[23] as u32 as u64));let b=((0x64u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d37088 b.ne #0x10d3709c
if cpu.condition(1){pc=0x10d3709c;continue;}pc=0x10d3708c;continue;
}
0x10d3708c=>{
// 0x10d3708c mov x21, xzr
cpu.x[21]=(((0u64))) as u64;
// 0x10d37090 mov x25, xzr
cpu.x[25]=(((0u64))) as u64;
// 0x10d37094 mov w20, #0x64
cpu.x[20]=(((0x64u64))) as u32 as u64;
// 0x10d37098 b #0x10d37270
pc=0x10d37270;continue;
}
0x10d3709c=>{
// 0x10d3709c cmp w23, #6
{let a=((cpu.x[23] as u32 as u64));let b=((0x6u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d370a0 b.lt #0x10d370e0
if cpu.condition(11){pc=0x10d370e0;continue;}pc=0x10d370a4;continue;
}
0x10d370a4=>{
// 0x10d370a4 adrp x0, #0x102e0000
cpu.x[0]=(0x102e0000u64) as u64;
// 0x10d370a8 add x0, x0, #0x6ab
{let a=((cpu.x[0]));let b=((0x6abu64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d370ac add x8, sp, #0x10
{let a=((cpu.sp));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d370b0 mov w1, w23
cpu.x[1]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10d370b4 add x20, sp, #0x10
{let a=((cpu.sp));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[20]=(result) as u64;}
// 0x10d370b8 bl #0x114328cc
cpu.x[30]=0x10d370bc;super::chunk0::f_114328cc(cpu,rt,0x114328cc)?;
// 0x10d370bc ldr w8, [sp, #0x14]
{let address=(cpu.sp).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d370c0 ldr x9, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d370c4 add x10, x20, #8
{let a=((cpu.x[20]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x10d370c8 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d370cc cmp w8, #8
{let a=((cpu.x[8] as u32 as u64));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d370d0 csel x0, x10, x9, lt
cpu.x[0]=(if cpu.condition(11){((cpu.x[10]))}else{((cpu.x[9]))}) as u64;
// 0x10d370d4 bl #0x1128df18
cpu.x[30]=0x10d370d8;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10d370d8 add x0, sp, #0x10
{let a=((cpu.sp));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d370dc bl #0x1142fcc4
cpu.x[30]=0x10d370e0;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
pc=0x10d370e0;continue;
}
0x10d370e0=>{
// 0x10d370e0 adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x10d370e4 add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d370e8 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d370ec bl #0x1128df18
cpu.x[30]=0x10d370f0;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10d370f0 mov x25, xzr
cpu.x[25]=(((0u64))) as u64;
pc=0x10d370f4;continue;
}
0x10d370f4=>{
// 0x10d370f4 add x8, x19, w23, sxtw #3
{let a=((cpu.x[19]));let b=((((cpu.x[23] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d370f8 ldr x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d370fc sxtw x20, w23
cpu.x[20]=((((cpu.x[23] as u32 as u64))) as i32 as i64 as u64) as u64;
// 0x10d37100 ldr x8, [x8, #0x30]
{let address=(cpu.x[8]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d37104 ldp w1, w2, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[1]=(v0) as u32 as u64;cpu.x[2]=(v1) as u32 as u64;}
// 0x10d37108 bl #0x110cc5b4
cpu.x[30]=0x10d3710c;super::chunk13::f_110cc5b4(cpu,rt,0x110cc5b4)?;
pc=0x10d3710c;continue;
}
0x10d3710c=>{
// 0x10d3710c cmp w23, #6
{let a=((cpu.x[23] as u32 as u64));let b=((0x6u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d37110 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10d37114 b.hs #0x10d37270
if cpu.condition(2){pc=0x10d37270;continue;}pc=0x10d37118;continue;
}
0x10d37118=>{
// 0x10d37118 add x8, x19, x20, lsl #3
{let a=((cpu.x[19]));let b=(((cpu.x[20])) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d3711c ldr x22, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[22]=(v0) as u64;}
// 0x10d37120 cbz x25, #0x10d3728c
if (cpu.x[25]) == 0 {pc=0x10d3728c;continue;}pc=0x10d37124;continue;
}
0x10d37124=>{
// 0x10d37124 cbz x21, #0x10d372a0
if (cpu.x[21]) == 0 {pc=0x10d372a0;continue;}pc=0x10d37128;continue;
}
0x10d37128=>{
// 0x10d37128 cbz x22, #0x10d372b4
if (cpu.x[22]) == 0 {pc=0x10d372b4;continue;}pc=0x10d3712c;continue;
}
0x10d3712c=>{
// 0x10d3712c ldr x8, [x19, #0x1b0]
{let address=(cpu.x[19]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d37130 ldr x8, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d37134 cbz x8, #0x10d3714c
if (cpu.x[8]) == 0 {pc=0x10d3714c;continue;}pc=0x10d37138;continue;
}
0x10d37138=>{
// 0x10d37138 ldr x0, [x19, #0x190]
{let address=(cpu.x[19]).wrapping_add(0x190u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3713c bl #0x10d5676c
cpu.x[30]=0x10d37140;super::chunk10::f_10d5676c(cpu,rt,0x10d5676c)?;
pc=0x10d37140;continue;
}
0x10d37140=>{
// 0x10d37140 mov w1, w0
cpu.x[1]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d37144 mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10d37148 bl #0x10f9c6c0
cpu.x[30]=0x10d3714c;super::chunk2::f_10f9c6c0(cpu,rt,0x10f9c6c0)?;
pc=0x10d3714c;continue;
}
0x10d3714c=>{
// 0x10d3714c ldr x8, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d37150 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10d37154 ldr x8, [x8, #0x30]
{let address=(cpu.x[8]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d37158 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3715c;super::dispatch(cpu,rt,target)?;}
pc=0x10d3715c;continue;
}
0x10d3715c=>{
// 0x10d3715c tbz w0, #0, #0x10d37184
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d37184;continue;}pc=0x10d37160;continue;
}
0x10d37160=>{
// 0x10d37160 ldr x0, [x19, #0xc8]
{let address=(cpu.x[19]).wrapping_add(0xc8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d37164 cbz x0, #0x10d37184
if (cpu.x[0]) == 0 {pc=0x10d37184;continue;}pc=0x10d37168;continue;
}
0x10d37168=>{
// 0x10d37168 bl #0x10e5594c
cpu.x[30]=0x10d3716c;super::dispatch(cpu,rt,0x10e5594c)?;
// 0x10d3716c tbnz w0, #0x1f, #0x10d37184
if ((cpu.x[0] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x10d37184;continue;}pc=0x10d37170;continue;
}
0x10d37170=>{
// 0x10d37170 ldr x0, [x19, #0xc8]
{let address=(cpu.x[19]).wrapping_add(0xc8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d37174 bl #0x10e5594c
cpu.x[30]=0x10d37178;super::dispatch(cpu,rt,0x10e5594c)?;
// 0x10d37178 mov w1, w0
cpu.x[1]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d3717c mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10d37180 bl #0x10f9c6c0
cpu.x[30]=0x10d37184;super::chunk2::f_10f9c6c0(cpu,rt,0x10f9c6c0)?;
pc=0x10d37184;continue;
}
0x10d37184=>{
// 0x10d37184 cmp w23, #1
{let a=((cpu.x[23] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d37188 b.ne #0x10d372d4
if cpu.condition(1){pc=0x10d372d4;continue;}pc=0x10d3718c;continue;
}
0x10d3718c=>{
// 0x10d3718c ldr w8, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d37190 mov w28, wzr
cpu.x[28]=(((0u64))) as u32 as u64;
// 0x10d37194 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d37198 b.ne #0x10d372d8
if cpu.condition(1){pc=0x10d372d8;continue;}pc=0x10d3719c;continue;
}
0x10d3719c=>{
// 0x10d3719c ldr x8, [x19, #0xc8]
{let address=(cpu.x[19]).wrapping_add(0xc8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d371a0 cbz x8, #0x10d372d8
if (cpu.x[8]) == 0 {pc=0x10d372d8;continue;}pc=0x10d371a4;continue;
}
0x10d371a4=>{
// 0x10d371a4 ldr x0, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d371a8 mov w1, #0x30
cpu.x[1]=(((0x30u64))) as u32 as u64;
// 0x10d371ac bl #0x10e1552c
cpu.x[30]=0x10d371b0;super::chunk12::f_10e1552c(cpu,rt,0x10e1552c)?;
// 0x10d371b0 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d371b4 mov x23, x0
cpu.x[23]=(((cpu.x[0]))) as u64;
// 0x10d371b8 ldr x8, [x8, #0x20]
{let address=(cpu.x[8]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d371bc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d371c0;super::dispatch(cpu,rt,target)?;}
// 0x10d371c0 cmp w0, #0
{let a=((cpu.x[0] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d371c4 b.le #0x10d3737c
if cpu.condition(13){pc=0x10d3737c;continue;}pc=0x10d371c8;continue;
}
0x10d371c8=>{
// 0x10d371c8 mov w24, w0
cpu.x[24]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d371cc str x25, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d371d0 mov w28, wzr
cpu.x[28]=(((0u64))) as u32 as u64;
// 0x10d371d4 mov w25, wzr
cpu.x[25]=(((0u64))) as u32 as u64;
// 0x10d371d8 mov w20, #1
cpu.x[20]=(((0x1u64))) as u32 as u64;
// 0x10d371dc str wzr, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d371e0 b #0x10d371f8
pc=0x10d371f8;continue;
}
0x10d371e4=>{
// 0x10d371e4 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10d371e8 str w8, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10d371ec;continue;
}
0x10d371ec=>{
// 0x10d371ec add w25, w25, #1
{let a=((cpu.x[25] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[25]=(result) as u32 as u64;}
// 0x10d371f0 cmp w24, w25
{let a=((cpu.x[24] as u32 as u64));let b=((cpu.x[25] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d371f4 b.eq #0x10d3733c
if cpu.condition(0){pc=0x10d3733c;continue;}pc=0x10d371f8;continue;
}
0x10d371f8=>{
// 0x10d371f8 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d371fc mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d37200 mov w1, w25
cpu.x[1]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10d37204 ldr x8, [x8, #0x30]
{let address=(cpu.x[8]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d37208 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d3720c;super::dispatch(cpu,rt,target)?;}
// 0x10d3720c ldr x27, [x19, #0xb8]
{let address=(cpu.x[19]).wrapping_add(0xb8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[27]=(v0) as u64;}
// 0x10d37210 mov x26, x0
cpu.x[26]=(((cpu.x[0]))) as u64;
// 0x10d37214 bl #0x10e9dd24
cpu.x[30]=0x10d37218;super::dispatch(cpu,rt,0x10e9dd24)?;
// 0x10d37218 mov x1, x0
cpu.x[1]=(((cpu.x[0]))) as u64;
// 0x10d3721c mov x0, x27
cpu.x[0]=(((cpu.x[27]))) as u64;
// 0x10d37220 bl #0x10e0bc20
cpu.x[30]=0x10d37224;super::chunk1::f_10e0bc20(cpu,rt,0x10e0bc20)?;
// 0x10d37224 tbz w0, #0, #0x10d371ec
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d371ec;continue;}pc=0x10d37228;continue;
}
0x10d37228=>{
// 0x10d37228 ldr x27, [x19, #0xc8]
{let address=(cpu.x[19]).wrapping_add(0xc8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[27]=(v0) as u64;}
// 0x10d3722c mov x0, x26
cpu.x[0]=(((cpu.x[26]))) as u64;
// 0x10d37230 bl #0x10e9dd2c
cpu.x[30]=0x10d37234;super::dispatch(cpu,rt,0x10e9dd2c)?;
// 0x10d37234 mov x1, x0
cpu.x[1]=(((cpu.x[0]))) as u64;
// 0x10d37238 mov x0, x27
cpu.x[0]=(((cpu.x[27]))) as u64;
// 0x10d3723c bl #0x10e0bc20
cpu.x[30]=0x10d37240;super::chunk1::f_10e0bc20(cpu,rt,0x10e0bc20)?;
// 0x10d37240 tbz w0, #0, #0x10d371ec
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d371ec;continue;}pc=0x10d37244;continue;
}
0x10d37244=>{
// 0x10d37244 mov x0, x26
cpu.x[0]=(((cpu.x[26]))) as u64;
// 0x10d37248 bl #0x10e9dd34
cpu.x[30]=0x10d3724c;super::dispatch(cpu,rt,0x10e9dd34)?;
// 0x10d3724c cbz x0, #0x10d371e4
if (cpu.x[0]) == 0 {pc=0x10d371e4;continue;}pc=0x10d37250;continue;
}
0x10d37250=>{
// 0x10d37250 ldr x8, [x22]
{let address=(cpu.x[22]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d37254 mov x1, x0
cpu.x[1]=(((cpu.x[0]))) as u64;
// 0x10d37258 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d3725c ldr x8, [x8, #0x90]
{let address=(cpu.x[8]).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d37260 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d37264;super::dispatch(cpu,rt,target)?;}
// 0x10d37264 mov w28, #1
cpu.x[28]=(((0x1u64))) as u32 as u64;
// 0x10d37268 and w20, w0, w20
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[20] as u32 as u64));let result=a & b;cpu.x[20]=(result) as u32 as u64;}
// 0x10d3726c b #0x10d371e4
pc=0x10d371e4;continue;
}
0x10d37270=>{
// 0x10d37270 adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x10d37274 add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d37278 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d3727c bl #0x1128df18
cpu.x[30]=0x10d37280;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10d37280 add x8, x19, x20, lsl #3
{let a=((cpu.x[19]));let b=(((cpu.x[20])) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d37284 ldr x22, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[22]=(v0) as u64;}
// 0x10d37288 cbnz x25, #0x10d37124
if (cpu.x[25]) != 0 {pc=0x10d37124;continue;}pc=0x10d3728c;continue;
}
0x10d3728c=>{
// 0x10d3728c adrp x0, #0x10327000
cpu.x[0]=(0x10327000u64) as u64;
// 0x10d37290 add x0, x0, #0xfcc
{let a=((cpu.x[0]));let b=((0xfccu64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d37294 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d37298 bl #0x1128df18
cpu.x[30]=0x10d3729c;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10d3729c cbnz x21, #0x10d37128
if (cpu.x[21]) != 0 {pc=0x10d37128;continue;}pc=0x10d372a0;continue;
}
0x10d372a0=>{
// 0x10d372a0 adrp x0, #0x10338000
cpu.x[0]=(0x10338000u64) as u64;
// 0x10d372a4 add x0, x0, #0xb4e
{let a=((cpu.x[0]));let b=((0xb4eu64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d372a8 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d372ac bl #0x1128df18
cpu.x[30]=0x10d372b0;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10d372b0 cbnz x22, #0x10d3712c
if (cpu.x[22]) != 0 {pc=0x10d3712c;continue;}pc=0x10d372b4;continue;
}
0x10d372b4=>{
// 0x10d372b4 adrp x0, #0x102e4000
cpu.x[0]=(0x102e4000u64) as u64;
// 0x10d372b8 add x0, x0, #0xd0a
{let a=((cpu.x[0]));let b=((0xd0au64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d372bc mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d372c0 bl #0x1128df18
cpu.x[30]=0x10d372c4;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10d372c4 ldr x8, [x19, #0x1b0]
{let address=(cpu.x[19]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d372c8 ldr x8, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d372cc cbnz x8, #0x10d37138
if (cpu.x[8]) != 0 {pc=0x10d37138;continue;}pc=0x10d372d0;continue;
}
0x10d372d0=>{
// 0x10d372d0 b #0x10d3714c
pc=0x10d3714c;continue;
}
0x10d372d4=>{
// 0x10d372d4 mov w28, wzr
cpu.x[28]=(((0u64))) as u32 as u64;
pc=0x10d372d8;continue;
}
0x10d372d8=>{
// 0x10d372d8 and w2, w28, #1
{let a=((cpu.x[28] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[2]=(result) as u32 as u64;}
// 0x10d372dc mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10d372e0 mov x1, x22
cpu.x[1]=(((cpu.x[22]))) as u64;
pc=0x10d372e4;continue;
}
0x10d372e4=>{
// 0x10d372e4 mov x3, x21
cpu.x[3]=(((cpu.x[21]))) as u64;
// 0x10d372e8 bl #0x10f9d820
cpu.x[30]=0x10d372ec;super::chunk3::f_10f9d820(cpu,rt,0x10f9d820)?;
pc=0x10d372ec;continue;
}
0x10d372ec=>{
// 0x10d372ec ldr x8, [x19, #0x1b0]
{let address=(cpu.x[19]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d372f0 ldr x0, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d372f4 cbz x0, #0x10d3731c
if (cpu.x[0]) == 0 {pc=0x10d3731c;continue;}pc=0x10d372f8;continue;
}
0x10d372f8=>{
// 0x10d372f8 bl #0x10e2d5a4
cpu.x[30]=0x10d372fc;super::chunk4::f_10e2d5a4(cpu,rt,0x10e2d5a4)?;
pc=0x10d372fc;continue;
}
0x10d372fc=>{
// 0x10d372fc tbnz w0, #0, #0x10d37314
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10d37314;continue;}pc=0x10d37300;continue;
}
0x10d37300=>{
// 0x10d37300 ldr x8, [x19, #0x1b0]
{let address=(cpu.x[19]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d37304 ldr x0, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d37308 cbz x0, #0x10d3731c
if (cpu.x[0]) == 0 {pc=0x10d3731c;continue;}pc=0x10d3730c;continue;
}
0x10d3730c=>{
// 0x10d3730c bl #0x10e2d928
cpu.x[30]=0x10d37310;super::chunk15::f_10e2d928(cpu,rt,0x10e2d928)?;
pc=0x10d37310;continue;
}
0x10d37310=>{
// 0x10d37310 tbz w0, #0, #0x10d3731c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d3731c;continue;}pc=0x10d37314;continue;
}
0x10d37314=>{
// 0x10d37314 mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10d37318 bl #0x10f9c640
cpu.x[30]=0x10d3731c;super::dispatch(cpu,rt,0x10f9c640)?;
pc=0x10d3731c;continue;
}
0x10d3731c=>{
// 0x10d3731c ldp x20, x19, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d37320 ldp x22, x21, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d37324 ldp x24, x23, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10d37328 ldp x26, x25, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10d3732c ldp x28, x27, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10d37330 ldp x29, x30, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10d37334 add sp, sp, #0x80
{let a=((cpu.sp));let b=((0x80u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10d37338 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10d3733c=>{
// 0x10d3733c tbz w20, #0, #0x10d3738c
if ((cpu.x[20] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d3738c;continue;}pc=0x10d37340;continue;
}
0x10d37340=>{
// 0x10d37340 ldr x25, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[25]=(v0) as u64;}
// 0x10d37344 and w2, w28, #1
{let a=((cpu.x[28] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[2]=(result) as u32 as u64;}
// 0x10d37348 mov x1, x22
cpu.x[1]=(((cpu.x[22]))) as u64;
// 0x10d3734c mov x3, x21
cpu.x[3]=(((cpu.x[21]))) as u64;
// 0x10d37350 mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10d37354 bl #0x10f9d820
cpu.x[30]=0x10d37358;super::chunk3::f_10f9d820(cpu,rt,0x10f9d820)?;
// 0x10d37358 ldr w8, [sp, #0xc]
{let address=(cpu.sp).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3735c tbz w8, #0, #0x10d372ec
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d372ec;continue;}pc=0x10d37360;continue;
}
0x10d37360=>{
// 0x10d37360 ldr x0, [x19, #0xc8]
{let address=(cpu.x[19]).wrapping_add(0xc8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d37364 ldr x21, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10d37368 bl #0x10e55954
cpu.x[30]=0x10d3736c;super::dispatch(cpu,rt,0x10e55954)?;
// 0x10d3736c mov w1, w0
cpu.x[1]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d37370 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10d37374 bl #0x110c9968
cpu.x[30]=0x10d37378;super::chunk14::f_110c9968(cpu,rt,0x110c9968)?;
// 0x10d37378 b #0x10d372ec
pc=0x10d372ec;continue;
}
0x10d3737c=>{
// 0x10d3737c mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10d37380 mov x1, x22
cpu.x[1]=(((cpu.x[22]))) as u64;
// 0x10d37384 mov w2, wzr
cpu.x[2]=(((0u64))) as u32 as u64;
// 0x10d37388 b #0x10d372e4
pc=0x10d372e4;continue;
}
0x10d3738c=>{
// 0x10d3738c cmp w24, #1
{let a=((cpu.x[24] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d37390 b.lt #0x10d37428
if cpu.condition(11){pc=0x10d37428;continue;}pc=0x10d37394;continue;
}
0x10d37394=>{
// 0x10d37394 mov w25, wzr
cpu.x[25]=(((0u64))) as u32 as u64;
// 0x10d37398 mov w20, #1
cpu.x[20]=(((0x1u64))) as u32 as u64;
// 0x10d3739c b #0x10d373ac
pc=0x10d373ac;continue;
}
0x10d373a0=>{
// 0x10d373a0 add w25, w25, #1
{let a=((cpu.x[25] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[25]=(result) as u32 as u64;}
// 0x10d373a4 cmp w24, w25
{let a=((cpu.x[24] as u32 as u64));let b=((cpu.x[25] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d373a8 b.eq #0x10d37428
if cpu.condition(0){pc=0x10d37428;continue;}pc=0x10d373ac;continue;
}
0x10d373ac=>{
// 0x10d373ac ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d373b0 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d373b4 mov w1, w25
cpu.x[1]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10d373b8 ldr x8, [x8, #0x30]
{let address=(cpu.x[8]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d373bc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d373c0;super::dispatch(cpu,rt,target)?;}
// 0x10d373c0 mov x26, x0
cpu.x[26]=(((cpu.x[0]))) as u64;
// 0x10d373c4 bl #0x10e9dc70
cpu.x[30]=0x10d373c8;super::dispatch(cpu,rt,0x10e9dc70)?;
// 0x10d373c8 tbnz w0, #0, #0x10d373a0
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10d373a0;continue;}pc=0x10d373cc;continue;
}
0x10d373cc=>{
// 0x10d373cc ldr x27, [x19, #0xb8]
{let address=(cpu.x[19]).wrapping_add(0xb8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[27]=(v0) as u64;}
// 0x10d373d0 mov x0, x26
cpu.x[0]=(((cpu.x[26]))) as u64;
// 0x10d373d4 bl #0x10e9dd24
cpu.x[30]=0x10d373d8;super::dispatch(cpu,rt,0x10e9dd24)?;
// 0x10d373d8 mov x1, x0
cpu.x[1]=(((cpu.x[0]))) as u64;
// 0x10d373dc mov x0, x27
cpu.x[0]=(((cpu.x[27]))) as u64;
// 0x10d373e0 bl #0x10e0bc20
cpu.x[30]=0x10d373e4;super::chunk1::f_10e0bc20(cpu,rt,0x10e0bc20)?;
// 0x10d373e4 tbz w0, #0, #0x10d373a0
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d373a0;continue;}pc=0x10d373e8;continue;
}
0x10d373e8=>{
// 0x10d373e8 ldr x27, [x19, #0xc8]
{let address=(cpu.x[19]).wrapping_add(0xc8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[27]=(v0) as u64;}
// 0x10d373ec mov x0, x26
cpu.x[0]=(((cpu.x[26]))) as u64;
// 0x10d373f0 bl #0x10e9dd2c
cpu.x[30]=0x10d373f4;super::dispatch(cpu,rt,0x10e9dd2c)?;
// 0x10d373f4 mov x1, x0
cpu.x[1]=(((cpu.x[0]))) as u64;
// 0x10d373f8 mov x0, x27
cpu.x[0]=(((cpu.x[27]))) as u64;
// 0x10d373fc bl #0x10e0bc20
cpu.x[30]=0x10d37400;super::chunk1::f_10e0bc20(cpu,rt,0x10e0bc20)?;
// 0x10d37400 tbz w0, #0, #0x10d373a0
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d373a0;continue;}pc=0x10d37404;continue;
}
0x10d37404=>{
// 0x10d37404 mov x0, x26
cpu.x[0]=(((cpu.x[26]))) as u64;
// 0x10d37408 bl #0x10e0b668
cpu.x[30]=0x10d3740c;super::chunk4::f_10e0b668(cpu,rt,0x10e0b668)?;
// 0x10d3740c asr w8, w0, #5
cpu.x[8]=(((((cpu.x[0] as u32 as u64))) as i32).wrapping_shr(((((0x5u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10d37410 lsl w9, w20, w0
cpu.x[9]=(((((cpu.x[20] as u32 as u64))) as u32).wrapping_shl(((((cpu.x[0] as u32 as u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10d37414 add x8, x19, w8, sxtw #2
{let a=((cpu.x[19]));let b=((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d37418 ldr w10, [x8, #0x168]
{let address=(cpu.x[8]).wrapping_add(0x168u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10d3741c orr w9, w9, w10
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a | b;cpu.x[9]=(result) as u32 as u64;}
// 0x10d37420 str w9, [x8, #0x168]
{let address=(cpu.x[8]).wrapping_add(0x168u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d37424 b #0x10d373a0
pc=0x10d373a0;continue;
}
0x10d37428=>{
// 0x10d37428 ldr x25, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[25]=(v0) as u64;}
// 0x10d3742c b #0x10d372d8
pc=0x10d372d8;continue;
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d37038")),}}}
#[inline]
pub(super) unsafe fn f_10d37e08(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d37e08=>{
// 0x10d37e08 ldr x0, [x0, #0xc0]
{let address=(cpu.x[0]).wrapping_add(0xc0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d37e0c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d37e08")),}}}
#[inline]
pub(super) unsafe fn f_10d3d2f0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d3d2f0=>{
// 0x10d3d2f0 stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d3d2f4 stp x24, x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d3d2f8 stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d3d2fc stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d3d300 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d3d304 ldr w8, [x3, #8]
{let address=(cpu.x[3]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3d308 mov w21, w4
cpu.x[21]=(((cpu.x[4] as u32 as u64))) as u32 as u64;
// 0x10d3d30c mov x19, x2
cpu.x[19]=(((cpu.x[2]))) as u64;
// 0x10d3d310 mov w22, w1
cpu.x[22]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x10d3d314 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10d3d318 cmp w8, #0xa
{let a=((cpu.x[8] as u32 as u64));let b=((0xau64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3d31c b.eq #0x10d3d358
if cpu.condition(0){pc=0x10d3d358;continue;}pc=0x10d3d320;continue;
}
0x10d3d320=>{
// 0x10d3d320 ldr x8, [x3, #0x80]
{let address=(cpu.x[3]).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3d324 cbz x8, #0x10d3d3c8
if (cpu.x[8]) == 0 {pc=0x10d3d3c8;continue;}pc=0x10d3d328;continue;
}
0x10d3d328=>{
// 0x10d3d328 ldr x23, [x20, #0x20]
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10d3d32c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d3d330 mov x24, x3
cpu.x[24]=(((cpu.x[3]))) as u64;
// 0x10d3d334 bl #0x10d26e70
cpu.x[30]=0x10d3d338;super::dispatch(cpu,rt,0x10d26e70)?;
// 0x10d3d338 mov w1, w0
cpu.x[1]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d3d33c mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10d3d340 bl #0x10e1b338
cpu.x[30]=0x10d3d344;super::dispatch(cpu,rt,0x10e1b338)?;
// 0x10d3d344 ldr x1, [x24, #0x80]
{let address=(cpu.x[24]).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10d3d348 ldr w0, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10d3d34c and w2, w22, #1
{let a=((cpu.x[22] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[2]=(result) as u32 as u64;}
// 0x10d3d350 bl #0x111e2f50
cpu.x[30]=0x10d3d354;super::dispatch(cpu,rt,0x111e2f50)?;
// 0x10d3d354 add w21, w0, w21
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[21] as u32 as u64));let result=a.wrapping_add(b);cpu.x[21]=(result) as u32 as u64;}
pc=0x10d3d358;continue;
}
0x10d3d358=>{
// 0x10d3d358 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d3d35c mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10d3d360 bl #0x10d2801c
cpu.x[30]=0x10d3d364;super::chunk3::f_10d2801c(cpu,rt,0x10d2801c)?;
// 0x10d3d364 tbz w22, #0, #0x10d3d434
if ((cpu.x[22] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d3d434;continue;}pc=0x10d3d368;continue;
}
0x10d3d368=>{
// 0x10d3d368 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d3d36c bl #0x10d28cdc
cpu.x[30]=0x10d3d370;super::chunk11::f_10d28cdc(cpu,rt,0x10d28cdc)?;
// 0x10d3d370 tbz w0, #0, #0x10d3d434
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d3d434;continue;}pc=0x10d3d374;continue;
}
0x10d3d374=>{
// 0x10d3d374 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d3d378 bl #0x10d27ee0
cpu.x[30]=0x10d3d37c;super::dispatch(cpu,rt,0x10d27ee0)?;
// 0x10d3d37c ldr x8, [x20, #0x20]
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3d380 mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d3d384 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10d3d388 bl #0x10e16208
cpu.x[30]=0x10d3d38c;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
// 0x10d3d38c ldr w8, [x0, #0x4f4]
{let address=(cpu.x[0]).wrapping_add(0x4f4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3d390 cmp w21, w8
{let a=((cpu.x[21] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3d394 b.ge #0x10d3d434
if cpu.condition(10){pc=0x10d3d434;continue;}pc=0x10d3d398;continue;
}
0x10d3d398=>{
// 0x10d3d398 ldr x0, [x20, #0x20]
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3d39c bl #0x10e16208
cpu.x[30]=0x10d3d3a0;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
// 0x10d3d3a0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d3d3a4 bl #0x10d27ee0
cpu.x[30]=0x10d3d3a8;super::dispatch(cpu,rt,0x10d27ee0)?;
// 0x10d3d3a8 mov w8, w0
cpu.x[8]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d3d3ac mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d3d3b0 add w1, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u32 as u64;}
// 0x10d3d3b4 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d3d3b8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d3d3bc ldp x24, x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10d3d3c0 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10d3d3c4 b #0x10d27fd4
super::dispatch(cpu,rt,0x10d27fd4)?;return Ok(());
}
0x10d3d3c8=>{
// 0x10d3d3c8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d3d3cc bl #0x10d27da4
cpu.x[30]=0x10d3d3d0;super::chunk6::f_10d27da4(cpu,rt,0x10d27da4)?;
pc=0x10d3d3d0;continue;
}
0x10d3d3d0=>{
// 0x10d3d3d0 ldr x8, [x20, #0x20]
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3d3d4 mov w23, w0
cpu.x[23]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d3d3d8 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10d3d3dc bl #0x10e16208
cpu.x[30]=0x10d3d3e0;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
pc=0x10d3d3e0;continue;
}
0x10d3d3e0=>{
// 0x10d3d3e0 tbz w22, #0, #0x10d3d448
if ((cpu.x[22] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d3d448;continue;}pc=0x10d3d3e4;continue;
}
0x10d3d3e4=>{
// 0x10d3d3e4 ldr w8, [x0, #0x170]
{let address=(cpu.x[0]).wrapping_add(0x170u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3d3e8 cmp w23, w8
{let a=((cpu.x[23] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3d3ec b.lt #0x10d3d400
if cpu.condition(11){pc=0x10d3d400;continue;}pc=0x10d3d3f0;continue;
}
0x10d3d3f0=>{
// 0x10d3d3f0 ldr x0, [x20, #0x20]
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3d3f4 bl #0x10e16208
cpu.x[30]=0x10d3d3f8;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
// 0x10d3d3f8 ldrb w8, [x0, #0x174]
{let address=(cpu.x[0]).wrapping_add(0x174u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3d3fc cbnz w8, #0x10d3d41c
if (cpu.x[8] as u32 as u64) != 0 {pc=0x10d3d41c;continue;}pc=0x10d3d400;continue;
}
0x10d3d400=>{
// 0x10d3d400 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d3d404 bl #0x10d25710
cpu.x[30]=0x10d3d408;super::chunk12::f_10d25710(cpu,rt,0x10d25710)?;
pc=0x10d3d408;continue;
}
0x10d3d408=>{
// 0x10d3d408 bl #0x10dc7390
cpu.x[30]=0x10d3d40c;super::chunk15::f_10dc7390(cpu,rt,0x10dc7390)?;
pc=0x10d3d40c;continue;
}
0x10d3d40c=>{
// 0x10d3d40c add w21, w0, w21
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[21] as u32 as u64));let result=a.wrapping_add(b);cpu.x[21]=(result) as u32 as u64;}
// 0x10d3d410 add w1, w23, #1
{let a=((cpu.x[23] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u32 as u64;}
// 0x10d3d414 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d3d418 bl #0x10d27e98
cpu.x[30]=0x10d3d41c;super::chunk6::f_10d27e98(cpu,rt,0x10d27e98)?;
pc=0x10d3d41c;continue;
}
0x10d3d41c=>{
// 0x10d3d41c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d3d420 mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10d3d424 bl #0x10d2801c
cpu.x[30]=0x10d3d428;super::chunk3::f_10d2801c(cpu,rt,0x10d2801c)?;
pc=0x10d3d428;continue;
}
0x10d3d428=>{
// 0x10d3d428 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d3d42c bl #0x10d28cdc
cpu.x[30]=0x10d3d430;super::chunk11::f_10d28cdc(cpu,rt,0x10d28cdc)?;
pc=0x10d3d430;continue;
}
0x10d3d430=>{
// 0x10d3d430 tbnz w0, #0, #0x10d3d374
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10d3d374;continue;}pc=0x10d3d434;continue;
}
0x10d3d434=>{
// 0x10d3d434 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d3d438 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d3d43c ldp x24, x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10d3d440 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10d3d444 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10d3d448=>{
// 0x10d3d448 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d3d44c mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10d3d450 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d3d454 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d3d458 ldp x24, x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10d3d45c ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10d3d460 b #0x10d2801c
super::chunk3::f_10d2801c(cpu,rt,0x10d2801c)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d3d2f0")),}}}
#[inline]
pub(super) unsafe fn f_10d43840(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d43840=>{
// 0x10d43840 ldr x8, [x0, #0x1b0]
{let address=(cpu.x[0]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d43844 ldr x0, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d43848 cbz x0, #0x10d43850
if (cpu.x[0]) == 0 {pc=0x10d43850;continue;}pc=0x10d4384c;continue;
}
0x10d4384c=>{
// 0x10d4384c b #0x10e2d5b4
super::chunk9::f_10e2d5b4(cpu,rt,0x10e2d5b4)?;return Ok(());
}
0x10d43850=>{
// 0x10d43850 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d43840")),}}}
#[inline]
pub(super) unsafe fn f_10d448b0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d448b0=>{
// 0x10d448b0 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d448b4 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d448b8 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d448bc mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10d448c0 ldr x0, [x0, #0x190]
{let address=(cpu.x[0]).wrapping_add(0x190u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d448c4 mov w19, w1
cpu.x[19]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x10d448c8 bl #0x10d56778
cpu.x[30]=0x10d448cc;super::chunk5::f_10d56778(cpu,rt,0x10d56778)?;
pc=0x10d448cc;continue;
}
0x10d448cc=>{
// 0x10d448cc ldr x8, [x20, #0x340]
{let address=(cpu.x[20]).wrapping_add(0x340u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d448d0 cbz x8, #0x10d448ec
if (cpu.x[8]) == 0 {pc=0x10d448ec;continue;}pc=0x10d448d4;continue;
}
0x10d448d4=>{
// 0x10d448d4 mov w9, #0x64
cpu.x[9]=(((0x64u64))) as u32 as u64;
// 0x10d448d8 mov w1, w19
cpu.x[1]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10d448dc mul w20, w0, w9
cpu.x[20]=((((cpu.x[0] as u32 as u64))).wrapping_mul(((cpu.x[9] as u32 as u64)))) as u32 as u64;
// 0x10d448e0 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10d448e4 bl #0x10d2ea20
cpu.x[30]=0x10d448e8;super::dispatch(cpu,rt,0x10d2ea20)?;
// 0x10d448e8 sdiv w0, w20, w0
{let a=((cpu.x[20] as u32 as u64)) as i32;let b=((cpu.x[0] as u32 as u64)) as i32;cpu.x[0]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u32 as u64;}
pc=0x10d448ec;continue;
}
0x10d448ec=>{
// 0x10d448ec ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d448f0 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10d448f4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d448b0")),}}}
#[inline]
pub(super) unsafe fn f_10d48870(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d48870=>{
// 0x10d48870 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d48874 str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d48878 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d4887c mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10d48880 ldr x0, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d48884 cbz x0, #0x10d488ac
if (cpu.x[0]) == 0 {pc=0x10d488ac;continue;}pc=0x10d48888;continue;
}
0x10d48888=>{
// 0x10d48888 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d4888c ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d48890 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d48894;super::dispatch(cpu,rt,target)?;}
pc=0x10d48894;continue;
}
0x10d48894=>{
// 0x10d48894 ldr x0, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d48898 cbz x0, #0x10d488a8
if (cpu.x[0]) == 0 {pc=0x10d488a8;continue;}pc=0x10d4889c;continue;
}
0x10d4889c=>{
// 0x10d4889c ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d488a0 ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d488a4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d488a8;super::dispatch(cpu,rt,target)?;}
pc=0x10d488a8;continue;
}
0x10d488a8=>{
// 0x10d488a8 str xzr, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10d488ac;continue;
}
0x10d488ac=>{
// 0x10d488ac ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10d488b0 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10d488b4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d48870")),}}}
#[inline]
pub(super) unsafe fn f_10d497ac(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d497ac=>{
// 0x10d497ac movi v0.2d, #0000000000000000
cpu.q[0]=(lane_dup(0u128,64,2)) & 0xffffffffffffffffffffffffffffffffu128;
// 0x10d497b0 mov w8, #0x3f800000
cpu.x[8]=(((0x3f800000u64))) as u32 as u64;
// 0x10d497b4 stp xzr, xzr, [x0, #0x50]
{let address=(cpu.x[0]).wrapping_add(0x50u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d497b8 stp xzr, xzr, [x0, #0x68]
{let address=(cpu.x[0]).wrapping_add(0x68u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d497bc str w8, [x0, #0xa0]
{let address=(cpu.x[0]).wrapping_add(0xa0u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d497c0 str xzr, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d497c4 stp q0, q0, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10d497c8 stp q0, q0, [x0, #0x30]
{let address=(cpu.x[0]).wrapping_add(0x30u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10d497cc stp q0, q0, [x0, #0x80]
{let address=(cpu.x[0]).wrapping_add(0x80u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10d497d0 str wzr, [x0, #0x74]
{let address=(cpu.x[0]).wrapping_add(0x74u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d497d4 strb wzr, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10d497d8 str x1, [x0, #0x60]
{let address=(cpu.x[0]).wrapping_add(0x60u64);let v0=cpu.x[1];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d497dc stp wzr, w2, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=0u64;let v1=cpu.x[2] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10d497e0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d497ac")),}}}
#[inline]
pub(super) unsafe fn f_10d54a24(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d54a24=>{
// 0x10d54a24 mov w5, w4
cpu.x[5]=(((cpu.x[4] as u32 as u64))) as u32 as u64;
// 0x10d54a28 mov w4, #0x1b
cpu.x[4]=(((0x1bu64))) as u32 as u64;
// 0x10d54a2c b #0x10d54530
super::chunk14::f_10d54530(cpu,rt,0x10d54530)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d54a24")),}}}
#[inline]
pub(super) unsafe fn f_10d57c18(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d57c18=>{
// 0x10d57c18 stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d57c1c str x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d57c20 stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d57c24 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d57c28 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d57c2c ldr x21, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10d57c30 ldr x22, [x1]
{let address=(cpu.x[1]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[22]=(v0) as u64;}
// 0x10d57c34 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10d57c38 mov x20, x3
cpu.x[20]=(((cpu.x[3]))) as u64;
// 0x10d57c3c cbz x21, #0x10d57c64
if (cpu.x[21]) == 0 {pc=0x10d57c64;continue;}pc=0x10d57c40;continue;
}
0x10d57c40=>{
// 0x10d57c40 fmov d0, x21
cpu.q[0]=(cpu.x[21]) as u128 & 0xffffffffffffffffu128;
// 0x10d57c44 cnt v0.8b, v0.8b
cpu.q[0]=(lane_count((cpu.q[0] & 0xffffffffffffffffu128),8,8)) & 0xffffffffffffffffu128;
// 0x10d57c48 uaddlv h0, v0.8b
cpu.q[0]=(lane_reduce((cpu.q[0] & 0xffffffffffffffffu128),8,8,"uaddlv")) & 0xffffu128;
// 0x10d57c4c fmov w8, s0
cpu.x[8]=((cpu.q[0] & 0xffffffffu128) as u64) as u32 as u64;
// 0x10d57c50 cmp x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d57c54 b.hi #0x10d57c68
if cpu.condition(8){pc=0x10d57c68;continue;}pc=0x10d57c58;continue;
}
0x10d57c58=>{
// 0x10d57c58 sub x9, x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u64;}
// 0x10d57c5c and x23, x9, x22
{let a=((cpu.x[9]));let b=((cpu.x[22]));let result=a & b;cpu.x[23]=(result) as u64;}
// 0x10d57c60 b #0x10d57c7c
pc=0x10d57c7c;continue;
}
0x10d57c64=>{
// 0x10d57c64 b #0x10d57d24
pc=0x10d57d24;continue;
}
0x10d57c68=>{
// 0x10d57c68 cmp x22, x21
{let a=((cpu.x[22]));let b=((cpu.x[21]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d57c6c mov x23, x22
cpu.x[23]=(((cpu.x[22]))) as u64;
// 0x10d57c70 b.lo #0x10d57c7c
if cpu.condition(3){pc=0x10d57c7c;continue;}pc=0x10d57c74;continue;
}
0x10d57c74=>{
// 0x10d57c74 udiv x9, x22, x21
{let a=((cpu.x[22])) as u64;let b=((cpu.x[21])) as u64;cpu.x[9]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u64;}
// 0x10d57c78 msub x23, x9, x21, x22
cpu.x[23]=((((cpu.x[22]))).wrapping_sub((((cpu.x[9]))).wrapping_mul(((cpu.x[21]))))) as u64;
pc=0x10d57c7c;continue;
}
0x10d57c7c=>{
// 0x10d57c7c ldr x9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d57c80 ldr x9, [x9, x23, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[23])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d57c84 cbz x9, #0x10d57d24
if (cpu.x[9]) == 0 {pc=0x10d57d24;continue;}pc=0x10d57c88;continue;
}
0x10d57c88=>{
// 0x10d57c88 ldr x0, [x9]
{let address=(cpu.x[9]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d57c8c cbz x0, #0x10d57d24
if (cpu.x[0]) == 0 {pc=0x10d57d24;continue;}pc=0x10d57c90;continue;
}
0x10d57c90=>{
// 0x10d57c90 cmp x8, #2
{let a=((cpu.x[8]));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d57c94 b.hs #0x10d57cb0
if cpu.condition(2){pc=0x10d57cb0;continue;}pc=0x10d57c98;continue;
}
0x10d57c98=>{
// 0x10d57c98 sub x8, x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u64;}
// 0x10d57c9c b #0x10d57cf4
pc=0x10d57cf4;continue;
}
0x10d57ca0=>{
// 0x10d57ca0 cmp x8, x23
{let a=((cpu.x[8]));let b=((cpu.x[23]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d57ca4 b.ne #0x10d57d24
if cpu.condition(1){pc=0x10d57d24;continue;}pc=0x10d57ca8;continue;
}
0x10d57ca8=>{
// 0x10d57ca8 ldr x0, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d57cac cbz x0, #0x10d57d24
if (cpu.x[0]) == 0 {pc=0x10d57d24;continue;}pc=0x10d57cb0;continue;
}
0x10d57cb0=>{
// 0x10d57cb0 ldr x8, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d57cb4 cmp x8, x22
{let a=((cpu.x[8]));let b=((cpu.x[22]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d57cb8 b.ne #0x10d57ccc
if cpu.condition(1){pc=0x10d57ccc;continue;}pc=0x10d57cbc;continue;
}
0x10d57cbc=>{
// 0x10d57cbc ldr x8, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d57cc0 cmp x8, x22
{let a=((cpu.x[8]));let b=((cpu.x[22]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d57cc4 b.ne #0x10d57ca8
if cpu.condition(1){pc=0x10d57ca8;continue;}pc=0x10d57cc8;continue;
}
0x10d57cc8=>{
// 0x10d57cc8 b #0x10d57d0c
pc=0x10d57d0c;continue;
}
0x10d57ccc=>{
// 0x10d57ccc cmp x8, x21
{let a=((cpu.x[8]));let b=((cpu.x[21]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d57cd0 b.lo #0x10d57ca0
if cpu.condition(3){pc=0x10d57ca0;continue;}pc=0x10d57cd4;continue;
}
0x10d57cd4=>{
// 0x10d57cd4 udiv x9, x8, x21
{let a=((cpu.x[8])) as u64;let b=((cpu.x[21])) as u64;cpu.x[9]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u64;}
// 0x10d57cd8 msub x8, x9, x21, x8
cpu.x[8]=((((cpu.x[8]))).wrapping_sub((((cpu.x[9]))).wrapping_mul(((cpu.x[21]))))) as u64;
// 0x10d57cdc b #0x10d57ca0
pc=0x10d57ca0;continue;
}
0x10d57ce0=>{
// 0x10d57ce0 and x9, x9, x8
{let a=((cpu.x[9]));let b=((cpu.x[8]));let result=a & b;cpu.x[9]=(result) as u64;}
// 0x10d57ce4 cmp x9, x23
{let a=((cpu.x[9]));let b=((cpu.x[23]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d57ce8 b.ne #0x10d57d24
if cpu.condition(1){pc=0x10d57d24;continue;}pc=0x10d57cec;continue;
}
0x10d57cec=>{
// 0x10d57cec ldr x0, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d57cf0 cbz x0, #0x10d57d24
if (cpu.x[0]) == 0 {pc=0x10d57d24;continue;}pc=0x10d57cf4;continue;
}
0x10d57cf4=>{
// 0x10d57cf4 ldr x9, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d57cf8 cmp x9, x22
{let a=((cpu.x[9]));let b=((cpu.x[22]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d57cfc b.ne #0x10d57ce0
if cpu.condition(1){pc=0x10d57ce0;continue;}pc=0x10d57d00;continue;
}
0x10d57d00=>{
// 0x10d57d00 ldr x9, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d57d04 cmp x9, x22
{let a=((cpu.x[9]));let b=((cpu.x[22]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d57d08 b.ne #0x10d57cec
if cpu.condition(1){pc=0x10d57cec;continue;}pc=0x10d57d0c;continue;
}
0x10d57d0c=>{
// 0x10d57d0c mov x1, xzr
cpu.x[1]=(((0u64))) as u64;
// 0x10d57d10 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d57d14 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10d57d18 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d57d1c ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10d57d20 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10d57d24=>{
// 0x10d57d24 mov w0, #0x20
cpu.x[0]=(((0x20u64))) as u32 as u64;
// 0x10d57d28 bl #0x1187d2b0
cpu.x[30]=0x10d57d2c;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
pc=0x10d57d2c;continue;
}
0x10d57d2c=>{
// 0x10d57d2c ldr x8, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d57d30 ldr x9, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d57d34 ldr s1, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.q[1]=(v0) as u128 & 0xffffffffu128;}
// 0x10d57d38 stp xzr, x22, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=0u64;let v1=cpu.x[22];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d57d3c add x8, x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d57d40 ldr x9, [x9]
{let address=(cpu.x[9]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d57d44 str wzr, [x0, #0x18]
{let address=(cpu.x[0]).wrapping_add(0x18u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d57d48 ucvtf s0, x8
cpu.q[0]=(((cpu.x[8]) as u64 as f32).to_bits()) as u128 & 0xffffffffu128;
// 0x10d57d4c str x9, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=cpu.x[9];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d57d50 cbz x21, #0x10d57d7c
if (cpu.x[21]) == 0 {pc=0x10d57d7c;continue;}pc=0x10d57d54;continue;
}
0x10d57d54=>{
// 0x10d57d54 ucvtf s2, x21
cpu.q[2]=(((cpu.x[21]) as u64 as f32).to_bits()) as u128 & 0xffffffffu128;
// 0x10d57d58 fmul s2, s1, s2
fp_domain(cpu,&[f32::from_bits(((cpu.q[1] & 0xffffffffu128) as u64) as u32) as f64,f32::from_bits(((cpu.q[2] & 0xffffffffu128) as u64) as u32) as f64])?;if f32::from_bits(((cpu.q[2] & 0xffffffffu128) as u64) as u32).is_subnormal(){return Err("subnormal f32 operand requires architectural handling".into());}if f32::from_bits(((cpu.q[1] & 0xffffffffu128) as u64) as u32).is_subnormal(){return Err("subnormal f32 operand requires architectural handling".into());}cpu.q[2]=((f32::from_bits(((cpu.q[1] & 0xffffffffu128) as u64) as u32)*f32::from_bits(((cpu.q[2] & 0xffffffffu128) as u64) as u32)).to_bits()) as u128 & 0xffffffffu128;fp_domain(cpu,&[f32::from_bits(((cpu.q[2] & 0xffffffffu128) as u64) as u32) as f64])?;
// 0x10d57d5c fcmp s2, s0
fp_domain(cpu,&[f32::from_bits(((cpu.q[2] & 0xffffffffu128) as u64) as u32) as f64,f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32) as f64])?;if f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32).is_subnormal(){return Err("subnormal f32 operand requires architectural handling".into());}if f32::from_bits(((cpu.q[2] & 0xffffffffu128) as u64) as u32).is_subnormal(){return Err("subnormal f32 operand requires architectural handling".into());}{let a=f32::from_bits(((cpu.q[2] & 0xffffffffu128) as u64) as u32);let b=f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32);cpu.nzcv=if a.is_nan()||b.is_nan(){0x30000000}else if a==b{0x60000000}else if a<b{0x80000000}else{0x20000000};}
// 0x10d57d60 b.mi #0x10d57d7c
if cpu.condition(4){pc=0x10d57d7c;continue;}pc=0x10d57d64;continue;
}
0x10d57d64=>{
// 0x10d57d64 ldr x9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d57d68 ldr x8, [x9, x23, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[23])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d57d6c cbz x8, #0x10d57eb0
if (cpu.x[8]) == 0 {pc=0x10d57eb0;continue;}pc=0x10d57d70;continue;
}
0x10d57d70=>{
// 0x10d57d70 ldr x9, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d57d74 str x9, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=cpu.x[9];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d57d78 b #0x10d57f30
pc=0x10d57f30;continue;
}
0x10d57d7c=>{
// 0x10d57d7c lsl x8, x21, #1
cpu.x[8]=(((((cpu.x[21]))) as u64).wrapping_shl(((((0x1u64))) & 63) as u32) as u64) as u64;
// 0x10d57d80 cmp x21, #3
{let a=((cpu.x[21]));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d57d84 b.lo #0x10d57d98
if cpu.condition(3){pc=0x10d57d98;continue;}pc=0x10d57d88;continue;
}
0x10d57d88=>{
// 0x10d57d88 sub x9, x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u64;}
// 0x10d57d8c tst x21, x9
{let a=((cpu.x[21]));let b=((cpu.x[9]));let result=a & b;cpu.logical_flags(result,64);}
// 0x10d57d90 cset w9, ne
cpu.x[9]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x10d57d94 b #0x10d57d9c
pc=0x10d57d9c;continue;
}
0x10d57d98=>{
// 0x10d57d98 mov w9, #1
cpu.x[9]=(((0x1u64))) as u32 as u64;
pc=0x10d57d9c;continue;
}
0x10d57d9c=>{
// 0x10d57d9c fdiv s0, s0, s1
fp_domain(cpu,&[f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32) as f64,f32::from_bits(((cpu.q[1] & 0xffffffffu128) as u64) as u32) as f64])?;if f32::from_bits(((cpu.q[1] & 0xffffffffu128) as u64) as u32).is_subnormal(){return Err("subnormal f32 operand requires architectural handling".into());}if f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32).is_subnormal(){return Err("subnormal f32 operand requires architectural handling".into());}cpu.q[0]=((f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32)/f32::from_bits(((cpu.q[1] & 0xffffffffu128) as u64) as u32)).to_bits()) as u128 & 0xffffffffu128;fp_domain(cpu,&[f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32) as f64])?;
// 0x10d57da0 orr x8, x9, x8
{let a=((cpu.x[9]));let b=((cpu.x[8]));let result=a | b;cpu.x[8]=(result) as u64;}
// 0x10d57da4 fcvtpu x10, s0
fp_domain(cpu,&[f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32) as f64])?;if f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32).is_subnormal(){return Err("subnormal f32 operand requires architectural handling".into());}cpu.x[10]=((f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32).ceil() as u64) as u64) as u64;
// 0x10d57da8 cmp x8, x10
{let a=((cpu.x[8]));let b=((cpu.x[10]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d57dac csel x1, x8, x10, hi
cpu.x[1]=(if cpu.condition(8){((cpu.x[8]))}else{((cpu.x[10]))}) as u64;
// 0x10d57db0 subs x8, x1, #1
{let a=((cpu.x[1]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x10d57db4 b.ne #0x10d57dc0
if cpu.condition(1){pc=0x10d57dc0;continue;}pc=0x10d57db8;continue;
}
0x10d57db8=>{
// 0x10d57db8 mov w1, #2
cpu.x[1]=(((0x2u64))) as u32 as u64;
// 0x10d57dbc b #0x10d57de0
pc=0x10d57de0;continue;
}
0x10d57dc0=>{
// 0x10d57dc0 tst x1, x8
{let a=((cpu.x[1]));let b=((cpu.x[8]));let result=a & b;cpu.logical_flags(result,64);}
// 0x10d57dc4 b.eq #0x10d57de0
if cpu.condition(0){pc=0x10d57de0;continue;}pc=0x10d57dc8;continue;
}
0x10d57dc8=>{
// 0x10d57dc8 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10d57dcc mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x10d57dd0 bl #0x1187d3f0
cpu.x[30]=0x10d57dd4;super::dispatch(cpu,rt,0x1187d3f0)?;
pc=0x10d57dd4;continue;
}
0x10d57dd4=>{
// 0x10d57dd4 ldr x21, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10d57dd8 mov x1, x0
cpu.x[1]=(((cpu.x[0]))) as u64;
// 0x10d57ddc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
pc=0x10d57de0;continue;
}
0x10d57de0=>{
// 0x10d57de0 cmp x1, x21
{let a=((cpu.x[1]));let b=((cpu.x[21]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d57de4 b.ls #0x10d57dfc
if cpu.condition(9){pc=0x10d57dfc;continue;}pc=0x10d57de8;continue;
}
0x10d57de8=>{
// 0x10d57de8 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10d57dec mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d57df0 bl #0x10d57f58
cpu.x[30]=0x10d57df4;super::chunk10::f_10d57f58(cpu,rt,0x10d57f58)?;
pc=0x10d57df4;continue;
}
0x10d57df4=>{
// 0x10d57df4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d57df8 b #0x10d57e90
pc=0x10d57e90;continue;
}
0x10d57dfc=>{
// 0x10d57dfc b.hs #0x10d57e90
if cpu.condition(2){pc=0x10d57e90;continue;}pc=0x10d57e00;continue;
}
0x10d57e00=>{
// 0x10d57e00 ldr x8, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d57e04 ldr s1, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.q[1]=(v0) as u128 & 0xffffffffu128;}
// 0x10d57e08 cmp x21, #3
{let a=((cpu.x[21]));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d57e0c ucvtf s0, x8
cpu.q[0]=(((cpu.x[8]) as u64 as f32).to_bits()) as u128 & 0xffffffffu128;
// 0x10d57e10 fdiv s0, s0, s1
fp_domain(cpu,&[f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32) as f64,f32::from_bits(((cpu.q[1] & 0xffffffffu128) as u64) as u32) as f64])?;if f32::from_bits(((cpu.q[1] & 0xffffffffu128) as u64) as u32).is_subnormal(){return Err("subnormal f32 operand requires architectural handling".into());}if f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32).is_subnormal(){return Err("subnormal f32 operand requires architectural handling".into());}cpu.q[0]=((f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32)/f32::from_bits(((cpu.q[1] & 0xffffffffu128) as u64) as u32)).to_bits()) as u128 & 0xffffffffu128;fp_domain(cpu,&[f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32) as f64])?;
// 0x10d57e14 fcvtpu x8, s0
fp_domain(cpu,&[f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32) as f64])?;if f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32).is_subnormal(){return Err("subnormal f32 operand requires architectural handling".into());}cpu.x[8]=((f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32).ceil() as u64) as u64) as u64;
// 0x10d57e18 b.lo #0x10d57e64
if cpu.condition(3){pc=0x10d57e64;continue;}pc=0x10d57e1c;continue;
}
0x10d57e1c=>{
// 0x10d57e1c fmov d0, x21
cpu.q[0]=(cpu.x[21]) as u128 & 0xffffffffffffffffu128;
// 0x10d57e20 cnt v0.8b, v0.8b
cpu.q[0]=(lane_count((cpu.q[0] & 0xffffffffffffffffu128),8,8)) & 0xffffffffffffffffu128;
// 0x10d57e24 uaddlv h0, v0.8b
cpu.q[0]=(lane_reduce((cpu.q[0] & 0xffffffffffffffffu128),8,8,"uaddlv")) & 0xffffu128;
// 0x10d57e28 fmov w9, s0
cpu.x[9]=((cpu.q[0] & 0xffffffffu128) as u64) as u32 as u64;
// 0x10d57e2c cmp x9, #1
{let a=((cpu.x[9]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d57e30 b.hi #0x10d57e64
if cpu.condition(8){pc=0x10d57e64;continue;}pc=0x10d57e34;continue;
}
0x10d57e34=>{
// 0x10d57e34 sub x9, x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u64;}
// 0x10d57e38 mov w10, #1
cpu.x[10]=(((0x1u64))) as u32 as u64;
// 0x10d57e3c cmp x8, #2
{let a=((cpu.x[8]));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d57e40 clz x9, x9
cpu.x[9]=((((cpu.x[9])) as u64).leading_zeros() as u64) as u64;
// 0x10d57e44 neg x9, x9
cpu.x[9]=(0u64.wrapping_sub(((cpu.x[9])))) as u64;
// 0x10d57e48 lsl x9, x10, x9
cpu.x[9]=(((((cpu.x[10]))) as u64).wrapping_shl(((((cpu.x[9]))) & 63) as u32) as u64) as u64;
// 0x10d57e4c csel x8, x8, x9, lo
cpu.x[8]=(if cpu.condition(3){((cpu.x[8]))}else{((cpu.x[9]))}) as u64;
// 0x10d57e50 cmp x1, x8
{let a=((cpu.x[1]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d57e54 csel x1, x1, x8, hi
cpu.x[1]=(if cpu.condition(8){((cpu.x[1]))}else{((cpu.x[8]))}) as u64;
// 0x10d57e58 cmp x1, x21
{let a=((cpu.x[1]));let b=((cpu.x[21]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d57e5c b.lo #0x10d57de8
if cpu.condition(3){pc=0x10d57de8;continue;}pc=0x10d57e60;continue;
}
0x10d57e60=>{
// 0x10d57e60 b #0x10d57e90
pc=0x10d57e90;continue;
}
0x10d57e64=>{
// 0x10d57e64 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10d57e68 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10d57e6c mov x23, x1
cpu.x[23]=(((cpu.x[1]))) as u64;
// 0x10d57e70 bl #0x1187d3f0
cpu.x[30]=0x10d57e74;super::dispatch(cpu,rt,0x1187d3f0)?;
// 0x10d57e74 mov x1, x23
cpu.x[1]=(((cpu.x[23]))) as u64;
// 0x10d57e78 mov x8, x0
cpu.x[8]=(((cpu.x[0]))) as u64;
// 0x10d57e7c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d57e80 cmp x23, x8
{let a=((cpu.x[23]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d57e84 csel x1, x23, x8, hi
cpu.x[1]=(if cpu.condition(8){((cpu.x[23]))}else{((cpu.x[8]))}) as u64;
// 0x10d57e88 cmp x1, x21
{let a=((cpu.x[1]));let b=((cpu.x[21]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d57e8c b.lo #0x10d57de8
if cpu.condition(3){pc=0x10d57de8;continue;}pc=0x10d57e90;continue;
}
0x10d57e90=>{
// 0x10d57e90 ldr x21, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10d57e94 sub x8, x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u64;}
// 0x10d57e98 tst x21, x8
{let a=((cpu.x[21]));let b=((cpu.x[8]));let result=a & b;cpu.logical_flags(result,64);}
// 0x10d57e9c b.ne #0x10d57ee4
if cpu.condition(1){pc=0x10d57ee4;continue;}pc=0x10d57ea0;continue;
}
0x10d57ea0=>{
// 0x10d57ea0 and x23, x8, x22
{let a=((cpu.x[8]));let b=((cpu.x[22]));let result=a & b;cpu.x[23]=(result) as u64;}
// 0x10d57ea4 ldr x9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d57ea8 ldr x8, [x9, x23, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[23])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d57eac cbnz x8, #0x10d57d70
if (cpu.x[8]) != 0 {pc=0x10d57d70;continue;}pc=0x10d57eb0;continue;
}
0x10d57eb0=>{
// 0x10d57eb0 mov x8, x19
cpu.x[8]=(((cpu.x[19]))) as u64;
// 0x10d57eb4 ldr x10, [x8, #0x10]!
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;cpu.x[8]=(address) as u64;}
// 0x10d57eb8 str x0, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d57ebc str x10, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=cpu.x[10];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d57ec0 str x8, [x9, x23, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[23])) as u64).wrapping_shl(3) as u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d57ec4 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d57ec8 cbz x8, #0x10d57f34
if (cpu.x[8]) == 0 {pc=0x10d57f34;continue;}pc=0x10d57ecc;continue;
}
0x10d57ecc=>{
// 0x10d57ecc sub x9, x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u64;}
// 0x10d57ed0 ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d57ed4 tst x21, x9
{let a=((cpu.x[21]));let b=((cpu.x[9]));let result=a & b;cpu.logical_flags(result,64);}
// 0x10d57ed8 b.ne #0x10d57f18
if cpu.condition(1){pc=0x10d57f18;continue;}pc=0x10d57edc;continue;
}
0x10d57edc=>{
// 0x10d57edc and x8, x8, x9
{let a=((cpu.x[8]));let b=((cpu.x[9]));let result=a & b;cpu.x[8]=(result) as u64;}
// 0x10d57ee0 b #0x10d57f28
pc=0x10d57f28;continue;
}
0x10d57ee4=>{
// 0x10d57ee4 cmp x22, x21
{let a=((cpu.x[22]));let b=((cpu.x[21]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d57ee8 b.hs #0x10d57f00
if cpu.condition(2){pc=0x10d57f00;continue;}pc=0x10d57eec;continue;
}
0x10d57eec=>{
// 0x10d57eec mov x23, x22
cpu.x[23]=(((cpu.x[22]))) as u64;
// 0x10d57ef0 ldr x9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d57ef4 ldr x8, [x9, x22, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[22])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d57ef8 cbnz x8, #0x10d57d70
if (cpu.x[8]) != 0 {pc=0x10d57d70;continue;}pc=0x10d57efc;continue;
}
0x10d57efc=>{
// 0x10d57efc b #0x10d57eb0
pc=0x10d57eb0;continue;
}
0x10d57f00=>{
// 0x10d57f00 udiv x8, x22, x21
{let a=((cpu.x[22])) as u64;let b=((cpu.x[21])) as u64;cpu.x[8]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u64;}
// 0x10d57f04 msub x23, x8, x21, x22
cpu.x[23]=((((cpu.x[22]))).wrapping_sub((((cpu.x[8]))).wrapping_mul(((cpu.x[21]))))) as u64;
// 0x10d57f08 ldr x9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d57f0c ldr x8, [x9, x23, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[23])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d57f10 cbnz x8, #0x10d57d70
if (cpu.x[8]) != 0 {pc=0x10d57d70;continue;}pc=0x10d57f14;continue;
}
0x10d57f14=>{
// 0x10d57f14 b #0x10d57eb0
pc=0x10d57eb0;continue;
}
0x10d57f18=>{
// 0x10d57f18 cmp x8, x21
{let a=((cpu.x[8]));let b=((cpu.x[21]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d57f1c b.lo #0x10d57f28
if cpu.condition(3){pc=0x10d57f28;continue;}pc=0x10d57f20;continue;
}
0x10d57f20=>{
// 0x10d57f20 udiv x9, x8, x21
{let a=((cpu.x[8])) as u64;let b=((cpu.x[21])) as u64;cpu.x[9]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u64;}
// 0x10d57f24 msub x8, x9, x21, x8
cpu.x[8]=((((cpu.x[8]))).wrapping_sub((((cpu.x[9]))).wrapping_mul(((cpu.x[21]))))) as u64;
pc=0x10d57f28;continue;
}
0x10d57f28=>{
// 0x10d57f28 ldr x9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d57f2c add x8, x9, x8, lsl #3
{let a=((cpu.x[9]));let b=(((cpu.x[8])) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
pc=0x10d57f30;continue;
}
0x10d57f30=>{
// 0x10d57f30 str x0, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10d57f34;continue;
}
0x10d57f34=>{
// 0x10d57f34 ldr x8, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d57f38 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d57f3c add x8, x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d57f40 str x8, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d57f44 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d57f48 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10d57f4c ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d57f50 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10d57f54 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d57c18")),}}}
#[inline]
pub(super) unsafe fn f_10d5d07c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d5d07c=>{
// 0x10d5d07c mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10d5d080 adrp x1, #0x10338000
cpu.x[1]=(0x10338000u64) as u64;
// 0x10d5d084 add x1, x1, #0xb90
{let a=((cpu.x[1]));let b=((0xb90u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x10d5d088 b #0x1142fde4
super::chunk6::f_1142fde4(cpu,rt,0x1142fde4)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d5d07c")),}}}
#[inline]
pub(super) unsafe fn f_10d611ac(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d611ac=>{
// 0x10d611ac mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10d611b0 adrp x1, #0x1030e000
cpu.x[1]=(0x1030e000u64) as u64;
// 0x10d611b4 add x1, x1, #0x2cd
{let a=((cpu.x[1]));let b=((0x2cdu64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x10d611b8 b #0x1142fde4
super::chunk6::f_1142fde4(cpu,rt,0x1142fde4)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d611ac")),}}}
#[inline]
pub(super) unsafe fn f_10d75bc0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d75bc0=>{
// 0x10d75bc0 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d75bc4 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d75bc8 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d75bcc ldr x8, [x1]
{let address=(cpu.x[1]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d75bd0 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10d75bd4 mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x10d75bd8 mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10d75bdc ldr x8, [x8, #0x20]
{let address=(cpu.x[8]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d75be0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d75be4;super::dispatch(cpu,rt,target)?;}
pc=0x10d75be4;continue;
}
0x10d75be4=>{
// 0x10d75be4 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d75be8 mov w1, w0
cpu.x[1]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d75bec mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d75bf0 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d75bf4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d75bf8;super::dispatch(cpu,rt,target)?;}
pc=0x10d75bf8;continue;
}
0x10d75bf8=>{
// 0x10d75bf8 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d75bfc mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
pc=0x10d75c00;continue;
}
0x10d75c00=>{
// 0x10d75c00 ldr x2, [x8, #0x28]
{let address=(cpu.x[8]).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10d75c04 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d75c08 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d75c0c ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10d75c10 br x2
{let target=cpu.x[2];if target>=0x10d75bc0 && target<0x10d75c14{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d75bc0")),}}}
#[inline]
pub(super) unsafe fn f_10d77a14(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d77a14=>{
// 0x10d77a14 stp w1, w2, [x0, #0x28]
{let address=(cpu.x[0]).wrapping_add(0x28u64);let v0=cpu.x[1] as u32 as u64;let v1=cpu.x[2] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10d77a18 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d77a14")),}}}
#[inline]
pub(super) unsafe fn f_10dc7010(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10dc7010=>{
// 0x10dc7010 ldrb w0, [x0, #0xa0]
{let address=(cpu.x[0]).wrapping_add(0xa0u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10dc7014 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10dc7010")),}}}
#[inline]
pub(super) unsafe fn f_10dc7390(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10dc7390=>{
// 0x10dc7390 ldr w0, [x0, #0x94]
{let address=(cpu.x[0]).wrapping_add(0x94u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10dc7394 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10dc7390")),}}}
#[inline]
pub(super) unsafe fn f_10ddae14(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10ddae14=>{
// 0x10ddae14 ldr w8, [x0, #0x110]
{let address=(cpu.x[0]).wrapping_add(0x110u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10ddae18 ldr w9, [x0, #0x114]
{let address=(cpu.x[0]).wrapping_add(0x114u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10ddae1c orr w8, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10ddae20 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10ddae24 cset w0, ne
cpu.x[0]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x10ddae28 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10ddae14")),}}}
#[inline]
pub(super) unsafe fn f_10de02c4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10de02c4=>{
// 0x10de02c4 ldrb w0, [x0, #0x571]
{let address=(cpu.x[0]).wrapping_add(0x571u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10de02c8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10de02c4")),}}}
#[inline]
pub(super) unsafe fn f_10de0450(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10de0450=>{
// 0x10de0450 ldp x9, x8, [x0, #0x128]
{let address=(cpu.x[0]).wrapping_add(0x128u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[9]=(v0) as u64;cpu.x[8]=(v1) as u64;}
// 0x10de0454 cmp x8, #0
{let a=((cpu.x[8]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10de0458 cset w10, ne
cpu.x[10]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x10de045c tst w1, w10
{let a=((cpu.x[1] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a & b;cpu.logical_flags(result,32);}
// 0x10de0460 csel x0, x8, x9, ne
cpu.x[0]=(if cpu.condition(1){((cpu.x[8]))}else{((cpu.x[9]))}) as u64;
// 0x10de0464 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10de0450")),}}}
#[inline]
pub(super) unsafe fn f_10de048c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10de048c=>{
// 0x10de048c ldr x0, [x0, #0x1e8]
{let address=(cpu.x[0]).wrapping_add(0x1e8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10de0490 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10de048c")),}}}
#[inline]
pub(super) unsafe fn f_10de0708(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10de0708=>{
// 0x10de0708 ldr w0, [x0, #0x3a8]
{let address=(cpu.x[0]).wrapping_add(0x3a8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10de070c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10de0708")),}}}
#[inline]
pub(super) unsafe fn f_10de0710(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10de0710=>{
// 0x10de0710 ldrb w0, [x0, #0x3a5]
{let address=(cpu.x[0]).wrapping_add(0x3a5u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10de0714 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10de0710")),}}}
#[inline]
pub(super) unsafe fn f_10de07ec(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10de07ec=>{
// 0x10de07ec mov x8, x0
cpu.x[8]=(((cpu.x[0]))) as u64;
// 0x10de07f0 ldr w0, [x0, #0x4e8]
{let address=(cpu.x[0]).wrapping_add(0x4e8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10de07f4 cmp w0, #0
{let a=((cpu.x[0] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10de07f8 b.le #0x10de0800
if cpu.condition(13){pc=0x10de0800;continue;}pc=0x10de07fc;continue;
}
0x10de07fc=>{
// 0x10de07fc ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10de0800=>{
// 0x10de0800 ldr w8, [x8, #0x4e4]
{let address=(cpu.x[8]).wrapping_add(0x4e4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10de0804 mov w9, #0x4dd3
cpu.x[9]=(((0x4dd3u64))) as u32 as u64;
// 0x10de0808 movk w9, #0x1062, lsl #16
cpu.x[9]=((cpu.x[9] as u32 as u64 & !(0xffff0000u64)) | 0x10620000u64) as u32 as u64;
// 0x10de080c add w8, w8, #0x1f3
{let a=((cpu.x[8] as u32 as u64));let b=((0x1f3u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10de0810 smull x8, w8, w9
cpu.x[8]=(((((cpu.x[8] as u32 as u64))) as i32 as i64).wrapping_mul((((cpu.x[9] as u32 as u64))) as i32 as i64) as u64) as u64;
// 0x10de0814 lsr x9, x8, #0x3f
cpu.x[9]=(((((cpu.x[8]))) as u64).wrapping_shr(((((0x3fu64))) & 63) as u32) as u64) as u64;
// 0x10de0818 asr x8, x8, #0x25
cpu.x[8]=(((((cpu.x[8]))) as i64).wrapping_shr(((((0x25u64))) & 63) as u32) as u64) as u64;
// 0x10de081c add w8, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10de0820 add w0, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u32 as u64;}
// 0x10de0824 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10de07ec")),}}}
#[inline]
pub(super) unsafe fn f_10de08d0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10de08d0=>{
// 0x10de08d0 ldr w0, [x0, #0x594]
{let address=(cpu.x[0]).wrapping_add(0x594u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10de08d4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10de08d0")),}}}
#[inline]
pub(super) unsafe fn f_10de1230(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10de1230=>{
// 0x10de1230 ldr w0, [x0, #0x4f0]
{let address=(cpu.x[0]).wrapping_add(0x4f0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10de1234 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10de1230")),}}}
#[inline]
pub(super) unsafe fn f_10de1290(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10de1290=>{
// 0x10de1290 ldr w0, [x0, #0x5a0]
{let address=(cpu.x[0]).wrapping_add(0x5a0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10de1294 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10de1290")),}}}
#[inline]
pub(super) unsafe fn f_10de13d0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10de13d0=>{
// 0x10de13d0 ldr w8, [x0, #0x53c]
{let address=(cpu.x[0]).wrapping_add(0x53cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10de13d4 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10de13d8 cset w0, ne
cpu.x[0]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x10de13dc ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10de13d0")),}}}
#[inline]
pub(super) unsafe fn f_10e032c8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e032c8=>{
// 0x10e032c8 ldr w0, [x0, #0xc4]
{let address=(cpu.x[0]).wrapping_add(0xc4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10e032cc ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e032c8")),}}}
#[inline]
pub(super) unsafe fn f_10e167a4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e167a4=>{
// 0x10e167a4 ldr x0, [x0, #0x710]
{let address=(cpu.x[0]).wrapping_add(0x710u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10e167a8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e167a4")),}}}
#[inline]
pub(super) unsafe fn f_10e2d928(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e2d928=>{
// 0x10e2d928 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10e2d92c str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10e2d930 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10e2d934 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10e2d938 ldrsw x8, [x0, #0x16c]
{let address=(cpu.x[0]).wrapping_add(0x16cu64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10e2d93c mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10e2d940 cbz w8, #0x10e2d970
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10e2d970;continue;}pc=0x10e2d944;continue;
}
0x10e2d944=>{
// 0x10e2d944 ldr x20, [x19, #0x160]
{let address=(cpu.x[19]).wrapping_add(0x160u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10e2d948 lsl x21, x8, #3
cpu.x[21]=(((((cpu.x[8]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
pc=0x10e2d94c;continue;
}
0x10e2d94c=>{
// 0x10e2d94c ldr x0, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10e2d950 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e2d954 ldr x8, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e2d958 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10e2d95c;super::dispatch(cpu,rt,target)?;}
pc=0x10e2d95c;continue;
}
0x10e2d95c=>{
// 0x10e2d95c cmp w0, #7
{let a=((cpu.x[0] as u32 as u64));let b=((0x7u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e2d960 b.eq #0x10e2d9b0
if cpu.condition(0){pc=0x10e2d9b0;continue;}pc=0x10e2d964;continue;
}
0x10e2d964=>{
// 0x10e2d964 subs x21, x21, #8
{let a=((cpu.x[21]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.x[21]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x10e2d968 add x20, x20, #8
{let a=((cpu.x[20]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[20]=(result) as u64;}
// 0x10e2d96c b.ne #0x10e2d94c
if cpu.condition(1){pc=0x10e2d94c;continue;}pc=0x10e2d970;continue;
}
0x10e2d970=>{
// 0x10e2d970 ldrsw x8, [x19, #0x15c]
{let address=(cpu.x[19]).wrapping_add(0x15cu64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10e2d974 cbz w8, #0x10e2d9cc
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10e2d9cc;continue;}pc=0x10e2d978;continue;
}
0x10e2d978=>{
// 0x10e2d978 lsl x9, x8, #3
cpu.x[9]=(((((cpu.x[8]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10e2d97c ldr x8, [x19, #0x150]
{let address=(cpu.x[19]).wrapping_add(0x150u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e2d980 sub x9, x9, #8
{let a=((cpu.x[9]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u64;}
pc=0x10e2d984;continue;
}
0x10e2d984=>{
// 0x10e2d984 ldr x10, [x8], #8
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;cpu.x[8]=((cpu.x[8]).wrapping_add(((0x8u64)))) as u64;}
// 0x10e2d988 ldr w10, [x10, #0x70]
{let address=(cpu.x[10]).wrapping_add(0x70u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10e2d98c cmp w10, #7
{let a=((cpu.x[10] as u32 as u64));let b=((0x7u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e2d990 cset w0, eq
cpu.x[0]=((cpu.condition(0)) as u64) as u32 as u64;
// 0x10e2d994 ccmp x9, #0, #4, ne
if cpu.condition(1){{let a=((cpu.x[9]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}}else{cpu.nzcv=4u64 << 28;}
// 0x10e2d998 sub x9, x9, #8
{let a=((cpu.x[9]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u64;}
// 0x10e2d99c b.ne #0x10e2d984
if cpu.condition(1){pc=0x10e2d984;continue;}pc=0x10e2d9a0;continue;
}
0x10e2d9a0=>{
// 0x10e2d9a0 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10e2d9a4 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10e2d9a8 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10e2d9ac ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10e2d9b0=>{
// 0x10e2d9b0 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e2d9b4 cmp x8, #0
{let a=((cpu.x[8]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10e2d9b8 cset w0, ne
cpu.x[0]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x10e2d9bc ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10e2d9c0 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10e2d9c4 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10e2d9c8 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10e2d9cc=>{
// 0x10e2d9cc mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10e2d9d0 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10e2d9d4 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10e2d9d8 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10e2d9dc ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e2d928")),}}}
#[inline]
pub(super) unsafe fn f_10e7e7d8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e7e7d8=>{
// 0x10e7e7d8 ldp x8, x9, [x0, #0xa8]
{let address=(cpu.x[0]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[8]=(v0) as u64;cpu.x[9]=(v1) as u64;}
// 0x10e7e7dc ldr w10, [x0, #0x150]
{let address=(cpu.x[0]).wrapping_add(0x150u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10e7e7e0 ldr w11, [x0, #0x164]
{let address=(cpu.x[0]).wrapping_add(0x164u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
// 0x10e7e7e4 cmp x8, #0
{let a=((cpu.x[8]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10e7e7e8 csel w10, wzr, w10, eq
cpu.x[10]=(if cpu.condition(0){((0u64))}else{((cpu.x[10] as u32 as u64))}) as u32 as u64;
// 0x10e7e7ec cmp x9, #0
{let a=((cpu.x[9]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10e7e7f0 csel w11, wzr, w11, eq
cpu.x[11]=(if cpu.condition(0){((0u64))}else{((cpu.x[11] as u32 as u64))}) as u32 as u64;
// 0x10e7e7f4 add w11, w11, w10
{let a=((cpu.x[11] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u32 as u64;}
// 0x10e7e7f8 subs w11, w1, w11
{let a=((cpu.x[1] as u32 as u64));let b=((cpu.x[11] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[11]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10e7e7fc b.lt #0x10e7e818
if cpu.condition(11){pc=0x10e7e818;continue;}pc=0x10e7e800;continue;
}
0x10e7e800=>{
// 0x10e7e800 ldr w12, [x0, #0x2f4]
{let address=(cpu.x[0]).wrapping_add(0x2f4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[12]=(v0) as u32 as u64;}
// 0x10e7e804 cmp w12, w11
{let a=((cpu.x[12] as u32 as u64));let b=((cpu.x[11] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e7e808 b.le #0x10e7e818
if cpu.condition(13){pc=0x10e7e818;continue;}pc=0x10e7e80c;continue;
}
0x10e7e80c=>{
// 0x10e7e80c ldr x8, [x0, #0x2e8]
{let address=(cpu.x[0]).wrapping_add(0x2e8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e7e810 ldr x0, [x8, w11, sxtw #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[11] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10e7e814 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10e7e818=>{
// 0x10e7e818 cmp w10, w1
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e7e81c ccmp x9, #0, #4, le
if cpu.condition(13){{let a=((cpu.x[9]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}}else{cpu.nzcv=4u64 << 28;}
// 0x10e7e820 csel x0, x8, x9, eq
cpu.x[0]=(if cpu.condition(0){((cpu.x[8]))}else{((cpu.x[9]))}) as u64;
// 0x10e7e824 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e7e7d8")),}}}
#[inline]
pub(super) unsafe fn f_10e7e984(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e7e984=>{
// 0x10e7e984 ldr x0, [x0, #0xc8]
{let address=(cpu.x[0]).wrapping_add(0xc8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10e7e988 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e7e984")),}}}
#[inline]
pub(super) unsafe fn f_10e80ab4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e80ab4=>{
// 0x10e80ab4 adrp x0, #0x11a5f000
cpu.x[0]=(0x11a5f000u64) as u64;
// 0x10e80ab8 add x0, x0, #0x5e0
{let a=((cpu.x[0]));let b=((0x5e0u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10e80abc ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e80ab4")),}}}
#[inline]
pub(super) unsafe fn f_10ee25e0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10ee25e0=>{
// 0x10ee25e0 sub sp, sp, #0x40
{let a=((cpu.sp));let b=((0x40u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x10ee25e4 stp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10ee25e8 str x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10ee25ec stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10ee25f0 add x29, sp, #0x10
{let a=((cpu.sp));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x10ee25f4 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10ee25f8 mov x19, x2
cpu.x[19]=(((cpu.x[2]))) as u64;
// 0x10ee25fc mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x10ee2600 ldr x8, [x8, #0x58]
{let address=(cpu.x[8]).wrapping_add(0x58u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10ee2604 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10ee2608;super::dispatch(cpu,rt,target)?;}
pc=0x10ee2608;continue;
}
0x10ee2608=>{
// 0x10ee2608 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10ee260c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10ee2610 bl #0x1187d2d0
cpu.x[30]=0x10ee2614;super::dispatch(cpu,rt,0x1187d2d0)?;
pc=0x10ee2614;continue;
}
0x10ee2614=>{
// 0x10ee2614 stp x20, x0, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=cpu.x[20];let v1=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10ee2618 add x0, x21, #0x18
{let a=((cpu.x[21]));let b=((0x18u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10ee261c mov x1, sp
cpu.x[1]=(((cpu.sp))) as u64;
// 0x10ee2620 bl #0x10d5a458
cpu.x[30]=0x10ee2624;super::chunk11::f_10d5a458(cpu,rt,0x10d5a458)?;
pc=0x10ee2624;continue;
}
0x10ee2624=>{
// 0x10ee2624 cbz x0, #0x10ee2688
if (cpu.x[0]) == 0 {pc=0x10ee2688;continue;}pc=0x10ee2628;continue;
}
0x10ee2628=>{
// 0x10ee2628 ldr x20, [x0, #0x20]
{let address=(cpu.x[0]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10ee262c cbz x20, #0x10ee269c
if (cpu.x[20]) == 0 {pc=0x10ee269c;continue;}pc=0x10ee2630;continue;
}
0x10ee2630=>{
// 0x10ee2630 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10ee2634 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10ee2638 bl #0x1128f410
cpu.x[30]=0x10ee263c;super::dispatch(cpu,rt,0x1128f410)?;
// 0x10ee263c cbz x0, #0x10ee2658
if (cpu.x[0]) == 0 {pc=0x10ee2658;continue;}pc=0x10ee2640;continue;
}
0x10ee2640=>{
// 0x10ee2640 ldr w8, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10ee2644 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10ee2648 b.lt #0x10ee269c
if cpu.condition(11){pc=0x10ee269c;continue;}pc=0x10ee264c;continue;
}
0x10ee264c=>{
// 0x10ee264c ldr x8, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10ee2650 ldr x20, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10ee2654 cbz x20, #0x10ee269c
if (cpu.x[20]) == 0 {pc=0x10ee269c;continue;}pc=0x10ee2658;continue;
}
0x10ee2658=>{
// 0x10ee2658 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10ee265c mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10ee2660 bl #0x1128f380
cpu.x[30]=0x10ee2664;super::dispatch(cpu,rt,0x1128f380)?;
// 0x10ee2664 mov x8, x0
cpu.x[8]=(((cpu.x[0]))) as u64;
// 0x10ee2668 mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
// 0x10ee266c ldr s0, [x8, #0xc]
{let address=(cpu.x[8]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.q[0]=(v0) as u128 & 0xffffffffu128;}
// 0x10ee2670 ldrb w10, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10ee2674 ldr w8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10ee2678 fcvtzs w9, s0
fp_domain(cpu,&[f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32) as f64])?;if f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32).is_subnormal(){return Err("subnormal f32 operand requires architectural handling".into());}cpu.x[9]=((f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32).trunc() as i32) as u64) as u32 as u64;
// 0x10ee267c cmp w10, #0
{let a=((cpu.x[10] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10ee2680 csel w8, w8, w9, eq
cpu.x[8]=(if cpu.condition(0){((cpu.x[8] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10ee2684 str w8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10ee2688;continue;
}
0x10ee2688=>{
// 0x10ee2688 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10ee268c ldr x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10ee2690 ldp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10ee2694 add sp, sp, #0x40
{let a=((cpu.sp));let b=((0x40u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10ee2698 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10ee269c=>{
// 0x10ee269c mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10ee26a0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10ee26a4 ldr x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10ee26a8 ldp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10ee26ac add sp, sp, #0x40
{let a=((cpu.sp));let b=((0x40u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10ee26b0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10ee25e0")),}}}
#[inline]
pub(super) unsafe fn f_10f077e0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f077e0=>{
// 0x10f077e0 movi v0.2d, #0000000000000000
cpu.q[0]=(lane_dup(0u128,64,2)) & 0xffffffffffffffffffffffffffffffffu128;
// 0x10f077e4 adrp x8, #0x11930000
cpu.x[8]=(0x11930000u64) as u64;
// 0x10f077e8 add x8, x8, #0x378
{let a=((cpu.x[8]));let b=((0x378u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10f077ec mov w9, #0x3f800000
cpu.x[9]=(((0x3f800000u64))) as u32 as u64;
// 0x10f077f0 str x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f077f4 str w9, [x0, #0x28]
{let address=(cpu.x[0]).wrapping_add(0x28u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f077f8 str w9, [x0, #0x50]
{let address=(cpu.x[0]).wrapping_add(0x50u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f077fc stur q0, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
pc=0x10f07800;continue;
}
0x10f07800=>{
// 0x10f07800 stur q0, [x0, #0x18]
{let address=(cpu.x[0]).wrapping_add(0x18u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10f07804 stp q0, q0, [x0, #0x30]
{let address=(cpu.x[0]).wrapping_add(0x30u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10f07808 stp x1, x2, [x0, #0x58]
{let address=(cpu.x[0]).wrapping_add(0x58u64);let v0=cpu.x[1];let v1=cpu.x[2];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f0780c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f077e0")),}}}
#[inline]
pub(super) unsafe fn f_10f73f38(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f73f38=>{
// 0x10f73f38 cbz x1, #0x10f73f54
if (cpu.x[1]) == 0 {pc=0x10f73f54;continue;}pc=0x10f73f3c;continue;
}
0x10f73f3c=>{
// 0x10f73f3c ldr x8, [x0, #0x18]
{let address=(cpu.x[0]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f73f40 ldrb w9, [x8, #0x90]
{let address=(cpu.x[8]).wrapping_add(0x90u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f73f44 cbz w9, #0x10f73f54
if (cpu.x[9] as u32 as u64) == 0 {pc=0x10f73f54;continue;}pc=0x10f73f48;continue;
}
0x10f73f48=>{
// 0x10f73f48 mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x10f73f4c mov x1, x8
cpu.x[1]=(((cpu.x[8]))) as u64;
// 0x10f73f50 b #0x10e0bc20
super::chunk1::f_10e0bc20(cpu,rt,0x10e0bc20)?;return Ok(());
}
0x10f73f54=>{
// 0x10f73f54 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10f73f58 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f73f38")),}}}
#[inline]
pub(super) unsafe fn f_10f74154(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f74154=>{
// 0x10f74154 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f74154")),}}}
#[inline]
pub(super) unsafe fn f_10f74164(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f74164=>{
// 0x10f74164 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f74164")),}}}
#[inline]
pub(super) unsafe fn f_10f79064(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f79064=>{
// 0x10f79064 ldr w8, [x0, #0x2c]
{let address=(cpu.x[0]).wrapping_add(0x2cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f79068 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f7906c b.lt #0x10f79098
if cpu.condition(11){pc=0x10f79098;continue;}pc=0x10f79070;continue;
}
0x10f79070=>{
// 0x10f79070 ldr x8, [x0, #0x20]
{let address=(cpu.x[0]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f79074 mov x9, xzr
cpu.x[9]=(((0u64))) as u64;
pc=0x10f79078;continue;
}
0x10f79078=>{
// 0x10f79078 lsl x10, x9, #2
cpu.x[10]=(((((cpu.x[9]))) as u64).wrapping_shl(((((0x2u64))) & 63) as u32) as u64) as u64;
// 0x10f7907c add x9, x9, #1
{let a=((cpu.x[9]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
// 0x10f79080 ldr w11, [x8, x10]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[10])));let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
// 0x10f79084 sub w11, w11, #1
{let a=((cpu.x[11] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[11]=(result) as u32 as u64;}
// 0x10f79088 str w11, [x8, x10]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[10])));let v0=cpu.x[11] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f7908c ldrsw x10, [x0, #0x2c]
{let address=(cpu.x[0]).wrapping_add(0x2cu64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[10]=(v0) as u64;}
// 0x10f79090 cmp x9, x10
{let a=((cpu.x[9]));let b=((cpu.x[10]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f79094 b.lt #0x10f79078
if cpu.condition(11){pc=0x10f79078;continue;}pc=0x10f79098;continue;
}
0x10f79098=>{
// 0x10f79098 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f79064")),}}}
#[inline]
pub(super) unsafe fn f_10f7ba70(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f7ba70=>{
// 0x10f7ba70 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f7ba70")),}}}
#[inline]
pub(super) unsafe fn f_10f7badc(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f7badc=>{
// 0x10f7badc ldrb w0, [x0, #0x119]
{let address=(cpu.x[0]).wrapping_add(0x119u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10f7bae0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f7badc")),}}}
#[inline]
pub(super) unsafe fn f_10f7bb1c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f7bb1c=>{
// 0x10f7bb1c stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f7bb20 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f7bb24 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f7bb28 adrp x8, #0x11980000
cpu.x[8]=(0x11980000u64) as u64;
// 0x10f7bb2c mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f7bb30 ldr x8, [x8, #0x5a0]
{let address=(cpu.x[8]).wrapping_add(0x5a0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7bb34 ldr x9, [x0, #0xe0]
{let address=(cpu.x[0]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f7bb38 add x8, x8, #0x10
{let a=((cpu.x[8]));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10f7bb3c str x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f7bb40 cbz x9, #0x10f7bb84
if (cpu.x[9]) == 0 {pc=0x10f7bb84;continue;}pc=0x10f7bb44;continue;
}
0x10f7bb44=>{
// 0x10f7bb44 ldr x0, [x19, #0xd8]
{let address=(cpu.x[19]).wrapping_add(0xd8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f7bb48 cbz x0, #0x10f7bb5c
if (cpu.x[0]) == 0 {pc=0x10f7bb5c;continue;}pc=0x10f7bb4c;continue;
}
0x10f7bb4c=>{
// 0x10f7bb4c ldr x20, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10f7bb50 bl #0x1187d2c0
cpu.x[30]=0x10f7bb54;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10f7bb54;continue;
}
0x10f7bb54=>{
// 0x10f7bb54 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7bb58 cbnz x20, #0x10f7bb4c
if (cpu.x[20]) != 0 {pc=0x10f7bb4c;continue;}pc=0x10f7bb5c;continue;
}
0x10f7bb5c=>{
// 0x10f7bb5c ldr x8, [x19, #0xd0]
{let address=(cpu.x[19]).wrapping_add(0xd0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7bb60 str xzr, [x19, #0xd8]
{let address=(cpu.x[19]).wrapping_add(0xd8u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f7bb64 cbz x8, #0x10f7bb80
if (cpu.x[8]) == 0 {pc=0x10f7bb80;continue;}pc=0x10f7bb68;continue;
}
0x10f7bb68=>{
// 0x10f7bb68 mov x9, xzr
cpu.x[9]=(((0u64))) as u64;
pc=0x10f7bb6c;continue;
}
0x10f7bb6c=>{
// 0x10f7bb6c ldr x10, [x19, #0xc8]
{let address=(cpu.x[19]).wrapping_add(0xc8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10f7bb70 str xzr, [x10, x9, lsl #3]
{let address=((cpu.x[10]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[9])) as u64).wrapping_shl(3) as u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f7bb74 add x9, x9, #1
{let a=((cpu.x[9]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
// 0x10f7bb78 cmp x8, x9
{let a=((cpu.x[8]));let b=((cpu.x[9]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f7bb7c b.ne #0x10f7bb6c
if cpu.condition(1){pc=0x10f7bb6c;continue;}pc=0x10f7bb80;continue;
}
0x10f7bb80=>{
// 0x10f7bb80 str xzr, [x19, #0xe0]
{let address=(cpu.x[19]).wrapping_add(0xe0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10f7bb84;continue;
}
0x10f7bb84=>{
// 0x10f7bb84 ldr x0, [x19, #0xd8]
{let address=(cpu.x[19]).wrapping_add(0xd8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f7bb88 cbz x0, #0x10f7bb9c
if (cpu.x[0]) == 0 {pc=0x10f7bb9c;continue;}pc=0x10f7bb8c;continue;
}
0x10f7bb8c=>{
// 0x10f7bb8c ldr x20, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10f7bb90 bl #0x1187d2c0
cpu.x[30]=0x10f7bb94;super::dispatch(cpu,rt,0x1187d2c0)?;
// 0x10f7bb94 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f7bb98 cbnz x20, #0x10f7bb8c
if (cpu.x[20]) != 0 {pc=0x10f7bb8c;continue;}pc=0x10f7bb9c;continue;
}
0x10f7bb9c=>{
// 0x10f7bb9c ldr x0, [x19, #0xc8]
{let address=(cpu.x[19]).wrapping_add(0xc8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f7bba0 str xzr, [x19, #0xc8]
{let address=(cpu.x[19]).wrapping_add(0xc8u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f7bba4 cbz x0, #0x10f7bbac
if (cpu.x[0]) == 0 {pc=0x10f7bbac;continue;}pc=0x10f7bba8;continue;
}
0x10f7bba8=>{
// 0x10f7bba8 bl #0x1187d2c0
cpu.x[30]=0x10f7bbac;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10f7bbac;continue;
}
0x10f7bbac=>{
// 0x10f7bbac mov x20, x19
cpu.x[20]=(((cpu.x[19]))) as u64;
// 0x10f7bbb0 ldr x0, [x20, #0x60]!
{let address=(cpu.x[20]).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;cpu.x[20]=(address) as u64;}
// 0x10f7bbb4 cbz x0, #0x10f7bbbc
if (cpu.x[0]) == 0 {pc=0x10f7bbbc;continue;}pc=0x10f7bbb8;continue;
}
0x10f7bbb8=>{
// 0x10f7bbb8 bl #0x1187d360
cpu.x[30]=0x10f7bbbc;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x10f7bbbc;continue;
}
0x10f7bbbc=>{
// 0x10f7bbbc adrp x8, #0x11935000
cpu.x[8]=(0x11935000u64) as u64;
// 0x10f7bbc0 add x8, x8, #0xd08
{let a=((cpu.x[8]));let b=((0xd08u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10f7bbc4 stp xzr, xzr, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f7bbc8 add x8, x8, #0x10
{let a=((cpu.x[8]));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10f7bbcc str x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f7bbd0 ldr x0, [x19, #0x18]!
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;cpu.x[19]=(address) as u64;}
// 0x10f7bbd4 cbz x0, #0x10f7bbdc
if (cpu.x[0]) == 0 {pc=0x10f7bbdc;continue;}pc=0x10f7bbd8;continue;
}
0x10f7bbd8=>{
// 0x10f7bbd8 bl #0x1187d360
cpu.x[30]=0x10f7bbdc;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x10f7bbdc;continue;
}
0x10f7bbdc=>{
// 0x10f7bbdc stp xzr, xzr, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f7bbe0 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f7bbe4 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f7bbe8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f7bb1c")),}}}
#[inline]
pub(super) unsafe fn f_10f81d0c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f81d0c=>{
// 0x10f81d0c add x0, x0, #0xf8
{let a=((cpu.x[0]));let b=((0xf8u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10f81d10 b #0x11294e04
super::chunk4::f_11294e04(cpu,rt,0x11294e04)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f81d0c")),}}}
#[inline]
pub(super) unsafe fn f_10f825cc(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f825cc=>{
// 0x10f825cc ldrb w8, [x0, #0x173]
{let address=(cpu.x[0]).wrapping_add(0x173u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f825d0 cbz w8, #0x10f825dc
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f825dc;continue;}pc=0x10f825d4;continue;
}
0x10f825d4=>{
// 0x10f825d4 mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
// 0x10f825d8 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f825dc=>{
// 0x10f825dc b #0x10f94650
super::chunk6::f_10f94650(cpu,rt,0x10f94650)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f825cc")),}}}
#[inline]
pub(super) unsafe fn f_10f825e4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f825e4=>{
// 0x10f825e4 ldr x8, [x0, #0x180]
{let address=(cpu.x[0]).wrapping_add(0x180u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f825e8 cbz x8, #0x10f825f4
if (cpu.x[8]) == 0 {pc=0x10f825f4;continue;}pc=0x10f825ec;continue;
}
0x10f825ec=>{
// 0x10f825ec and w0, wzr, #1
{let a=((0u64));let b=((0x1u64));let result=a & b;cpu.x[0]=(result) as u32 as u64;}
// 0x10f825f0 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f825f4=>{
// 0x10f825f4 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f825f8 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f825fc stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f82600 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f82604 ldrb w8, [x0, #0x174]
{let address=(cpu.x[0]).wrapping_add(0x174u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f82608 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f8260c mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x10f82610 cbz w8, #0x10f82648
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f82648;continue;}pc=0x10f82614;continue;
}
0x10f82614=>{
// 0x10f82614 cbz x20, #0x10f82634
if (cpu.x[20]) == 0 {pc=0x10f82634;continue;}pc=0x10f82618;continue;
}
0x10f82618=>{
// 0x10f82618 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8261c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f82620 mov w21, w2
cpu.x[21]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x10f82624 ldr x8, [x8, #0x190]
{let address=(cpu.x[8]).wrapping_add(0x190u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f82628 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f8262c;super::dispatch(cpu,rt,target)?;}
pc=0x10f8262c;continue;
}
0x10f8262c=>{
// 0x10f8262c mov w2, w21
cpu.x[2]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10f82630 tbz w0, #0, #0x10f82648
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f82648;continue;}pc=0x10f82634;continue;
}
0x10f82634=>{
// 0x10f82634 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f82638 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f8263c ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f82640 and w0, wzr, #1
{let a=((0u64));let b=((0x1u64));let result=a & b;cpu.x[0]=(result) as u32 as u64;}
// 0x10f82644 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f82648=>{
// 0x10f82648 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8264c mov w21, w2
cpu.x[21]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x10f82650 bl #0x10f940fc
cpu.x[30]=0x10f82654;super::chunk9::f_10f940fc(cpu,rt,0x10f940fc)?;
pc=0x10f82654;continue;
}
0x10f82654=>{
// 0x10f82654 cbz x0, #0x10f826c0
if (cpu.x[0]) == 0 {pc=0x10f826c0;continue;}pc=0x10f82658;continue;
}
0x10f82658=>{
// 0x10f82658 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8265c bl #0x10f940fc
cpu.x[30]=0x10f82660;super::chunk9::f_10f940fc(cpu,rt,0x10f940fc)?;
pc=0x10f82660;continue;
}
0x10f82660=>{
// 0x10f82660 ldr w8, [x0, #0x34]
{let address=(cpu.x[0]).wrapping_add(0x34u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f82664 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f82668 b.lt #0x10f826c0
if cpu.condition(11){pc=0x10f826c0;continue;}pc=0x10f8266c;continue;
}
0x10f8266c=>{
// 0x10f8266c ldr x8, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f82670 ldrb w8, [x8, #0x768]
{let address=(cpu.x[8]).wrapping_add(0x768u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f82674 cbz w8, #0x10f82684
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f82684;continue;}pc=0x10f82678;continue;
}
0x10f82678=>{
// 0x10f82678 cbz x20, #0x10f826c0
if (cpu.x[20]) == 0 {pc=0x10f826c0;continue;}pc=0x10f8267c;continue;
}
0x10f8267c=>{
// 0x10f8267c tbz w21, #0, #0x10f82688
if ((cpu.x[21] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f82688;continue;}pc=0x10f82680;continue;
}
0x10f82680=>{
// 0x10f82680 b #0x10f826c0
pc=0x10f826c0;continue;
}
0x10f82684=>{
// 0x10f82684 cbz x20, #0x10f826c0
if (cpu.x[20]) == 0 {pc=0x10f826c0;continue;}pc=0x10f82688;continue;
}
0x10f82688=>{
// 0x10f82688 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8268c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f82690 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f82694 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f82698;super::dispatch(cpu,rt,target)?;}
pc=0x10f82698;continue;
}
0x10f82698=>{
// 0x10f82698 cmp w0, #5
{let a=((cpu.x[0] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8269c b.ne #0x10f826c0
if cpu.condition(1){pc=0x10f826c0;continue;}pc=0x10f826a0;continue;
}
0x10f826a0=>{
// 0x10f826a0 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f826a4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f826a8 ldr x8, [x8, #0x180]
{let address=(cpu.x[8]).wrapping_add(0x180u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f826ac blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f826b0;super::dispatch(cpu,rt,target)?;}
pc=0x10f826b0;continue;
}
0x10f826b0=>{
// 0x10f826b0 tbz w0, #0, #0x10f82634
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f82634;continue;}pc=0x10f826b4;continue;
}
0x10f826b4=>{
// 0x10f826b4 ldr x0, [x20, #0x48]
{let address=(cpu.x[20]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f826b8 bl #0x10de07cc
cpu.x[30]=0x10f826bc;super::chunk14::f_10de07cc(cpu,rt,0x10de07cc)?;
pc=0x10f826bc;continue;
}
0x10f826bc=>{
// 0x10f826bc tbz w0, #0, #0x10f82634
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f82634;continue;}pc=0x10f826c0;continue;
}
0x10f826c0=>{
// 0x10f826c0 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f826c4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f826c8 ldr x8, [x8, #0xf8]
{let address=(cpu.x[8]).wrapping_add(0xf8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f826cc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f826d0;super::dispatch(cpu,rt,target)?;}
pc=0x10f826d0;continue;
}
0x10f826d0=>{
// 0x10f826d0 eor w8, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a ^ b;cpu.x[8]=(result) as u32 as u64;}
// 0x10f826d4 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f826d8 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f826dc ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f826e0 and w0, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[0]=(result) as u32 as u64;}
// 0x10f826e4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f825e4")),}}}
#[inline]
pub(super) unsafe fn f_10f827c0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f827c0=>{
// 0x10f827c0 adrp x8, #0x11980000
cpu.x[8]=(0x11980000u64) as u64;
// 0x10f827c4 adrp x9, #0x11980000
cpu.x[9]=(0x11980000u64) as u64;
// 0x10f827c8 ldr x8, [x8, #0x538]
{let address=(cpu.x[8]).wrapping_add(0x538u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f827cc ldr x9, [x9, #0x548]
{let address=(cpu.x[9]).wrapping_add(0x548u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f827d0 ldr x10, [x0, #0xb0]
{let address=(cpu.x[0]).wrapping_add(0xb0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10f827d4 ldr x8, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f827d8 ldr x9, [x9]
{let address=(cpu.x[9]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f827dc orr x8, x9, x8
{let a=((cpu.x[9]));let b=((cpu.x[8]));let result=a | b;cpu.x[8]=(result) as u64;}
// 0x10f827e0 tst x10, x8
{let a=((cpu.x[10]));let b=((cpu.x[8]));let result=a & b;cpu.logical_flags(result,64);}
// 0x10f827e4 b.eq #0x10f827f0
if cpu.condition(0){pc=0x10f827f0;continue;}pc=0x10f827e8;continue;
}
0x10f827e8=>{
// 0x10f827e8 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10f827ec ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f827f0=>{
// 0x10f827f0 ldr x8, [x0, #0x48]
{let address=(cpu.x[0]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f827f4 ldrb w8, [x8, #0x3a1]
{let address=(cpu.x[8]).wrapping_add(0x3a1u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f827f8 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f827fc cset w0, ne
cpu.x[0]=((cpu.condition(1)) as u64) as u32 as u64;
pc=0x10f82800;continue;
}
0x10f82800=>{
// 0x10f82800 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f827c0")),}}}
#[inline]
pub(super) unsafe fn f_10f84c5c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f84c5c=>{
// 0x10f84c5c ldr w0, [x0, #0x13c]
{let address=(cpu.x[0]).wrapping_add(0x13cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10f84c60 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f84c5c")),}}}
#[inline]
pub(super) unsafe fn f_10f85848(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f85848=>{
// 0x10f85848 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f85848")),}}}
#[inline]
pub(super) unsafe fn f_10f8584c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f8584c=>{
// 0x10f8584c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f8584c")),}}}
#[inline]
pub(super) unsafe fn f_10f870d8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f870d8=>{
// 0x10f870d8 str w1, [x0, #0x34]
{let address=(cpu.x[0]).wrapping_add(0x34u64);let v0=cpu.x[1] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f870dc ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f870d8")),}}}
#[inline]
pub(super) unsafe fn f_10f87118(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f87118=>{
// 0x10f87118 strb w1, [x0, #0x60]
{let address=(cpu.x[0]).wrapping_add(0x60u64);let v0=cpu.x[1] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f8711c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f87118")),}}}
#[inline]
pub(super) unsafe fn f_10f87150(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f87150=>{
// 0x10f87150 strb w1, [x0, #0x66]
{let address=(cpu.x[0]).wrapping_add(0x66u64);let v0=cpu.x[1] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f87154 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f87150")),}}}
#[inline]
pub(super) unsafe fn f_10f87204(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f87204=>{
// 0x10f87204 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f87208 str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f8720c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f87210 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f87214 bl #0x10f7bb1c
cpu.x[30]=0x10f87218;super::chunk15::f_10f7bb1c(cpu,rt,0x10f7bb1c)?;
pc=0x10f87218;continue;
}
0x10f87218=>{
// 0x10f87218 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8721c ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10f87220 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f87224 b #0x1187d2c0
super::dispatch(cpu,rt,0x1187d2c0)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f87204")),}}}
#[inline]
pub(super) unsafe fn f_10f87244(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f87244=>{
// 0x10f87244 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f87244")),}}}
#[inline]
pub(super) unsafe fn f_10f87fe0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f87fe0=>{
// 0x10f87fe0 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f87fe4 str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f87fe8 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f87fec mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f87ff0 bl #0x10f88004
cpu.x[30]=0x10f87ff4;super::chunk10::f_10f88004(cpu,rt,0x10f88004)?;
pc=0x10f87ff4;continue;
}
0x10f87ff4=>{
// 0x10f87ff4 str xzr, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f87ff8 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10f87ffc ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
pc=0x10f88000;continue;
}
0x10f88000=>{
// 0x10f88000 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f87fe0")),}}}
#[inline]
pub(super) unsafe fn f_10f89e08(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f89e08=>{
// 0x10f89e08 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f89e0c str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f89e10 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f89e14 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f89e18 mov w19, w1
cpu.x[19]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x10f89e1c cmp w1, #0
{let a=((cpu.x[1] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f89e20 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10f89e24 ubfiz x8, x19, #3, #0x20
cpu.x[8]=(((((cpu.x[19])) & 0xffffffffu64) << 3)) as u64;
// 0x10f89e28 csinv x0, x8, xzr, ge
cpu.x[0]=(if cpu.condition(10){((cpu.x[8]))}else{!(((0u64)))}) as u64;
// 0x10f89e2c bl #0x1187d520
cpu.x[30]=0x10f89e30;super::chunk8::f_1187d520(cpu,rt,0x1187d520)?;
pc=0x10f89e30;continue;
}
0x10f89e30=>{
// 0x10f89e30 ldr w9, [x20, #0xc]
{let address=(cpu.x[20]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f89e34 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f89e38 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10f89e3c cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f89e40 b.lt #0x10f89ec8
if cpu.condition(11){pc=0x10f89ec8;continue;}pc=0x10f89e44;continue;
}
0x10f89e44=>{
// 0x10f89e44 cmp w9, #4
{let a=((cpu.x[9] as u32 as u64));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f89e48 b.lo #0x10f89ea8
if cpu.condition(3){pc=0x10f89ea8;continue;}pc=0x10f89e4c;continue;
}
0x10f89e4c=>{
// 0x10f89e4c sub x10, x21, x8
{let a=((cpu.x[21]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.x[10]=(result) as u64;}
// 0x10f89e50 mov x11, x21
cpu.x[11]=(((cpu.x[21]))) as u64;
// 0x10f89e54 cmp x10, #0x20
{let a=((cpu.x[10]));let b=((0x20u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f89e58 mov x10, x8
cpu.x[10]=(((cpu.x[8]))) as u64;
// 0x10f89e5c b.lo #0x10f89eb0
if cpu.condition(3){pc=0x10f89eb0;continue;}pc=0x10f89e60;continue;
}
0x10f89e60=>{
// 0x10f89e60 mov w12, w9
cpu.x[12]=(((cpu.x[9] as u32 as u64))) as u32 as u64;
// 0x10f89e64 add x14, x21, #0x10
{let a=((cpu.x[21]));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[14]=(result) as u64;}
// 0x10f89e68 add x15, x8, #0x10
{let a=((cpu.x[8]));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[15]=(result) as u64;}
// 0x10f89e6c and x13, x12, #0x7ffffffc
{let a=((cpu.x[12]));let b=((0x7ffffffcu64));let result=a & b;cpu.x[13]=(result) as u64;}
// 0x10f89e70 lsl x11, x13, #3
cpu.x[11]=(((((cpu.x[13]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10f89e74 sub w9, w9, w13
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[13] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10f89e78 mov x16, x13
cpu.x[16]=(((cpu.x[13]))) as u64;
// 0x10f89e7c add x10, x8, x11
{let a=((cpu.x[8]));let b=((cpu.x[11]));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x10f89e80 add x11, x21, x11
{let a=((cpu.x[21]));let b=((cpu.x[11]));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
pc=0x10f89e84;continue;
}
0x10f89e84=>{
// 0x10f89e84 ldp q0, q1, [x15, #-0x10]
{let address=(cpu.x[15]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));let v1=rt.memory.read::<u128>(address.wrapping_add(16));cpu.q[0]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;cpu.q[1]=(v1) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x10f89e88 subs x16, x16, #4
{let a=((cpu.x[16]));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.x[16]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x10f89e8c add x15, x15, #0x20
{let a=((cpu.x[15]));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[15]=(result) as u64;}
// 0x10f89e90 stp q0, q1, [x14, #-0x10]
{let address=(cpu.x[14]).wrapping_add(0xfffffffffffffff0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[1] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10f89e94 add x14, x14, #0x20
{let a=((cpu.x[14]));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[14]=(result) as u64;}
// 0x10f89e98 b.ne #0x10f89e84
if cpu.condition(1){pc=0x10f89e84;continue;}pc=0x10f89e9c;continue;
}
0x10f89e9c=>{
// 0x10f89e9c cmp x13, x12
{let a=((cpu.x[13]));let b=((cpu.x[12]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f89ea0 b.ne #0x10f89eb0
if cpu.condition(1){pc=0x10f89eb0;continue;}pc=0x10f89ea4;continue;
}
0x10f89ea4=>{
// 0x10f89ea4 b #0x10f89ec8
pc=0x10f89ec8;continue;
}
0x10f89ea8=>{
// 0x10f89ea8 mov x10, x8
cpu.x[10]=(((cpu.x[8]))) as u64;
// 0x10f89eac mov x11, x21
cpu.x[11]=(((cpu.x[21]))) as u64;
pc=0x10f89eb0;continue;
}
0x10f89eb0=>{
// 0x10f89eb0 add w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
pc=0x10f89eb4;continue;
}
0x10f89eb4=>{
// 0x10f89eb4 ldr x12, [x10], #8
{let address=(cpu.x[10]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[12]=(v0) as u64;cpu.x[10]=((cpu.x[10]).wrapping_add(((0x8u64)))) as u64;}
// 0x10f89eb8 sub w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10f89ebc cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f89ec0 str x12, [x11], #8
{let address=(cpu.x[11]).wrapping_add(0x0u64);let v0=cpu.x[12];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);cpu.x[11]=((cpu.x[11]).wrapping_add(((0x8u64)))) as u64;}
// 0x10f89ec4 b.hi #0x10f89eb4
if cpu.condition(8){pc=0x10f89eb4;continue;}pc=0x10f89ec8;continue;
}
0x10f89ec8=>{
// 0x10f89ec8 cbz x8, #0x10f89ed4
if (cpu.x[8]) == 0 {pc=0x10f89ed4;continue;}pc=0x10f89ecc;continue;
}
0x10f89ecc=>{
// 0x10f89ecc mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10f89ed0 bl #0x1187d360
cpu.x[30]=0x10f89ed4;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x10f89ed4;continue;
}
0x10f89ed4=>{
// 0x10f89ed4 str x21, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f89ed8 str w19, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=cpu.x[19] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f89edc ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f89ee0 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f89ee4 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f89ee8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f89e08")),}}}
#[inline]
pub(super) unsafe fn f_10f8bc88(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f8bc88=>{
// 0x10f8bc88 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f8bc8c str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f8bc90 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f8bc94 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f8bc98 mov w0, w1
cpu.x[0]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x10f8bc9c mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10f8bca0 bl #0x10fdb6b4
cpu.x[30]=0x10f8bca4;super::chunk0::f_10fdb6b4(cpu,rt,0x10fdb6b4)?;
pc=0x10f8bca4;continue;
}
0x10f8bca4=>{
// 0x10f8bca4 ldr w10, [x19, #0x14]
{let address=(cpu.x[19]).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10f8bca8 mov w8, wzr
cpu.x[8]=(((0u64))) as u32 as u64;
// 0x10f8bcac cmp w10, #2
{let a=((cpu.x[10] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8bcb0 b.lt #0x10f8bcf4
if cpu.condition(11){pc=0x10f8bcf4;continue;}pc=0x10f8bcb4;continue;
}
0x10f8bcb4=>{
// 0x10f8bcb4 ldr x9, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f8bcb8 mov w8, wzr
cpu.x[8]=(((0u64))) as u32 as u64;
// 0x10f8bcbc sub w10, w10, #1
{let a=((cpu.x[10] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10f8bcc0 b #0x10f8bcd0
pc=0x10f8bcd0;continue;
}
0x10f8bcc4=>{
// 0x10f8bcc4 cmp w11, w8
{let a=((cpu.x[11] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8bcc8 mov w10, w11
cpu.x[10]=(((cpu.x[11] as u32 as u64))) as u32 as u64;
// 0x10f8bccc b.le #0x10f8bcf4
if cpu.condition(13){pc=0x10f8bcf4;continue;}pc=0x10f8bcd0;continue;
}
0x10f8bcd0=>{
// 0x10f8bcd0 add w11, w10, w8
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u32 as u64;}
// 0x10f8bcd4 asr w11, w11, #1
cpu.x[11]=(((((cpu.x[11] as u32 as u64))) as i32).wrapping_shr(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10f8bcd8 ldr x12, [x9, w11, sxtw #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[11] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[12]=(v0) as u64;}
// 0x10f8bcdc ldr w12, [x12, #8]
{let address=(cpu.x[12]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[12]=(v0) as u32 as u64;}
// 0x10f8bce0 cmp w12, w0
{let a=((cpu.x[12] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8bce4 b.ge #0x10f8bcc4
if cpu.condition(10){pc=0x10f8bcc4;continue;}pc=0x10f8bce8;continue;
}
0x10f8bce8=>{
// 0x10f8bce8 add w8, w11, #1
{let a=((cpu.x[11] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10f8bcec mov w11, w10
cpu.x[11]=(((cpu.x[10] as u32 as u64))) as u32 as u64;
// 0x10f8bcf0 b #0x10f8bcc4
pc=0x10f8bcc4;continue;
}
0x10f8bcf4=>{
// 0x10f8bcf4 mov w0, w8
cpu.x[0]=(((cpu.x[8] as u32 as u64))) as u32 as u64;
// 0x10f8bcf8 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10f8bcfc ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f8bd00 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f8bc88")),}}}
#[inline]
pub(super) unsafe fn f_10f8c3ac(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f8c3ac=>{
// 0x10f8c3ac ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8c3b0 ldr x2, [x8, #0x50]
{let address=(cpu.x[8]).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10f8c3b4 br x2
{let target=cpu.x[2];if target>=0x10f8c3ac && target<0x10f8c3b8{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f8c3ac")),}}}
#[inline]
pub(super) unsafe fn f_10f8e4c0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f8e4c0=>{
// 0x10f8e4c0 mov w0, #4
cpu.x[0]=(((0x4u64))) as u32 as u64;
// 0x10f8e4c4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f8e4c0")),}}}
#[inline]
pub(super) unsafe fn f_10f9188c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f9188c=>{
// 0x10f9188c sub sp, sp, #0xe0
{let a=((cpu.sp));let b=((0xe0u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x10f91890 str d8, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=(cpu.q[8] & 0xffffffffffffffffu128) as u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f91894 stp x29, x30, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f91898 stp x28, x27, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=cpu.x[28];let v1=cpu.x[27];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f9189c stp x26, x25, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=cpu.x[26];let v1=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f918a0 stp x24, x23, [sp, #0xb0]
{let address=(cpu.sp).wrapping_add(0xb0u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f918a4 stp x22, x21, [sp, #0xc0]
{let address=(cpu.sp).wrapping_add(0xc0u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f918a8 stp x20, x19, [sp, #0xd0]
{let address=(cpu.sp).wrapping_add(0xd0u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f918ac add x29, sp, #0x80
{let a=((cpu.sp));let b=((0x80u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x10f918b0 ldr x8, [x0, #0x100]
{let address=(cpu.x[0]).wrapping_add(0x100u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f918b4 mov w21, w2
cpu.x[21]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x10f918b8 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f918bc mov w22, w1
cpu.x[22]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x10f918c0 cbz x8, #0x10f918d4
if (cpu.x[8]) == 0 {pc=0x10f918d4;continue;}pc=0x10f918c4;continue;
}
0x10f918c4=>{
// 0x10f918c4 ldr x0, [x8, #0x58]
{let address=(cpu.x[8]).wrapping_add(0x58u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f918c8 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f918cc ldr x8, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f918d0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f918d4;super::dispatch(cpu,rt,target)?;}
pc=0x10f918d4;continue;
}
0x10f918d4=>{
// 0x10f918d4 ldr x20, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10f918d8 ldr x8, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f918dc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f918e0 stur x8, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f918e4 bl #0x10e5aeb0
cpu.x[30]=0x10f918e8;super::chunk6::f_10e5aeb0(cpu,rt,0x10e5aeb0)?;
pc=0x10f918e8;continue;
}
0x10f918e8=>{
// 0x10f918e8 ldr w27, [x19, #0x198]
{let address=(cpu.x[19]).wrapping_add(0x198u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[27]=(v0) as u32 as u64;}
// 0x10f918ec mov w25, w0
cpu.x[25]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f918f0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f918f4 mov x1, x27
cpu.x[1]=(((cpu.x[27]))) as u64;
// 0x10f918f8 bl #0x10e5ae54
cpu.x[30]=0x10f918fc;super::chunk9::f_10e5ae54(cpu,rt,0x10e5ae54)?;
pc=0x10f918fc;continue;
}
0x10f918fc=>{
// 0x10f918fc mov w23, w0
cpu.x[23]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f91900 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f91904 mov x1, x27
cpu.x[1]=(((cpu.x[27]))) as u64;
// 0x10f91908 bl #0x10e5ad94
cpu.x[30]=0x10f9190c;super::chunk14::f_10e5ad94(cpu,rt,0x10e5ad94)?;
pc=0x10f9190c;continue;
}
0x10f9190c=>{
// 0x10f9190c mov w24, w0
cpu.x[24]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f91910 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f91914 mov x1, x27
cpu.x[1]=(((cpu.x[27]))) as u64;
// 0x10f91918 bl #0x10e5adcc
cpu.x[30]=0x10f9191c;super::chunk10::f_10e5adcc(cpu,rt,0x10e5adcc)?;
pc=0x10f9191c;continue;
}
0x10f9191c=>{
// 0x10f9191c mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f91920 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f91924 mov x1, x27
cpu.x[1]=(((cpu.x[27]))) as u64;
// 0x10f91928 bl #0x10e5aed8
cpu.x[30]=0x10f9192c;super::chunk6::f_10e5aed8(cpu,rt,0x10e5aed8)?;
pc=0x10f9192c;continue;
}
0x10f9192c=>{
// 0x10f9192c ldr w8, [x19, #0x148]
{let address=(cpu.x[19]).wrapping_add(0x148u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f91930 mov w28, w0
cpu.x[28]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f91934 str w22, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[22] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f91938 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9193c b.lt #0x10f91984
if cpu.condition(11){pc=0x10f91984;continue;}pc=0x10f91940;continue;
}
0x10f91940=>{
// 0x10f91940 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f91944 mov w1, w23
cpu.x[1]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10f91948 bl #0x10e5afcc
cpu.x[30]=0x10f9194c;super::dispatch(cpu,rt,0x10e5afcc)?;
// 0x10f9194c mov w23, w0
cpu.x[23]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f91950 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f91954 mov w1, w24
cpu.x[1]=(((cpu.x[24] as u32 as u64))) as u32 as u64;
// 0x10f91958 bl #0x10e5afcc
cpu.x[30]=0x10f9195c;super::dispatch(cpu,rt,0x10e5afcc)?;
// 0x10f9195c stur w0, [x29, #-0x38]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffc8u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f91960 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f91964 mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10f91968 bl #0x10e5af70
cpu.x[30]=0x10f9196c;super::dispatch(cpu,rt,0x10e5af70)?;
// 0x10f9196c mov w28, w0
cpu.x[28]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f91970 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f91974 mov w1, w26
cpu.x[1]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10f91978 bl #0x10e5af70
cpu.x[30]=0x10f9197c;super::dispatch(cpu,rt,0x10e5af70)?;
// 0x10f9197c mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f91980 b #0x10f91988
pc=0x10f91988;continue;
}
0x10f91984=>{
// 0x10f91984 stur w24, [x29, #-0x38]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffc8u64);let v0=cpu.x[24] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10f91988;continue;
}
0x10f91988=>{
// 0x10f91988 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f9198c bl #0x10f93d2c
cpu.x[30]=0x10f91990;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
pc=0x10f91990;continue;
}
0x10f91990=>{
// 0x10f91990 ldr w8, [x0, #0x324]
{let address=(cpu.x[0]).wrapping_add(0x324u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f91994 stur w25, [x29, #-0x34]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffccu64);let v0=cpu.x[25] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f91998 stur w26, [x29, #-0x3c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffc4u64);let v0=cpu.x[26] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f9199c add w24, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[24]=(result) as u32 as u64;}
// 0x10f919a0 str w24, [x0, #0x324]
{let address=(cpu.x[0]).wrapping_add(0x324u64);let v0=cpu.x[24] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f919a4 ldr x22, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[22]=(v0) as u64;}
// 0x10f919a8 cbz x22, #0x10f919ec
if (cpu.x[22]) == 0 {pc=0x10f919ec;continue;}pc=0x10f919ac;continue;
}
0x10f919ac=>{
// 0x10f919ac ldr w4, [x19, #0x198]
{let address=(cpu.x[19]).wrapping_add(0x198u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[4]=(v0) as u32 as u64;}
// 0x10f919b0 ldr x5, [x19, #0x108]
{let address=(cpu.x[19]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[5]=(v0) as u64;}
// 0x10f919b4 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f919b8 mov w1, w23
cpu.x[1]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10f919bc mov w2, w24
cpu.x[2]=(((cpu.x[24] as u32 as u64))) as u32 as u64;
// 0x10f919c0 mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10f919c4 bl #0x10f77e90
cpu.x[30]=0x10f919c8;super::chunk12::f_10f77e90(cpu,rt,0x10f77e90)?;
// 0x10f919c8 ldr w4, [x19, #0x198]
{let address=(cpu.x[19]).wrapping_add(0x198u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[4]=(v0) as u32 as u64;}
// 0x10f919cc ldr x5, [x19, #0x108]
{let address=(cpu.x[19]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[5]=(v0) as u64;}
// 0x10f919d0 mov w23, w0
cpu.x[23]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f919d4 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f919d8 mov w1, w28
cpu.x[1]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10f919dc mov w2, w24
cpu.x[2]=(((cpu.x[24] as u32 as u64))) as u32 as u64;
// 0x10f919e0 mov w3, #1
cpu.x[3]=(((0x1u64))) as u32 as u64;
// 0x10f919e4 bl #0x10f77e90
cpu.x[30]=0x10f919e8;super::chunk12::f_10f77e90(cpu,rt,0x10f77e90)?;
// 0x10f919e8 mov w28, w0
cpu.x[28]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
pc=0x10f919ec;continue;
}
0x10f919ec=>{
// 0x10f919ec mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f919f0 bl #0x10e5aec0
cpu.x[30]=0x10f919f4;super::chunk7::f_10e5aec0(cpu,rt,0x10e5aec0)?;
pc=0x10f919f4;continue;
}
0x10f919f4=>{
// 0x10f919f4 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f919f8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f919fc bl #0x10e5a950
cpu.x[30]=0x10f91a00;super::chunk12::f_10e5a950(cpu,rt,0x10e5a950)?;
pc=0x10f91a00;continue;
}
0x10f91a00=>{
// 0x10f91a00 ldr x8, [x20, #0x120]
{let address=(cpu.x[20]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f91a04 mov w25, w0
cpu.x[25]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f91a08 str w21, [sp, #0x34]
{let address=(cpu.sp).wrapping_add(0x34u64);let v0=cpu.x[21] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f91a0c str x8, [sp, #0x38]
{let address=(cpu.sp).wrapping_add(0x38u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f91a10 tbz w21, #0, #0x10f91a48
if ((cpu.x[21] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f91a48;continue;}pc=0x10f91a14;continue;
}
0x10f91a14=>{
// 0x10f91a14 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f91a18 bl #0x10f94298
cpu.x[30]=0x10f91a1c;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
// 0x10f91a1c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f91a20 bl #0x10f942a0
cpu.x[30]=0x10f91a24;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
// 0x10f91a24 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f91a28 bl #0x10f94298
cpu.x[30]=0x10f91a2c;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
// 0x10f91a2c stur w0, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f91a30 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f91a34 bl #0x10f942a0
cpu.x[30]=0x10f91a38;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
// 0x10f91a38 stur w0, [x29, #-0x30]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f91a3c ldr x0, [x19, #0x108]
{let address=(cpu.x[19]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f91a40 cbnz x0, #0x10f91aa4
if (cpu.x[0]) != 0 {pc=0x10f91aa4;continue;}pc=0x10f91a44;continue;
}
0x10f91a44=>{
// 0x10f91a44 b #0x10f91ad4
pc=0x10f91ad4;continue;
}
0x10f91a48=>{
// 0x10f91a48 ldrb w8, [x19, #0x14c]
{let address=(cpu.x[19]).wrapping_add(0x14cu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f91a4c cbz w8, #0x10f91a84
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f91a84;continue;}pc=0x10f91a50;continue;
}
0x10f91a50=>{
// 0x10f91a50 mov x9, x19
cpu.x[9]=(((cpu.x[19]))) as u64;
pc=0x10f91a54;continue;
}
0x10f91a54=>{
// 0x10f91a54 mov x8, x9
cpu.x[8]=(((cpu.x[9]))) as u64;
// 0x10f91a58 ldr x9, [x9, #0x150]
{let address=(cpu.x[9]).wrapping_add(0x150u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f91a5c cbnz x9, #0x10f91a54
if (cpu.x[9]) != 0 {pc=0x10f91a54;continue;}pc=0x10f91a60;continue;
}
0x10f91a60=>{
// 0x10f91a60 ldr x8, [x8, #0x168]
{let address=(cpu.x[8]).wrapping_add(0x168u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f91a64 cbz x8, #0x10f91a84
if (cpu.x[8]) == 0 {pc=0x10f91a84;continue;}pc=0x10f91a68;continue;
}
0x10f91a68=>{
// 0x10f91a68 ldr w9, [x19, #0x148]
{let address=(cpu.x[19]).wrapping_add(0x148u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f91a6c cbnz w9, #0x10f91a84
if (cpu.x[9] as u32 as u64) != 0 {pc=0x10f91a84;continue;}pc=0x10f91a70;continue;
}
0x10f91a70=>{
// 0x10f91a70 ldrb w8, [x8, #0xd]
{let address=(cpu.x[8]).wrapping_add(0xdu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f91a74 cbz w8, #0x10f91a84
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f91a84;continue;}pc=0x10f91a78;continue;
}
0x10f91a78=>{
// 0x10f91a78 add x8, x19, #0x160
{let a=((cpu.x[19]));let b=((0x160u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10f91a7c add x9, x19, #0x164
{let a=((cpu.x[19]));let b=((0x164u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
// 0x10f91a80 b #0x10f91a8c
pc=0x10f91a8c;continue;
}
0x10f91a84=>{
// 0x10f91a84 add x8, x19, #0x120
{let a=((cpu.x[19]));let b=((0x120u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10f91a88 add x9, x19, #0x124
{let a=((cpu.x[19]));let b=((0x124u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
pc=0x10f91a8c;continue;
}
0x10f91a8c=>{
// 0x10f91a8c ldr w8, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f91a90 stur w8, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f91a94 ldr w8, [x9]
{let address=(cpu.x[9]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f91a98 stur w8, [x29, #-0x30]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f91a9c ldr x0, [x19, #0x108]
{let address=(cpu.x[19]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f91aa0 cbz x0, #0x10f91ad4
if (cpu.x[0]) == 0 {pc=0x10f91ad4;continue;}pc=0x10f91aa4;continue;
}
0x10f91aa4=>{
// 0x10f91aa4 ldr x2, [x20, #0x2c0]
{let address=(cpu.x[20]).wrapping_add(0x2c0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10f91aa8 cbz x2, #0x10f91ad4
if (cpu.x[2]) == 0 {pc=0x10f91ad4;continue;}pc=0x10f91aac;continue;
}
0x10f91aac=>{
// 0x10f91aac ldr x8, [x19, #0x208]
{let address=(cpu.x[19]).wrapping_add(0x208u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f91ab0 ldr w21, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[21]=(v0) as u32 as u64;}
// 0x10f91ab4 ldr w9, [x19, #0x214]
{let address=(cpu.x[19]).wrapping_add(0x214u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f91ab8 sub x8, x8, #4
{let a=((cpu.x[8]));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u64;}
pc=0x10f91abc;continue;
}
0x10f91abc=>{
// 0x10f91abc cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f91ac0 b.lt #0x10f922f4
if cpu.condition(11){pc=0x10f922f4;continue;}pc=0x10f91ac4;continue;
}
0x10f91ac4=>{
// 0x10f91ac4 ldr w10, [x8, x9, lsl #2]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[9])) as u64).wrapping_shl(2) as u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10f91ac8 sub x9, x9, #1
{let a=((cpu.x[9]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u64;}
// 0x10f91acc cmp w10, w21
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[21] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f91ad0 b.ne #0x10f91abc
if cpu.condition(1){pc=0x10f91abc;continue;}pc=0x10f91ad4;continue;
}
0x10f91ad4=>{
// 0x10f91ad4 ldrb w22, [x20, #0x231]
{let address=(cpu.x[20]).wrapping_add(0x231u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[22]=(v0) as u32 as u64;}
// 0x10f91ad8 cmp w26, #1
{let a=((cpu.x[26] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f91adc b.lt #0x10f9234c
if cpu.condition(11){pc=0x10f9234c;continue;}pc=0x10f91ae0;continue;
}
0x10f91ae0=>{
// 0x10f91ae0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f91ae4 str w24, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[24] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f91ae8 mov w24, w23
cpu.x[24]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10f91aec bl #0x10e5aed0
cpu.x[30]=0x10f91af0;super::chunk12::f_10e5aed0(cpu,rt,0x10e5aed0)?;
pc=0x10f91af0;continue;
}
0x10f91af0=>{
// 0x10f91af0 ldur w21, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[21]=(v0) as u32 as u64;}
// 0x10f91af4 ldur w27, [x29, #-0x30]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffd0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[27]=(v0) as u32 as u64;}
// 0x10f91af8 mov w23, w0
cpu.x[23]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f91afc cbz w22, #0x10f91b1c
if (cpu.x[22] as u32 as u64) == 0 {pc=0x10f91b1c;continue;}pc=0x10f91b00;continue;
}
0x10f91b00=>{
// 0x10f91b00 ldr x5, [sp, #0x38]
{let address=(cpu.sp).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[5]=(v0) as u64;}
// 0x10f91b04 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f91b08 mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10f91b0c mov w2, w27
cpu.x[2]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10f91b10 mov w3, w26
cpu.x[3]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10f91b14 mov w4, w23
cpu.x[4]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10f91b18 bl #0x10f8f4bc
cpu.x[30]=0x10f91b1c;super::dispatch(cpu,rt,0x10f8f4bc)?;
pc=0x10f91b1c;continue;
}
0x10f91b1c=>{
// 0x10f91b1c ldrb w8, [x20, #0x2d6]
{let address=(cpu.x[20]).wrapping_add(0x2d6u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f91b20 ldr w6, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[6]=(v0) as u32 as u64;}
// 0x10f91b24 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f91b28 ldur w7, [x29, #-0x34]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffccu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[7]=(v0) as u32 as u64;}
// 0x10f91b2c mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10f91b30 mov w2, w27
cpu.x[2]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10f91b34 strb w8, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f91b38 ldur w8, [x29, #-0x3c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffc4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f91b3c mov w3, w26
cpu.x[3]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10f91b40 mov w4, w25
cpu.x[4]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10f91b44 mov w5, w24
cpu.x[5]=(((cpu.x[24] as u32 as u64))) as u32 as u64;
// 0x10f91b48 str w28, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=cpu.x[28] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f91b4c str w8, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f91b50 ldur w8, [x29, #-0x38]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffc8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f91b54 str w23, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=cpu.x[23] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f91b58 str w8, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f91b5c bl #0x10f8f03c
cpu.x[30]=0x10f91b60;super::chunk2::f_10f8f03c(cpu,rt,0x10f8f03c)?;
pc=0x10f91b60;continue;
}
0x10f91b60=>{
// 0x10f91b60 ldur x25, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[25]=(v0) as u64;}
pc=0x10f91b64;continue;
}
0x10f91b64=>{
// 0x10f91b64 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f91b68 bl #0x10e5a908
cpu.x[30]=0x10f91b6c;super::chunk5::f_10e5a908(cpu,rt,0x10e5a908)?;
pc=0x10f91b6c;continue;
}
0x10f91b6c=>{
// 0x10f91b6c tbz w0, #0, #0x10f91bd8
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f91bd8;continue;}pc=0x10f91b70;continue;
}
0x10f91b70=>{
// 0x10f91b70 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f91b74 bl #0x10e5b088
cpu.x[30]=0x10f91b78;super::chunk4::f_10e5b088(cpu,rt,0x10e5b088)?;
pc=0x10f91b78;continue;
}
0x10f91b78=>{
// 0x10f91b78 ldr w2, [x19, #0x120]
{let address=(cpu.x[19]).wrapping_add(0x120u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[2]=(v0) as u32 as u64;}
// 0x10f91b7c ldr w3, [x19, #0x124]
{let address=(cpu.x[19]).wrapping_add(0x124u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10f91b80 mov x1, x0
cpu.x[1]=(((cpu.x[0]))) as u64;
// 0x10f91b84 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f91b88 mov w4, wzr
cpu.x[4]=(((0u64))) as u32 as u64;
// 0x10f91b8c bl #0x10f8f978
cpu.x[30]=0x10f91b90;super::chunk14::f_10f8f978(cpu,rt,0x10f8f978)?;
pc=0x10f91b90;continue;
}
0x10f91b90=>{
// 0x10f91b90 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f91b94 bl #0x10f94298
cpu.x[30]=0x10f91b98;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
pc=0x10f91b98;continue;
}
0x10f91b98=>{
// 0x10f91b98 mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f91b9c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f91ba0 bl #0x10f942a0
cpu.x[30]=0x10f91ba4;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
pc=0x10f91ba4;continue;
}
0x10f91ba4=>{
// 0x10f91ba4 mov w23, w0
cpu.x[23]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f91ba8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f91bac bl #0x10f942a8
cpu.x[30]=0x10f91bb0;super::chunk11::f_10f942a8(cpu,rt,0x10f942a8)?;
pc=0x10f91bb0;continue;
}
0x10f91bb0=>{
// 0x10f91bb0 mov w3, w0
cpu.x[3]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f91bb4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f91bb8 mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10f91bbc mov w2, w23
cpu.x[2]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10f91bc0 mov w4, wzr
cpu.x[4]=(((0u64))) as u32 as u64;
// 0x10f91bc4 bl #0x10f8f2d0
cpu.x[30]=0x10f91bc8;super::chunk2::f_10f8f2d0(cpu,rt,0x10f8f2d0)?;
pc=0x10f91bc8;continue;
}
0x10f91bc8=>{
// 0x10f91bc8 ldr w8, [x19, #0x148]
{let address=(cpu.x[19]).wrapping_add(0x148u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f91bcc cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f91bd0 b.ge #0x10f91c74
if cpu.condition(10){pc=0x10f91c74;continue;}pc=0x10f91bd4;continue;
}
0x10f91bd4=>{
// 0x10f91bd4 b #0x10f91c7c
pc=0x10f91c7c;continue;
}
0x10f91bd8=>{
// 0x10f91bd8 ldr x23, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10f91bdc ldrb w8, [x23, #0x340]
{let address=(cpu.x[23]).wrapping_add(0x340u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f91be0 cbz w8, #0x10f91c1c
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f91c1c;continue;}pc=0x10f91be4;continue;
}
0x10f91be4=>{
// 0x10f91be4 ldr x8, [x19, #0x108]
{let address=(cpu.x[19]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f91be8 cbnz x8, #0x10f91c1c
if (cpu.x[8]) != 0 {pc=0x10f91c1c;continue;}pc=0x10f91bec;continue;
}
0x10f91bec=>{
// 0x10f91bec ldr x8, [x19, #0x110]
{let address=(cpu.x[19]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f91bf0 cbnz x8, #0x10f91c1c
if (cpu.x[8]) != 0 {pc=0x10f91c1c;continue;}pc=0x10f91bf4;continue;
}
0x10f91bf4=>{
// 0x10f91bf4 ldr x0, [x19, #0x100]
{let address=(cpu.x[19]).wrapping_add(0x100u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f91bf8 cbz x0, #0x10f91c1c
if (cpu.x[0]) == 0 {pc=0x10f91c1c;continue;}pc=0x10f91bfc;continue;
}
0x10f91bfc=>{
// 0x10f91bfc ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f91c00 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f91c04 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f91c08;super::dispatch(cpu,rt,target)?;}
// 0x10f91c08 cmp w0, #5
{let a=((cpu.x[0] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f91c0c b.ne #0x10f91c1c
if cpu.condition(1){pc=0x10f91c1c;continue;}pc=0x10f91c10;continue;
}
0x10f91c10=>{
// 0x10f91c10 ldr x0, [x19, #0x100]
{let address=(cpu.x[19]).wrapping_add(0x100u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f91c14 cbz x0, #0x10f91c1c
if (cpu.x[0]) == 0 {pc=0x10f91c1c;continue;}pc=0x10f91c18;continue;
}
0x10f91c18=>{
// 0x10f91c18 bl #0x10f94310
cpu.x[30]=0x10f91c1c;super::chunk7::f_10f94310(cpu,rt,0x10f94310)?;
pc=0x10f91c1c;continue;
}
0x10f91c1c=>{
// 0x10f91c1c mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10f91c20 bl #0x10e5ad8c
cpu.x[30]=0x10f91c24;super::chunk12::f_10e5ad8c(cpu,rt,0x10e5ad8c)?;
pc=0x10f91c24;continue;
}
0x10f91c24=>{
// 0x10f91c24 ldr w8, [sp, #0x34]
{let address=(cpu.sp).wrapping_add(0x34u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f91c28 mov x23, x0
cpu.x[23]=(((cpu.x[0]))) as u64;
// 0x10f91c2c tbz w8, #0, #0x10f91c4c
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f91c4c;continue;}pc=0x10f91c30;continue;
}
0x10f91c30=>{
// 0x10f91c30 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f91c34 bl #0x10f94298
cpu.x[30]=0x10f91c38;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
// 0x10f91c38 mov w24, w0
cpu.x[24]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f91c3c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f91c40 bl #0x10f942a0
cpu.x[30]=0x10f91c44;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
// 0x10f91c44 mov w3, w0
cpu.x[3]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f91c48 b #0x10f91c54
pc=0x10f91c54;continue;
}
0x10f91c4c=>{
// 0x10f91c4c ldr w24, [x19, #0x120]
{let address=(cpu.x[19]).wrapping_add(0x120u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[24]=(v0) as u32 as u64;}
// 0x10f91c50 ldr w3, [x19, #0x124]
{let address=(cpu.x[19]).wrapping_add(0x124u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
pc=0x10f91c54;continue;
}
0x10f91c54=>{
// 0x10f91c54 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f91c58 mov x1, x23
cpu.x[1]=(((cpu.x[23]))) as u64;
// 0x10f91c5c mov w2, w24
cpu.x[2]=(((cpu.x[24] as u32 as u64))) as u32 as u64;
// 0x10f91c60 mov w4, wzr
cpu.x[4]=(((0u64))) as u32 as u64;
// 0x10f91c64 bl #0x10f8f978
cpu.x[30]=0x10f91c68;super::chunk14::f_10f8f978(cpu,rt,0x10f8f978)?;
pc=0x10f91c68;continue;
}
0x10f91c68=>{
// 0x10f91c68 ldr w8, [x19, #0x148]
{let address=(cpu.x[19]).wrapping_add(0x148u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f91c6c cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f91c70 b.lt #0x10f91c7c
if cpu.condition(11){pc=0x10f91c7c;continue;}pc=0x10f91c74;continue;
}
0x10f91c74=>{
// 0x10f91c74 ldr x24, [x20, #0x110]
{let address=(cpu.x[20]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[24]=(v0) as u64;}
// 0x10f91c78 cbnz x24, #0x10f91c8c
if (cpu.x[24]) != 0 {pc=0x10f91c8c;continue;}pc=0x10f91c7c;continue;
}
0x10f91c7c=>{
// 0x10f91c7c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f91c80 bl #0x10e5b04c
cpu.x[30]=0x10f91c84;super::chunk9::f_10e5b04c(cpu,rt,0x10e5b04c)?;
pc=0x10f91c84;continue;
}
0x10f91c84=>{
// 0x10f91c84 mov x24, x0
cpu.x[24]=(((cpu.x[0]))) as u64;
// 0x10f91c88 cbz x0, #0x10f91ef0
if (cpu.x[0]) == 0 {pc=0x10f91ef0;continue;}pc=0x10f91c8c;continue;
}
0x10f91c8c=>{
// 0x10f91c8c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f91c90 bl #0x10f94124
cpu.x[30]=0x10f91c94;super::chunk5::f_10f94124(cpu,rt,0x10f94124)?;
pc=0x10f91c94;continue;
}
0x10f91c94=>{
// 0x10f91c94 stur w0, [x29, #-0x34]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffccu64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f91c98 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f91c9c bl #0x10e5b05c
cpu.x[30]=0x10f91ca0;super::chunk11::f_10e5b05c(cpu,rt,0x10e5b05c)?;
pc=0x10f91ca0;continue;
}
0x10f91ca0=>{
// 0x10f91ca0 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f91ca4 b.lt #0x10f91ef0
if cpu.condition(11){pc=0x10f91ef0;continue;}pc=0x10f91ca8;continue;
}
0x10f91ca8=>{
// 0x10f91ca8 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f91cac mov w22, wzr
cpu.x[22]=(((0u64))) as u32 as u64;
// 0x10f91cb0 mov w27, wzr
cpu.x[27]=(((0u64))) as u32 as u64;
// 0x10f91cb4 b #0x10f91cc8
pc=0x10f91cc8;continue;
}
0x10f91cb8=>{
// 0x10f91cb8 add w27, w27, #1
{let a=((cpu.x[27] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[27]=(result) as u32 as u64;}
// 0x10f91cbc add w22, w22, #0x14
{let a=((cpu.x[22] as u32 as u64));let b=((0x14u64));let result=a.wrapping_add(b);cpu.x[22]=(result) as u32 as u64;}
// 0x10f91cc0 cmp w26, w27
{let a=((cpu.x[26] as u32 as u64));let b=((cpu.x[27] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f91cc4 b.eq #0x10f91ef0
if cpu.condition(0){pc=0x10f91ef0;continue;}pc=0x10f91cc8;continue;
}
0x10f91cc8=>{
// 0x10f91cc8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f91ccc stur xzr, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f91cd0 bl #0x10f93d2c
cpu.x[30]=0x10f91cd4;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
pc=0x10f91cd4;continue;
}
0x10f91cd4=>{
// 0x10f91cd4 ldur w1, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f91cd8 ldur w2, [x29, #-0x30]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffd0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[2]=(v0) as u32 as u64;}
// 0x10f91cdc mov w3, #-1
cpu.x[3]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10f91ce0 mov w4, wzr
cpu.x[4]=(((0u64))) as u32 as u64;
// 0x10f91ce4 bl #0x10d38d9c
cpu.x[30]=0x10f91ce8;super::chunk6::f_10d38d9c(cpu,rt,0x10d38d9c)?;
pc=0x10f91ce8;continue;
}
0x10f91ce8=>{
// 0x10f91ce8 mov w28, w0
cpu.x[28]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f91cec cmp w26, #1
{let a=((cpu.x[26] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f91cf0 mov w23, wzr
cpu.x[23]=(((0u64))) as u32 as u64;
// 0x10f91cf4 b.eq #0x10f91d04
if cpu.condition(0){pc=0x10f91d04;continue;}pc=0x10f91cf8;continue;
}
0x10f91cf8=>{
// 0x10f91cf8 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10f91cfc bl #0x10de0430
cpu.x[30]=0x10f91d00;super::chunk11::f_10de0430(cpu,rt,0x10de0430)?;
// 0x10f91d00 mov w23, w0
cpu.x[23]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
pc=0x10f91d04;continue;
}
0x10f91d04=>{
// 0x10f91d04 ldur w8, [x29, #-0x34]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffccu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f91d08 ldr x0, [x25, #0x68]
{let address=(cpu.x[25]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f91d0c ldr w21, [x24, #0x38c]
{let address=(cpu.x[24]).wrapping_add(0x38cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[21]=(v0) as u32 as u64;}
// 0x10f91d10 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f91d14 cset w25, eq
cpu.x[25]=((cpu.condition(0)) as u64) as u32 as u64;
// 0x10f91d18 bl #0x110cbf10
cpu.x[30]=0x10f91d1c;super::chunk14::f_110cbf10(cpu,rt,0x110cbf10)?;
pc=0x10f91d1c;continue;
}
0x10f91d1c=>{
// 0x10f91d1c bl #0x10e16208
cpu.x[30]=0x10f91d20;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
pc=0x10f91d20;continue;
}
0x10f91d20=>{
// 0x10f91d20 mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10f91d24 str x0, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f91d28 sub x7, x29, #8
{let a=((cpu.x[29]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.x[7]=(result) as u64;}
// 0x10f91d2c mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10f91d30 mov w0, w27
cpu.x[0]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10f91d34 mov w1, w26
cpu.x[1]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10f91d38 mov w2, w23
cpu.x[2]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10f91d3c mov w4, w25
cpu.x[4]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10f91d40 mov w5, w28
cpu.x[5]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10f91d44 mov w6, w21
cpu.x[6]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10f91d48 strb w8, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f91d4c str wzr, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f91d50 bl #0x10d42ce0
cpu.x[30]=0x10f91d54;super::chunk4::f_10d42ce0(cpu,rt,0x10d42ce0)?;
pc=0x10f91d54;continue;
}
0x10f91d54=>{
// 0x10f91d54 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f91d58 bl #0x10f93d2c
cpu.x[30]=0x10f91d5c;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
pc=0x10f91d5c;continue;
}
0x10f91d5c=>{
// 0x10f91d5c ldp w23, w25, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[23]=(v0) as u32 as u64;cpu.x[25]=(v1) as u32 as u64;}
// 0x10f91d60 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10f91d64 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f91d68 bl #0x10f93fc0
cpu.x[30]=0x10f91d6c;super::chunk5::f_10f93fc0(cpu,rt,0x10f93fc0)?;
pc=0x10f91d6c;continue;
}
0x10f91d6c=>{
// 0x10f91d6c ldur w7, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[7]=(v0) as u32 as u64;}
// 0x10f91d70 ldur w8, [x29, #-0x30]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffd0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f91d74 mov x4, x0
cpu.x[4]=(((cpu.x[0]))) as u64;
// 0x10f91d78 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f91d7c mov x1, x24
cpu.x[1]=(((cpu.x[24]))) as u64;
// 0x10f91d80 mov x5, xzr
cpu.x[5]=(((0u64))) as u64;
// 0x10f91d84 add w2, w23, w7
{let a=((cpu.x[23] as u32 as u64));let b=((cpu.x[7] as u32 as u64));let result=a.wrapping_add(b);cpu.x[2]=(result) as u32 as u64;}
// 0x10f91d88 add w3, w25, w8
{let a=((cpu.x[25] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_add(b);cpu.x[3]=(result) as u32 as u64;}
// 0x10f91d8c mov w6, wzr
cpu.x[6]=(((0u64))) as u32 as u64;
// 0x10f91d90 strb wzr, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f91d94 bl #0x10d35ab4
cpu.x[30]=0x10f91d98;super::chunk5::f_10d35ab4(cpu,rt,0x10d35ab4)?;
pc=0x10f91d98;continue;
}
0x10f91d98=>{
// 0x10f91d98 ldrb w8, [x20, #0x1a3]
{let address=(cpu.x[20]).wrapping_add(0x1a3u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f91d9c mov x28, x0
cpu.x[28]=(((cpu.x[0]))) as u64;
// 0x10f91da0 cbz w8, #0x10f91dac
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f91dac;continue;}pc=0x10f91da4;continue;
}
0x10f91da4=>{
// 0x10f91da4 mul w8, w22, w22
cpu.x[8]=((((cpu.x[22] as u32 as u64))).wrapping_mul(((cpu.x[22] as u32 as u64)))) as u32 as u64;
// 0x10f91da8 str w8, [x28, #0x1a0]
{let address=(cpu.x[28]).wrapping_add(0x1a0u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10f91dac;continue;
}
0x10f91dac=>{
// 0x10f91dac ldr w1, [x19, #0x198]
{let address=(cpu.x[19]).wrapping_add(0x198u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f91db0 mov x0, x28
cpu.x[0]=(((cpu.x[28]))) as u64;
// 0x10f91db4 bl #0x10f7ed38
cpu.x[30]=0x10f91db8;super::chunk11::f_10f7ed38(cpu,rt,0x10f7ed38)?;
pc=0x10f91db8;continue;
}
0x10f91db8=>{
// 0x10f91db8 mov x0, x28
cpu.x[0]=(((cpu.x[28]))) as u64;
// 0x10f91dbc mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f91dc0 bl #0x10f94068
cpu.x[30]=0x10f91dc4;super::chunk13::f_10f94068(cpu,rt,0x10f94068)?;
pc=0x10f91dc4;continue;
}
0x10f91dc4=>{
// 0x10f91dc4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f91dc8 bl #0x10f95c00
cpu.x[30]=0x10f91dcc;super::chunk11::f_10f95c00(cpu,rt,0x10f95c00)?;
pc=0x10f91dcc;continue;
}
0x10f91dcc=>{
// 0x10f91dcc mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f91dd0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f91dd4 bl #0x10f95c08
cpu.x[30]=0x10f91dd8;super::chunk13::f_10f95c08(cpu,rt,0x10f95c08)?;
pc=0x10f91dd8;continue;
}
0x10f91dd8=>{
// 0x10f91dd8 ldrb w3, [x19, #0xa4]
{let address=(cpu.x[19]).wrapping_add(0xa4u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10f91ddc ldr w4, [x19, #0xac]
{let address=(cpu.x[19]).wrapping_add(0xacu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[4]=(v0) as u32 as u64;}
// 0x10f91de0 mov w2, w0
cpu.x[2]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f91de4 mov x0, x28
cpu.x[0]=(((cpu.x[28]))) as u64;
// 0x10f91de8 mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10f91dec bl #0x10f9407c
cpu.x[30]=0x10f91df0;super::chunk4::f_10f9407c(cpu,rt,0x10f9407c)?;
pc=0x10f91df0;continue;
}
0x10f91df0=>{
// 0x10f91df0 ldr w1, [x19, #0xa0]
{let address=(cpu.x[19]).wrapping_add(0xa0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f91df4 mov x0, x28
cpu.x[0]=(((cpu.x[28]))) as u64;
// 0x10f91df8 bl #0x10f95c20
cpu.x[30]=0x10f91dfc;super::chunk4::f_10f95c20(cpu,rt,0x10f95c20)?;
pc=0x10f91dfc;continue;
}
0x10f91dfc=>{
// 0x10f91dfc ldr w8, [x19, #0xa8]
{let address=(cpu.x[19]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f91e00 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f91e04 str w8, [x28, #0xa8]
{let address=(cpu.x[28]).wrapping_add(0xa8u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f91e08 ldr w8, [x19, #0x1c4]
{let address=(cpu.x[19]).wrapping_add(0x1c4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f91e0c str w8, [x28, #0x124]
{let address=(cpu.x[28]).wrapping_add(0x124u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f91e10 bl #0x10e5b054
cpu.x[30]=0x10f91e14;super::chunk12::f_10e5b054(cpu,rt,0x10e5b054)?;
pc=0x10f91e14;continue;
}
0x10f91e14=>{
// 0x10f91e14 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f91e18 b.lt #0x10f91e28
if cpu.condition(11){pc=0x10f91e28;continue;}pc=0x10f91e1c;continue;
}
0x10f91e1c=>{
// 0x10f91e1c mov w1, w0
cpu.x[1]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f91e20 mov x0, x28
cpu.x[0]=(((cpu.x[28]))) as u64;
// 0x10f91e24 bl #0x10f85324
cpu.x[30]=0x10f91e28;super::chunk14::f_10f85324(cpu,rt,0x10f85324)?;
pc=0x10f91e28;continue;
}
0x10f91e28=>{
// 0x10f91e28 ldrb w8, [x20, #0x108]
{let address=(cpu.x[20]).wrapping_add(0x108u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f91e2c ldur x25, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[25]=(v0) as u64;}
// 0x10f91e30 cbz w8, #0x10f91e50
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f91e50;continue;}pc=0x10f91e34;continue;
}
0x10f91e34=>{
// 0x10f91e34 mov x0, x28
cpu.x[0]=(((cpu.x[28]))) as u64;
// 0x10f91e38 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10f91e3c mov x2, xzr
cpu.x[2]=(((0u64))) as u64;
// 0x10f91e40 bl #0x10f83114
cpu.x[30]=0x10f91e44;super::chunk3::f_10f83114(cpu,rt,0x10f83114)?;
// 0x10f91e44 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10f91e48 strb w8, [x28, #0x1f4]
{let address=(cpu.x[28]).wrapping_add(0x1f4u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f91e4c b #0x10f91e60
pc=0x10f91e60;continue;
}
0x10f91e50=>{
// 0x10f91e50 ldrb w1, [x19, #0x188]
{let address=(cpu.x[19]).wrapping_add(0x188u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f91e54 mov x0, x28
cpu.x[0]=(((cpu.x[28]))) as u64;
// 0x10f91e58 mov x2, xzr
cpu.x[2]=(((0u64))) as u64;
// 0x10f91e5c bl #0x10f83114
cpu.x[30]=0x10f91e60;super::chunk3::f_10f83114(cpu,rt,0x10f83114)?;
pc=0x10f91e60;continue;
}
0x10f91e60=>{
// 0x10f91e60 mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10f91e64 mov x1, x28
cpu.x[1]=(((cpu.x[28]))) as u64;
// 0x10f91e68 mov w2, wzr
cpu.x[2]=(((0u64))) as u32 as u64;
// 0x10f91e6c bl #0x10f8c294
cpu.x[30]=0x10f91e70;super::chunk4::f_10f8c294(cpu,rt,0x10f8c294)?;
pc=0x10f91e70;continue;
}
0x10f91e70=>{
// 0x10f91e70 mov x0, x28
cpu.x[0]=(((cpu.x[28]))) as u64;
// 0x10f91e74 bl #0x10f94298
cpu.x[30]=0x10f91e78;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
pc=0x10f91e78;continue;
}
0x10f91e78=>{
// 0x10f91e78 mov w23, w0
cpu.x[23]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f91e7c mov x0, x28
cpu.x[0]=(((cpu.x[28]))) as u64;
// 0x10f91e80 bl #0x10f942a0
cpu.x[30]=0x10f91e84;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
pc=0x10f91e84;continue;
}
0x10f91e84=>{
// 0x10f91e84 mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f91e88 mov x0, x28
cpu.x[0]=(((cpu.x[28]))) as u64;
// 0x10f91e8c bl #0x10f942a8
cpu.x[30]=0x10f91e90;super::chunk11::f_10f942a8(cpu,rt,0x10f942a8)?;
pc=0x10f91e90;continue;
}
0x10f91e90=>{
// 0x10f91e90 cbz w0, #0x10f91ea0
if (cpu.x[0] as u32 as u64) == 0 {pc=0x10f91ea0;continue;}pc=0x10f91e94;continue;
}
0x10f91e94=>{
// 0x10f91e94 ldr w8, [x20, #0x15c]
{let address=(cpu.x[20]).wrapping_add(0x15cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f91e98 cbnz w8, #0x10f91ed4
if (cpu.x[8] as u32 as u64) != 0 {pc=0x10f91ed4;continue;}pc=0x10f91e9c;continue;
}
0x10f91e9c=>{
// 0x10f91e9c b #0x10f91cb8
pc=0x10f91cb8;continue;
}
0x10f91ea0=>{
// 0x10f91ea0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f91ea4 bl #0x10f93d2c
cpu.x[30]=0x10f91ea8;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
pc=0x10f91ea8;continue;
}
0x10f91ea8=>{
// 0x10f91ea8 mov w1, w23
cpu.x[1]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10f91eac mov w2, w21
cpu.x[2]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10f91eb0 mov w3, #-1
cpu.x[3]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10f91eb4 bl #0x10d392c8
cpu.x[30]=0x10f91eb8;super::chunk4::f_10d392c8(cpu,rt,0x10d392c8)?;
pc=0x10f91eb8;continue;
}
0x10f91eb8=>{
// 0x10f91eb8 asr w2, w0, #0x10
cpu.x[2]=(((((cpu.x[0] as u32 as u64))) as i32).wrapping_shr(((((0x10u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10f91ebc and w1, w0, #0xffff
{let a=((cpu.x[0] as u32 as u64));let b=((0xffffu64));let result=a & b;cpu.x[1]=(result) as u32 as u64;}
// 0x10f91ec0 mov x0, x28
cpu.x[0]=(((cpu.x[28]))) as u64;
// 0x10f91ec4 mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10f91ec8 bl #0x10f94250
cpu.x[30]=0x10f91ecc;super::chunk3::f_10f94250(cpu,rt,0x10f94250)?;
pc=0x10f91ecc;continue;
}
0x10f91ecc=>{
// 0x10f91ecc ldr w8, [x20, #0x15c]
{let address=(cpu.x[20]).wrapping_add(0x15cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f91ed0 cbz w8, #0x10f91cb8
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f91cb8;continue;}pc=0x10f91ed4;continue;
}
0x10f91ed4=>{
// 0x10f91ed4 ldr x0, [x28, #0x58]
{let address=(cpu.x[28]).wrapping_add(0x58u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f91ed8 add x1, x20, #0x158
{let a=((cpu.x[20]));let b=((0x158u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x10f91edc mov w2, #1
cpu.x[2]=(((0x1u64))) as u32 as u64;
// 0x10f91ee0 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f91ee4 ldr x8, [x8, #0x140]
{let address=(cpu.x[8]).wrapping_add(0x140u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f91ee8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f91eec;super::dispatch(cpu,rt,target)?;}
pc=0x10f91eec;continue;
}
0x10f91eec=>{
// 0x10f91eec b #0x10f91cb8
pc=0x10f91cb8;continue;
}
0x10f91ef0=>{
// 0x10f91ef0 ldr x1, [x20, #0x118]
{let address=(cpu.x[20]).wrapping_add(0x118u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10f91ef4 cbz x1, #0x10f91f14
if (cpu.x[1]) == 0 {pc=0x10f91f14;continue;}pc=0x10f91ef8;continue;
}
0x10f91ef8=>{
// 0x10f91ef8 ldr w8, [x1, #0x1a0]
{let address=(cpu.x[1]).wrapping_add(0x1a0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f91efc cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f91f00 b.eq #0x10f91f14
if cpu.condition(0){pc=0x10f91f14;continue;}pc=0x10f91f04;continue;
}
0x10f91f04=>{
// 0x10f91f04 ldur w2, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[2]=(v0) as u32 as u64;}
// 0x10f91f08 ldur w3, [x29, #-0x30]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffd0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10f91f0c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f91f10 bl #0x10f8fccc
cpu.x[30]=0x10f91f14;super::dispatch(cpu,rt,0x10f8fccc)?;
pc=0x10f91f14;continue;
}
0x10f91f14=>{
// 0x10f91f14 ldr x8, [x20, #0x138]
{let address=(cpu.x[20]).wrapping_add(0x138u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f91f18 cbz x8, #0x10f91f30
if (cpu.x[8]) == 0 {pc=0x10f91f30;continue;}pc=0x10f91f1c;continue;
}
0x10f91f1c=>{
// 0x10f91f1c ldr w9, [x19, #0x1c8]
{let address=(cpu.x[19]).wrapping_add(0x1c8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f91f20 cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f91f24 b.lt #0x10f92194
if cpu.condition(11){pc=0x10f92194;continue;}pc=0x10f91f28;continue;
}
0x10f91f28=>{
// 0x10f91f28 ldr w8, [x8, #0x228]
{let address=(cpu.x[8]).wrapping_add(0x228u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f91f2c b #0x10f91f48
pc=0x10f91f48;continue;
}
0x10f91f30=>{
// 0x10f91f30 ldr x8, [x19, #0x1d0]
{let address=(cpu.x[19]).wrapping_add(0x1d0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f91f34 cbz x8, #0x10f92194
if (cpu.x[8]) == 0 {pc=0x10f92194;continue;}pc=0x10f91f38;continue;
}
0x10f91f38=>{
// 0x10f91f38 ldr w8, [x19, #0x1c8]
{let address=(cpu.x[19]).wrapping_add(0x1c8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f91f3c cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f91f40 b.le #0x10f92194
if cpu.condition(13){pc=0x10f92194;continue;}pc=0x10f91f44;continue;
}
0x10f91f44=>{
// 0x10f91f44 mov w8, wzr
cpu.x[8]=(((0u64))) as u32 as u64;
pc=0x10f91f48;continue;
}
0x10f91f48=>{
// 0x10f91f48 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f91f4c csinc w27, w8, wzr, gt
cpu.x[27]=(if cpu.condition(12){((cpu.x[8] as u32 as u64))}else{(((0u64))).wrapping_add(1)}) as u32 as u64;
// 0x10f91f50 add x8, x19, #0x12c
{let a=((cpu.x[19]));let b=((0x12cu64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10f91f54 neg w28, w27, lsr #1
cpu.x[28]=(0u64.wrapping_sub((((cpu.x[27] as u32 as u64)) as u32).wrapping_shr(1) as u64)) as u32 as u64;
// 0x10f91f58 stur x8, [x29, #-0x30]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f91f5c stur w27, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=cpu.x[27] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f91f60 b #0x10f91fec
pc=0x10f91fec;continue;
}
0x10f91f64=>{
// 0x10f91f64 ldr w9, [x20, #0x198]
{let address=(cpu.x[20]).wrapping_add(0x198u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f91f68 ldr w10, [x19, #0x1d8]
{let address=(cpu.x[19]).wrapping_add(0x1d8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10f91f6c madd w26, w10, w9, w8
cpu.x[26]=((((cpu.x[8] as u32 as u64))).wrapping_add((((cpu.x[10] as u32 as u64))).wrapping_mul(((cpu.x[9] as u32 as u64))))) as u32 as u64;
pc=0x10f91f70;continue;
}
0x10f91f70=>{
// 0x10f91f70 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f91f74 bl #0x10f94298
cpu.x[30]=0x10f91f78;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
pc=0x10f91f78;continue;
}
0x10f91f78=>{
// 0x10f91f78 mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f91f7c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f91f80 bl #0x10f942a0
cpu.x[30]=0x10f91f84;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
pc=0x10f91f84;continue;
}
0x10f91f84=>{
// 0x10f91f84 ldr w8, [x19, #0x198]
{let address=(cpu.x[19]).wrapping_add(0x198u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f91f88 mov w3, w0
cpu.x[3]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f91f8c mov x1, xzr
cpu.x[1]=(((0u64))) as u64;
// 0x10f91f90 mov x5, xzr
cpu.x[5]=(((0u64))) as u64;
// 0x10f91f94 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f91f98 mov w2, w21
cpu.x[2]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10f91f9c mov w4, w24
cpu.x[4]=(((cpu.x[24] as u32 as u64))) as u32 as u64;
// 0x10f91fa0 mov w6, w26
cpu.x[6]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10f91fa4 mov w7, w25
cpu.x[7]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10f91fa8 str wzr, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f91fac str wzr, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f91fb0 stp x23, x8, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=cpu.x[23];let v1=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f91fb4 str x19, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f91fb8 bl #0x10f8e6fc
cpu.x[30]=0x10f91fbc;super::chunk5::f_10f8e6fc(cpu,rt,0x10f8e6fc)?;
pc=0x10f91fbc;continue;
}
0x10f91fbc=>{
// 0x10f91fbc ldur x25, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[25]=(v0) as u64;}
// 0x10f91fc0 mov x1, x22
cpu.x[1]=(((cpu.x[22]))) as u64;
// 0x10f91fc4 mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10f91fc8 bl #0x10f8c2a8
cpu.x[30]=0x10f91fcc;super::chunk5::f_10f8c2a8(cpu,rt,0x10f8c2a8)?;
pc=0x10f91fcc;continue;
}
0x10f91fcc=>{
// 0x10f91fcc ldr w8, [x19, #0x1c8]
{let address=(cpu.x[19]).wrapping_add(0x1c8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f91fd0 subs w27, w27, #1
{let a=((cpu.x[27] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[27]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10f91fd4 add w28, w28, #1
{let a=((cpu.x[28] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[28]=(result) as u32 as u64;}
// 0x10f91fd8 sub w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10f91fdc str w8, [x22, #0x1c8]
{let address=(cpu.x[22]).wrapping_add(0x1c8u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f91fe0 ldr w8, [x19, #0x1d8]
{let address=(cpu.x[19]).wrapping_add(0x1d8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f91fe4 str w8, [x22, #0x1d8]
{let address=(cpu.x[22]).wrapping_add(0x1d8u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f91fe8 b.eq #0x10f92194
if cpu.condition(0){pc=0x10f92194;continue;}pc=0x10f91fec;continue;
}
0x10f91fec=>{
// 0x10f91fec ldr x0, [x20, #0x138]
{let address=(cpu.x[20]).wrapping_add(0x138u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f91ff0 cbz x0, #0x10f92034
if (cpu.x[0]) == 0 {pc=0x10f92034;continue;}pc=0x10f91ff4;continue;
}
0x10f91ff4=>{
// 0x10f91ff4 mov w1, #4
cpu.x[1]=(((0x4u64))) as u32 as u64;
// 0x10f91ff8 bl #0x10f8a138
cpu.x[30]=0x10f91ffc;super::chunk11::f_10f8a138(cpu,rt,0x10f8a138)?;
pc=0x10f91ffc;continue;
}
0x10f91ffc=>{
// 0x10f91ffc ldr x8, [x20, #0x138]
{let address=(cpu.x[20]).wrapping_add(0x138u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
pc=0x10f92000;continue;
}
0x10f92000=>{
// 0x10f92000 mov x22, x0
cpu.x[22]=(((cpu.x[0]))) as u64;
// 0x10f92004 str x8, [x0, #0x1d0]
{let address=(cpu.x[0]).wrapping_add(0x1d0u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f92008 ldrb w8, [x20, #0x224]
{let address=(cpu.x[20]).wrapping_add(0x224u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f9200c cbz w8, #0x10f9204c
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f9204c;continue;}pc=0x10f92010;continue;
}
0x10f92010=>{
// 0x10f92010 ldr w8, [x19, #0x120]
{let address=(cpu.x[19]).wrapping_add(0x120u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f92014 mov w9, #0x2327
cpu.x[9]=(((0x2327u64))) as u32 as u64;
// 0x10f92018 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9201c b.gt #0x10f92170
if cpu.condition(12){pc=0x10f92170;continue;}pc=0x10f92020;continue;
}
0x10f92020=>{
// 0x10f92020 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10f92024 str w8, [x19, #0x1d8]
{let address=(cpu.x[19]).wrapping_add(0x1d8u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f92028 ldrb w8, [x20, #0x1b8]
{let address=(cpu.x[20]).wrapping_add(0x1b8u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f9202c cbnz w8, #0x10f9205c
if (cpu.x[8] as u32 as u64) != 0 {pc=0x10f9205c;continue;}pc=0x10f92030;continue;
}
0x10f92030=>{
// 0x10f92030 b #0x10f92054
pc=0x10f92054;continue;
}
0x10f92034=>{
// 0x10f92034 ldr x0, [x19, #0x1d0]
{let address=(cpu.x[19]).wrapping_add(0x1d0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f92038 mov w1, #4
cpu.x[1]=(((0x4u64))) as u32 as u64;
// 0x10f9203c bl #0x10f8a138
cpu.x[30]=0x10f92040;super::chunk11::f_10f8a138(cpu,rt,0x10f8a138)?;
// 0x10f92040 ldr x8, [x19, #0x1d0]
{let address=(cpu.x[19]).wrapping_add(0x1d0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f92044 mov x22, x0
cpu.x[22]=(((cpu.x[0]))) as u64;
// 0x10f92048 str x8, [x0, #0x1d0]
{let address=(cpu.x[0]).wrapping_add(0x1d0u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10f9204c;continue;
}
0x10f9204c=>{
// 0x10f9204c ldrb w8, [x20, #0x1b8]
{let address=(cpu.x[20]).wrapping_add(0x1b8u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f92050 cbnz w8, #0x10f9205c
if (cpu.x[8] as u32 as u64) != 0 {pc=0x10f9205c;continue;}pc=0x10f92054;continue;
}
0x10f92054=>{
// 0x10f92054 ldr w8, [x19, #0xc0]
{let address=(cpu.x[19]).wrapping_add(0xc0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f92058 str w8, [x22, #0xc0]
{let address=(cpu.x[22]).wrapping_add(0xc0u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10f9205c;continue;
}
0x10f9205c=>{
// 0x10f9205c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f92060 bl #0x10f93d3c
cpu.x[30]=0x10f92064;super::chunk3::f_10f93d3c(cpu,rt,0x10f93d3c)?;
pc=0x10f92064;continue;
}
0x10f92064=>{
// 0x10f92064 mov w1, w0
cpu.x[1]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f92068 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f9206c bl #0x10f94050
cpu.x[30]=0x10f92070;super::chunk10::f_10f94050(cpu,rt,0x10f94050)?;
pc=0x10f92070;continue;
}
0x10f92070=>{
// 0x10f92070 ldr w1, [x19, #0xa0]
{let address=(cpu.x[19]).wrapping_add(0xa0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f92074 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f92078 bl #0x10f95c20
cpu.x[30]=0x10f9207c;super::chunk4::f_10f95c20(cpu,rt,0x10f95c20)?;
pc=0x10f9207c;continue;
}
0x10f9207c=>{
// 0x10f9207c ldr w8, [x19, #0x1c4]
{let address=(cpu.x[19]).wrapping_add(0x1c4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f92080 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f92084 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f92088 str w8, [x22, #0x1c4]
{let address=(cpu.x[22]).wrapping_add(0x1c4u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f9208c bl #0x10f94068
cpu.x[30]=0x10f92090;super::chunk13::f_10f94068(cpu,rt,0x10f94068)?;
pc=0x10f92090;continue;
}
0x10f92090=>{
// 0x10f92090 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f92094 bl #0x10f95c00
cpu.x[30]=0x10f92098;super::chunk11::f_10f95c00(cpu,rt,0x10f95c00)?;
pc=0x10f92098;continue;
}
0x10f92098=>{
// 0x10f92098 mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f9209c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f920a0 bl #0x10f95c08
cpu.x[30]=0x10f920a4;super::chunk13::f_10f95c08(cpu,rt,0x10f95c08)?;
pc=0x10f920a4;continue;
}
0x10f920a4=>{
// 0x10f920a4 ldrb w3, [x19, #0xa4]
{let address=(cpu.x[19]).wrapping_add(0xa4u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10f920a8 ldr w4, [x19, #0xac]
{let address=(cpu.x[19]).wrapping_add(0xacu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[4]=(v0) as u32 as u64;}
// 0x10f920ac mov w2, w0
cpu.x[2]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f920b0 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f920b4 mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10f920b8 bl #0x10f9407c
cpu.x[30]=0x10f920bc;super::chunk4::f_10f9407c(cpu,rt,0x10f9407c)?;
pc=0x10f920bc;continue;
}
0x10f920bc=>{
// 0x10f920bc ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f920c0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f920c4 ldr x8, [x8, #0x150]
{let address=(cpu.x[8]).wrapping_add(0x150u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f920c8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f920cc;super::dispatch(cpu,rt,target)?;}
pc=0x10f920cc;continue;
}
0x10f920cc=>{
// 0x10f920cc mov x23, x0
cpu.x[23]=(((cpu.x[0]))) as u64;
// 0x10f920d0 cbnz x0, #0x10f920e8
if (cpu.x[0]) != 0 {pc=0x10f920e8;continue;}pc=0x10f920d4;continue;
}
0x10f920d4=>{
// 0x10f920d4 ldr x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f920d8 bl #0x10e0b6b4
cpu.x[30]=0x10f920dc;super::chunk8::f_10e0b6b4(cpu,rt,0x10e0b6b4)?;
pc=0x10f920dc;continue;
}
0x10f920dc=>{
// 0x10f920dc ldr x0, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f920e0 bl #0x10e19dc0
cpu.x[30]=0x10f920e4;super::chunk9::f_10e19dc0(cpu,rt,0x10e19dc0)?;
pc=0x10f920e4;continue;
}
0x10f920e4=>{
// 0x10f920e4 mov x23, x0
cpu.x[23]=(((cpu.x[0]))) as u64;
pc=0x10f920e8;continue;
}
0x10f920e8=>{
// 0x10f920e8 ldr x8, [x22, #0x48]
{let address=(cpu.x[22]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f920ec ldur w9, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f920f0 sub x0, x29, #8
{let a=((cpu.x[29]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.x[0]=(result) as u64;}
// 0x10f920f4 ldr d0, [x19, #0x120]
{let address=(cpu.x[19]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.q[0]=(v0) as u128 & 0xffffffffffffffffu128;}
// 0x10f920f8 ldr w8, [x8, #0x22c]
{let address=(cpu.x[8]).wrapping_add(0x22cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f920fc mul w8, w8, w28
cpu.x[8]=((((cpu.x[8] as u32 as u64))).wrapping_mul(((cpu.x[28] as u32 as u64)))) as u32 as u64;
// 0x10f92100 sdiv w1, w8, w9
{let a=((cpu.x[8] as u32 as u64)) as i32;let b=((cpu.x[9] as u32 as u64)) as i32;cpu.x[1]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u32 as u64;}
// 0x10f92104 ldur x8, [x29, #-0x30]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffd0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f92108 ldr d1, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.q[1]=(v0) as u128 & 0xffffffffffffffffu128;}
// 0x10f9210c sub v8.2s, v0.2s, v1.2s
cpu.q[8]=(lane_binary((cpu.q[0] & 0xffffffffffffffffu128),(cpu.q[1] & 0xffffffffffffffffu128),32,2,"sub")) & 0xffffffffffffffffu128;
// 0x10f92110 stur d8, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=(cpu.q[8] & 0xffffffffffffffffu128) as u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f92114 bl #0x11295174
cpu.x[30]=0x10f92118;super::chunk8::f_11295174(cpu,rt,0x11295174)?;
pc=0x10f92118;continue;
}
0x10f92118=>{
// 0x10f92118 sub x0, x29, #0x18
{let a=((cpu.x[29]));let b=((0x18u64));let result=a.wrapping_sub(b);cpu.x[0]=(result) as u64;}
// 0x10f9211c mov w1, #0x5dc
cpu.x[1]=(((0x5dcu64))) as u32 as u64;
// 0x10f92120 stur d8, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=(cpu.q[8] & 0xffffffffffffffffu128) as u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f92124 bl #0x11294d74
cpu.x[30]=0x10f92128;super::chunk2::f_11294d74(cpu,rt,0x11294d74)?;
pc=0x10f92128;continue;
}
0x10f92128=>{
// 0x10f92128 ldrb w9, [x20, #0x224]
{let address=(cpu.x[20]).wrapping_add(0x224u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f9212c ldr w8, [x19, #0x120]
{let address=(cpu.x[19]).wrapping_add(0x120u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f92130 ldr w25, [x19, #0x124]
{let address=(cpu.x[19]).wrapping_add(0x124u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[25]=(v0) as u32 as u64;}
// 0x10f92134 ldr w24, [x19, #0x128]
{let address=(cpu.x[19]).wrapping_add(0x128u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[24]=(v0) as u32 as u64;}
// 0x10f92138 cbnz w9, #0x10f91f64
if (cpu.x[9] as u32 as u64) != 0 {pc=0x10f91f64;continue;}pc=0x10f9213c;continue;
}
0x10f9213c=>{
// 0x10f9213c ldrb w9, [x20, #0x225]
{let address=(cpu.x[20]).wrapping_add(0x225u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f92140 cbz w9, #0x10f92184
if (cpu.x[9] as u32 as u64) == 0 {pc=0x10f92184;continue;}pc=0x10f92144;continue;
}
0x10f92144=>{
// 0x10f92144 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f92148 bl #0x10f9414c
cpu.x[30]=0x10f9214c;super::chunk3::f_10f9414c(cpu,rt,0x10f9414c)?;
// 0x10f9214c tst w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.logical_flags(result,32);}
// 0x10f92150 mov w8, #-1
cpu.x[8]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10f92154 ldr w10, [x19, #0x124]
{let address=(cpu.x[19]).wrapping_add(0x124u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10f92158 cneg w8, w8, eq
cpu.x[8]=(if cpu.condition(0){0u64.wrapping_sub(((cpu.x[8] as u32 as u64)))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10f9215c ldr w26, [x19, #0x120]
{let address=(cpu.x[19]).wrapping_add(0x120u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[26]=(v0) as u32 as u64;}
// 0x10f92160 str w8, [x19, #0x1d8]
{let address=(cpu.x[19]).wrapping_add(0x1d8u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f92164 ldr w9, [x20, #0x198]
{let address=(cpu.x[20]).wrapping_add(0x198u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f92168 msub w25, w8, w9, w10
cpu.x[25]=((((cpu.x[10] as u32 as u64))).wrapping_sub((((cpu.x[8] as u32 as u64))).wrapping_mul(((cpu.x[9] as u32 as u64))))) as u32 as u64;
// 0x10f9216c b #0x10f91f70
pc=0x10f91f70;continue;
}
0x10f92170=>{
// 0x10f92170 mov w8, #-1
cpu.x[8]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10f92174 str w8, [x19, #0x1d8]
{let address=(cpu.x[19]).wrapping_add(0x1d8u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f92178 ldrb w8, [x20, #0x1b8]
{let address=(cpu.x[20]).wrapping_add(0x1b8u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f9217c cbnz w8, #0x10f9205c
if (cpu.x[8] as u32 as u64) != 0 {pc=0x10f9205c;continue;}pc=0x10f92180;continue;
}
0x10f92180=>{
// 0x10f92180 b #0x10f92054
pc=0x10f92054;continue;
}
0x10f92184=>{
// 0x10f92184 ldp w10, w9, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[10]=(v0) as u32 as u64;cpu.x[9]=(v1) as u32 as u64;}
// 0x10f92188 add w25, w9, w25
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[25] as u32 as u64));let result=a.wrapping_add(b);cpu.x[25]=(result) as u32 as u64;}
// 0x10f9218c add w26, w10, w8
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_add(b);cpu.x[26]=(result) as u32 as u64;}
// 0x10f92190 b #0x10f91f70
pc=0x10f91f70;continue;
}
0x10f92194=>{
// 0x10f92194 ldr w8, [x20, #0x1b0]
{let address=(cpu.x[20]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f92198 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9219c b.lt #0x10f92454
if cpu.condition(11){pc=0x10f92454;continue;}pc=0x10f921a0;continue;
}
0x10f921a0=>{
// 0x10f921a0 ldr w8, [x19, #0x1b8]
{let address=(cpu.x[19]).wrapping_add(0x1b8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f921a4 ldr w9, [x20, #0x1b4]
{let address=(cpu.x[20]).wrapping_add(0x1b4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f921a8 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f921ac b.ge #0x10f9228c
if cpu.condition(10){pc=0x10f9228c;continue;}pc=0x10f921b0;continue;
}
0x10f921b0=>{
// 0x10f921b0 ldr w8, [x25, #0x14]
{let address=(cpu.x[25]).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f921b4 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f921b8 b.lt #0x10f9243c
if cpu.condition(11){pc=0x10f9243c;continue;}pc=0x10f921bc;continue;
}
0x10f921bc=>{
// 0x10f921bc mov x24, xzr
cpu.x[24]=(((0u64))) as u64;
// 0x10f921c0 mov x22, xzr
cpu.x[22]=(((0u64))) as u64;
// 0x10f921c4 mov x9, x25
cpu.x[9]=(((cpu.x[25]))) as u64;
// 0x10f921c8 mov w25, #0x7fffffff
cpu.x[25]=(((0x7fffffffu64))) as u32 as u64;
// 0x10f921cc b #0x10f921ec
pc=0x10f921ec;continue;
}
0x10f921d0=>{
// 0x10f921d0 mov x22, x23
cpu.x[22]=(((cpu.x[23]))) as u64;
// 0x10f921d4 mov w25, w0
cpu.x[25]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
pc=0x10f921d8;continue;
}
0x10f921d8=>{
// 0x10f921d8 ldur x9, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f921dc add x24, x24, #1
{let a=((cpu.x[24]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[24]=(result) as u64;}
// 0x10f921e0 ldrsw x8, [x9, #0x14]
{let address=(cpu.x[9]).wrapping_add(0x14u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10f921e4 cmp x24, x8
{let a=((cpu.x[24]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f921e8 b.ge #0x10f92394
if cpu.condition(10){pc=0x10f92394;continue;}pc=0x10f921ec;continue;
}
0x10f921ec=>{
// 0x10f921ec ldr x8, [x9, #8]
{let address=(cpu.x[9]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f921f0 ldr x23, [x8, x24, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[24])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10f921f4 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f921f8 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10f921fc ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f92200 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f92204;super::dispatch(cpu,rt,target)?;}
// 0x10f92204 cmp w0, #5
{let a=((cpu.x[0] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f92208 b.ne #0x10f921d8
if cpu.condition(1){pc=0x10f921d8;continue;}pc=0x10f9220c;continue;
}
0x10f9220c=>{
// 0x10f9220c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f92210 mov x1, x23
cpu.x[1]=(((cpu.x[23]))) as u64;
// 0x10f92214 mov w2, wzr
cpu.x[2]=(((0u64))) as u32 as u64;
// 0x10f92218 mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10f9221c mov w4, wzr
cpu.x[4]=(((0u64))) as u32 as u64;
// 0x10f92220 mov w5, #1
cpu.x[5]=(((0x1u64))) as u32 as u64;
// 0x10f92224 bl #0x10f94824
cpu.x[30]=0x10f92228;super::chunk5::f_10f94824(cpu,rt,0x10f94824)?;
// 0x10f92228 tbz w0, #0, #0x10f921d8
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f921d8;continue;}pc=0x10f9222c;continue;
}
0x10f9222c=>{
// 0x10f9222c mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10f92230 bl #0x10f942c0
cpu.x[30]=0x10f92234;super::chunk2::f_10f942c0(cpu,rt,0x10f942c0)?;
// 0x10f92234 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10f92238 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f9223c bl #0x10f942c0
cpu.x[30]=0x10f92240;super::chunk2::f_10f942c0(cpu,rt,0x10f942c0)?;
// 0x10f92240 mov x1, x0
cpu.x[1]=(((cpu.x[0]))) as u64;
// 0x10f92244 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f92248 bl #0x11294e10
cpu.x[30]=0x10f9224c;super::chunk14::f_11294e10(cpu,rt,0x11294e10)?;
// 0x10f9224c ldr w8, [x20, #0x1b0]
{let address=(cpu.x[20]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f92250 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f92254 b.ge #0x10f921d8
if cpu.condition(10){pc=0x10f921d8;continue;}pc=0x10f92258;continue;
}
0x10f92258=>{
// 0x10f92258 cmp w0, w25
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[25] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9225c b.ge #0x10f921d8
if cpu.condition(10){pc=0x10f921d8;continue;}pc=0x10f92260;continue;
}
0x10f92260=>{
// 0x10f92260 ldr x9, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f92264 ldr w8, [x23, #8]
{let address=(cpu.x[23]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f92268 ldr w10, [x19, #0x6c]
{let address=(cpu.x[19]).wrapping_add(0x6cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10f9226c sub x9, x9, #4
{let a=((cpu.x[9]));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u64;}
pc=0x10f92270;continue;
}
0x10f92270=>{
// 0x10f92270 cmp w10, #1
{let a=((cpu.x[10] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f92274 b.lt #0x10f921d0
if cpu.condition(11){pc=0x10f921d0;continue;}pc=0x10f92278;continue;
}
0x10f92278=>{
// 0x10f92278 ldr w11, [x9, x10, lsl #2]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[10])) as u64).wrapping_shl(2) as u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
// 0x10f9227c sub x10, x10, #1
{let a=((cpu.x[10]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[10]=(result) as u64;}
// 0x10f92280 cmp w11, w8
{let a=((cpu.x[11] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f92284 b.ne #0x10f92270
if cpu.condition(1){pc=0x10f92270;continue;}pc=0x10f92288;continue;
}
0x10f92288=>{
// 0x10f92288 b #0x10f921d8
pc=0x10f921d8;continue;
}
0x10f9228c=>{
// 0x10f9228c mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10f92290 ldr x9, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f92294 strb w8, [x19, #0x170]
{let address=(cpu.x[19]).wrapping_add(0x170u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f92298 ldr x2, [x9, #0x2b8]
{let address=(cpu.x[9]).wrapping_add(0x2b8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10f9229c cbz x2, #0x10f922ac
if (cpu.x[2]) == 0 {pc=0x10f922ac;continue;}pc=0x10f922a0;continue;
}
0x10f922a0=>{
// 0x10f922a0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f922a4 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f922a8 bl #0x10f89954
cpu.x[30]=0x10f922ac;super::chunk11::f_10f89954(cpu,rt,0x10f89954)?;
pc=0x10f922ac;continue;
}
0x10f922ac=>{
// 0x10f922ac mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f922b0 bl #0x10f94058
cpu.x[30]=0x10f922b4;super::chunk6::f_10f94058(cpu,rt,0x10f94058)?;
// 0x10f922b4 tbnz w0, #0, #0x10f92454
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10f92454;continue;}pc=0x10f922b8;continue;
}
0x10f922b8=>{
// 0x10f922b8 ldrb w8, [x19, #0x1dc]
{let address=(cpu.x[19]).wrapping_add(0x1dcu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f922bc cbz w8, #0x10f92454
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f92454;continue;}pc=0x10f922c0;continue;
}
0x10f922c0=>{
// 0x10f922c0 ldr x1, [x19, #0x108]
{let address=(cpu.x[19]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10f922c4 cbz x1, #0x10f92454
if (cpu.x[1]) == 0 {pc=0x10f92454;continue;}pc=0x10f922c8;continue;
}
0x10f922c8=>{
// 0x10f922c8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f922cc bl #0x10f8ef58
cpu.x[30]=0x10f922d0;super::chunk6::f_10f8ef58(cpu,rt,0x10f8ef58)?;
// 0x10f922d0 ldr x8, [x19, #0x108]
{let address=(cpu.x[19]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f922d4 neg w1, w0
cpu.x[1]=(0u64.wrapping_sub(((cpu.x[0] as u32 as u64)))) as u32 as u64;
// 0x10f922d8 mov w2, #-1
cpu.x[2]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10f922dc ldr x9, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f922e0 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10f922e4 ldr x9, [x9, #0x1b8]
{let address=(cpu.x[9]).wrapping_add(0x1b8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f922e8 blr x9
{let target=cpu.x[9];cpu.x[30]=0x10f922ec;super::dispatch(cpu,rt,target)?;}
// 0x10f922ec strb wzr, [x19, #0x1dc]
{let address=(cpu.x[19]).wrapping_add(0x1dcu64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f922f0 b #0x10f92454
pc=0x10f92454;continue;
}
0x10f922f4=>{
// 0x10f922f4 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f922f8 bl #0x10f89954
cpu.x[30]=0x10f922fc;super::chunk11::f_10f89954(cpu,rt,0x10f89954)?;
// 0x10f922fc ldr w8, [x19, #0x214]
{let address=(cpu.x[19]).wrapping_add(0x214u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f92300 ldr w9, [x19, #0x210]
{let address=(cpu.x[19]).wrapping_add(0x210u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f92304 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f92308 b.ne #0x10f92330
if cpu.condition(1){pc=0x10f92330;continue;}pc=0x10f9230c;continue;
}
0x10f9230c=>{
// 0x10f9230c lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10f92310 mov w10, #5
cpu.x[10]=(((0x5u64))) as u32 as u64;
// 0x10f92314 cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f92318 csel w1, w10, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[10] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10f9231c cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f92320 b.ge #0x10f92330
if cpu.condition(10){pc=0x10f92330;continue;}pc=0x10f92324;continue;
}
0x10f92324=>{
// 0x10f92324 add x0, x19, #0x208
{let a=((cpu.x[19]));let b=((0x208u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10f92328 bl #0x1078f4d4
cpu.x[30]=0x10f9232c;super::chunk2::f_1078f4d4(cpu,rt,0x1078f4d4)?;
// 0x10f9232c ldr w8, [x19, #0x214]
{let address=(cpu.x[19]).wrapping_add(0x214u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
pc=0x10f92330;continue;
}
0x10f92330=>{
// 0x10f92330 ldr x9, [x19, #0x208]
{let address=(cpu.x[19]).wrapping_add(0x208u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f92334 add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10f92338 str w10, [x19, #0x214]
{let address=(cpu.x[19]).wrapping_add(0x214u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f9233c str w21, [x9, w8, sxtw #2]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=cpu.x[21] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f92340 ldrb w22, [x20, #0x231]
{let address=(cpu.x[20]).wrapping_add(0x231u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[22]=(v0) as u32 as u64;}
// 0x10f92344 cmp w26, #1
{let a=((cpu.x[26] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f92348 b.ge #0x10f91ae0
if cpu.condition(10){pc=0x10f91ae0;continue;}pc=0x10f9234c;continue;
}
0x10f9234c=>{
// 0x10f9234c ldr x8, [x19, #0x108]
{let address=(cpu.x[19]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f92350 ldur x25, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[25]=(v0) as u64;}
// 0x10f92354 cbz x8, #0x10f91b64
if (cpu.x[8]) == 0 {pc=0x10f91b64;continue;}pc=0x10f92358;continue;
}
0x10f92358=>{
// 0x10f92358 ldr w8, [x20, #0x1f4]
{let address=(cpu.x[20]).wrapping_add(0x1f4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f9235c ldr x21, [x19, #0x190]
{let address=(cpu.x[19]).wrapping_add(0x190u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f92360 cbz w8, #0x10f92478
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f92478;continue;}pc=0x10f92364;continue;
}
0x10f92364=>{
// 0x10f92364 add x0, x19, #0x12c
{let a=((cpu.x[19]));let b=((0x12cu64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10f92368 add x1, x19, #0x120
{let a=((cpu.x[19]));let b=((0x120u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x10f9236c bl #0x11294e10
cpu.x[30]=0x10f92370;super::chunk14::f_11294e10(cpu,rt,0x11294e10)?;
// 0x10f92370 ldr w8, [x20, #0x1f8]
{let address=(cpu.x[20]).wrapping_add(0x1f8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f92374 ldur w26, [x29, #-0x34]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffccu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[26]=(v0) as u32 as u64;}
// 0x10f92378 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9237c b.ge #0x10f9247c
if cpu.condition(10){pc=0x10f9247c;continue;}pc=0x10f92380;continue;
}
0x10f92380=>{
// 0x10f92380 ldr x8, [x20, #0xb0]
{let address=(cpu.x[20]).wrapping_add(0xb0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f92384 ldr w23, [x20, #0x1f4]
{let address=(cpu.x[20]).wrapping_add(0x1f4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[23]=(v0) as u32 as u64;}
// 0x10f92388 cmp x8, #0
{let a=((cpu.x[8]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f9238c csel x21, x21, x8, eq
cpu.x[21]=(if cpu.condition(0){((cpu.x[21]))}else{((cpu.x[8]))}) as u64;
// 0x10f92390 b #0x10f9247c
pc=0x10f9247c;continue;
}
0x10f92394=>{
// 0x10f92394 cbz x22, #0x10f9243c
if (cpu.x[22]) == 0 {pc=0x10f9243c;continue;}pc=0x10f92398;continue;
}
0x10f92398=>{
// 0x10f92398 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f9239c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f923a0 ldr x8, [x8, #0x150]
{let address=(cpu.x[8]).wrapping_add(0x150u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f923a4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f923a8;super::dispatch(cpu,rt,target)?;}
// 0x10f923a8 ldr w8, [x19, #0x120]
{let address=(cpu.x[19]).wrapping_add(0x120u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f923ac ldr w9, [x19, #0x124]
{let address=(cpu.x[19]).wrapping_add(0x124u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f923b0 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10f923b4 ldr w10, [x19, #0x12c]
{let address=(cpu.x[19]).wrapping_add(0x12cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10f923b8 ldr w11, [x19, #0x130]
{let address=(cpu.x[19]).wrapping_add(0x130u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
// 0x10f923bc mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f923c0 lsl w8, w8, #1
cpu.x[8]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10f923c4 lsl w9, w9, #1
cpu.x[9]=(((((cpu.x[9] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10f923c8 ldr w21, [x19, #0x128]
{let address=(cpu.x[19]).wrapping_add(0x128u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[21]=(v0) as u32 as u64;}
// 0x10f923cc ldr x26, [x19, #0x100]
{let address=(cpu.x[19]).wrapping_add(0x100u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[26]=(v0) as u64;}
// 0x10f923d0 str xzr, [x19, #0x108]
{let address=(cpu.x[19]).wrapping_add(0x108u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f923d4 sub w23, w8, w10
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[23]=(result) as u32 as u64;}
// 0x10f923d8 sub w24, w9, w11
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[11] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[24]=(result) as u32 as u64;}
// 0x10f923dc bl #0x10f94298
cpu.x[30]=0x10f923e0;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
// 0x10f923e0 mov w25, w0
cpu.x[25]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f923e4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f923e8 bl #0x10f942a0
cpu.x[30]=0x10f923ec;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
// 0x10f923ec cmp x26, #0
{let a=((cpu.x[26]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f923f0 ldr w8, [x19, #0x198]
{let address=(cpu.x[19]).wrapping_add(0x198u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f923f4 mov w3, w0
cpu.x[3]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f923f8 csel x9, x19, x26, eq
cpu.x[9]=(if cpu.condition(0){((cpu.x[19]))}else{((cpu.x[26]))}) as u64;
// 0x10f923fc mov x1, xzr
cpu.x[1]=(((0u64))) as u64;
// 0x10f92400 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f92404 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10f92408 mov w4, w21
cpu.x[4]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10f9240c mov x5, x22
cpu.x[5]=(((cpu.x[22]))) as u64;
// 0x10f92410 mov w6, w23
cpu.x[6]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10f92414 mov w7, w24
cpu.x[7]=(((cpu.x[24] as u32 as u64))) as u32 as u64;
// 0x10f92418 str wzr, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f9241c str wzr, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f92420 stp x20, x8, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=cpu.x[20];let v1=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f92424 str x9, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=cpu.x[9];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f92428 bl #0x10f8e6fc
cpu.x[30]=0x10f9242c;super::chunk5::f_10f8e6fc(cpu,rt,0x10f8e6fc)?;
// 0x10f9242c mov w8, #0x96
cpu.x[8]=(((0x96u64))) as u32 as u64;
// 0x10f92430 strb wzr, [x19, #0x170]
{let address=(cpu.x[19]).wrapping_add(0x170u64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f92434 str w8, [x19, #0x1b0]
{let address=(cpu.x[19]).wrapping_add(0x1b0u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f92438 b #0x10f92454
pc=0x10f92454;continue;
}
0x10f9243c=>{
// 0x10f9243c ldr x1, [x20, #0x140]
{let address=(cpu.x[20]).wrapping_add(0x140u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10f92440 cbz x1, #0x10f92454
if (cpu.x[1]) == 0 {pc=0x10f92454;continue;}pc=0x10f92444;continue;
}
0x10f92444=>{
// 0x10f92444 ldr x0, [x19, #0x58]
{let address=(cpu.x[19]).wrapping_add(0x58u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f92448 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f9244c ldr x8, [x8, #0x80]
{let address=(cpu.x[8]).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f92450 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f92454;super::dispatch(cpu,rt,target)?;}
pc=0x10f92454;continue;
}
0x10f92454=>{
// 0x10f92454 ldp x20, x19, [sp, #0xd0]
{let address=(cpu.sp).wrapping_add(0xd0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f92458 ldr d8, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.q[8]=(v0) as u128 & 0xffffffffffffffffu128;}
// 0x10f9245c ldp x22, x21, [sp, #0xc0]
{let address=(cpu.sp).wrapping_add(0xc0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f92460 ldp x24, x23, [sp, #0xb0]
{let address=(cpu.sp).wrapping_add(0xb0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10f92464 ldp x26, x25, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10f92468 ldp x28, x27, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10f9246c ldp x29, x30, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10f92470 add sp, sp, #0xe0
{let a=((cpu.sp));let b=((0xe0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10f92474 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f92478=>{
// 0x10f92478 ldur w26, [x29, #-0x34]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffccu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[26]=(v0) as u32 as u64;}
pc=0x10f9247c;continue;
}
0x10f9247c=>{
// 0x10f9247c ldr x1, [x19, #0x108]
{let address=(cpu.x[19]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10f92480 cbz w22, #0x10f92494
if (cpu.x[22] as u32 as u64) == 0 {pc=0x10f92494;continue;}pc=0x10f92484;continue;
}
0x10f92484=>{
// 0x10f92484 ldr x2, [sp, #0x38]
{let address=(cpu.sp).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10f92488 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f9248c bl #0x10f8f890
cpu.x[30]=0x10f92490;super::chunk3::f_10f8f890(cpu,rt,0x10f8f890)?;
// 0x10f92490 ldr x1, [x19, #0x108]
{let address=(cpu.x[19]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
pc=0x10f92494;continue;
}
0x10f92494=>{
// 0x10f92494 ldr w8, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f92498 ldr w2, [x19, #0x120]
{let address=(cpu.x[19]).wrapping_add(0x120u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[2]=(v0) as u32 as u64;}
// 0x10f9249c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f924a0 ldr w3, [x19, #0x124]
{let address=(cpu.x[19]).wrapping_add(0x124u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10f924a4 mov w4, w23
cpu.x[4]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10f924a8 mov w5, w28
cpu.x[5]=(((cpu.x[28] as u32 as u64))) as u32 as u64;
// 0x10f924ac and w7, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[7]=(result) as u32 as u64;}
// 0x10f924b0 mov w6, w24
cpu.x[6]=(((cpu.x[24] as u32 as u64))) as u32 as u64;
// 0x10f924b4 str x21, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f924b8 bl #0x10f8f618
cpu.x[30]=0x10f924bc;super::chunk5::f_10f8f618(cpu,rt,0x10f8f618)?;
pc=0x10f924bc;continue;
}
0x10f924bc=>{
// 0x10f924bc cmp w26, #1
{let a=((cpu.x[26] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f924c0 b.lt #0x10f9254c
if cpu.condition(11){pc=0x10f9254c;continue;}pc=0x10f924c4;continue;
}
0x10f924c4=>{
// 0x10f924c4 ldr x0, [x19, #0x108]
{let address=(cpu.x[19]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f924c8 bl #0x10f942f0
cpu.x[30]=0x10f924cc;super::chunk11::f_10f942f0(cpu,rt,0x10f942f0)?;
// 0x10f924cc cbz x0, #0x10f9254c
if (cpu.x[0]) == 0 {pc=0x10f9254c;continue;}pc=0x10f924d0;continue;
}
0x10f924d0=>{
// 0x10f924d0 ldr x0, [x19, #0x108]
{let address=(cpu.x[19]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f924d4 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10f924d8 mov w2, #1
cpu.x[2]=(((0x1u64))) as u32 as u64;
// 0x10f924dc bl #0x10f89258
cpu.x[30]=0x10f924e0;super::chunk5::f_10f89258(cpu,rt,0x10f89258)?;
// 0x10f924e0 ldr x0, [x19, #0x108]
{let address=(cpu.x[19]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f924e4 ldrb w21, [x20, #0x2d8]
{let address=(cpu.x[20]).wrapping_add(0x2d8u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[21]=(v0) as u32 as u64;}
// 0x10f924e8 bl #0x10f942f0
cpu.x[30]=0x10f924ec;super::chunk11::f_10f942f0(cpu,rt,0x10f942f0)?;
// 0x10f924ec mov x24, x0
cpu.x[24]=(((cpu.x[0]))) as u64;
// 0x10f924f0 cbz w21, #0x10f925f4
if (cpu.x[21] as u32 as u64) == 0 {pc=0x10f925f4;continue;}pc=0x10f924f4;continue;
}
0x10f924f4=>{
// 0x10f924f4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f924f8 bl #0x10f94298
cpu.x[30]=0x10f924fc;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
// 0x10f924fc ldr w8, [x19, #0x12c]
{let address=(cpu.x[19]).wrapping_add(0x12cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f92500 sub w21, w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[21]=(result) as u32 as u64;}
// 0x10f92504 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f92508 bl #0x10f942a0
cpu.x[30]=0x10f9250c;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
// 0x10f9250c ldr w8, [x19, #0x130]
{let address=(cpu.x[19]).wrapping_add(0x130u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f92510 mov w22, w23
cpu.x[22]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10f92514 sub w23, w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[23]=(result) as u32 as u64;}
// 0x10f92518 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f9251c bl #0x10e5aeb8
cpu.x[30]=0x10f92520;super::chunk4::f_10e5aeb8(cpu,rt,0x10e5aeb8)?;
// 0x10f92520 and w5, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[5]=(result) as u32 as u64;}
// 0x10f92524 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10f92528 mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10f9252c mov w2, w23
cpu.x[2]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10f92530 mov w3, w26
cpu.x[3]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10f92534 mov x4, x19
cpu.x[4]=(((cpu.x[19]))) as u64;
// 0x10f92538 mov w6, wzr
cpu.x[6]=(((0u64))) as u32 as u64;
// 0x10f9253c mov w7, wzr
cpu.x[7]=(((0u64))) as u32 as u64;
// 0x10f92540 mov w23, w22
cpu.x[23]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10f92544 strb wzr, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f92548 bl #0x10fcfe18
cpu.x[30]=0x10f9254c;super::dispatch(cpu,rt,0x10fcfe18)?;
pc=0x10f9254c;continue;
}
0x10f9254c=>{
// 0x10f9254c ldr x8, [x19, #0x100]
{let address=(cpu.x[19]).wrapping_add(0x100u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f92550 cbz x8, #0x10f91b64
if (cpu.x[8]) == 0 {pc=0x10f91b64;continue;}pc=0x10f92554;continue;
}
0x10f92554=>{
// 0x10f92554 ldr x9, [x19, #0x108]
{let address=(cpu.x[19]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f92558 cmp x9, x8
{let a=((cpu.x[9]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f9255c b.ne #0x10f91b64
if cpu.condition(1){pc=0x10f91b64;continue;}pc=0x10f92560;continue;
}
0x10f92560=>{
// 0x10f92560 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f92564 mov x1, x27
cpu.x[1]=(((cpu.x[27]))) as u64;
// 0x10f92568 bl #0x10e5ad94
cpu.x[30]=0x10f9256c;super::chunk14::f_10e5ad94(cpu,rt,0x10e5ad94)?;
// 0x10f9256c cbz w0, #0x10f91b64
if (cpu.x[0] as u32 as u64) == 0 {pc=0x10f91b64;continue;}pc=0x10f92570;continue;
}
0x10f92570=>{
// 0x10f92570 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f92574 mov x1, x27
cpu.x[1]=(((cpu.x[27]))) as u64;
// 0x10f92578 bl #0x10e5ad94
cpu.x[30]=0x10f9257c;super::chunk14::f_10e5ad94(cpu,rt,0x10e5ad94)?;
// 0x10f9257c ldr x8, [x19, #0x100]
{let address=(cpu.x[19]).wrapping_add(0x100u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f92580 mul w21, w0, w23
cpu.x[21]=((((cpu.x[0] as u32 as u64))).wrapping_mul(((cpu.x[23] as u32 as u64)))) as u32 as u64;
// 0x10f92584 ldr x9, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f92588 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10f9258c ldr x9, [x9, #0x120]
{let address=(cpu.x[9]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f92590 blr x9
{let target=cpu.x[9];cpu.x[30]=0x10f92594;super::dispatch(cpu,rt,target)?;}
// 0x10f92594 mov x23, x0
cpu.x[23]=(((cpu.x[0]))) as u64;
// 0x10f92598 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f9259c bl #0x10f95b64
cpu.x[30]=0x10f925a0;super::chunk0::f_10f95b64(cpu,rt,0x10f95b64)?;
// 0x10f925a0 mov x24, x0
cpu.x[24]=(((cpu.x[0]))) as u64;
// 0x10f925a4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f925a8 bl #0x10f95bec
cpu.x[30]=0x10f925ac;super::chunk6::f_10f95bec(cpu,rt,0x10f95bec)?;
// 0x10f925ac mov w25, w0
cpu.x[25]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f925b0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f925b4 bl #0x10f95c08
cpu.x[30]=0x10f925b8;super::chunk13::f_10f95c08(cpu,rt,0x10f95c08)?;
// 0x10f925b8 ldrb w6, [x19, #0xa4]
{let address=(cpu.x[19]).wrapping_add(0xa4u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[6]=(v0) as u32 as u64;}
// 0x10f925bc mov w5, w0
cpu.x[5]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f925c0 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10f925c4 mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10f925c8 mov w2, wzr
cpu.x[2]=(((0u64))) as u32 as u64;
// 0x10f925cc mov x3, x24
cpu.x[3]=(((cpu.x[24]))) as u64;
// 0x10f925d0 mov w4, w25
cpu.x[4]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10f925d4 ldur x25, [x29, #-0x20]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[25]=(v0) as u64;}
// 0x10f925d8 bl #0x10fcbde8
cpu.x[30]=0x10f925dc;super::dispatch(cpu,rt,0x10fcbde8)?;
// 0x10f925dc ldr x2, [x19, #0x100]
{let address=(cpu.x[19]).wrapping_add(0x100u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10f925e0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f925e4 mov w1, #6
cpu.x[1]=(((0x6u64))) as u32 as u64;
// 0x10f925e8 mov w3, w21
cpu.x[3]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10f925ec bl #0x10f94078
cpu.x[30]=0x10f925f0;super::chunk8::f_10f94078(cpu,rt,0x10f94078)?;
// 0x10f925f0 b #0x10f91b64
pc=0x10f91b64;continue;
}
0x10f925f4=>{
// 0x10f925f4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f925f8 bl #0x10e5aeb8
cpu.x[30]=0x10f925fc;super::chunk4::f_10e5aeb8(cpu,rt,0x10e5aeb8)?;
// 0x10f925fc ldur w1, [x29, #-0x24]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffdcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f92600 ldur w2, [x29, #-0x30]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffd0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[2]=(v0) as u32 as u64;}
// 0x10f92604 mov w6, wzr
cpu.x[6]=(((0u64))) as u32 as u64;
// 0x10f92608 mov w7, wzr
cpu.x[7]=(((0u64))) as u32 as u64;
// 0x10f9260c and w5, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[5]=(result) as u32 as u64;}
// 0x10f92610 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10f92614 mov w3, w26
cpu.x[3]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10f92618 mov x4, x19
cpu.x[4]=(((cpu.x[19]))) as u64;
// 0x10f9261c strb wzr, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f92620 strb wzr, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f92624 bl #0x10fcfac4
cpu.x[30]=0x10f92628;super::chunk6::f_10fcfac4(cpu,rt,0x10fcfac4)?;
// 0x10f92628 ldr x8, [x19, #0x100]
{let address=(cpu.x[19]).wrapping_add(0x100u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f9262c cbnz x8, #0x10f92554
if (cpu.x[8]) != 0 {pc=0x10f92554;continue;}pc=0x10f92630;continue;
}
0x10f92630=>{
// 0x10f92630 b #0x10f91b64
pc=0x10f91b64;continue;
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f9188c")),}}}
#[inline]
pub(super) unsafe fn f_10f927ec(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f927ec=>{
// 0x10f927ec stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f927f0 stp x24, x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f927f4 stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f927f8 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f927fc mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
pc=0x10f92800;continue;
}
0x10f92800=>{
// 0x10f92800 ldr x8, [x0, #0x108]
{let address=(cpu.x[0]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f92804 mov w20, w2
cpu.x[20]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x10f92808 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f9280c mov w21, w1
cpu.x[21]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x10f92810 cbz x8, #0x10f92838
if (cpu.x[8]) == 0 {pc=0x10f92838;continue;}pc=0x10f92814;continue;
}
0x10f92814=>{
// 0x10f92814 ldr x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f92818 bl #0x10e5acdc
cpu.x[30]=0x10f9281c;super::chunk13::f_10e5acdc(cpu,rt,0x10e5acdc)?;
pc=0x10f9281c;continue;
}
0x10f9281c=>{
// 0x10f9281c tbz w0, #0, #0x10f92838
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f92838;continue;}pc=0x10f92820;continue;
}
0x10f92820=>{
// 0x10f92820 ldr x0, [x19, #0x108]
{let address=(cpu.x[19]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f92824 bl #0x10f94298
cpu.x[30]=0x10f92828;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
pc=0x10f92828;continue;
}
0x10f92828=>{
// 0x10f92828 mov w22, w0
cpu.x[22]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f9282c ldr x8, [x19, #0x108]
{let address=(cpu.x[19]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f92830 cbnz x8, #0x10f92844
if (cpu.x[8]) != 0 {pc=0x10f92844;continue;}pc=0x10f92834;continue;
}
0x10f92834=>{
// 0x10f92834 b #0x10f92860
pc=0x10f92860;continue;
}
0x10f92838=>{
// 0x10f92838 ldr w22, [x19, #0x120]
{let address=(cpu.x[19]).wrapping_add(0x120u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[22]=(v0) as u32 as u64;}
// 0x10f9283c ldr x8, [x19, #0x108]
{let address=(cpu.x[19]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f92840 cbz x8, #0x10f92860
if (cpu.x[8]) == 0 {pc=0x10f92860;continue;}pc=0x10f92844;continue;
}
0x10f92844=>{
// 0x10f92844 ldr x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f92848 bl #0x10e5acdc
cpu.x[30]=0x10f9284c;super::chunk13::f_10e5acdc(cpu,rt,0x10e5acdc)?;
pc=0x10f9284c;continue;
}
0x10f9284c=>{
// 0x10f9284c tbz w0, #0, #0x10f92860
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f92860;continue;}pc=0x10f92850;continue;
}
0x10f92850=>{
// 0x10f92850 ldr x0, [x19, #0x108]
{let address=(cpu.x[19]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f92854 bl #0x10f942a0
cpu.x[30]=0x10f92858;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
pc=0x10f92858;continue;
}
0x10f92858=>{
// 0x10f92858 mov w23, w0
cpu.x[23]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f9285c b #0x10f92864
pc=0x10f92864;continue;
}
0x10f92860=>{
// 0x10f92860 ldr w23, [x19, #0x124]
{let address=(cpu.x[19]).wrapping_add(0x124u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[23]=(v0) as u32 as u64;}
pc=0x10f92864;continue;
}
0x10f92864=>{
// 0x10f92864 ldr x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f92868 bl #0x10e5ac8c
cpu.x[30]=0x10f9286c;super::chunk11::f_10e5ac8c(cpu,rt,0x10e5ac8c)?;
pc=0x10f9286c;continue;
}
0x10f9286c=>{
// 0x10f9286c cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f92870 mov w1, w22
cpu.x[1]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10f92874 mov w2, w23
cpu.x[2]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10f92878 csinc w24, w0, wzr, gt
cpu.x[24]=(if cpu.condition(12){((cpu.x[0] as u32 as u64))}else{(((0u64))).wrapping_add(1)}) as u32 as u64;
// 0x10f9287c add x0, x19, #0x12c
{let a=((cpu.x[19]));let b=((0x12cu64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10f92880 bl #0x11294ed8
cpu.x[30]=0x10f92884;super::chunk15::f_11294ed8(cpu,rt,0x11294ed8)?;
pc=0x10f92884;continue;
}
0x10f92884=>{
// 0x10f92884 sdiv w8, w0, w24
{let a=((cpu.x[0] as u32 as u64)) as i32;let b=((cpu.x[24] as u32 as u64)) as i32;cpu.x[8]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u32 as u64;}
// 0x10f92888 add x0, x19, #0x12c
{let a=((cpu.x[19]));let b=((0x12cu64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10f9288c mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10f92890 mov w2, w20
cpu.x[2]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x10f92894 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f92898 csinc w22, w8, wzr, gt
cpu.x[22]=(if cpu.condition(12){((cpu.x[8] as u32 as u64))}else{(((0u64))).wrapping_add(1)}) as u32 as u64;
// 0x10f9289c bl #0x11294ed8
cpu.x[30]=0x10f928a0;super::chunk15::f_11294ed8(cpu,rt,0x11294ed8)?;
pc=0x10f928a0;continue;
}
0x10f928a0=>{
// 0x10f928a0 sdiv w8, w0, w24
{let a=((cpu.x[0] as u32 as u64)) as i32;let b=((cpu.x[24] as u32 as u64)) as i32;cpu.x[8]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u32 as u64;}
// 0x10f928a4 ldr w9, [x19, #0x128]
{let address=(cpu.x[19]).wrapping_add(0x128u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f928a8 ldr w10, [x19, #0x134]
{let address=(cpu.x[19]).wrapping_add(0x134u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10f928ac ldr x11, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[11]=(v0) as u64;}
// 0x10f928b0 mov w12, #-0x1f4
cpu.x[12]=(((0xfffffffffffffe0cu64))) as u32 as u64;
// 0x10f928b4 sub w9, w9, w10
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10f928b8 ldr w11, [x11, #0x188]
{let address=(cpu.x[11]).wrapping_add(0x188u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
// 0x10f928bc mul w11, w11, w12
cpu.x[11]=((((cpu.x[11] as u32 as u64))).wrapping_mul(((cpu.x[12] as u32 as u64)))) as u32 as u64;
// 0x10f928c0 mov w12, #0x4dd3
cpu.x[12]=(((0x4dd3u64))) as u32 as u64;
// 0x10f928c4 movk w12, #0x1062, lsl #16
cpu.x[12]=((cpu.x[12] as u32 as u64 & !(0xffff0000u64)) | 0x10620000u64) as u32 as u64;
// 0x10f928c8 mul w9, w9, w8
cpu.x[9]=((((cpu.x[9] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64)))) as u32 as u64;
// 0x10f928cc smull x11, w11, w12
cpu.x[11]=(((((cpu.x[11] as u32 as u64))) as i32 as i64).wrapping_mul((((cpu.x[12] as u32 as u64))) as i32 as i64) as u64) as u64;
// 0x10f928d0 sub w12, w8, w22
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[22] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[12]=(result) as u32 as u64;}
// 0x10f928d4 mul w8, w12, w8
cpu.x[8]=((((cpu.x[12] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64)))) as u32 as u64;
// 0x10f928d8 sdiv w9, w9, w22
{let a=((cpu.x[9] as u32 as u64)) as i32;let b=((cpu.x[22] as u32 as u64)) as i32;cpu.x[9]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u32 as u64;}
// 0x10f928dc lsr x13, x11, #0x3f
cpu.x[13]=(((((cpu.x[11]))) as u64).wrapping_shr(((((0x3fu64))) & 63) as u32) as u64) as u64;
// 0x10f928e0 asr x11, x11, #0x26
cpu.x[11]=(((((cpu.x[11]))) as i64).wrapping_shr(((((0x26u64))) & 63) as u32) as u64) as u64;
// 0x10f928e4 add w11, w11, w13
{let a=((cpu.x[11] as u32 as u64));let b=((cpu.x[13] as u32 as u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u32 as u64;}
// 0x10f928e8 add w9, w9, w10
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10f928ec madd w0, w8, w11, w9
cpu.x[0]=((((cpu.x[9] as u32 as u64))).wrapping_add((((cpu.x[8] as u32 as u64))).wrapping_mul(((cpu.x[11] as u32 as u64))))) as u32 as u64;
// 0x10f928f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f928f4 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f928f8 ldp x24, x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10f928fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10f92900 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f927ec")),}}}
#[inline]
pub(super) unsafe fn f_10f92988(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f92988=>{
// 0x10f92988 stp x29, x30, [sp, #-0x10]!
{let address=(cpu.sp).wrapping_add(0xfffffffffffffff0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f9298c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f92990 ldrb w8, [x0, #0x14c]
{let address=(cpu.x[0]).wrapping_add(0x14cu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f92994 cbz w8, #0x10f929c8
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f929c8;continue;}pc=0x10f92998;continue;
}
0x10f92998=>{
// 0x10f92998 mov x9, x0
cpu.x[9]=(((cpu.x[0]))) as u64;
pc=0x10f9299c;continue;
}
0x10f9299c=>{
// 0x10f9299c mov x8, x9
cpu.x[8]=(((cpu.x[9]))) as u64;
// 0x10f929a0 ldr x9, [x9, #0x150]
{let address=(cpu.x[9]).wrapping_add(0x150u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f929a4 cbnz x9, #0x10f9299c
if (cpu.x[9]) != 0 {pc=0x10f9299c;continue;}pc=0x10f929a8;continue;
}
0x10f929a8=>{
// 0x10f929a8 ldr x8, [x8, #0x168]
{let address=(cpu.x[8]).wrapping_add(0x168u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f929ac cbz x8, #0x10f929c8
if (cpu.x[8]) == 0 {pc=0x10f929c8;continue;}pc=0x10f929b0;continue;
}
0x10f929b0=>{
// 0x10f929b0 mov w9, #1
cpu.x[9]=(((0x1u64))) as u32 as u64;
// 0x10f929b4 str w1, [x0, #0x160]
{let address=(cpu.x[0]).wrapping_add(0x160u64);let v0=cpu.x[1] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f929b8 str w2, [x0, #0x164]
{let address=(cpu.x[0]).wrapping_add(0x164u64);let v0=cpu.x[2] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f929bc strb w9, [x8, #0xd]
{let address=(cpu.x[8]).wrapping_add(0xdu64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f929c0 ldp x29, x30, [sp], #0x10
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x10u64)))) as u64;}
// 0x10f929c4 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f929c8=>{
// 0x10f929c8 adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x10f929cc add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10f929d0 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10f929d4 bl #0x1128df18
cpu.x[30]=0x10f929d8;super::dispatch(cpu,rt,0x1128df18)?;
return Err("fell through function 0x10f92988".into());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f92988")),}}}
#[inline]
pub(super) unsafe fn f_10f94560(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f94560=>{
// 0x10f94560 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f94564 str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f94568 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f9456c ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f94570 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f94574 ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f94578 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f9457c;super::dispatch(cpu,rt,target)?;}
pc=0x10f9457c;continue;
}
0x10f9457c=>{
// 0x10f9457c cbz x0, #0x10f9459c
if (cpu.x[0]) == 0 {pc=0x10f9459c;continue;}pc=0x10f94580;continue;
}
0x10f94580=>{
// 0x10f94580 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f94584 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f94588 ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f9458c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f94590;super::dispatch(cpu,rt,target)?;}
pc=0x10f94590;continue;
}
0x10f94590=>{
// 0x10f94590 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10f94594 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f94598 b #0x10fca23c
super::chunk13::f_10fca23c(cpu,rt,0x10fca23c)?;return Ok(());
}
0x10f9459c=>{
// 0x10f9459c mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
// 0x10f945a0 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10f945a4 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f945a8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f94560")),}}}
#[inline]
pub(super) unsafe fn f_10f94628(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f94628=>{
// 0x10f94628 ldrb w9, [x0, #0x30]
{let address=(cpu.x[0]).wrapping_add(0x30u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f9462c mov x8, x0
cpu.x[8]=(((cpu.x[0]))) as u64;
// 0x10f94630 mov x0, xzr
cpu.x[0]=(((0u64))) as u64;
// 0x10f94634 tbz w9, #2, #0x10f9464c
if ((cpu.x[9] as u32 as u64) & (1u64 << 2)) == 0 {pc=0x10f9464c;continue;}pc=0x10f94638;continue;
}
0x10f94638=>{
// 0x10f94638 ldr w9, [x8, #0x24]
{let address=(cpu.x[8]).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f9463c cmp w9, #3
{let a=((cpu.x[9] as u32 as u64));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f94640 b.lt #0x10f9464c
if cpu.condition(11){pc=0x10f9464c;continue;}pc=0x10f94644;continue;
}
0x10f94644=>{
// 0x10f94644 ldr x8, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f94648 ldr x0, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
pc=0x10f9464c;continue;
}
0x10f9464c=>{
// 0x10f9464c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f94628")),}}}
#[inline]
pub(super) unsafe fn f_10f985a4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f985a4=>{
// 0x10f985a4 sub sp, sp, #0x60
{let a=((cpu.sp));let b=((0x60u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x10f985a8 stp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f985ac stp x26, x25, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[26];let v1=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f985b0 stp x24, x23, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f985b4 stp x22, x21, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f985b8 stp x20, x19, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f985bc add x29, sp, #0x10
{let a=((cpu.sp));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x10f985c0 ldr x8, [x2]
{let address=(cpu.x[2]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f985c4 mov x0, x2
cpu.x[0]=(((cpu.x[2]))) as u64;
// 0x10f985c8 mov w20, w4
cpu.x[20]=(((cpu.x[4] as u32 as u64))) as u32 as u64;
// 0x10f985cc mov x21, x3
cpu.x[21]=(((cpu.x[3]))) as u64;
// 0x10f985d0 mov x19, x2
cpu.x[19]=(((cpu.x[2]))) as u64;
// 0x10f985d4 mov x22, x1
cpu.x[22]=(((cpu.x[1]))) as u64;
// 0x10f985d8 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f985dc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f985e0;super::dispatch(cpu,rt,target)?;}
pc=0x10f985e0;continue;
}
0x10f985e0=>{
// 0x10f985e0 cmp w0, #5
{let a=((cpu.x[0] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f985e4 b.ne #0x10f985f0
if cpu.condition(1){pc=0x10f985f0;continue;}pc=0x10f985e8;continue;
}
0x10f985e8=>{
// 0x10f985e8 ldr w24, [x19, #0x120]
{let address=(cpu.x[19]).wrapping_add(0x120u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[24]=(v0) as u32 as u64;}
// 0x10f985ec b #0x10f98634
pc=0x10f98634;continue;
}
0x10f985f0=>{
// 0x10f985f0 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f985f4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f985f8 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f985fc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f98600;super::dispatch(cpu,rt,target)?;}
pc=0x10f98600;continue;
}
0x10f98600=>{
// 0x10f98600 cmp w0, #4
{let a=((cpu.x[0] as u32 as u64));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f98604 b.ne #0x10f98610
if cpu.condition(1){pc=0x10f98610;continue;}pc=0x10f98608;continue;
}
0x10f98608=>{
// 0x10f98608 ldr w24, [x19, #0x198]
{let address=(cpu.x[19]).wrapping_add(0x198u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[24]=(v0) as u32 as u64;}
// 0x10f9860c b #0x10f98634
pc=0x10f98634;continue;
}
0x10f98610=>{
// 0x10f98610 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f98614 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f98618 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f9861c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f98620;super::dispatch(cpu,rt,target)?;}
pc=0x10f98620;continue;
}
0x10f98620=>{
// 0x10f98620 cmp w0, #3
{let a=((cpu.x[0] as u32 as u64));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f98624 b.ne #0x10f98630
if cpu.condition(1){pc=0x10f98630;continue;}pc=0x10f98628;continue;
}
0x10f98628=>{
// 0x10f98628 ldr w24, [x19, #0xfc]
{let address=(cpu.x[19]).wrapping_add(0xfcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[24]=(v0) as u32 as u64;}
// 0x10f9862c b #0x10f98634
pc=0x10f98634;continue;
}
0x10f98630=>{
// 0x10f98630 mov w24, wzr
cpu.x[24]=(((0u64))) as u32 as u64;
pc=0x10f98634;continue;
}
0x10f98634=>{
// 0x10f98634 ldrb w8, [x21, #0x129]
{let address=(cpu.x[21]).wrapping_add(0x129u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f98638 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f9863c cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f98640 csel x23, xzr, x19, eq
cpu.x[23]=(if cpu.condition(0){((0u64))}else{((cpu.x[19]))}) as u64;
// 0x10f98644 bl #0x10f940fc
cpu.x[30]=0x10f98648;super::chunk9::f_10f940fc(cpu,rt,0x10f940fc)?;
pc=0x10f98648;continue;
}
0x10f98648=>{
// 0x10f98648 mov x25, x0
cpu.x[25]=(((cpu.x[0]))) as u64;
// 0x10f9864c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f98650 bl #0x10f93d3c
cpu.x[30]=0x10f98654;super::chunk3::f_10f93d3c(cpu,rt,0x10f93d3c)?;
pc=0x10f98654;continue;
}
0x10f98654=>{
// 0x10f98654 mov w24, w24
cpu.x[24]=(((cpu.x[24] as u32 as u64))) as u32 as u64;
// 0x10f98658 mov w6, w0
cpu.x[6]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f9865c mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10f98660 mov x1, x23
cpu.x[1]=(((cpu.x[23]))) as u64;
// 0x10f98664 mov x2, x21
cpu.x[2]=(((cpu.x[21]))) as u64;
// 0x10f98668 mov w3, w20
cpu.x[3]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x10f9866c mov x4, x24
cpu.x[4]=(((cpu.x[24]))) as u64;
// 0x10f98670 mov x5, x19
cpu.x[5]=(((cpu.x[19]))) as u64;
// 0x10f98674 mov w7, wzr
cpu.x[7]=(((0u64))) as u32 as u64;
// 0x10f98678 str xzr, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f9867c bl #0x10fc0e08
cpu.x[30]=0x10f98680;super::chunk14::f_10fc0e08(cpu,rt,0x10fc0e08)?;
pc=0x10f98680;continue;
}
0x10f98680=>{
// 0x10f98680 ldr x8, [x22]
{let address=(cpu.x[22]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f98684 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f98688 ldr x8, [x8, #0x170]
{let address=(cpu.x[8]).wrapping_add(0x170u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f9868c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f98690;super::dispatch(cpu,rt,target)?;}
pc=0x10f98690;continue;
}
0x10f98690=>{
// 0x10f98690 tbz w0, #0, #0x10f98764
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f98764;continue;}pc=0x10f98694;continue;
}
0x10f98694=>{
// 0x10f98694 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f98698 bl #0x10f93d2c
cpu.x[30]=0x10f9869c;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
pc=0x10f9869c;continue;
}
0x10f9869c=>{
// 0x10f9869c bl #0x10d36e90
cpu.x[30]=0x10f986a0;super::chunk6::f_10d36e90(cpu,rt,0x10d36e90)?;
pc=0x10f986a0;continue;
}
0x10f986a0=>{
// 0x10f986a0 tbz w0, #0, #0x10f98764
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f98764;continue;}pc=0x10f986a4;continue;
}
0x10f986a4=>{
// 0x10f986a4 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f986a8 mov x1, xzr
cpu.x[1]=(((0u64))) as u64;
// 0x10f986ac bl #0x10ddaa80
cpu.x[30]=0x10f986b0;super::chunk14::f_10ddaa80(cpu,rt,0x10ddaa80)?;
// 0x10f986b0 mov w8, w0
cpu.x[8]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f986b4 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f986b8 cbnz w8, #0x10f9870c
if (cpu.x[8] as u32 as u64) != 0 {pc=0x10f9870c;continue;}pc=0x10f986bc;continue;
}
0x10f986bc=>{
// 0x10f986bc mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f986c0 bl #0x10f93d2c
cpu.x[30]=0x10f986c4;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
// 0x10f986c4 mov x1, x22
cpu.x[1]=(((cpu.x[22]))) as u64;
// 0x10f986c8 bl #0x10d3c2ac
cpu.x[30]=0x10f986cc;super::chunk1::f_10d3c2ac(cpu,rt,0x10d3c2ac)?;
// 0x10f986cc mov x25, x0
cpu.x[25]=(((cpu.x[0]))) as u64;
// 0x10f986d0 bl #0x10f940fc
cpu.x[30]=0x10f986d4;super::chunk9::f_10f940fc(cpu,rt,0x10f940fc)?;
// 0x10f986d4 mov x26, x0
cpu.x[26]=(((cpu.x[0]))) as u64;
// 0x10f986d8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f986dc bl #0x10f93d3c
cpu.x[30]=0x10f986e0;super::chunk3::f_10f93d3c(cpu,rt,0x10f93d3c)?;
// 0x10f986e0 mov w6, w0
cpu.x[6]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f986e4 mov x0, x26
cpu.x[0]=(((cpu.x[26]))) as u64;
// 0x10f986e8 mov x1, x23
cpu.x[1]=(((cpu.x[23]))) as u64;
// 0x10f986ec mov x2, x21
cpu.x[2]=(((cpu.x[21]))) as u64;
// 0x10f986f0 mov w3, w20
cpu.x[3]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x10f986f4 mov x4, x24
cpu.x[4]=(((cpu.x[24]))) as u64;
// 0x10f986f8 mov x5, x19
cpu.x[5]=(((cpu.x[19]))) as u64;
// 0x10f986fc mov w7, wzr
cpu.x[7]=(((0u64))) as u32 as u64;
// 0x10f98700 str xzr, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f98704 bl #0x10fc0e08
cpu.x[30]=0x10f98708;super::chunk14::f_10fc0e08(cpu,rt,0x10fc0e08)?;
// 0x10f98708 mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
pc=0x10f9870c;continue;
}
0x10f9870c=>{
// 0x10f9870c bl #0x10f93d2c
cpu.x[30]=0x10f98710;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
// 0x10f98710 mov x25, x0
cpu.x[25]=(((cpu.x[0]))) as u64;
// 0x10f98714 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f98718 bl #0x10f94124
cpu.x[30]=0x10f9871c;super::chunk5::f_10f94124(cpu,rt,0x10f94124)?;
// 0x10f9871c mov w1, w0
cpu.x[1]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f98720 mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10f98724 bl #0x10d3c9f0
cpu.x[30]=0x10f98728;super::dispatch(cpu,rt,0x10d3c9f0)?;
// 0x10f98728 cbz x0, #0x10f98764
if (cpu.x[0]) == 0 {pc=0x10f98764;continue;}pc=0x10f9872c;continue;
}
0x10f9872c=>{
// 0x10f9872c bl #0x10f940fc
cpu.x[30]=0x10f98730;super::chunk9::f_10f940fc(cpu,rt,0x10f940fc)?;
// 0x10f98730 mov x22, x0
cpu.x[22]=(((cpu.x[0]))) as u64;
// 0x10f98734 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f98738 bl #0x10f93d3c
cpu.x[30]=0x10f9873c;super::chunk3::f_10f93d3c(cpu,rt,0x10f93d3c)?;
// 0x10f9873c mov w6, w0
cpu.x[6]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f98740 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f98744 mov x1, x23
cpu.x[1]=(((cpu.x[23]))) as u64;
// 0x10f98748 mov x2, x21
cpu.x[2]=(((cpu.x[21]))) as u64;
// 0x10f9874c mov w3, w20
cpu.x[3]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x10f98750 mov x4, x24
cpu.x[4]=(((cpu.x[24]))) as u64;
// 0x10f98754 mov x5, x19
cpu.x[5]=(((cpu.x[19]))) as u64;
// 0x10f98758 mov w7, wzr
cpu.x[7]=(((0u64))) as u32 as u64;
// 0x10f9875c str xzr, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f98760 bl #0x10fc0e08
cpu.x[30]=0x10f98764;super::chunk14::f_10fc0e08(cpu,rt,0x10fc0e08)?;
pc=0x10f98764;continue;
}
0x10f98764=>{
// 0x10f98764 mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
// 0x10f98768 ldp x20, x19, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f9876c ldp x22, x21, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f98770 ldp x24, x23, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10f98774 ldp x26, x25, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10f98778 ldp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10f9877c add sp, sp, #0x60
{let a=((cpu.sp));let b=((0x60u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10f98780 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f985a4")),}}}
#[inline]
pub(super) unsafe fn f_10f99a38(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f99a38=>{
// 0x10f99a38 ldr w8, [x0, #0x1c]
{let address=(cpu.x[0]).wrapping_add(0x1cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f99a3c cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f99a40 b.lt #0x10f99a60
if cpu.condition(11){pc=0x10f99a60;continue;}pc=0x10f99a44;continue;
}
0x10f99a44=>{
// 0x10f99a44 ldr x9, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f99a48 ldr x10, [x9]
{let address=(cpu.x[9]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10f99a4c ldr w10, [x10]
{let address=(cpu.x[10]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10f99a50 cmp w10, w1
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f99a54 b.ne #0x10f99a68
if cpu.condition(1){pc=0x10f99a68;continue;}pc=0x10f99a58;continue;
}
0x10f99a58=>{
// 0x10f99a58 mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
// 0x10f99a5c ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f99a60=>{
// 0x10f99a60 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10f99a64 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f99a68=>{
// 0x10f99a68 mov w11, #1
cpu.x[11]=(((0x1u64))) as u32 as u64;
pc=0x10f99a6c;continue;
}
0x10f99a6c=>{
// 0x10f99a6c mov x10, x11
cpu.x[10]=(((cpu.x[11]))) as u64;
// 0x10f99a70 cmp x8, x11
{let a=((cpu.x[8]));let b=((cpu.x[11]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f99a74 b.eq #0x10f99a8c
if cpu.condition(0){pc=0x10f99a8c;continue;}pc=0x10f99a78;continue;
}
0x10f99a78=>{
// 0x10f99a78 ldr x11, [x9, x10, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[10])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[11]=(v0) as u64;}
// 0x10f99a7c ldr w11, [x11]
{let address=(cpu.x[11]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
// 0x10f99a80 cmp w11, w1
{let a=((cpu.x[11] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f99a84 add x11, x10, #1
{let a=((cpu.x[10]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
// 0x10f99a88 b.ne #0x10f99a6c
if cpu.condition(1){pc=0x10f99a6c;continue;}pc=0x10f99a8c;continue;
}
0x10f99a8c=>{
// 0x10f99a8c cmp x10, x8
{let a=((cpu.x[10]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f99a90 cset w0, lo
cpu.x[0]=((cpu.condition(3)) as u64) as u32 as u64;
// 0x10f99a94 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f99a38")),}}}
#[inline]
pub(super) unsafe fn f_10f99a98(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f99a98=>{
// 0x10f99a98 ldr w13, [x0, #0x1c]
{let address=(cpu.x[0]).wrapping_add(0x1cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[13]=(v0) as u32 as u64;}
// 0x10f99a9c cmp w13, #1
{let a=((cpu.x[13] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f99aa0 b.lt #0x10f99ae0
if cpu.condition(11){pc=0x10f99ae0;continue;}pc=0x10f99aa4;continue;
}
0x10f99aa4=>{
// 0x10f99aa4 ldr x11, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[11]=(v0) as u64;}
// 0x10f99aa8 lsl x14, x13, #3
cpu.x[14]=(((((cpu.x[13]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10f99aac mov x8, x0
cpu.x[8]=(((cpu.x[0]))) as u64;
// 0x10f99ab0 mov x9, xzr
cpu.x[9]=(((0u64))) as u64;
// 0x10f99ab4 mov x10, xzr
cpu.x[10]=(((0u64))) as u64;
// 0x10f99ab8 sub w12, w13, #1
{let a=((cpu.x[13] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[12]=(result) as u32 as u64;}
pc=0x10f99abc;continue;
}
0x10f99abc=>{
// 0x10f99abc ldr x0, [x11, x9]
{let address=((cpu.x[11]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[9])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f99ac0 ldr w15, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[15]=(v0) as u32 as u64;}
// 0x10f99ac4 cmp w15, w1
{let a=((cpu.x[15] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f99ac8 b.eq #0x10f99ae4
if cpu.condition(0){pc=0x10f99ae4;continue;}pc=0x10f99acc;continue;
}
0x10f99acc=>{
// 0x10f99acc add x9, x9, #8
{let a=((cpu.x[9]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
// 0x10f99ad0 sub w12, w12, #1
{let a=((cpu.x[12] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[12]=(result) as u32 as u64;}
// 0x10f99ad4 sub x10, x10, #1
{let a=((cpu.x[10]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[10]=(result) as u64;}
// 0x10f99ad8 cmp x14, x9
{let a=((cpu.x[14]));let b=((cpu.x[9]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f99adc b.ne #0x10f99abc
if cpu.condition(1){pc=0x10f99abc;continue;}pc=0x10f99ae0;continue;
}
0x10f99ae0=>{
// 0x10f99ae0 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f99ae4=>{
// 0x10f99ae4 neg w15, w10
cpu.x[15]=(0u64.wrapping_sub(((cpu.x[10] as u32 as u64)))) as u32 as u64;
// 0x10f99ae8 sub w14, w13, #1
{let a=((cpu.x[13] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[14]=(result) as u32 as u64;}
// 0x10f99aec cmp w14, w15
{let a=((cpu.x[14] as u32 as u64));let b=((cpu.x[15] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f99af0 str w14, [x8, #0x1c]
{let address=(cpu.x[8]).wrapping_add(0x1cu64);let v0=cpu.x[14] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f99af4 b.le #0x10f99b80
if cpu.condition(13){pc=0x10f99b80;continue;}pc=0x10f99af8;continue;
}
0x10f99af8=>{
// 0x10f99af8 add x13, x14, x10
{let a=((cpu.x[14]));let b=((cpu.x[10]));let result=a.wrapping_add(b);cpu.x[13]=(result) as u64;}
// 0x10f99afc cmp w13, #1
{let a=((cpu.x[13] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f99b00 b.lt #0x10f99b80
if cpu.condition(11){pc=0x10f99b80;continue;}pc=0x10f99b04;continue;
}
0x10f99b04=>{
// 0x10f99b04 cmp w13, #3
{let a=((cpu.x[13] as u32 as u64));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f99b08 add x8, x11, x9
{let a=((cpu.x[11]));let b=((cpu.x[9]));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10f99b0c b.ls #0x10f99b60
if cpu.condition(9){pc=0x10f99b60;continue;}pc=0x10f99b10;continue;
}
0x10f99b10=>{
// 0x10f99b10 mov w12, w12
cpu.x[12]=(((cpu.x[12] as u32 as u64))) as u32 as u64;
// 0x10f99b14 lsr x15, x12, #2
cpu.x[15]=(((((cpu.x[12]))) as u64).wrapping_shr(((((0x2u64))) & 63) as u32) as u64) as u64;
// 0x10f99b18 and x12, x13, #0xffffffff
{let a=((cpu.x[13]));let b=((0xffffffffu64));let result=a & b;cpu.x[12]=(result) as u64;}
// 0x10f99b1c neg w16, w15, lsl #2
cpu.x[16]=(0u64.wrapping_sub((((cpu.x[15] as u32 as u64)) as u32).wrapping_shl(2) as u64)) as u32 as u64;
// 0x10f99b20 add x15, x11, x15, lsl #5
{let a=((cpu.x[11]));let b=(((cpu.x[15])) as u64).wrapping_shl(5) as u64;let result=a.wrapping_add(b);cpu.x[15]=(result) as u64;}
// 0x10f99b24 and x11, x12, #0xfffffffc
{let a=((cpu.x[12]));let b=((0xfffffffcu64));let result=a & b;cpu.x[11]=(result) as u64;}
// 0x10f99b28 add x16, x16, w14, uxtw
{let a=((cpu.x[16]));let b=(((cpu.x[14] as u32 as u64) as u32 as u64));let result=a.wrapping_add(b);cpu.x[16]=(result) as u64;}
// 0x10f99b2c add x14, x8, #0x10
{let a=((cpu.x[8]));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[14]=(result) as u64;}
// 0x10f99b30 add x8, x15, x9
{let a=((cpu.x[15]));let b=((cpu.x[9]));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10f99b34 and x9, x13, #0xfffffffc
{let a=((cpu.x[13]));let b=((0xfffffffcu64));let result=a & b;cpu.x[9]=(result) as u64;}
// 0x10f99b38 add x10, x16, x10
{let a=((cpu.x[16]));let b=((cpu.x[10]));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
pc=0x10f99b3c;continue;
}
0x10f99b3c=>{
// 0x10f99b3c ldur q0, [x14, #-8]
{let address=(cpu.x[14]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));cpu.q[0]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x10f99b40 ldur q1, [x14, #8]
{let address=(cpu.x[14]).wrapping_add(0x8u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));cpu.q[1]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x10f99b44 subs x9, x9, #4
{let a=((cpu.x[9]));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x10f99b48 stp q0, q1, [x14, #-0x10]
{let address=(cpu.x[14]).wrapping_add(0xfffffffffffffff0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[1] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10f99b4c add x14, x14, #0x20
{let a=((cpu.x[14]));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[14]=(result) as u64;}
// 0x10f99b50 b.ne #0x10f99b3c
if cpu.condition(1){pc=0x10f99b3c;continue;}pc=0x10f99b54;continue;
}
0x10f99b54=>{
// 0x10f99b54 cmp x11, x12
{let a=((cpu.x[11]));let b=((cpu.x[12]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f99b58 b.eq #0x10f99b80
if cpu.condition(0){pc=0x10f99b80;continue;}pc=0x10f99b5c;continue;
}
0x10f99b5c=>{
// 0x10f99b5c mov w13, w10
cpu.x[13]=(((cpu.x[10] as u32 as u64))) as u32 as u64;
pc=0x10f99b60;continue;
}
0x10f99b60=>{
// 0x10f99b60 add w9, w13, #1
{let a=((cpu.x[13] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10f99b64 add x8, x8, #8
{let a=((cpu.x[8]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
pc=0x10f99b68;continue;
}
0x10f99b68=>{
// 0x10f99b68 ldr x10, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10f99b6c sub w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10f99b70 cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f99b74 stur x10, [x8, #-8]
{let address=(cpu.x[8]).wrapping_add(0xfffffffffffffff8u64);let v0=cpu.x[10];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f99b78 add x8, x8, #8
{let a=((cpu.x[8]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10f99b7c b.hi #0x10f99b68
if cpu.condition(8){pc=0x10f99b68;continue;}pc=0x10f99b80;continue;
}
0x10f99b80=>{
// 0x10f99b80 b #0x1187d2c0
super::dispatch(cpu,rt,0x1187d2c0)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f99a98")),}}}
#[inline]
pub(super) unsafe fn f_10f9ac50(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f9ac50=>{
// 0x10f9ac50 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f9ac54 stp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f9ac58 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f9ac5c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f9ac60 ldr w8, [x0, #0x24]
{let address=(cpu.x[0]).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f9ac64 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f9ac68 mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x10f9ac6c cmp w8, #0xa
{let a=((cpu.x[8] as u32 as u64));let b=((0xau64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9ac70 b.eq #0x10f9ac84
if cpu.condition(0){pc=0x10f9ac84;continue;}pc=0x10f9ac74;continue;
}
0x10f9ac74=>{
// 0x10f9ac74 adrp x0, #0x1032d000
cpu.x[0]=(0x1032d000u64) as u64;
// 0x10f9ac78 add x0, x0, #0x4bb
{let a=((cpu.x[0]));let b=((0x4bbu64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10f9ac7c mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10f9ac80 bl #0x1128df18
cpu.x[30]=0x10f9ac84;super::dispatch(cpu,rt,0x1128df18)?;
pc=0x10f9ac84;continue;
}
0x10f9ac84=>{
// 0x10f9ac84 ldp w8, w9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[8]=(v0) as u32 as u64;cpu.x[9]=(v1) as u32 as u64;}
// 0x10f9ac88 strb wzr, [x19, #0x40]
{let address=(cpu.x[19]).wrapping_add(0x40u64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f9ac8c mul w8, w9, w8
cpu.x[8]=((((cpu.x[9] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64)))) as u32 as u64;
// 0x10f9ac90 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9ac94 b.lt #0x10f9acf0
if cpu.condition(11){pc=0x10f9acf0;continue;}pc=0x10f9ac98;continue;
}
0x10f9ac98=>{
// 0x10f9ac98 ldr x9, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f9ac9c cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9aca0 b.ne #0x10f9acac
if cpu.condition(1){pc=0x10f9acac;continue;}pc=0x10f9aca4;continue;
}
0x10f9aca4=>{
// 0x10f9aca4 mov x10, xzr
cpu.x[10]=(((0u64))) as u64;
// 0x10f9aca8 b #0x10f9acd8
pc=0x10f9acd8;continue;
}
0x10f9acac=>{
// 0x10f9acac and x10, x8, #0x7ffffffe
{let a=((cpu.x[8]));let b=((0x7ffffffeu64));let result=a & b;cpu.x[10]=(result) as u64;}
// 0x10f9acb0 add x11, x9, #8
{let a=((cpu.x[9]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
// 0x10f9acb4 mov x12, x10
cpu.x[12]=(((cpu.x[10]))) as u64;
pc=0x10f9acb8;continue;
}
0x10f9acb8=>{
// 0x10f9acb8 ldp x13, x14, [x11, #-8]
{let address=(cpu.x[11]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[13]=(v0) as u64;cpu.x[14]=(v1) as u64;}
// 0x10f9acbc subs x12, x12, #2
{let a=((cpu.x[12]));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.x[12]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x10f9acc0 add x11, x11, #0x10
{let a=((cpu.x[11]));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
// 0x10f9acc4 str wzr, [x13, #0xc]
{let address=(cpu.x[13]).wrapping_add(0xcu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f9acc8 str wzr, [x14, #0xc]
{let address=(cpu.x[14]).wrapping_add(0xcu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f9accc b.ne #0x10f9acb8
if cpu.condition(1){pc=0x10f9acb8;continue;}pc=0x10f9acd0;continue;
}
0x10f9acd0=>{
// 0x10f9acd0 cmp x10, x8
{let a=((cpu.x[10]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f9acd4 b.eq #0x10f9acf0
if cpu.condition(0){pc=0x10f9acf0;continue;}pc=0x10f9acd8;continue;
}
0x10f9acd8=>{
// 0x10f9acd8 add x9, x9, x10, lsl #3
{let a=((cpu.x[9]));let b=(((cpu.x[10])) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
// 0x10f9acdc sub x8, x8, x10
{let a=((cpu.x[8]));let b=((cpu.x[10]));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u64;}
pc=0x10f9ace0;continue;
}
0x10f9ace0=>{
// 0x10f9ace0 ldr x10, [x9], #8
{let address=(cpu.x[9]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;cpu.x[9]=((cpu.x[9]).wrapping_add(((0x8u64)))) as u64;}
// 0x10f9ace4 subs x8, x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x10f9ace8 str wzr, [x10, #0xc]
{let address=(cpu.x[10]).wrapping_add(0xcu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f9acec b.ne #0x10f9ace0
if cpu.condition(1){pc=0x10f9ace0;continue;}pc=0x10f9acf0;continue;
}
0x10f9acf0=>{
// 0x10f9acf0 ldr w8, [x20, #0x14]
{let address=(cpu.x[20]).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f9acf4 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9acf8 b.lt #0x10f9ad20
if cpu.condition(11){pc=0x10f9ad20;continue;}pc=0x10f9acfc;continue;
}
0x10f9acfc=>{
// 0x10f9acfc lsl x21, x8, #3
cpu.x[21]=(((((cpu.x[8]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10f9ad00 mov x22, xzr
cpu.x[22]=(((0u64))) as u64;
pc=0x10f9ad04;continue;
}
0x10f9ad04=>{
// 0x10f9ad04 ldr x8, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f9ad08 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f9ad0c ldr x1, [x8, x22]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[22])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10f9ad10 bl #0x10f9ada0
cpu.x[30]=0x10f9ad14;super::chunk2::f_10f9ada0(cpu,rt,0x10f9ada0)?;
pc=0x10f9ad14;continue;
}
0x10f9ad14=>{
// 0x10f9ad14 add x22, x22, #8
{let a=((cpu.x[22]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[22]=(result) as u64;}
// 0x10f9ad18 cmp x21, x22
{let a=((cpu.x[21]));let b=((cpu.x[22]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f9ad1c b.ne #0x10f9ad04
if cpu.condition(1){pc=0x10f9ad04;continue;}pc=0x10f9ad20;continue;
}
0x10f9ad20=>{
// 0x10f9ad20 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f9ad24 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f9ad28 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f9ad2c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f9ac50")),}}}
#[inline]
pub(super) unsafe fn f_10f9c5c8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f9c5c8=>{
// 0x10f9c5c8 ldr x0, [x0, #0x248]
{let address=(cpu.x[0]).wrapping_add(0x248u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f9c5cc b #0x10f99780
super::chunk8::f_10f99780(cpu,rt,0x10f99780)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f9c5c8")),}}}
#[inline]
pub(super) unsafe fn f_10f9c678(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f9c678=>{
// 0x10f9c678 ldr w0, [x0, #0x320]
{let address=(cpu.x[0]).wrapping_add(0x320u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10f9c67c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f9c678")),}}}
#[inline]
pub(super) unsafe fn f_10f9dd98(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f9dd98=>{
// 0x10f9dd98 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f9dd9c stp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f9dda0 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f9dda4 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f9dda8 ldr x8, [x0, #0x210]
{let address=(cpu.x[0]).wrapping_add(0x210u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f9ddac ldr w19, [x8, w1, sxtw #2]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[1] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[19]=(v0) as u32 as u64;}
// 0x10f9ddb0 cmn w19, #1
{let a=((cpu.x[19] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.flags(a,b,result,32,false);}
// 0x10f9ddb4 b.eq #0x10f9de18
if cpu.condition(0){pc=0x10f9de18;continue;}pc=0x10f9ddb8;continue;
}
0x10f9ddb8=>{
// 0x10f9ddb8 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10f9ddbc bl #0x10f93d2c
cpu.x[30]=0x10f9ddc0;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
pc=0x10f9ddc0;continue;
}
0x10f9ddc0=>{
// 0x10f9ddc0 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10f9ddc4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f9ddc8 bl #0x10f93fc0
cpu.x[30]=0x10f9ddcc;super::chunk5::f_10f93fc0(cpu,rt,0x10f93fc0)?;
pc=0x10f9ddcc;continue;
}
0x10f9ddcc=>{
// 0x10f9ddcc bl #0x112942b8
cpu.x[30]=0x10f9ddd0;super::chunk12::f_112942b8(cpu,rt,0x112942b8)?;
pc=0x10f9ddd0;continue;
}
0x10f9ddd0=>{
// 0x10f9ddd0 mov w22, w0
cpu.x[22]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f9ddd4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f9ddd8 bl #0x10f93fc0
cpu.x[30]=0x10f9dddc;super::chunk5::f_10f93fc0(cpu,rt,0x10f93fc0)?;
pc=0x10f9dddc;continue;
}
0x10f9dddc=>{
// 0x10f9dddc bl #0x112942c0
cpu.x[30]=0x10f9dde0;super::chunk6::f_112942c0(cpu,rt,0x112942c0)?;
pc=0x10f9dde0;continue;
}
0x10f9dde0=>{
// 0x10f9dde0 mov w2, w0
cpu.x[2]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f9dde4 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f9dde8 mov w1, w22
cpu.x[1]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10f9ddec bl #0x10d33a9c
cpu.x[30]=0x10f9ddf0;super::chunk4::f_10d33a9c(cpu,rt,0x10d33a9c)?;
pc=0x10f9ddf0;continue;
}
0x10f9ddf0=>{
// 0x10f9ddf0 mov w1, w0
cpu.x[1]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f9ddf4 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f9ddf8 bl #0x10d32c84
cpu.x[30]=0x10f9ddfc;super::chunk15::f_10d32c84(cpu,rt,0x10d32c84)?;
pc=0x10f9ddfc;continue;
}
0x10f9ddfc=>{
// 0x10f9ddfc ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f9de00 ldr x2, [x8, #0x38]
{let address=(cpu.x[8]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10f9de04 mov w1, w19
cpu.x[1]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10f9de08 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f9de0c ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f9de10 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f9de14 br x2
{let target=cpu.x[2];if target>=0x10f9dd98 && target<0x10f9de2c{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
0x10f9de18=>{
// 0x10f9de18 mov x0, xzr
cpu.x[0]=(((0u64))) as u64;
// 0x10f9de1c ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f9de20 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f9de24 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f9de28 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f9dd98")),}}}
#[inline]
pub(super) unsafe fn f_10fa14bc(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fa14bc=>{
// 0x10fa14bc stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fa14c0 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fa14c4 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fa14c8 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10fa14cc ldr x0, [x0, #0x48]
{let address=(cpu.x[0]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fa14d0 bl #0x10e0b6b4
cpu.x[30]=0x10fa14d4;super::chunk8::f_10e0b6b4(cpu,rt,0x10e0b6b4)?;
pc=0x10fa14d4;continue;
}
0x10fa14d4=>{
// 0x10fa14d4 ldr x0, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fa14d8 bl #0x10e16208
cpu.x[30]=0x10fa14dc;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
pc=0x10fa14dc;continue;
}
0x10fa14dc=>{
// 0x10fa14dc ldrb w8, [x0, #0x795]
{let address=(cpu.x[0]).wrapping_add(0x795u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fa14e0 cbnz w8, #0x10fa150c
if (cpu.x[8] as u32 as u64) != 0 {pc=0x10fa150c;continue;}pc=0x10fa14e4;continue;
}
0x10fa14e4=>{
// 0x10fa14e4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fa14e8 bl #0x10f93f94
cpu.x[30]=0x10fa14ec;super::chunk3::f_10f93f94(cpu,rt,0x10f93f94)?;
// 0x10fa14ec ldr x0, [x0, #0x68]
{let address=(cpu.x[0]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fa14f0 bl #0x110ca464
cpu.x[30]=0x10fa14f4;super::chunk3::f_110ca464(cpu,rt,0x110ca464)?;
// 0x10fa14f4 bl #0x111dc5f0
cpu.x[30]=0x10fa14f8;super::dispatch(cpu,rt,0x111dc5f0)?;
// 0x10fa14f8 tbz w0, #0, #0x10fa150c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fa150c;continue;}pc=0x10fa14fc;continue;
}
0x10fa14fc=>{
// 0x10fa14fc mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10fa1500 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fa1504 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10fa1508 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10fa150c=>{
// 0x10fa150c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fa1510 bl #0x10f93d3c
cpu.x[30]=0x10fa1514;super::chunk3::f_10f93d3c(cpu,rt,0x10f93d3c)?;
pc=0x10fa1514;continue;
}
0x10fa1514=>{
// 0x10fa1514 mov w20, w0
cpu.x[20]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fa1518 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fa151c bl #0x10f93d2c
cpu.x[30]=0x10fa1520;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
pc=0x10fa1520;continue;
}
0x10fa1520=>{
// 0x10fa1520 mov w1, w20
cpu.x[1]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x10fa1524 bl #0x10d448b0
cpu.x[30]=0x10fa1528;super::chunk15::f_10d448b0(cpu,rt,0x10d448b0)?;
pc=0x10fa1528;continue;
}
0x10fa1528=>{
// 0x10fa1528 cmp w0, #0
{let a=((cpu.x[0] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa152c cset w0, ne
cpu.x[0]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x10fa1530 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fa1534 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10fa1538 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fa14bc")),}}}
#[inline]
pub(super) unsafe fn f_10fa3490(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fa3490=>{
// 0x10fa3490 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fa3494 str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fa3498 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fa349c mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10fa34a0 bl #0x10f93d2c
cpu.x[30]=0x10fa34a4;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
pc=0x10fa34a4;continue;
}
0x10fa34a4=>{
// 0x10fa34a4 bl #0x10d34864
cpu.x[30]=0x10fa34a8;super::chunk9::f_10d34864(cpu,rt,0x10d34864)?;
pc=0x10fa34a8;continue;
}
0x10fa34a8=>{
// 0x10fa34a8 tbz w0, #0, #0x10fa34b8
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fa34b8;continue;}pc=0x10fa34ac;continue;
}
0x10fa34ac=>{
// 0x10fa34ac ldr w8, [x19, #0x33c]
{let address=(cpu.x[19]).wrapping_add(0x33cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fa34b0 add w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fa34b4 str w8, [x19, #0x33c]
{let address=(cpu.x[19]).wrapping_add(0x33cu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10fa34b8;continue;
}
0x10fa34b8=>{
// 0x10fa34b8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fa34bc bl #0x10f94180
cpu.x[30]=0x10fa34c0;super::chunk14::f_10f94180(cpu,rt,0x10f94180)?;
pc=0x10fa34c0;continue;
}
0x10fa34c0=>{
// 0x10fa34c0 tbz w0, #0, #0x10fa34ec
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fa34ec;continue;}pc=0x10fa34c4;continue;
}
0x10fa34c4=>{
// 0x10fa34c4 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fa34c8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fa34cc ldr x8, [x8, #0x138]
{let address=(cpu.x[8]).wrapping_add(0x138u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fa34d0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fa34d4;super::dispatch(cpu,rt,target)?;}
// 0x10fa34d4 tbz w0, #0, #0x10fa34ec
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fa34ec;continue;}pc=0x10fa34d8;continue;
}
0x10fa34d8=>{
// 0x10fa34d8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fa34dc mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10fa34e0 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10fa34e4 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10fa34e8 b #0x10fa1d9c
super::dispatch(cpu,rt,0x10fa1d9c)?;return Ok(());
}
0x10fa34ec=>{
// 0x10fa34ec ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10fa34f0 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10fa34f4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fa3490")),}}}
#[inline]
pub(super) unsafe fn f_10fa3e84(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fa3e84=>{
// 0x10fa3e84 ldr x8, [x0, #0x210]
{let address=(cpu.x[0]).wrapping_add(0x210u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fa3e88 ldr w0, [x8, w1, sxtw #2]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[1] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10fa3e8c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fa3e84")),}}}
#[inline]
pub(super) unsafe fn f_10fa3e90(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fa3e90=>{
// 0x10fa3e90 add x0, x0, #0x208
{let a=((cpu.x[0]));let b=((0x208u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fa3e94 b #0x10fa58e0
super::chunk10::f_10fa58e0(cpu,rt,0x10fa58e0)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fa3e90")),}}}
#[inline]
pub(super) unsafe fn f_10fa4038(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fa4038=>{
// 0x10fa4038 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fa403c str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fa4040 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fa4044 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fa4048 mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x10fa404c mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10fa4050 bl #0x10f93d2c
cpu.x[30]=0x10fa4054;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
pc=0x10fa4054;continue;
}
0x10fa4054=>{
// 0x10fa4054 bl #0x10d39854
cpu.x[30]=0x10fa4058;super::chunk13::f_10d39854(cpu,rt,0x10d39854)?;
pc=0x10fa4058;continue;
}
0x10fa4058=>{
// 0x10fa4058 tbnz w0, #0, #0x10fa408c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10fa408c;continue;}pc=0x10fa405c;continue;
}
0x10fa405c=>{
// 0x10fa405c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fa4060 bl #0x10f93d2c
cpu.x[30]=0x10fa4064;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
pc=0x10fa4064;continue;
}
0x10fa4064=>{
// 0x10fa4064 bl #0x10d39808
cpu.x[30]=0x10fa4068;super::chunk4::f_10d39808(cpu,rt,0x10d39808)?;
pc=0x10fa4068;continue;
}
0x10fa4068=>{
// 0x10fa4068 tbnz w0, #0, #0x10fa408c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10fa408c;continue;}pc=0x10fa406c;continue;
}
0x10fa406c=>{
// 0x10fa406c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fa4070 bl #0x10f93d2c
cpu.x[30]=0x10fa4074;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
pc=0x10fa4074;continue;
}
0x10fa4074=>{
// 0x10fa4074 bl #0x10d34864
cpu.x[30]=0x10fa4078;super::chunk9::f_10d34864(cpu,rt,0x10d34864)?;
pc=0x10fa4078;continue;
}
0x10fa4078=>{
// 0x10fa4078 tbnz w0, #0, #0x10fa408c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10fa408c;continue;}pc=0x10fa407c;continue;
}
0x10fa407c=>{
// 0x10fa407c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fa4080 bl #0x10f93d2c
cpu.x[30]=0x10fa4084;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
pc=0x10fa4084;continue;
}
0x10fa4084=>{
// 0x10fa4084 bl #0x10d36084
cpu.x[30]=0x10fa4088;super::chunk6::f_10d36084(cpu,rt,0x10d36084)?;
pc=0x10fa4088;continue;
}
0x10fa4088=>{
// 0x10fa4088 tbz w0, #0, #0x10fa40a0
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fa40a0;continue;}pc=0x10fa408c;continue;
}
0x10fa408c=>{
// 0x10fa408c ldr w0, [x19, #0x33c]
{let address=(cpu.x[19]).wrapping_add(0x33cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10fa4090 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fa4094 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10fa4098 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10fa409c ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10fa40a0=>{
// 0x10fa40a0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fa40a4 bl #0x10f93d2c
cpu.x[30]=0x10fa40a8;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
pc=0x10fa40a8;continue;
}
0x10fa40a8=>{
// 0x10fa40a8 bl #0x10d36e90
cpu.x[30]=0x10fa40ac;super::chunk6::f_10d36e90(cpu,rt,0x10d36e90)?;
pc=0x10fa40ac;continue;
}
0x10fa40ac=>{
// 0x10fa40ac mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fa40b0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fa40b4 bl #0x10f93d2c
cpu.x[30]=0x10fa40b8;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
pc=0x10fa40b8;continue;
}
0x10fa40b8=>{
// 0x10fa40b8 tbz w21, #0, #0x10fa40fc
if ((cpu.x[21] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fa40fc;continue;}pc=0x10fa40bc;continue;
}
0x10fa40bc=>{
// 0x10fa40bc mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10fa40c0 bl #0x10d3c2ac
cpu.x[30]=0x10fa40c4;super::chunk1::f_10d3c2ac(cpu,rt,0x10d3c2ac)?;
// 0x10fa40c4 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fa40c8 ldr x8, [x8, #0x138]
{let address=(cpu.x[8]).wrapping_add(0x138u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fa40cc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fa40d0;super::dispatch(cpu,rt,target)?;}
// 0x10fa40d0 tbz w0, #0, #0x10fa40e8
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fa40e8;continue;}pc=0x10fa40d4;continue;
}
0x10fa40d4=>{
// 0x10fa40d4 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fa40d8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fa40dc ldr x8, [x8, #0x138]
{let address=(cpu.x[8]).wrapping_add(0x138u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fa40e0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fa40e4;super::dispatch(cpu,rt,target)?;}
// 0x10fa40e4 tbnz w0, #0, #0x10fa4138
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10fa4138;continue;}pc=0x10fa40e8;continue;
}
0x10fa40e8=>{
// 0x10fa40e8 mov w0, #3
cpu.x[0]=(((0x3u64))) as u32 as u64;
// 0x10fa40ec ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fa40f0 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10fa40f4 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10fa40f8 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10fa40fc=>{
// 0x10fa40fc bl #0x10d3ae8c
cpu.x[30]=0x10fa4100;super::chunk5::f_10d3ae8c(cpu,rt,0x10d3ae8c)?;
pc=0x10fa4100;continue;
}
0x10fa4100=>{
// 0x10fa4100 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fa4104 mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fa4108 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fa410c ldr x8, [x8, #0x138]
{let address=(cpu.x[8]).wrapping_add(0x138u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fa4110 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fa4114;super::dispatch(cpu,rt,target)?;}
pc=0x10fa4114;continue;
}
0x10fa4114=>{
// 0x10fa4114 tbz w21, #0, #0x10fa4134
if ((cpu.x[21] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fa4134;continue;}pc=0x10fa4118;continue;
}
0x10fa4118=>{
// 0x10fa4118 tst w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.logical_flags(result,32);}
// 0x10fa411c mov w8, #3
cpu.x[8]=(((0x3u64))) as u32 as u64;
// 0x10fa4120 csel w0, wzr, w8, ne
cpu.x[0]=(if cpu.condition(1){((0u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10fa4124 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fa4128 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10fa412c ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10fa4130 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10fa4134=>{
// 0x10fa4134 tbz w0, #0, #0x10fa40e8
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fa40e8;continue;}pc=0x10fa4138;continue;
}
0x10fa4138=>{
// 0x10fa4138 ldr w20, [x20, #0x78]
{let address=(cpu.x[20]).wrapping_add(0x78u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[20]=(v0) as u32 as u64;}
// 0x10fa413c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fa4140 bl #0x10f93d2c
cpu.x[30]=0x10fa4144;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
pc=0x10fa4144;continue;
}
0x10fa4144=>{
// 0x10fa4144 mov w1, w20
cpu.x[1]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x10fa4148 bl #0x10d3c460
cpu.x[30]=0x10fa414c;super::chunk0::f_10d3c460(cpu,rt,0x10d3c460)?;
pc=0x10fa414c;continue;
}
0x10fa414c=>{
// 0x10fa414c ldr w9, [x0, #0xc]
{let address=(cpu.x[0]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fa4150 mov w8, #2
cpu.x[8]=(((0x2u64))) as u32 as u64;
// 0x10fa4154 cmp w9, #2
{let a=((cpu.x[9] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa4158 csel w9, w9, w8, lt
cpu.x[9]=(if cpu.condition(11){((cpu.x[9] as u32 as u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10fa415c sub w0, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[0]=(result) as u32 as u64;}
// 0x10fa4160 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fa4164 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10fa4168 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10fa416c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fa4038")),}}}
#[inline]
pub(super) unsafe fn f_10fa4fe4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fa4fe4=>{
// 0x10fa4fe4 stp x29, x30, [sp, #-0x60]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffa0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fa4fe8 stp x28, x27, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[28];let v1=cpu.x[27];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fa4fec stp x26, x25, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[26];let v1=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fa4ff0 stp x24, x23, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fa4ff4 stp x22, x21, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fa4ff8 stp x20, x19, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fa4ffc mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
pc=0x10fa5000;continue;
}
0x10fa5000=>{
// 0x10fa5000 sub sp, sp, #0xa20
{let a=((cpu.sp));let b=((0xa20u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x10fa5004 mov x25, x0
cpu.x[25]=(((cpu.x[0]))) as u64;
// 0x10fa5008 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10fa500c mov x19, x4
cpu.x[19]=(((cpu.x[4]))) as u64;
// 0x10fa5010 str wzr, [x25, #0x24]!
{let address=(cpu.x[25]).wrapping_add(0x24u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);cpu.x[25]=(address) as u64;}
// 0x10fa5014 sub x23, x25, #0xc
{let a=((cpu.x[25]));let b=((0xcu64));let result=a.wrapping_sub(b);cpu.x[23]=(result) as u64;}
// 0x10fa5018 ldr w27, [x4, #0xc]
{let address=(cpu.x[4]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[27]=(v0) as u32 as u64;}
// 0x10fa501c str x3, [sp, #0x38]
{let address=(cpu.sp).wrapping_add(0x38u64);let v0=cpu.x[3];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fa5020 str x2, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=cpu.x[2];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fa5024 lsl x26, x27, #2
cpu.x[26]=(((((cpu.x[27]))) as u64).wrapping_shl(((((0x2u64))) & 63) as u32) as u64) as u64;
// 0x10fa5028 cmp w27, #0
{let a=((cpu.x[27] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa502c str x1, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=cpu.x[1];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fa5030 stur xzr, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fa5034 csinv x0, x26, xzr, ge
cpu.x[0]=(if cpu.condition(10){((cpu.x[26]))}else{!(((0u64)))}) as u64;
// 0x10fa5038 stur w27, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=cpu.x[27] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fa503c bl #0x1187d520
cpu.x[30]=0x10fa5040;super::chunk8::f_1187d520(cpu,rt,0x1187d520)?;
pc=0x10fa5040;continue;
}
0x10fa5040=>{
// 0x10fa5040 cmp w27, #1
{let a=((cpu.x[27] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa5044 stur x0, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fa5048 str x23, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fa504c b.lt #0x10fa53a8
if cpu.condition(11){pc=0x10fa53a8;continue;}pc=0x10fa5050;continue;
}
0x10fa5050=>{
// 0x10fa5050 ldr x8, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fa5054 mov w28, wzr
cpu.x[28]=(((0u64))) as u32 as u64;
// 0x10fa5058 mvn w22, w27
cpu.x[22]=(!(((cpu.x[27] as u32 as u64)))) as u32 as u64;
// 0x10fa505c str w22, [sp, #0x2c]
{let address=(cpu.sp).wrapping_add(0x2cu64);let v0=cpu.x[22] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fa5060 ldr x8, [x8, #0x1b0]
{let address=(cpu.x[8]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fa5064 ldr x8, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fa5068 ldr x8, [x8, #0xa8]
{let address=(cpu.x[8]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fa506c str x8, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fa5070 sub x8, x29, #0x18
{let a=((cpu.x[29]));let b=((0x18u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u64;}
// 0x10fa5074 add x8, x8, #0xc
{let a=((cpu.x[8]));let b=((0xcu64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fa5078 str x8, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fa507c b #0x10fa50b4
pc=0x10fa50b4;continue;
}
0x10fa5080=>{
// 0x10fa5080 str xzr, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10fa5084;continue;
}
0x10fa5084=>{
// 0x10fa5084 mov x9, x25
cpu.x[9]=(((cpu.x[25]))) as u64;
// 0x10fa5088 mov x21, x23
cpu.x[21]=(((cpu.x[23]))) as u64;
pc=0x10fa508c;continue;
}
0x10fa508c=>{
// 0x10fa508c ldr x10, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10fa5090 add w11, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u32 as u64;}
// 0x10fa5094 str w11, [x9]
{let address=(cpu.x[9]).wrapping_add(0x0u64);let v0=cpu.x[11] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fa5098 str w14, [x10, w8, sxtw #2]
{let address=((cpu.x[10]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=cpu.x[14] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10fa509c;continue;
}
0x10fa509c=>{
// 0x10fa509c add w28, w28, #1
{let a=((cpu.x[28] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[28]=(result) as u32 as u64;}
// 0x10fa50a0 cmp x27, #1
{let a=((cpu.x[27]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fa50a4 add w22, w22, #1
{let a=((cpu.x[22] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[22]=(result) as u32 as u64;}
// 0x10fa50a8 sub x26, x26, #4
{let a=((cpu.x[26]));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.x[26]=(result) as u64;}
// 0x10fa50ac mov x27, x24
cpu.x[27]=(((cpu.x[24]))) as u64;
// 0x10fa50b0 b.le #0x10fa53a8
if cpu.condition(13){pc=0x10fa53a8;continue;}pc=0x10fa50b4;continue;
}
0x10fa50b4=>{
// 0x10fa50b4 ldr x0, [sp, #0x38]
{let address=(cpu.sp).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fa50b8 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fa50bc sub x24, x27, #1
{let a=((cpu.x[27]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[24]=(result) as u64;}
// 0x10fa50c0 ldr x9, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fa50c4 ldr w1, [x8, x24, lsl #2]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[24])) as u64).wrapping_shl(2) as u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10fa50c8 ldr x8, [x9, #0x38]
{let address=(cpu.x[9]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fa50cc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fa50d0;super::dispatch(cpu,rt,target)?;}
pc=0x10fa50d0;continue;
}
0x10fa50d0=>{
// 0x10fa50d0 cbz x0, #0x10fa509c
if (cpu.x[0]) == 0 {pc=0x10fa509c;continue;}pc=0x10fa50d4;continue;
}
0x10fa50d4=>{
// 0x10fa50d4 ldr w9, [sp, #0x2c]
{let address=(cpu.sp).wrapping_add(0x2cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fa50d8 ldr x8, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fa50dc add w21, w28, w9
{let a=((cpu.x[28] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_add(b);cpu.x[21]=(result) as u32 as u64;}
// 0x10fa50e0 ldr x9, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fa50e4 cbz x9, #0x10fa51c4
if (cpu.x[9]) == 0 {pc=0x10fa51c4;continue;}pc=0x10fa50e8;continue;
}
0x10fa50e8=>{
// 0x10fa50e8 cmp x8, x9
{let a=((cpu.x[8]));let b=((cpu.x[9]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fa50ec b.ne #0x10fa51c4
if cpu.condition(1){pc=0x10fa51c4;continue;}pc=0x10fa50f0;continue;
}
0x10fa50f0=>{
// 0x10fa50f0 ldr x10, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10fa50f4 ldrsw x13, [x19, #0xc]
{let address=(cpu.x[19]).wrapping_add(0xcu64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[13]=(v0) as u64;}
// 0x10fa50f8 add x8, x10, x24, lsl #2
{let a=((cpu.x[10]));let b=(((cpu.x[24])) as u64).wrapping_shl(2) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fa50fc sub w9, w13, #1
{let a=((cpu.x[13] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fa5100 cmp x27, x13
{let a=((cpu.x[27]));let b=((cpu.x[13]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fa5104 ldr w14, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[14]=(v0) as u32 as u64;}
// 0x10fa5108 str w9, [x19, #0xc]
{let address=(cpu.x[19]).wrapping_add(0xcu64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fa510c b.ge #0x10fa5188
if cpu.condition(10){pc=0x10fa5188;continue;}pc=0x10fa5110;continue;
}
0x10fa5110=>{
// 0x10fa5110 sub w9, w13, w27
{let a=((cpu.x[13] as u32 as u64));let b=((cpu.x[27] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fa5114 cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa5118 b.lt #0x10fa5188
if cpu.condition(11){pc=0x10fa5188;continue;}pc=0x10fa511c;continue;
}
0x10fa511c=>{
// 0x10fa511c add w11, w13, w21
{let a=((cpu.x[13] as u32 as u64));let b=((cpu.x[21] as u32 as u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u32 as u64;}
// 0x10fa5120 cmp w11, #7
{let a=((cpu.x[11] as u32 as u64));let b=((0x7u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa5124 b.lo #0x10fa5168
if cpu.condition(3){pc=0x10fa5168;continue;}pc=0x10fa5128;continue;
}
0x10fa5128=>{
// 0x10fa5128 add x11, x11, #1
{let a=((cpu.x[11]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
// 0x10fa512c add w13, w13, w22
{let a=((cpu.x[13] as u32 as u64));let b=((cpu.x[22] as u32 as u64));let result=a.wrapping_add(b);cpu.x[13]=(result) as u32 as u64;}
// 0x10fa5130 add x10, x10, x26
{let a=((cpu.x[10]));let b=((cpu.x[26]));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x10fa5134 and x12, x11, #0x1fffffff8
{let a=((cpu.x[11]));let b=((0x1fffffff8u64));let result=a & b;cpu.x[12]=(result) as u64;}
// 0x10fa5138 add x13, x13, #1
{let a=((cpu.x[13]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[13]=(result) as u64;}
// 0x10fa513c add x8, x8, x12, lsl #2
{let a=((cpu.x[8]));let b=(((cpu.x[12])) as u64).wrapping_shl(2) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fa5140 sub w9, w9, w12
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[12] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fa5144 and x13, x13, #0x1fffffff8
{let a=((cpu.x[13]));let b=((0x1fffffff8u64));let result=a & b;cpu.x[13]=(result) as u64;}
pc=0x10fa5148;continue;
}
0x10fa5148=>{
// 0x10fa5148 ldp q0, q1, [x10]
{let address=(cpu.x[10]).wrapping_add(0x0u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));let v1=rt.memory.read::<u128>(address.wrapping_add(16));cpu.q[0]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;cpu.q[1]=(v1) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x10fa514c subs x13, x13, #8
{let a=((cpu.x[13]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.x[13]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x10fa5150 stur q0, [x10, #-4]
{let address=(cpu.x[10]).wrapping_add(0xfffffffffffffffcu64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10fa5154 stur q1, [x10, #0xc]
{let address=(cpu.x[10]).wrapping_add(0xcu64);let v0=(cpu.q[1] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10fa5158 add x10, x10, #0x20
{let a=((cpu.x[10]));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x10fa515c b.ne #0x10fa5148
if cpu.condition(1){pc=0x10fa5148;continue;}pc=0x10fa5160;continue;
}
0x10fa5160=>{
// 0x10fa5160 cmp x11, x12
{let a=((cpu.x[11]));let b=((cpu.x[12]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fa5164 b.eq #0x10fa5188
if cpu.condition(0){pc=0x10fa5188;continue;}pc=0x10fa5168;continue;
}
0x10fa5168=>{
// 0x10fa5168 add w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fa516c add x8, x8, #4
{let a=((cpu.x[8]));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
pc=0x10fa5170;continue;
}
0x10fa5170=>{
// 0x10fa5170 ldr w10, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10fa5174 sub w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fa5178 cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa517c stur w10, [x8, #-4]
{let address=(cpu.x[8]).wrapping_add(0xfffffffffffffffcu64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fa5180 add x8, x8, #4
{let a=((cpu.x[8]));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fa5184 b.hi #0x10fa5170
if cpu.condition(8){pc=0x10fa5170;continue;}pc=0x10fa5188;continue;
}
0x10fa5188=>{
// 0x10fa5188 ldp w9, w8, [x20, #0x20]
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10fa518c cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa5190 b.ne #0x10fa5080
if cpu.condition(1){pc=0x10fa5080;continue;}pc=0x10fa5194;continue;
}
0x10fa5194=>{
// 0x10fa5194 lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10fa5198 mov w10, #5
cpu.x[10]=(((0x5u64))) as u32 as u64;
// 0x10fa519c cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa51a0 csel w1, w10, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[10] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10fa51a4 cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa51a8 b.ge #0x10fa5080
if cpu.condition(10){pc=0x10fa5080;continue;}pc=0x10fa51ac;continue;
}
0x10fa51ac=>{
// 0x10fa51ac mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fa51b0 mov w21, w14
cpu.x[21]=(((cpu.x[14] as u32 as u64))) as u32 as u64;
// 0x10fa51b4 bl #0x1078f4d4
cpu.x[30]=0x10fa51b8;super::chunk2::f_1078f4d4(cpu,rt,0x1078f4d4)?;
// 0x10fa51b8 ldr w8, [x25]
{let address=(cpu.x[25]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fa51bc mov w14, w21
cpu.x[14]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10fa51c0 b #0x10fa5080
pc=0x10fa5080;continue;
}
0x10fa51c4=>{
// 0x10fa51c4 mov x23, x0
cpu.x[23]=(((cpu.x[0]))) as u64;
// 0x10fa51c8 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10fa51cc bl #0x10e7f0e4
cpu.x[30]=0x10fa51d0;super::chunk11::f_10e7f0e4(cpu,rt,0x10e7f0e4)?;
pc=0x10fa51d0;continue;
}
0x10fa51d0=>{
// 0x10fa51d0 tbz w0, #0, #0x10fa52ac
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fa52ac;continue;}pc=0x10fa51d4;continue;
}
0x10fa51d4=>{
// 0x10fa51d4 ldr x10, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10fa51d8 ldrsw x13, [x19, #0xc]
{let address=(cpu.x[19]).wrapping_add(0xcu64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[13]=(v0) as u64;}
// 0x10fa51dc ldr x23, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10fa51e0 add x8, x10, x24, lsl #2
{let a=((cpu.x[10]));let b=(((cpu.x[24])) as u64).wrapping_shl(2) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fa51e4 sub w9, w13, #1
{let a=((cpu.x[13] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fa51e8 cmp x27, x13
{let a=((cpu.x[27]));let b=((cpu.x[13]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fa51ec ldr w14, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[14]=(v0) as u32 as u64;}
// 0x10fa51f0 str w9, [x19, #0xc]
{let address=(cpu.x[19]).wrapping_add(0xcu64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fa51f4 b.ge #0x10fa5270
if cpu.condition(10){pc=0x10fa5270;continue;}pc=0x10fa51f8;continue;
}
0x10fa51f8=>{
// 0x10fa51f8 sub w9, w13, w27
{let a=((cpu.x[13] as u32 as u64));let b=((cpu.x[27] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fa51fc cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa5200 b.lt #0x10fa5270
if cpu.condition(11){pc=0x10fa5270;continue;}pc=0x10fa5204;continue;
}
0x10fa5204=>{
// 0x10fa5204 add w11, w13, w21
{let a=((cpu.x[13] as u32 as u64));let b=((cpu.x[21] as u32 as u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u32 as u64;}
// 0x10fa5208 cmp w11, #7
{let a=((cpu.x[11] as u32 as u64));let b=((0x7u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa520c b.lo #0x10fa5250
if cpu.condition(3){pc=0x10fa5250;continue;}pc=0x10fa5210;continue;
}
0x10fa5210=>{
// 0x10fa5210 add x11, x11, #1
{let a=((cpu.x[11]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
// 0x10fa5214 add w13, w13, w22
{let a=((cpu.x[13] as u32 as u64));let b=((cpu.x[22] as u32 as u64));let result=a.wrapping_add(b);cpu.x[13]=(result) as u32 as u64;}
// 0x10fa5218 add x10, x10, x26
{let a=((cpu.x[10]));let b=((cpu.x[26]));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x10fa521c and x12, x11, #0x1fffffff8
{let a=((cpu.x[11]));let b=((0x1fffffff8u64));let result=a & b;cpu.x[12]=(result) as u64;}
// 0x10fa5220 add x13, x13, #1
{let a=((cpu.x[13]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[13]=(result) as u64;}
// 0x10fa5224 add x8, x8, x12, lsl #2
{let a=((cpu.x[8]));let b=(((cpu.x[12])) as u64).wrapping_shl(2) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fa5228 sub w9, w9, w12
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[12] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fa522c and x13, x13, #0x1fffffff8
{let a=((cpu.x[13]));let b=((0x1fffffff8u64));let result=a & b;cpu.x[13]=(result) as u64;}
pc=0x10fa5230;continue;
}
0x10fa5230=>{
// 0x10fa5230 ldp q0, q1, [x10]
{let address=(cpu.x[10]).wrapping_add(0x0u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));let v1=rt.memory.read::<u128>(address.wrapping_add(16));cpu.q[0]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;cpu.q[1]=(v1) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x10fa5234 subs x13, x13, #8
{let a=((cpu.x[13]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.x[13]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x10fa5238 stur q0, [x10, #-4]
{let address=(cpu.x[10]).wrapping_add(0xfffffffffffffffcu64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10fa523c stur q1, [x10, #0xc]
{let address=(cpu.x[10]).wrapping_add(0xcu64);let v0=(cpu.q[1] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10fa5240 add x10, x10, #0x20
{let a=((cpu.x[10]));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x10fa5244 b.ne #0x10fa5230
if cpu.condition(1){pc=0x10fa5230;continue;}pc=0x10fa5248;continue;
}
0x10fa5248=>{
// 0x10fa5248 cmp x11, x12
{let a=((cpu.x[11]));let b=((cpu.x[12]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fa524c b.eq #0x10fa5270
if cpu.condition(0){pc=0x10fa5270;continue;}pc=0x10fa5250;continue;
}
0x10fa5250=>{
// 0x10fa5250 add w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fa5254 add x8, x8, #4
{let a=((cpu.x[8]));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
pc=0x10fa5258;continue;
}
0x10fa5258=>{
// 0x10fa5258 ldr w10, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10fa525c sub w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fa5260 cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa5264 stur w10, [x8, #-4]
{let address=(cpu.x[8]).wrapping_add(0xfffffffffffffffcu64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fa5268 add x8, x8, #4
{let a=((cpu.x[8]));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fa526c b.hi #0x10fa5258
if cpu.condition(8){pc=0x10fa5258;continue;}pc=0x10fa5270;continue;
}
0x10fa5270=>{
// 0x10fa5270 ldp w9, w8, [x20, #0x20]
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10fa5274 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa5278 b.ne #0x10fa5084
if cpu.condition(1){pc=0x10fa5084;continue;}pc=0x10fa527c;continue;
}
0x10fa527c=>{
// 0x10fa527c lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10fa5280 mov w10, #5
cpu.x[10]=(((0x5u64))) as u32 as u64;
// 0x10fa5284 cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa5288 csel w1, w10, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[10] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10fa528c cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa5290 b.ge #0x10fa5084
if cpu.condition(10){pc=0x10fa5084;continue;}pc=0x10fa5294;continue;
}
0x10fa5294=>{
// 0x10fa5294 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fa5298 mov w21, w14
cpu.x[21]=(((cpu.x[14] as u32 as u64))) as u32 as u64;
// 0x10fa529c bl #0x1078f4d4
cpu.x[30]=0x10fa52a0;super::chunk2::f_1078f4d4(cpu,rt,0x1078f4d4)?;
// 0x10fa52a0 ldr w8, [x25]
{let address=(cpu.x[25]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fa52a4 mov w14, w21
cpu.x[14]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10fa52a8 b #0x10fa5084
pc=0x10fa5084;continue;
}
0x10fa52ac=>{
// 0x10fa52ac ldr x0, [x23, #0x10]
{let address=(cpu.x[23]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fa52b0 bl #0x10e7f0ec
cpu.x[30]=0x10fa52b4;super::chunk9::f_10e7f0ec(cpu,rt,0x10e7f0ec)?;
pc=0x10fa52b4;continue;
}
0x10fa52b4=>{
// 0x10fa52b4 tbz w0, #0, #0x10fa5394
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fa5394;continue;}pc=0x10fa52b8;continue;
}
0x10fa52b8=>{
// 0x10fa52b8 ldr x10, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10fa52bc ldrsw x13, [x19, #0xc]
{let address=(cpu.x[19]).wrapping_add(0xcu64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[13]=(v0) as u64;}
// 0x10fa52c0 ldr x23, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10fa52c4 add x8, x10, x24, lsl #2
{let a=((cpu.x[10]));let b=(((cpu.x[24])) as u64).wrapping_shl(2) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fa52c8 sub w9, w13, #1
{let a=((cpu.x[13] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fa52cc cmp x27, x13
{let a=((cpu.x[27]));let b=((cpu.x[13]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fa52d0 ldr w14, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[14]=(v0) as u32 as u64;}
// 0x10fa52d4 str w9, [x19, #0xc]
{let address=(cpu.x[19]).wrapping_add(0xcu64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fa52d8 b.ge #0x10fa5354
if cpu.condition(10){pc=0x10fa5354;continue;}pc=0x10fa52dc;continue;
}
0x10fa52dc=>{
// 0x10fa52dc sub w9, w13, w27
{let a=((cpu.x[13] as u32 as u64));let b=((cpu.x[27] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fa52e0 cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa52e4 b.lt #0x10fa5354
if cpu.condition(11){pc=0x10fa5354;continue;}pc=0x10fa52e8;continue;
}
0x10fa52e8=>{
// 0x10fa52e8 add w11, w13, w21
{let a=((cpu.x[13] as u32 as u64));let b=((cpu.x[21] as u32 as u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u32 as u64;}
// 0x10fa52ec cmp w11, #7
{let a=((cpu.x[11] as u32 as u64));let b=((0x7u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa52f0 b.lo #0x10fa5334
if cpu.condition(3){pc=0x10fa5334;continue;}pc=0x10fa52f4;continue;
}
0x10fa52f4=>{
// 0x10fa52f4 add x11, x11, #1
{let a=((cpu.x[11]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
// 0x10fa52f8 add w13, w13, w22
{let a=((cpu.x[13] as u32 as u64));let b=((cpu.x[22] as u32 as u64));let result=a.wrapping_add(b);cpu.x[13]=(result) as u32 as u64;}
// 0x10fa52fc add x10, x10, x26
{let a=((cpu.x[10]));let b=((cpu.x[26]));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x10fa5300 and x12, x11, #0x1fffffff8
{let a=((cpu.x[11]));let b=((0x1fffffff8u64));let result=a & b;cpu.x[12]=(result) as u64;}
// 0x10fa5304 add x13, x13, #1
{let a=((cpu.x[13]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[13]=(result) as u64;}
// 0x10fa5308 add x8, x8, x12, lsl #2
{let a=((cpu.x[8]));let b=(((cpu.x[12])) as u64).wrapping_shl(2) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fa530c sub w9, w9, w12
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[12] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fa5310 and x13, x13, #0x1fffffff8
{let a=((cpu.x[13]));let b=((0x1fffffff8u64));let result=a & b;cpu.x[13]=(result) as u64;}
pc=0x10fa5314;continue;
}
0x10fa5314=>{
// 0x10fa5314 ldp q0, q1, [x10]
{let address=(cpu.x[10]).wrapping_add(0x0u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));let v1=rt.memory.read::<u128>(address.wrapping_add(16));cpu.q[0]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;cpu.q[1]=(v1) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x10fa5318 subs x13, x13, #8
{let a=((cpu.x[13]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.x[13]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x10fa531c stur q0, [x10, #-4]
{let address=(cpu.x[10]).wrapping_add(0xfffffffffffffffcu64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10fa5320 stur q1, [x10, #0xc]
{let address=(cpu.x[10]).wrapping_add(0xcu64);let v0=(cpu.q[1] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10fa5324 add x10, x10, #0x20
{let a=((cpu.x[10]));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x10fa5328 b.ne #0x10fa5314
if cpu.condition(1){pc=0x10fa5314;continue;}pc=0x10fa532c;continue;
}
0x10fa532c=>{
// 0x10fa532c cmp x11, x12
{let a=((cpu.x[11]));let b=((cpu.x[12]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fa5330 b.eq #0x10fa5354
if cpu.condition(0){pc=0x10fa5354;continue;}pc=0x10fa5334;continue;
}
0x10fa5334=>{
// 0x10fa5334 add w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fa5338 add x8, x8, #4
{let a=((cpu.x[8]));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
pc=0x10fa533c;continue;
}
0x10fa533c=>{
// 0x10fa533c ldr w10, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10fa5340 sub w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fa5344 cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa5348 stur w10, [x8, #-4]
{let address=(cpu.x[8]).wrapping_add(0xfffffffffffffffcu64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fa534c add x8, x8, #4
{let a=((cpu.x[8]));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fa5350 b.hi #0x10fa533c
if cpu.condition(8){pc=0x10fa533c;continue;}pc=0x10fa5354;continue;
}
0x10fa5354=>{
// 0x10fa5354 ldp w9, w8, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10fa5358 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa535c b.ne #0x10fa539c
if cpu.condition(1){pc=0x10fa539c;continue;}pc=0x10fa5360;continue;
}
0x10fa5360=>{
// 0x10fa5360 lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10fa5364 mov w10, #5
cpu.x[10]=(((0x5u64))) as u32 as u64;
// 0x10fa5368 cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa536c csel w1, w10, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[10] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10fa5370 cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa5374 b.ge #0x10fa539c
if cpu.condition(10){pc=0x10fa539c;continue;}pc=0x10fa5378;continue;
}
0x10fa5378=>{
// 0x10fa5378 sub x0, x29, #0x18
{let a=((cpu.x[29]));let b=((0x18u64));let result=a.wrapping_sub(b);cpu.x[0]=(result) as u64;}
// 0x10fa537c sub x21, x29, #0x18
{let a=((cpu.x[29]));let b=((0x18u64));let result=a.wrapping_sub(b);cpu.x[21]=(result) as u64;}
// 0x10fa5380 str w14, [sp, #4]
{let address=(cpu.sp).wrapping_add(0x4u64);let v0=cpu.x[14] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fa5384 bl #0x1078f4d4
cpu.x[30]=0x10fa5388;super::chunk2::f_1078f4d4(cpu,rt,0x1078f4d4)?;
// 0x10fa5388 ldr w14, [sp, #4]
{let address=(cpu.sp).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[14]=(v0) as u32 as u64;}
// 0x10fa538c ldur w8, [x29, #-0xc]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fa5390 b #0x10fa53a0
pc=0x10fa53a0;continue;
}
0x10fa5394=>{
// 0x10fa5394 ldr x23, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10fa5398 b #0x10fa509c
pc=0x10fa509c;continue;
}
0x10fa539c=>{
// 0x10fa539c sub x21, x29, #0x18
{let a=((cpu.x[29]));let b=((0x18u64));let result=a.wrapping_sub(b);cpu.x[21]=(result) as u64;}
pc=0x10fa53a0;continue;
}
0x10fa53a0=>{
// 0x10fa53a0 ldr x9, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fa53a4 b #0x10fa508c
pc=0x10fa508c;continue;
}
0x10fa53a8=>{
// 0x10fa53a8 ldr x0, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fa53ac mov w1, #0xfffffff
cpu.x[1]=(((0xfffffffu64))) as u32 as u64;
// 0x10fa53b0 bl #0x10d33644
cpu.x[30]=0x10fa53b4;super::chunk7::f_10d33644(cpu,rt,0x10d33644)?;
pc=0x10fa53b4;continue;
}
0x10fa53b4=>{
// 0x10fa53b4 mov w23, w0
cpu.x[23]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fa53b8 ldr x0, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fa53bc bl #0x10d22e78
cpu.x[30]=0x10fa53c0;super::chunk0::f_10d22e78(cpu,rt,0x10d22e78)?;
pc=0x10fa53c0;continue;
}
0x10fa53c0=>{
// 0x10fa53c0 bl #0x112942c0
cpu.x[30]=0x10fa53c4;super::chunk6::f_112942c0(cpu,rt,0x112942c0)?;
pc=0x10fa53c4;continue;
}
0x10fa53c4=>{
// 0x10fa53c4 mov w8, w0
cpu.x[8]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fa53c8 add x0, sp, #0x40
{let a=((cpu.sp));let b=((0x40u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fa53cc add w1, w8, w23
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[23] as u32 as u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u32 as u64;}
// 0x10fa53d0 bl #0x1129497c
cpu.x[30]=0x10fa53d4;super::chunk15::f_1129497c(cpu,rt,0x1129497c)?;
pc=0x10fa53d4;continue;
}
0x10fa53d4=>{
// 0x10fa53d4 ldr w1, [x19, #0xc]
{let address=(cpu.x[19]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10fa53d8 cbz w1, #0x10fa5744
if (cpu.x[1] as u32 as u64) == 0 {pc=0x10fa5744;continue;}pc=0x10fa53dc;continue;
}
0x10fa53dc=>{
// 0x10fa53dc ldr w8, [x25]
{let address=(cpu.x[25]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fa53e0 cmp w8, #3
{let a=((cpu.x[8] as u32 as u64));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa53e4 b.gt #0x10fa5730
if cpu.condition(12){pc=0x10fa5730;continue;}pc=0x10fa53e8;continue;
}
0x10fa53e8=>{
// 0x10fa53e8 ldr x23, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10fa53ec mov w21, #5
cpu.x[21]=(((0x5u64))) as u32 as u64;
pc=0x10fa53f0;continue;
}
0x10fa53f0=>{
// 0x10fa53f0 add x0, sp, #0x40
{let a=((cpu.sp));let b=((0x40u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fa53f4 bl #0x112949ec
cpu.x[30]=0x10fa53f8;super::chunk9::f_112949ec(cpu,rt,0x112949ec)?;
pc=0x10fa53f8;continue;
}
0x10fa53f8=>{
// 0x10fa53f8 ldr x10, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10fa53fc ldr w11, [x19, #0xc]
{let address=(cpu.x[19]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
pc=0x10fa5400;continue;
}
0x10fa5400=>{
// 0x10fa5400 add x8, x10, w0, sxtw #2
{let a=((cpu.x[10]));let b=((((cpu.x[0] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fa5404 sub w12, w11, #1
{let a=((cpu.x[11] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[12]=(result) as u32 as u64;}
// 0x10fa5408 subs w9, w12, w0
{let a=((cpu.x[12] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10fa540c ldr w22, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[22]=(v0) as u32 as u64;}
// 0x10fa5410 str w12, [x19, #0xc]
{let address=(cpu.x[19]).wrapping_add(0xcu64);let v0=cpu.x[12] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fa5414 b.le #0x10fa5490
if cpu.condition(13){pc=0x10fa5490;continue;}pc=0x10fa5418;continue;
}
0x10fa5418=>{
// 0x10fa5418 cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa541c b.lt #0x10fa5490
if cpu.condition(11){pc=0x10fa5490;continue;}pc=0x10fa5420;continue;
}
0x10fa5420=>{
// 0x10fa5420 sxtw x13, w0
cpu.x[13]=((((cpu.x[0] as u32 as u64))) as i32 as i64 as u64) as u64;
// 0x10fa5424 sub w11, w11, w13
{let a=((cpu.x[11] as u32 as u64));let b=((cpu.x[13] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[11]=(result) as u32 as u64;}
// 0x10fa5428 sub w11, w11, #2
{let a=((cpu.x[11] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.x[11]=(result) as u32 as u64;}
// 0x10fa542c cmp w11, #7
{let a=((cpu.x[11] as u32 as u64));let b=((0x7u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa5430 b.lo #0x10fa5470
if cpu.condition(3){pc=0x10fa5470;continue;}pc=0x10fa5434;continue;
}
0x10fa5434=>{
// 0x10fa5434 add x11, x11, #1
{let a=((cpu.x[11]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
// 0x10fa5438 add x10, x10, x13, lsl #2
{let a=((cpu.x[10]));let b=(((cpu.x[13])) as u64).wrapping_shl(2) as u64;let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x10fa543c and x12, x11, #0x1fffffff8
{let a=((cpu.x[11]));let b=((0x1fffffff8u64));let result=a & b;cpu.x[12]=(result) as u64;}
// 0x10fa5440 add x8, x8, x12, lsl #2
{let a=((cpu.x[8]));let b=(((cpu.x[12])) as u64).wrapping_shl(2) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fa5444 sub w9, w9, w12
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[12] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fa5448 add x10, x10, #0x14
{let a=((cpu.x[10]));let b=((0x14u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x10fa544c mov x13, x12
cpu.x[13]=(((cpu.x[12]))) as u64;
pc=0x10fa5450;continue;
}
0x10fa5450=>{
// 0x10fa5450 ldp q0, q1, [x10, #-0x10]
{let address=(cpu.x[10]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));let v1=rt.memory.read::<u128>(address.wrapping_add(16));cpu.q[0]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;cpu.q[1]=(v1) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x10fa5454 subs x13, x13, #8
{let a=((cpu.x[13]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.x[13]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x10fa5458 stur q0, [x10, #-0x14]
{let address=(cpu.x[10]).wrapping_add(0xffffffffffffffecu64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10fa545c stur q1, [x10, #-4]
{let address=(cpu.x[10]).wrapping_add(0xfffffffffffffffcu64);let v0=(cpu.q[1] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10fa5460 add x10, x10, #0x20
{let a=((cpu.x[10]));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x10fa5464 b.ne #0x10fa5450
if cpu.condition(1){pc=0x10fa5450;continue;}pc=0x10fa5468;continue;
}
0x10fa5468=>{
// 0x10fa5468 cmp x11, x12
{let a=((cpu.x[11]));let b=((cpu.x[12]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fa546c b.eq #0x10fa5490
if cpu.condition(0){pc=0x10fa5490;continue;}pc=0x10fa5470;continue;
}
0x10fa5470=>{
// 0x10fa5470 add w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fa5474 add x8, x8, #4
{let a=((cpu.x[8]));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
pc=0x10fa5478;continue;
}
0x10fa5478=>{
// 0x10fa5478 ldr w10, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10fa547c sub w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fa5480 cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa5484 stur w10, [x8, #-4]
{let address=(cpu.x[8]).wrapping_add(0xfffffffffffffffcu64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fa5488 add x8, x8, #4
{let a=((cpu.x[8]));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fa548c b.hi #0x10fa5478
if cpu.condition(8){pc=0x10fa5478;continue;}pc=0x10fa5490;continue;
}
0x10fa5490=>{
// 0x10fa5490 ldp w9, w8, [x20, #0x20]
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10fa5494 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa5498 b.ne #0x10fa54bc
if cpu.condition(1){pc=0x10fa54bc;continue;}pc=0x10fa549c;continue;
}
0x10fa549c=>{
// 0x10fa549c lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10fa54a0 cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa54a4 csel w1, w21, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[21] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10fa54a8 cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa54ac b.ge #0x10fa54bc
if cpu.condition(10){pc=0x10fa54bc;continue;}pc=0x10fa54b0;continue;
}
0x10fa54b0=>{
// 0x10fa54b0 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fa54b4 bl #0x1078f4d4
cpu.x[30]=0x10fa54b8;super::chunk2::f_1078f4d4(cpu,rt,0x1078f4d4)?;
// 0x10fa54b8 ldr w8, [x25]
{let address=(cpu.x[25]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
pc=0x10fa54bc;continue;
}
0x10fa54bc=>{
// 0x10fa54bc ldr x9, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fa54c0 add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10fa54c4 str w10, [x25]
{let address=(cpu.x[25]).wrapping_add(0x0u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fa54c8 str w22, [x9, w8, sxtw #2]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=cpu.x[22] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fa54cc ldr w1, [x19, #0xc]
{let address=(cpu.x[19]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10fa54d0 cbz w1, #0x10fa54e0
if (cpu.x[1] as u32 as u64) == 0 {pc=0x10fa54e0;continue;}pc=0x10fa54d4;continue;
}
0x10fa54d4=>{
// 0x10fa54d4 ldr w8, [x25]
{let address=(cpu.x[25]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fa54d8 cmp w8, #4
{let a=((cpu.x[8] as u32 as u64));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa54dc b.lt #0x10fa53f0
if cpu.condition(11){pc=0x10fa53f0;continue;}pc=0x10fa54e0;continue;
}
0x10fa54e0=>{
// 0x10fa54e0 ldur w21, [x29, #-0xc]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[21]=(v0) as u32 as u64;}
// 0x10fa54e4 cmp w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa54e8 b.lt #0x10fa5608
if cpu.condition(11){pc=0x10fa5608;continue;}pc=0x10fa54ec;continue;
}
0x10fa54ec=>{
// 0x10fa54ec mvn w23, w21
cpu.x[23]=(!(((cpu.x[21] as u32 as u64)))) as u32 as u64;
// 0x10fa54f0 lsl x24, x21, #2
cpu.x[24]=(((((cpu.x[21]))) as u64).wrapping_shl(((((0x2u64))) & 63) as u32) as u64) as u64;
// 0x10fa54f4 mov w22, wzr
cpu.x[22]=(((0u64))) as u32 as u64;
// 0x10fa54f8 mov w14, #5
cpu.x[14]=(((0x5u64))) as u32 as u64;
// 0x10fa54fc mov w27, w23
cpu.x[27]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10fa5500 b #0x10fa552c
pc=0x10fa552c;continue;
}
0x10fa5504=>{
// 0x10fa5504 ldr x9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fa5508 add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10fa550c add w22, w22, #1
{let a=((cpu.x[22] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[22]=(result) as u32 as u64;}
// 0x10fa5510 cmp x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fa5514 add w27, w27, #1
{let a=((cpu.x[27] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[27]=(result) as u32 as u64;}
// 0x10fa5518 sub x24, x24, #4
{let a=((cpu.x[24]));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.x[24]=(result) as u64;}
// 0x10fa551c mov x21, x28
cpu.x[21]=(((cpu.x[28]))) as u64;
// 0x10fa5520 str w10, [x19, #0xc]
{let address=(cpu.x[19]).wrapping_add(0xcu64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fa5524 str w26, [x9, w8, sxtw #2]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=cpu.x[26] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fa5528 b.le #0x10fa5600
if cpu.condition(13){pc=0x10fa5600;continue;}pc=0x10fa552c;continue;
}
0x10fa552c=>{
// 0x10fa552c ldur x10, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10fa5530 sub x28, x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[28]=(result) as u64;}
// 0x10fa5534 ldursw x13, [x29, #-0xc]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff4u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[13]=(v0) as u64;}
// 0x10fa5538 add x8, x10, x28, lsl #2
{let a=((cpu.x[10]));let b=(((cpu.x[28])) as u64).wrapping_shl(2) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fa553c sub w9, w13, #1
{let a=((cpu.x[13] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fa5540 cmp x21, x13
{let a=((cpu.x[21]));let b=((cpu.x[13]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fa5544 ldr w26, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[26]=(v0) as u32 as u64;}
// 0x10fa5548 stur w9, [x29, #-0xc]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff4u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fa554c b.ge #0x10fa55cc
if cpu.condition(10){pc=0x10fa55cc;continue;}pc=0x10fa5550;continue;
}
0x10fa5550=>{
// 0x10fa5550 sub w9, w13, w21
{let a=((cpu.x[13] as u32 as u64));let b=((cpu.x[21] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fa5554 cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa5558 b.lt #0x10fa55cc
if cpu.condition(11){pc=0x10fa55cc;continue;}pc=0x10fa555c;continue;
}
0x10fa555c=>{
// 0x10fa555c add w11, w22, w23
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[23] as u32 as u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u32 as u64;}
// 0x10fa5560 add w11, w13, w11
{let a=((cpu.x[13] as u32 as u64));let b=((cpu.x[11] as u32 as u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u32 as u64;}
// 0x10fa5564 cmp w11, #7
{let a=((cpu.x[11] as u32 as u64));let b=((0x7u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa5568 b.lo #0x10fa55ac
if cpu.condition(3){pc=0x10fa55ac;continue;}pc=0x10fa556c;continue;
}
0x10fa556c=>{
// 0x10fa556c add x11, x11, #1
{let a=((cpu.x[11]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
// 0x10fa5570 add w13, w13, w27
{let a=((cpu.x[13] as u32 as u64));let b=((cpu.x[27] as u32 as u64));let result=a.wrapping_add(b);cpu.x[13]=(result) as u32 as u64;}
// 0x10fa5574 add x10, x10, x24
{let a=((cpu.x[10]));let b=((cpu.x[24]));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x10fa5578 and x12, x11, #0x1fffffff8
{let a=((cpu.x[11]));let b=((0x1fffffff8u64));let result=a & b;cpu.x[12]=(result) as u64;}
// 0x10fa557c add x13, x13, #1
{let a=((cpu.x[13]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[13]=(result) as u64;}
// 0x10fa5580 add x8, x8, x12, lsl #2
{let a=((cpu.x[8]));let b=(((cpu.x[12])) as u64).wrapping_shl(2) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fa5584 sub w9, w9, w12
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[12] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fa5588 and x13, x13, #0x1fffffff8
{let a=((cpu.x[13]));let b=((0x1fffffff8u64));let result=a & b;cpu.x[13]=(result) as u64;}
pc=0x10fa558c;continue;
}
0x10fa558c=>{
// 0x10fa558c ldp q0, q1, [x10]
{let address=(cpu.x[10]).wrapping_add(0x0u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));let v1=rt.memory.read::<u128>(address.wrapping_add(16));cpu.q[0]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;cpu.q[1]=(v1) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x10fa5590 subs x13, x13, #8
{let a=((cpu.x[13]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.x[13]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x10fa5594 stur q0, [x10, #-4]
{let address=(cpu.x[10]).wrapping_add(0xfffffffffffffffcu64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10fa5598 stur q1, [x10, #0xc]
{let address=(cpu.x[10]).wrapping_add(0xcu64);let v0=(cpu.q[1] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10fa559c add x10, x10, #0x20
{let a=((cpu.x[10]));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x10fa55a0 b.ne #0x10fa558c
if cpu.condition(1){pc=0x10fa558c;continue;}pc=0x10fa55a4;continue;
}
0x10fa55a4=>{
// 0x10fa55a4 cmp x11, x12
{let a=((cpu.x[11]));let b=((cpu.x[12]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fa55a8 b.eq #0x10fa55cc
if cpu.condition(0){pc=0x10fa55cc;continue;}pc=0x10fa55ac;continue;
}
0x10fa55ac=>{
// 0x10fa55ac add w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fa55b0 add x8, x8, #4
{let a=((cpu.x[8]));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
pc=0x10fa55b4;continue;
}
0x10fa55b4=>{
// 0x10fa55b4 ldr w10, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10fa55b8 sub w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fa55bc cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa55c0 stur w10, [x8, #-4]
{let address=(cpu.x[8]).wrapping_add(0xfffffffffffffffcu64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fa55c4 add x8, x8, #4
{let a=((cpu.x[8]));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fa55c8 b.hi #0x10fa55b4
if cpu.condition(8){pc=0x10fa55b4;continue;}pc=0x10fa55cc;continue;
}
0x10fa55cc=>{
// 0x10fa55cc ldp w9, w8, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10fa55d0 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa55d4 b.ne #0x10fa5504
if cpu.condition(1){pc=0x10fa5504;continue;}pc=0x10fa55d8;continue;
}
0x10fa55d8=>{
// 0x10fa55d8 lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10fa55dc cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa55e0 csel w1, w14, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[14] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10fa55e4 cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa55e8 b.ge #0x10fa5504
if cpu.condition(10){pc=0x10fa5504;continue;}pc=0x10fa55ec;continue;
}
0x10fa55ec=>{
// 0x10fa55ec mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fa55f0 bl #0x1078f4d4
cpu.x[30]=0x10fa55f4;super::chunk2::f_1078f4d4(cpu,rt,0x1078f4d4)?;
// 0x10fa55f4 ldr w8, [x19, #0xc]
{let address=(cpu.x[19]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fa55f8 mov w14, #5
cpu.x[14]=(((0x5u64))) as u32 as u64;
// 0x10fa55fc b #0x10fa5504
pc=0x10fa5504;continue;
}
0x10fa5600=>{
// 0x10fa5600 ldr w1, [x19, #0xc]
{let address=(cpu.x[19]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10fa5604 ldr x23, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
pc=0x10fa5608;continue;
}
0x10fa5608=>{
// 0x10fa5608 cbz w1, #0x10fa56fc
if (cpu.x[1] as u32 as u64) == 0 {pc=0x10fa56fc;continue;}pc=0x10fa560c;continue;
}
0x10fa560c=>{
// 0x10fa560c mov w21, #5
cpu.x[21]=(((0x5u64))) as u32 as u64;
// 0x10fa5610 b #0x10fa562c
pc=0x10fa562c;continue;
}
0x10fa5614=>{
// 0x10fa5614 ldr x9, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fa5618 add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10fa561c str w10, [x25]
{let address=(cpu.x[25]).wrapping_add(0x0u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fa5620 str w22, [x9, w8, sxtw #2]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=cpu.x[22] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fa5624 ldr w1, [x19, #0xc]
{let address=(cpu.x[19]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10fa5628 cbz w1, #0x10fa56fc
if (cpu.x[1] as u32 as u64) == 0 {pc=0x10fa56fc;continue;}pc=0x10fa562c;continue;
}
0x10fa562c=>{
// 0x10fa562c add x0, sp, #0x40
{let a=((cpu.sp));let b=((0x40u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fa5630 bl #0x112949ec
cpu.x[30]=0x10fa5634;super::chunk9::f_112949ec(cpu,rt,0x112949ec)?;
pc=0x10fa5634;continue;
}
0x10fa5634=>{
// 0x10fa5634 ldr x10, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10fa5638 ldr w11, [x19, #0xc]
{let address=(cpu.x[19]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
// 0x10fa563c add x8, x10, w0, sxtw #2
{let a=((cpu.x[10]));let b=((((cpu.x[0] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fa5640 sub w12, w11, #1
{let a=((cpu.x[11] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[12]=(result) as u32 as u64;}
// 0x10fa5644 subs w9, w12, w0
{let a=((cpu.x[12] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10fa5648 ldr w22, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[22]=(v0) as u32 as u64;}
// 0x10fa564c str w12, [x19, #0xc]
{let address=(cpu.x[19]).wrapping_add(0xcu64);let v0=cpu.x[12] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fa5650 b.le #0x10fa56cc
if cpu.condition(13){pc=0x10fa56cc;continue;}pc=0x10fa5654;continue;
}
0x10fa5654=>{
// 0x10fa5654 cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa5658 b.lt #0x10fa56cc
if cpu.condition(11){pc=0x10fa56cc;continue;}pc=0x10fa565c;continue;
}
0x10fa565c=>{
// 0x10fa565c sxtw x13, w0
cpu.x[13]=((((cpu.x[0] as u32 as u64))) as i32 as i64 as u64) as u64;
// 0x10fa5660 sub w11, w11, w13
{let a=((cpu.x[11] as u32 as u64));let b=((cpu.x[13] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[11]=(result) as u32 as u64;}
// 0x10fa5664 sub w11, w11, #2
{let a=((cpu.x[11] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.x[11]=(result) as u32 as u64;}
// 0x10fa5668 cmp w11, #7
{let a=((cpu.x[11] as u32 as u64));let b=((0x7u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa566c b.lo #0x10fa56ac
if cpu.condition(3){pc=0x10fa56ac;continue;}pc=0x10fa5670;continue;
}
0x10fa5670=>{
// 0x10fa5670 add x11, x11, #1
{let a=((cpu.x[11]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
// 0x10fa5674 add x10, x10, x13, lsl #2
{let a=((cpu.x[10]));let b=(((cpu.x[13])) as u64).wrapping_shl(2) as u64;let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x10fa5678 and x12, x11, #0x1fffffff8
{let a=((cpu.x[11]));let b=((0x1fffffff8u64));let result=a & b;cpu.x[12]=(result) as u64;}
// 0x10fa567c add x8, x8, x12, lsl #2
{let a=((cpu.x[8]));let b=(((cpu.x[12])) as u64).wrapping_shl(2) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fa5680 sub w9, w9, w12
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[12] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fa5684 add x10, x10, #0x14
{let a=((cpu.x[10]));let b=((0x14u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x10fa5688 mov x13, x12
cpu.x[13]=(((cpu.x[12]))) as u64;
pc=0x10fa568c;continue;
}
0x10fa568c=>{
// 0x10fa568c ldp q0, q1, [x10, #-0x10]
{let address=(cpu.x[10]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));let v1=rt.memory.read::<u128>(address.wrapping_add(16));cpu.q[0]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;cpu.q[1]=(v1) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x10fa5690 subs x13, x13, #8
{let a=((cpu.x[13]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.x[13]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x10fa5694 stur q0, [x10, #-0x14]
{let address=(cpu.x[10]).wrapping_add(0xffffffffffffffecu64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10fa5698 stur q1, [x10, #-4]
{let address=(cpu.x[10]).wrapping_add(0xfffffffffffffffcu64);let v0=(cpu.q[1] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10fa569c add x10, x10, #0x20
{let a=((cpu.x[10]));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x10fa56a0 b.ne #0x10fa568c
if cpu.condition(1){pc=0x10fa568c;continue;}pc=0x10fa56a4;continue;
}
0x10fa56a4=>{
// 0x10fa56a4 cmp x11, x12
{let a=((cpu.x[11]));let b=((cpu.x[12]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fa56a8 b.eq #0x10fa56cc
if cpu.condition(0){pc=0x10fa56cc;continue;}pc=0x10fa56ac;continue;
}
0x10fa56ac=>{
// 0x10fa56ac add w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fa56b0 add x8, x8, #4
{let a=((cpu.x[8]));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
pc=0x10fa56b4;continue;
}
0x10fa56b4=>{
// 0x10fa56b4 ldr w10, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10fa56b8 sub w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fa56bc cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa56c0 stur w10, [x8, #-4]
{let address=(cpu.x[8]).wrapping_add(0xfffffffffffffffcu64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fa56c4 add x8, x8, #4
{let a=((cpu.x[8]));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fa56c8 b.hi #0x10fa56b4
if cpu.condition(8){pc=0x10fa56b4;continue;}pc=0x10fa56cc;continue;
}
0x10fa56cc=>{
// 0x10fa56cc ldp w9, w8, [x20, #0x20]
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10fa56d0 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa56d4 b.ne #0x10fa5614
if cpu.condition(1){pc=0x10fa5614;continue;}pc=0x10fa56d8;continue;
}
0x10fa56d8=>{
// 0x10fa56d8 lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10fa56dc cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa56e0 csel w1, w21, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[21] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10fa56e4 cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa56e8 b.ge #0x10fa5614
if cpu.condition(10){pc=0x10fa5614;continue;}pc=0x10fa56ec;continue;
}
0x10fa56ec=>{
// 0x10fa56ec mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fa56f0 bl #0x1078f4d4
cpu.x[30]=0x10fa56f4;super::chunk2::f_1078f4d4(cpu,rt,0x1078f4d4)?;
// 0x10fa56f4 ldr w8, [x25]
{let address=(cpu.x[25]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fa56f8 b #0x10fa5614
pc=0x10fa5614;continue;
}
0x10fa56fc=>{
// 0x10fa56fc add x0, sp, #0x40
{let a=((cpu.sp));let b=((0x40u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fa5700 bl #0x112949e0
cpu.x[30]=0x10fa5704;super::chunk11::f_112949e0(cpu,rt,0x112949e0)?;
pc=0x10fa5704;continue;
}
0x10fa5704=>{
// 0x10fa5704 ldur x0, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fa5708 cbz x0, #0x10fa5710
if (cpu.x[0]) == 0 {pc=0x10fa5710;continue;}pc=0x10fa570c;continue;
}
0x10fa570c=>{
// 0x10fa570c bl #0x1187d360
cpu.x[30]=0x10fa5710;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x10fa5710;continue;
}
0x10fa5710=>{
// 0x10fa5710 add sp, sp, #0xa20
{let a=((cpu.sp));let b=((0xa20u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fa5714 ldp x20, x19, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fa5718 ldp x22, x21, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fa571c ldp x24, x23, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fa5720 ldp x26, x25, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fa5724 ldp x28, x27, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fa5728 ldp x29, x30, [sp], #0x60
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x60u64)))) as u64;}
// 0x10fa572c ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10fa5730=>{
// 0x10fa5730 ldr x23, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10fa5734 ldur w21, [x29, #-0xc]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[21]=(v0) as u32 as u64;}
// 0x10fa5738 cmp w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa573c b.ge #0x10fa54ec
if cpu.condition(10){pc=0x10fa54ec;continue;}pc=0x10fa5740;continue;
}
0x10fa5740=>{
// 0x10fa5740 b #0x10fa5608
pc=0x10fa5608;continue;
}
0x10fa5744=>{
// 0x10fa5744 ldr x23, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10fa5748 ldur w21, [x29, #-0xc]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[21]=(v0) as u32 as u64;}
// 0x10fa574c cmp w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa5750 b.ge #0x10fa54ec
if cpu.condition(10){pc=0x10fa54ec;continue;}pc=0x10fa5754;continue;
}
0x10fa5754=>{
// 0x10fa5754 b #0x10fa5608
pc=0x10fa5608;continue;
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fa4fe4")),}}}
#[inline]
pub(super) unsafe fn f_10fadf5c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fadf5c=>{
// 0x10fadf5c ldr w8, [x1, #8]
{let address=(cpu.x[1]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fadf60 mov w9, #0xcc21
cpu.x[9]=(((0xcc21u64))) as u32 as u64;
// 0x10fadf64 movk w9, #0xc9d, lsl #16
cpu.x[9]=((cpu.x[9] as u32 as u64 & !(0xffff0000u64)) | 0xc9d0000u64) as u32 as u64;
// 0x10fadf68 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fadf6c b.eq #0x10fadf88
if cpu.condition(0){pc=0x10fadf88;continue;}pc=0x10fadf70;continue;
}
0x10fadf70=>{
// 0x10fadf70 mov w9, #0xcc20
cpu.x[9]=(((0xcc20u64))) as u32 as u64;
// 0x10fadf74 movk w9, #0xc9d, lsl #16
cpu.x[9]=((cpu.x[9] as u32 as u64 & !(0xffff0000u64)) | 0xc9d0000u64) as u32 as u64;
// 0x10fadf78 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fadf7c b.ne #0x10fadfb4
if cpu.condition(1){pc=0x10fadfb4;continue;}pc=0x10fadf80;continue;
}
0x10fadf80=>{
// 0x10fadf80 sub x0, x0, #0x60
{let a=((cpu.x[0]));let b=((0x60u64));let result=a.wrapping_sub(b);cpu.x[0]=(result) as u64;}
// 0x10fadf84 b #0x10fad358
super::chunk10::f_10fad358(cpu,rt,0x10fad358)?;return Ok(());
}
0x10fadf88=>{
// 0x10fadf88 ldr x8, [x0, #0x30]
{let address=(cpu.x[0]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fadf8c str wzr, [x0, #0x18]
{let address=(cpu.x[0]).wrapping_add(0x18u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fadf90 cbz x8, #0x10fadfb4
if (cpu.x[8]) == 0 {pc=0x10fadfb4;continue;}pc=0x10fadf94;continue;
}
0x10fadf94=>{
// 0x10fadf94 ldr x8, [x8, #0x710]
{let address=(cpu.x[8]).wrapping_add(0x710u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fadf98 ldr w8, [x8, #0xb0]
{let address=(cpu.x[8]).wrapping_add(0xb0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fadf9c cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fadfa0 b.lt #0x10fadfb4
if cpu.condition(11){pc=0x10fadfb4;continue;}pc=0x10fadfa4;continue;
}
0x10fadfa4=>{
// 0x10fadfa4 ldr w9, [x0, #0x20]
{let address=(cpu.x[0]).wrapping_add(0x20u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fadfa8 cmp w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fadfac b.ge #0x10fadfb4
if cpu.condition(10){pc=0x10fadfb4;continue;}pc=0x10fadfb0;continue;
}
0x10fadfb0=>{
// 0x10fadfb0 str w8, [x0, #0x20]
{let address=(cpu.x[0]).wrapping_add(0x20u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10fadfb4;continue;
}
0x10fadfb4=>{
// 0x10fadfb4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fadf5c")),}}}
#[inline]
pub(super) unsafe fn f_10fbc9a8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fbc9a8=>{
// 0x10fbc9a8 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fbc9ac str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fbc9b0 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fbc9b4 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10fbc9b8 bl #0x10f74060
cpu.x[30]=0x10fbc9bc;super::chunk2::f_10f74060(cpu,rt,0x10f74060)?;
pc=0x10fbc9bc;continue;
}
0x10fbc9bc=>{
// 0x10fbc9bc mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fbc9c0 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10fbc9c4 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10fbc9c8 b #0x1187d2c0
super::dispatch(cpu,rt,0x1187d2c0)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fbc9a8")),}}}
#[inline]
pub(super) unsafe fn f_10fbf590(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fbf590=>{
// 0x10fbf590 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fbf594 str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fbf598 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fbf59c mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10fbf5a0 bl #0x10f74060
cpu.x[30]=0x10fbf5a4;super::chunk2::f_10f74060(cpu,rt,0x10f74060)?;
pc=0x10fbf5a4;continue;
}
0x10fbf5a4=>{
// 0x10fbf5a4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fbf5a8 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10fbf5ac ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10fbf5b0 b #0x1187d2c0
super::dispatch(cpu,rt,0x1187d2c0)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fbf590")),}}}
#[inline]
pub(super) unsafe fn f_10fc2c4c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fc2c4c=>{
// 0x10fc2c4c ldr w8, [x0, #0x24]
{let address=(cpu.x[0]).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc2c50 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc2c54 b.lt #0x10fc2ca0
if cpu.condition(11){pc=0x10fc2ca0;continue;}pc=0x10fc2c58;continue;
}
0x10fc2c58=>{
// 0x10fc2c58 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fc2c5c stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc2c60 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fc2c64 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10fc2c68 mov x20, xzr
cpu.x[20]=(((0u64))) as u64;
pc=0x10fc2c6c;continue;
}
0x10fc2c6c=>{
// 0x10fc2c6c ldr x8, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc2c70 ldr x8, [x8, x20, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[20])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc2c74 ldr x0, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fc2c78 bl #0x10ddae4c
cpu.x[30]=0x10fc2c7c;super::chunk1::f_10ddae4c(cpu,rt,0x10ddae4c)?;
pc=0x10fc2c7c;continue;
}
0x10fc2c7c=>{
// 0x10fc2c7c tbnz w0, #0, #0x10fc2c90
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10fc2c90;continue;}pc=0x10fc2c80;continue;
}
0x10fc2c80=>{
// 0x10fc2c80 ldrsw x8, [x19, #0x24]
{let address=(cpu.x[19]).wrapping_add(0x24u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10fc2c84 add x20, x20, #1
{let a=((cpu.x[20]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[20]=(result) as u64;}
// 0x10fc2c88 cmp x20, x8
{let a=((cpu.x[20]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fc2c8c b.lt #0x10fc2c6c
if cpu.condition(11){pc=0x10fc2c6c;continue;}pc=0x10fc2c90;continue;
}
0x10fc2c90=>{
// 0x10fc2c90 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fc2c94 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10fc2c98 and w0, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[0]=(result) as u32 as u64;}
// 0x10fc2c9c ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10fc2ca0=>{
// 0x10fc2ca0 and w0, wzr, #1
{let a=((0u64));let b=((0x1u64));let result=a & b;cpu.x[0]=(result) as u32 as u64;}
// 0x10fc2ca4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fc2c4c")),}}}
#[inline]
pub(super) unsafe fn f_10fc3f74(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fc3f74=>{
// 0x10fc3f74 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fc3f78 stp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc3f7c stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc3f80 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fc3f84 mov w19, w2
cpu.x[19]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x10fc3f88 mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x10fc3f8c mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10fc3f90 bl #0x10fc4afc
cpu.x[30]=0x10fc3f94;super::chunk3::f_10fc4afc(cpu,rt,0x10fc4afc)?;
pc=0x10fc3f94;continue;
}
0x10fc3f94=>{
// 0x10fc3f94 mov w22, w0
cpu.x[22]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fc3f98 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fc3f9c bl #0x10fc4a68
cpu.x[30]=0x10fc3fa0;super::chunk12::f_10fc4a68(cpu,rt,0x10fc4a68)?;
pc=0x10fc3fa0;continue;
}
0x10fc3fa0=>{
// 0x10fc3fa0 ldr x8, [x21, #8]
{let address=(cpu.x[21]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc3fa4 mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fc3fa8 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10fc3fac bl #0x10f942c0
cpu.x[30]=0x10fc3fb0;super::chunk2::f_10f942c0(cpu,rt,0x10f942c0)?;
pc=0x10fc3fb0;continue;
}
0x10fc3fb0=>{
// 0x10fc3fb0 mov x1, x0
cpu.x[1]=(((cpu.x[0]))) as u64;
// 0x10fc3fb4 add w2, w22, w19
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_add(b);cpu.x[2]=(result) as u32 as u64;}
// 0x10fc3fb8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fc3fbc mov w3, w21
cpu.x[3]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10fc3fc0 mov w4, wzr
cpu.x[4]=(((0u64))) as u32 as u64;
// 0x10fc3fc4 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fc3fc8 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fc3fcc ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10fc3fd0 b #0x10f958e0
super::chunk6::f_10f958e0(cpu,rt,0x10f958e0)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fc3f74")),}}}
#[inline]
pub(super) unsafe fn f_10fc40ec(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fc40ec=>{
// 0x10fc40ec ldr w8, [x0, #0x48]
{let address=(cpu.x[0]).wrapping_add(0x48u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc40f0 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc40f4 cset w0, gt
cpu.x[0]=((cpu.condition(12)) as u64) as u32 as u64;
// 0x10fc40f8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fc40ec")),}}}
#[inline]
pub(super) unsafe fn f_10fc4240(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fc4240=>{
// 0x10fc4240 stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fc4244 str x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fc4248 stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc424c stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc4250 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fc4254 ldr w8, [x0, #0xc4]
{let address=(cpu.x[0]).wrapping_add(0xc4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc4258 ldr w9, [x0, #0xd4]
{let address=(cpu.x[0]).wrapping_add(0xd4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fc425c cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc4260 csel w21, w8, w9, lt
cpu.x[21]=(if cpu.condition(11){((cpu.x[8] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10fc4264 cbz w21, #0x10fc42f0
if (cpu.x[21] as u32 as u64) == 0 {pc=0x10fc42f0;continue;}pc=0x10fc4268;continue;
}
0x10fc4268=>{
// 0x10fc4268 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10fc426c bl #0x10fd049c
cpu.x[30]=0x10fc4270;super::chunk6::f_10fd049c(cpu,rt,0x10fd049c)?;
// 0x10fc4270 cmp w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc4274 b.lt #0x10fc42e8
if cpu.condition(11){pc=0x10fc42e8;continue;}pc=0x10fc4278;continue;
}
0x10fc4278=>{
// 0x10fc4278 ubfiz x23, x21, #2, #0x20
cpu.x[23]=(((((cpu.x[21])) & 0xffffffffu64) << 2)) as u64;
// 0x10fc427c mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10fc4280 mov x22, xzr
cpu.x[22]=(((0u64))) as u64;
// 0x10fc4284 mov x21, xzr
cpu.x[21]=(((0u64))) as u64;
// 0x10fc4288 b #0x10fc4298
pc=0x10fc4298;continue;
}
0x10fc428c=>{
// 0x10fc428c add x22, x22, #4
{let a=((cpu.x[22]));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[22]=(result) as u64;}
// 0x10fc4290 cmp x23, x22
{let a=((cpu.x[23]));let b=((cpu.x[22]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fc4294 b.eq #0x10fc42c4
if cpu.condition(0){pc=0x10fc42c4;continue;}pc=0x10fc4298;continue;
}
0x10fc4298=>{
// 0x10fc4298 ldr x8, [x19, #0xb8]
{let address=(cpu.x[19]).wrapping_add(0xb8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc429c ldr w8, [x8, x22]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[22])));let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc42a0 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc42a4 b.lt #0x10fc428c
if cpu.condition(11){pc=0x10fc428c;continue;}pc=0x10fc42a8;continue;
}
0x10fc42a8=>{
// 0x10fc42a8 ldr x8, [x19, #0xc8]
{let address=(cpu.x[19]).wrapping_add(0xc8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc42ac mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fc42b0 ldr w1, [x8, x22]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[22])));let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10fc42b4 bl #0x10f8be18
cpu.x[30]=0x10fc42b8;super::chunk5::f_10f8be18(cpu,rt,0x10f8be18)?;
// 0x10fc42b8 cmp x0, #0
{let a=((cpu.x[0]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fc42bc csel x21, x21, x0, eq
cpu.x[21]=(if cpu.condition(0){((cpu.x[21]))}else{((cpu.x[0]))}) as u64;
// 0x10fc42c0 b #0x10fc428c
pc=0x10fc428c;continue;
}
0x10fc42c4=>{
// 0x10fc42c4 cbz x21, #0x10fc42e8
if (cpu.x[21]) == 0 {pc=0x10fc42e8;continue;}pc=0x10fc42c8;continue;
}
0x10fc42c8=>{
// 0x10fc42c8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc42cc mov x1, x21
cpu.x[1]=(((cpu.x[21]))) as u64;
// 0x10fc42d0 mov w2, wzr
cpu.x[2]=(((0u64))) as u32 as u64;
// 0x10fc42d4 mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10fc42d8 mov w4, #1
cpu.x[4]=(((0x1u64))) as u32 as u64;
// 0x10fc42dc mov w20, #1
cpu.x[20]=(((0x1u64))) as u32 as u64;
// 0x10fc42e0 bl #0x10fc3cb4
cpu.x[30]=0x10fc42e4;super::chunk12::f_10fc3cb4(cpu,rt,0x10fc3cb4)?;
// 0x10fc42e4 strb w20, [x19, #0xd8]
{let address=(cpu.x[19]).wrapping_add(0xd8u64);let v0=cpu.x[20] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
pc=0x10fc42e8;continue;
}
0x10fc42e8=>{
// 0x10fc42e8 str wzr, [x19, #0xc4]
{let address=(cpu.x[19]).wrapping_add(0xc4u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc42ec str wzr, [x19, #0xd4]
{let address=(cpu.x[19]).wrapping_add(0xd4u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10fc42f0;continue;
}
0x10fc42f0=>{
// 0x10fc42f0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fc42f4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10fc42f8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fc42fc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10fc4300 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fc4240")),}}}
#[inline]
pub(super) unsafe fn f_10fc96e8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fc96e8=>{
// 0x10fc96e8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fc96e8")),}}}
#[inline]
pub(super) unsafe fn f_10fc9ea8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fc9ea8=>{
// 0x10fc9ea8 stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fc9eac str x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fc9eb0 stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc9eb4 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc9eb8 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fc9ebc mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10fc9ec0 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10fc9ec4 bl #0x10fc96e8
cpu.x[30]=0x10fc9ec8;super::chunk15::f_10fc96e8(cpu,rt,0x10fc96e8)?;
pc=0x10fc9ec8;continue;
}
0x10fc9ec8=>{
// 0x10fc9ec8 ldr x0, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fc9ecc bl #0x10f7e050
cpu.x[30]=0x10fc9ed0;super::chunk10::f_10f7e050(cpu,rt,0x10f7e050)?;
pc=0x10fc9ed0;continue;
}
0x10fc9ed0=>{
// 0x10fc9ed0 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9ed4 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10fc9ed8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc9edc ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9ee0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc9ee4;super::dispatch(cpu,rt,target)?;}
pc=0x10fc9ee4;continue;
}
0x10fc9ee4=>{
// 0x10fc9ee4 str w0, [x20, #0x1c]
{let address=(cpu.x[20]).wrapping_add(0x1cu64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc9ee8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fc9eec bl #0x10fc9774
cpu.x[30]=0x10fc9ef0;super::chunk14::f_10fc9774(cpu,rt,0x10fc9774)?;
pc=0x10fc9ef0;continue;
}
0x10fc9ef0=>{
// 0x10fc9ef0 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9ef4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc9ef8 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9efc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc9f00;super::dispatch(cpu,rt,target)?;}
pc=0x10fc9f00;continue;
}
0x10fc9f00=>{
// 0x10fc9f00 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9f04 str w0, [x20, #0x10]
{let address=(cpu.x[20]).wrapping_add(0x10u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc9f08 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc9f0c ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9f10 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc9f14;super::dispatch(cpu,rt,target)?;}
pc=0x10fc9f14;continue;
}
0x10fc9f14=>{
// 0x10fc9f14 ldr w8, [x20, #0x38]
{let address=(cpu.x[20]).wrapping_add(0x38u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc9f18 str w0, [x20, #0x14]
{let address=(cpu.x[20]).wrapping_add(0x14u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc9f1c cbz w8, #0x10fc9f34
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10fc9f34;continue;}pc=0x10fc9f20;continue;
}
0x10fc9f20=>{
// 0x10fc9f20 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9f24 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc9f28 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9f2c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc9f30;super::dispatch(cpu,rt,target)?;}
pc=0x10fc9f30;continue;
}
0x10fc9f30=>{
// 0x10fc9f30 b #0x10fc9f38
pc=0x10fc9f38;continue;
}
0x10fc9f34=>{
// 0x10fc9f34 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
pc=0x10fc9f38;continue;
}
0x10fc9f38=>{
// 0x10fc9f38 str w0, [x20, #0x34]
{let address=(cpu.x[20]).wrapping_add(0x34u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc9f3c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc9f40 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9f44 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9f48 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc9f4c;super::dispatch(cpu,rt,target)?;}
pc=0x10fc9f4c;continue;
}
0x10fc9f4c=>{
// 0x10fc9f4c ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9f50 str w0, [x20, #0x2c]
{let address=(cpu.x[20]).wrapping_add(0x2cu64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc9f54 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc9f58 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9f5c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc9f60;super::dispatch(cpu,rt,target)?;}
pc=0x10fc9f60;continue;
}
0x10fc9f60=>{
// 0x10fc9f60 str w0, [x20, #0x30]
{let address=(cpu.x[20]).wrapping_add(0x30u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc9f64 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fc9f68 bl #0x10ddff74
cpu.x[30]=0x10fc9f6c;super::chunk11::f_10ddff74(cpu,rt,0x10ddff74)?;
pc=0x10fc9f6c;continue;
}
0x10fc9f6c=>{
// 0x10fc9f6c tbnz w0, #0, #0x10fca16c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10fca16c;continue;}pc=0x10fc9f70;continue;
}
0x10fc9f70=>{
// 0x10fc9f70 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fc9f74 bl #0x10de0ba8
cpu.x[30]=0x10fc9f78;super::chunk12::f_10de0ba8(cpu,rt,0x10de0ba8)?;
pc=0x10fc9f78;continue;
}
0x10fc9f78=>{
// 0x10fc9f78 tbnz w0, #0, #0x10fca18c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10fca18c;continue;}pc=0x10fc9f7c;continue;
}
0x10fc9f7c=>{
// 0x10fc9f7c ldr w8, [x21, #0x674]
{let address=(cpu.x[21]).wrapping_add(0x674u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc9f80 cbz w8, #0x10fc9f98
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10fc9f98;continue;}pc=0x10fc9f84;continue;
}
0x10fc9f84=>{
// 0x10fc9f84 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9f88 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc9f8c ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9f90 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc9f94;super::dispatch(cpu,rt,target)?;}
// 0x10fc9f94 str w0, [x20, #0x44]
{let address=(cpu.x[20]).wrapping_add(0x44u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10fc9f98;continue;
}
0x10fc9f98=>{
// 0x10fc9f98 ldr x8, [x21, #0x638]
{let address=(cpu.x[21]).wrapping_add(0x638u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9f9c cbz x8, #0x10fca078
if (cpu.x[8]) == 0 {pc=0x10fca078;continue;}pc=0x10fc9fa0;continue;
}
0x10fc9fa0=>{
// 0x10fc9fa0 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9fa4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc9fa8 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc9fac blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fc9fb0;super::dispatch(cpu,rt,target)?;}
pc=0x10fc9fb0;continue;
}
0x10fc9fb0=>{
// 0x10fc9fb0 ldr w8, [x20, #0x54]
{let address=(cpu.x[20]).wrapping_add(0x54u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc9fb4 mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fc9fb8 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc9fbc b.lt #0x10fc9fec
if cpu.condition(11){pc=0x10fc9fec;continue;}pc=0x10fc9fc0;continue;
}
0x10fc9fc0=>{
// 0x10fc9fc0 mov x22, xzr
cpu.x[22]=(((0u64))) as u64;
// 0x10fc9fc4 b #0x10fc9fd4
pc=0x10fc9fd4;continue;
}
0x10fc9fc8=>{
// 0x10fc9fc8 add x22, x22, #1
{let a=((cpu.x[22]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[22]=(result) as u64;}
// 0x10fc9fcc cmp x22, w8, sxtw
{let a=((cpu.x[22]));let b=(((cpu.x[8] as u32 as u64) as i32 as i64 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fc9fd0 b.ge #0x10fc9fec
if cpu.condition(10){pc=0x10fc9fec;continue;}pc=0x10fc9fd4;continue;
}
0x10fc9fd4=>{
// 0x10fc9fd4 ldur x9, [x20, #0x48]
{let address=(cpu.x[20]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fc9fd8 ldr x0, [x9, x22, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[22])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fc9fdc cbz x0, #0x10fc9fc8
if (cpu.x[0]) == 0 {pc=0x10fc9fc8;continue;}pc=0x10fc9fe0;continue;
}
0x10fc9fe0=>{
// 0x10fc9fe0 bl #0x1187d2c0
cpu.x[30]=0x10fc9fe4;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10fc9fe4;continue;
}
0x10fc9fe4=>{
// 0x10fc9fe4 ldr w8, [x20, #0x54]
{let address=(cpu.x[20]).wrapping_add(0x54u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc9fe8 b #0x10fc9fc8
pc=0x10fc9fc8;continue;
}
0x10fc9fec=>{
// 0x10fc9fec ldr w8, [x20, #0x50]
{let address=(cpu.x[20]).wrapping_add(0x50u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc9ff0 str wzr, [x20, #0x54]
{let address=(cpu.x[20]).wrapping_add(0x54u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc9ff4 cmp w8, w21
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[21] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc9ff8 b.ge #0x10fca008
if cpu.condition(10){pc=0x10fca008;continue;}pc=0x10fc9ffc;continue;
}
0x10fc9ffc=>{
// 0x10fc9ffc add x0, x20, #0x48
{let a=((cpu.x[20]));let b=((0x48u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
pc=0x10fca000;continue;
}
0x10fca000=>{
// 0x10fca000 mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10fca004 bl #0x10f48cb8
cpu.x[30]=0x10fca008;super::chunk0::f_10f48cb8(cpu,rt,0x10f48cb8)?;
pc=0x10fca008;continue;
}
0x10fca008=>{
// 0x10fca008 cmp w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fca00c b.lt #0x10fca078
if cpu.condition(11){pc=0x10fca078;continue;}pc=0x10fca010;continue;
}
0x10fca010=>{
// 0x10fca010 mov w22, #5
cpu.x[22]=(((0x5u64))) as u32 as u64;
// 0x10fca014 b #0x10fca030
pc=0x10fca030;continue;
}
0x10fca018=>{
// 0x10fca018 ldr x9, [x20, #0x48]
{let address=(cpu.x[20]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fca01c add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10fca020 subs w21, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[21]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10fca024 str w10, [x20, #0x54]
{let address=(cpu.x[20]).wrapping_add(0x54u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fca028 str x0, [x9, w8, sxtw #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fca02c b.eq #0x10fca078
if cpu.condition(0){pc=0x10fca078;continue;}pc=0x10fca030;continue;
}
0x10fca030=>{
// 0x10fca030 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fca034 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fca038 ldr x8, [x8, #0x128]
{let address=(cpu.x[8]).wrapping_add(0x128u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fca03c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fca040;super::dispatch(cpu,rt,target)?;}
pc=0x10fca040;continue;
}
0x10fca040=>{
// 0x10fca040 ldp w9, w8, [x20, #0x50]
{let address=(cpu.x[20]).wrapping_add(0x50u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10fca044 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fca048 b.ne #0x10fca018
if cpu.condition(1){pc=0x10fca018;continue;}pc=0x10fca04c;continue;
}
0x10fca04c=>{
// 0x10fca04c lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10fca050 cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fca054 csel w1, w22, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[22] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10fca058 cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fca05c b.ge #0x10fca018
if cpu.condition(10){pc=0x10fca018;continue;}pc=0x10fca060;continue;
}
0x10fca060=>{
// 0x10fca060 mov x23, x0
cpu.x[23]=(((cpu.x[0]))) as u64;
// 0x10fca064 add x0, x20, #0x48
{let a=((cpu.x[20]));let b=((0x48u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fca068 bl #0x10f48cb8
cpu.x[30]=0x10fca06c;super::chunk0::f_10f48cb8(cpu,rt,0x10f48cb8)?;
// 0x10fca06c ldr w8, [x20, #0x54]
{let address=(cpu.x[20]).wrapping_add(0x54u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fca070 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fca074 b #0x10fca018
pc=0x10fca018;continue;
}
0x10fca078=>{
// 0x10fca078 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fca07c str wzr, [x20, #0x64]
{let address=(cpu.x[20]).wrapping_add(0x64u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fca080 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fca084 str wzr, [x20, #0x74]
{let address=(cpu.x[20]).wrapping_add(0x74u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fca088 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fca08c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fca090;super::dispatch(cpu,rt,target)?;}
pc=0x10fca090;continue;
}
0x10fca090=>{
// 0x10fca090 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fca094 b.lt #0x10fca158
if cpu.condition(11){pc=0x10fca158;continue;}pc=0x10fca098;continue;
}
0x10fca098=>{
// 0x10fca098 mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fca09c mov w22, #5
cpu.x[22]=(((0x5u64))) as u32 as u64;
// 0x10fca0a0 b #0x10fca0bc
pc=0x10fca0bc;continue;
}
0x10fca0a4=>{
// 0x10fca0a4 ldr x9, [x20, #0x68]
{let address=(cpu.x[20]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fca0a8 add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10fca0ac subs w21, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[21]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10fca0b0 str w10, [x20, #0x74]
{let address=(cpu.x[20]).wrapping_add(0x74u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fca0b4 str w0, [x9, w8, sxtw #2]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fca0b8 b.eq #0x10fca158
if cpu.condition(0){pc=0x10fca158;continue;}pc=0x10fca0bc;continue;
}
0x10fca0bc=>{
// 0x10fca0bc ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fca0c0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fca0c4 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fca0c8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fca0cc;super::dispatch(cpu,rt,target)?;}
// 0x10fca0cc ldp w9, w8, [x20, #0x60]
{let address=(cpu.x[20]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10fca0d0 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fca0d4 b.ne #0x10fca100
if cpu.condition(1){pc=0x10fca100;continue;}pc=0x10fca0d8;continue;
}
0x10fca0d8=>{
// 0x10fca0d8 lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10fca0dc cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fca0e0 csel w1, w22, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[22] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10fca0e4 cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fca0e8 b.ge #0x10fca100
if cpu.condition(10){pc=0x10fca100;continue;}pc=0x10fca0ec;continue;
}
0x10fca0ec=>{
// 0x10fca0ec mov w23, w0
cpu.x[23]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fca0f0 add x0, x20, #0x58
{let a=((cpu.x[20]));let b=((0x58u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fca0f4 bl #0x1078f4d4
cpu.x[30]=0x10fca0f8;super::chunk2::f_1078f4d4(cpu,rt,0x1078f4d4)?;
// 0x10fca0f8 ldr w8, [x20, #0x64]
{let address=(cpu.x[20]).wrapping_add(0x64u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fca0fc mov w0, w23
cpu.x[0]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
pc=0x10fca100;continue;
}
0x10fca100=>{
// 0x10fca100 ldr x9, [x20, #0x58]
{let address=(cpu.x[20]).wrapping_add(0x58u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fca104 add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10fca108 str w10, [x20, #0x64]
{let address=(cpu.x[20]).wrapping_add(0x64u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fca10c ldr x10, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10fca110 str w0, [x9, w8, sxtw #2]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fca114 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fca118 ldr x8, [x10, #0x110]
{let address=(cpu.x[10]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fca11c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fca120;super::dispatch(cpu,rt,target)?;}
// 0x10fca120 ldp w9, w8, [x20, #0x70]
{let address=(cpu.x[20]).wrapping_add(0x70u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10fca124 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fca128 b.ne #0x10fca0a4
if cpu.condition(1){pc=0x10fca0a4;continue;}pc=0x10fca12c;continue;
}
0x10fca12c=>{
// 0x10fca12c lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10fca130 cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fca134 csel w1, w22, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[22] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10fca138 cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fca13c b.ge #0x10fca0a4
if cpu.condition(10){pc=0x10fca0a4;continue;}pc=0x10fca140;continue;
}
0x10fca140=>{
// 0x10fca140 mov w23, w0
cpu.x[23]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fca144 add x0, x20, #0x68
{let a=((cpu.x[20]));let b=((0x68u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fca148 bl #0x1078f4d4
cpu.x[30]=0x10fca14c;super::chunk2::f_1078f4d4(cpu,rt,0x1078f4d4)?;
// 0x10fca14c ldr w8, [x20, #0x74]
{let address=(cpu.x[20]).wrapping_add(0x74u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fca150 mov w0, w23
cpu.x[0]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10fca154 b #0x10fca0a4
pc=0x10fca0a4;continue;
}
0x10fca158=>{
// 0x10fca158 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fca15c ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10fca160 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fca164 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10fca168 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10fca16c=>{
// 0x10fca16c ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fca170 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fca174 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fca178 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fca17c;super::dispatch(cpu,rt,target)?;}
pc=0x10fca17c;continue;
}
0x10fca17c=>{
// 0x10fca17c str w0, [x20, #0x3c]
{let address=(cpu.x[20]).wrapping_add(0x3cu64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fca180 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fca184 bl #0x10de0ba8
cpu.x[30]=0x10fca188;super::chunk12::f_10de0ba8(cpu,rt,0x10de0ba8)?;
pc=0x10fca188;continue;
}
0x10fca188=>{
// 0x10fca188 tbz w0, #0, #0x10fc9f7c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fc9f7c;continue;}pc=0x10fca18c;continue;
}
0x10fca18c=>{
// 0x10fca18c ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fca190 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fca194 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fca198 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fca19c;super::dispatch(cpu,rt,target)?;}
// 0x10fca19c ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fca1a0 str w0, [x20, #0x20]
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fca1a4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fca1a8 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fca1ac blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fca1b0;super::dispatch(cpu,rt,target)?;}
// 0x10fca1b0 str w0, [x20, #0x24]
{let address=(cpu.x[20]).wrapping_add(0x24u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fca1b4 ldr w8, [x21, #0x674]
{let address=(cpu.x[21]).wrapping_add(0x674u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fca1b8 cbnz w8, #0x10fc9f84
if (cpu.x[8] as u32 as u64) != 0 {pc=0x10fc9f84;continue;}pc=0x10fca1bc;continue;
}
0x10fca1bc=>{
// 0x10fca1bc b #0x10fc9f98
pc=0x10fc9f98;continue;
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fc9ea8")),}}}
#[inline]
pub(super) unsafe fn f_10fcaa2c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fcaa2c=>{
// 0x10fcaa2c cbz x1, #0x10fcaeb0
if (cpu.x[1]) == 0 {pc=0x10fcaeb0;continue;}pc=0x10fcaa30;continue;
}
0x10fcaa30=>{
// 0x10fcaa30 sub sp, sp, #0xc0
{let a=((cpu.sp));let b=((0xc0u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x10fcaa34 stp x29, x30, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fcaa38 stp x28, x27, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=cpu.x[28];let v1=cpu.x[27];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fcaa3c stp x26, x25, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=cpu.x[26];let v1=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fcaa40 stp x24, x23, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fcaa44 stp x22, x21, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fcaa48 stp x20, x19, [sp, #0xb0]
{let address=(cpu.sp).wrapping_add(0xb0u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fcaa4c add x29, sp, #0x60
{let a=((cpu.sp));let b=((0x60u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x10fcaa50 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10fcaa54 ldr x20, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10fcaa58 mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x10fcaa5c mov w26, w4
cpu.x[26]=(((cpu.x[4] as u32 as u64))) as u32 as u64;
// 0x10fcaa60 mov w27, w3
cpu.x[27]=(((cpu.x[3] as u32 as u64))) as u32 as u64;
// 0x10fcaa64 mov x22, x1
cpu.x[22]=(((cpu.x[1]))) as u64;
// 0x10fcaa68 mov w19, w2
cpu.x[19]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x10fcaa6c stur w5, [x29, #-4]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffffcu64);let v0=cpu.x[5] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fcaa70 bl #0x10f95b64
cpu.x[30]=0x10fcaa74;super::chunk0::f_10f95b64(cpu,rt,0x10f95b64)?;
pc=0x10fcaa74;continue;
}
0x10fcaa74=>{
// 0x10fcaa74 cbz x0, #0x10fcad4c
if (cpu.x[0]) == 0 {pc=0x10fcad4c;continue;}pc=0x10fcaa78;continue;
}
0x10fcaa78=>{
// 0x10fcaa78 stur x21, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fcaa7c ldr x21, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10fcaa80 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10fcaa84 bl #0x10f95c18
cpu.x[30]=0x10fcaa88;super::chunk8::f_10f95c18(cpu,rt,0x10f95c18)?;
pc=0x10fcaa88;continue;
}
0x10fcaa88=>{
// 0x10fcaa88 mov w23, w0
cpu.x[23]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fcaa8c mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10fcaa90 bl #0x10f940fc
cpu.x[30]=0x10fcaa94;super::chunk9::f_10f940fc(cpu,rt,0x10f940fc)?;
pc=0x10fcaa94;continue;
}
0x10fcaa94=>{
// 0x10fcaa94 mov x25, x0
cpu.x[25]=(((cpu.x[0]))) as u64;
// 0x10fcaa98 stur w27, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=cpu.x[27] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fcaa9c stur x21, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fcaaa0 stur w23, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=cpu.x[23] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fcaaa4 cbz x0, #0x10fcab7c
if (cpu.x[0]) == 0 {pc=0x10fcab7c;continue;}pc=0x10fcaaa8;continue;
}
0x10fcaaa8=>{
// 0x10fcaaa8 mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10fcaaac bl #0x10fc22f0
cpu.x[30]=0x10fcaab0;super::chunk3::f_10fc22f0(cpu,rt,0x10fc22f0)?;
pc=0x10fcaab0;continue;
}
0x10fcaab0=>{
// 0x10fcaab0 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcaab4 b.lt #0x10fcaca8
if cpu.condition(11){pc=0x10fcaca8;continue;}pc=0x10fcaab8;continue;
}
0x10fcaab8=>{
// 0x10fcaab8 mov w27, w0
cpu.x[27]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fcaabc mov w28, wzr
cpu.x[28]=(((0u64))) as u32 as u64;
// 0x10fcaac0 mov w21, wzr
cpu.x[21]=(((0u64))) as u32 as u64;
// 0x10fcaac4 b #0x10fcaad4
pc=0x10fcaad4;continue;
}
0x10fcaac8=>{
// 0x10fcaac8 add w21, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[21]=(result) as u32 as u64;}
// 0x10fcaacc cmp w27, w21
{let a=((cpu.x[27] as u32 as u64));let b=((cpu.x[21] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcaad0 b.eq #0x10fcacac
if cpu.condition(0){pc=0x10fcacac;continue;}pc=0x10fcaad4;continue;
}
0x10fcaad4=>{
// 0x10fcaad4 mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10fcaad8 mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10fcaadc bl #0x10fc23e4
cpu.x[30]=0x10fcaae0;super::chunk11::f_10fc23e4(cpu,rt,0x10fc23e4)?;
pc=0x10fcaae0;continue;
}
0x10fcaae0=>{
// 0x10fcaae0 cbz x0, #0x10fcaac8
if (cpu.x[0]) == 0 {pc=0x10fcaac8;continue;}pc=0x10fcaae4;continue;
}
0x10fcaae4=>{
// 0x10fcaae4 mov x24, x0
cpu.x[24]=(((cpu.x[0]))) as u64;
// 0x10fcaae8 ldr x0, [x0, #0x18]
{let address=(cpu.x[0]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fcaaec cbz x0, #0x10fcaac8
if (cpu.x[0]) == 0 {pc=0x10fcaac8;continue;}pc=0x10fcaaf0;continue;
}
0x10fcaaf0=>{
// 0x10fcaaf0 tbz w26, #0, #0x10fcab20
if ((cpu.x[26] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fcab20;continue;}pc=0x10fcaaf4;continue;
}
0x10fcaaf4=>{
// 0x10fcaaf4 mov w26, #1
cpu.x[26]=(((0x1u64))) as u32 as u64;
// 0x10fcaaf8 tbnz w28, #0, #0x10fcab30
if ((cpu.x[28] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10fcab30;continue;}pc=0x10fcaafc;continue;
}
0x10fcaafc=>{
// 0x10fcaafc bl #0x10ddacf0
cpu.x[30]=0x10fcab00;super::chunk3::f_10ddacf0(cpu,rt,0x10ddacf0)?;
pc=0x10fcab00;continue;
}
0x10fcab00=>{
// 0x10fcab00 mov w28, w0
cpu.x[28]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fcab04 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10fcab08 bl #0x10f86bb8
cpu.x[30]=0x10fcab0c;super::chunk1::f_10f86bb8(cpu,rt,0x10f86bb8)?;
pc=0x10fcab0c;continue;
}
0x10fcab0c=>{
// 0x10fcab0c mov x23, x0
cpu.x[23]=(((cpu.x[0]))) as u64;
// 0x10fcab10 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10fcab14 bl #0x10f86db0
cpu.x[30]=0x10fcab18;super::chunk2::f_10f86db0(cpu,rt,0x10f86db0)?;
pc=0x10fcab18;continue;
}
0x10fcab18=>{
// 0x10fcab18 cbnz x23, #0x10fcab4c
if (cpu.x[23]) != 0 {pc=0x10fcab4c;continue;}pc=0x10fcab1c;continue;
}
0x10fcab1c=>{
// 0x10fcab1c b #0x10fcaac8
pc=0x10fcaac8;continue;
}
0x10fcab20=>{
// 0x10fcab20 ldrb w8, [x0, #0x12c]
{let address=(cpu.x[0]).wrapping_add(0x12cu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcab24 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcab28 cset w26, ne
cpu.x[26]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x10fcab2c tbz w28, #0, #0x10fcaafc
if ((cpu.x[28] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fcaafc;continue;}pc=0x10fcab30;continue;
}
0x10fcab30=>{
// 0x10fcab30 mov w28, #1
cpu.x[28]=(((0x1u64))) as u32 as u64;
// 0x10fcab34 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10fcab38 bl #0x10f86bb8
cpu.x[30]=0x10fcab3c;super::chunk1::f_10f86bb8(cpu,rt,0x10f86bb8)?;
// 0x10fcab3c mov x23, x0
cpu.x[23]=(((cpu.x[0]))) as u64;
// 0x10fcab40 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10fcab44 bl #0x10f86db0
cpu.x[30]=0x10fcab48;super::chunk2::f_10f86db0(cpu,rt,0x10f86db0)?;
// 0x10fcab48 cbz x23, #0x10fcaac8
if (cpu.x[23]) == 0 {pc=0x10fcaac8;continue;}pc=0x10fcab4c;continue;
}
0x10fcab4c=>{
// 0x10fcab4c cmn w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.flags(a,b,result,32,false);}
// 0x10fcab50 b.eq #0x10fcaac8
if cpu.condition(0){pc=0x10fcaac8;continue;}pc=0x10fcab54;continue;
}
0x10fcab54=>{
// 0x10fcab54 ldr w1, [x24, #0x64]
{let address=(cpu.x[24]).wrapping_add(0x64u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10fcab58 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcab5c bl #0x10d3cac8
cpu.x[30]=0x10fcab60;super::chunk5::f_10d3cac8(cpu,rt,0x10d3cac8)?;
pc=0x10fcab60;continue;
}
0x10fcab60=>{
// 0x10fcab60 cbz x0, #0x10fcaac8
if (cpu.x[0]) == 0 {pc=0x10fcaac8;continue;}pc=0x10fcab64;continue;
}
0x10fcab64=>{
// 0x10fcab64 ldr x1, [x23, #0x10]
{let address=(cpu.x[23]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10fcab68 ldr w2, [x24, #0x68]
{let address=(cpu.x[24]).wrapping_add(0x68u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[2]=(v0) as u32 as u64;}
// 0x10fcab6c mov w4, w19
cpu.x[4]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10fcab70 ldrb w3, [x24, #0x6c]
{let address=(cpu.x[24]).wrapping_add(0x6cu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10fcab74 bl #0x10d549d0
cpu.x[30]=0x10fcab78;super::chunk3::f_10d549d0(cpu,rt,0x10d549d0)?;
pc=0x10fcab78;continue;
}
0x10fcab78=>{
// 0x10fcab78 b #0x10fcaac8
pc=0x10fcaac8;continue;
}
0x10fcab7c=>{
// 0x10fcab7c ldr x8, [x22]
{let address=(cpu.x[22]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcab80 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10fcab84 ldr x8, [x8, #0x198]
{let address=(cpu.x[8]).wrapping_add(0x198u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcab88 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fcab8c;super::dispatch(cpu,rt,target)?;}
pc=0x10fcab8c;continue;
}
0x10fcab8c=>{
// 0x10fcab8c tbz w0, #0, #0x10fcaca8
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fcaca8;continue;}pc=0x10fcab90;continue;
}
0x10fcab90=>{
// 0x10fcab90 ldr x0, [x22, #0x118]
{let address=(cpu.x[22]).wrapping_add(0x118u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fcab94 cbz x0, #0x10fcaba8
if (cpu.x[0]) == 0 {pc=0x10fcaba8;continue;}pc=0x10fcab98;continue;
}
0x10fcab98=>{
// 0x10fcab98 bl #0x10f940fc
cpu.x[30]=0x10fcab9c;super::chunk9::f_10f940fc(cpu,rt,0x10f940fc)?;
pc=0x10fcab9c;continue;
}
0x10fcab9c=>{
// 0x10fcab9c mov x25, x0
cpu.x[25]=(((cpu.x[0]))) as u64;
// 0x10fcaba0 cbnz x0, #0x10fcaaa8
if (cpu.x[0]) != 0 {pc=0x10fcaaa8;continue;}pc=0x10fcaba4;continue;
}
0x10fcaba4=>{
// 0x10fcaba4 b #0x10fcaca8
pc=0x10fcaca8;continue;
}
0x10fcaba8=>{
// 0x10fcaba8 cbz x20, #0x10fcaca8
if (cpu.x[20]) == 0 {pc=0x10fcaca8;continue;}pc=0x10fcabac;continue;
}
0x10fcabac=>{
// 0x10fcabac ldr x23, [x22, #0x1e8]
{let address=(cpu.x[22]).wrapping_add(0x1e8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10fcabb0 mov w28, wzr
cpu.x[28]=(((0u64))) as u32 as u64;
// 0x10fcabb4 cbz x23, #0x10fcacac
if (cpu.x[23]) == 0 {pc=0x10fcacac;continue;}pc=0x10fcabb8;continue;
}
0x10fcabb8=>{
// 0x10fcabb8 ldr x21, [x22, #0x1e0]
{let address=(cpu.x[22]).wrapping_add(0x1e0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10fcabbc cbz x21, #0x10fcacac
if (cpu.x[21]) == 0 {pc=0x10fcacac;continue;}pc=0x10fcabc0;continue;
}
0x10fcabc0=>{
// 0x10fcabc0 ldr x8, [x22, #0x1f0]
{let address=(cpu.x[22]).wrapping_add(0x1f0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcabc4 stur x8, [x29, #-0x28]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffd8u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fcabc8 cbz x8, #0x10fcacac
if (cpu.x[8]) == 0 {pc=0x10fcacac;continue;}pc=0x10fcabcc;continue;
}
0x10fcabcc=>{
// 0x10fcabcc ldr w8, [x23, #0xc]
{let address=(cpu.x[23]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcabd0 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcabd4 b.lt #0x10fcaca8
if cpu.condition(11){pc=0x10fcaca8;continue;}pc=0x10fcabd8;continue;
}
0x10fcabd8=>{
// 0x10fcabd8 ldr x8, [x22, #0x1f8]
{let address=(cpu.x[22]).wrapping_add(0x1f8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcabdc mov x28, xzr
cpu.x[28]=(((0u64))) as u64;
// 0x10fcabe0 str x8, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fcabe4 b #0x10fcabf8
pc=0x10fcabf8;continue;
}
0x10fcabe8=>{
// 0x10fcabe8 ldrsw x8, [x23, #0xc]
{let address=(cpu.x[23]).wrapping_add(0xcu64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10fcabec add x28, x28, #1
{let a=((cpu.x[28]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[28]=(result) as u64;}
// 0x10fcabf0 cmp x28, x8
{let a=((cpu.x[28]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fcabf4 b.ge #0x10fcaca8
if cpu.condition(10){pc=0x10fcaca8;continue;}pc=0x10fcabf8;continue;
}
0x10fcabf8=>{
// 0x10fcabf8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcabfc bl #0x10f93d2c
cpu.x[30]=0x10fcac00;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
// 0x10fcac00 cbz x0, #0x10fcabe8
if (cpu.x[0]) == 0 {pc=0x10fcabe8;continue;}pc=0x10fcac04;continue;
}
0x10fcac04=>{
// 0x10fcac04 ldr x9, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fcac08 lsl x8, x28, #2
cpu.x[8]=(((((cpu.x[28]))) as u64).wrapping_shl(((((0x2u64))) & 63) as u32) as u64) as u64;
// 0x10fcac0c ldr w27, [x9, x8]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[8])));let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[27]=(v0) as u32 as u64;}
// 0x10fcac10 tbnz w27, #0x1f, #0x10fcabe8
if ((cpu.x[27] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x10fcabe8;continue;}pc=0x10fcac14;continue;
}
0x10fcac14=>{
// 0x10fcac14 ldr x9, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fcac18 ldr w25, [x9, x8]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[8])));let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[25]=(v0) as u32 as u64;}
// 0x10fcac1c tbnz w25, #0x1f, #0x10fcabe8
if ((cpu.x[25] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x10fcabe8;continue;}pc=0x10fcac20;continue;
}
0x10fcac20=>{
// 0x10fcac20 ldur x9, [x29, #-0x28]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffd8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fcac24 ldr x10, [sp, #0x28]
{let address=(cpu.sp).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10fcac28 mov w1, w25
cpu.x[1]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fcac2c mov x24, x0
cpu.x[24]=(((cpu.x[0]))) as u64;
// 0x10fcac30 ldr x9, [x9]
{let address=(cpu.x[9]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fcac34 ldr x10, [x10]
{let address=(cpu.x[10]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10fcac38 ldr w8, [x9, x8]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[8])));let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcac3c stur w8, [x29, #-0x2c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffd4u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fcac40 ldrb w8, [x10, x28]
{let address=((cpu.x[10]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[28])));let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcac44 str w8, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fcac48 bl #0x10d32c84
cpu.x[30]=0x10fcac4c;super::chunk15::f_10d32c84(cpu,rt,0x10d32c84)?;
// 0x10fcac4c cbz x0, #0x10fcabe8
if (cpu.x[0]) == 0 {pc=0x10fcabe8;continue;}pc=0x10fcac50;continue;
}
0x10fcac50=>{
// 0x10fcac50 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcac54 mov w1, w27
cpu.x[1]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fcac58 ldr x8, [x8, #0x38]
{let address=(cpu.x[8]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcac5c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fcac60;super::dispatch(cpu,rt,target)?;}
// 0x10fcac60 cbz x0, #0x10fcabe8
if (cpu.x[0]) == 0 {pc=0x10fcabe8;continue;}pc=0x10fcac64;continue;
}
0x10fcac64=>{
// 0x10fcac64 mov x27, x0
cpu.x[27]=(((cpu.x[0]))) as u64;
// 0x10fcac68 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10fcac6c mov w1, w25
cpu.x[1]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fcac70 bl #0x10d39264
cpu.x[30]=0x10fcac74;super::chunk4::f_10d39264(cpu,rt,0x10d39264)?;
// 0x10fcac74 tbnz w0, #0x1f, #0x10fcabe8
if ((cpu.x[0] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x10fcabe8;continue;}pc=0x10fcac78;continue;
}
0x10fcac78=>{
// 0x10fcac78 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcac7c mov w1, w25
cpu.x[1]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10fcac80 bl #0x10d3cac8
cpu.x[30]=0x10fcac84;super::chunk5::f_10d3cac8(cpu,rt,0x10d3cac8)?;
// 0x10fcac84 cbz x0, #0x10fcabe8
if (cpu.x[0]) == 0 {pc=0x10fcabe8;continue;}pc=0x10fcac88;continue;
}
0x10fcac88=>{
// 0x10fcac88 ldr w8, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcac8c ldr x1, [x27, #0x10]
{let address=(cpu.x[27]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10fcac90 mov w4, w19
cpu.x[4]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10fcac94 ldur w2, [x29, #-0x2c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffd4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[2]=(v0) as u32 as u64;}
// 0x10fcac98 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcac9c cset w3, ne
cpu.x[3]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x10fcaca0 bl #0x10d549d0
cpu.x[30]=0x10fcaca4;super::chunk3::f_10d549d0(cpu,rt,0x10d549d0)?;
// 0x10fcaca4 b #0x10fcabe8
pc=0x10fcabe8;continue;
}
0x10fcaca8=>{
// 0x10fcaca8 mov w28, wzr
cpu.x[28]=(((0u64))) as u32 as u64;
pc=0x10fcacac;continue;
}
0x10fcacac=>{
// 0x10fcacac mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10fcacb0 bl #0x10f9433c
cpu.x[30]=0x10fcacb4;super::chunk3::f_10f9433c(cpu,rt,0x10f9433c)?;
pc=0x10fcacb4;continue;
}
0x10fcacb4=>{
// 0x10fcacb4 cbz x0, #0x10fcaeb4
if (cpu.x[0]) == 0 {pc=0x10fcaeb4;continue;}pc=0x10fcacb8;continue;
}
0x10fcacb8=>{
// 0x10fcacb8 ldur w27, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[27]=(v0) as u32 as u64;}
// 0x10fcacbc ldur x25, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[25]=(v0) as u64;}
// 0x10fcacc0 tbnz w26, #0, #0x10fcaccc
if ((cpu.x[26] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10fcaccc;continue;}pc=0x10fcacc4;continue;
}
0x10fcacc4=>{
// 0x10fcacc4 ldrb w8, [x0, #0xa8]
{let address=(cpu.x[0]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcacc8 cbz w8, #0x10fcaedc
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10fcaedc;continue;}pc=0x10fcaccc;continue;
}
0x10fcaccc=>{
// 0x10fcaccc mov w26, #1
cpu.x[26]=(((0x1u64))) as u32 as u64;
pc=0x10fcacd0;continue;
}
0x10fcacd0=>{
// 0x10fcacd0 ldrb w8, [x0, #0xaa]
{let address=(cpu.x[0]).wrapping_add(0xaau64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcacd4 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcacd8 cset w23, ne
cpu.x[23]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x10fcacdc mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10fcace0 bl #0x10f95bec
cpu.x[30]=0x10fcace4;super::chunk6::f_10f95bec(cpu,rt,0x10f95bec)?;
pc=0x10fcace4;continue;
}
0x10fcace4=>{
// 0x10fcace4 mov w1, w0
cpu.x[1]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fcace8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcacec bl #0x10d3cac8
cpu.x[30]=0x10fcacf0;super::chunk5::f_10d3cac8(cpu,rt,0x10d3cac8)?;
pc=0x10fcacf0;continue;
}
0x10fcacf0=>{
// 0x10fcacf0 cbz x0, #0x10fcad4c
if (cpu.x[0]) == 0 {pc=0x10fcad4c;continue;}pc=0x10fcacf4;continue;
}
0x10fcacf4=>{
// 0x10fcacf4 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10fcacf8 cmp w19, w27
{let a=((cpu.x[19] as u32 as u64));let b=((cpu.x[27] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcacfc mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10fcad00 cset w24, ge
cpu.x[24]=((cpu.condition(10)) as u64) as u32 as u64;
// 0x10fcad04 bl #0x10f95c08
cpu.x[30]=0x10fcad08;super::chunk13::f_10f95c08(cpu,rt,0x10f95c08)?;
pc=0x10fcad08;continue;
}
0x10fcad08=>{
// 0x10fcad08 ldur w10, [x29, #-4]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffffcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10fcad0c ldrb w3, [x22, #0xa4]
{let address=(cpu.x[22]).wrapping_add(0xa4u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10fcad10 mov w2, w0
cpu.x[2]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fcad14 ldur x1, [x29, #-0x18]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10fcad18 and w8, w28, #1
{let a=((cpu.x[28] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fcad1c and w9, w26, #1
{let a=((cpu.x[26] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[9]=(result) as u32 as u64;}
// 0x10fcad20 and w6, w10, #1
{let a=((cpu.x[10] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[6]=(result) as u32 as u64;}
// 0x10fcad24 ldur w10, [x29, #-0x1c]
{let address=(cpu.x[29]).wrapping_add(0xffffffffffffffe4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10fcad28 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fcad2c mov x4, x25
cpu.x[4]=(((cpu.x[25]))) as u64;
// 0x10fcad30 mov w5, w19
cpu.x[5]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10fcad34 strb w24, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=cpu.x[24] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10fcad38 and w7, w10, #1
{let a=((cpu.x[10] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[7]=(result) as u32 as u64;}
// 0x10fcad3c strb w23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[23] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10fcad40 strb w8, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10fcad44 strb w9, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10fcad48 bl #0x10d54230
cpu.x[30]=0x10fcad4c;super::chunk2::f_10d54230(cpu,rt,0x10d54230)?;
pc=0x10fcad4c;continue;
}
0x10fcad4c=>{
// 0x10fcad4c cbz x20, #0x10fcae94
if (cpu.x[20]) == 0 {pc=0x10fcae94;continue;}pc=0x10fcad50;continue;
}
0x10fcad50=>{
// 0x10fcad50 ldr x8, [x22]
{let address=(cpu.x[22]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcad54 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10fcad58 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcad5c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fcad60;super::dispatch(cpu,rt,target)?;}
pc=0x10fcad60;continue;
}
0x10fcad60=>{
// 0x10fcad60 cmp w0, #4
{let a=((cpu.x[0] as u32 as u64));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcad64 b.ne #0x10fcada0
if cpu.condition(1){pc=0x10fcada0;continue;}pc=0x10fcad68;continue;
}
0x10fcad68=>{
// 0x10fcad68 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10fcad6c bl #0x10f8fc08
cpu.x[30]=0x10fcad70;super::chunk10::f_10f8fc08(cpu,rt,0x10f8fc08)?;
pc=0x10fcad70;continue;
}
0x10fcad70=>{
// 0x10fcad70 cbz x0, #0x10fcada0
if (cpu.x[0]) == 0 {pc=0x10fcada0;continue;}pc=0x10fcad74;continue;
}
0x10fcad74=>{
// 0x10fcad74 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcad78 ldr x8, [x8, #0x178]
{let address=(cpu.x[8]).wrapping_add(0x178u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcad7c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fcad80;super::dispatch(cpu,rt,target)?;}
pc=0x10fcad80;continue;
}
0x10fcad80=>{
// 0x10fcad80 mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fcad84 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcad88 bl #0x10f94124
cpu.x[30]=0x10fcad8c;super::chunk5::f_10f94124(cpu,rt,0x10f94124)?;
pc=0x10fcad8c;continue;
}
0x10fcad8c=>{
// 0x10fcad8c mov w22, w0
cpu.x[22]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fcad90 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcad94 bl #0x10f95b64
cpu.x[30]=0x10fcad98;super::chunk0::f_10f95b64(cpu,rt,0x10f95b64)?;
pc=0x10fcad98;continue;
}
0x10fcad98=>{
// 0x10fcad98 cbnz x0, #0x10fcadbc
if (cpu.x[0]) != 0 {pc=0x10fcadbc;continue;}pc=0x10fcad9c;continue;
}
0x10fcad9c=>{
// 0x10fcad9c b #0x10fcae00
pc=0x10fcae00;continue;
}
0x10fcada0=>{
// 0x10fcada0 mov w21, wzr
cpu.x[21]=(((0u64))) as u32 as u64;
// 0x10fcada4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcada8 bl #0x10f94124
cpu.x[30]=0x10fcadac;super::chunk5::f_10f94124(cpu,rt,0x10f94124)?;
pc=0x10fcadac;continue;
}
0x10fcadac=>{
// 0x10fcadac mov w22, w0
cpu.x[22]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fcadb0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcadb4 bl #0x10f95b64
cpu.x[30]=0x10fcadb8;super::chunk0::f_10f95b64(cpu,rt,0x10f95b64)?;
pc=0x10fcadb8;continue;
}
0x10fcadb8=>{
// 0x10fcadb8 cbz x0, #0x10fcae00
if (cpu.x[0]) == 0 {pc=0x10fcae00;continue;}pc=0x10fcadbc;continue;
}
0x10fcadbc=>{
// 0x10fcadbc ldr x23, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10fcadc0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcadc4 bl #0x10f95bec
cpu.x[30]=0x10fcadc8;super::chunk6::f_10f95bec(cpu,rt,0x10f95bec)?;
pc=0x10fcadc8;continue;
}
0x10fcadc8=>{
// 0x10fcadc8 mov w1, w0
cpu.x[1]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fcadcc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcadd0 bl #0x10d3cac8
cpu.x[30]=0x10fcadd4;super::chunk5::f_10d3cac8(cpu,rt,0x10d3cac8)?;
pc=0x10fcadd4;continue;
}
0x10fcadd4=>{
// 0x10fcadd4 cbz x0, #0x10fcae00
if (cpu.x[0]) == 0 {pc=0x10fcae00;continue;}pc=0x10fcadd8;continue;
}
0x10fcadd8=>{
// 0x10fcadd8 mov x24, x0
cpu.x[24]=(((cpu.x[0]))) as u64;
// 0x10fcaddc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcade0 bl #0x10f95c08
cpu.x[30]=0x10fcade4;super::chunk13::f_10f95c08(cpu,rt,0x10f95c08)?;
pc=0x10fcade4;continue;
}
0x10fcade4=>{
// 0x10fcade4 ldrb w3, [x20, #0xa4]
{let address=(cpu.x[20]).wrapping_add(0xa4u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10fcade8 mov w2, w0
cpu.x[2]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fcadec and w5, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[5]=(result) as u32 as u64;}
// 0x10fcadf0 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10fcadf4 mov x1, x23
cpu.x[1]=(((cpu.x[23]))) as u64;
// 0x10fcadf8 mov w4, w27
cpu.x[4]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fcadfc bl #0x10d54898
cpu.x[30]=0x10fcae00;super::chunk6::f_10d54898(cpu,rt,0x10d54898)?;
pc=0x10fcae00;continue;
}
0x10fcae00=>{
// 0x10fcae00 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcae04 bl #0x10f940fc
cpu.x[30]=0x10fcae08;super::chunk9::f_10f940fc(cpu,rt,0x10f940fc)?;
pc=0x10fcae08;continue;
}
0x10fcae08=>{
// 0x10fcae08 cbz x0, #0x10fcae94
if (cpu.x[0]) == 0 {pc=0x10fcae94;continue;}pc=0x10fcae0c;continue;
}
0x10fcae0c=>{
// 0x10fcae0c mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10fcae10 bl #0x10fc22f0
cpu.x[30]=0x10fcae14;super::chunk3::f_10fc22f0(cpu,rt,0x10fc22f0)?;
pc=0x10fcae14;continue;
}
0x10fcae14=>{
// 0x10fcae14 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcae18 b.lt #0x10fcae94
if cpu.condition(11){pc=0x10fcae94;continue;}pc=0x10fcae1c;continue;
}
0x10fcae1c=>{
// 0x10fcae1c mov w23, w0
cpu.x[23]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fcae20 mov w24, wzr
cpu.x[24]=(((0u64))) as u32 as u64;
// 0x10fcae24 b #0x10fcae34
pc=0x10fcae34;continue;
}
0x10fcae28=>{
// 0x10fcae28 add w24, w24, #1
{let a=((cpu.x[24] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[24]=(result) as u32 as u64;}
// 0x10fcae2c cmp w23, w24
{let a=((cpu.x[23] as u32 as u64));let b=((cpu.x[24] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcae30 b.eq #0x10fcae94
if cpu.condition(0){pc=0x10fcae94;continue;}pc=0x10fcae34;continue;
}
0x10fcae34=>{
// 0x10fcae34 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fcae38 mov w1, w24
cpu.x[1]=(((cpu.x[24] as u32 as u64))) as u32 as u64;
// 0x10fcae3c bl #0x10fc23e4
cpu.x[30]=0x10fcae40;super::chunk11::f_10fc23e4(cpu,rt,0x10fc23e4)?;
pc=0x10fcae40;continue;
}
0x10fcae40=>{
// 0x10fcae40 cbz x0, #0x10fcae28
if (cpu.x[0]) == 0 {pc=0x10fcae28;continue;}pc=0x10fcae44;continue;
}
0x10fcae44=>{
// 0x10fcae44 mov x25, x0
cpu.x[25]=(((cpu.x[0]))) as u64;
// 0x10fcae48 bl #0x10f86bb8
cpu.x[30]=0x10fcae4c;super::chunk1::f_10f86bb8(cpu,rt,0x10f86bb8)?;
pc=0x10fcae4c;continue;
}
0x10fcae4c=>{
// 0x10fcae4c mov x26, x0
cpu.x[26]=(((cpu.x[0]))) as u64;
// 0x10fcae50 mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10fcae54 bl #0x10f86db0
cpu.x[30]=0x10fcae58;super::chunk2::f_10f86db0(cpu,rt,0x10f86db0)?;
pc=0x10fcae58;continue;
}
0x10fcae58=>{
// 0x10fcae58 cmp w0, w22
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[22] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcae5c b.eq #0x10fcae28
if cpu.condition(0){pc=0x10fcae28;continue;}pc=0x10fcae60;continue;
}
0x10fcae60=>{
// 0x10fcae60 cbz x26, #0x10fcae28
if (cpu.x[26]) == 0 {pc=0x10fcae28;continue;}pc=0x10fcae64;continue;
}
0x10fcae64=>{
// 0x10fcae64 cmn w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.flags(a,b,result,32,false);}
// 0x10fcae68 b.eq #0x10fcae28
if cpu.condition(0){pc=0x10fcae28;continue;}pc=0x10fcae6c;continue;
}
0x10fcae6c=>{
// 0x10fcae6c ldr w1, [x25, #0x64]
{let address=(cpu.x[25]).wrapping_add(0x64u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10fcae70 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcae74 bl #0x10d3cac8
cpu.x[30]=0x10fcae78;super::chunk5::f_10d3cac8(cpu,rt,0x10d3cac8)?;
pc=0x10fcae78;continue;
}
0x10fcae78=>{
// 0x10fcae78 cbz x0, #0x10fcae28
if (cpu.x[0]) == 0 {pc=0x10fcae28;continue;}pc=0x10fcae7c;continue;
}
0x10fcae7c=>{
// 0x10fcae7c ldr x1, [x26, #0x10]
{let address=(cpu.x[26]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10fcae80 ldr w2, [x25, #0x68]
{let address=(cpu.x[25]).wrapping_add(0x68u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[2]=(v0) as u32 as u64;}
// 0x10fcae84 mov w4, w19
cpu.x[4]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10fcae88 ldrb w3, [x25, #0x6c]
{let address=(cpu.x[25]).wrapping_add(0x6cu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10fcae8c bl #0x10d549d0
cpu.x[30]=0x10fcae90;super::chunk3::f_10d549d0(cpu,rt,0x10d549d0)?;
pc=0x10fcae90;continue;
}
0x10fcae90=>{
// 0x10fcae90 b #0x10fcae28
pc=0x10fcae28;continue;
}
0x10fcae94=>{
// 0x10fcae94 ldp x20, x19, [sp, #0xb0]
{let address=(cpu.sp).wrapping_add(0xb0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fcae98 ldp x22, x21, [sp, #0xa0]
{let address=(cpu.sp).wrapping_add(0xa0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fcae9c ldp x24, x23, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fcaea0 ldp x26, x25, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fcaea4 ldp x28, x27, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fcaea8 ldp x29, x30, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fcaeac add sp, sp, #0xc0
{let a=((cpu.sp));let b=((0xc0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
pc=0x10fcaeb0;continue;
}
0x10fcaeb0=>{
// 0x10fcaeb0 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10fcaeb4=>{
// 0x10fcaeb4 ldur w27, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[27]=(v0) as u32 as u64;}
// 0x10fcaeb8 ldur x25, [x29, #-0x10]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[25]=(v0) as u64;}
// 0x10fcaebc mov w23, wzr
cpu.x[23]=(((0u64))) as u32 as u64;
// 0x10fcaec0 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10fcaec4 bl #0x10f95bec
cpu.x[30]=0x10fcaec8;super::chunk6::f_10f95bec(cpu,rt,0x10f95bec)?;
pc=0x10fcaec8;continue;
}
0x10fcaec8=>{
// 0x10fcaec8 mov w1, w0
cpu.x[1]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fcaecc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fcaed0 bl #0x10d3cac8
cpu.x[30]=0x10fcaed4;super::chunk5::f_10d3cac8(cpu,rt,0x10d3cac8)?;
pc=0x10fcaed4;continue;
}
0x10fcaed4=>{
// 0x10fcaed4 cbnz x0, #0x10fcacf4
if (cpu.x[0]) != 0 {pc=0x10fcacf4;continue;}pc=0x10fcaed8;continue;
}
0x10fcaed8=>{
// 0x10fcaed8 b #0x10fcad4c
pc=0x10fcad4c;continue;
}
0x10fcaedc=>{
// 0x10fcaedc ldrb w8, [x0, #0xa9]
{let address=(cpu.x[0]).wrapping_add(0xa9u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcaee0 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcaee4 cset w26, ne
cpu.x[26]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x10fcaee8 b #0x10fcacd0
pc=0x10fcacd0;continue;
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fcaa2c")),}}}
#[inline]
pub(super) unsafe fn f_10fd0be4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fd0be4=>{
// 0x10fd0be4 stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fd0be8 str x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fd0bec stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fd0bf0 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fd0bf4 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fd0bf8 ldr x8, [x1]
{let address=(cpu.x[1]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fd0bfc mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
pc=0x10fd0c00;continue;
}
0x10fd0c00=>{
// 0x10fd0c00 mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x10fd0c04 mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10fd0c08 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fd0c0c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fd0c10;super::dispatch(cpu,rt,target)?;}
pc=0x10fd0c10;continue;
}
0x10fd0c10=>{
// 0x10fd0c10 ldr w23, [x20, #0x1c]
{let address=(cpu.x[20]).wrapping_add(0x1cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[23]=(v0) as u32 as u64;}
// 0x10fd0c14 subs w22, w0, w23
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[23] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[22]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10fd0c18 b.le #0x10fd0ca0
if cpu.condition(13){pc=0x10fd0ca0;continue;}pc=0x10fd0c1c;continue;
}
0x10fd0c1c=>{
// 0x10fd0c1c ldr w8, [x20, #0x18]
{let address=(cpu.x[20]).wrapping_add(0x18u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fd0c20 mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fd0c24 mov w9, w23
cpu.x[9]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10fd0c28 cmp w8, w0
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fd0c2c b.ge #0x10fd0c40
if cpu.condition(10){pc=0x10fd0c40;continue;}pc=0x10fd0c30;continue;
}
0x10fd0c30=>{
// 0x10fd0c30 add x0, x20, #0x10
{let a=((cpu.x[20]));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fd0c34 mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10fd0c38 bl #0x10fd0d6c
cpu.x[30]=0x10fd0c3c;super::chunk11::f_10fd0d6c(cpu,rt,0x10fd0d6c)?;
// 0x10fd0c3c ldr w9, [x20, #0x1c]
{let address=(cpu.x[20]).wrapping_add(0x1cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
pc=0x10fd0c40;continue;
}
0x10fd0c40=>{
// 0x10fd0c40 sxtw x8, w23
cpu.x[8]=((((cpu.x[23] as u32 as u64))) as i32 as i64 as u64) as u64;
// 0x10fd0c44 cmp w9, w21
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[21] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fd0c48 b.ge #0x10fd0c68
if cpu.condition(10){pc=0x10fd0c68;continue;}pc=0x10fd0c4c;continue;
}
0x10fd0c4c=>{
// 0x10fd0c4c sxtw x9, w9
cpu.x[9]=((((cpu.x[9] as u32 as u64))) as i32 as i64 as u64) as u64;
// 0x10fd0c50 sxtw x10, w21
cpu.x[10]=((((cpu.x[21] as u32 as u64))) as i32 as i64 as u64) as u64;
pc=0x10fd0c54;continue;
}
0x10fd0c54=>{
// 0x10fd0c54 ldur x11, [x20, #0x10]
{let address=(cpu.x[20]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[11]=(v0) as u64;}
// 0x10fd0c58 str xzr, [x11, x9, lsl #3]
{let address=((cpu.x[11]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[9])) as u64).wrapping_shl(3) as u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fd0c5c add x9, x9, #1
{let a=((cpu.x[9]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
// 0x10fd0c60 cmp x10, x9
{let a=((cpu.x[10]));let b=((cpu.x[9]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fd0c64 b.ne #0x10fd0c54
if cpu.condition(1){pc=0x10fd0c54;continue;}pc=0x10fd0c68;continue;
}
0x10fd0c68=>{
// 0x10fd0c68 lsl x23, x8, #3
cpu.x[23]=(((((cpu.x[8]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10fd0c6c str w21, [x20, #0x1c]
{let address=(cpu.x[20]).wrapping_add(0x1cu64);let v0=cpu.x[21] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10fd0c70;continue;
}
0x10fd0c70=>{
// 0x10fd0c70 mov w0, #0x40
cpu.x[0]=(((0x40u64))) as u32 as u64;
// 0x10fd0c74 bl #0x1187d2b0
cpu.x[30]=0x10fd0c78;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
// 0x10fd0c78 movi v0.2d, #0000000000000000
cpu.q[0]=(lane_dup(0u128,64,2)) & 0xffffffffffffffffffffffffffffffffu128;
// 0x10fd0c7c ldur x8, [x20, #0x10]
{let address=(cpu.x[20]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fd0c80 str wzr, [x0, #0x20]
{let address=(cpu.x[0]).wrapping_add(0x20u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fd0c84 stp xzr, xzr, [x0, #0x30]
{let address=(cpu.x[0]).wrapping_add(0x30u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fd0c88 subs w22, w22, #1
{let a=((cpu.x[22] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[22]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10fd0c8c str xzr, [x0, #0x28]
{let address=(cpu.x[0]).wrapping_add(0x28u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fd0c90 stp q0, q0, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10fd0c94 str x0, [x8, x23]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[23])));let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fd0c98 add x23, x23, #8
{let a=((cpu.x[23]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[23]=(result) as u64;}
// 0x10fd0c9c b.ne #0x10fd0c70
if cpu.condition(1){pc=0x10fd0c70;continue;}pc=0x10fd0ca0;continue;
}
0x10fd0ca0=>{
// 0x10fd0ca0 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fd0ca4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fd0ca8 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fd0cac blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fd0cb0;super::dispatch(cpu,rt,target)?;}
pc=0x10fd0cb0;continue;
}
0x10fd0cb0=>{
// 0x10fd0cb0 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fd0cb4 str w0, [x20, #0x20]
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fd0cb8 b.lt #0x10fd0ce4
if cpu.condition(11){pc=0x10fd0ce4;continue;}pc=0x10fd0cbc;continue;
}
0x10fd0cbc=>{
// 0x10fd0cbc mov x21, xzr
cpu.x[21]=(((0u64))) as u64;
pc=0x10fd0cc0;continue;
}
0x10fd0cc0=>{
// 0x10fd0cc0 ldp x2, x8, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[2]=(v0) as u64;cpu.x[8]=(v1) as u64;}
// 0x10fd0cc4 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10fd0cc8 ldr x3, [x20, #0x28]
{let address=(cpu.x[20]).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[3]=(v0) as u64;}
// 0x10fd0ccc ldr x0, [x8, x21, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[21])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fd0cd0 bl #0x10fd0fcc
cpu.x[30]=0x10fd0cd4;super::dispatch(cpu,rt,0x10fd0fcc)?;
// 0x10fd0cd4 ldrsw x8, [x20, #0x20]
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10fd0cd8 add x21, x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[21]=(result) as u64;}
// 0x10fd0cdc cmp x21, x8
{let a=((cpu.x[21]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fd0ce0 b.lt #0x10fd0cc0
if cpu.condition(11){pc=0x10fd0cc0;continue;}pc=0x10fd0ce4;continue;
}
0x10fd0ce4=>{
// 0x10fd0ce4 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fd0ce8 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10fd0cec ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fd0cf0 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10fd0cf4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fd0be4")),}}}
#[inline]
pub(super) unsafe fn f_10fea548(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fea548=>{
// 0x10fea548 stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fea54c stp x24, x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fea550 stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fea554 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fea558 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fea55c adrp x8, #0x1193b000
cpu.x[8]=(0x1193b000u64) as u64;
// 0x10fea560 add x8, x8, #0xcb0
{let a=((cpu.x[8]));let b=((0xcb0u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fea564 mov x19, x2
cpu.x[19]=(((cpu.x[2]))) as u64;
// 0x10fea568 mov x21, x1
cpu.x[21]=(((cpu.x[1]))) as u64;
// 0x10fea56c mov x22, x0
cpu.x[22]=(((cpu.x[0]))) as u64;
// 0x10fea570 stp x8, xzr, [x0], #0x10
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=cpu.x[8];let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.x[0]=((cpu.x[0]).wrapping_add(((0x10u64)))) as u64;}
// 0x10fea574 bl #0x11058798
cpu.x[30]=0x10fea578;super::chunk1::f_11058798(cpu,rt,0x11058798)?;
pc=0x10fea578;continue;
}
0x10fea578=>{
// 0x10fea578 movi v0.2d, #0000000000000000
cpu.q[0]=(lane_dup(0u128,64,2)) & 0xffffffffffffffffffffffffffffffffu128;
// 0x10fea57c mov x20, x22
cpu.x[20]=(((cpu.x[22]))) as u64;
// 0x10fea580 mov w9, #0x28
cpu.x[9]=(((0x28u64))) as u32 as u64;
// 0x10fea584 str xzr, [x22, #8]
{let address=(cpu.x[22]).wrapping_add(0x8u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fea588 str q0, [x20, #0x20]!
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);cpu.x[20]=(address) as u64;}
// 0x10fea58c mov x23, x20
cpu.x[23]=(((cpu.x[20]))) as u64;
// 0x10fea590 str wzr, [x20, #0x30]
{let address=(cpu.x[20]).wrapping_add(0x30u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fea594 str q0, [x23, #0x20]!
{let address=(cpu.x[23]).wrapping_add(0x20u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);cpu.x[23]=(address) as u64;}
// 0x10fea598 str q0, [x20, #0x10]
{let address=(cpu.x[20]).wrapping_add(0x10u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10fea59c ldr w8, [x19, #0xc]
{let address=(cpu.x[19]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fea5a0 cmp w8, #0x28
{let a=((cpu.x[8] as u32 as u64));let b=((0x28u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fea5a4 csel w24, w8, w9, lt
cpu.x[24]=(if cpu.condition(11){((cpu.x[8] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10fea5a8 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fea5ac b.lt #0x10fea5e0
if cpu.condition(11){pc=0x10fea5e0;continue;}pc=0x10fea5b0;continue;
}
0x10fea5b0=>{
// 0x10fea5b0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fea5b4 mov w1, w24
cpu.x[1]=(((cpu.x[24] as u32 as u64))) as u32 as u64;
// 0x10fea5b8 bl #0x108c5574
cpu.x[30]=0x10fea5bc;super::chunk12::f_108c5574(cpu,rt,0x108c5574)?;
pc=0x10fea5bc;continue;
}
0x10fea5bc=>{
// 0x10fea5bc ldrsw x8, [x22, #0x2c]
{let address=(cpu.x[22]).wrapping_add(0x2cu64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10fea5c0 cmp w8, w24
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[24] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fea5c4 b.ge #0x10fea5ec
if cpu.condition(10){pc=0x10fea5ec;continue;}pc=0x10fea5c8;continue;
}
0x10fea5c8=>{
// 0x10fea5c8 ldr x9, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fea5cc str xzr, [x9, x8, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[8])) as u64).wrapping_shl(3) as u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fea5d0 add x8, x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fea5d4 cmp x24, x8
{let a=((cpu.x[24]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fea5d8 b.ne #0x10fea5c8
if cpu.condition(1){pc=0x10fea5c8;continue;}pc=0x10fea5dc;continue;
}
0x10fea5dc=>{
// 0x10fea5dc b #0x10fea5ec
pc=0x10fea5ec;continue;
}
0x10fea5e0=>{
// 0x10fea5e0 tbnz w8, #0x1f, #0x10fea5ec
if ((cpu.x[8] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x10fea5ec;continue;}pc=0x10fea5e4;continue;
}
0x10fea5e4=>{
// 0x10fea5e4 mov w8, wzr
cpu.x[8]=(((0u64))) as u32 as u64;
// 0x10fea5e8 b #0x10fea618
pc=0x10fea618;continue;
}
0x10fea5ec=>{
// 0x10fea5ec ldr w8, [x22, #0x4c]
{let address=(cpu.x[22]).wrapping_add(0x4cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fea5f0 str w24, [x22, #0x2c]
{let address=(cpu.x[22]).wrapping_add(0x2cu64);let v0=cpu.x[24] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fea5f4 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fea5f8 b.le #0x10fea618
if cpu.condition(13){pc=0x10fea618;continue;}pc=0x10fea5fc;continue;
}
0x10fea5fc=>{
// 0x10fea5fc cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fea600 b.ne #0x10fea658
if cpu.condition(1){pc=0x10fea658;continue;}pc=0x10fea604;continue;
}
0x10fea604=>{
// 0x10fea604 ldr w8, [x19, #0xc]
{let address=(cpu.x[19]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fea608 str x21, [x22, #8]
{let address=(cpu.x[22]).wrapping_add(0x8u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fea60c cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fea610 b.ge #0x10fea670
if cpu.condition(10){pc=0x10fea670;continue;}pc=0x10fea614;continue;
}
0x10fea614=>{
// 0x10fea614 b #0x10fea6a0
pc=0x10fea6a0;continue;
}
0x10fea618=>{
// 0x10fea618 ldr w9, [x22, #0x48]
{let address=(cpu.x[22]).wrapping_add(0x48u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fea61c cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fea620 b.le #0x10fea640
if cpu.condition(13){pc=0x10fea640;continue;}pc=0x10fea624;continue;
}
0x10fea624=>{
// 0x10fea624 sxtw x8, w8
cpu.x[8]=((((cpu.x[8] as u32 as u64))) as i32 as i64 as u64) as u64;
pc=0x10fea628;continue;
}
0x10fea628=>{
// 0x10fea628 ldr x9, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fea62c str xzr, [x9, x8, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[8])) as u64).wrapping_shl(3) as u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fea630 add x8, x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fea634 cmp x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fea638 b.ne #0x10fea628
if cpu.condition(1){pc=0x10fea628;continue;}pc=0x10fea63c;continue;
}
0x10fea63c=>{
// 0x10fea63c b #0x10fea658
pc=0x10fea658;continue;
}
0x10fea640=>{
// 0x10fea640 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fea644 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10fea648 bl #0x10feda98
cpu.x[30]=0x10fea64c;super::chunk0::f_10feda98(cpu,rt,0x10feda98)?;
pc=0x10fea64c;continue;
}
0x10fea64c=>{
// 0x10fea64c ldr w8, [x22, #0x4c]
{let address=(cpu.x[22]).wrapping_add(0x4cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fea650 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fea654 b.le #0x10fea624
if cpu.condition(13){pc=0x10fea624;continue;}pc=0x10fea658;continue;
}
0x10fea658=>{
// 0x10fea658 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10fea65c str w8, [x22, #0x4c]
{let address=(cpu.x[22]).wrapping_add(0x4cu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fea660 ldr w8, [x19, #0xc]
{let address=(cpu.x[19]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fea664 str x21, [x22, #8]
{let address=(cpu.x[22]).wrapping_add(0x8u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fea668 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fea66c b.lt #0x10fea6a0
if cpu.condition(11){pc=0x10fea6a0;continue;}pc=0x10fea670;continue;
}
0x10fea670=>{
// 0x10fea670 lsl x8, x8, #3
cpu.x[8]=(((((cpu.x[8]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10fea674 mov x9, xzr
cpu.x[9]=(((0u64))) as u64;
// 0x10fea678 b #0x10fea688
pc=0x10fea688;continue;
}
0x10fea67c=>{
// 0x10fea67c add x9, x9, #8
{let a=((cpu.x[9]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
// 0x10fea680 cmp x8, x9
{let a=((cpu.x[8]));let b=((cpu.x[9]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fea684 b.eq #0x10fea6a0
if cpu.condition(0){pc=0x10fea6a0;continue;}pc=0x10fea688;continue;
}
0x10fea688=>{
// 0x10fea688 ldr x10, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10fea68c ldr x10, [x10, x9]
{let address=((cpu.x[10]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[9])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10fea690 cbz x10, #0x10fea67c
if (cpu.x[10]) == 0 {pc=0x10fea67c;continue;}pc=0x10fea694;continue;
}
0x10fea694=>{
// 0x10fea694 ldr x11, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[11]=(v0) as u64;}
// 0x10fea698 str x10, [x11, x9]
{let address=((cpu.x[11]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[9])));let v0=cpu.x[10];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fea69c b #0x10fea67c
pc=0x10fea67c;continue;
}
0x10fea6a0=>{
// 0x10fea6a0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fea6a4 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fea6a8 ldp x24, x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fea6ac ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10fea6b0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fea548")),}}}
#[inline]
pub(super) unsafe fn f_10fec3c0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fec3c0=>{
// 0x10fec3c0 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fec3c4 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fec3c8 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fec3cc mov w19, w1
cpu.x[19]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x10fec3d0 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10fec3d4 tbnz w1, #0x1f, #0x10fec3f8
if ((cpu.x[1] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x10fec3f8;continue;}pc=0x10fec3d8;continue;
}
0x10fec3d8=>{
// 0x10fec3d8 ldr w8, [x20, #0x4c]
{let address=(cpu.x[20]).wrapping_add(0x4cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fec3dc cmp w8, w19
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fec3e0 b.le #0x10fec3f8
if cpu.condition(13){pc=0x10fec3f8;continue;}pc=0x10fec3e4;continue;
}
0x10fec3e4=>{
// 0x10fec3e4 ldr x8, [x20, #0x40]
{let address=(cpu.x[20]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fec3e8 ldr x0, [x8, w19, sxtw #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[19] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fec3ec ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fec3f0 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10fec3f4 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10fec3f8=>{
// 0x10fec3f8 adrp x0, #0x102e6000
cpu.x[0]=(0x102e6000u64) as u64;
// 0x10fec3fc add x0, x0, #0x50f
{let a=((cpu.x[0]));let b=((0x50fu64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fec400 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10fec404 bl #0x1128df18
cpu.x[30]=0x10fec408;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10fec408 ldr x8, [x20, #0x40]
{let address=(cpu.x[20]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fec40c ldr x0, [x8, w19, sxtw #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[19] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fec410 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fec414 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10fec418 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fec3c0")),}}}
#[inline]
pub(super) unsafe fn f_10fecdec(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fecdec=>{
// 0x10fecdec stp x29, x30, [sp, #-0x10]!
{let address=(cpu.sp).wrapping_add(0xfffffffffffffff0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fecdf0 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fecdf4 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fecdf8 ldr x8, [x8, #0x78]
{let address=(cpu.x[8]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fecdfc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fece00;super::dispatch(cpu,rt,target)?;}
pc=0x10fece00;continue;
}
0x10fece00=>{
// 0x10fece00 cmp w0, #0
{let a=((cpu.x[0] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fece04 cset w0, eq
cpu.x[0]=((cpu.condition(0)) as u64) as u32 as u64;
// 0x10fece08 ldp x29, x30, [sp], #0x10
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x10u64)))) as u64;}
// 0x10fece0c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fecdec")),}}}
#[inline]
pub(super) unsafe fn f_11058310(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x11058310=>{
// 0x11058310 sub sp, sp, #0xa0
{let a=((cpu.sp));let b=((0xa0u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x11058314 stp x29, x30, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x11058318 str x27, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=cpu.x[27];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x1105831c stp x26, x25, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=cpu.x[26];let v1=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x11058320 stp x24, x23, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x11058324 stp x22, x21, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x11058328 stp x20, x19, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x1105832c add x29, sp, #0x40
{let a=((cpu.sp));let b=((0x40u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x11058330 ldr x20, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x11058334 mov x21, x1
cpu.x[21]=(((cpu.x[1]))) as u64;
// 0x11058338 mov x23, x0
cpu.x[23]=(((cpu.x[0]))) as u64;
// 0x1105833c mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x11058340 mov w22, w2
cpu.x[22]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x11058344 mov x19, x8
cpu.x[19]=(((cpu.x[8]))) as u64;
// 0x11058348 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x1105834c bl #0x10f9de2c
cpu.x[30]=0x11058350;super::chunk14::f_10f9de2c(cpu,rt,0x10f9de2c)?;
pc=0x11058350;continue;
}
0x11058350=>{
// 0x11058350 mov w24, w0
cpu.x[24]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x11058354 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x11058358 mov w1, w24
cpu.x[1]=(((cpu.x[24] as u32 as u64))) as u32 as u64;
// 0x1105835c bl #0x10fa461c
cpu.x[30]=0x11058360;super::chunk10::f_10fa461c(cpu,rt,0x10fa461c)?;
pc=0x11058360;continue;
}
0x11058360=>{
// 0x11058360 tbz w0, #0, #0x1105836c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x1105836c;continue;}pc=0x11058364;continue;
}
0x11058364=>{
// 0x11058364 mov w25, #1
cpu.x[25]=(((0x1u64))) as u32 as u64;
// 0x11058368 b #0x11058384
pc=0x11058384;continue;
}
0x1105836c=>{
// 0x1105836c ldr w0, [x23, #0x1c]
{let address=(cpu.x[23]).wrapping_add(0x1cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x11058370 mov w1, #2
cpu.x[1]=(((0x2u64))) as u32 as u64;
// 0x11058374 mov w25, #2
cpu.x[25]=(((0x2u64))) as u32 as u64;
// 0x11058378 bl #0x10dd4bdc
cpu.x[30]=0x1105837c;super::chunk6::f_10dd4bdc(cpu,rt,0x10dd4bdc)?;
pc=0x1105837c;continue;
}
0x1105837c=>{
// 0x1105837c tst w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.logical_flags(result,32);}
// 0x11058380 csel w25, w25, wzr, ne
cpu.x[25]=(if cpu.condition(1){((cpu.x[25] as u32 as u64))}else{((0u64))}) as u32 as u64;
pc=0x11058384;continue;
}
0x11058384=>{
// 0x11058384 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11058388 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x1105838c ldr x8, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11058390 blr x8
{let target=cpu.x[8];cpu.x[30]=0x11058394;super::dispatch(cpu,rt,target)?;}
pc=0x11058394;continue;
}
0x11058394=>{
// 0x11058394 mov w27, w0
cpu.x[27]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x11058398 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x1105839c bl #0x10e7e7d0
cpu.x[30]=0x110583a0;super::chunk14::f_10e7e7d0(cpu,rt,0x10e7e7d0)?;
pc=0x110583a0;continue;
}
0x110583a0=>{
// 0x110583a0 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110583a4 ldr x8, [x8, #0x118]
{let address=(cpu.x[8]).wrapping_add(0x118u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110583a8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110583ac;super::dispatch(cpu,rt,target)?;}
pc=0x110583ac;continue;
}
0x110583ac=>{
// 0x110583ac ldr w26, [x23, #0x1c]
{let address=(cpu.x[23]).wrapping_add(0x1cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[26]=(v0) as u32 as u64;}
// 0x110583b0 and w8, w0, #0xff
{let a=((cpu.x[0] as u32 as u64));let b=((0xffu64));let result=a & b;cpu.x[8]=(result) as u32 as u64;}
// 0x110583b4 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x110583b8 add w27, w8, w27, sxtb
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[27] as u32 as u64) as i8 as i64 as u64));let result=a.wrapping_add(b);cpu.x[27]=(result) as u32 as u64;}
// 0x110583bc mov w0, w26
cpu.x[0]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x110583c0 bl #0x10dd4bdc
cpu.x[30]=0x110583c4;super::chunk6::f_10dd4bdc(cpu,rt,0x10dd4bdc)?;
pc=0x110583c4;continue;
}
0x110583c4=>{
// 0x110583c4 tbz w0, #0, #0x110583dc
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x110583dc;continue;}pc=0x110583c8;continue;
}
0x110583c8=>{
// 0x110583c8 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x110583cc mov w1, w24
cpu.x[1]=(((cpu.x[24] as u32 as u64))) as u32 as u64;
// 0x110583d0 bl #0x10fa46a4
cpu.x[30]=0x110583d4;super::dispatch(cpu,rt,0x10fa46a4)?;
// 0x110583d4 add w7, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[7]=(result) as u32 as u64;}
// 0x110583d8 b #0x110583e0
pc=0x110583e0;continue;
}
0x110583dc=>{
// 0x110583dc mov w7, wzr
cpu.x[7]=(((0u64))) as u32 as u64;
pc=0x110583e0;continue;
}
0x110583e0=>{
// 0x110583e0 ldr w9, [x23, #0x24]
{let address=(cpu.x[23]).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x110583e4 mov x8, x19
cpu.x[8]=(((cpu.x[19]))) as u64;
// 0x110583e8 mov w0, w24
cpu.x[0]=(((cpu.x[24] as u32 as u64))) as u32 as u64;
// 0x110583ec mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x110583f0 mov w2, w25
cpu.x[2]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x110583f4 mov w3, #-1
cpu.x[3]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x110583f8 mov w4, wzr
cpu.x[4]=(((0u64))) as u32 as u64;
// 0x110583fc mov w5, w27
cpu.x[5]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
pc=0x11058400;continue;
}
0x11058400=>{
// 0x11058400 mov w6, w26
cpu.x[6]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x11058404 str w9, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x11058408 bl #0x10e80c00
cpu.x[30]=0x1105840c;super::chunk14::f_10e80c00(cpu,rt,0x10e80c00)?;
pc=0x1105840c;continue;
}
0x1105840c=>{
// 0x1105840c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x11058410 bl #0x10e80680
cpu.x[30]=0x11058414;super::chunk0::f_10e80680(cpu,rt,0x10e80680)?;
pc=0x11058414;continue;
}
0x11058414=>{
// 0x11058414 mov x1, x0
cpu.x[1]=(((cpu.x[0]))) as u64;
// 0x11058418 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x1105841c tbz w22, #0, #0x11058440
if ((cpu.x[22] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x11058440;continue;}pc=0x11058420;continue;
}
0x11058420=>{
// 0x11058420 bl #0x10fa4304
cpu.x[30]=0x11058424;super::chunk6::f_10fa4304(cpu,rt,0x10fa4304)?;
pc=0x11058424;continue;
}
0x11058424=>{
// 0x11058424 mov w1, w0
cpu.x[1]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x11058428 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x1105842c bl #0x10e80644
cpu.x[30]=0x11058430;super::chunk12::f_10e80644(cpu,rt,0x10e80644)?;
pc=0x11058430;continue;
}
0x11058430=>{
// 0x11058430 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x11058434 bl #0x10e7edfc
cpu.x[30]=0x11058438;super::chunk4::f_10e7edfc(cpu,rt,0x10e7edfc)?;
pc=0x11058438;continue;
}
0x11058438=>{
// 0x11058438 tbnz w0, #0, #0x1105845c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x1105845c;continue;}pc=0x1105843c;continue;
}
0x1105843c=>{
// 0x1105843c b #0x11058498
pc=0x11058498;continue;
}
0x11058440=>{
// 0x11058440 bl #0x10fa3e98
cpu.x[30]=0x11058444;super::dispatch(cpu,rt,0x10fa3e98)?;
// 0x11058444 mov w1, w0
cpu.x[1]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x11058448 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x1105844c bl #0x10e80644
cpu.x[30]=0x11058450;super::chunk12::f_10e80644(cpu,rt,0x10e80644)?;
// 0x11058450 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x11058454 bl #0x10e7edfc
cpu.x[30]=0x11058458;super::chunk4::f_10e7edfc(cpu,rt,0x10e7edfc)?;
// 0x11058458 tbz w0, #0, #0x11058498
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x11058498;continue;}pc=0x1105845c;continue;
}
0x1105845c=>{
// 0x1105845c sub x8, x29, #0x18
{let a=((cpu.x[29]));let b=((0x18u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u64;}
// 0x11058460 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x11058464 tbz w22, #0, #0x11058470
if ((cpu.x[22] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x11058470;continue;}pc=0x11058468;continue;
}
0x11058468=>{
// 0x11058468 bl #0x10fa4228
cpu.x[30]=0x1105846c;super::dispatch(cpu,rt,0x10fa4228)?;
// 0x1105846c b #0x11058474
pc=0x11058474;continue;
}
0x11058470=>{
// 0x11058470 bl #0x10fa344c
cpu.x[30]=0x11058474;super::dispatch(cpu,rt,0x10fa344c)?;
pc=0x11058474;continue;
}
0x11058474=>{
// 0x11058474 add x8, sp, #0x10
{let a=((cpu.sp));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x11058478 sub x1, x29, #0x18
{let a=((cpu.x[29]));let b=((0x18u64));let result=a.wrapping_sub(b);cpu.x[1]=(result) as u64;}
// 0x1105847c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x11058480 mov x2, x19
cpu.x[2]=(((cpu.x[19]))) as u64;
// 0x11058484 bl #0x10e80c50
cpu.x[30]=0x11058488;super::dispatch(cpu,rt,0x10e80c50)?;
// 0x11058488 ldr q0, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));cpu.q[0]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x1105848c ldr w8, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x11058490 str q0, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x11058494 str w8, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x11058498;continue;
}
0x11058498=>{
// 0x11058498 ldp x20, x19, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x1105849c ldr x27, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[27]=(v0) as u64;}
// 0x110584a0 ldp x22, x21, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x110584a4 ldp x24, x23, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x110584a8 ldp x26, x25, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x110584ac ldp x29, x30, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x110584b0 add sp, sp, #0xa0
{let a=((cpu.sp));let b=((0xa0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x110584b4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x11058310")),}}}
#[inline]
pub(super) unsafe fn f_110585b8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x110585b8=>{
// 0x110585b8 b #0x1187d2c0
super::dispatch(cpu,rt,0x1187d2c0)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x110585b8")),}}}
#[inline]
pub(super) unsafe fn f_110c9a30(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x110c9a30=>{
// 0x110c9a30 stp x29, x30, [sp, #-0x50]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffb0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x110c9a34 str x25, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x110c9a38 stp x24, x23, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x110c9a3c stp x22, x21, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x110c9a40 stp x20, x19, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x110c9a44 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x110c9a48 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x110c9a4c ldr x0, [x0, #0xd8]
{let address=(cpu.x[0]).wrapping_add(0xd8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9a50 cbz x0, #0x110c9a78
if (cpu.x[0]) == 0 {pc=0x110c9a78;continue;}pc=0x110c9a54;continue;
}
0x110c9a54=>{
// 0x110c9a54 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9a58 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9a5c blr x8
{let target=cpu.x[8];cpu.x[30]=0x110c9a60;super::dispatch(cpu,rt,target)?;}
pc=0x110c9a60;continue;
}
0x110c9a60=>{
// 0x110c9a60 ldr x0, [x19, #0xd8]
{let address=(cpu.x[19]).wrapping_add(0xd8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9a64 cbz x0, #0x110c9a74
if (cpu.x[0]) == 0 {pc=0x110c9a74;continue;}pc=0x110c9a68;continue;
}
0x110c9a68=>{
// 0x110c9a68 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9a6c ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9a70 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110c9a74;super::dispatch(cpu,rt,target)?;}
pc=0x110c9a74;continue;
}
0x110c9a74=>{
// 0x110c9a74 str xzr, [x19, #0xd8]
{let address=(cpu.x[19]).wrapping_add(0xd8u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x110c9a78;continue;
}
0x110c9a78=>{
// 0x110c9a78 ldr x20, [x19, #0x118]
{let address=(cpu.x[19]).wrapping_add(0x118u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x110c9a7c cbz x20, #0x110c9a94
if (cpu.x[20]) == 0 {pc=0x110c9a94;continue;}pc=0x110c9a80;continue;
}
0x110c9a80=>{
// 0x110c9a80 ldr x0, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9a84 cbz x0, #0x110c9a8c
if (cpu.x[0]) == 0 {pc=0x110c9a8c;continue;}pc=0x110c9a88;continue;
}
0x110c9a88=>{
// 0x110c9a88 bl #0x1187d360
cpu.x[30]=0x110c9a8c;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x110c9a8c;continue;
}
0x110c9a8c=>{
// 0x110c9a8c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x110c9a90 bl #0x1187d2c0
cpu.x[30]=0x110c9a94;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x110c9a94;continue;
}
0x110c9a94=>{
// 0x110c9a94 ldr x0, [x19, #0xe0]
{let address=(cpu.x[19]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9a98 str xzr, [x19, #0x118]
{let address=(cpu.x[19]).wrapping_add(0x118u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x110c9a9c cbz x0, #0x110c9ac4
if (cpu.x[0]) == 0 {pc=0x110c9ac4;continue;}pc=0x110c9aa0;continue;
}
0x110c9aa0=>{
// 0x110c9aa0 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9aa4 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9aa8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110c9aac;super::dispatch(cpu,rt,target)?;}
pc=0x110c9aac;continue;
}
0x110c9aac=>{
// 0x110c9aac ldr x0, [x19, #0xe0]
{let address=(cpu.x[19]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9ab0 cbz x0, #0x110c9ac0
if (cpu.x[0]) == 0 {pc=0x110c9ac0;continue;}pc=0x110c9ab4;continue;
}
0x110c9ab4=>{
// 0x110c9ab4 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9ab8 ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9abc blr x8
{let target=cpu.x[8];cpu.x[30]=0x110c9ac0;super::dispatch(cpu,rt,target)?;}
pc=0x110c9ac0;continue;
}
0x110c9ac0=>{
// 0x110c9ac0 str xzr, [x19, #0xe0]
{let address=(cpu.x[19]).wrapping_add(0xe0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x110c9ac4;continue;
}
0x110c9ac4=>{
// 0x110c9ac4 ldr x20, [x19, #0x120]
{let address=(cpu.x[19]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x110c9ac8 cbz x20, #0x110c9ae0
if (cpu.x[20]) == 0 {pc=0x110c9ae0;continue;}pc=0x110c9acc;continue;
}
0x110c9acc=>{
// 0x110c9acc ldr x0, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9ad0 cbz x0, #0x110c9ad8
if (cpu.x[0]) == 0 {pc=0x110c9ad8;continue;}pc=0x110c9ad4;continue;
}
0x110c9ad4=>{
// 0x110c9ad4 bl #0x1187d360
cpu.x[30]=0x110c9ad8;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x110c9ad8;continue;
}
0x110c9ad8=>{
// 0x110c9ad8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x110c9adc bl #0x1187d2c0
cpu.x[30]=0x110c9ae0;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x110c9ae0;continue;
}
0x110c9ae0=>{
// 0x110c9ae0 ldr x0, [x19, #0xe8]
{let address=(cpu.x[19]).wrapping_add(0xe8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9ae4 str xzr, [x19, #0x120]
{let address=(cpu.x[19]).wrapping_add(0x120u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x110c9ae8 cbz x0, #0x110c9b10
if (cpu.x[0]) == 0 {pc=0x110c9b10;continue;}pc=0x110c9aec;continue;
}
0x110c9aec=>{
// 0x110c9aec ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9af0 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9af4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110c9af8;super::dispatch(cpu,rt,target)?;}
// 0x110c9af8 ldr x0, [x19, #0xe8]
{let address=(cpu.x[19]).wrapping_add(0xe8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9afc cbz x0, #0x110c9b0c
if (cpu.x[0]) == 0 {pc=0x110c9b0c;continue;}pc=0x110c9b00;continue;
}
0x110c9b00=>{
// 0x110c9b00 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9b04 ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9b08 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110c9b0c;super::dispatch(cpu,rt,target)?;}
pc=0x110c9b0c;continue;
}
0x110c9b0c=>{
// 0x110c9b0c str xzr, [x19, #0xe8]
{let address=(cpu.x[19]).wrapping_add(0xe8u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x110c9b10;continue;
}
0x110c9b10=>{
// 0x110c9b10 ldr x20, [x19, #0x128]
{let address=(cpu.x[19]).wrapping_add(0x128u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x110c9b14 cbz x20, #0x110c9b2c
if (cpu.x[20]) == 0 {pc=0x110c9b2c;continue;}pc=0x110c9b18;continue;
}
0x110c9b18=>{
// 0x110c9b18 ldr x0, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9b1c cbz x0, #0x110c9b24
if (cpu.x[0]) == 0 {pc=0x110c9b24;continue;}pc=0x110c9b20;continue;
}
0x110c9b20=>{
// 0x110c9b20 bl #0x1187d360
cpu.x[30]=0x110c9b24;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x110c9b24;continue;
}
0x110c9b24=>{
// 0x110c9b24 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x110c9b28 bl #0x1187d2c0
cpu.x[30]=0x110c9b2c;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x110c9b2c;continue;
}
0x110c9b2c=>{
// 0x110c9b2c ldr x0, [x19, #0xf0]
{let address=(cpu.x[19]).wrapping_add(0xf0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9b30 str xzr, [x19, #0x128]
{let address=(cpu.x[19]).wrapping_add(0x128u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x110c9b34 cbz x0, #0x110c9b5c
if (cpu.x[0]) == 0 {pc=0x110c9b5c;continue;}pc=0x110c9b38;continue;
}
0x110c9b38=>{
// 0x110c9b38 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9b3c ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9b40 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110c9b44;super::dispatch(cpu,rt,target)?;}
// 0x110c9b44 ldr x0, [x19, #0xf0]
{let address=(cpu.x[19]).wrapping_add(0xf0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9b48 cbz x0, #0x110c9b58
if (cpu.x[0]) == 0 {pc=0x110c9b58;continue;}pc=0x110c9b4c;continue;
}
0x110c9b4c=>{
// 0x110c9b4c ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9b50 ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9b54 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110c9b58;super::dispatch(cpu,rt,target)?;}
pc=0x110c9b58;continue;
}
0x110c9b58=>{
// 0x110c9b58 str xzr, [x19, #0xf0]
{let address=(cpu.x[19]).wrapping_add(0xf0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x110c9b5c;continue;
}
0x110c9b5c=>{
// 0x110c9b5c ldr x20, [x19, #0x130]
{let address=(cpu.x[19]).wrapping_add(0x130u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x110c9b60 cbz x20, #0x110c9b78
if (cpu.x[20]) == 0 {pc=0x110c9b78;continue;}pc=0x110c9b64;continue;
}
0x110c9b64=>{
// 0x110c9b64 ldr x0, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9b68 cbz x0, #0x110c9b70
if (cpu.x[0]) == 0 {pc=0x110c9b70;continue;}pc=0x110c9b6c;continue;
}
0x110c9b6c=>{
// 0x110c9b6c bl #0x1187d360
cpu.x[30]=0x110c9b70;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x110c9b70;continue;
}
0x110c9b70=>{
// 0x110c9b70 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x110c9b74 bl #0x1187d2c0
cpu.x[30]=0x110c9b78;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x110c9b78;continue;
}
0x110c9b78=>{
// 0x110c9b78 ldr x0, [x19, #0xf8]
{let address=(cpu.x[19]).wrapping_add(0xf8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9b7c str xzr, [x19, #0x130]
{let address=(cpu.x[19]).wrapping_add(0x130u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x110c9b80 cbz x0, #0x110c9ba8
if (cpu.x[0]) == 0 {pc=0x110c9ba8;continue;}pc=0x110c9b84;continue;
}
0x110c9b84=>{
// 0x110c9b84 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9b88 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9b8c blr x8
{let target=cpu.x[8];cpu.x[30]=0x110c9b90;super::dispatch(cpu,rt,target)?;}
// 0x110c9b90 ldr x0, [x19, #0xf8]
{let address=(cpu.x[19]).wrapping_add(0xf8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9b94 cbz x0, #0x110c9ba4
if (cpu.x[0]) == 0 {pc=0x110c9ba4;continue;}pc=0x110c9b98;continue;
}
0x110c9b98=>{
// 0x110c9b98 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9b9c ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9ba0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110c9ba4;super::dispatch(cpu,rt,target)?;}
pc=0x110c9ba4;continue;
}
0x110c9ba4=>{
// 0x110c9ba4 str xzr, [x19, #0xf8]
{let address=(cpu.x[19]).wrapping_add(0xf8u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x110c9ba8;continue;
}
0x110c9ba8=>{
// 0x110c9ba8 ldr x20, [x19, #0x138]
{let address=(cpu.x[19]).wrapping_add(0x138u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x110c9bac cbz x20, #0x110c9bc4
if (cpu.x[20]) == 0 {pc=0x110c9bc4;continue;}pc=0x110c9bb0;continue;
}
0x110c9bb0=>{
// 0x110c9bb0 ldr x0, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9bb4 cbz x0, #0x110c9bbc
if (cpu.x[0]) == 0 {pc=0x110c9bbc;continue;}pc=0x110c9bb8;continue;
}
0x110c9bb8=>{
// 0x110c9bb8 bl #0x1187d360
cpu.x[30]=0x110c9bbc;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x110c9bbc;continue;
}
0x110c9bbc=>{
// 0x110c9bbc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x110c9bc0 bl #0x1187d2c0
cpu.x[30]=0x110c9bc4;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x110c9bc4;continue;
}
0x110c9bc4=>{
// 0x110c9bc4 ldr x0, [x19, #0x100]
{let address=(cpu.x[19]).wrapping_add(0x100u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9bc8 str xzr, [x19, #0x138]
{let address=(cpu.x[19]).wrapping_add(0x138u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x110c9bcc cbz x0, #0x110c9bf4
if (cpu.x[0]) == 0 {pc=0x110c9bf4;continue;}pc=0x110c9bd0;continue;
}
0x110c9bd0=>{
// 0x110c9bd0 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9bd4 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9bd8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110c9bdc;super::dispatch(cpu,rt,target)?;}
// 0x110c9bdc ldr x0, [x19, #0x100]
{let address=(cpu.x[19]).wrapping_add(0x100u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9be0 cbz x0, #0x110c9bf0
if (cpu.x[0]) == 0 {pc=0x110c9bf0;continue;}pc=0x110c9be4;continue;
}
0x110c9be4=>{
// 0x110c9be4 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9be8 ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9bec blr x8
{let target=cpu.x[8];cpu.x[30]=0x110c9bf0;super::dispatch(cpu,rt,target)?;}
pc=0x110c9bf0;continue;
}
0x110c9bf0=>{
// 0x110c9bf0 str xzr, [x19, #0x100]
{let address=(cpu.x[19]).wrapping_add(0x100u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x110c9bf4;continue;
}
0x110c9bf4=>{
// 0x110c9bf4 ldr x20, [x19, #0x140]
{let address=(cpu.x[19]).wrapping_add(0x140u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x110c9bf8 cbz x20, #0x110c9c10
if (cpu.x[20]) == 0 {pc=0x110c9c10;continue;}pc=0x110c9bfc;continue;
}
0x110c9bfc=>{
// 0x110c9bfc ldr x0, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9c00 cbz x0, #0x110c9c08
if (cpu.x[0]) == 0 {pc=0x110c9c08;continue;}pc=0x110c9c04;continue;
}
0x110c9c04=>{
// 0x110c9c04 bl #0x1187d360
cpu.x[30]=0x110c9c08;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x110c9c08;continue;
}
0x110c9c08=>{
// 0x110c9c08 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x110c9c0c bl #0x1187d2c0
cpu.x[30]=0x110c9c10;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x110c9c10;continue;
}
0x110c9c10=>{
// 0x110c9c10 ldr x0, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9c14 str xzr, [x19, #0x140]
{let address=(cpu.x[19]).wrapping_add(0x140u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x110c9c18 cbz x0, #0x110c9c3c
if (cpu.x[0]) == 0 {pc=0x110c9c3c;continue;}pc=0x110c9c1c;continue;
}
0x110c9c1c=>{
// 0x110c9c1c bl #0x110c9a30
cpu.x[30]=0x110c9c20;super::chunk15::f_110c9a30(cpu,rt,0x110c9a30)?;
// 0x110c9c20 ldr x20, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x110c9c24 cbz x20, #0x110c9c38
if (cpu.x[20]) == 0 {pc=0x110c9c38;continue;}pc=0x110c9c28;continue;
}
0x110c9c28=>{
// 0x110c9c28 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x110c9c2c bl #0x1099b50c
cpu.x[30]=0x110c9c30;super::chunk13::f_1099b50c(cpu,rt,0x1099b50c)?;
// 0x110c9c30 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x110c9c34 bl #0x1187d2c0
cpu.x[30]=0x110c9c38;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x110c9c38;continue;
}
0x110c9c38=>{
// 0x110c9c38 str xzr, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x110c9c3c;continue;
}
0x110c9c3c=>{
// 0x110c9c3c mov x21, x19
cpu.x[21]=(((cpu.x[19]))) as u64;
// 0x110c9c40 ldr x0, [x21, #0xd0]!
{let address=(cpu.x[21]).wrapping_add(0xd0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;cpu.x[21]=(address) as u64;}
// 0x110c9c44 cbz x0, #0x110c9c6c
if (cpu.x[0]) == 0 {pc=0x110c9c6c;continue;}pc=0x110c9c48;continue;
}
0x110c9c48=>{
// 0x110c9c48 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9c4c ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9c50 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110c9c54;super::dispatch(cpu,rt,target)?;}
// 0x110c9c54 ldr x0, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9c58 cbz x0, #0x110c9c68
if (cpu.x[0]) == 0 {pc=0x110c9c68;continue;}pc=0x110c9c5c;continue;
}
0x110c9c5c=>{
// 0x110c9c5c ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9c60 ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9c64 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110c9c68;super::dispatch(cpu,rt,target)?;}
pc=0x110c9c68;continue;
}
0x110c9c68=>{
// 0x110c9c68 str xzr, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x110c9c6c;continue;
}
0x110c9c6c=>{
// 0x110c9c6c ldr w8, [x19, #0x184]
{let address=(cpu.x[19]).wrapping_add(0x184u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x110c9c70 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x110c9c74 b.lt #0x110c9cc0
if cpu.condition(11){pc=0x110c9cc0;continue;}pc=0x110c9c78;continue;
}
0x110c9c78=>{
// 0x110c9c78 sub x22, x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[22]=(result) as u64;}
// 0x110c9c7c b #0x110c9c90
pc=0x110c9c90;continue;
}
0x110c9c80=>{
// 0x110c9c80 add x8, x22, #1
{let a=((cpu.x[22]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x110c9c84 sub x22, x22, #1
{let a=((cpu.x[22]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[22]=(result) as u64;}
// 0x110c9c88 cmp x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x110c9c8c b.ls #0x110c9cc0
if cpu.condition(9){pc=0x110c9cc0;continue;}pc=0x110c9c90;continue;
}
0x110c9c90=>{
// 0x110c9c90 ldr x8, [x19, #0x178]
{let address=(cpu.x[19]).wrapping_add(0x178u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9c94 ldr x20, [x8, x22, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[22])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x110c9c98 cbz x20, #0x110c9c80
if (cpu.x[20]) == 0 {pc=0x110c9c80;continue;}pc=0x110c9c9c;continue;
}
0x110c9c9c=>{
// 0x110c9c9c ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9ca0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x110c9ca4 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9ca8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110c9cac;super::dispatch(cpu,rt,target)?;}
// 0x110c9cac ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9cb0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x110c9cb4 ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9cb8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110c9cbc;super::dispatch(cpu,rt,target)?;}
// 0x110c9cbc b #0x110c9c80
pc=0x110c9c80;continue;
}
0x110c9cc0=>{
// 0x110c9cc0 ldr w8, [x19, #0x194]
{let address=(cpu.x[19]).wrapping_add(0x194u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x110c9cc4 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x110c9cc8 b.lt #0x110c9d14
if cpu.condition(11){pc=0x110c9d14;continue;}pc=0x110c9ccc;continue;
}
0x110c9ccc=>{
// 0x110c9ccc sub x22, x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[22]=(result) as u64;}
// 0x110c9cd0 b #0x110c9ce4
pc=0x110c9ce4;continue;
}
0x110c9cd4=>{
// 0x110c9cd4 add x8, x22, #1
{let a=((cpu.x[22]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x110c9cd8 sub x22, x22, #1
{let a=((cpu.x[22]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[22]=(result) as u64;}
// 0x110c9cdc cmp x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x110c9ce0 b.ls #0x110c9d14
if cpu.condition(9){pc=0x110c9d14;continue;}pc=0x110c9ce4;continue;
}
0x110c9ce4=>{
// 0x110c9ce4 ldr x8, [x19, #0x188]
{let address=(cpu.x[19]).wrapping_add(0x188u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9ce8 ldr x20, [x8, x22, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[22])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x110c9cec cbz x20, #0x110c9cd4
if (cpu.x[20]) == 0 {pc=0x110c9cd4;continue;}pc=0x110c9cf0;continue;
}
0x110c9cf0=>{
// 0x110c9cf0 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9cf4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x110c9cf8 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9cfc blr x8
{let target=cpu.x[8];cpu.x[30]=0x110c9d00;super::dispatch(cpu,rt,target)?;}
// 0x110c9d00 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9d04 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x110c9d08 ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9d0c blr x8
{let target=cpu.x[8];cpu.x[30]=0x110c9d10;super::dispatch(cpu,rt,target)?;}
// 0x110c9d10 b #0x110c9cd4
pc=0x110c9cd4;continue;
}
0x110c9d14=>{
// 0x110c9d14 ldr x0, [x19, #0x40]
{let address=(cpu.x[19]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9d18 cbz x0, #0x110c9d40
if (cpu.x[0]) == 0 {pc=0x110c9d40;continue;}pc=0x110c9d1c;continue;
}
0x110c9d1c=>{
// 0x110c9d1c ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9d20 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9d24 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110c9d28;super::dispatch(cpu,rt,target)?;}
pc=0x110c9d28;continue;
}
0x110c9d28=>{
// 0x110c9d28 ldr x0, [x19, #0x40]
{let address=(cpu.x[19]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9d2c cbz x0, #0x110c9d3c
if (cpu.x[0]) == 0 {pc=0x110c9d3c;continue;}pc=0x110c9d30;continue;
}
0x110c9d30=>{
// 0x110c9d30 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9d34 ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9d38 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110c9d3c;super::dispatch(cpu,rt,target)?;}
pc=0x110c9d3c;continue;
}
0x110c9d3c=>{
// 0x110c9d3c str xzr, [x19, #0x40]
{let address=(cpu.x[19]).wrapping_add(0x40u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x110c9d40;continue;
}
0x110c9d40=>{
// 0x110c9d40 mov x22, x19
cpu.x[22]=(((cpu.x[19]))) as u64;
// 0x110c9d44 ldr x0, [x22, #0xa8]!
{let address=(cpu.x[22]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;cpu.x[22]=(address) as u64;}
// 0x110c9d48 cbz x0, #0x110c9d70
if (cpu.x[0]) == 0 {pc=0x110c9d70;continue;}pc=0x110c9d4c;continue;
}
0x110c9d4c=>{
// 0x110c9d4c bl #0x10d32d10
cpu.x[30]=0x110c9d50;super::chunk3::f_10d32d10(cpu,rt,0x10d32d10)?;
pc=0x110c9d50;continue;
}
0x110c9d50=>{
// 0x110c9d50 ldr x20, [x22]
{let address=(cpu.x[22]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x110c9d54 cbz x20, #0x110c9d6c
if (cpu.x[20]) == 0 {pc=0x110c9d6c;continue;}pc=0x110c9d58;continue;
}
0x110c9d58=>{
// 0x110c9d58 ldr x0, [x20, #0x360]
{let address=(cpu.x[20]).wrapping_add(0x360u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9d5c cbz x0, #0x110c9d64
if (cpu.x[0]) == 0 {pc=0x110c9d64;continue;}pc=0x110c9d60;continue;
}
0x110c9d60=>{
// 0x110c9d60 bl #0x1187d360
cpu.x[30]=0x110c9d64;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x110c9d64;continue;
}
0x110c9d64=>{
// 0x110c9d64 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x110c9d68 bl #0x1187d2c0
cpu.x[30]=0x110c9d6c;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x110c9d6c;continue;
}
0x110c9d6c=>{
// 0x110c9d6c str xzr, [x22]
{let address=(cpu.x[22]).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x110c9d70;continue;
}
0x110c9d70=>{
// 0x110c9d70 add x0, x19, #0x60
{let a=((cpu.x[19]));let b=((0x60u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x110c9d74 bl #0x111d9d44
cpu.x[30]=0x110c9d78;super::chunk5::f_111d9d44(cpu,rt,0x111d9d44)?;
pc=0x110c9d78;continue;
}
0x110c9d78=>{
// 0x110c9d78 ldr x0, [x19, #0x38]
{let address=(cpu.x[19]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9d7c cbz x0, #0x110c9da4
if (cpu.x[0]) == 0 {pc=0x110c9da4;continue;}pc=0x110c9d80;continue;
}
0x110c9d80=>{
// 0x110c9d80 bl #0x10d75994
cpu.x[30]=0x110c9d84;super::chunk4::f_10d75994(cpu,rt,0x10d75994)?;
pc=0x110c9d84;continue;
}
0x110c9d84=>{
// 0x110c9d84 ldr x20, [x19, #0x38]
{let address=(cpu.x[19]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x110c9d88 cbz x20, #0x110c9da0
if (cpu.x[20]) == 0 {pc=0x110c9da0;continue;}pc=0x110c9d8c;continue;
}
0x110c9d8c=>{
// 0x110c9d8c ldr x0, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9d90 cbz x0, #0x110c9d98
if (cpu.x[0]) == 0 {pc=0x110c9d98;continue;}pc=0x110c9d94;continue;
}
0x110c9d94=>{
// 0x110c9d94 bl #0x1187d360
cpu.x[30]=0x110c9d98;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x110c9d98;continue;
}
0x110c9d98=>{
// 0x110c9d98 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x110c9d9c bl #0x1187d2c0
cpu.x[30]=0x110c9da0;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x110c9da0;continue;
}
0x110c9da0=>{
// 0x110c9da0 str xzr, [x19, #0x38]
{let address=(cpu.x[19]).wrapping_add(0x38u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x110c9da4;continue;
}
0x110c9da4=>{
// 0x110c9da4 ldr x0, [x19, #0xc0]
{let address=(cpu.x[19]).wrapping_add(0xc0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9da8 str xzr, [x19, #0xb0]
{let address=(cpu.x[19]).wrapping_add(0xb0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x110c9dac cbz x0, #0x110c9dd4
if (cpu.x[0]) == 0 {pc=0x110c9dd4;continue;}pc=0x110c9db0;continue;
}
0x110c9db0=>{
// 0x110c9db0 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9db4 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9db8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110c9dbc;super::dispatch(cpu,rt,target)?;}
pc=0x110c9dbc;continue;
}
0x110c9dbc=>{
// 0x110c9dbc ldr x0, [x19, #0xc0]
{let address=(cpu.x[19]).wrapping_add(0xc0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9dc0 cbz x0, #0x110c9dd0
if (cpu.x[0]) == 0 {pc=0x110c9dd0;continue;}pc=0x110c9dc4;continue;
}
0x110c9dc4=>{
// 0x110c9dc4 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9dc8 ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9dcc blr x8
{let target=cpu.x[8];cpu.x[30]=0x110c9dd0;super::dispatch(cpu,rt,target)?;}
pc=0x110c9dd0;continue;
}
0x110c9dd0=>{
// 0x110c9dd0 str xzr, [x19, #0xc0]
{let address=(cpu.x[19]).wrapping_add(0xc0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x110c9dd4;continue;
}
0x110c9dd4=>{
// 0x110c9dd4 add x0, x19, #0xa0
{let a=((cpu.x[19]));let b=((0xa0u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x110c9dd8 bl #0x11294ba4
cpu.x[30]=0x110c9ddc;super::chunk6::f_11294ba4(cpu,rt,0x11294ba4)?;
pc=0x110c9ddc;continue;
}
0x110c9ddc=>{
// 0x110c9ddc ldr x0, [x19, #0x50]
{let address=(cpu.x[19]).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9de0 cbz x0, #0x110c9e04
if (cpu.x[0]) == 0 {pc=0x110c9e04;continue;}pc=0x110c9de4;continue;
}
0x110c9de4=>{
// 0x110c9de4 bl #0x10ffb834
cpu.x[30]=0x110c9de8;super::dispatch(cpu,rt,0x10ffb834)?;
// 0x110c9de8 ldr x20, [x19, #0x50]
{let address=(cpu.x[19]).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x110c9dec cbz x20, #0x110c9e00
if (cpu.x[20]) == 0 {pc=0x110c9e00;continue;}pc=0x110c9df0;continue;
}
0x110c9df0=>{
// 0x110c9df0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x110c9df4 bl #0x10cc9660
cpu.x[30]=0x110c9df8;super::dispatch(cpu,rt,0x10cc9660)?;
// 0x110c9df8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x110c9dfc bl #0x1187d2c0
cpu.x[30]=0x110c9e00;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x110c9e00;continue;
}
0x110c9e00=>{
// 0x110c9e00 str xzr, [x19, #0x50]
{let address=(cpu.x[19]).wrapping_add(0x50u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x110c9e04;continue;
}
0x110c9e04=>{
// 0x110c9e04 ldr x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9e08 cbz x0, #0x110c9e20
if (cpu.x[0]) == 0 {pc=0x110c9e20;continue;}pc=0x110c9e0c;continue;
}
0x110c9e0c=>{
// 0x110c9e0c bl #0x10d19b80
cpu.x[30]=0x110c9e10;super::chunk12::f_10d19b80(cpu,rt,0x10d19b80)?;
pc=0x110c9e10;continue;
}
0x110c9e10=>{
// 0x110c9e10 ldr x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9e14 cbz x0, #0x110c9e1c
if (cpu.x[0]) == 0 {pc=0x110c9e1c;continue;}pc=0x110c9e18;continue;
}
0x110c9e18=>{
// 0x110c9e18 bl #0x1187d2c0
cpu.x[30]=0x110c9e1c;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x110c9e1c;continue;
}
0x110c9e1c=>{
// 0x110c9e1c str xzr, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x110c9e20;continue;
}
0x110c9e20=>{
// 0x110c9e20 ldr x0, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9e24 cbz x0, #0x110c9e3c
if (cpu.x[0]) == 0 {pc=0x110c9e3c;continue;}pc=0x110c9e28;continue;
}
0x110c9e28=>{
// 0x110c9e28 bl #0x111cfa08
cpu.x[30]=0x110c9e2c;super::dispatch(cpu,rt,0x111cfa08)?;
// 0x110c9e2c ldr x0, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9e30 cbz x0, #0x110c9e38
if (cpu.x[0]) == 0 {pc=0x110c9e38;continue;}pc=0x110c9e34;continue;
}
0x110c9e34=>{
// 0x110c9e34 bl #0x1187d2c0
cpu.x[30]=0x110c9e38;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x110c9e38;continue;
}
0x110c9e38=>{
// 0x110c9e38 str xzr, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x110c9e3c;continue;
}
0x110c9e3c=>{
// 0x110c9e3c mov x23, x19
cpu.x[23]=(((cpu.x[19]))) as u64;
// 0x110c9e40 ldr x0, [x23, #0x10]!
{let address=(cpu.x[23]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;cpu.x[23]=(address) as u64;}
// 0x110c9e44 cbz x0, #0x110c9e6c
if (cpu.x[0]) == 0 {pc=0x110c9e6c;continue;}pc=0x110c9e48;continue;
}
0x110c9e48=>{
// 0x110c9e48 bl #0x111cdc6c
cpu.x[30]=0x110c9e4c;super::dispatch(cpu,rt,0x111cdc6c)?;
// 0x110c9e4c ldr x20, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x110c9e50 cbz x20, #0x110c9e68
if (cpu.x[20]) == 0 {pc=0x110c9e68;continue;}pc=0x110c9e54;continue;
}
0x110c9e54=>{
// 0x110c9e54 ldr x0, [x20, #0x48]
{let address=(cpu.x[20]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9e58 cbz x0, #0x110c9e60
if (cpu.x[0]) == 0 {pc=0x110c9e60;continue;}pc=0x110c9e5c;continue;
}
0x110c9e5c=>{
// 0x110c9e5c bl #0x1187d360
cpu.x[30]=0x110c9e60;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x110c9e60;continue;
}
0x110c9e60=>{
// 0x110c9e60 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x110c9e64 bl #0x1187d2c0
cpu.x[30]=0x110c9e68;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x110c9e68;continue;
}
0x110c9e68=>{
// 0x110c9e68 str xzr, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x110c9e6c;continue;
}
0x110c9e6c=>{
// 0x110c9e6c ldr x0, [x19, #0x1d8]
{let address=(cpu.x[19]).wrapping_add(0x1d8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9e70 add x24, x19, #0x1d8
{let a=((cpu.x[19]));let b=((0x1d8u64));let result=a.wrapping_add(b);cpu.x[24]=(result) as u64;}
// 0x110c9e74 cbz x0, #0x110c9e8c
if (cpu.x[0]) == 0 {pc=0x110c9e8c;continue;}pc=0x110c9e78;continue;
}
0x110c9e78=>{
// 0x110c9e78 bl #0x111db894
cpu.x[30]=0x110c9e7c;super::chunk8::f_111db894(cpu,rt,0x111db894)?;
pc=0x110c9e7c;continue;
}
0x110c9e7c=>{
// 0x110c9e7c ldr x0, [x24]
{let address=(cpu.x[24]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9e80 cbz x0, #0x110c9e88
if (cpu.x[0]) == 0 {pc=0x110c9e88;continue;}pc=0x110c9e84;continue;
}
0x110c9e84=>{
// 0x110c9e84 bl #0x1187d2c0
cpu.x[30]=0x110c9e88;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x110c9e88;continue;
}
0x110c9e88=>{
// 0x110c9e88 str xzr, [x24]
{let address=(cpu.x[24]).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x110c9e8c;continue;
}
0x110c9e8c=>{
// 0x110c9e8c ldr x20, [x19, #0x1e0]
{let address=(cpu.x[19]).wrapping_add(0x1e0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x110c9e90 cbz x20, #0x110c9ef0
if (cpu.x[20]) == 0 {pc=0x110c9ef0;continue;}pc=0x110c9e94;continue;
}
0x110c9e94=>{
// 0x110c9e94 ldr w8, [x20, #0xc]
{let address=(cpu.x[20]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x110c9e98 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x110c9e9c b.lt #0x110c9ed8
if cpu.condition(11){pc=0x110c9ed8;continue;}pc=0x110c9ea0;continue;
}
0x110c9ea0=>{
// 0x110c9ea0 mov x25, xzr
cpu.x[25]=(((0u64))) as u64;
// 0x110c9ea4 b #0x110c9eb8
pc=0x110c9eb8;continue;
}
0x110c9ea8=>{
// 0x110c9ea8 ldrsw x8, [x20, #0xc]
{let address=(cpu.x[20]).wrapping_add(0xcu64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x110c9eac add x25, x25, #1
{let a=((cpu.x[25]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[25]=(result) as u64;}
// 0x110c9eb0 cmp x25, x8
{let a=((cpu.x[25]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x110c9eb4 b.ge #0x110c9ed8
if cpu.condition(10){pc=0x110c9ed8;continue;}pc=0x110c9eb8;continue;
}
0x110c9eb8=>{
// 0x110c9eb8 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9ebc ldr x0, [x8, x25, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[25])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9ec0 cbz x0, #0x110c9ea8
if (cpu.x[0]) == 0 {pc=0x110c9ea8;continue;}pc=0x110c9ec4;continue;
}
0x110c9ec4=>{
// 0x110c9ec4 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9ec8 ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9ecc blr x8
{let target=cpu.x[8];cpu.x[30]=0x110c9ed0;super::dispatch(cpu,rt,target)?;}
// 0x110c9ed0 ldr x20, [x19, #0x1e0]
{let address=(cpu.x[19]).wrapping_add(0x1e0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x110c9ed4 b #0x110c9ea8
pc=0x110c9ea8;continue;
}
0x110c9ed8=>{
// 0x110c9ed8 ldr x0, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9edc cbz x0, #0x110c9ee4
if (cpu.x[0]) == 0 {pc=0x110c9ee4;continue;}pc=0x110c9ee0;continue;
}
0x110c9ee0=>{
// 0x110c9ee0 bl #0x1187d360
cpu.x[30]=0x110c9ee4;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x110c9ee4;continue;
}
0x110c9ee4=>{
// 0x110c9ee4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x110c9ee8 bl #0x1187d2c0
cpu.x[30]=0x110c9eec;super::dispatch(cpu,rt,0x1187d2c0)?;
// 0x110c9eec str xzr, [x19, #0x1e0]
{let address=(cpu.x[19]).wrapping_add(0x1e0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x110c9ef0;continue;
}
0x110c9ef0=>{
// 0x110c9ef0 ldr x0, [x19, #0x218]
{let address=(cpu.x[19]).wrapping_add(0x218u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9ef4 cbz x0, #0x110c9f1c
if (cpu.x[0]) == 0 {pc=0x110c9f1c;continue;}pc=0x110c9ef8;continue;
}
0x110c9ef8=>{
// 0x110c9ef8 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9efc ldr x8, [x8, #0xb8]
{let address=(cpu.x[8]).wrapping_add(0xb8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9f00 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110c9f04;super::dispatch(cpu,rt,target)?;}
// 0x110c9f04 ldr x0, [x19, #0x218]
{let address=(cpu.x[19]).wrapping_add(0x218u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9f08 cbz x0, #0x110c9f18
if (cpu.x[0]) == 0 {pc=0x110c9f18;continue;}pc=0x110c9f0c;continue;
}
0x110c9f0c=>{
// 0x110c9f0c ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9f10 ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9f14 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110c9f18;super::dispatch(cpu,rt,target)?;}
pc=0x110c9f18;continue;
}
0x110c9f18=>{
// 0x110c9f18 str xzr, [x19, #0x218]
{let address=(cpu.x[19]).wrapping_add(0x218u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x110c9f1c;continue;
}
0x110c9f1c=>{
// 0x110c9f1c ldr x0, [x19, #0x220]
{let address=(cpu.x[19]).wrapping_add(0x220u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9f20 cbz x0, #0x110c9f30
if (cpu.x[0]) == 0 {pc=0x110c9f30;continue;}pc=0x110c9f24;continue;
}
0x110c9f24=>{
// 0x110c9f24 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9f28 ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9f2c blr x8
{let target=cpu.x[8];cpu.x[30]=0x110c9f30;super::dispatch(cpu,rt,target)?;}
pc=0x110c9f30;continue;
}
0x110c9f30=>{
// 0x110c9f30 ldr x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9f34 str xzr, [x19, #0x220]
{let address=(cpu.x[19]).wrapping_add(0x220u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x110c9f38 cbz x0, #0x110c9f60
if (cpu.x[0]) == 0 {pc=0x110c9f60;continue;}pc=0x110c9f3c;continue;
}
0x110c9f3c=>{
// 0x110c9f3c ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9f40 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9f44 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110c9f48;super::dispatch(cpu,rt,target)?;}
pc=0x110c9f48;continue;
}
0x110c9f48=>{
// 0x110c9f48 ldr x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9f4c cbz x0, #0x110c9f5c
if (cpu.x[0]) == 0 {pc=0x110c9f5c;continue;}pc=0x110c9f50;continue;
}
0x110c9f50=>{
// 0x110c9f50 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9f54 ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110c9f58 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110c9f5c;super::dispatch(cpu,rt,target)?;}
pc=0x110c9f5c;continue;
}
0x110c9f5c=>{
// 0x110c9f5c str xzr, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x110c9f60;continue;
}
0x110c9f60=>{
// 0x110c9f60 ldr x0, [x19, #0x228]
{let address=(cpu.x[19]).wrapping_add(0x228u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9f64 cbz x0, #0x110c9f94
if (cpu.x[0]) == 0 {pc=0x110c9f94;continue;}pc=0x110c9f68;continue;
}
0x110c9f68=>{
// 0x110c9f68 bl #0x111ebb3c
cpu.x[30]=0x110c9f6c;super::dispatch(cpu,rt,0x111ebb3c)?;
// 0x110c9f6c ldr x20, [x19, #0x228]
{let address=(cpu.x[19]).wrapping_add(0x228u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x110c9f70 cbz x20, #0x110c9f90
if (cpu.x[20]) == 0 {pc=0x110c9f90;continue;}pc=0x110c9f74;continue;
}
0x110c9f74=>{
// 0x110c9f74 add x0, x20, #0x20
{let a=((cpu.x[20]));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x110c9f78 bl #0x1142fcc4
cpu.x[30]=0x110c9f7c;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
// 0x110c9f7c ldr x0, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9f80 cbz x0, #0x110c9f88
if (cpu.x[0]) == 0 {pc=0x110c9f88;continue;}pc=0x110c9f84;continue;
}
0x110c9f84=>{
// 0x110c9f84 bl #0x1187d360
cpu.x[30]=0x110c9f88;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x110c9f88;continue;
}
0x110c9f88=>{
// 0x110c9f88 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x110c9f8c bl #0x1187d2c0
cpu.x[30]=0x110c9f90;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x110c9f90;continue;
}
0x110c9f90=>{
// 0x110c9f90 str xzr, [x19, #0x228]
{let address=(cpu.x[19]).wrapping_add(0x228u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x110c9f94;continue;
}
0x110c9f94=>{
// 0x110c9f94 ldr x0, [x19, #0x230]
{let address=(cpu.x[19]).wrapping_add(0x230u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9f98 add x25, x19, #0x118
{let a=((cpu.x[19]));let b=((0x118u64));let result=a.wrapping_add(b);cpu.x[25]=(result) as u64;}
// 0x110c9f9c cbz x0, #0x110c9fcc
if (cpu.x[0]) == 0 {pc=0x110c9fcc;continue;}pc=0x110c9fa0;continue;
}
0x110c9fa0=>{
// 0x110c9fa0 bl #0x111ebb3c
cpu.x[30]=0x110c9fa4;super::dispatch(cpu,rt,0x111ebb3c)?;
// 0x110c9fa4 ldr x20, [x19, #0x230]
{let address=(cpu.x[19]).wrapping_add(0x230u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x110c9fa8 cbz x20, #0x110c9fc8
if (cpu.x[20]) == 0 {pc=0x110c9fc8;continue;}pc=0x110c9fac;continue;
}
0x110c9fac=>{
// 0x110c9fac add x0, x20, #0x20
{let a=((cpu.x[20]));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x110c9fb0 bl #0x1142fcc4
cpu.x[30]=0x110c9fb4;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
// 0x110c9fb4 ldr x0, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110c9fb8 cbz x0, #0x110c9fc0
if (cpu.x[0]) == 0 {pc=0x110c9fc0;continue;}pc=0x110c9fbc;continue;
}
0x110c9fbc=>{
// 0x110c9fbc bl #0x1187d360
cpu.x[30]=0x110c9fc0;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x110c9fc0;continue;
}
0x110c9fc0=>{
// 0x110c9fc0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x110c9fc4 bl #0x1187d2c0
cpu.x[30]=0x110c9fc8;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x110c9fc8;continue;
}
0x110c9fc8=>{
// 0x110c9fc8 str xzr, [x19, #0x230]
{let address=(cpu.x[19]).wrapping_add(0x230u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x110c9fcc;continue;
}
0x110c9fcc=>{
// 0x110c9fcc adrp x8, #0x102b1000
cpu.x[8]=(0x102b1000u64) as u64;
// 0x110c9fd0 movi v0.2d, #0000000000000000
cpu.q[0]=(lane_dup(0u128,64,2)) & 0xffffffffffffffffffffffffffffffffu128;
// 0x110c9fd4 str wzr, [x19, #0x9c]
{let address=(cpu.x[19]).wrapping_add(0x9cu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x110c9fd8 ldr d1, [x8, #0x850]
{let address=(cpu.x[8]).wrapping_add(0x850u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.q[1]=(v0) as u128 & 0xffffffffffffffffu128;}
// 0x110c9fdc mov x8, #-1
cpu.x[8]=(((0xffffffffffffffffu64))) as u64;
// 0x110c9fe0 str wzr, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x110c9fe4 str x8, [x19, #0x1c0]
{let address=(cpu.x[19]).wrapping_add(0x1c0u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x110c9fe8 add x8, x19, #0x1ed
{let a=((cpu.x[19]));let b=((0x1edu64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x110c9fec str d1, [x19, #0x1b8]
{let address=(cpu.x[19]).wrapping_add(0x1b8u64);let v0=(cpu.q[1] & 0xffffffffffffffffu128) as u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x110c9ff0 movi v1.2d, #0xffffffffffffffff
cpu.q[1]=(lane_dup(18446744073709551615u128,64,2)) & 0xffffffffffffffffffffffffffffffffu128;
// 0x110c9ff4 str wzr, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x110c9ff8 mov w8, #-1
cpu.x[8]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x110c9ffc str w8, [x19, #0x200]
{let address=(cpu.x[19]).wrapping_add(0x200u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x110ca000;continue;
}
0x110ca000=>{
// 0x110ca000 add x8, x19, #0x208
{let a=((cpu.x[19]));let b=((0x208u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x110ca004 stp q0, q0, [x25]
{let address=(cpu.x[25]).wrapping_add(0x0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x110ca008 str q0, [x25, #0x20]
{let address=(cpu.x[25]).wrapping_add(0x20u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x110ca00c str q0, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x110ca010 stur q0, [x19, #0x34]
{let address=(cpu.x[19]).wrapping_add(0x34u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x110ca014 stur q0, [x19, #0x44]
{let address=(cpu.x[19]).wrapping_add(0x44u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x110ca018 str q0, [x19, #0x50]
{let address=(cpu.x[19]).wrapping_add(0x50u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x110ca01c stp q0, q0, [x22]
{let address=(cpu.x[22]).wrapping_add(0x0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x110ca020 str wzr, [x22, #0x20]
{let address=(cpu.x[22]).wrapping_add(0x20u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x110ca024 stp q0, q0, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x110ca028 str q0, [x21, #0x20]
{let address=(cpu.x[21]).wrapping_add(0x20u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x110ca02c str xzr, [x21, #0x30]
{let address=(cpu.x[21]).wrapping_add(0x30u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x110ca030 strb wzr, [x19, #0x1ec]
{let address=(cpu.x[19]).wrapping_add(0x1ecu64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x110ca034 str wzr, [x19, #0x1e8]
{let address=(cpu.x[19]).wrapping_add(0x1e8u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x110ca038 str q0, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x110ca03c str q0, [x24]
{let address=(cpu.x[24]).wrapping_add(0x0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x110ca040 str d1, [x19, #0x1c8]
{let address=(cpu.x[19]).wrapping_add(0x1c8u64);let v0=(cpu.q[1] & 0xffffffffffffffffu128) as u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x110ca044 str wzr, [x19, #0x1f4]
{let address=(cpu.x[19]).wrapping_add(0x1f4u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x110ca048 str xzr, [x19, #0x1f8]
{let address=(cpu.x[19]).wrapping_add(0x1f8u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x110ca04c strb wzr, [x19, #0x204]
{let address=(cpu.x[19]).wrapping_add(0x204u64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x110ca050 strb wzr, [x19, #0x1d0]
{let address=(cpu.x[19]).wrapping_add(0x1d0u64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x110ca054 str xzr, [x19, #0x218]
{let address=(cpu.x[19]).wrapping_add(0x218u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x110ca058 str q0, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x110ca05c ldp x20, x19, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x110ca060 ldr x25, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[25]=(v0) as u64;}
// 0x110ca064 ldp x22, x21, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x110ca068 ldp x24, x23, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x110ca06c ldp x29, x30, [sp], #0x50
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x50u64)))) as u64;}
// 0x110ca070 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x110c9a30")),}}}
#[inline]
pub(super) unsafe fn f_110ca9a0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x110ca9a0=>{
// 0x110ca9a0 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x110ca9a4 stp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x110ca9a8 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x110ca9ac mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x110ca9b0 ldr w8, [x0, #0x20]
{let address=(cpu.x[0]).wrapping_add(0x20u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x110ca9b4 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x110ca9b8 cmp w8, #4
{let a=((cpu.x[8] as u32 as u64));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x110ca9bc b.ne #0x110ca9c8
if cpu.condition(1){pc=0x110ca9c8;continue;}pc=0x110ca9c0;continue;
}
0x110ca9c0=>{
// 0x110ca9c0 ldr x0, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110ca9c4 bl #0x111d03fc
cpu.x[30]=0x110ca9c8;super::dispatch(cpu,rt,0x111d03fc)?;
pc=0x110ca9c8;continue;
}
0x110ca9c8=>{
// 0x110ca9c8 ldr x0, [x19, #0xa8]
{let address=(cpu.x[19]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110ca9cc cbz x0, #0x110caa88
if (cpu.x[0]) == 0 {pc=0x110caa88;continue;}pc=0x110ca9d0;continue;
}
0x110ca9d0=>{
// 0x110ca9d0 ldr x8, [x0, #0x1b0]
{let address=(cpu.x[0]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110ca9d4 ldr x8, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110ca9d8 ldrb w21, [x8, #0x181]
{let address=(cpu.x[8]).wrapping_add(0x181u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[21]=(v0) as u32 as u64;}
// 0x110ca9dc ldrb w22, [x8, #0x180]
{let address=(cpu.x[8]).wrapping_add(0x180u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[22]=(v0) as u32 as u64;}
// 0x110ca9e0 bl #0x10d3d63c
cpu.x[30]=0x110ca9e4;super::chunk8::f_10d3d63c(cpu,rt,0x10d3d63c)?;
pc=0x110ca9e4;continue;
}
0x110ca9e4=>{
// 0x110ca9e4 tbz w0, #0, #0x110caa30
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x110caa30;continue;}pc=0x110ca9e8;continue;
}
0x110ca9e8=>{
// 0x110ca9e8 ldr x0, [x19, #0xa8]
{let address=(cpu.x[19]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110ca9ec bl #0x10d36084
cpu.x[30]=0x110ca9f0;super::chunk6::f_10d36084(cpu,rt,0x10d36084)?;
pc=0x110ca9f0;continue;
}
0x110ca9f0=>{
// 0x110ca9f0 mov w20, wzr
cpu.x[20]=(((0u64))) as u32 as u64;
// 0x110ca9f4 cbz w22, #0x110caa14
if (cpu.x[22] as u32 as u64) == 0 {pc=0x110caa14;continue;}pc=0x110ca9f8;continue;
}
0x110ca9f8=>{
// 0x110ca9f8 tbnz w0, #0, #0x110caa14
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x110caa14;continue;}pc=0x110ca9fc;continue;
}
0x110ca9fc=>{
// 0x110ca9fc ldrb w8, [x19, #0x1ef]
{let address=(cpu.x[19]).wrapping_add(0x1efu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x110caa00 cbnz w8, #0x110caa14
if (cpu.x[8] as u32 as u64) != 0 {pc=0x110caa14;continue;}pc=0x110caa04;continue;
}
0x110caa04=>{
// 0x110caa04 ldr x0, [x19, #0xa8]
{let address=(cpu.x[19]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110caa08 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x110caa0c bl #0x10d4220c
cpu.x[30]=0x110caa10;super::chunk12::f_10d4220c(cpu,rt,0x10d4220c)?;
pc=0x110caa10;continue;
}
0x110caa10=>{
// 0x110caa10 eor w20, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a ^ b;cpu.x[20]=(result) as u32 as u64;}
pc=0x110caa14;continue;
}
0x110caa14=>{
// 0x110caa14 ldr x0, [x19, #0xa8]
{let address=(cpu.x[19]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110caa18 bl #0x10d360a8
cpu.x[30]=0x110caa1c;super::chunk4::f_10d360a8(cpu,rt,0x10d360a8)?;
pc=0x110caa1c;continue;
}
0x110caa1c=>{
// 0x110caa1c tbnz w0, #0, #0x110caa40
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x110caa40;continue;}pc=0x110caa20;continue;
}
0x110caa20=>{
// 0x110caa20 ldr x0, [x19, #0xa8]
{let address=(cpu.x[19]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110caa24 bl #0x10d3581c
cpu.x[30]=0x110caa28;super::chunk11::f_10d3581c(cpu,rt,0x10d3581c)?;
pc=0x110caa28;continue;
}
0x110caa28=>{
// 0x110caa28 tbz w0, #0, #0x110caa88
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x110caa88;continue;}pc=0x110caa2c;continue;
}
0x110caa2c=>{
// 0x110caa2c b #0x110caafc
pc=0x110caafc;continue;
}
0x110caa30=>{
// 0x110caa30 mov w20, wzr
cpu.x[20]=(((0u64))) as u32 as u64;
// 0x110caa34 ldr x0, [x19, #0xa8]
{let address=(cpu.x[19]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110caa38 bl #0x10d360a8
cpu.x[30]=0x110caa3c;super::chunk4::f_10d360a8(cpu,rt,0x10d360a8)?;
pc=0x110caa3c;continue;
}
0x110caa3c=>{
// 0x110caa3c tbz w0, #0, #0x110caa20
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x110caa20;continue;}pc=0x110caa40;continue;
}
0x110caa40=>{
// 0x110caa40 tbz w20, #0, #0x110caa74
if ((cpu.x[20] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x110caa74;continue;}pc=0x110caa44;continue;
}
0x110caa44=>{
// 0x110caa44 ldr w8, [x19, #0x1f4]
{let address=(cpu.x[19]).wrapping_add(0x1f4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x110caa48 add w9, w8, #0x32
{let a=((cpu.x[8] as u32 as u64));let b=((0x32u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x110caa4c cmp w8, #0x5aa
{let a=((cpu.x[8] as u32 as u64));let b=((0x5aau64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x110caa50 str w9, [x19, #0x1f4]
{let address=(cpu.x[19]).wrapping_add(0x1f4u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x110caa54 b.gt #0x110cab8c
if cpu.condition(12){pc=0x110cab8c;continue;}pc=0x110caa58;continue;
}
0x110caa58=>{
// 0x110caa58 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x110caa5c bl #0x110cad30
cpu.x[30]=0x110caa60;super::chunk13::f_110cad30(cpu,rt,0x110cad30)?;
pc=0x110caa60;continue;
}
0x110caa60=>{
// 0x110caa60 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x110caa64 b.lt #0x110cacf0
if cpu.condition(11){pc=0x110cacf0;continue;}pc=0x110caa68;continue;
}
0x110caa68=>{
// 0x110caa68 ldr x0, [x19, #0xa8]
{let address=(cpu.x[19]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110caa6c bl #0x10d33f74
cpu.x[30]=0x110caa70;super::chunk0::f_10d33f74(cpu,rt,0x10d33f74)?;
pc=0x110caa70;continue;
}
0x110caa70=>{
// 0x110caa70 b #0x110cacf0
pc=0x110cacf0;continue;
}
0x110caa74=>{
// 0x110caa74 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x110caa78 bl #0x110cb17c
cpu.x[30]=0x110caa7c;super::chunk14::f_110cb17c(cpu,rt,0x110cb17c)?;
pc=0x110caa7c;continue;
}
0x110caa7c=>{
// 0x110caa7c ldr x0, [x19, #0xa8]
{let address=(cpu.x[19]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110caa80 bl #0x10d3581c
cpu.x[30]=0x110caa84;super::chunk11::f_10d3581c(cpu,rt,0x10d3581c)?;
pc=0x110caa84;continue;
}
0x110caa84=>{
// 0x110caa84 tbnz w0, #0, #0x110caafc
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x110caafc;continue;}pc=0x110caa88;continue;
}
0x110caa88=>{
// 0x110caa88 ldr x0, [x19, #0x208]
{let address=(cpu.x[19]).wrapping_add(0x208u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110caa8c bl #0x10e16208
cpu.x[30]=0x110caa90;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
pc=0x110caa90;continue;
}
0x110caa90=>{
// 0x110caa90 ldrb w8, [x0, #0x7a6]
{let address=(cpu.x[0]).wrapping_add(0x7a6u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x110caa94 cbz w8, #0x110caaa0
if (cpu.x[8] as u32 as u64) == 0 {pc=0x110caaa0;continue;}pc=0x110caa98;continue;
}
0x110caa98=>{
// 0x110caa98 ldr x0, [x19, #0x1d8]
{let address=(cpu.x[19]).wrapping_add(0x1d8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110caa9c bl #0x111db8a4
cpu.x[30]=0x110caaa0;super::chunk9::f_111db8a4(cpu,rt,0x111db8a4)?;
pc=0x110caaa0;continue;
}
0x110caaa0=>{
// 0x110caaa0 ldr x0, [x19, #0x218]
{let address=(cpu.x[19]).wrapping_add(0x218u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110caaa4 cbz x0, #0x110caab4
if (cpu.x[0]) == 0 {pc=0x110caab4;continue;}pc=0x110caaa8;continue;
}
0x110caaa8=>{
// 0x110caaa8 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110caaac ldr x8, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110caab0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110caab4;super::dispatch(cpu,rt,target)?;}
pc=0x110caab4;continue;
}
0x110caab4=>{
// 0x110caab4 ldr x0, [x19, #0x38]
{let address=(cpu.x[19]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110caab8 bl #0x10d75ed0
cpu.x[30]=0x110caabc;super::chunk6::f_10d75ed0(cpu,rt,0x10d75ed0)?;
pc=0x110caabc;continue;
}
0x110caabc=>{
// 0x110caabc ldr w8, [x19, #0x200]
{let address=(cpu.x[19]).wrapping_add(0x200u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x110caac0 subs w8, w8, #0x32
{let a=((cpu.x[8] as u32 as u64));let b=((0x32u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x110caac4 str w8, [x19, #0x200]
{let address=(cpu.x[19]).wrapping_add(0x200u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x110caac8 b.eq #0x110cab0c
if cpu.condition(0){pc=0x110cab0c;continue;}pc=0x110caacc;continue;
}
0x110caacc=>{
// 0x110caacc ldr x0, [x19, #0x50]
{let address=(cpu.x[19]).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110caad0 cbz x0, #0x110cab20
if (cpu.x[0]) == 0 {pc=0x110cab20;continue;}pc=0x110caad4;continue;
}
0x110caad4=>{
// 0x110caad4 bl #0x11003590
cpu.x[30]=0x110caad8;super::dispatch(cpu,rt,0x11003590)?;
pc=0x110caad8;continue;
}
0x110caad8=>{
// 0x110caad8 ldr x0, [x19, #0xb0]
{let address=(cpu.x[19]).wrapping_add(0xb0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110caadc cbz x0, #0x110caafc
if (cpu.x[0]) == 0 {pc=0x110caafc;continue;}pc=0x110caae0;continue;
}
0x110caae0=>{
// 0x110caae0 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110caae4 ldr w1, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x110caae8 ldr x2, [x8, #0x4b0]
{let address=(cpu.x[8]).wrapping_add(0x4b0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x110caaec ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x110caaf0 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x110caaf4 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x110caaf8 br x2
{let target=cpu.x[2];if target>=0x110ca9a0 && target<0x110cad30{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
0x110caafc=>{
// 0x110caafc ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x110cab00 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x110cab04 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x110cab08 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x110cab0c=>{
// 0x110cab0c ldr x0, [x19, #0xa8]
{let address=(cpu.x[19]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110cab10 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x110cab14 bl #0x10d41ae4
cpu.x[30]=0x110cab18;super::dispatch(cpu,rt,0x10d41ae4)?;
// 0x110cab18 ldr x0, [x19, #0x50]
{let address=(cpu.x[19]).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110cab1c cbnz x0, #0x110caad4
if (cpu.x[0]) != 0 {pc=0x110caad4;continue;}pc=0x110cab20;continue;
}
0x110cab20=>{
// 0x110cab20 ldr x0, [x19, #0xa8]
{let address=(cpu.x[19]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110cab24 cbz x0, #0x110cab74
if (cpu.x[0]) == 0 {pc=0x110cab74;continue;}pc=0x110cab28;continue;
}
0x110cab28=>{
// 0x110cab28 bl #0x10d33f74
cpu.x[30]=0x110cab2c;super::chunk0::f_10d33f74(cpu,rt,0x10d33f74)?;
pc=0x110cab2c;continue;
}
0x110cab2c=>{
// 0x110cab2c ldr x0, [x19, #0xa8]
{let address=(cpu.x[19]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110cab30 ldr x8, [x0, #0x1b0]
{let address=(cpu.x[0]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110cab34 ldr x8, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110cab38 ldrb w21, [x8, #0x180]
{let address=(cpu.x[8]).wrapping_add(0x180u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[21]=(v0) as u32 as u64;}
// 0x110cab3c bl #0x10d3d63c
cpu.x[30]=0x110cab40;super::chunk8::f_10d3d63c(cpu,rt,0x10d3d63c)?;
pc=0x110cab40;continue;
}
0x110cab40=>{
// 0x110cab40 tbz w0, #0, #0x110cabe4
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x110cabe4;continue;}pc=0x110cab44;continue;
}
0x110cab44=>{
// 0x110cab44 ldr x0, [x19, #0xa8]
{let address=(cpu.x[19]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110cab48 bl #0x10d36084
cpu.x[30]=0x110cab4c;super::chunk6::f_10d36084(cpu,rt,0x10d36084)?;
pc=0x110cab4c;continue;
}
0x110cab4c=>{
// 0x110cab4c mov w20, wzr
cpu.x[20]=(((0u64))) as u32 as u64;
// 0x110cab50 cbz w21, #0x110cabe8
if (cpu.x[21] as u32 as u64) == 0 {pc=0x110cabe8;continue;}pc=0x110cab54;continue;
}
0x110cab54=>{
// 0x110cab54 tbnz w0, #0, #0x110cabe8
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x110cabe8;continue;}pc=0x110cab58;continue;
}
0x110cab58=>{
// 0x110cab58 ldrb w8, [x19, #0x1ef]
{let address=(cpu.x[19]).wrapping_add(0x1efu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x110cab5c cbnz w8, #0x110cabe8
if (cpu.x[8] as u32 as u64) != 0 {pc=0x110cabe8;continue;}pc=0x110cab60;continue;
}
0x110cab60=>{
// 0x110cab60 ldr x0, [x19, #0xa8]
{let address=(cpu.x[19]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110cab64 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x110cab68 bl #0x10d4220c
cpu.x[30]=0x110cab6c;super::chunk12::f_10d4220c(cpu,rt,0x10d4220c)?;
pc=0x110cab6c;continue;
}
0x110cab6c=>{
// 0x110cab6c eor w20, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a ^ b;cpu.x[20]=(result) as u32 as u64;}
// 0x110cab70 b #0x110cabe8
pc=0x110cabe8;continue;
}
0x110cab74=>{
// 0x110cab74 ldr x0, [x19, #0x218]
{let address=(cpu.x[19]).wrapping_add(0x218u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110cab78 cbz x0, #0x110caad8
if (cpu.x[0]) == 0 {pc=0x110caad8;continue;}pc=0x110cab7c;continue;
}
0x110cab7c=>{
// 0x110cab7c ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110cab80 ldr x8, [x8, #0x20]
{let address=(cpu.x[8]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110cab84 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110cab88;super::dispatch(cpu,rt,target)?;}
// 0x110cab88 b #0x110caad8
pc=0x110caad8;continue;
}
0x110cab8c=>{
// 0x110cab8c cmp w8, #0xcb2
{let a=((cpu.x[8] as u32 as u64));let b=((0xcb2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x110cab90 b.lo #0x110cacf0
if cpu.condition(3){pc=0x110cacf0;continue;}pc=0x110cab94;continue;
}
0x110cab94=>{
// 0x110cab94 ldr x0, [x19, #0xa8]
{let address=(cpu.x[19]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110cab98 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x110cab9c bl #0x10d3c0ec
cpu.x[30]=0x110caba0;super::chunk2::f_10d3c0ec(cpu,rt,0x10d3c0ec)?;
pc=0x110caba0;continue;
}
0x110caba0=>{
// 0x110caba0 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x110caba4 b.lt #0x110cac04
if cpu.condition(11){pc=0x110cac04;continue;}pc=0x110caba8;continue;
}
0x110caba8=>{
// 0x110caba8 ldr x0, [x19, #0xa8]
{let address=(cpu.x[19]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110cabac mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x110cabb0 bl #0x10d3c0ec
cpu.x[30]=0x110cabb4;super::chunk2::f_10d3c0ec(cpu,rt,0x10d3c0ec)?;
pc=0x110cabb4;continue;
}
0x110cabb4=>{
// 0x110cabb4 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x110cabb8 cset w20, lt
cpu.x[20]=((cpu.condition(11)) as u64) as u32 as u64;
// 0x110cabbc mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x110cabc0 cbz w21, #0x110cac10
if (cpu.x[21] as u32 as u64) == 0 {pc=0x110cac10;continue;}pc=0x110cabc4;continue;
}
0x110cabc4=>{
// 0x110cabc4 bl #0x110cb060
cpu.x[30]=0x110cabc8;super::chunk11::f_110cb060(cpu,rt,0x110cb060)?;
// 0x110cabc8 tbz w0, #0, #0x110cac3c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x110cac3c;continue;}pc=0x110cabcc;continue;
}
0x110cabcc=>{
// 0x110cabcc ldr x0, [x19, #0xa8]
{let address=(cpu.x[19]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110cabd0 bl #0x10d3d464
cpu.x[30]=0x110cabd4;super::chunk1::f_10d3d464(cpu,rt,0x10d3d464)?;
// 0x110cabd4 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x110cabd8 csinc w8, w20, wzr, ne
cpu.x[8]=(if cpu.condition(1){((cpu.x[20] as u32 as u64))}else{(((0u64))).wrapping_add(1)}) as u32 as u64;
// 0x110cabdc tbnz w8, #0, #0x110cac40
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x110cac40;continue;}pc=0x110cabe0;continue;
}
0x110cabe0=>{
// 0x110cabe0 b #0x110cac58
pc=0x110cac58;continue;
}
0x110cabe4=>{
// 0x110cabe4 mov w20, wzr
cpu.x[20]=(((0u64))) as u32 as u64;
pc=0x110cabe8;continue;
}
0x110cabe8=>{
// 0x110cabe8 ldr x0, [x19, #0xa8]
{let address=(cpu.x[19]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110cabec bl #0x10d360a8
cpu.x[30]=0x110cabf0;super::chunk4::f_10d360a8(cpu,rt,0x10d360a8)?;
pc=0x110cabf0;continue;
}
0x110cabf0=>{
// 0x110cabf0 tbnz w20, #0, #0x110caad8
if ((cpu.x[20] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x110caad8;continue;}pc=0x110cabf4;continue;
}
0x110cabf4=>{
// 0x110cabf4 tbz w0, #0, #0x110caad8
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x110caad8;continue;}pc=0x110cabf8;continue;
}
0x110cabf8=>{
// 0x110cabf8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x110cabfc bl #0x110cb17c
cpu.x[30]=0x110cac00;super::chunk14::f_110cb17c(cpu,rt,0x110cb17c)?;
pc=0x110cac00;continue;
}
0x110cac00=>{
// 0x110cac00 b #0x110caad8
pc=0x110caad8;continue;
}
0x110cac04=>{
// 0x110cac04 mov w20, #1
cpu.x[20]=(((0x1u64))) as u32 as u64;
// 0x110cac08 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x110cac0c cbnz w21, #0x110cabc4
if (cpu.x[21] as u32 as u64) != 0 {pc=0x110cabc4;continue;}pc=0x110cac10;continue;
}
0x110cac10=>{
// 0x110cac10 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x110cac14 mov w2, wzr
cpu.x[2]=(((0u64))) as u32 as u64;
// 0x110cac18 bl #0x110cae90
cpu.x[30]=0x110cac1c;super::chunk7::f_110cae90(cpu,rt,0x110cae90)?;
pc=0x110cac1c;continue;
}
0x110cac1c=>{
// 0x110cac1c ldr x0, [x19, #0xa8]
{let address=(cpu.x[19]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110cac20 bl #0x10d3d464
cpu.x[30]=0x110cac24;super::chunk1::f_10d3d464(cpu,rt,0x10d3d464)?;
pc=0x110cac24;continue;
}
0x110cac24=>{
// 0x110cac24 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x110cac28 b.lt #0x110cacd0
if cpu.condition(11){pc=0x110cacd0;continue;}pc=0x110cac2c;continue;
}
0x110cac2c=>{
// 0x110cac2c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x110cac30 bl #0x110cb060
cpu.x[30]=0x110cac34;super::chunk11::f_110cb060(cpu,rt,0x110cb060)?;
pc=0x110cac34;continue;
}
0x110cac34=>{
// 0x110cac34 tbnz w0, #0, #0x110cacd0
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x110cacd0;continue;}pc=0x110cac38;continue;
}
0x110cac38=>{
// 0x110cac38 b #0x110cacf0
pc=0x110cacf0;continue;
}
0x110cac3c=>{
// 0x110cac3c cbz w20, #0x110cacac
if (cpu.x[20] as u32 as u64) == 0 {pc=0x110cacac;continue;}pc=0x110cac40;continue;
}
0x110cac40=>{
// 0x110cac40 ldr w8, [x19, #0x1f8]
{let address=(cpu.x[19]).wrapping_add(0x1f8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x110cac44 cmp w8, #0x9c3
{let a=((cpu.x[8] as u32 as u64));let b=((0x9c3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x110cac48 b.gt #0x110cac58
if cpu.condition(12){pc=0x110cac58;continue;}pc=0x110cac4c;continue;
}
0x110cac4c=>{
// 0x110cac4c add w8, w8, #0x32
{let a=((cpu.x[8] as u32 as u64));let b=((0x32u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x110cac50 str w8, [x19, #0x1f8]
{let address=(cpu.x[19]).wrapping_add(0x1f8u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x110cac54 b #0x110cacbc
pc=0x110cacbc;continue;
}
0x110cac58=>{
// 0x110cac58 cbz w20, #0x110cacac
if (cpu.x[20] as u32 as u64) == 0 {pc=0x110cacac;continue;}pc=0x110cac5c;continue;
}
0x110cac5c=>{
// 0x110cac5c ldr x0, [x19, #0xa8]
{let address=(cpu.x[19]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110cac60 bl #0x10d36e90
cpu.x[30]=0x110cac64;super::chunk6::f_10d36e90(cpu,rt,0x10d36e90)?;
// 0x110cac64 tbz w0, #0, #0x110cac88
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x110cac88;continue;}pc=0x110cac68;continue;
}
0x110cac68=>{
// 0x110cac68 ldr x0, [x19, #0xa8]
{let address=(cpu.x[19]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110cac6c mov w1, #2
cpu.x[1]=(((0x2u64))) as u32 as u64;
// 0x110cac70 bl #0x10d35d5c
cpu.x[30]=0x110cac74;super::chunk13::f_10d35d5c(cpu,rt,0x10d35d5c)?;
// 0x110cac74 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110cac78 ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110cac7c blr x8
{let target=cpu.x[8];cpu.x[30]=0x110cac80;super::dispatch(cpu,rt,target)?;}
// 0x110cac80 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x110cac84 str w8, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x110cac88;continue;
}
0x110cac88=>{
// 0x110cac88 ldr x0, [x19, #0xa8]
{let address=(cpu.x[19]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110cac8c mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x110cac90 bl #0x10d35d5c
cpu.x[30]=0x110cac94;super::chunk13::f_10d35d5c(cpu,rt,0x10d35d5c)?;
// 0x110cac94 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110cac98 ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110cac9c blr x8
{let target=cpu.x[8];cpu.x[30]=0x110caca0;super::dispatch(cpu,rt,target)?;}
// 0x110caca0 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x110caca4 str w8, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x110caca8 b #0x110cacbc
pc=0x110cacbc;continue;
}
0x110cacac=>{
// 0x110cacac mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x110cacb0 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x110cacb4 mov w2, #1
cpu.x[2]=(((0x1u64))) as u32 as u64;
// 0x110cacb8 bl #0x110cae90
cpu.x[30]=0x110cacbc;super::chunk7::f_110cae90(cpu,rt,0x110cae90)?;
pc=0x110cacbc;continue;
}
0x110cacbc=>{
// 0x110cacbc ldr x0, [x19, #0xa8]
{let address=(cpu.x[19]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110cacc0 bl #0x10d3d464
cpu.x[30]=0x110cacc4;super::chunk1::f_10d3d464(cpu,rt,0x10d3d464)?;
// 0x110cacc4 cmp w0, #0
{let a=((cpu.x[0] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x110cacc8 csinc w8, w20, wzr, le
cpu.x[8]=(if cpu.condition(13){((cpu.x[20] as u32 as u64))}else{(((0u64))).wrapping_add(1)}) as u32 as u64;
// 0x110caccc tbnz w8, #0, #0x110cacf0
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x110cacf0;continue;}pc=0x110cacd0;continue;
}
0x110cacd0=>{
// 0x110cacd0 ldr x0, [x19, #0xa8]
{let address=(cpu.x[19]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110cacd4 bl #0x10d34428
cpu.x[30]=0x110cacd8;super::chunk13::f_10d34428(cpu,rt,0x10d34428)?;
// 0x110cacd8 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110cacdc mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x110cace0 ldr x8, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110cace4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110cace8;super::dispatch(cpu,rt,target)?;}
// 0x110cace8 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x110cacec strb w8, [x19, #0x1ef]
{let address=(cpu.x[19]).wrapping_add(0x1efu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
pc=0x110cacf0;continue;
}
0x110cacf0=>{
// 0x110cacf0 ldr x0, [x19, #0xa8]
{let address=(cpu.x[19]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110cacf4 bl #0x10d34428
cpu.x[30]=0x110cacf8;super::chunk13::f_10d34428(cpu,rt,0x10d34428)?;
pc=0x110cacf8;continue;
}
0x110cacf8=>{
// 0x110cacf8 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110cacfc ldr x8, [x8, #0x90]
{let address=(cpu.x[8]).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110cad00 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110cad04;super::dispatch(cpu,rt,target)?;}
pc=0x110cad04;continue;
}
0x110cad04=>{
// 0x110cad04 ldr x0, [x19, #0xa8]
{let address=(cpu.x[19]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110cad08 bl #0x10d34428
cpu.x[30]=0x110cad0c;super::chunk13::f_10d34428(cpu,rt,0x10d34428)?;
pc=0x110cad0c;continue;
}
0x110cad0c=>{
// 0x110cad0c ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110cad10 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x110cad14 ldr x8, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x110cad18 blr x8
{let target=cpu.x[8];cpu.x[30]=0x110cad1c;super::dispatch(cpu,rt,target)?;}
pc=0x110cad1c;continue;
}
0x110cad1c=>{
// 0x110cad1c ldr x0, [x19, #0x38]
{let address=(cpu.x[19]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110cad20 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x110cad24 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x110cad28 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x110cad2c b #0x10d75ed0
super::chunk6::f_10d75ed0(cpu,rt,0x10d75ed0)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x110ca9a0")),}}}
#[inline]
pub(super) unsafe fn f_110cb308(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x110cb308=>{
// 0x110cb308 ldr x0, [x0, #0xb0]
{let address=(cpu.x[0]).wrapping_add(0xb0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x110cb30c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x110cb308")),}}}
#[inline]
pub(super) unsafe fn f_111d6398(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111d6398=>{
// 0x111d6398 sub sp, sp, #0xa0
{let a=((cpu.sp));let b=((0xa0u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x111d639c stp x29, x30, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d63a0 stp x28, x27, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=cpu.x[28];let v1=cpu.x[27];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d63a4 stp x26, x25, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=cpu.x[26];let v1=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d63a8 stp x24, x23, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d63ac stp x22, x21, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d63b0 stp x20, x19, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d63b4 add x29, sp, #0x40
{let a=((cpu.sp));let b=((0x40u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x111d63b8 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x111d63bc ldr x0, [x0, #0x20]
{let address=(cpu.x[0]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d63c0 mov x27, x8
cpu.x[27]=(((cpu.x[8]))) as u64;
// 0x111d63c4 ldr w9, [x20, #0x34]
{let address=(cpu.x[20]).wrapping_add(0x34u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x111d63c8 ldr x21, [x0, #0xa8]
{let address=(cpu.x[0]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x111d63cc cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d63d0 b.lt #0x111d6748
if cpu.condition(11){pc=0x111d6748;continue;}pc=0x111d63d4;continue;
}
0x111d63d4=>{
// 0x111d63d4 mov x22, xzr
cpu.x[22]=(((0u64))) as u64;
// 0x111d63d8 b #0x111d6404
pc=0x111d6404;continue;
}
0x111d63dc=>{
// 0x111d63dc mov x25, xzr
cpu.x[25]=(((0u64))) as u64;
// 0x111d63e0 ldr x8, [x20, #0x28]
{let address=(cpu.x[20]).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d63e4 ldr w19, [x8, x22, lsl #2]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[22])) as u64).wrapping_shl(2) as u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[19]=(v0) as u32 as u64;}
// 0x111d63e8 subs w9, w19, #1
{let a=((cpu.x[19] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x111d63ec b.lt #0x111d64d8
if cpu.condition(11){pc=0x111d64d8;continue;}pc=0x111d63f0;continue;
}
0x111d63f0=>{
// 0x111d63f0 str w9, [x8, x22, lsl #2]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[22])) as u64).wrapping_shl(2) as u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x111d63f4;continue;
}
0x111d63f4=>{
// 0x111d63f4 ldrsw x8, [x20, #0x34]
{let address=(cpu.x[20]).wrapping_add(0x34u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x111d63f8 add x22, x22, #1
{let a=((cpu.x[22]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[22]=(result) as u64;}
// 0x111d63fc cmp x22, x8
{let a=((cpu.x[22]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
pc=0x111d6400;continue;
}
0x111d6400=>{
// 0x111d6400 b.ge #0x111d6744
if cpu.condition(10){pc=0x111d6744;continue;}pc=0x111d6404;continue;
}
0x111d6404=>{
// 0x111d6404 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x111d6408 mov w1, w22
cpu.x[1]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x111d640c bl #0x10d37430
cpu.x[30]=0x111d6410;super::chunk0::f_10d37430(cpu,rt,0x10d37430)?;
pc=0x111d6410;continue;
}
0x111d6410=>{
// 0x111d6410 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d6414 ldr x8, [x8, #0x30]
{let address=(cpu.x[8]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d6418 blr x8
{let target=cpu.x[8];cpu.x[30]=0x111d641c;super::dispatch(cpu,rt,target)?;}
pc=0x111d641c;continue;
}
0x111d641c=>{
// 0x111d641c tbnz w0, #0, #0x111d63f4
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x111d63f4;continue;}pc=0x111d6420;continue;
}
0x111d6420=>{
// 0x111d6420 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x111d6424 bl #0x10d36720
cpu.x[30]=0x111d6428;super::chunk11::f_10d36720(cpu,rt,0x10d36720)?;
pc=0x111d6428;continue;
}
0x111d6428=>{
// 0x111d6428 tbnz w0, #0, #0x111d63f4
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x111d63f4;continue;}pc=0x111d642c;continue;
}
0x111d642c=>{
// 0x111d642c mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x111d6430 bl #0x10d36098
cpu.x[30]=0x111d6434;super::chunk1::f_10d36098(cpu,rt,0x10d36098)?;
pc=0x111d6434;continue;
}
0x111d6434=>{
// 0x111d6434 tbnz w0, #0, #0x111d63f4
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x111d63f4;continue;}pc=0x111d6438;continue;
}
0x111d6438=>{
// 0x111d6438 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x111d643c bl #0x10d360a8
cpu.x[30]=0x111d6440;super::chunk4::f_10d360a8(cpu,rt,0x10d360a8)?;
pc=0x111d6440;continue;
}
0x111d6440=>{
// 0x111d6440 tbnz w0, #0, #0x111d63f4
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x111d63f4;continue;}pc=0x111d6444;continue;
}
0x111d6444=>{
// 0x111d6444 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x111d6448 mov w1, w22
cpu.x[1]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x111d644c bl #0x10d35d5c
cpu.x[30]=0x111d6450;super::chunk13::f_10d35d5c(cpu,rt,0x10d35d5c)?;
pc=0x111d6450;continue;
}
0x111d6450=>{
// 0x111d6450 mov x24, x0
cpu.x[24]=(((cpu.x[0]))) as u64;
// 0x111d6454 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x111d6458 mov w1, w22
cpu.x[1]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x111d645c bl #0x10d3c980
cpu.x[30]=0x111d6460;super::chunk13::f_10d3c980(cpu,rt,0x10d3c980)?;
pc=0x111d6460;continue;
}
0x111d6460=>{
// 0x111d6460 ldr x8, [x20, #0x20]
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d6464 mov x23, x0
cpu.x[23]=(((cpu.x[0]))) as u64;
// 0x111d6468 ldr x26, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[26]=(v0) as u64;}
// 0x111d646c ldr w8, [x26, #0x14]
{let address=(cpu.x[26]).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d6470 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d6474 b.lt #0x111d63dc
if cpu.condition(11){pc=0x111d63dc;continue;}pc=0x111d6478;continue;
}
0x111d6478=>{
// 0x111d6478 mov x19, xzr
cpu.x[19]=(((0u64))) as u64;
// 0x111d647c b #0x111d6490
pc=0x111d6490;continue;
}
0x111d6480=>{
// 0x111d6480 ldrsw x8, [x26, #0x14]
{let address=(cpu.x[26]).wrapping_add(0x14u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x111d6484 add x19, x19, #1
{let a=((cpu.x[19]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[19]=(result) as u64;}
// 0x111d6488 cmp x19, x8
{let a=((cpu.x[19]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x111d648c b.ge #0x111d63dc
if cpu.condition(10){pc=0x111d63dc;continue;}pc=0x111d6490;continue;
}
0x111d6490=>{
// 0x111d6490 ldr x8, [x26, #8]
{let address=(cpu.x[26]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d6494 ldr x25, [x8, x19, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[19])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[25]=(v0) as u64;}
// 0x111d6498 mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x111d649c bl #0x10f944ac
cpu.x[30]=0x111d64a0;super::chunk4::f_10f944ac(cpu,rt,0x10f944ac)?;
pc=0x111d64a0;continue;
}
0x111d64a0=>{
// 0x111d64a0 cmp x0, x24
{let a=((cpu.x[0]));let b=((cpu.x[24]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x111d64a4 b.ne #0x111d6480
if cpu.condition(1){pc=0x111d6480;continue;}pc=0x111d64a8;continue;
}
0x111d64a8=>{
// 0x111d64a8 ldr x8, [x25]
{let address=(cpu.x[25]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d64ac mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x111d64b0 ldr x8, [x8, #0x190]
{let address=(cpu.x[8]).wrapping_add(0x190u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d64b4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x111d64b8;super::dispatch(cpu,rt,target)?;}
pc=0x111d64b8;continue;
}
0x111d64b8=>{
// 0x111d64b8 tbz w0, #0, #0x111d6480
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x111d6480;continue;}pc=0x111d64bc;continue;
}
0x111d64bc=>{
// 0x111d64bc mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x111d64c0 bl #0x10f85a08
cpu.x[30]=0x111d64c4;super::chunk12::f_10f85a08(cpu,rt,0x10f85a08)?;
pc=0x111d64c4;continue;
}
0x111d64c4=>{
// 0x111d64c4 tbz w0, #0, #0x111d6480
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x111d6480;continue;}pc=0x111d64c8;continue;
}
0x111d64c8=>{
// 0x111d64c8 ldr x8, [x20, #0x28]
{let address=(cpu.x[20]).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d64cc ldr w19, [x8, x22, lsl #2]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[22])) as u64).wrapping_shl(2) as u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[19]=(v0) as u32 as u64;}
// 0x111d64d0 subs w9, w19, #1
{let a=((cpu.x[19] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x111d64d4 b.ge #0x111d63f0
if cpu.condition(10){pc=0x111d63f0;continue;}pc=0x111d64d8;continue;
}
0x111d64d8=>{
// 0x111d64d8 ldr x0, [x20, #0x18]
{let address=(cpu.x[20]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d64dc mov w1, #0xb
cpu.x[1]=(((0xbu64))) as u32 as u64;
// 0x111d64e0 bl #0x11294bb4
cpu.x[30]=0x111d64e4;super::chunk13::f_11294bb4(cpu,rt,0x11294bb4)?;
pc=0x111d64e4;continue;
}
0x111d64e4=>{
// 0x111d64e4 ldr x8, [x20, #0x28]
{let address=(cpu.x[20]).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d64e8 cmn w19, #0x3e7
{let a=((cpu.x[19] as u32 as u64));let b=((0x3e7u64));let result=a.wrapping_add(b);cpu.flags(a,b,result,32,false);}
// 0x111d64ec str w0, [x8, x22, lsl #2]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[22])) as u64).wrapping_shl(2) as u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111d64f0 b.eq #0x111d63f4
if cpu.condition(0){pc=0x111d63f4;continue;}pc=0x111d64f4;continue;
}
0x111d64f4=>{
// 0x111d64f4 ldr x8, [x20, #0x38]
{let address=(cpu.x[20]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d64f8 mov w19, #1
cpu.x[19]=(((0x1u64))) as u32 as u64;
// 0x111d64fc ldr w28, [x8, x22, lsl #2]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[22])) as u64).wrapping_shl(2) as u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[28]=(v0) as u32 as u64;}
// 0x111d6500 sub w8, w28, #1
{let a=((cpu.x[28] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x111d6504 cmp w8, #4
{let a=((cpu.x[8] as u32 as u64));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d6508 b.hs #0x111d6528
if cpu.condition(2){pc=0x111d6528;continue;}pc=0x111d650c;continue;
}
0x111d650c=>{
// 0x111d650c mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x111d6510 bl #0x10d36084
cpu.x[30]=0x111d6514;super::chunk6::f_10d36084(cpu,rt,0x10d36084)?;
pc=0x111d6514;continue;
}
0x111d6514=>{
// 0x111d6514 tbz w0, #0, #0x111d6570
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x111d6570;continue;}pc=0x111d6518;continue;
}
0x111d6518=>{
// 0x111d6518 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x111d651c bl #0x10d43930
cpu.x[30]=0x111d6520;super::dispatch(cpu,rt,0x10d43930)?;
// 0x111d6520 tbz w0, #0, #0x111d6570
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x111d6570;continue;}pc=0x111d6524;continue;
}
0x111d6524=>{
// 0x111d6524 b #0x111d6734
pc=0x111d6734;continue;
}
0x111d6528=>{
// 0x111d6528 cmp w28, #5
{let a=((cpu.x[28] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d652c b.eq #0x111d66a0
if cpu.condition(0){pc=0x111d66a0;continue;}pc=0x111d6530;continue;
}
0x111d6530=>{
// 0x111d6530 cbnz w28, #0x111d6734
if (cpu.x[28] as u32 as u64) != 0 {pc=0x111d6734;continue;}pc=0x111d6534;continue;
}
0x111d6534=>{
// 0x111d6534 cbz x25, #0x111d654c
if (cpu.x[25]) == 0 {pc=0x111d654c;continue;}pc=0x111d6538;continue;
}
0x111d6538=>{
// 0x111d6538 ldr x0, [x20, #0x18]
{let address=(cpu.x[20]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d653c mov w1, #0x64
cpu.x[1]=(((0x64u64))) as u32 as u64;
// 0x111d6540 bl #0x11294bb4
cpu.x[30]=0x111d6544;super::chunk13::f_11294bb4(cpu,rt,0x11294bb4)?;
pc=0x111d6544;continue;
}
0x111d6544=>{
// 0x111d6544 cmp w0, #0x55
{let a=((cpu.x[0] as u32 as u64));let b=((0x55u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d6548 b.lt #0x111d66a4
if cpu.condition(11){pc=0x111d66a4;continue;}pc=0x111d654c;continue;
}
0x111d654c=>{
// 0x111d654c ldr x0, [x20, #0x18]
{let address=(cpu.x[20]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6550 mov w1, #4
cpu.x[1]=(((0x4u64))) as u32 as u64;
// 0x111d6554 bl #0x11294bb4
cpu.x[30]=0x111d6558;super::chunk13::f_11294bb4(cpu,rt,0x11294bb4)?;
pc=0x111d6558;continue;
}
0x111d6558=>{
// 0x111d6558 sub w8, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x111d655c cmp w8, #3
{let a=((cpu.x[8] as u32 as u64));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d6560 csinc w28, w19, w0, hs
cpu.x[28]=(if cpu.condition(2){((cpu.x[19] as u32 as u64))}else{(((cpu.x[0] as u32 as u64))).wrapping_add(1)}) as u32 as u64;
// 0x111d6564 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x111d6568 bl #0x10d36084
cpu.x[30]=0x111d656c;super::chunk6::f_10d36084(cpu,rt,0x10d36084)?;
pc=0x111d656c;continue;
}
0x111d656c=>{
// 0x111d656c tbnz w0, #0, #0x111d6518
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x111d6518;continue;}pc=0x111d6570;continue;
}
0x111d6570=>{
// 0x111d6570 sub w25, w28, #1
{let a=((cpu.x[28] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[25]=(result) as u32 as u64;}
// 0x111d6574 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x111d6578 mov w1, w25
cpu.x[1]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x111d657c bl #0x10fa3e84
cpu.x[30]=0x111d6580;super::chunk15::f_10fa3e84(cpu,rt,0x10fa3e84)?;
pc=0x111d6580;continue;
}
0x111d6580=>{
// 0x111d6580 tbnz w0, #0x1f, #0x111d6738
if ((cpu.x[0] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x111d6738;continue;}pc=0x111d6584;continue;
}
0x111d6584=>{
// 0x111d6584 mov w1, w0
cpu.x[1]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x111d6588 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x111d658c bl #0x10f9c5c8
cpu.x[30]=0x111d6590;super::chunk15::f_10f9c5c8(cpu,rt,0x10f9c5c8)?;
pc=0x111d6590;continue;
}
0x111d6590=>{
// 0x111d6590 tbnz w0, #0, #0x111d6738
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x111d6738;continue;}pc=0x111d6594;continue;
}
0x111d6594=>{
// 0x111d6594 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x111d6598 mov w1, w25
cpu.x[1]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x111d659c bl #0x10f9dd98
cpu.x[30]=0x111d65a0;super::chunk15::f_10f9dd98(cpu,rt,0x10f9dd98)?;
pc=0x111d65a0;continue;
}
0x111d65a0=>{
// 0x111d65a0 cbz x0, #0x111d6738
if (cpu.x[0]) == 0 {pc=0x111d6738;continue;}pc=0x111d65a4;continue;
}
0x111d65a4=>{
// 0x111d65a4 sub x8, x29, #0x18
{let a=((cpu.x[29]));let b=((0x18u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u64;}
// 0x111d65a8 mov x1, x24
cpu.x[1]=(((cpu.x[24]))) as u64;
// 0x111d65ac mov w2, #1
cpu.x[2]=(((0x1u64))) as u32 as u64;
// 0x111d65b0 stp x23, x27, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=cpu.x[23];let v1=cpu.x[27];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d65b4 bl #0x10e80bbc
cpu.x[30]=0x111d65b8;super::chunk0::f_10e80bbc(cpu,rt,0x10e80bbc)?;
pc=0x111d65b8;continue;
}
0x111d65b8=>{
// 0x111d65b8 sub x0, x29, #0x18
{let a=((cpu.x[29]));let b=((0x18u64));let result=a.wrapping_sub(b);cpu.x[0]=(result) as u64;}
// 0x111d65bc bl #0x10e80680
cpu.x[30]=0x111d65c0;super::chunk0::f_10e80680(cpu,rt,0x10e80680)?;
pc=0x111d65c0;continue;
}
0x111d65c0=>{
// 0x111d65c0 mov x27, x0
cpu.x[27]=(((cpu.x[0]))) as u64;
// 0x111d65c4 mov w19, wzr
cpu.x[19]=(((0u64))) as u32 as u64;
pc=0x111d65c8;continue;
}
0x111d65c8=>{
// 0x111d65c8 ldr x25, [x20, #0x18]
{let address=(cpu.x[20]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[25]=(v0) as u64;}
// 0x111d65cc mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x111d65d0 bl #0x10d358a0
cpu.x[30]=0x111d65d4;super::chunk0::f_10d358a0(cpu,rt,0x10d358a0)?;
pc=0x111d65d4;continue;
}
0x111d65d4=>{
// 0x111d65d4 mov w23, #0x1f4
cpu.x[23]=(((0x1f4u64))) as u32 as u64;
// 0x111d65d8 mul w1, w0, w23
cpu.x[1]=((((cpu.x[0] as u32 as u64))).wrapping_mul(((cpu.x[23] as u32 as u64)))) as u32 as u64;
// 0x111d65dc mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x111d65e0 bl #0x11294bb4
cpu.x[30]=0x111d65e4;super::chunk13::f_11294bb4(cpu,rt,0x11294bb4)?;
pc=0x111d65e4;continue;
}
0x111d65e4=>{
// 0x111d65e4 ldr x26, [x20, #0x18]
{let address=(cpu.x[20]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[26]=(v0) as u64;}
// 0x111d65e8 mov w25, w0
cpu.x[25]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x111d65ec mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x111d65f0 bl #0x10d358a8
cpu.x[30]=0x111d65f4;super::chunk13::f_10d358a8(cpu,rt,0x10d358a8)?;
pc=0x111d65f4;continue;
}
0x111d65f4=>{
// 0x111d65f4 mul w1, w0, w23
cpu.x[1]=((((cpu.x[0] as u32 as u64))).wrapping_mul(((cpu.x[23] as u32 as u64)))) as u32 as u64;
// 0x111d65f8 mov x0, x26
cpu.x[0]=(((cpu.x[26]))) as u64;
// 0x111d65fc bl #0x11294bb4
cpu.x[30]=0x111d6600;super::chunk13::f_11294bb4(cpu,rt,0x11294bb4)?;
pc=0x111d6600;continue;
}
0x111d6600=>{
// 0x111d6600 mov w26, w0
cpu.x[26]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x111d6604 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x111d6608 mov w1, w25
cpu.x[1]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x111d660c mov w2, w26
cpu.x[2]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x111d6610 mov x3, x27
cpu.x[3]=(((cpu.x[27]))) as u64;
// 0x111d6614 mov w4, wzr
cpu.x[4]=(((0u64))) as u32 as u64;
// 0x111d6618 bl #0x10d42214
cpu.x[30]=0x111d661c;super::chunk4::f_10d42214(cpu,rt,0x10d42214)?;
pc=0x111d661c;continue;
}
0x111d661c=>{
// 0x111d661c cbz w0, #0x111d662c
if (cpu.x[0] as u32 as u64) == 0 {pc=0x111d662c;continue;}pc=0x111d6620;continue;
}
0x111d6620=>{
// 0x111d6620 cmp w19, #0x1f
{let a=((cpu.x[19] as u32 as u64));let b=((0x1fu64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d6624 add w19, w19, #1
{let a=((cpu.x[19] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[19]=(result) as u32 as u64;}
// 0x111d6628 b.lo #0x111d65c8
if cpu.condition(3){pc=0x111d65c8;continue;}pc=0x111d662c;continue;
}
0x111d662c=>{
// 0x111d662c mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x111d6630 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x111d6634 bl #0x10fa1c84
cpu.x[30]=0x111d6638;super::chunk14::f_10fa1c84(cpu,rt,0x10fa1c84)?;
pc=0x111d6638;continue;
}
0x111d6638=>{
// 0x111d6638 mov w24, w0
cpu.x[24]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x111d663c sub x0, x29, #0x18
{let a=((cpu.x[29]));let b=((0x18u64));let result=a.wrapping_sub(b);cpu.x[0]=(result) as u64;}
// 0x111d6640 bl #0x10e80638
cpu.x[30]=0x111d6644;super::chunk0::f_10e80638(cpu,rt,0x10e80638)?;
pc=0x111d6644;continue;
}
0x111d6644=>{
// 0x111d6644 cmp w24, w0
{let a=((cpu.x[24] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d6648 b.lt #0x111d6698
if cpu.condition(11){pc=0x111d6698;continue;}pc=0x111d664c;continue;
}
0x111d664c=>{
// 0x111d664c ldr x24, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[24]=(v0) as u64;}
// 0x111d6650 mov w0, #0x58
cpu.x[0]=(((0x58u64))) as u32 as u64;
// 0x111d6654 bl #0x1187d2b0
cpu.x[30]=0x111d6658;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
pc=0x111d6658;continue;
}
0x111d6658=>{
// 0x111d6658 mov x1, x24
cpu.x[1]=(((cpu.x[24]))) as u64;
// 0x111d665c mov x27, x0
cpu.x[27]=(((cpu.x[0]))) as u64;
// 0x111d6660 bl #0x10d774ac
cpu.x[30]=0x111d6664;super::chunk6::f_10d774ac(cpu,rt,0x10d774ac)?;
pc=0x111d6664;continue;
}
0x111d6664=>{
// 0x111d6664 sub x1, x29, #0x18
{let a=((cpu.x[29]));let b=((0x18u64));let result=a.wrapping_sub(b);cpu.x[1]=(result) as u64;}
// 0x111d6668 mov x0, x27
cpu.x[0]=(((cpu.x[27]))) as u64;
// 0x111d666c bl #0x10d77c8c
cpu.x[30]=0x111d6670;super::chunk8::f_10d77c8c(cpu,rt,0x10d77c8c)?;
pc=0x111d6670;continue;
}
0x111d6670=>{
// 0x111d6670 mov x0, x27
cpu.x[0]=(((cpu.x[27]))) as u64;
// 0x111d6674 mov w1, w25
cpu.x[1]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x111d6678 mov w2, w26
cpu.x[2]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x111d667c bl #0x10d77a14
cpu.x[30]=0x111d6680;super::chunk15::f_10d77a14(cpu,rt,0x10d77a14)?;
pc=0x111d6680;continue;
}
0x111d6680=>{
// 0x111d6680 ldr x2, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x111d6684 add x1, sp, #0x20
{let a=((cpu.sp));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x111d6688 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111d668c str x27, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[27];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111d6690 bl #0x111d680c
cpu.x[30]=0x111d6694;super::chunk7::f_111d680c(cpu,rt,0x111d680c)?;
pc=0x111d6694;continue;
}
0x111d6694=>{
// 0x111d6694 mov w28, wzr
cpu.x[28]=(((0u64))) as u32 as u64;
pc=0x111d6698;continue;
}
0x111d6698=>{
// 0x111d6698 ldr x27, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[27]=(v0) as u64;}
// 0x111d669c b #0x111d6738
pc=0x111d6738;continue;
}
0x111d66a0=>{
// 0x111d66a0 cbz x25, #0x111d6734
if (cpu.x[25]) == 0 {pc=0x111d6734;continue;}pc=0x111d66a4;continue;
}
0x111d66a4=>{
// 0x111d66a4 ldr x1, [x20, #0x20]
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x111d66a8 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x111d66ac mov x2, x25
cpu.x[2]=(((cpu.x[25]))) as u64;
// 0x111d66b0 bl #0x10fa4768
cpu.x[30]=0x111d66b4;super::chunk6::f_10fa4768(cpu,rt,0x10fa4768)?;
pc=0x111d66b4;continue;
}
0x111d66b4=>{
// 0x111d66b4 cbnz w0, #0x111d66fc
if (cpu.x[0] as u32 as u64) != 0 {pc=0x111d66fc;continue;}pc=0x111d66b8;continue;
}
0x111d66b8=>{
// 0x111d66b8 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x111d66bc mov x1, x25
cpu.x[1]=(((cpu.x[25]))) as u64;
// 0x111d66c0 bl #0x10fa46c8
cpu.x[30]=0x111d66c4;super::chunk0::f_10fa46c8(cpu,rt,0x10fa46c8)?;
pc=0x111d66c4;continue;
}
0x111d66c4=>{
// 0x111d66c4 mov w1, w0
cpu.x[1]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x111d66c8 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x111d66cc bl #0x10fa04a8
cpu.x[30]=0x111d66d0;super::chunk2::f_10fa04a8(cpu,rt,0x10fa04a8)?;
pc=0x111d66d0;continue;
}
0x111d66d0=>{
// 0x111d66d0 ldr w8, [x0, #0xb4]
{let address=(cpu.x[0]).wrapping_add(0xb4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d66d4 tbz w8, #0x1f, #0x111d66fc
if ((cpu.x[8] as u32 as u64) & (1u64 << 31)) == 0 {pc=0x111d66fc;continue;}pc=0x111d66d8;continue;
}
0x111d66d8=>{
// 0x111d66d8 mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x111d66dc bl #0x10f7e050
cpu.x[30]=0x111d66e0;super::chunk10::f_10f7e050(cpu,rt,0x10f7e050)?;
pc=0x111d66e0;continue;
}
0x111d66e0=>{
// 0x111d66e0 ldr x19, [x0, #0x710]
{let address=(cpu.x[0]).wrapping_add(0x710u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x111d66e4 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x111d66e8 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x111d66ec bl #0x10fa1c84
cpu.x[30]=0x111d66f0;super::chunk14::f_10fa1c84(cpu,rt,0x10fa1c84)?;
pc=0x111d66f0;continue;
}
0x111d66f0=>{
// 0x111d66f0 ldr w8, [x19, #0xb4]
{let address=(cpu.x[19]).wrapping_add(0xb4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d66f4 cmp w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d66f8 b.ge #0x111d6704
if cpu.condition(10){pc=0x111d6704;continue;}pc=0x111d66fc;continue;
}
0x111d66fc=>{
// 0x111d66fc mov w28, #5
cpu.x[28]=(((0x5u64))) as u32 as u64;
// 0x111d6700 b #0x111d6738
pc=0x111d6738;continue;
}
0x111d6704=>{
// 0x111d6704 ldr x24, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[24]=(v0) as u64;}
// 0x111d6708 mov w0, #0x38
cpu.x[0]=(((0x38u64))) as u32 as u64;
// 0x111d670c bl #0x1187d2b0
cpu.x[30]=0x111d6710;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
pc=0x111d6710;continue;
}
0x111d6710=>{
// 0x111d6710 mov x1, x24
cpu.x[1]=(((cpu.x[24]))) as u64;
// 0x111d6714 mov x2, x25
cpu.x[2]=(((cpu.x[25]))) as u64;
// 0x111d6718 mov x26, x0
cpu.x[26]=(((cpu.x[0]))) as u64;
// 0x111d671c bl #0x10d7939c
cpu.x[30]=0x111d6720;super::chunk1::f_10d7939c(cpu,rt,0x10d7939c)?;
pc=0x111d6720;continue;
}
0x111d6720=>{
// 0x111d6720 add x1, sp, #0x18
{let a=((cpu.sp));let b=((0x18u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x111d6724 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111d6728 mov x2, x23
cpu.x[2]=(((cpu.x[23]))) as u64;
// 0x111d672c str x26, [sp, #0x18]
{let address=(cpu.sp).wrapping_add(0x18u64);let v0=cpu.x[26];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111d6730 bl #0x111d680c
cpu.x[30]=0x111d6734;super::chunk7::f_111d680c(cpu,rt,0x111d680c)?;
pc=0x111d6734;continue;
}
0x111d6734=>{
// 0x111d6734 mov w28, wzr
cpu.x[28]=(((0u64))) as u32 as u64;
pc=0x111d6738;continue;
}
0x111d6738=>{
// 0x111d6738 ldr x8, [x20, #0x38]
{let address=(cpu.x[20]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d673c str w28, [x8, x22, lsl #2]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[22])) as u64).wrapping_shl(2) as u64);let v0=cpu.x[28] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111d6740 b #0x111d63f4
pc=0x111d63f4;continue;
}
0x111d6744=>{
// 0x111d6744 ldr x0, [x20, #0x20]
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
pc=0x111d6748;continue;
}
0x111d6748=>{
// 0x111d6748 bl #0x110ca998
cpu.x[30]=0x111d674c;super::chunk3::f_110ca998(cpu,rt,0x110ca998)?;
pc=0x111d674c;continue;
}
0x111d674c=>{
// 0x111d674c ldr x0, [x20, #0x20]
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d6750 bl #0x110ca81c
cpu.x[30]=0x111d6754;super::chunk13::f_110ca81c(cpu,rt,0x110ca81c)?;
pc=0x111d6754;continue;
}
0x111d6754=>{
// 0x111d6754 ldr x8, [x20, #0x20]
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d6758 ldr x0, [x20, #0x70]
{let address=(cpu.x[20]).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d675c ldr x1, [x8, #0xa8]
{let address=(cpu.x[8]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x111d6760 bl #0x10d483a8
cpu.x[30]=0x111d6764;super::chunk7::f_10d483a8(cpu,rt,0x10d483a8)?;
pc=0x111d6764;continue;
}
0x111d6764=>{
// 0x111d6764 ldr x8, [x20, #0x10]
{let address=(cpu.x[20]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111d6768 ldrb w8, [x8, #0x68]
{let address=(cpu.x[8]).wrapping_add(0x68u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d676c tbnz w8, #0, #0x111d6778
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x111d6778;continue;}pc=0x111d6770;continue;
}
0x111d6770=>{
// 0x111d6770 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111d6774 bl #0x111d6ab4
cpu.x[30]=0x111d6778;super::chunk3::f_111d6ab4(cpu,rt,0x111d6ab4)?;
pc=0x111d6778;continue;
}
0x111d6778=>{
// 0x111d6778 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x111d677c mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x111d6780 bl #0x10d4220c
cpu.x[30]=0x111d6784;super::chunk12::f_10d4220c(cpu,rt,0x10d4220c)?;
pc=0x111d6784;continue;
}
0x111d6784=>{
// 0x111d6784 tbz w0, #0, #0x111d6794
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x111d6794;continue;}pc=0x111d6788;continue;
}
0x111d6788=>{
// 0x111d6788 mov x8, x27
cpu.x[8]=(((cpu.x[27]))) as u64;
// 0x111d678c bl #0x111d7a30
cpu.x[30]=0x111d6790;super::chunk4::f_111d7a30(cpu,rt,0x111d7a30)?;
pc=0x111d6790;continue;
}
0x111d6790=>{
// 0x111d6790 b #0x111d67ec
pc=0x111d67ec;continue;
}
0x111d6794=>{
// 0x111d6794 ldr w8, [x20, #0x48]
{let address=(cpu.x[20]).wrapping_add(0x48u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d6798 mov w9, #0x8ca0
cpu.x[9]=(((0x8ca0u64))) as u32 as u64;
// 0x111d679c add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x111d67a0 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d67a4 str w10, [x20, #0x48]
{let address=(cpu.x[20]).wrapping_add(0x48u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111d67a8 b.lt #0x111d67e8
if cpu.condition(11){pc=0x111d67e8;continue;}pc=0x111d67ac;continue;
}
0x111d67ac=>{
// 0x111d67ac mov w0, #0x18
cpu.x[0]=(((0x18u64))) as u32 as u64;
// 0x111d67b0 bl #0x1187d2b0
cpu.x[30]=0x111d67b4;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
// 0x111d67b4 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x111d67b8 adrp x1, #0x102d1000
cpu.x[1]=(0x102d1000u64) as u64;
// 0x111d67bc add x1, x1, #0xfd6
{let a=((cpu.x[1]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x111d67c0 sub x0, x29, #0x18
{let a=((cpu.x[29]));let b=((0x18u64));let result=a.wrapping_sub(b);cpu.x[0]=(result) as u64;}
// 0x111d67c4 bl #0x1142fde4
cpu.x[30]=0x111d67c8;super::chunk6::f_1142fde4(cpu,rt,0x1142fde4)?;
// 0x111d67c8 sub x2, x29, #0x18
{let a=((cpu.x[29]));let b=((0x18u64));let result=a.wrapping_sub(b);cpu.x[2]=(result) as u64;}
// 0x111d67cc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111d67d0 mov w1, #3
cpu.x[1]=(((0x3u64))) as u32 as u64;
// 0x111d67d4 bl #0x111d79f0
cpu.x[30]=0x111d67d8;super::dispatch(cpu,rt,0x111d79f0)?;
// 0x111d67d8 sub x0, x29, #0x18
{let a=((cpu.x[29]));let b=((0x18u64));let result=a.wrapping_sub(b);cpu.x[0]=(result) as u64;}
// 0x111d67dc str x20, [x27]
{let address=(cpu.x[27]).wrapping_add(0x0u64);let v0=cpu.x[20];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111d67e0 bl #0x1142fcc4
cpu.x[30]=0x111d67e4;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
// 0x111d67e4 b #0x111d67ec
pc=0x111d67ec;continue;
}
0x111d67e8=>{
// 0x111d67e8 str xzr, [x27]
{let address=(cpu.x[27]).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x111d67ec;continue;
}
0x111d67ec=>{
// 0x111d67ec ldp x20, x19, [sp, #0x90]
{let address=(cpu.sp).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d67f0 ldp x22, x21, [sp, #0x80]
{let address=(cpu.sp).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111d67f4 ldp x24, x23, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x111d67f8 ldp x26, x25, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x111d67fc ldp x28, x27, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
pc=0x111d6800;continue;
}
0x111d6800=>{
// 0x111d6800 ldp x29, x30, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x111d6804 add sp, sp, #0xa0
{let a=((cpu.sp));let b=((0xa0u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x111d6808 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111d6398")),}}}
#[inline]
pub(super) unsafe fn f_111d8afc(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111d8afc=>{
// 0x111d8afc stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x111d8b00 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111d8b04 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d8b08 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x111d8b0c mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x111d8b10 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x111d8b14 add x1, x0, #0x74
{let a=((cpu.x[0]));let b=((0x74u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x111d8b18 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111d8b1c mov w2, #2
cpu.x[2]=(((0x2u64))) as u32 as u64;
// 0x111d8b20 bl #0x111dd248
cpu.x[30]=0x111d8b24;super::chunk11::f_111dd248(cpu,rt,0x111dd248)?;
pc=0x111d8b24;continue;
}
0x111d8b24=>{
// 0x111d8b24 add x1, x19, #0x80
{let a=((cpu.x[19]));let b=((0x80u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x111d8b28 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111d8b2c bl #0x111dd4e8
cpu.x[30]=0x111d8b30;super::chunk2::f_111dd4e8(cpu,rt,0x111dd4e8)?;
pc=0x111d8b30;continue;
}
0x111d8b30=>{
// 0x111d8b30 ldp w8, w9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[8]=(v0) as u32 as u64;cpu.x[9]=(v1) as u32 as u64;}
// 0x111d8b34 ldr x21, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x111d8b38 strh wzr, [x19, #0x7c]
{let address=(cpu.x[19]).wrapping_add(0x7cu64);let v0=0u64;rt.memory.write::<u16>(address.wrapping_add(0),v0 as u16);}
// 0x111d8b3c strb wzr, [x19, #0x71]
{let address=(cpu.x[19]).wrapping_add(0x71u64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x111d8b40 mul w20, w9, w8
cpu.x[20]=((((cpu.x[9] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64)))) as u32 as u64;
// 0x111d8b44 str wzr, [x21, #0xc]
{let address=(cpu.x[21]).wrapping_add(0xcu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111d8b48 cmp w20, #1
{let a=((cpu.x[20] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d8b4c b.lt #0x111d8b78
if cpu.condition(11){pc=0x111d8b78;continue;}pc=0x111d8b50;continue;
}
0x111d8b50=>{
// 0x111d8b50 ldr w8, [x21, #8]
{let address=(cpu.x[21]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d8b54 cmp w8, w20
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[20] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d8b58 b.ge #0x111d8b90
if cpu.condition(10){pc=0x111d8b90;continue;}pc=0x111d8b5c;continue;
}
0x111d8b5c=>{
// 0x111d8b5c mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x111d8b60 mov w1, w20
cpu.x[1]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x111d8b64 bl #0x1078f4d4
cpu.x[30]=0x111d8b68;super::chunk2::f_1078f4d4(cpu,rt,0x1078f4d4)?;
// 0x111d8b68 ldr w8, [x21, #0xc]
{let address=(cpu.x[21]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d8b6c cmp w8, w20
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[20] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d8b70 b.lt #0x111d8b9c
if cpu.condition(11){pc=0x111d8b9c;continue;}pc=0x111d8b74;continue;
}
0x111d8b74=>{
// 0x111d8b74 b #0x111d8bb4
pc=0x111d8bb4;continue;
}
0x111d8b78=>{
// 0x111d8b78 tbnz w20, #0x1f, #0x111d8bb4
if ((cpu.x[20] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x111d8bb4;continue;}pc=0x111d8b7c;continue;
}
0x111d8b7c=>{
// 0x111d8b7c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x111d8b80 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d8b84 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x111d8b88 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x111d8b8c b #0x111d8bcc
super::chunk13::f_111d8bcc(cpu,rt,0x111d8bcc)?;return Ok(());
}
0x111d8b90=>{
// 0x111d8b90 mov w8, wzr
cpu.x[8]=(((0u64))) as u32 as u64;
// 0x111d8b94 cmp wzr, w20
{let a=((0u64));let b=((cpu.x[20] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d8b98 b.ge #0x111d8bb4
if cpu.condition(10){pc=0x111d8bb4;continue;}pc=0x111d8b9c;continue;
}
0x111d8b9c=>{
// 0x111d8b9c ldr x9, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111d8ba0 sub x10, x20, w8, sxtw
{let a=((cpu.x[20]));let b=(((cpu.x[8] as u32 as u64) as i32 as i64 as u64));let result=a.wrapping_sub(b);cpu.x[10]=(result) as u64;}
// 0x111d8ba4 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x111d8ba8 add x0, x9, w8, sxtw #2
{let a=((cpu.x[9]));let b=((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64;let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x111d8bac lsl x2, x10, #2
cpu.x[2]=(((((cpu.x[10]))) as u64).wrapping_shl(((((0x2u64))) & 63) as u32) as u64) as u64;
// 0x111d8bb0 bl #0x1187d480
cpu.x[30]=0x111d8bb4;super::chunk15::f_1187d480(cpu,rt,0x1187d480)?;
pc=0x111d8bb4;continue;
}
0x111d8bb4=>{
// 0x111d8bb4 str w20, [x21, #0xc]
{let address=(cpu.x[21]).wrapping_add(0xcu64);let v0=cpu.x[20] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111d8bb8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x111d8bbc ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d8bc0 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x111d8bc4 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x111d8bc8 b #0x111d8bcc
super::chunk13::f_111d8bcc(cpu,rt,0x111d8bcc)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111d8afc")),}}}
#[inline]
pub(super) unsafe fn f_111dc9e0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111dc9e0=>{
// 0x111dc9e0 movi v0.2d, #0000000000000000
cpu.q[0]=(lane_dup(0u128,64,2)) & 0xffffffffffffffffffffffffffffffffu128;
// 0x111dc9e4 str xzr, [x0, #0x28]
{let address=(cpu.x[0]).wrapping_add(0x28u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111dc9e8 str x1, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=cpu.x[1];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111dc9ec stur q0, [x0, #0x18]
{let address=(cpu.x[0]).wrapping_add(0x18u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x111dc9f0 stur q0, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x111dc9f4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111dc9e0")),}}}
#[inline]
pub(super) unsafe fn f_111dd6dc(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111dd6dc=>{
// 0x111dd6dc stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x111dd6e0 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111dd6e4 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111dd6e8 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x111dd6ec ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111dd6f0 mov x19, x2
cpu.x[19]=(((cpu.x[2]))) as u64;
// 0x111dd6f4 mov w20, w1
cpu.x[20]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x111dd6f8 ldr x8, [x8, #0xf0]
{let address=(cpu.x[8]).wrapping_add(0xf0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111dd6fc blr x8
{let target=cpu.x[8];cpu.x[30]=0x111dd700;super::dispatch(cpu,rt,target)?;}
pc=0x111dd700;continue;
}
0x111dd700=>{
// 0x111dd700 cbz w0, #0x111dd728
if (cpu.x[0] as u32 as u64) == 0 {pc=0x111dd728;continue;}pc=0x111dd704;continue;
}
0x111dd704=>{
// 0x111dd704 mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x111dd708 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x111dd70c mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x111dd710 bl #0x10e16538
cpu.x[30]=0x111dd714;super::chunk9::f_10e16538(cpu,rt,0x10e16538)?;
pc=0x111dd714;continue;
}
0x111dd714=>{
// 0x111dd714 cbz x0, #0x111dd73c
if (cpu.x[0]) == 0 {pc=0x111dd73c;continue;}pc=0x111dd718;continue;
}
0x111dd718=>{
// 0x111dd718 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111dd71c ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x111dd720 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x111dd724 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x111dd728=>{
// 0x111dd728 mov x0, xzr
cpu.x[0]=(((0u64))) as u64;
// 0x111dd72c ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111dd730 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x111dd734 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x111dd738 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x111dd73c=>{
// 0x111dd73c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x111dd740 mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x111dd744 mov w2, w20
cpu.x[2]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x111dd748 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111dd74c ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x111dd750 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x111dd754 b #0x10e16654
super::dispatch(cpu,rt,0x10e16654)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111dd6dc")),}}}
#[inline]
pub(super) unsafe fn f_111dd77c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111dd77c=>{
// 0x111dd77c stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x111dd780 stp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111dd784 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111dd788 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x111dd78c cbz x1, #0x111dd7f4
if (cpu.x[1]) == 0 {pc=0x111dd7f4;continue;}pc=0x111dd790;continue;
}
0x111dd790=>{
// 0x111dd790 ldrb w8, [x1, #0x38]
{let address=(cpu.x[1]).wrapping_add(0x38u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111dd794 mov x21, x3
cpu.x[21]=(((cpu.x[3]))) as u64;
// 0x111dd798 mov w20, w2
cpu.x[20]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x111dd79c mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x111dd7a0 mov x22, x0
cpu.x[22]=(((cpu.x[0]))) as u64;
// 0x111dd7a4 cbz w8, #0x111dd7b8
if (cpu.x[8] as u32 as u64) == 0 {pc=0x111dd7b8;continue;}pc=0x111dd7a8;continue;
}
0x111dd7a8=>{
// 0x111dd7a8 adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x111dd7ac add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x111dd7b0 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x111dd7b4 bl #0x1128df18
cpu.x[30]=0x111dd7b8;super::dispatch(cpu,rt,0x1128df18)?;
pc=0x111dd7b8;continue;
}
0x111dd7b8=>{
// 0x111dd7b8 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x111dd7bc mov w1, w20
cpu.x[1]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x111dd7c0 bl #0x10e1552c
cpu.x[30]=0x111dd7c4;super::chunk12::f_10e1552c(cpu,rt,0x10e1552c)?;
pc=0x111dd7c4;continue;
}
0x111dd7c4=>{
// 0x111dd7c4 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111dd7c8 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x111dd7cc ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111dd7d0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x111dd7d4;super::dispatch(cpu,rt,target)?;}
pc=0x111dd7d4;continue;
}
0x111dd7d4=>{
// 0x111dd7d4 mov w19, w0
cpu.x[19]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x111dd7d8 tbz w0, #0x1f, #0x111dd7ec
if ((cpu.x[0] as u32 as u64) & (1u64 << 31)) == 0 {pc=0x111dd7ec;continue;}pc=0x111dd7dc;continue;
}
0x111dd7dc=>{
// 0x111dd7dc adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x111dd7e0 add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x111dd7e4 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x111dd7e8 bl #0x1128df18
cpu.x[30]=0x111dd7ec;super::dispatch(cpu,rt,0x1128df18)?;
pc=0x111dd7ec;continue;
}
0x111dd7ec=>{
// 0x111dd7ec add w1, w19, #1
{let a=((cpu.x[19] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u32 as u64;}
// 0x111dd7f0 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
pc=0x111dd7f4;continue;
}
0x111dd7f4=>{
// 0x111dd7f4 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111dd7f8 ldr x2, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x111dd7fc ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
pc=0x111dd800;continue;
}
0x111dd800=>{
// 0x111dd800 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111dd804 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x111dd808 br x2
{let target=cpu.x[2];if target>=0x111dd77c && target<0x111dd80c{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111dd77c")),}}}
#[inline]
pub(super) unsafe fn f_1121f760(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x1121f760=>{
// 0x1121f760 add x0, x0, #0x18
{let a=((cpu.x[0]));let b=((0x18u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x1121f764 b #0x113bec58
super::chunk7::f_113bec58(cpu,rt,0x113bec58)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x1121f760")),}}}
#[inline]
pub(super) unsafe fn f_1128b714(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x1128b714=>{
// 0x1128b714 stp x29, x30, [sp, #-0x50]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffb0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x1128b718 str x25, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x1128b71c stp x24, x23, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x1128b720 stp x22, x21, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x1128b724 stp x20, x19, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x1128b728 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x1128b72c mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x1128b730 ldp x8, x1, [x1]
{let address=(cpu.x[1]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[8]=(v0) as u64;cpu.x[1]=(v1) as u64;}
// 0x1128b734 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x1128b738 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x1128b73c bl #0x11379154
cpu.x[30]=0x1128b740;super::chunk8::f_11379154(cpu,rt,0x11379154)?;
pc=0x1128b740;continue;
}
0x1128b740=>{
// 0x1128b740 ldr x25, [x21, #8]
{let address=(cpu.x[21]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[25]=(v0) as u64;}
// 0x1128b744 cbz x25, #0x1128b870
if (cpu.x[25]) == 0 {pc=0x1128b870;continue;}pc=0x1128b748;continue;
}
0x1128b748=>{
// 0x1128b748 fmov d0, x25
cpu.q[0]=(cpu.x[25]) as u128 & 0xffffffffffffffffu128;
// 0x1128b74c mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x1128b750 cnt v0.8b, v0.8b
cpu.q[0]=(lane_count((cpu.q[0] & 0xffffffffffffffffu128),8,8)) & 0xffffffffffffffffu128;
// 0x1128b754 uaddlv h0, v0.8b
cpu.q[0]=(lane_reduce((cpu.q[0] & 0xffffffffffffffffu128),8,8,"uaddlv")) & 0xffffu128;
// 0x1128b758 fmov w8, s0
cpu.x[8]=((cpu.q[0] & 0xffffffffu128) as u64) as u32 as u64;
// 0x1128b75c cmp x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x1128b760 b.hi #0x1128b770
if cpu.condition(8){pc=0x1128b770;continue;}pc=0x1128b764;continue;
}
0x1128b764=>{
// 0x1128b764 sub x9, x25, #1
{let a=((cpu.x[25]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u64;}
// 0x1128b768 and x24, x9, x20
{let a=((cpu.x[9]));let b=((cpu.x[20]));let result=a & b;cpu.x[24]=(result) as u64;}
// 0x1128b76c b #0x1128b784
pc=0x1128b784;continue;
}
0x1128b770=>{
// 0x1128b770 cmp x20, x25
{let a=((cpu.x[20]));let b=((cpu.x[25]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x1128b774 mov x24, x20
cpu.x[24]=(((cpu.x[20]))) as u64;
// 0x1128b778 b.lo #0x1128b784
if cpu.condition(3){pc=0x1128b784;continue;}pc=0x1128b77c;continue;
}
0x1128b77c=>{
// 0x1128b77c udiv x9, x20, x25
{let a=((cpu.x[20])) as u64;let b=((cpu.x[25])) as u64;cpu.x[9]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u64;}
// 0x1128b780 msub x24, x9, x25, x20
cpu.x[24]=((((cpu.x[20]))).wrapping_sub((((cpu.x[9]))).wrapping_mul(((cpu.x[25]))))) as u64;
pc=0x1128b784;continue;
}
0x1128b784=>{
// 0x1128b784 ldr x9, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x1128b788 ldr x9, [x9, x24, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[24])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x1128b78c cbz x9, #0x1128b870
if (cpu.x[9]) == 0 {pc=0x1128b870;continue;}pc=0x1128b790;continue;
}
0x1128b790=>{
// 0x1128b790 ldr x21, [x9]
{let address=(cpu.x[9]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x1128b794 cbz x21, #0x1128b874
if (cpu.x[21]) == 0 {pc=0x1128b874;continue;}pc=0x1128b798;continue;
}
0x1128b798=>{
// 0x1128b798 cmp x8, #2
{let a=((cpu.x[8]));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x1128b79c b.hs #0x1128b814
if cpu.condition(2){pc=0x1128b814;continue;}pc=0x1128b7a0;continue;
}
0x1128b7a0=>{
// 0x1128b7a0 sub x25, x25, #1
{let a=((cpu.x[25]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[25]=(result) as u64;}
// 0x1128b7a4 b #0x1128b7bc
pc=0x1128b7bc;continue;
}
0x1128b7a8=>{
// 0x1128b7a8 and x8, x8, x25
{let a=((cpu.x[8]));let b=((cpu.x[25]));let result=a & b;cpu.x[8]=(result) as u64;}
// 0x1128b7ac cmp x8, x24
{let a=((cpu.x[8]));let b=((cpu.x[24]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x1128b7b0 b.ne #0x1128b870
if cpu.condition(1){pc=0x1128b870;continue;}pc=0x1128b7b4;continue;
}
0x1128b7b4=>{
// 0x1128b7b4 ldr x21, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x1128b7b8 cbz x21, #0x1128b874
if (cpu.x[21]) == 0 {pc=0x1128b874;continue;}pc=0x1128b7bc;continue;
}
0x1128b7bc=>{
// 0x1128b7bc ldr x8, [x21, #8]
{let address=(cpu.x[21]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x1128b7c0 cmp x20, x8
{let a=((cpu.x[20]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x1128b7c4 b.ne #0x1128b7a8
if cpu.condition(1){pc=0x1128b7a8;continue;}pc=0x1128b7c8;continue;
}
0x1128b7c8=>{
// 0x1128b7c8 ldp x22, x23, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x1128b7cc add x0, x21, #0x10
{let a=((cpu.x[21]));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x1128b7d0 bl #0x1142effc
cpu.x[30]=0x1128b7d4;super::chunk3::f_1142effc(cpu,rt,0x1142effc)?;
// 0x1128b7d4 cmp x23, w0, sxtw
{let a=((cpu.x[23]));let b=(((cpu.x[0] as u32 as u64) as i32 as i64 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x1128b7d8 b.ne #0x1128b7b4
if cpu.condition(1){pc=0x1128b7b4;continue;}pc=0x1128b7dc;continue;
}
0x1128b7dc=>{
// 0x1128b7dc mov x8, x21
cpu.x[8]=(((cpu.x[21]))) as u64;
// 0x1128b7e0 mov x1, x22
cpu.x[1]=(((cpu.x[22]))) as u64;
// 0x1128b7e4 mov x2, x23
cpu.x[2]=(((cpu.x[23]))) as u64;
// 0x1128b7e8 ldr x9, [x8, #0x18]!
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;cpu.x[8]=(address) as u64;}
// 0x1128b7ec ldur w10, [x8, #-4]
{let address=(cpu.x[8]).wrapping_add(0xfffffffffffffffcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x1128b7f0 cmp w10, #8
{let a=((cpu.x[10] as u32 as u64));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x1128b7f4 csel x0, x8, x9, lt
cpu.x[0]=(if cpu.condition(11){((cpu.x[8]))}else{((cpu.x[9]))}) as u64;
// 0x1128b7f8 bl #0x1187d490
cpu.x[30]=0x1128b7fc;super::dispatch(cpu,rt,0x1187d490)?;
// 0x1128b7fc cbnz w0, #0x1128b7b4
if (cpu.x[0] as u32 as u64) != 0 {pc=0x1128b7b4;continue;}pc=0x1128b800;continue;
}
0x1128b800=>{
// 0x1128b800 b #0x1128b874
pc=0x1128b874;continue;
}
0x1128b804=>{
// 0x1128b804 cmp x8, x24
{let a=((cpu.x[8]));let b=((cpu.x[24]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x1128b808 b.ne #0x1128b870
if cpu.condition(1){pc=0x1128b870;continue;}pc=0x1128b80c;continue;
}
0x1128b80c=>{
// 0x1128b80c ldr x21, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x1128b810 cbz x21, #0x1128b874
if (cpu.x[21]) == 0 {pc=0x1128b874;continue;}pc=0x1128b814;continue;
}
0x1128b814=>{
// 0x1128b814 ldr x8, [x21, #8]
{let address=(cpu.x[21]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x1128b818 cmp x20, x8
{let a=((cpu.x[20]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x1128b81c b.ne #0x1128b85c
if cpu.condition(1){pc=0x1128b85c;continue;}pc=0x1128b820;continue;
}
0x1128b820=>{
// 0x1128b820 ldp x22, x23, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x1128b824 add x0, x21, #0x10
{let a=((cpu.x[21]));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x1128b828 bl #0x1142effc
cpu.x[30]=0x1128b82c;super::chunk3::f_1142effc(cpu,rt,0x1142effc)?;
pc=0x1128b82c;continue;
}
0x1128b82c=>{
// 0x1128b82c cmp x23, w0, sxtw
{let a=((cpu.x[23]));let b=(((cpu.x[0] as u32 as u64) as i32 as i64 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x1128b830 b.ne #0x1128b80c
if cpu.condition(1){pc=0x1128b80c;continue;}pc=0x1128b834;continue;
}
0x1128b834=>{
// 0x1128b834 mov x8, x21
cpu.x[8]=(((cpu.x[21]))) as u64;
// 0x1128b838 mov x1, x22
cpu.x[1]=(((cpu.x[22]))) as u64;
// 0x1128b83c mov x2, x23
cpu.x[2]=(((cpu.x[23]))) as u64;
// 0x1128b840 ldr x9, [x8, #0x18]!
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;cpu.x[8]=(address) as u64;}
// 0x1128b844 ldur w10, [x8, #-4]
{let address=(cpu.x[8]).wrapping_add(0xfffffffffffffffcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x1128b848 cmp w10, #8
{let a=((cpu.x[10] as u32 as u64));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x1128b84c csel x0, x8, x9, lt
cpu.x[0]=(if cpu.condition(11){((cpu.x[8]))}else{((cpu.x[9]))}) as u64;
// 0x1128b850 bl #0x1187d490
cpu.x[30]=0x1128b854;super::dispatch(cpu,rt,0x1187d490)?;
pc=0x1128b854;continue;
}
0x1128b854=>{
// 0x1128b854 cbnz w0, #0x1128b80c
if (cpu.x[0] as u32 as u64) != 0 {pc=0x1128b80c;continue;}pc=0x1128b858;continue;
}
0x1128b858=>{
// 0x1128b858 b #0x1128b874
pc=0x1128b874;continue;
}
0x1128b85c=>{
// 0x1128b85c cmp x8, x25
{let a=((cpu.x[8]));let b=((cpu.x[25]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x1128b860 b.lo #0x1128b804
if cpu.condition(3){pc=0x1128b804;continue;}pc=0x1128b864;continue;
}
0x1128b864=>{
// 0x1128b864 udiv x9, x8, x25
{let a=((cpu.x[8])) as u64;let b=((cpu.x[25])) as u64;cpu.x[9]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u64;}
// 0x1128b868 msub x8, x9, x25, x8
cpu.x[8]=((((cpu.x[8]))).wrapping_sub((((cpu.x[9]))).wrapping_mul(((cpu.x[25]))))) as u64;
// 0x1128b86c b #0x1128b804
pc=0x1128b804;continue;
}
0x1128b870=>{
// 0x1128b870 mov x21, xzr
cpu.x[21]=(((0u64))) as u64;
pc=0x1128b874;continue;
}
0x1128b874=>{
// 0x1128b874 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x1128b878 ldp x20, x19, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x1128b87c ldr x25, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[25]=(v0) as u64;}
// 0x1128b880 ldp x22, x21, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x1128b884 ldp x24, x23, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x1128b888 ldp x29, x30, [sp], #0x50
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x50u64)))) as u64;}
// 0x1128b88c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x1128b714")),}}}
#[inline]
pub(super) unsafe fn f_1128bba8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x1128bba8=>{
// 0x1128bba8 str wzr, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128bbac str xzr, [x0, #0x20]
{let address=(cpu.x[0]).wrapping_add(0x20u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x1128bbb0 str wzr, [x0, #0x1c]
{let address=(cpu.x[0]).wrapping_add(0x1cu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128bbb4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x1128bba8")),}}}
#[inline]
pub(super) unsafe fn f_1128d1e4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x1128d1e4=>{
// 0x1128d1e4 ldrsw x10, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[10]=(v0) as u64;}
// 0x1128d1e8 ldr x9, [x0, #0x38]
{let address=(cpu.x[0]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x1128d1ec str wzr, [x0, #0x1c]
{let address=(cpu.x[0]).wrapping_add(0x1cu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128d1f0 add x11, x10, #1
{let a=((cpu.x[10]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
// 0x1128d1f4 str w11, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[11] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128d1f8 ldrb w12, [x9, x10]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[10])));let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[12]=(v0) as u32 as u64;}
// 0x1128d1fc tbnz w12, #6, #0x1128d210
if ((cpu.x[12] as u32 as u64) & (1u64 << 6)) != 0 {{// 0x1128d210 tbnz w12, #7, #0x1128d270
if ((cpu.x[12] as u32 as u64) & (1u64 << 7)) != 0 {{// 0x1128d270 add x13, x10, #2
{let a=((cpu.x[10]));let b=((0x2u64));let result=a.wrapping_add(b);cpu.x[13]=(result) as u64;}
// 0x1128d274 and w8, w12, #0x3f
{let a=((cpu.x[12] as u32 as u64));let b=((0x3fu64));let result=a & b;cpu.x[8]=(result) as u32 as u64;}
// 0x1128d278 str w13, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[13] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128d27c ldrsb w11, [x9, x11]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[11])));let v0=rt.memory.read::<i8>(address.wrapping_add(0)) as i64 as u64;cpu.x[11]=(v0) as u32 as u64;}
// 0x1128d280 bfi w8, w11, #6, #7
cpu.x[8]=((cpu.x[8] as u32 as u64 & !0x1fc0u64) | ((((cpu.x[11] as u32 as u64)) & 0x7fu64) << 6)) as u32 as u64;
// 0x1128d284 tbnz w11, #7, #0x1128d290
if ((cpu.x[11] as u32 as u64) & (1u64 << 7)) != 0 {{// 0x1128d290 add x12, x10, #3
{let a=((cpu.x[10]));let b=((0x3u64));let result=a.wrapping_add(b);cpu.x[12]=(result) as u64;}
// 0x1128d294 str w12, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[12] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128d298 ldrsb w11, [x9, x13]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[13])));let v0=rt.memory.read::<i8>(address.wrapping_add(0)) as i64 as u64;cpu.x[11]=(v0) as u32 as u64;}
// 0x1128d29c bfi w8, w11, #0xd, #7
cpu.x[8]=((cpu.x[8] as u32 as u64 & !0xfe000u64) | ((((cpu.x[11] as u32 as u64)) & 0x7fu64) << 13)) as u32 as u64;
// 0x1128d2a0 tbnz w11, #7, #0x1128d2ac
if ((cpu.x[11] as u32 as u64) & (1u64 << 7)) != 0 {{// 0x1128d2ac add x11, x10, #4
{let a=((cpu.x[10]));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
// 0x1128d2b0 str w11, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[11] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128d2b4 ldrsb w12, [x9, x12]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[12])));let v0=rt.memory.read::<i8>(address.wrapping_add(0)) as i64 as u64;cpu.x[12]=(v0) as u32 as u64;}
// 0x1128d2b8 bfi w8, w12, #0x14, #7
cpu.x[8]=((cpu.x[8] as u32 as u64 & !0x7f00000u64) | ((((cpu.x[12] as u32 as u64)) & 0x7fu64) << 20)) as u32 as u64;
// 0x1128d2bc tbnz w12, #7, #0x1128d2c8
if ((cpu.x[12] as u32 as u64) & (1u64 << 7)) != 0 {{// 0x1128d2c8 add w10, w10, #5
{let a=((cpu.x[10] as u32 as u64));let b=((0x5u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x1128d2cc str w10, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128d2d0 ldrb w9, [x9, x11]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[11])));let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x1128d2d4 orr w8, w8, w9, lsl #27
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[9] as u32 as u64)) as u32).wrapping_shl(27) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x1128d2d8 orr w0, w8, #0x80000000
{let a=((cpu.x[8] as u32 as u64));let b=((0x80000000u64));let result=a | b;cpu.x[0]=(result) as u32 as u64;}
// 0x1128d2dc ret
cpu.pc=cpu.x[30];return Ok(());}}{// 0x1128d2c0 orr w0, w8, #0xf8000000
{let a=((cpu.x[8] as u32 as u64));let b=((0xf8000000u64));let result=a | b;cpu.x[0]=(result) as u32 as u64;}
// 0x1128d2c4 ret
cpu.pc=cpu.x[30];return Ok(());}}}{// 0x1128d2a4 orr w0, w8, #0xfff00000
{let a=((cpu.x[8] as u32 as u64));let b=((0xfff00000u64));let result=a | b;cpu.x[0]=(result) as u32 as u64;}
// 0x1128d2a8 ret
cpu.pc=cpu.x[30];return Ok(());}}}{// 0x1128d288 orr w0, w8, #0xffffe000
{let a=((cpu.x[8] as u32 as u64));let b=((0xffffe000u64));let result=a | b;cpu.x[0]=(result) as u32 as u64;}
// 0x1128d28c ret
cpu.pc=cpu.x[30];return Ok(());}}}{// 0x1128d214 orr w8, w12, #0xffffffc0
{let a=((cpu.x[12] as u32 as u64));let b=((0xffffffc0u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x1128d218 mov w0, w8
cpu.x[0]=(((cpu.x[8] as u32 as u64))) as u32 as u64;
// 0x1128d21c ret
cpu.pc=cpu.x[30];return Ok(());}}}{// 0x1128d200 and w8, w12, #0x3f
{let a=((cpu.x[12] as u32 as u64));let b=((0x3fu64));let result=a & b;cpu.x[8]=(result) as u32 as u64;}
// 0x1128d204 tbnz w12, #7, #0x1128d220
if ((cpu.x[12] as u32 as u64) & (1u64 << 7)) != 0 {{// 0x1128d220 add x12, x10, #2
{let a=((cpu.x[10]));let b=((0x2u64));let result=a.wrapping_add(b);cpu.x[12]=(result) as u64;}
// 0x1128d224 str w12, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[12] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128d228 ldrsb w11, [x9, x11]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[11])));let v0=rt.memory.read::<i8>(address.wrapping_add(0)) as i64 as u64;cpu.x[11]=(v0) as u32 as u64;}
// 0x1128d22c bfi w8, w11, #6, #7
cpu.x[8]=((cpu.x[8] as u32 as u64 & !0x1fc0u64) | ((((cpu.x[11] as u32 as u64)) & 0x7fu64) << 6)) as u32 as u64;
// 0x1128d230 tbz w11, #7, #0x1128d208
if ((cpu.x[11] as u32 as u64) & (1u64 << 7)) == 0 {pc=0x1128d208;continue;}{// 0x1128d234 add x13, x10, #3
{let a=((cpu.x[10]));let b=((0x3u64));let result=a.wrapping_add(b);cpu.x[13]=(result) as u64;}
// 0x1128d238 str w13, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[13] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128d23c ldrsb w11, [x9, x12]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[12])));let v0=rt.memory.read::<i8>(address.wrapping_add(0)) as i64 as u64;cpu.x[11]=(v0) as u32 as u64;}
// 0x1128d240 bfi w8, w11, #0xd, #7
cpu.x[8]=((cpu.x[8] as u32 as u64 & !0xfe000u64) | ((((cpu.x[11] as u32 as u64)) & 0x7fu64) << 13)) as u32 as u64;
// 0x1128d244 tbz w11, #7, #0x1128d208
if ((cpu.x[11] as u32 as u64) & (1u64 << 7)) == 0 {pc=0x1128d208;continue;}{// 0x1128d248 add x11, x10, #4
{let a=((cpu.x[10]));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
// 0x1128d24c str w11, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[11] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128d250 ldrsb w12, [x9, x13]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[13])));let v0=rt.memory.read::<i8>(address.wrapping_add(0)) as i64 as u64;cpu.x[12]=(v0) as u32 as u64;}
// 0x1128d254 bfi w8, w12, #0x14, #7
cpu.x[8]=((cpu.x[8] as u32 as u64 & !0x7f00000u64) | ((((cpu.x[12] as u32 as u64)) & 0x7fu64) << 20)) as u32 as u64;
// 0x1128d258 tbz w12, #7, #0x1128d208
if ((cpu.x[12] as u32 as u64) & (1u64 << 7)) == 0 {pc=0x1128d208;continue;}{// 0x1128d25c add w10, w10, #5
{let a=((cpu.x[10] as u32 as u64));let b=((0x5u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x1128d260 str w10, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128d264 ldrb w9, [x9, x11]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[11])));let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x1128d268 orr w0, w8, w9, lsl #27
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[9] as u32 as u64)) as u32).wrapping_shl(27) as u64;let result=a | b;cpu.x[0]=(result) as u32 as u64;}
// 0x1128d26c ret
cpu.pc=cpu.x[30];return Ok(());}}}}}{// 0x1128d208 mov w0, w8
cpu.x[0]=(((cpu.x[8] as u32 as u64))) as u32 as u64;
// 0x1128d20c ret
cpu.pc=cpu.x[30];return Ok(());}}
}
0x1128d200=>{
// 0x1128d200 and w8, w12, #0x3f
{let a=((cpu.x[12] as u32 as u64));let b=((0x3fu64));let result=a & b;cpu.x[8]=(result) as u32 as u64;}
// 0x1128d204 tbnz w12, #7, #0x1128d220
if ((cpu.x[12] as u32 as u64) & (1u64 << 7)) != 0 {{// 0x1128d220 add x12, x10, #2
{let a=((cpu.x[10]));let b=((0x2u64));let result=a.wrapping_add(b);cpu.x[12]=(result) as u64;}
// 0x1128d224 str w12, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[12] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128d228 ldrsb w11, [x9, x11]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[11])));let v0=rt.memory.read::<i8>(address.wrapping_add(0)) as i64 as u64;cpu.x[11]=(v0) as u32 as u64;}
// 0x1128d22c bfi w8, w11, #6, #7
cpu.x[8]=((cpu.x[8] as u32 as u64 & !0x1fc0u64) | ((((cpu.x[11] as u32 as u64)) & 0x7fu64) << 6)) as u32 as u64;
// 0x1128d230 tbz w11, #7, #0x1128d208
if ((cpu.x[11] as u32 as u64) & (1u64 << 7)) == 0 {pc=0x1128d208;continue;}{// 0x1128d234 add x13, x10, #3
{let a=((cpu.x[10]));let b=((0x3u64));let result=a.wrapping_add(b);cpu.x[13]=(result) as u64;}
// 0x1128d238 str w13, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[13] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128d23c ldrsb w11, [x9, x12]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[12])));let v0=rt.memory.read::<i8>(address.wrapping_add(0)) as i64 as u64;cpu.x[11]=(v0) as u32 as u64;}
// 0x1128d240 bfi w8, w11, #0xd, #7
cpu.x[8]=((cpu.x[8] as u32 as u64 & !0xfe000u64) | ((((cpu.x[11] as u32 as u64)) & 0x7fu64) << 13)) as u32 as u64;
// 0x1128d244 tbz w11, #7, #0x1128d208
if ((cpu.x[11] as u32 as u64) & (1u64 << 7)) == 0 {pc=0x1128d208;continue;}{// 0x1128d248 add x11, x10, #4
{let a=((cpu.x[10]));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
// 0x1128d24c str w11, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[11] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128d250 ldrsb w12, [x9, x13]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[13])));let v0=rt.memory.read::<i8>(address.wrapping_add(0)) as i64 as u64;cpu.x[12]=(v0) as u32 as u64;}
// 0x1128d254 bfi w8, w12, #0x14, #7
cpu.x[8]=((cpu.x[8] as u32 as u64 & !0x7f00000u64) | ((((cpu.x[12] as u32 as u64)) & 0x7fu64) << 20)) as u32 as u64;
// 0x1128d258 tbz w12, #7, #0x1128d208
if ((cpu.x[12] as u32 as u64) & (1u64 << 7)) == 0 {pc=0x1128d208;continue;}{// 0x1128d25c add w10, w10, #5
{let a=((cpu.x[10] as u32 as u64));let b=((0x5u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x1128d260 str w10, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128d264 ldrb w9, [x9, x11]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[11])));let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x1128d268 orr w0, w8, w9, lsl #27
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[9] as u32 as u64)) as u32).wrapping_shl(27) as u64;let result=a | b;cpu.x[0]=(result) as u32 as u64;}
// 0x1128d26c ret
cpu.pc=cpu.x[30];return Ok(());}}}}}{// 0x1128d208 mov w0, w8
cpu.x[0]=(((cpu.x[8] as u32 as u64))) as u32 as u64;
// 0x1128d20c ret
cpu.pc=cpu.x[30];return Ok(());}
}
0x1128d208=>{
// 0x1128d208 mov w0, w8
cpu.x[0]=(((cpu.x[8] as u32 as u64))) as u32 as u64;
// 0x1128d20c ret
cpu.pc=cpu.x[30];return Ok(());
}
0x1128d210=>{
// 0x1128d210 tbnz w12, #7, #0x1128d270
if ((cpu.x[12] as u32 as u64) & (1u64 << 7)) != 0 {{// 0x1128d270 add x13, x10, #2
{let a=((cpu.x[10]));let b=((0x2u64));let result=a.wrapping_add(b);cpu.x[13]=(result) as u64;}
// 0x1128d274 and w8, w12, #0x3f
{let a=((cpu.x[12] as u32 as u64));let b=((0x3fu64));let result=a & b;cpu.x[8]=(result) as u32 as u64;}
// 0x1128d278 str w13, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[13] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128d27c ldrsb w11, [x9, x11]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[11])));let v0=rt.memory.read::<i8>(address.wrapping_add(0)) as i64 as u64;cpu.x[11]=(v0) as u32 as u64;}
// 0x1128d280 bfi w8, w11, #6, #7
cpu.x[8]=((cpu.x[8] as u32 as u64 & !0x1fc0u64) | ((((cpu.x[11] as u32 as u64)) & 0x7fu64) << 6)) as u32 as u64;
// 0x1128d284 tbnz w11, #7, #0x1128d290
if ((cpu.x[11] as u32 as u64) & (1u64 << 7)) != 0 {{// 0x1128d290 add x12, x10, #3
{let a=((cpu.x[10]));let b=((0x3u64));let result=a.wrapping_add(b);cpu.x[12]=(result) as u64;}
// 0x1128d294 str w12, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[12] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128d298 ldrsb w11, [x9, x13]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[13])));let v0=rt.memory.read::<i8>(address.wrapping_add(0)) as i64 as u64;cpu.x[11]=(v0) as u32 as u64;}
// 0x1128d29c bfi w8, w11, #0xd, #7
cpu.x[8]=((cpu.x[8] as u32 as u64 & !0xfe000u64) | ((((cpu.x[11] as u32 as u64)) & 0x7fu64) << 13)) as u32 as u64;
// 0x1128d2a0 tbnz w11, #7, #0x1128d2ac
if ((cpu.x[11] as u32 as u64) & (1u64 << 7)) != 0 {{// 0x1128d2ac add x11, x10, #4
{let a=((cpu.x[10]));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
// 0x1128d2b0 str w11, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[11] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128d2b4 ldrsb w12, [x9, x12]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[12])));let v0=rt.memory.read::<i8>(address.wrapping_add(0)) as i64 as u64;cpu.x[12]=(v0) as u32 as u64;}
// 0x1128d2b8 bfi w8, w12, #0x14, #7
cpu.x[8]=((cpu.x[8] as u32 as u64 & !0x7f00000u64) | ((((cpu.x[12] as u32 as u64)) & 0x7fu64) << 20)) as u32 as u64;
// 0x1128d2bc tbnz w12, #7, #0x1128d2c8
if ((cpu.x[12] as u32 as u64) & (1u64 << 7)) != 0 {{// 0x1128d2c8 add w10, w10, #5
{let a=((cpu.x[10] as u32 as u64));let b=((0x5u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x1128d2cc str w10, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128d2d0 ldrb w9, [x9, x11]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[11])));let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x1128d2d4 orr w8, w8, w9, lsl #27
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[9] as u32 as u64)) as u32).wrapping_shl(27) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x1128d2d8 orr w0, w8, #0x80000000
{let a=((cpu.x[8] as u32 as u64));let b=((0x80000000u64));let result=a | b;cpu.x[0]=(result) as u32 as u64;}
// 0x1128d2dc ret
cpu.pc=cpu.x[30];return Ok(());}}{// 0x1128d2c0 orr w0, w8, #0xf8000000
{let a=((cpu.x[8] as u32 as u64));let b=((0xf8000000u64));let result=a | b;cpu.x[0]=(result) as u32 as u64;}
// 0x1128d2c4 ret
cpu.pc=cpu.x[30];return Ok(());}}}{// 0x1128d2a4 orr w0, w8, #0xfff00000
{let a=((cpu.x[8] as u32 as u64));let b=((0xfff00000u64));let result=a | b;cpu.x[0]=(result) as u32 as u64;}
// 0x1128d2a8 ret
cpu.pc=cpu.x[30];return Ok(());}}}{// 0x1128d288 orr w0, w8, #0xffffe000
{let a=((cpu.x[8] as u32 as u64));let b=((0xffffe000u64));let result=a | b;cpu.x[0]=(result) as u32 as u64;}
// 0x1128d28c ret
cpu.pc=cpu.x[30];return Ok(());}}}{// 0x1128d214 orr w8, w12, #0xffffffc0
{let a=((cpu.x[12] as u32 as u64));let b=((0xffffffc0u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x1128d218 mov w0, w8
cpu.x[0]=(((cpu.x[8] as u32 as u64))) as u32 as u64;
// 0x1128d21c ret
cpu.pc=cpu.x[30];return Ok(());}
}
0x1128d214=>{
// 0x1128d214 orr w8, w12, #0xffffffc0
{let a=((cpu.x[12] as u32 as u64));let b=((0xffffffc0u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x1128d218 mov w0, w8
cpu.x[0]=(((cpu.x[8] as u32 as u64))) as u32 as u64;
// 0x1128d21c ret
cpu.pc=cpu.x[30];return Ok(());
}
0x1128d220=>{
// 0x1128d220 add x12, x10, #2
{let a=((cpu.x[10]));let b=((0x2u64));let result=a.wrapping_add(b);cpu.x[12]=(result) as u64;}
// 0x1128d224 str w12, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[12] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128d228 ldrsb w11, [x9, x11]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[11])));let v0=rt.memory.read::<i8>(address.wrapping_add(0)) as i64 as u64;cpu.x[11]=(v0) as u32 as u64;}
// 0x1128d22c bfi w8, w11, #6, #7
cpu.x[8]=((cpu.x[8] as u32 as u64 & !0x1fc0u64) | ((((cpu.x[11] as u32 as u64)) & 0x7fu64) << 6)) as u32 as u64;
// 0x1128d230 tbz w11, #7, #0x1128d208
if ((cpu.x[11] as u32 as u64) & (1u64 << 7)) == 0 {pc=0x1128d208;continue;}{// 0x1128d234 add x13, x10, #3
{let a=((cpu.x[10]));let b=((0x3u64));let result=a.wrapping_add(b);cpu.x[13]=(result) as u64;}
// 0x1128d238 str w13, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[13] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128d23c ldrsb w11, [x9, x12]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[12])));let v0=rt.memory.read::<i8>(address.wrapping_add(0)) as i64 as u64;cpu.x[11]=(v0) as u32 as u64;}
// 0x1128d240 bfi w8, w11, #0xd, #7
cpu.x[8]=((cpu.x[8] as u32 as u64 & !0xfe000u64) | ((((cpu.x[11] as u32 as u64)) & 0x7fu64) << 13)) as u32 as u64;
// 0x1128d244 tbz w11, #7, #0x1128d208
if ((cpu.x[11] as u32 as u64) & (1u64 << 7)) == 0 {pc=0x1128d208;continue;}{// 0x1128d248 add x11, x10, #4
{let a=((cpu.x[10]));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
// 0x1128d24c str w11, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[11] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128d250 ldrsb w12, [x9, x13]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[13])));let v0=rt.memory.read::<i8>(address.wrapping_add(0)) as i64 as u64;cpu.x[12]=(v0) as u32 as u64;}
// 0x1128d254 bfi w8, w12, #0x14, #7
cpu.x[8]=((cpu.x[8] as u32 as u64 & !0x7f00000u64) | ((((cpu.x[12] as u32 as u64)) & 0x7fu64) << 20)) as u32 as u64;
// 0x1128d258 tbz w12, #7, #0x1128d208
if ((cpu.x[12] as u32 as u64) & (1u64 << 7)) == 0 {pc=0x1128d208;continue;}{// 0x1128d25c add w10, w10, #5
{let a=((cpu.x[10] as u32 as u64));let b=((0x5u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x1128d260 str w10, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128d264 ldrb w9, [x9, x11]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[11])));let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x1128d268 orr w0, w8, w9, lsl #27
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[9] as u32 as u64)) as u32).wrapping_shl(27) as u64;let result=a | b;cpu.x[0]=(result) as u32 as u64;}
// 0x1128d26c ret
cpu.pc=cpu.x[30];return Ok(());}}}
}
0x1128d234=>{
// 0x1128d234 add x13, x10, #3
{let a=((cpu.x[10]));let b=((0x3u64));let result=a.wrapping_add(b);cpu.x[13]=(result) as u64;}
// 0x1128d238 str w13, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[13] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128d23c ldrsb w11, [x9, x12]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[12])));let v0=rt.memory.read::<i8>(address.wrapping_add(0)) as i64 as u64;cpu.x[11]=(v0) as u32 as u64;}
// 0x1128d240 bfi w8, w11, #0xd, #7
cpu.x[8]=((cpu.x[8] as u32 as u64 & !0xfe000u64) | ((((cpu.x[11] as u32 as u64)) & 0x7fu64) << 13)) as u32 as u64;
// 0x1128d244 tbz w11, #7, #0x1128d208
if ((cpu.x[11] as u32 as u64) & (1u64 << 7)) == 0 {pc=0x1128d208;continue;}{// 0x1128d248 add x11, x10, #4
{let a=((cpu.x[10]));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
// 0x1128d24c str w11, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[11] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128d250 ldrsb w12, [x9, x13]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[13])));let v0=rt.memory.read::<i8>(address.wrapping_add(0)) as i64 as u64;cpu.x[12]=(v0) as u32 as u64;}
// 0x1128d254 bfi w8, w12, #0x14, #7
cpu.x[8]=((cpu.x[8] as u32 as u64 & !0x7f00000u64) | ((((cpu.x[12] as u32 as u64)) & 0x7fu64) << 20)) as u32 as u64;
// 0x1128d258 tbz w12, #7, #0x1128d208
if ((cpu.x[12] as u32 as u64) & (1u64 << 7)) == 0 {pc=0x1128d208;continue;}{// 0x1128d25c add w10, w10, #5
{let a=((cpu.x[10] as u32 as u64));let b=((0x5u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x1128d260 str w10, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128d264 ldrb w9, [x9, x11]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[11])));let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x1128d268 orr w0, w8, w9, lsl #27
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[9] as u32 as u64)) as u32).wrapping_shl(27) as u64;let result=a | b;cpu.x[0]=(result) as u32 as u64;}
// 0x1128d26c ret
cpu.pc=cpu.x[30];return Ok(());}}
}
0x1128d248=>{
// 0x1128d248 add x11, x10, #4
{let a=((cpu.x[10]));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
// 0x1128d24c str w11, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[11] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128d250 ldrsb w12, [x9, x13]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[13])));let v0=rt.memory.read::<i8>(address.wrapping_add(0)) as i64 as u64;cpu.x[12]=(v0) as u32 as u64;}
// 0x1128d254 bfi w8, w12, #0x14, #7
cpu.x[8]=((cpu.x[8] as u32 as u64 & !0x7f00000u64) | ((((cpu.x[12] as u32 as u64)) & 0x7fu64) << 20)) as u32 as u64;
// 0x1128d258 tbz w12, #7, #0x1128d208
if ((cpu.x[12] as u32 as u64) & (1u64 << 7)) == 0 {pc=0x1128d208;continue;}{// 0x1128d25c add w10, w10, #5
{let a=((cpu.x[10] as u32 as u64));let b=((0x5u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x1128d260 str w10, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128d264 ldrb w9, [x9, x11]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[11])));let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x1128d268 orr w0, w8, w9, lsl #27
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[9] as u32 as u64)) as u32).wrapping_shl(27) as u64;let result=a | b;cpu.x[0]=(result) as u32 as u64;}
// 0x1128d26c ret
cpu.pc=cpu.x[30];return Ok(());}
}
0x1128d25c=>{
// 0x1128d25c add w10, w10, #5
{let a=((cpu.x[10] as u32 as u64));let b=((0x5u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x1128d260 str w10, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128d264 ldrb w9, [x9, x11]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[11])));let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x1128d268 orr w0, w8, w9, lsl #27
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[9] as u32 as u64)) as u32).wrapping_shl(27) as u64;let result=a | b;cpu.x[0]=(result) as u32 as u64;}
// 0x1128d26c ret
cpu.pc=cpu.x[30];return Ok(());
}
0x1128d270=>{
// 0x1128d270 add x13, x10, #2
{let a=((cpu.x[10]));let b=((0x2u64));let result=a.wrapping_add(b);cpu.x[13]=(result) as u64;}
// 0x1128d274 and w8, w12, #0x3f
{let a=((cpu.x[12] as u32 as u64));let b=((0x3fu64));let result=a & b;cpu.x[8]=(result) as u32 as u64;}
// 0x1128d278 str w13, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[13] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128d27c ldrsb w11, [x9, x11]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[11])));let v0=rt.memory.read::<i8>(address.wrapping_add(0)) as i64 as u64;cpu.x[11]=(v0) as u32 as u64;}
// 0x1128d280 bfi w8, w11, #6, #7
cpu.x[8]=((cpu.x[8] as u32 as u64 & !0x1fc0u64) | ((((cpu.x[11] as u32 as u64)) & 0x7fu64) << 6)) as u32 as u64;
// 0x1128d284 tbnz w11, #7, #0x1128d290
if ((cpu.x[11] as u32 as u64) & (1u64 << 7)) != 0 {{// 0x1128d290 add x12, x10, #3
{let a=((cpu.x[10]));let b=((0x3u64));let result=a.wrapping_add(b);cpu.x[12]=(result) as u64;}
// 0x1128d294 str w12, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[12] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128d298 ldrsb w11, [x9, x13]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[13])));let v0=rt.memory.read::<i8>(address.wrapping_add(0)) as i64 as u64;cpu.x[11]=(v0) as u32 as u64;}
// 0x1128d29c bfi w8, w11, #0xd, #7
cpu.x[8]=((cpu.x[8] as u32 as u64 & !0xfe000u64) | ((((cpu.x[11] as u32 as u64)) & 0x7fu64) << 13)) as u32 as u64;
// 0x1128d2a0 tbnz w11, #7, #0x1128d2ac
if ((cpu.x[11] as u32 as u64) & (1u64 << 7)) != 0 {{// 0x1128d2ac add x11, x10, #4
{let a=((cpu.x[10]));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
// 0x1128d2b0 str w11, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[11] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128d2b4 ldrsb w12, [x9, x12]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[12])));let v0=rt.memory.read::<i8>(address.wrapping_add(0)) as i64 as u64;cpu.x[12]=(v0) as u32 as u64;}
// 0x1128d2b8 bfi w8, w12, #0x14, #7
cpu.x[8]=((cpu.x[8] as u32 as u64 & !0x7f00000u64) | ((((cpu.x[12] as u32 as u64)) & 0x7fu64) << 20)) as u32 as u64;
// 0x1128d2bc tbnz w12, #7, #0x1128d2c8
if ((cpu.x[12] as u32 as u64) & (1u64 << 7)) != 0 {{// 0x1128d2c8 add w10, w10, #5
{let a=((cpu.x[10] as u32 as u64));let b=((0x5u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x1128d2cc str w10, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128d2d0 ldrb w9, [x9, x11]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[11])));let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x1128d2d4 orr w8, w8, w9, lsl #27
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[9] as u32 as u64)) as u32).wrapping_shl(27) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x1128d2d8 orr w0, w8, #0x80000000
{let a=((cpu.x[8] as u32 as u64));let b=((0x80000000u64));let result=a | b;cpu.x[0]=(result) as u32 as u64;}
// 0x1128d2dc ret
cpu.pc=cpu.x[30];return Ok(());}}{// 0x1128d2c0 orr w0, w8, #0xf8000000
{let a=((cpu.x[8] as u32 as u64));let b=((0xf8000000u64));let result=a | b;cpu.x[0]=(result) as u32 as u64;}
// 0x1128d2c4 ret
cpu.pc=cpu.x[30];return Ok(());}}}{// 0x1128d2a4 orr w0, w8, #0xfff00000
{let a=((cpu.x[8] as u32 as u64));let b=((0xfff00000u64));let result=a | b;cpu.x[0]=(result) as u32 as u64;}
// 0x1128d2a8 ret
cpu.pc=cpu.x[30];return Ok(());}}}{// 0x1128d288 orr w0, w8, #0xffffe000
{let a=((cpu.x[8] as u32 as u64));let b=((0xffffe000u64));let result=a | b;cpu.x[0]=(result) as u32 as u64;}
// 0x1128d28c ret
cpu.pc=cpu.x[30];return Ok(());}
}
0x1128d288=>{
// 0x1128d288 orr w0, w8, #0xffffe000
{let a=((cpu.x[8] as u32 as u64));let b=((0xffffe000u64));let result=a | b;cpu.x[0]=(result) as u32 as u64;}
// 0x1128d28c ret
cpu.pc=cpu.x[30];return Ok(());
}
0x1128d290=>{
// 0x1128d290 add x12, x10, #3
{let a=((cpu.x[10]));let b=((0x3u64));let result=a.wrapping_add(b);cpu.x[12]=(result) as u64;}
// 0x1128d294 str w12, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[12] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128d298 ldrsb w11, [x9, x13]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[13])));let v0=rt.memory.read::<i8>(address.wrapping_add(0)) as i64 as u64;cpu.x[11]=(v0) as u32 as u64;}
// 0x1128d29c bfi w8, w11, #0xd, #7
cpu.x[8]=((cpu.x[8] as u32 as u64 & !0xfe000u64) | ((((cpu.x[11] as u32 as u64)) & 0x7fu64) << 13)) as u32 as u64;
// 0x1128d2a0 tbnz w11, #7, #0x1128d2ac
if ((cpu.x[11] as u32 as u64) & (1u64 << 7)) != 0 {{// 0x1128d2ac add x11, x10, #4
{let a=((cpu.x[10]));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
// 0x1128d2b0 str w11, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[11] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128d2b4 ldrsb w12, [x9, x12]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[12])));let v0=rt.memory.read::<i8>(address.wrapping_add(0)) as i64 as u64;cpu.x[12]=(v0) as u32 as u64;}
// 0x1128d2b8 bfi w8, w12, #0x14, #7
cpu.x[8]=((cpu.x[8] as u32 as u64 & !0x7f00000u64) | ((((cpu.x[12] as u32 as u64)) & 0x7fu64) << 20)) as u32 as u64;
// 0x1128d2bc tbnz w12, #7, #0x1128d2c8
if ((cpu.x[12] as u32 as u64) & (1u64 << 7)) != 0 {{// 0x1128d2c8 add w10, w10, #5
{let a=((cpu.x[10] as u32 as u64));let b=((0x5u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x1128d2cc str w10, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128d2d0 ldrb w9, [x9, x11]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[11])));let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x1128d2d4 orr w8, w8, w9, lsl #27
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[9] as u32 as u64)) as u32).wrapping_shl(27) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x1128d2d8 orr w0, w8, #0x80000000
{let a=((cpu.x[8] as u32 as u64));let b=((0x80000000u64));let result=a | b;cpu.x[0]=(result) as u32 as u64;}
// 0x1128d2dc ret
cpu.pc=cpu.x[30];return Ok(());}}{// 0x1128d2c0 orr w0, w8, #0xf8000000
{let a=((cpu.x[8] as u32 as u64));let b=((0xf8000000u64));let result=a | b;cpu.x[0]=(result) as u32 as u64;}
// 0x1128d2c4 ret
cpu.pc=cpu.x[30];return Ok(());}}}{// 0x1128d2a4 orr w0, w8, #0xfff00000
{let a=((cpu.x[8] as u32 as u64));let b=((0xfff00000u64));let result=a | b;cpu.x[0]=(result) as u32 as u64;}
// 0x1128d2a8 ret
cpu.pc=cpu.x[30];return Ok(());}
}
0x1128d2a4=>{
// 0x1128d2a4 orr w0, w8, #0xfff00000
{let a=((cpu.x[8] as u32 as u64));let b=((0xfff00000u64));let result=a | b;cpu.x[0]=(result) as u32 as u64;}
// 0x1128d2a8 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x1128d2ac=>{
// 0x1128d2ac add x11, x10, #4
{let a=((cpu.x[10]));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
// 0x1128d2b0 str w11, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[11] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128d2b4 ldrsb w12, [x9, x12]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[12])));let v0=rt.memory.read::<i8>(address.wrapping_add(0)) as i64 as u64;cpu.x[12]=(v0) as u32 as u64;}
// 0x1128d2b8 bfi w8, w12, #0x14, #7
cpu.x[8]=((cpu.x[8] as u32 as u64 & !0x7f00000u64) | ((((cpu.x[12] as u32 as u64)) & 0x7fu64) << 20)) as u32 as u64;
// 0x1128d2bc tbnz w12, #7, #0x1128d2c8
if ((cpu.x[12] as u32 as u64) & (1u64 << 7)) != 0 {{// 0x1128d2c8 add w10, w10, #5
{let a=((cpu.x[10] as u32 as u64));let b=((0x5u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x1128d2cc str w10, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128d2d0 ldrb w9, [x9, x11]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[11])));let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x1128d2d4 orr w8, w8, w9, lsl #27
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[9] as u32 as u64)) as u32).wrapping_shl(27) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x1128d2d8 orr w0, w8, #0x80000000
{let a=((cpu.x[8] as u32 as u64));let b=((0x80000000u64));let result=a | b;cpu.x[0]=(result) as u32 as u64;}
// 0x1128d2dc ret
cpu.pc=cpu.x[30];return Ok(());}}{// 0x1128d2c0 orr w0, w8, #0xf8000000
{let a=((cpu.x[8] as u32 as u64));let b=((0xf8000000u64));let result=a | b;cpu.x[0]=(result) as u32 as u64;}
// 0x1128d2c4 ret
cpu.pc=cpu.x[30];return Ok(());}
}
0x1128d2c0=>{
// 0x1128d2c0 orr w0, w8, #0xf8000000
{let a=((cpu.x[8] as u32 as u64));let b=((0xf8000000u64));let result=a | b;cpu.x[0]=(result) as u32 as u64;}
// 0x1128d2c4 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x1128d2c8=>{
// 0x1128d2c8 add w10, w10, #5
{let a=((cpu.x[10] as u32 as u64));let b=((0x5u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x1128d2cc str w10, [x0, #0x14]
{let address=(cpu.x[0]).wrapping_add(0x14u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128d2d0 ldrb w9, [x9, x11]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[11])));let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x1128d2d4 orr w8, w8, w9, lsl #27
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[9] as u32 as u64)) as u32).wrapping_shl(27) as u64;let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x1128d2d8 orr w0, w8, #0x80000000
{let a=((cpu.x[8] as u32 as u64));let b=((0x80000000u64));let result=a | b;cpu.x[0]=(result) as u32 as u64;}
// 0x1128d2dc ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x1128d1e4")),}}}
#[inline]
pub(super) unsafe fn f_1129497c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x1129497c=>{
// 0x1129497c stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x11294980 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x11294984 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x11294988 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x1129498c mov w20, w1
cpu.x[20]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x11294990 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x11294994 mov w21, #0x8965
cpu.x[21]=(((0x8965u64))) as u32 as u64;
// 0x11294998 add x0, x0, #4
{let a=((cpu.x[0]));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x1129499c mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x112949a0 mov w2, #0x9c0
cpu.x[2]=(((0x9c0u64))) as u32 as u64;
// 0x112949a4 movk w21, #0x6c07, lsl #16
cpu.x[21]=((cpu.x[21] as u32 as u64 & !(0xffff0000u64)) | 0x6c070000u64) as u32 as u64;
// 0x112949a8 bl #0x1187d480
cpu.x[30]=0x112949ac;super::chunk15::f_1187d480(cpu,rt,0x1187d480)?;
pc=0x112949ac;continue;
}
0x112949ac=>{
// 0x112949ac mov w8, #4
cpu.x[8]=(((0x4u64))) as u32 as u64;
// 0x112949b0 str w20, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=cpu.x[20] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x112949b4 str wzr, [x19, #0x9c4]
{let address=(cpu.x[19]).wrapping_add(0x9c4u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x112949b8;continue;
}
0x112949b8=>{
// 0x112949b8 eor w9, w20, w20, asr #30
{let a=((cpu.x[20] as u32 as u64));let b=(((cpu.x[20] as u32 as u64)) as i32).wrapping_shr(30) as u64;let result=a ^ b;cpu.x[9]=(result) as u32 as u64;}
// 0x112949bc madd w20, w9, w21, w21
cpu.x[20]=((((cpu.x[21] as u32 as u64))).wrapping_add((((cpu.x[9] as u32 as u64))).wrapping_mul(((cpu.x[21] as u32 as u64))))) as u32 as u64;
// 0x112949c0 str w20, [x19, x8]
{let address=((cpu.x[19]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[8])));let v0=cpu.x[20] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x112949c4 add x8, x8, #4
{let a=((cpu.x[8]));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x112949c8 cmp x8, #0x9c0
{let a=((cpu.x[8]));let b=((0x9c0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x112949cc b.ne #0x112949b8
if cpu.condition(1){pc=0x112949b8;continue;}pc=0x112949d0;continue;
}
0x112949d0=>{
// 0x112949d0 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x112949d4 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x112949d8 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x112949dc ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x1129497c")),}}}
#[inline]
pub(super) unsafe fn f_11294ed8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x11294ed8=>{
// 0x11294ed8 ldr w8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x11294edc mov w9, #-0xb505
cpu.x[9]=(((0xffffffffffff4afbu64))) as u32 as u64;
// 0x11294ee0 mov w11, #0x95f7
cpu.x[11]=(((0x95f7u64))) as u32 as u64;
// 0x11294ee4 movk w11, #0xfffe, lsl #16
cpu.x[11]=((cpu.x[11] as u32 as u64 & !(0xffff0000u64)) | 0xfffe0000u64) as u32 as u64;
// 0x11294ee8 sub w8, w1, w8
{let a=((cpu.x[1] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x11294eec add w10, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x11294ef0 cmp w10, w11
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[11] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x11294ef4 b.hs #0x11294f00
if cpu.condition(2){pc=0x11294f00;continue;}pc=0x11294ef8;continue;
}
0x11294ef8=>{
// 0x11294ef8 mov w0, #0x7fffffff
cpu.x[0]=(((0x7fffffffu64))) as u32 as u64;
// 0x11294efc b #0x112944a4
super::chunk0::f_112944a4(cpu,rt,0x112944a4)?;return Ok(());
}
0x11294f00=>{
// 0x11294f00 ldr w10, [x0, #4]
{let address=(cpu.x[0]).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x11294f04 mov w0, #0x7fffffff
cpu.x[0]=(((0x7fffffffu64))) as u32 as u64;
// 0x11294f08 sub w10, w2, w10
{let a=((cpu.x[2] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[10]=(result) as u32 as u64;}
// 0x11294f0c add w9, w10, w9
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x11294f10 cmp w9, w11
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[11] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x11294f14 b.lo #0x11294f30
if cpu.condition(3){pc=0x11294f30;continue;}pc=0x11294f18;continue;
}
0x11294f18=>{
// 0x11294f18 mul w8, w8, w8
cpu.x[8]=((((cpu.x[8] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64)))) as u32 as u64;
// 0x11294f1c mul w9, w10, w10
cpu.x[9]=((((cpu.x[10] as u32 as u64))).wrapping_mul(((cpu.x[10] as u32 as u64)))) as u32 as u64;
// 0x11294f20 eor w10, w8, #0x7fffffff
{let a=((cpu.x[8] as u32 as u64));let b=((0x7fffffffu64));let result=a ^ b;cpu.x[10]=(result) as u32 as u64;}
// 0x11294f24 add w8, w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x11294f28 cmp w9, w10
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x11294f2c csel w0, w8, w0, lo
cpu.x[0]=(if cpu.condition(3){((cpu.x[8] as u32 as u64))}else{((cpu.x[0] as u32 as u64))}) as u32 as u64;
pc=0x11294f30;continue;
}
0x11294f30=>{
// 0x11294f30 b #0x112944a4
super::chunk0::f_112944a4(cpu,rt,0x112944a4)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x11294ed8")),}}}
#[inline]
pub(super) unsafe fn f_112ae75c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x112ae75c=>{
// 0x112ae75c cbz w3, #0x112ae7f4
if (cpu.x[3] as u32 as u64) == 0 {pc=0x112ae7f4;continue;}pc=0x112ae760;continue;
}
0x112ae760=>{
// 0x112ae760 stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x112ae764 str x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x112ae768 stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x112ae76c stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x112ae770 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x112ae774 sub w8, w2, w3
{let a=((cpu.x[2] as u32 as u64));let b=((cpu.x[3] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x112ae778 add w22, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[22]=(result) as u32 as u64;}
// 0x112ae77c cmp w22, #1
{let a=((cpu.x[22] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x112ae780 b.lt #0x112ae7dc
if cpu.condition(11){pc=0x112ae7dc;continue;}pc=0x112ae784;continue;
}
0x112ae784=>{
// 0x112ae784 ldrb w23, [x1]
{let address=(cpu.x[1]).wrapping_add(0x0u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[23]=(v0) as u32 as u64;}
// 0x112ae788 sxtw x21, w3
cpu.x[21]=((((cpu.x[3] as u32 as u64))) as i32 as i64 as u64) as u64;
// 0x112ae78c mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x112ae790 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x112ae794 b #0x112ae7a4
pc=0x112ae7a4;continue;
}
0x112ae798=>{
// 0x112ae798 subs x22, x22, #1
{let a=((cpu.x[22]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[22]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x112ae79c add x20, x20, #1
{let a=((cpu.x[20]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[20]=(result) as u64;}
// 0x112ae7a0 b.eq #0x112ae7dc
if cpu.condition(0){pc=0x112ae7dc;continue;}pc=0x112ae7a4;continue;
}
0x112ae7a4=>{
// 0x112ae7a4 ldrb w8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x112ae7a8 cmp w8, w23
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[23] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x112ae7ac b.ne #0x112ae798
if cpu.condition(1){pc=0x112ae798;continue;}pc=0x112ae7b0;continue;
}
0x112ae7b0=>{
// 0x112ae7b0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x112ae7b4 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x112ae7b8 mov x2, x21
cpu.x[2]=(((cpu.x[21]))) as u64;
// 0x112ae7bc bl #0x1187d490
cpu.x[30]=0x112ae7c0;super::dispatch(cpu,rt,0x1187d490)?;
// 0x112ae7c0 cbnz w0, #0x112ae798
if (cpu.x[0] as u32 as u64) != 0 {pc=0x112ae798;continue;}pc=0x112ae7c4;continue;
}
0x112ae7c4=>{
// 0x112ae7c4 mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
// 0x112ae7c8 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x112ae7cc ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x112ae7d0 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x112ae7d4 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x112ae7d8 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x112ae7dc=>{
// 0x112ae7dc mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x112ae7e0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x112ae7e4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x112ae7e8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x112ae7ec ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x112ae7f0 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x112ae7f4=>{
// 0x112ae7f4 mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
// 0x112ae7f8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x112ae75c")),}}}
#[inline]
pub(super) unsafe fn f_112aef98(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x112aef98=>{
// 0x112aef98 mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x112aef9c b #0x1142effc
super::chunk3::f_1142effc(cpu,rt,0x1142effc)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x112aef98")),}}}
#[inline]
pub(super) unsafe fn f_114307a8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x114307a8=>{
// 0x114307a8 ldrsw x2, [x0, #4]
{let address=(cpu.x[0]).wrapping_add(0x4u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[2]=(v0) as u64;}
// 0x114307ac ldr w8, [x1, #4]
{let address=(cpu.x[1]).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x114307b0 cmp w2, w8
{let a=((cpu.x[2] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x114307b4 b.ne #0x114307e8
if cpu.condition(1){pc=0x114307e8;continue;}pc=0x114307b8;continue;
}
0x114307b8=>{
// 0x114307b8 stp x29, x30, [sp, #-0x10]!
{let address=(cpu.sp).wrapping_add(0xfffffffffffffff0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x114307bc mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x114307c0 ldr x8, [x0, #8]!
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;cpu.x[0]=(address) as u64;}
// 0x114307c4 cmp w2, #8
{let a=((cpu.x[2] as u32 as u64));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x114307c8 ldr x9, [x1, #8]!
{let address=(cpu.x[1]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;cpu.x[1]=(address) as u64;}
// 0x114307cc csel x0, x0, x8, lt
cpu.x[0]=(if cpu.condition(11){((cpu.x[0]))}else{((cpu.x[8]))}) as u64;
// 0x114307d0 csel x1, x1, x9, lt
cpu.x[1]=(if cpu.condition(11){((cpu.x[1]))}else{((cpu.x[9]))}) as u64;
// 0x114307d4 bl #0x1187d490
cpu.x[30]=0x114307d8;super::dispatch(cpu,rt,0x1187d490)?;
pc=0x114307d8;continue;
}
0x114307d8=>{
// 0x114307d8 cmp w0, #0
{let a=((cpu.x[0] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x114307dc cset w0, eq
cpu.x[0]=((cpu.condition(0)) as u64) as u32 as u64;
// 0x114307e0 ldp x29, x30, [sp], #0x10
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x10u64)))) as u64;}
// 0x114307e4 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x114307e8=>{
// 0x114307e8 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x114307ec ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x114307a8")),}}}
#[inline]
pub(super) unsafe fn f_1187d2b0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x1187d2b0=>{
// 0x1187d2b0 adrp x16, #0x11981000
cpu.x[16]=(0x11981000u64) as u64;
// 0x1187d2b4 ldr x17, [x16, #0x968]
{let address=(cpu.x[16]).wrapping_add(0x968u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[17]=(v0) as u64;}
// 0x1187d2b8 add x16, x16, #0x968
{let a=((cpu.x[16]));let b=((0x968u64));let result=a.wrapping_add(b);cpu.x[16]=(result) as u64;}
// 0x1187d2bc br x17
{let target=cpu.x[17];if target>=0x1187d2b0 && target<0x1187d2f0{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
0x1187d2c0=>{
// 0x1187d2c0 adrp x16, #0x11981000
cpu.x[16]=(0x11981000u64) as u64;
// 0x1187d2c4 ldr x17, [x16, #0x970]
{let address=(cpu.x[16]).wrapping_add(0x970u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[17]=(v0) as u64;}
// 0x1187d2c8 add x16, x16, #0x970
{let a=((cpu.x[16]));let b=((0x970u64));let result=a.wrapping_add(b);cpu.x[16]=(result) as u64;}
// 0x1187d2cc br x17
{let target=cpu.x[17];if target>=0x1187d2b0 && target<0x1187d2f0{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
0x1187d2d0=>{
// 0x1187d2d0 adrp x16, #0x11981000
cpu.x[16]=(0x11981000u64) as u64;
// 0x1187d2d4 ldr x17, [x16, #0x978]
{let address=(cpu.x[16]).wrapping_add(0x978u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[17]=(v0) as u64;}
// 0x1187d2d8 add x16, x16, #0x978
{let a=((cpu.x[16]));let b=((0x978u64));let result=a.wrapping_add(b);cpu.x[16]=(result) as u64;}
// 0x1187d2dc br x17
{let target=cpu.x[17];if target>=0x1187d2b0 && target<0x1187d2f0{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
0x1187d2e0=>{
// 0x1187d2e0 adrp x16, #0x11981000
cpu.x[16]=(0x11981000u64) as u64;
// 0x1187d2e4 ldr x17, [x16, #0x980]
{let address=(cpu.x[16]).wrapping_add(0x980u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[17]=(v0) as u64;}
// 0x1187d2e8 add x16, x16, #0x980
{let a=((cpu.x[16]));let b=((0x980u64));let result=a.wrapping_add(b);cpu.x[16]=(result) as u64;}
// 0x1187d2ec br x17
{let target=cpu.x[17];if target>=0x1187d2b0 && target<0x1187d2f0{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
_=>return Err(format!("untranslated block {pc:#x} in 0x1187d2b0")),}}}
#[inline]
pub(super) unsafe fn f_1187d480(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x1187d480=>{
// 0x1187d480 adrp x16, #0x11981000
cpu.x[16]=(0x11981000u64) as u64;
// 0x1187d484 ldr x17, [x16, #0xa50]
{let address=(cpu.x[16]).wrapping_add(0xa50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[17]=(v0) as u64;}
// 0x1187d488 add x16, x16, #0xa50
{let a=((cpu.x[16]));let b=((0xa50u64));let result=a.wrapping_add(b);cpu.x[16]=(result) as u64;}
// 0x1187d48c br x17
{let target=cpu.x[17];if target>=0x1187d480 && target<0x1187d4a0{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
0x1187d490=>{
// 0x1187d490 adrp x16, #0x11981000
cpu.x[16]=(0x11981000u64) as u64;
// 0x1187d494 ldr x17, [x16, #0xa58]
{let address=(cpu.x[16]).wrapping_add(0xa58u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[17]=(v0) as u64;}
// 0x1187d498 add x16, x16, #0xa58
{let a=((cpu.x[16]));let b=((0xa58u64));let result=a.wrapping_add(b);cpu.x[16]=(result) as u64;}
// 0x1187d49c br x17
{let target=cpu.x[17];if target>=0x1187d480 && target<0x1187d4a0{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
_=>return Err(format!("untranslated block {pc:#x} in 0x1187d480")),}}}
