#![allow(unused_parens,unreachable_code,unused_variables,unused_mut)]
use crate::{cpu::Cpu,runtime::Runtime};
use super::helpers::*;
#[inline]
pub(super) unsafe fn f_108346d4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x108346d4=>{
// 0x108346d4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x108346d4")),}}}
#[inline]
pub(super) unsafe fn f_10834788(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10834788=>{
// 0x10834788 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10834788")),}}}
#[inline]
pub(super) unsafe fn f_10990b78(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10990b78=>{
// 0x10990b78 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10990b78")),}}}
#[inline]
pub(super) unsafe fn f_10d2d588(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d2d588=>{
// 0x10d2d588 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d2d588")),}}}
#[inline]
pub(super) unsafe fn f_10d2db5c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d2db5c=>{
// 0x10d2db5c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d2db5c")),}}}
#[inline]
pub(super) unsafe fn f_10d34864(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d34864=>{
// 0x10d34864 ldr x8, [x0, #0x1b0]
{let address=(cpu.x[0]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d34868 ldr x8, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3486c cbz x8, #0x10d34880
if (cpu.x[8]) == 0 {pc=0x10d34880;continue;}pc=0x10d34870;continue;
}
0x10d34870=>{
// 0x10d34870 ldrb w8, [x8, #0x116]
{let address=(cpu.x[8]).wrapping_add(0x116u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d34874 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d34878 cset w0, ne
cpu.x[0]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x10d3487c ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10d34880=>{
// 0x10d34880 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10d34884 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d34864")),}}}
#[inline]
pub(super) unsafe fn f_10d35630(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d35630=>{
// 0x10d35630 stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d35634 str x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d35638 stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d3563c stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d35640 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d35644 ldr x8, [x0, #0x1b0]
{let address=(cpu.x[0]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d35648 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10d3564c ldr x0, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d35650 cbz x0, #0x10d35808
if (cpu.x[0]) == 0 {pc=0x10d35808;continue;}pc=0x10d35654;continue;
}
0x10d35654=>{
// 0x10d35654 bl #0x10e2d5a4
cpu.x[30]=0x10d35658;super::chunk4::f_10e2d5a4(cpu,rt,0x10e2d5a4)?;
pc=0x10d35658;continue;
}
0x10d35658=>{
// 0x10d35658 tbz w0, #0, #0x10d35808
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d35808;continue;}pc=0x10d3565c;continue;
}
0x10d3565c=>{
// 0x10d3565c ldrb w8, [x19, #0x1f8]
{let address=(cpu.x[19]).wrapping_add(0x1f8u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d35660 cbz w8, #0x10d356a0
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10d356a0;continue;}pc=0x10d35664;continue;
}
0x10d35664=>{
// 0x10d35664 ldr w9, [x19, #0x1a0]
{let address=(cpu.x[19]).wrapping_add(0x1a0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10d35668 ldr w8, [x19, #0x1a4]
{let address=(cpu.x[19]).wrapping_add(0x1a4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3566c add w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10d35670 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d35674 str w9, [x19, #0x1a0]
{let address=(cpu.x[19]).wrapping_add(0x1a0u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d35678 b.lt #0x10d356a0
if cpu.condition(11){pc=0x10d356a0;continue;}pc=0x10d3567c;continue;
}
0x10d3567c=>{
// 0x10d3567c sub w8, w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10d35680 cmp w8, #0xc9
{let a=((cpu.x[8] as u32 as u64));let b=((0xc9u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d35684 b.lt #0x10d356a0
if cpu.condition(11){pc=0x10d356a0;continue;}pc=0x10d35688;continue;
}
0x10d35688=>{
// 0x10d35688 ldr x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3568c strb wzr, [x19, #0x1f8]
{let address=(cpu.x[19]).wrapping_add(0x1f8u64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10d35690 bl #0x110cb308
cpu.x[30]=0x10d35694;super::chunk15::f_110cb308(cpu,rt,0x110cb308)?;
// 0x10d35694 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d35698 ldr x8, [x8, #0x4d0]
{let address=(cpu.x[8]).wrapping_add(0x4d0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d3569c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d356a0;super::dispatch(cpu,rt,target)?;}
pc=0x10d356a0;continue;
}
0x10d356a0=>{
// 0x10d356a0 ldr w8, [x19, #0x1fc]
{let address=(cpu.x[19]).wrapping_add(0x1fcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d356a4 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d356a8 b.lt #0x10d35780
if cpu.condition(11){pc=0x10d35780;continue;}pc=0x10d356ac;continue;
}
0x10d356ac=>{
// 0x10d356ac ldr x8, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d356b0 ldr w1, [x19, #0x200]
{let address=(cpu.x[19]).wrapping_add(0x200u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d356b4 ldr x0, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d356b8 bl #0x10f8be18
cpu.x[30]=0x10d356bc;super::chunk5::f_10f8be18(cpu,rt,0x10f8be18)?;
// 0x10d356bc mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10d356c0 cbz x0, #0x10d356e0
if (cpu.x[0]) == 0 {pc=0x10d356e0;continue;}pc=0x10d356c4;continue;
}
0x10d356c4=>{
// 0x10d356c4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d356c8 bl #0x10f94298
cpu.x[30]=0x10d356cc;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
// 0x10d356cc mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d356d0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d356d4 bl #0x10f942a0
cpu.x[30]=0x10d356d8;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
// 0x10d356d8 mov w22, w0
cpu.x[22]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d356dc b #0x10d356e8
pc=0x10d356e8;continue;
}
0x10d356e0=>{
// 0x10d356e0 mov w21, wzr
cpu.x[21]=(((0u64))) as u32 as u64;
// 0x10d356e4 mov w22, wzr
cpu.x[22]=(((0u64))) as u32 as u64;
pc=0x10d356e8;continue;
}
0x10d356e8=>{
// 0x10d356e8 ldr x0, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d356ec bl #0x10e16208
cpu.x[30]=0x10d356f0;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
// 0x10d356f0 ldr w1, [x0, #0x4cc]
{let address=(cpu.x[0]).wrapping_add(0x4ccu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10d356f4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d356f8 mov w2, w21
cpu.x[2]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10d356fc mov w3, w22
cpu.x[3]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10d35700 mov w4, #-1
cpu.x[4]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10d35704 mov w5, wzr
cpu.x[5]=(((0u64))) as u32 as u64;
// 0x10d35708 bl #0x10d35de4
cpu.x[30]=0x10d3570c;super::dispatch(cpu,rt,0x10d35de4)?;
// 0x10d3570c ldr w8, [x19, #0x1fc]
{let address=(cpu.x[19]).wrapping_add(0x1fcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d35710 subs w8, w8, #0x32
{let a=((cpu.x[8] as u32 as u64));let b=((0x32u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10d35714 str w8, [x19, #0x1fc]
{let address=(cpu.x[19]).wrapping_add(0x1fcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d35718 b.gt #0x10d35780
if cpu.condition(12){pc=0x10d35780;continue;}pc=0x10d3571c;continue;
}
0x10d3571c=>{
// 0x10d3571c cbz x20, #0x10d35728
if (cpu.x[20]) == 0 {pc=0x10d35728;continue;}pc=0x10d35720;continue;
}
0x10d35720=>{
// 0x10d35720 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d35724 bl #0x10f84c18
cpu.x[30]=0x10d35728;super::dispatch(cpu,rt,0x10f84c18)?;
pc=0x10d35728;continue;
}
0x10d35728=>{
// 0x10d35728 mov w8, #-1
cpu.x[8]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10d3572c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d35730 str w8, [x19, #0x200]
{let address=(cpu.x[19]).wrapping_add(0x200u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d35734 bl #0x10d360a8
cpu.x[30]=0x10d35738;super::chunk4::f_10d360a8(cpu,rt,0x10d360a8)?;
// 0x10d35738 tbnz w0, #0, #0x10d35780
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10d35780;continue;}pc=0x10d3573c;continue;
}
0x10d3573c=>{
// 0x10d3573c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d35740 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10d35744 mov w2, wzr
cpu.x[2]=(((0u64))) as u32 as u64;
// 0x10d35748 mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x10d3574c mov w4, #-1
cpu.x[4]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10d35750 mov w5, #1
cpu.x[5]=(((0x1u64))) as u32 as u64;
// 0x10d35754 bl #0x10d35de4
cpu.x[30]=0x10d35758;super::dispatch(cpu,rt,0x10d35de4)?;
// 0x10d35758 ldr x0, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3575c bl #0x10e16208
cpu.x[30]=0x10d35760;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
// 0x10d35760 ldr x8, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d35764 ldr w20, [x0, #0x4d8]
{let address=(cpu.x[0]).wrapping_add(0x4d8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[20]=(v0) as u32 as u64;}
// 0x10d35768 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10d3576c bl #0x10e16208
cpu.x[30]=0x10d35770;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
// 0x10d35770 ldr w2, [x0, #0x4e0]
{let address=(cpu.x[0]).wrapping_add(0x4e0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[2]=(v0) as u32 as u64;}
// 0x10d35774 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d35778 mov w1, w20
cpu.x[1]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x10d3577c bl #0x10d36174
cpu.x[30]=0x10d35780;super::dispatch(cpu,rt,0x10d36174)?;
pc=0x10d35780;continue;
}
0x10d35780=>{
// 0x10d35780 ldrb w8, [x19, #0x1f8]
{let address=(cpu.x[19]).wrapping_add(0x1f8u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d35784 cbnz w8, #0x10d35808
if (cpu.x[8] as u32 as u64) != 0 {pc=0x10d35808;continue;}pc=0x10d35788;continue;
}
0x10d35788=>{
// 0x10d35788 ldr w8, [x19, #0x1fc]
{let address=(cpu.x[19]).wrapping_add(0x1fcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d3578c cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d35790 b.gt #0x10d35808
if cpu.condition(12){pc=0x10d35808;continue;}pc=0x10d35794;continue;
}
0x10d35794=>{
// 0x10d35794 ldr x8, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d35798 ldr x21, [x8, #0x40]
{let address=(cpu.x[8]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10d3579c ldr w8, [x21, #0x14]
{let address=(cpu.x[21]).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d357a0 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d357a4 b.lt #0x10d35808
if cpu.condition(11){pc=0x10d35808;continue;}pc=0x10d357a8;continue;
}
0x10d357a8=>{
// 0x10d357a8 lsl x22, x8, #3
cpu.x[22]=(((((cpu.x[8]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10d357ac mov x23, xzr
cpu.x[23]=(((0u64))) as u64;
// 0x10d357b0 b #0x10d357c0
pc=0x10d357c0;continue;
}
0x10d357b4=>{
// 0x10d357b4 add x23, x23, #8
{let a=((cpu.x[23]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[23]=(result) as u64;}
// 0x10d357b8 cmp x22, x23
{let a=((cpu.x[22]));let b=((cpu.x[23]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d357bc b.eq #0x10d35808
if cpu.condition(0){pc=0x10d35808;continue;}pc=0x10d357c0;continue;
}
0x10d357c0=>{
// 0x10d357c0 ldr x8, [x21, #8]
{let address=(cpu.x[21]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d357c4 ldr x20, [x8, x23]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[23])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10d357c8 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d357cc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d357d0 ldr x8, [x8, #0x190]
{let address=(cpu.x[8]).wrapping_add(0x190u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d357d4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d357d8;super::dispatch(cpu,rt,target)?;}
// 0x10d357d8 tbz w0, #0, #0x10d357b4
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d357b4;continue;}pc=0x10d357dc;continue;
}
0x10d357dc=>{
// 0x10d357dc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d357e0 bl #0x10f942f0
cpu.x[30]=0x10d357e4;super::chunk11::f_10f942f0(cpu,rt,0x10f942f0)?;
// 0x10d357e4 cbz x0, #0x10d357b4
if (cpu.x[0]) == 0 {pc=0x10d357b4;continue;}pc=0x10d357e8;continue;
}
0x10d357e8=>{
// 0x10d357e8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d357ec mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d357f0 bl #0x10d36248
cpu.x[30]=0x10d357f4;super::dispatch(cpu,rt,0x10d36248)?;
// 0x10d357f4 tbz w0, #0, #0x10d357b4
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d357b4;continue;}pc=0x10d357f8;continue;
}
0x10d357f8=>{
// 0x10d357f8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d357fc mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d35800 bl #0x10d36374
cpu.x[30]=0x10d35804;super::dispatch(cpu,rt,0x10d36374)?;
// 0x10d35804 b #0x10d357b4
pc=0x10d357b4;continue;
}
0x10d35808=>{
// 0x10d35808 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d3580c ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10d35810 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d35814 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10d35818 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d35630")),}}}
#[inline]
pub(super) unsafe fn f_10d378d8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d378d8=>{
// 0x10d378d8 sub sp, sp, #0x70
{let a=((cpu.sp));let b=((0x70u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x10d378dc stp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d378e0 stp x28, x27, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[28];let v1=cpu.x[27];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d378e4 stp x26, x25, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[26];let v1=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d378e8 stp x24, x23, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d378ec stp x22, x21, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d378f0 stp x20, x19, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d378f4 add x29, sp, #0x10
{let a=((cpu.sp));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x10d378f8 ldr x23, [x0, #0x1b0]
{let address=(cpu.x[0]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10d378fc ldr x20, [x0, #0xc0]
{let address=(cpu.x[0]).wrapping_add(0xc0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10d37900 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10d37904 cbz x23, #0x10d3791c
if (cpu.x[23]) == 0 {pc=0x10d3791c;continue;}pc=0x10d37908;continue;
}
0x10d37908=>{
// 0x10d37908 cbz x20, #0x10d37930
if (cpu.x[20]) == 0 {pc=0x10d37930;continue;}pc=0x10d3790c;continue;
}
0x10d3790c=>{
// 0x10d3790c ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d37910 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d37914 b.ge #0x10d3794c
if cpu.condition(10){pc=0x10d3794c;continue;}pc=0x10d37918;continue;
}
0x10d37918=>{
// 0x10d37918 b #0x10d37a88
pc=0x10d37a88;continue;
}
0x10d3791c=>{
// 0x10d3791c adrp x0, #0x1030e000
cpu.x[0]=(0x1030e000u64) as u64;
// 0x10d37920 add x0, x0, #0x27a
{let a=((cpu.x[0]));let b=((0x27au64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d37924 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d37928 bl #0x1128df18
cpu.x[30]=0x10d3792c;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10d3792c cbnz x20, #0x10d3790c
if (cpu.x[20]) != 0 {pc=0x10d3790c;continue;}pc=0x10d37930;continue;
}
0x10d37930=>{
// 0x10d37930 adrp x0, #0x10323000
cpu.x[0]=(0x10323000u64) as u64;
// 0x10d37934 add x0, x0, #0x562
{let a=((cpu.x[0]));let b=((0x562u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d37938 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d3793c bl #0x1128df18
cpu.x[30]=0x10d37940;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10d37940 ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d37944 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d37948 b.lt #0x10d37a88
if cpu.condition(11){pc=0x10d37a88;continue;}pc=0x10d3794c;continue;
}
0x10d3794c=>{
// 0x10d3794c ldr x9, [x19, #0xe0]
{let address=(cpu.x[19]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d37950 cbz x9, #0x10d37968
if (cpu.x[9]) == 0 {pc=0x10d37968;continue;}pc=0x10d37954;continue;
}
0x10d37954=>{
// 0x10d37954 adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x10d37958 add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d3795c mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d37960 bl #0x1128df18
cpu.x[30]=0x10d37964;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10d37964 ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
pc=0x10d37968;continue;
}
0x10d37968=>{
// 0x10d37968 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d3796c b.le #0x10d37a88
if cpu.condition(13){pc=0x10d37a88;continue;}pc=0x10d37970;continue;
}
0x10d37970=>{
// 0x10d37970 ldr x9, [x19, #0xe8]
{let address=(cpu.x[19]).wrapping_add(0xe8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d37974 cbz x9, #0x10d3798c
if (cpu.x[9]) == 0 {pc=0x10d3798c;continue;}pc=0x10d37978;continue;
}
0x10d37978=>{
// 0x10d37978 adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x10d3797c add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d37980 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d37984 bl #0x1128df18
cpu.x[30]=0x10d37988;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10d37988 ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
pc=0x10d3798c;continue;
}
0x10d3798c=>{
// 0x10d3798c cmp w8, #3
{let a=((cpu.x[8] as u32 as u64));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d37990 b.lt #0x10d37a88
if cpu.condition(11){pc=0x10d37a88;continue;}pc=0x10d37994;continue;
}
0x10d37994=>{
// 0x10d37994 ldr x9, [x19, #0xf0]
{let address=(cpu.x[19]).wrapping_add(0xf0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d37998 cbz x9, #0x10d379b0
if (cpu.x[9]) == 0 {pc=0x10d379b0;continue;}pc=0x10d3799c;continue;
}
0x10d3799c=>{
// 0x10d3799c adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x10d379a0 add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d379a4 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d379a8 bl #0x1128df18
cpu.x[30]=0x10d379ac;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10d379ac ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
pc=0x10d379b0;continue;
}
0x10d379b0=>{
// 0x10d379b0 cmp w8, #4
{let a=((cpu.x[8] as u32 as u64));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d379b4 b.lt #0x10d37a88
if cpu.condition(11){pc=0x10d37a88;continue;}pc=0x10d379b8;continue;
}
0x10d379b8=>{
// 0x10d379b8 ldr x9, [x19, #0xf8]
{let address=(cpu.x[19]).wrapping_add(0xf8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d379bc cbz x9, #0x10d379d4
if (cpu.x[9]) == 0 {pc=0x10d379d4;continue;}pc=0x10d379c0;continue;
}
0x10d379c0=>{
// 0x10d379c0 adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x10d379c4 add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d379c8 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d379cc bl #0x1128df18
cpu.x[30]=0x10d379d0;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10d379d0 ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
pc=0x10d379d4;continue;
}
0x10d379d4=>{
// 0x10d379d4 cmp w8, #5
{let a=((cpu.x[8] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d379d8 b.lt #0x10d37a88
if cpu.condition(11){pc=0x10d37a88;continue;}pc=0x10d379dc;continue;
}
0x10d379dc=>{
// 0x10d379dc ldr x9, [x19, #0x100]
{let address=(cpu.x[19]).wrapping_add(0x100u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d379e0 cbz x9, #0x10d379f8
if (cpu.x[9]) == 0 {pc=0x10d379f8;continue;}pc=0x10d379e4;continue;
}
0x10d379e4=>{
// 0x10d379e4 adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x10d379e8 add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d379ec mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d379f0 bl #0x1128df18
cpu.x[30]=0x10d379f4;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10d379f4 ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
pc=0x10d379f8;continue;
}
0x10d379f8=>{
// 0x10d379f8 cmp w8, #6
{let a=((cpu.x[8] as u32 as u64));let b=((0x6u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d379fc b.lt #0x10d37a88
if cpu.condition(11){pc=0x10d37a88;continue;}pc=0x10d37a00;continue;
}
0x10d37a00=>{
// 0x10d37a00 ldr x9, [x19, #0x108]
{let address=(cpu.x[19]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d37a04 cbz x9, #0x10d37a1c
if (cpu.x[9]) == 0 {pc=0x10d37a1c;continue;}pc=0x10d37a08;continue;
}
0x10d37a08=>{
// 0x10d37a08 adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x10d37a0c add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d37a10 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d37a14 bl #0x1128df18
cpu.x[30]=0x10d37a18;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10d37a18 ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
pc=0x10d37a1c;continue;
}
0x10d37a1c=>{
// 0x10d37a1c cmp w8, #7
{let a=((cpu.x[8] as u32 as u64));let b=((0x7u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d37a20 b.lt #0x10d37a88
if cpu.condition(11){pc=0x10d37a88;continue;}pc=0x10d37a24;continue;
}
0x10d37a24=>{
// 0x10d37a24 mov x9, sp
cpu.x[9]=(((cpu.sp))) as u64;
// 0x10d37a28 mov w21, #6
cpu.x[21]=(((0x6u64))) as u32 as u64;
// 0x10d37a2c adrp x22, #0x102e0000
cpu.x[22]=(0x102e0000u64) as u64;
// 0x10d37a30 add x22, x22, #0x6ab
{let a=((cpu.x[22]));let b=((0x6abu64));let result=a.wrapping_add(b);cpu.x[22]=(result) as u64;}
// 0x10d37a34 add x24, x9, #8
{let a=((cpu.x[9]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[24]=(result) as u64;}
// 0x10d37a38 b #0x10d37a48
pc=0x10d37a48;continue;
}
0x10d37a3c=>{
// 0x10d37a3c add x21, x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[21]=(result) as u64;}
// 0x10d37a40 cmp x21, w8, sxtw
{let a=((cpu.x[21]));let b=(((cpu.x[8] as u32 as u64) as i32 as i64 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d37a44 b.ge #0x10d37a88
if cpu.condition(10){pc=0x10d37a88;continue;}pc=0x10d37a48;continue;
}
0x10d37a48=>{
// 0x10d37a48 cmp x21, #0x64
{let a=((cpu.x[21]));let b=((0x64u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d37a4c b.eq #0x10d37a3c
if cpu.condition(0){pc=0x10d37a3c;continue;}pc=0x10d37a50;continue;
}
0x10d37a50=>{
// 0x10d37a50 mov x8, sp
cpu.x[8]=(((cpu.sp))) as u64;
// 0x10d37a54 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10d37a58 mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10d37a5c bl #0x114328cc
cpu.x[30]=0x10d37a60;super::chunk0::f_114328cc(cpu,rt,0x114328cc)?;
// 0x10d37a60 ldr w8, [sp, #4]
{let address=(cpu.sp).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d37a64 ldr x9, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d37a68 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d37a6c cmp w8, #8
{let a=((cpu.x[8] as u32 as u64));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d37a70 csel x0, x24, x9, lt
cpu.x[0]=(if cpu.condition(11){((cpu.x[24]))}else{((cpu.x[9]))}) as u64;
// 0x10d37a74 bl #0x1128df18
cpu.x[30]=0x10d37a78;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10d37a78 mov x0, sp
cpu.x[0]=(((cpu.sp))) as u64;
// 0x10d37a7c bl #0x1142fcc4
cpu.x[30]=0x10d37a80;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
// 0x10d37a80 ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d37a84 b #0x10d37a3c
pc=0x10d37a3c;continue;
}
0x10d37a88=>{
// 0x10d37a88 ldr x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d37a8c bl #0x110c98a0
cpu.x[30]=0x10d37a90;super::chunk7::f_110c98a0(cpu,rt,0x110c98a0)?;
pc=0x10d37a90;continue;
}
0x10d37a90=>{
// 0x10d37a90 ldr x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d37a94 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d37a98 bl #0x110c9904
cpu.x[30]=0x10d37a9c;super::chunk8::f_110c9904(cpu,rt,0x110c9904)?;
pc=0x10d37a9c;continue;
}
0x10d37a9c=>{
// 0x10d37a9c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d37aa0 bl #0x10e8bcd8
cpu.x[30]=0x10d37aa4;super::chunk8::f_10e8bcd8(cpu,rt,0x10e8bcd8)?;
pc=0x10d37aa4;continue;
}
0x10d37aa4=>{
// 0x10d37aa4 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d37aa8 b.lt #0x10d37bf8
if cpu.condition(11){pc=0x10d37bf8;continue;}pc=0x10d37aac;continue;
}
0x10d37aac=>{
// 0x10d37aac mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d37ab0 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10d37ab4 bl #0x10d37e10
cpu.x[30]=0x10d37ab8;super::dispatch(cpu,rt,0x10d37e10)?;
pc=0x10d37ab8;continue;
}
0x10d37ab8=>{
// 0x10d37ab8 ldr x9, [x23, #0x18]
{let address=(cpu.x[23]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d37abc ldr w8, [x9, #0x24c]
{let address=(cpu.x[9]).wrapping_add(0x24cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d37ac0 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d37ac4 b.lt #0x10d37b98
if cpu.condition(11){pc=0x10d37b98;continue;}pc=0x10d37ac8;continue;
}
0x10d37ac8=>{
// 0x10d37ac8 ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d37acc cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d37ad0 b.lt #0x10d37b98
if cpu.condition(11){pc=0x10d37b98;continue;}pc=0x10d37ad4;continue;
}
0x10d37ad4=>{
// 0x10d37ad4 mov w20, wzr
cpu.x[20]=(((0u64))) as u32 as u64;
// 0x10d37ad8 b #0x10d37aec
pc=0x10d37aec;continue;
}
0x10d37adc=>{
// 0x10d37adc ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
pc=0x10d37ae0;continue;
}
0x10d37ae0=>{
// 0x10d37ae0 add w20, w20, #1
{let a=((cpu.x[20] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[20]=(result) as u32 as u64;}
// 0x10d37ae4 cmp w20, w8
{let a=((cpu.x[20] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d37ae8 b.ge #0x10d37b90
if cpu.condition(10){pc=0x10d37b90;continue;}pc=0x10d37aec;continue;
}
0x10d37aec=>{
// 0x10d37aec ldr x24, [x23, #0x18]
{let address=(cpu.x[23]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[24]=(v0) as u64;}
// 0x10d37af0 ldr w25, [x24, #0x24c]
{let address=(cpu.x[24]).wrapping_add(0x24cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[25]=(v0) as u32 as u64;}
// 0x10d37af4 cmp w25, #1
{let a=((cpu.x[25] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d37af8 b.lt #0x10d37ae0
if cpu.condition(11){pc=0x10d37ae0;continue;}pc=0x10d37afc;continue;
}
0x10d37afc=>{
// 0x10d37afc mov x26, xzr
cpu.x[26]=(((0u64))) as u64;
// 0x10d37b00 b #0x10d37b10
pc=0x10d37b10;continue;
}
0x10d37b04=>{
// 0x10d37b04 add x26, x26, #1
{let a=((cpu.x[26]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[26]=(result) as u64;}
// 0x10d37b08 cmp x26, x25
{let a=((cpu.x[26]));let b=((cpu.x[25]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d37b0c b.eq #0x10d37adc
if cpu.condition(0){pc=0x10d37adc;continue;}pc=0x10d37b10;continue;
}
0x10d37b10=>{
// 0x10d37b10 ldr x8, [x24, #0x240]
{let address=(cpu.x[24]).wrapping_add(0x240u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d37b14 ldr x21, [x8, x26, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[26])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10d37b18 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10d37b1c bl #0x10e0b624
cpu.x[30]=0x10d37b20;super::chunk10::f_10e0b624(cpu,rt,0x10e0b624)?;
// 0x10d37b20 ldr w8, [x21, #0x9c]
{let address=(cpu.x[21]).wrapping_add(0x9cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d37b24 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d37b28 b.lt #0x10d37b04
if cpu.condition(11){pc=0x10d37b04;continue;}pc=0x10d37b2c;continue;
}
0x10d37b2c=>{
// 0x10d37b2c lsl x27, x8, #3
cpu.x[27]=(((((cpu.x[8]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10d37b30 mov w22, w0
cpu.x[22]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d37b34 mov x28, xzr
cpu.x[28]=(((0u64))) as u64;
// 0x10d37b38 b #0x10d37b6c
pc=0x10d37b6c;continue;
}
0x10d37b3c=>{
// 0x10d37b3c ldp w4, w5, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[4]=(v0) as u32 as u64;cpu.x[5]=(v1) as u32 as u64;}
// 0x10d37b40 cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d37b44 ldr x2, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10d37b48 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d37b4c cset w3, ne
cpu.x[3]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x10d37b50 mov w1, w20
cpu.x[1]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x10d37b54 mov w6, #-1
cpu.x[6]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10d37b58 mov w7, w22
cpu.x[7]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10d37b5c bl #0x10d388b4
cpu.x[30]=0x10d37b60;super::chunk3::f_10d388b4(cpu,rt,0x10d388b4)?;
pc=0x10d37b60;continue;
}
0x10d37b60=>{
// 0x10d37b60 add x28, x28, #8
{let a=((cpu.x[28]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[28]=(result) as u64;}
// 0x10d37b64 cmp x27, x28
{let a=((cpu.x[27]));let b=((cpu.x[28]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d37b68 b.eq #0x10d37b04
if cpu.condition(0){pc=0x10d37b04;continue;}pc=0x10d37b6c;continue;
}
0x10d37b6c=>{
// 0x10d37b6c ldr x8, [x21, #0x90]
{let address=(cpu.x[21]).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d37b70 ldrb w9, [x21, #0xa0]
{let address=(cpu.x[21]).wrapping_add(0xa0u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10d37b74 ldr x8, [x8, x28]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[28])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d37b78 cbnz w9, #0x10d37b3c
if (cpu.x[9] as u32 as u64) != 0 {pc=0x10d37b3c;continue;}pc=0x10d37b7c;continue;
}
0x10d37b7c=>{
// 0x10d37b7c ldr w10, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10d37b80 cmn w10, #1
{let a=((cpu.x[10] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.flags(a,b,result,32,false);}
// 0x10d37b84 ccmp w10, w20, #4, ne
if cpu.condition(1){{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[20] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}}else{cpu.nzcv=4u64 << 28;}
// 0x10d37b88 b.eq #0x10d37b3c
if cpu.condition(0){pc=0x10d37b3c;continue;}pc=0x10d37b8c;continue;
}
0x10d37b8c=>{
// 0x10d37b8c b #0x10d37b60
pc=0x10d37b60;continue;
}
0x10d37b90=>{
// 0x10d37b90 ldr x9, [x23, #0x18]
{let address=(cpu.x[23]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10d37b94 cbz x9, #0x10d37ba8
if (cpu.x[9]) == 0 {pc=0x10d37ba8;continue;}pc=0x10d37b98;continue;
}
0x10d37b98=>{
// 0x10d37b98 ldr x1, [x9, #0x1a8]
{let address=(cpu.x[9]).wrapping_add(0x1a8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10d37b9c cbz x1, #0x10d37ba8
if (cpu.x[1]) == 0 {pc=0x10d37ba8;continue;}pc=0x10d37ba0;continue;
}
0x10d37ba0=>{
// 0x10d37ba0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d37ba4 bl #0x10d359ac
cpu.x[30]=0x10d37ba8;super::dispatch(cpu,rt,0x10d359ac)?;
pc=0x10d37ba8;continue;
}
0x10d37ba8=>{
// 0x10d37ba8 ldr x8, [x19, #0x1b0]
{let address=(cpu.x[19]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d37bac ldr x0, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d37bb0 cbz x0, #0x10d37db4
if (cpu.x[0]) == 0 {pc=0x10d37db4;continue;}pc=0x10d37bb4;continue;
}
0x10d37bb4=>{
// 0x10d37bb4 bl #0x10e2d5a4
cpu.x[30]=0x10d37bb8;super::chunk4::f_10e2d5a4(cpu,rt,0x10e2d5a4)?;
pc=0x10d37bb8;continue;
}
0x10d37bb8=>{
// 0x10d37bb8 tbz w0, #0, #0x10d37db4
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d37db4;continue;}pc=0x10d37bbc;continue;
}
0x10d37bbc=>{
// 0x10d37bbc mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10d37bc0 ldr x0, [x19, #0x348]
{let address=(cpu.x[19]).wrapping_add(0x348u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d37bc4 strb w8, [x19, #0x1f8]
{let address=(cpu.x[19]).wrapping_add(0x1f8u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10d37bc8 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d37bcc ldr x8, [x8, #0x48]
{let address=(cpu.x[8]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d37bd0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d37bd4;super::dispatch(cpu,rt,target)?;}
// 0x10d37bd4 tbz w0, #0, #0x10d37be8
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d37be8;continue;}pc=0x10d37bd8;continue;
}
0x10d37bd8=>{
// 0x10d37bd8 ldr x8, [x19, #0x1b0]
{let address=(cpu.x[19]).wrapping_add(0x1b0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d37bdc ldr x0, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d37be0 bl #0x10e2d82c
cpu.x[30]=0x10d37be4;super::chunk0::f_10e2d82c(cpu,rt,0x10e2d82c)?;
// 0x10d37be4 tbz w0, #0, #0x10d37d8c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10d37d8c;continue;}pc=0x10d37be8;continue;
}
0x10d37be8=>{
// 0x10d37be8 ldr x0, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d37bec bl #0x10e16208
cpu.x[30]=0x10d37bf0;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
// 0x10d37bf0 add x8, x0, #0x4d0
{let a=((cpu.x[0]));let b=((0x4d0u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10d37bf4 b #0x10d37d98
pc=0x10d37d98;continue;
}
0x10d37bf8=>{
// 0x10d37bf8 ldr x8, [x23, #0x18]
{let address=(cpu.x[23]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d37bfc ldr w9, [x8, #0x23c]
{let address=(cpu.x[8]).wrapping_add(0x23cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
pc=0x10d37c00;continue;
}
0x10d37c00=>{
// 0x10d37c00 ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d37c04 cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d37c08 b.le #0x10d37cd0
if cpu.condition(13){pc=0x10d37cd0;continue;}pc=0x10d37c0c;continue;
}
0x10d37c0c=>{
// 0x10d37c0c cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d37c10 b.lt #0x10d37ab8
if cpu.condition(11){pc=0x10d37ab8;continue;}pc=0x10d37c14;continue;
}
0x10d37c14=>{
// 0x10d37c14 mov w20, wzr
cpu.x[20]=(((0u64))) as u32 as u64;
// 0x10d37c18 b #0x10d37c2c
pc=0x10d37c2c;continue;
}
0x10d37c1c=>{
// 0x10d37c1c ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
pc=0x10d37c20;continue;
}
0x10d37c20=>{
// 0x10d37c20 add w20, w20, #1
{let a=((cpu.x[20] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[20]=(result) as u32 as u64;}
// 0x10d37c24 cmp w20, w8
{let a=((cpu.x[20] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d37c28 b.ge #0x10d37ab8
if cpu.condition(10){pc=0x10d37ab8;continue;}pc=0x10d37c2c;continue;
}
0x10d37c2c=>{
// 0x10d37c2c ldr x24, [x23, #0x18]
{let address=(cpu.x[23]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[24]=(v0) as u64;}
// 0x10d37c30 ldr w25, [x24, #0x23c]
{let address=(cpu.x[24]).wrapping_add(0x23cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[25]=(v0) as u32 as u64;}
// 0x10d37c34 cmp w25, #1
{let a=((cpu.x[25] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d37c38 b.lt #0x10d37c20
if cpu.condition(11){pc=0x10d37c20;continue;}pc=0x10d37c3c;continue;
}
0x10d37c3c=>{
// 0x10d37c3c mov x26, xzr
cpu.x[26]=(((0u64))) as u64;
// 0x10d37c40 b #0x10d37c50
pc=0x10d37c50;continue;
}
0x10d37c44=>{
// 0x10d37c44 add x26, x26, #1
{let a=((cpu.x[26]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[26]=(result) as u64;}
// 0x10d37c48 cmp x26, x25
{let a=((cpu.x[26]));let b=((cpu.x[25]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d37c4c b.eq #0x10d37c1c
if cpu.condition(0){pc=0x10d37c1c;continue;}pc=0x10d37c50;continue;
}
0x10d37c50=>{
// 0x10d37c50 ldr x8, [x24, #0x230]
{let address=(cpu.x[24]).wrapping_add(0x230u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d37c54 ldr x21, [x8, x26, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[26])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10d37c58 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10d37c5c bl #0x10e0b624
cpu.x[30]=0x10d37c60;super::chunk10::f_10e0b624(cpu,rt,0x10e0b624)?;
// 0x10d37c60 ldr w8, [x21, #0x9c]
{let address=(cpu.x[21]).wrapping_add(0x9cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d37c64 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d37c68 b.lt #0x10d37c44
if cpu.condition(11){pc=0x10d37c44;continue;}pc=0x10d37c6c;continue;
}
0x10d37c6c=>{
// 0x10d37c6c lsl x27, x8, #3
cpu.x[27]=(((((cpu.x[8]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10d37c70 mov w22, w0
cpu.x[22]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d37c74 mov x28, xzr
cpu.x[28]=(((0u64))) as u64;
// 0x10d37c78 b #0x10d37cac
pc=0x10d37cac;continue;
}
0x10d37c7c=>{
// 0x10d37c7c ldp w4, w5, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[4]=(v0) as u32 as u64;cpu.x[5]=(v1) as u32 as u64;}
// 0x10d37c80 cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d37c84 ldr x2, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10d37c88 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d37c8c cset w3, ne
cpu.x[3]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x10d37c90 mov w1, w20
cpu.x[1]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x10d37c94 mov w6, #-1
cpu.x[6]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10d37c98 mov w7, w22
cpu.x[7]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10d37c9c bl #0x10d388b4
cpu.x[30]=0x10d37ca0;super::chunk3::f_10d388b4(cpu,rt,0x10d388b4)?;
pc=0x10d37ca0;continue;
}
0x10d37ca0=>{
// 0x10d37ca0 add x28, x28, #8
{let a=((cpu.x[28]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[28]=(result) as u64;}
// 0x10d37ca4 cmp x27, x28
{let a=((cpu.x[27]));let b=((cpu.x[28]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10d37ca8 b.eq #0x10d37c44
if cpu.condition(0){pc=0x10d37c44;continue;}pc=0x10d37cac;continue;
}
0x10d37cac=>{
// 0x10d37cac ldr x8, [x21, #0x90]
{let address=(cpu.x[21]).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d37cb0 ldrb w9, [x21, #0xa0]
{let address=(cpu.x[21]).wrapping_add(0xa0u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10d37cb4 ldr x8, [x8, x28]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[28])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d37cb8 cbnz w9, #0x10d37c7c
if (cpu.x[9] as u32 as u64) != 0 {pc=0x10d37c7c;continue;}pc=0x10d37cbc;continue;
}
0x10d37cbc=>{
// 0x10d37cbc ldr w10, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10d37cc0 cmn w10, #1
{let a=((cpu.x[10] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.flags(a,b,result,32,false);}
// 0x10d37cc4 ccmp w10, w20, #4, ne
if cpu.condition(1){{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[20] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}}else{cpu.nzcv=4u64 << 28;}
// 0x10d37cc8 b.eq #0x10d37c7c
if cpu.condition(0){pc=0x10d37c7c;continue;}pc=0x10d37ccc;continue;
}
0x10d37ccc=>{
// 0x10d37ccc b #0x10d37ca0
pc=0x10d37ca0;continue;
}
0x10d37cd0=>{
// 0x10d37cd0 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d37cd4 b.lt #0x10d37ab8
if cpu.condition(11){pc=0x10d37ab8;continue;}pc=0x10d37cd8;continue;
}
0x10d37cd8=>{
// 0x10d37cd8 mov w20, wzr
cpu.x[20]=(((0u64))) as u32 as u64;
pc=0x10d37cdc;continue;
}
0x10d37cdc=>{
// 0x10d37cdc mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d37ce0 mov w1, w20
cpu.x[1]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x10d37ce4 bl #0x10d38650
cpu.x[30]=0x10d37ce8;super::chunk6::f_10d38650(cpu,rt,0x10d38650)?;
pc=0x10d37ce8;continue;
}
0x10d37ce8=>{
// 0x10d37ce8 ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d37cec add w20, w20, #1
{let a=((cpu.x[20] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[20]=(result) as u32 as u64;}
// 0x10d37cf0 cmp w20, w8
{let a=((cpu.x[20] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d37cf4 b.lt #0x10d37cdc
if cpu.condition(11){pc=0x10d37cdc;continue;}pc=0x10d37cf8;continue;
}
0x10d37cf8=>{
// 0x10d37cf8 cmp w8, #2
{let a=((cpu.x[8] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d37cfc b.le #0x10d37ab8
if cpu.condition(13){pc=0x10d37ab8;continue;}pc=0x10d37d00;continue;
}
0x10d37d00=>{
// 0x10d37d00 ldr x20, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10d37d04 adrp x1, #0x10301000
cpu.x[1]=(0x10301000u64) as u64;
// 0x10d37d08 add x1, x1, #0xe27
{let a=((cpu.x[1]));let b=((0xe27u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x10d37d0c mov x0, sp
cpu.x[0]=(((cpu.sp))) as u64;
// 0x10d37d10 bl #0x1142fde4
cpu.x[30]=0x10d37d14;super::chunk6::f_1142fde4(cpu,rt,0x1142fde4)?;
// 0x10d37d14 mov x1, sp
cpu.x[1]=(((cpu.sp))) as u64;
// 0x10d37d18 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d37d1c mov x2, xzr
cpu.x[2]=(((0u64))) as u64;
// 0x10d37d20 bl #0x10e191a4
cpu.x[30]=0x10d37d24;super::dispatch(cpu,rt,0x10e191a4)?;
// 0x10d37d24 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10d37d28 mov x0, sp
cpu.x[0]=(((cpu.sp))) as u64;
// 0x10d37d2c bl #0x1142fcc4
cpu.x[30]=0x10d37d30;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
// 0x10d37d30 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d37d34 bl #0x10e0b624
cpu.x[30]=0x10d37d38;super::chunk10::f_10e0b624(cpu,rt,0x10e0b624)?;
// 0x10d37d38 mov w7, w0
cpu.x[7]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d37d3c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d37d40 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x10d37d44 mov x2, x20
cpu.x[2]=(((cpu.x[20]))) as u64;
// 0x10d37d48 mov w3, #1
cpu.x[3]=(((0x1u64))) as u32 as u64;
// 0x10d37d4c mov w4, #0x12
cpu.x[4]=(((0x12u64))) as u32 as u64;
// 0x10d37d50 mov w5, #6
cpu.x[5]=(((0x6u64))) as u32 as u64;
// 0x10d37d54 mov w6, #-1
cpu.x[6]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10d37d58 bl #0x10d388b4
cpu.x[30]=0x10d37d5c;super::chunk3::f_10d388b4(cpu,rt,0x10d388b4)?;
// 0x10d37d5c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10d37d60 bl #0x10e0b624
cpu.x[30]=0x10d37d64;super::chunk10::f_10e0b624(cpu,rt,0x10e0b624)?;
// 0x10d37d64 mov w7, w0
cpu.x[7]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10d37d68 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d37d6c mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10d37d70 mov x2, x20
cpu.x[2]=(((cpu.x[20]))) as u64;
// 0x10d37d74 mov w3, #1
cpu.x[3]=(((0x1u64))) as u32 as u64;
// 0x10d37d78 mov w4, #0x12
cpu.x[4]=(((0x12u64))) as u32 as u64;
// 0x10d37d7c mov w5, #6
cpu.x[5]=(((0x6u64))) as u32 as u64;
// 0x10d37d80 mov w6, #-1
cpu.x[6]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10d37d84 bl #0x10d388b4
cpu.x[30]=0x10d37d88;super::chunk3::f_10d388b4(cpu,rt,0x10d388b4)?;
// 0x10d37d88 b #0x10d37ab8
pc=0x10d37ab8;continue;
}
0x10d37d8c=>{
// 0x10d37d8c ldr x0, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d37d90 bl #0x10e16208
cpu.x[30]=0x10d37d94;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
// 0x10d37d94 add x8, x0, #0x4d4
{let a=((cpu.x[0]));let b=((0x4d4u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
pc=0x10d37d98;continue;
}
0x10d37d98=>{
// 0x10d37d98 ldr x0, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d37d9c ldr w20, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[20]=(v0) as u32 as u64;}
// 0x10d37da0 bl #0x10e16208
cpu.x[30]=0x10d37da4;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
// 0x10d37da4 ldr w2, [x0, #0x4dc]
{let address=(cpu.x[0]).wrapping_add(0x4dcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[2]=(v0) as u32 as u64;}
// 0x10d37da8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d37dac mov w1, w20
cpu.x[1]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x10d37db0 bl #0x10d36174
cpu.x[30]=0x10d37db4;super::dispatch(cpu,rt,0x10d36174)?;
pc=0x10d37db4;continue;
}
0x10d37db4=>{
// 0x10d37db4 ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d37db8 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d37dbc b.lt #0x10d37de0
if cpu.condition(11){pc=0x10d37de0;continue;}pc=0x10d37dc0;continue;
}
0x10d37dc0=>{
// 0x10d37dc0 mov w20, wzr
cpu.x[20]=(((0u64))) as u32 as u64;
pc=0x10d37dc4;continue;
}
0x10d37dc4=>{
// 0x10d37dc4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d37dc8 mov w1, w20
cpu.x[1]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x10d37dcc bl #0x10d37038
cpu.x[30]=0x10d37dd0;super::chunk15::f_10d37038(cpu,rt,0x10d37038)?;
pc=0x10d37dd0;continue;
}
0x10d37dd0=>{
// 0x10d37dd0 ldr w8, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d37dd4 add w20, w20, #1
{let a=((cpu.x[20] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[20]=(result) as u32 as u64;}
// 0x10d37dd8 cmp w20, w8
{let a=((cpu.x[20] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d37ddc b.lt #0x10d37dc4
if cpu.condition(11){pc=0x10d37dc4;continue;}pc=0x10d37de0;continue;
}
0x10d37de0=>{
// 0x10d37de0 ldr x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d37de4 bl #0x110c99ac
cpu.x[30]=0x10d37de8;super::chunk12::f_110c99ac(cpu,rt,0x110c99ac)?;
pc=0x10d37de8;continue;
}
0x10d37de8=>{
// 0x10d37de8 ldp x20, x19, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d37dec ldp x22, x21, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10d37df0 ldp x24, x23, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10d37df4 ldp x26, x25, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10d37df8 ldp x28, x27, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10d37dfc ldp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10d37e00 add sp, sp, #0x70
{let a=((cpu.sp));let b=((0x70u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10d37e04 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d378d8")),}}}
#[inline]
pub(super) unsafe fn f_10d3ce44(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d3ce44=>{
// 0x10d3ce44 sub sp, sp, #0x40
{let a=((cpu.sp));let b=((0x40u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x10d3ce48 stp x29, x30, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d3ce4c str x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d3ce50 add x29, sp, #0x20
{let a=((cpu.sp));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x10d3ce54 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10d3ce58 add x0, sp, #8
{let a=((cpu.sp));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d3ce5c bl #0x10d5a430
cpu.x[30]=0x10d3ce60;super::chunk13::f_10d5a430(cpu,rt,0x10d5a430)?;
pc=0x10d3ce60;continue;
}
0x10d3ce60=>{
// 0x10d3ce60 ldr x0, [x19, #0x330]
{let address=(cpu.x[19]).wrapping_add(0x330u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10d3ce64 add x1, sp, #8
{let a=((cpu.sp));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x10d3ce68 mov w2, #-1
cpu.x[2]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10d3ce6c bl #0x111fac10
cpu.x[30]=0x10d3ce70;super::chunk10::f_111fac10(cpu,rt,0x111fac10)?;
pc=0x10d3ce70;continue;
}
0x10d3ce70=>{
// 0x10d3ce70 add x0, sp, #8
{let a=((cpu.sp));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10d3ce74 bl #0x10d5a450
cpu.x[30]=0x10d3ce78;super::chunk10::f_10d5a450(cpu,rt,0x10d5a450)?;
pc=0x10d3ce78;continue;
}
0x10d3ce78=>{
// 0x10d3ce78 ldp x29, x30, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10d3ce7c ldr x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10d3ce80 add sp, sp, #0x40
{let a=((cpu.sp));let b=((0x40u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10d3ce84 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d3ce44")),}}}
#[inline]
pub(super) unsafe fn f_10d5402c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d5402c=>{
// 0x10d5402c movi v0.2d, #0000000000000000
cpu.q[0]=(lane_dup(0u128,64,2)) & 0xffffffffffffffffffffffffffffffffu128;
// 0x10d54030 str x1, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=cpu.x[1];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d54034 stur q0, [x0, #0x18]
{let address=(cpu.x[0]).wrapping_add(0x18u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d54038 stur q0, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10d5403c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d5402c")),}}}
#[inline]
pub(super) unsafe fn f_10d557a0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d557a0=>{
// 0x10d557a0 b #0x1187d2c0
super::dispatch(cpu,rt,0x1187d2c0)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d557a0")),}}}
#[inline]
pub(super) unsafe fn f_10d565a8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d565a8=>{
// 0x10d565a8 ldr w8, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d565ac bic w0, w8, w8, asr #31
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[8] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[0]=(result) as u32 as u64;}
// 0x10d565b0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d565a8")),}}}
#[inline]
pub(super) unsafe fn f_10d56610(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d56610=>{
// 0x10d56610 ldr w9, [x0, #0x28]
{let address=(cpu.x[0]).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10d56614 mov w8, wzr
cpu.x[8]=(((0u64))) as u32 as u64;
// 0x10d56618 cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d5661c b.lt #0x10d56644
if cpu.condition(11){pc=0x10d56644;continue;}pc=0x10d56620;continue;
}
0x10d56620=>{
// 0x10d56620 ldr w10, [x0, #0x24]
{let address=(cpu.x[0]).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10d56624 cmp w9, w10
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d56628 b.gt #0x10d56644
if cpu.condition(12){pc=0x10d56644;continue;}pc=0x10d5662c;continue;
}
0x10d5662c=>{
// 0x10d5662c ldr x8, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d56630 ldr w9, [x0, #0x20]
{let address=(cpu.x[0]).wrapping_add(0x20u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10d56634 ldr w8, [x8, #0x74]
{let address=(cpu.x[8]).wrapping_add(0x74u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10d56638 sub w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10d5663c cmp w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d56640 cset w8, ge
cpu.x[8]=((cpu.condition(10)) as u64) as u32 as u64;
pc=0x10d56644;continue;
}
0x10d56644=>{
// 0x10d56644 mov w0, w8
cpu.x[0]=(((cpu.x[8] as u32 as u64))) as u32 as u64;
// 0x10d56648 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d56610")),}}}
#[inline]
pub(super) unsafe fn f_10d5905c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d5905c=>{
// 0x10d5905c ldr w0, [x0, #0x18]
{let address=(cpu.x[0]).wrapping_add(0x18u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10d59060 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d5905c")),}}}
#[inline]
pub(super) unsafe fn f_10d5cf10(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d5cf10=>{
// 0x10d5cf10 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10d5cf14 adrp x1, #0x10316000
cpu.x[1]=(0x10316000u64) as u64;
// 0x10d5cf18 add x1, x1, #0xb77
{let a=((cpu.x[1]));let b=((0xb77u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x10d5cf1c b #0x1142fde4
super::chunk6::f_1142fde4(cpu,rt,0x1142fde4)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d5cf10")),}}}
#[inline]
pub(super) unsafe fn f_10d72c08(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d72c08=>{
// 0x10d72c08 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d72c0c str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10d72c10 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d72c14 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d72c18 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10d72c1c ldr x8, [x8, #0x20]
{let address=(cpu.x[8]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d72c20 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d72c24;super::dispatch(cpu,rt,target)?;}
pc=0x10d72c24;continue;
}
0x10d72c24=>{
// 0x10d72c24 cmp w0, #0xc8
{let a=((cpu.x[0] as u32 as u64));let b=((0xc8u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d72c28 b.lt #0x10d72c50
if cpu.condition(11){pc=0x10d72c50;continue;}pc=0x10d72c2c;continue;
}
0x10d72c2c=>{
// 0x10d72c2c ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d72c30 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d72c34 ldr x8, [x8, #0x20]
{let address=(cpu.x[8]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d72c38 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d72c3c;super::dispatch(cpu,rt,target)?;}
// 0x10d72c3c cmp w0, #0x1f4
{let a=((cpu.x[0] as u32 as u64));let b=((0x1f4u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10d72c40 cset w0, lt
cpu.x[0]=((cpu.condition(11)) as u64) as u32 as u64;
// 0x10d72c44 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10d72c48 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10d72c4c ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10d72c50=>{
// 0x10d72c50 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10d72c54 ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10d72c58 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10d72c5c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d72c08")),}}}
#[inline]
pub(super) unsafe fn f_10d797c4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10d797c4=>{
// 0x10d797c4 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10d797c8 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10d797cc mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10d797d0 mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10d797d4 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10d797d8 bl #0x10d727d4
cpu.x[30]=0x10d797dc;super::chunk7::f_10d727d4(cpu,rt,0x10d727d4)?;
pc=0x10d797dc;continue;
}
0x10d797dc=>{
// 0x10d797dc ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d797e0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d797e4 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d797e8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d797ec;super::dispatch(cpu,rt,target)?;}
pc=0x10d797ec;continue;
}
0x10d797ec=>{
// 0x10d797ec ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d797f0 str w0, [x20, #0x2c]
{let address=(cpu.x[20]).wrapping_add(0x2cu64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d797f4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d797f8 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d797fc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d79800;super::dispatch(cpu,rt,target)?;}
pc=0x10d79800;continue;
}
0x10d79800=>{
// 0x10d79800 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d79804 str w0, [x20, #0x34]
{let address=(cpu.x[20]).wrapping_add(0x34u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d79808 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d7980c ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d79810 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d79814;super::dispatch(cpu,rt,target)?;}
pc=0x10d79814;continue;
}
0x10d79814=>{
// 0x10d79814 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d79818 str w0, [x20, #0x28]
{let address=(cpu.x[20]).wrapping_add(0x28u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d7981c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10d79820 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10d79824 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10d79828;super::dispatch(cpu,rt,target)?;}
pc=0x10d79828;continue;
}
0x10d79828=>{
// 0x10d79828 str w0, [x20, #0x30]
{let address=(cpu.x[20]).wrapping_add(0x30u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10d7982c ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10d79830 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10d79834 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10d797c4")),}}}
#[inline]
pub(super) unsafe fn f_10daaec0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10daaec0=>{
// 0x10daaec0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10daaec0")),}}}
#[inline]
pub(super) unsafe fn f_10dc1180(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10dc1180=>{
// 0x10dc1180 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10dc1180")),}}}
#[inline]
pub(super) unsafe fn f_10de0698(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10de0698=>{
// 0x10de0698 ldr w0, [x0, #0x46c]
{let address=(cpu.x[0]).wrapping_add(0x46cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10de069c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10de0698")),}}}
#[inline]
pub(super) unsafe fn f_10de06a0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10de06a0=>{
// 0x10de06a0 ldrb w0, [x0, #0x57b]
{let address=(cpu.x[0]).wrapping_add(0x57bu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10de06a4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10de06a0")),}}}
#[inline]
pub(super) unsafe fn f_10de1250(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10de1250=>{
// 0x10de1250 ldrb w0, [x0, #0x3dc]
{let address=(cpu.x[0]).wrapping_add(0x3dcu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10de1254 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10de1250")),}}}
#[inline]
pub(super) unsafe fn f_10e16538(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e16538=>{
// 0x10e16538 ldr x0, [x0, #0x1010]
{let address=(cpu.x[0]).wrapping_add(0x1010u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10e1653c ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e16540 ldr x2, [x8, #0x48]
{let address=(cpu.x[8]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10e16544 br x2
{let target=cpu.x[2];if target>=0x10e16538 && target<0x10e16548{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e16538")),}}}
#[inline]
pub(super) unsafe fn f_10e167ac(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e167ac=>{
// 0x10e167ac ldr x0, [x0, #0x7f8]
{let address=(cpu.x[0]).wrapping_add(0x7f8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10e167b0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e167ac")),}}}
#[inline]
pub(super) unsafe fn f_10e19dc0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e19dc0=>{
// 0x10e19dc0 ldr x0, [x0, #0xec0]
{let address=(cpu.x[0]).wrapping_add(0xec0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10e19dc4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e19dc0")),}}}
#[inline]
pub(super) unsafe fn f_10e1c95c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e1c95c=>{
// 0x10e1c95c ldr x0, [x0, #0x1088]
{let address=(cpu.x[0]).wrapping_add(0x1088u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10e1c960 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e1c95c")),}}}
#[inline]
pub(super) unsafe fn f_10e2d5b4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e2d5b4=>{
// 0x10e2d5b4 stp x29, x30, [sp, #-0x10]!
{let address=(cpu.sp).wrapping_add(0xfffffffffffffff0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10e2d5b8 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10e2d5bc bl #0x10e0b6a0
cpu.x[30]=0x10e2d5c0;super::chunk0::f_10e0b6a0(cpu,rt,0x10e0b6a0)?;
pc=0x10e2d5c0;continue;
}
0x10e2d5c0=>{
// 0x10e2d5c0 adrp x1, #0x10302000
cpu.x[1]=(0x10302000u64) as u64;
// 0x10e2d5c4 add x1, x1, #0x60c
{let a=((cpu.x[1]));let b=((0x60cu64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x10e2d5c8 ldp x29, x30, [sp], #0x10
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x10u64)))) as u64;}
// 0x10e2d5cc b #0x114325a4
super::chunk14::f_114325a4(cpu,rt,0x114325a4)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e2d5b4")),}}}
#[inline]
pub(super) unsafe fn f_10e48ce4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e48ce4=>{
// 0x10e48ce4 adrp x8, #0x11986000
cpu.x[8]=(0x11986000u64) as u64;
// 0x10e48ce8 ldr x0, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10e48cec ldr w1, [x8, #0xd4c]
{let address=(cpu.x[8]).wrapping_add(0xd4cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10e48cf0 b #0x11289ec4
super::chunk5::f_11289ec4(cpu,rt,0x11289ec4)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e48ce4")),}}}
#[inline]
pub(super) unsafe fn f_10e5ae54(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e5ae54=>{
// 0x10e5ae54 ldr w8, [x0, #0x2d0]
{let address=(cpu.x[0]).wrapping_add(0x2d0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e5ae58 cmp w8, #2
{let a=((cpu.x[8] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e5ae5c b.hi #0x10e5aea8
if cpu.condition(8){pc=0x10e5aea8;continue;}pc=0x10e5ae60;continue;
}
0x10e5ae60=>{
// 0x10e5ae60 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10e5ae64 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10e5ae68 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10e5ae6c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10e5ae70 mov w9, #1
cpu.x[9]=(((0x1u64))) as u32 as u64;
// 0x10e5ae74 mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10e5ae78 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10e5ae7c orr w20, w9, w8, lsl #1
{let a=((cpu.x[9] as u32 as u64));let b=(((cpu.x[8] as u32 as u64)) as u32).wrapping_shl(1) as u64;let result=a | b;cpu.x[20]=(result) as u32 as u64;}
// 0x10e5ae80 bl #0x10e0b6b4
cpu.x[30]=0x10e5ae84;super::chunk8::f_10e0b6b4(cpu,rt,0x10e0b6b4)?;
pc=0x10e5ae84;continue;
}
0x10e5ae84=>{
// 0x10e5ae84 ldr x0, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10e5ae88 ldr w1, [x21, #0x2a4]
{let address=(cpu.x[21]).wrapping_add(0x2a4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10e5ae8c and x2, x19, #0xffffffff
{let a=((cpu.x[19]));let b=((0xffffffffu64));let result=a & b;cpu.x[2]=(result) as u64;}
// 0x10e5ae90 ldr x4, [x21, #0x1a8]
{let address=(cpu.x[21]).wrapping_add(0x1a8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[4]=(v0) as u64;}
// 0x10e5ae94 mov w3, w20
cpu.x[3]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x10e5ae98 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10e5ae9c ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10e5aea0 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10e5aea4 b #0x10e7e418
super::chunk9::f_10e7e418(cpu,rt,0x10e7e418)?;return Ok(());
}
0x10e5aea8=>{
// 0x10e5aea8 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10e5aeac ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e5ae54")),}}}
#[inline]
pub(super) unsafe fn f_10e5b04c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e5b04c=>{
// 0x10e5b04c ldr x0, [x0, #0xf8]
{let address=(cpu.x[0]).wrapping_add(0xf8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10e5b050 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e5b04c")),}}}
#[inline]
pub(super) unsafe fn f_10e7e418(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e7e418=>{
// 0x10e7e418 stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10e7e41c stp x24, x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10e7e420 stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10e7e424 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10e7e428 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10e7e42c sxtb w23, w2
cpu.x[23]=((((cpu.x[2] as u32 as u64))) as i8 as i64 as u64) as u32 as u64;
// 0x10e7e430 mov x22, x4
cpu.x[22]=(((cpu.x[4]))) as u64;
// 0x10e7e434 mov w21, w3
cpu.x[21]=(((cpu.x[3] as u32 as u64))) as u32 as u64;
// 0x10e7e438 mov w19, w1
cpu.x[19]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x10e7e43c mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10e7e440 cbz x4, #0x10e7e478
if (cpu.x[4]) == 0 {pc=0x10e7e478;continue;}pc=0x10e7e444;continue;
}
0x10e7e444=>{
// 0x10e7e444 sub w8, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10e7e448 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e7e44c b.hi #0x10e7e478
if cpu.condition(8){pc=0x10e7e478;continue;}pc=0x10e7e450;continue;
}
0x10e7e450=>{
// 0x10e7e450 lsl w8, w2, #0x18
cpu.x[8]=(((((cpu.x[2] as u32 as u64))) as u32).wrapping_shl(((((0x18u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10e7e454 cbz w8, #0x10e7e618
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10e7e618;continue;}pc=0x10e7e458;continue;
}
0x10e7e458=>{
// 0x10e7e458 sub w1, w23, #1
{let a=((cpu.x[23] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[1]=(result) as u32 as u64;}
// 0x10e7e45c mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10e7e460 bl #0x10e612f4
cpu.x[30]=0x10e7e464;super::chunk13::f_10e612f4(cpu,rt,0x10e612f4)?;
pc=0x10e7e464;continue;
}
0x10e7e464=>{
// 0x10e7e464 mul w9, w0, w19
cpu.x[9]=((((cpu.x[0] as u32 as u64))).wrapping_mul(((cpu.x[19] as u32 as u64)))) as u32 as u64;
// 0x10e7e468 mov w8, #0x851f
cpu.x[8]=(((0x851fu64))) as u32 as u64;
// 0x10e7e46c movk w8, #0x51eb, lsl #16
cpu.x[8]=((cpu.x[8] as u32 as u64 & !(0xffff0000u64)) | 0x51eb0000u64) as u32 as u64;
// 0x10e7e470 smull x8, w9, w8
cpu.x[8]=(((((cpu.x[9] as u32 as u64))) as i32 as i64).wrapping_mul((((cpu.x[8] as u32 as u64))) as i32 as i64) as u64) as u64;
// 0x10e7e474 b #0x10e7e60c
pc=0x10e7e60c;continue;
}
0x10e7e478=>{
// 0x10e7e478 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10e7e47c bl #0x10e16208
cpu.x[30]=0x10e7e480;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
pc=0x10e7e480;continue;
}
0x10e7e480=>{
// 0x10e7e480 ldr w24, [x0, #0x358]
{let address=(cpu.x[0]).wrapping_add(0x358u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[24]=(v0) as u32 as u64;}
// 0x10e7e484 cbz x22, #0x10e7e49c
if (cpu.x[22]) == 0 {pc=0x10e7e49c;continue;}pc=0x10e7e488;continue;
}
0x10e7e488=>{
// 0x10e7e488 ldr x8, [x22]
{let address=(cpu.x[22]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e7e48c mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10e7e490 ldr x8, [x8, #0x118]
{let address=(cpu.x[8]).wrapping_add(0x118u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e7e494 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10e7e498;super::dispatch(cpu,rt,target)?;}
pc=0x10e7e498;continue;
}
0x10e7e498=>{
// 0x10e7e498 sub w24, w24, w0
{let a=((cpu.x[24] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[24]=(result) as u32 as u64;}
pc=0x10e7e49c;continue;
}
0x10e7e49c=>{
// 0x10e7e49c cmp w21, #4
{let a=((cpu.x[21] as u32 as u64));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e7e4a0 mov w22, wzr
cpu.x[22]=(((0u64))) as u32 as u64;
// 0x10e7e4a4 b.gt #0x10e7e4f0
if cpu.condition(12){pc=0x10e7e4f0;continue;}pc=0x10e7e4a8;continue;
}
0x10e7e4a8=>{
// 0x10e7e4a8 cmp w21, #3
{let a=((cpu.x[21] as u32 as u64));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e7e4ac b.eq #0x10e7e538
if cpu.condition(0){pc=0x10e7e538;continue;}pc=0x10e7e4b0;continue;
}
0x10e7e4b0=>{
// 0x10e7e4b0 cmp w21, #4
{let a=((cpu.x[21] as u32 as u64));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e7e4b4 mov w21, w22
cpu.x[21]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10e7e4b8 mov w8, w22
cpu.x[8]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10e7e4bc b.ne #0x10e7e55c
if cpu.condition(1){pc=0x10e7e55c;continue;}pc=0x10e7e4c0;continue;
}
0x10e7e4c0=>{
// 0x10e7e4c0 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10e7e4c4 bl #0x10e16208
cpu.x[30]=0x10e7e4c8;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
pc=0x10e7e4c8;continue;
}
0x10e7e4c8=>{
// 0x10e7e4c8 ldr w22, [x0, #0x1e0]
{let address=(cpu.x[0]).wrapping_add(0x1e0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[22]=(v0) as u32 as u64;}
// 0x10e7e4cc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10e7e4d0 bl #0x10e16208
cpu.x[30]=0x10e7e4d4;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
pc=0x10e7e4d4;continue;
}
0x10e7e4d4=>{
// 0x10e7e4d4 ldr w21, [x0, #0x1e8]
{let address=(cpu.x[0]).wrapping_add(0x1e8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[21]=(v0) as u32 as u64;}
// 0x10e7e4d8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10e7e4dc bl #0x10e16208
cpu.x[30]=0x10e7e4e0;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
pc=0x10e7e4e0;continue;
}
0x10e7e4e0=>{
// 0x10e7e4e0 ldr w8, [x0, #0x1f0]
{let address=(cpu.x[0]).wrapping_add(0x1f0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e7e4e4 cmp w23, #1
{let a=((cpu.x[23] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e7e4e8 b.ge #0x10e7e564
if cpu.condition(10){pc=0x10e7e564;continue;}pc=0x10e7e4ec;continue;
}
0x10e7e4ec=>{
// 0x10e7e4ec b #0x10e7e5f8
pc=0x10e7e5f8;continue;
}
0x10e7e4f0=>{
// 0x10e7e4f0 cmp w21, #5
{let a=((cpu.x[21] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e7e4f4 b.eq #0x10e7e5cc
if cpu.condition(0){pc=0x10e7e5cc;continue;}pc=0x10e7e4f8;continue;
}
0x10e7e4f8=>{
// 0x10e7e4f8 cmp w21, #6
{let a=((cpu.x[21] as u32 as u64));let b=((0x6u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e7e4fc mov w21, w22
cpu.x[21]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10e7e500 mov w8, w22
cpu.x[8]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10e7e504 b.ne #0x10e7e55c
if cpu.condition(1){pc=0x10e7e55c;continue;}pc=0x10e7e508;continue;
}
0x10e7e508=>{
// 0x10e7e508 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10e7e50c bl #0x10e16208
cpu.x[30]=0x10e7e510;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
pc=0x10e7e510;continue;
}
0x10e7e510=>{
// 0x10e7e510 ldr w22, [x0, #0x1f8]
{let address=(cpu.x[0]).wrapping_add(0x1f8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[22]=(v0) as u32 as u64;}
// 0x10e7e514 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10e7e518 bl #0x10e16208
cpu.x[30]=0x10e7e51c;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
pc=0x10e7e51c;continue;
}
0x10e7e51c=>{
// 0x10e7e51c ldr w21, [x0, #0x200]
{let address=(cpu.x[0]).wrapping_add(0x200u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[21]=(v0) as u32 as u64;}
// 0x10e7e520 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10e7e524 bl #0x10e16208
cpu.x[30]=0x10e7e528;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
pc=0x10e7e528;continue;
}
0x10e7e528=>{
// 0x10e7e528 ldr w8, [x0, #0x208]
{let address=(cpu.x[0]).wrapping_add(0x208u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e7e52c cmp w23, #1
{let a=((cpu.x[23] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e7e530 b.ge #0x10e7e564
if cpu.condition(10){pc=0x10e7e564;continue;}pc=0x10e7e534;continue;
}
0x10e7e534=>{
// 0x10e7e534 b #0x10e7e5f8
pc=0x10e7e5f8;continue;
}
0x10e7e538=>{
// 0x10e7e538 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10e7e53c bl #0x10e16208
cpu.x[30]=0x10e7e540;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
pc=0x10e7e540;continue;
}
0x10e7e540=>{
// 0x10e7e540 ldr w22, [x0, #0x1dc]
{let address=(cpu.x[0]).wrapping_add(0x1dcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[22]=(v0) as u32 as u64;}
// 0x10e7e544 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10e7e548 bl #0x10e16208
cpu.x[30]=0x10e7e54c;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
pc=0x10e7e54c;continue;
}
0x10e7e54c=>{
// 0x10e7e54c ldr w21, [x0, #0x1e4]
{let address=(cpu.x[0]).wrapping_add(0x1e4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[21]=(v0) as u32 as u64;}
// 0x10e7e550 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10e7e554 bl #0x10e16208
cpu.x[30]=0x10e7e558;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
pc=0x10e7e558;continue;
}
0x10e7e558=>{
// 0x10e7e558 ldr w8, [x0, #0x1ec]
{let address=(cpu.x[0]).wrapping_add(0x1ecu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
pc=0x10e7e55c;continue;
}
0x10e7e55c=>{
// 0x10e7e55c cmp w23, #1
{let a=((cpu.x[23] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e7e560 b.lt #0x10e7e5f8
if cpu.condition(11){pc=0x10e7e5f8;continue;}pc=0x10e7e564;continue;
}
0x10e7e564=>{
// 0x10e7e564 mov w12, #0x86a0
cpu.x[12]=(((0x86a0u64))) as u32 as u64;
// 0x10e7e568 mov w13, #0x851f
cpu.x[13]=(((0x851fu64))) as u32 as u64;
// 0x10e7e56c mov w9, wzr
cpu.x[9]=(((0u64))) as u32 as u64;
// 0x10e7e570 sub w11, w24, #1
{let a=((cpu.x[24] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[11]=(result) as u32 as u64;}
// 0x10e7e574 mov w10, #0x64
cpu.x[10]=(((0x64u64))) as u32 as u64;
// 0x10e7e578 movk w12, #1, lsl #16
cpu.x[12]=((cpu.x[12] as u32 as u64 & !(0xffff0000u64)) | 0x10000u64) as u32 as u64;
// 0x10e7e57c movk w13, #0x51eb, lsl #16
cpu.x[13]=((cpu.x[13] as u32 as u64 & !(0xffff0000u64)) | 0x51eb0000u64) as u32 as u64;
pc=0x10e7e580;continue;
}
0x10e7e580=>{
// 0x10e7e580 cmp w9, w24
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[24] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e7e584 umull x16, w10, w13
cpu.x[16]=(((((cpu.x[10] as u32 as u64))) as u32 as u64).wrapping_mul((((cpu.x[13] as u32 as u64))) as u32 as u64) as u64) as u64;
// 0x10e7e588 csel w14, w22, w21, lt
cpu.x[14]=(if cpu.condition(11){((cpu.x[22] as u32 as u64))}else{((cpu.x[21] as u32 as u64))}) as u32 as u64;
// 0x10e7e58c cmp w11, w9
{let a=((cpu.x[11] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e7e590 add w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10e7e594 csel w22, w8, w14, eq
cpu.x[22]=(if cpu.condition(0){((cpu.x[8] as u32 as u64))}else{((cpu.x[14] as u32 as u64))}) as u32 as u64;
// 0x10e7e598 cmp w10, w12
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[12] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e7e59c add w14, w22, #0x64
{let a=((cpu.x[22] as u32 as u64));let b=((0x64u64));let result=a.wrapping_add(b);cpu.x[14]=(result) as u32 as u64;}
// 0x10e7e5a0 lsr x16, x16, #0x25
cpu.x[16]=(((((cpu.x[16]))) as u64).wrapping_shr(((((0x25u64))) & 63) as u32) as u64) as u64;
// 0x10e7e5a4 mul w15, w14, w10
cpu.x[15]=((((cpu.x[14] as u32 as u64))).wrapping_mul(((cpu.x[10] as u32 as u64)))) as u32 as u64;
// 0x10e7e5a8 mul w14, w14, w16
cpu.x[14]=((((cpu.x[14] as u32 as u64))).wrapping_mul(((cpu.x[16] as u32 as u64)))) as u32 as u64;
// 0x10e7e5ac smull x15, w15, w13
cpu.x[15]=(((((cpu.x[15] as u32 as u64))) as i32 as i64).wrapping_mul((((cpu.x[13] as u32 as u64))) as i32 as i64) as u64) as u64;
// 0x10e7e5b0 lsr x16, x15, #0x3f
cpu.x[16]=(((((cpu.x[15]))) as u64).wrapping_shr(((((0x3fu64))) & 63) as u32) as u64) as u64;
// 0x10e7e5b4 asr x15, x15, #0x25
cpu.x[15]=(((((cpu.x[15]))) as i64).wrapping_shr(((((0x25u64))) & 63) as u32) as u64) as u64;
// 0x10e7e5b8 add w10, w15, w16
{let a=((cpu.x[15] as u32 as u64));let b=((cpu.x[16] as u32 as u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10e7e5bc csel w10, w10, w14, lt
cpu.x[10]=(if cpu.condition(11){((cpu.x[10] as u32 as u64))}else{((cpu.x[14] as u32 as u64))}) as u32 as u64;
// 0x10e7e5c0 cmp w23, w9
{let a=((cpu.x[23] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e7e5c4 b.ne #0x10e7e580
if cpu.condition(1){pc=0x10e7e580;continue;}pc=0x10e7e5c8;continue;
}
0x10e7e5c8=>{
// 0x10e7e5c8 b #0x10e7e5fc
pc=0x10e7e5fc;continue;
}
0x10e7e5cc=>{
// 0x10e7e5cc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10e7e5d0 bl #0x10e16208
cpu.x[30]=0x10e7e5d4;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
pc=0x10e7e5d4;continue;
}
0x10e7e5d4=>{
// 0x10e7e5d4 ldr w22, [x0, #0x1f4]
{let address=(cpu.x[0]).wrapping_add(0x1f4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[22]=(v0) as u32 as u64;}
// 0x10e7e5d8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10e7e5dc bl #0x10e16208
cpu.x[30]=0x10e7e5e0;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
pc=0x10e7e5e0;continue;
}
0x10e7e5e0=>{
// 0x10e7e5e0 ldr w21, [x0, #0x1fc]
{let address=(cpu.x[0]).wrapping_add(0x1fcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[21]=(v0) as u32 as u64;}
// 0x10e7e5e4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10e7e5e8 bl #0x10e16208
cpu.x[30]=0x10e7e5ec;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
pc=0x10e7e5ec;continue;
}
0x10e7e5ec=>{
// 0x10e7e5ec ldr w8, [x0, #0x204]
{let address=(cpu.x[0]).wrapping_add(0x204u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e7e5f0 cmp w23, #1
{let a=((cpu.x[23] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10e7e5f4 b.ge #0x10e7e564
if cpu.condition(10){pc=0x10e7e564;continue;}pc=0x10e7e5f8;continue;
}
0x10e7e5f8=>{
// 0x10e7e5f8 mov w10, #0x64
cpu.x[10]=(((0x64u64))) as u32 as u64;
pc=0x10e7e5fc;continue;
}
0x10e7e5fc=>{
// 0x10e7e5fc mul w8, w10, w19
cpu.x[8]=((((cpu.x[10] as u32 as u64))).wrapping_mul(((cpu.x[19] as u32 as u64)))) as u32 as u64;
// 0x10e7e600 mov w9, #0x851f
cpu.x[9]=(((0x851fu64))) as u32 as u64;
// 0x10e7e604 movk w9, #0x51eb, lsl #16
cpu.x[9]=((cpu.x[9] as u32 as u64 & !(0xffff0000u64)) | 0x51eb0000u64) as u32 as u64;
// 0x10e7e608 smull x8, w8, w9
cpu.x[8]=(((((cpu.x[8] as u32 as u64))) as i32 as i64).wrapping_mul((((cpu.x[9] as u32 as u64))) as i32 as i64) as u64) as u64;
pc=0x10e7e60c;continue;
}
0x10e7e60c=>{
// 0x10e7e60c lsr x9, x8, #0x3f
cpu.x[9]=(((((cpu.x[8]))) as u64).wrapping_shr(((((0x3fu64))) & 63) as u32) as u64) as u64;
// 0x10e7e610 asr x8, x8, #0x25
cpu.x[8]=(((((cpu.x[8]))) as i64).wrapping_shr(((((0x25u64))) & 63) as u32) as u64) as u64;
// 0x10e7e614 add w19, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_add(b);cpu.x[19]=(result) as u32 as u64;}
pc=0x10e7e618;continue;
}
0x10e7e618=>{
// 0x10e7e618 mov w0, w19
cpu.x[0]=(((cpu.x[19] as u32 as u64))) as u32 as u64;
// 0x10e7e61c ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10e7e620 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10e7e624 ldp x24, x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10e7e628 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10e7e62c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e7e418")),}}}
#[inline]
pub(super) unsafe fn f_10e7e830(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e7e830=>{
// 0x10e7e830 ldr x0, [x0, #0xb0]
{let address=(cpu.x[0]).wrapping_add(0xb0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10e7e834 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e7e830")),}}}
#[inline]
pub(super) unsafe fn f_10e7e8d4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e7e8d4=>{
// 0x10e7e8d4 ldp x8, x10, [x0, #0xa8]
{let address=(cpu.x[0]).wrapping_add(0xa8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[8]=(v0) as u64;cpu.x[10]=(v1) as u64;}
// 0x10e7e8d8 ldr w9, [x0, #0x150]
{let address=(cpu.x[0]).wrapping_add(0x150u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10e7e8dc cmp x8, #0
{let a=((cpu.x[8]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10e7e8e0 ldr w8, [x0, #0x164]
{let address=(cpu.x[0]).wrapping_add(0x164u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e7e8e4 csel w9, wzr, w9, eq
cpu.x[9]=(if cpu.condition(0){((0u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10e7e8e8 cmp x10, #0
{let a=((cpu.x[10]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10e7e8ec csel w8, wzr, w8, eq
cpu.x[8]=(if cpu.condition(0){((0u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10e7e8f0 add w0, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u32 as u64;}
// 0x10e7e8f4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e7e8d4")),}}}
#[inline]
pub(super) unsafe fn f_10e7e944(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e7e944=>{
// 0x10e7e944 ldr w0, [x0, #0x144]
{let address=(cpu.x[0]).wrapping_add(0x144u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10e7e948 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e7e944")),}}}
#[inline]
pub(super) unsafe fn f_10e7e9ec(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e7e9ec=>{
// 0x10e7e9ec ldr w0, [x0, #0x1a4]
{let address=(cpu.x[0]).wrapping_add(0x1a4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10e7e9f0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e7e9ec")),}}}
#[inline]
pub(super) unsafe fn f_10e7f0ec(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e7f0ec=>{
// 0x10e7f0ec ldrb w0, [x0, #0x250]
{let address=(cpu.x[0]).wrapping_add(0x250u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10e7f0f0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e7f0ec")),}}}
#[inline]
pub(super) unsafe fn f_10e805d0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e805d0=>{
// 0x10e805d0 ldr w8, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10e805d4 and w0, w8, #0xf
{let a=((cpu.x[8] as u32 as u64));let b=((0xfu64));let result=a & b;cpu.x[0]=(result) as u32 as u64;}
// 0x10e805d8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e805d0")),}}}
#[inline]
pub(super) unsafe fn f_10e80934(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10e80934=>{
// 0x10e80934 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10e80938 stp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10e8093c stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10e80940 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10e80944 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10e80948 mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10e8094c mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x10e80950 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10e80954 mov x21, x8
cpu.x[21]=(((cpu.x[8]))) as u64;
// 0x10e80958 bl #0x111dd1b0
cpu.x[30]=0x10e8095c;super::chunk14::f_111dd1b0(cpu,rt,0x111dd1b0)?;
pc=0x10e8095c;continue;
}
0x10e8095c=>{
// 0x10e8095c mov x22, x0
cpu.x[22]=(((cpu.x[0]))) as u64;
// 0x10e80960 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10e80964 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10e80968 bl #0x111dd1b0
cpu.x[30]=0x10e8096c;super::chunk14::f_111dd1b0(cpu,rt,0x111dd1b0)?;
pc=0x10e8096c;continue;
}
0x10e8096c=>{
// 0x10e8096c ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e80970 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10e80974 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10e80978 ldr x8, [x8, #0xf0]
{let address=(cpu.x[8]).wrapping_add(0xf0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10e8097c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10e80980;super::dispatch(cpu,rt,target)?;}
pc=0x10e80980;continue;
}
0x10e80980=>{
// 0x10e80980 stp x22, x20, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=cpu.x[22];let v1=cpu.x[20];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10e80984 str w0, [x21, #0x10]
{let address=(cpu.x[21]).wrapping_add(0x10u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10e80988 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10e8098c ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10e80990 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10e80994 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10e80934")),}}}
#[inline]
pub(super) unsafe fn f_10ec3858(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10ec3858=>{
// 0x10ec3858 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10ec385c str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10ec3860 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10ec3864 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10ec3868 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10ec386c mov w0, #0x68
cpu.x[0]=(((0x68u64))) as u32 as u64;
// 0x10ec3870 mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10ec3874 bl #0x1187d2b0
cpu.x[30]=0x10ec3878;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
pc=0x10ec3878;continue;
}
0x10ec3878=>{
// 0x10ec3878 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10ec387c mov x2, x20
cpu.x[2]=(((cpu.x[20]))) as u64;
// 0x10ec3880 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10ec3884 bl #0x10fbab34
cpu.x[30]=0x10ec3888;super::chunk13::f_10fbab34(cpu,rt,0x10fbab34)?;
pc=0x10ec3888;continue;
}
0x10ec3888=>{
// 0x10ec3888 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10ec388c ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10ec3890 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10ec3894 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10ec3898 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10ec3858")),}}}
#[inline]
pub(super) unsafe fn f_10ed0470(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10ed0470=>{
// 0x10ed0470 mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x10ed0474 b #0x10f94298
super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10ed0470")),}}}
#[inline]
pub(super) unsafe fn f_10f07778(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f07778=>{
// 0x10f07778 adrp x0, #0x11987000
cpu.x[0]=(0x11987000u64) as u64;
// 0x10f0777c add x0, x0, #0x320
{let a=((cpu.x[0]));let b=((0x320u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10f07780 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f07778")),}}}
#[inline]
pub(super) unsafe fn f_10f4804c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f4804c=>{
// 0x10f4804c stp x29, x30, [sp, #-0x50]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffb0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f48050 stp x26, x25, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[26];let v1=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f48054 stp x24, x23, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f48058 stp x22, x21, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f4805c stp x20, x19, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f48060 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f48064 cmp x3, #2
{let a=((cpu.x[3]));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f48068 b.hs #0x10f48084
if cpu.condition(2){pc=0x10f48084;continue;}pc=0x10f4806c;continue;
}
0x10f4806c=>{
// 0x10f4806c ldp x20, x19, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f48070 ldp x22, x21, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f48074 ldp x24, x23, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10f48078 ldp x26, x25, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10f4807c ldp x29, x30, [sp], #0x50
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x50u64)))) as u64;}
// 0x10f48080 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f48084=>{
// 0x10f48084 b.ne #0x10f480c0
if cpu.condition(1){pc=0x10f480c0;continue;}pc=0x10f48088;continue;
}
0x10f48088=>{
// 0x10f48088 ldur x8, [x1, #-8]
{let address=(cpu.x[1]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f4808c ldr x9, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f48090 ldr w10, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10f48094 ldr w11, [x9, #0x10]
{let address=(cpu.x[9]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
// 0x10f48098 cmp w10, w11
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[11] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f4809c b.ge #0x10f4806c
if cpu.condition(10){pc=0x10f4806c;continue;}pc=0x10f480a0;continue;
}
0x10f480a0=>{
// 0x10f480a0 str x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f480a4 stur x9, [x1, #-8]
{let address=(cpu.x[1]).wrapping_add(0xfffffffffffffff8u64);let v0=cpu.x[9];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f480a8 ldp x20, x19, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f480ac ldp x22, x21, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f480b0 ldp x24, x23, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10f480b4 ldp x26, x25, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10f480b8 ldp x29, x30, [sp], #0x50
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x50u64)))) as u64;}
// 0x10f480bc ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f480c0=>{
// 0x10f480c0 cmp x3, #0x80
{let a=((cpu.x[3]));let b=((0x80u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f480c4 b.gt #0x10f48150
if cpu.condition(12){pc=0x10f48150;continue;}pc=0x10f480c8;continue;
}
0x10f480c8=>{
// 0x10f480c8 cmp x0, x1
{let a=((cpu.x[0]));let b=((cpu.x[1]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f480cc b.eq #0x10f4806c
if cpu.condition(0){pc=0x10f4806c;continue;}pc=0x10f480d0;continue;
}
0x10f480d0=>{
// 0x10f480d0 add x10, x0, #8
{let a=((cpu.x[0]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x10f480d4 cmp x10, x1
{let a=((cpu.x[10]));let b=((cpu.x[1]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f480d8 b.eq #0x10f4806c
if cpu.condition(0){pc=0x10f4806c;continue;}pc=0x10f480dc;continue;
}
0x10f480dc=>{
// 0x10f480dc mov x8, xzr
cpu.x[8]=(((0u64))) as u64;
// 0x10f480e0 mov x9, x0
cpu.x[9]=(((cpu.x[0]))) as u64;
// 0x10f480e4 b #0x10f48100
pc=0x10f48100;continue;
}
0x10f480e8=>{
// 0x10f480e8 mov x11, x0
cpu.x[11]=(((cpu.x[0]))) as u64;
pc=0x10f480ec;continue;
}
0x10f480ec=>{
// 0x10f480ec str x10, [x11]
{let address=(cpu.x[11]).wrapping_add(0x0u64);let v0=cpu.x[10];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10f480f0;continue;
}
0x10f480f0=>{
// 0x10f480f0 add x10, x9, #8
{let a=((cpu.x[9]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x10f480f4 add x8, x8, #8
{let a=((cpu.x[8]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10f480f8 cmp x10, x1
{let a=((cpu.x[10]));let b=((cpu.x[1]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f480fc b.eq #0x10f4806c
if cpu.condition(0){pc=0x10f4806c;continue;}pc=0x10f48100;continue;
}
0x10f48100=>{
// 0x10f48100 mov x11, x9
cpu.x[11]=(((cpu.x[9]))) as u64;
// 0x10f48104 mov x9, x10
cpu.x[9]=(((cpu.x[10]))) as u64;
// 0x10f48108 ldr x10, [x10]
{let address=(cpu.x[10]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10f4810c ldr x12, [x11]
{let address=(cpu.x[11]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[12]=(v0) as u64;}
// 0x10f48110 ldr w11, [x10, #0x10]
{let address=(cpu.x[10]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
// 0x10f48114 ldr w13, [x12, #0x10]
{let address=(cpu.x[12]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[13]=(v0) as u32 as u64;}
// 0x10f48118 cmp w11, w13
{let a=((cpu.x[11] as u32 as u64));let b=((cpu.x[13] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f4811c b.ge #0x10f480f0
if cpu.condition(10){pc=0x10f480f0;continue;}pc=0x10f48120;continue;
}
0x10f48120=>{
// 0x10f48120 mov x13, x8
cpu.x[13]=(((cpu.x[8]))) as u64;
pc=0x10f48124;continue;
}
0x10f48124=>{
// 0x10f48124 add x14, x0, x13
{let a=((cpu.x[0]));let b=((cpu.x[13]));let result=a.wrapping_add(b);cpu.x[14]=(result) as u64;}
// 0x10f48128 str x12, [x14, #8]
{let address=(cpu.x[14]).wrapping_add(0x8u64);let v0=cpu.x[12];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f4812c cbz x13, #0x10f480e8
if (cpu.x[13]) == 0 {pc=0x10f480e8;continue;}pc=0x10f48130;continue;
}
0x10f48130=>{
// 0x10f48130 ldur x12, [x14, #-8]
{let address=(cpu.x[14]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[12]=(v0) as u64;}
// 0x10f48134 sub x13, x13, #8
{let a=((cpu.x[13]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.x[13]=(result) as u64;}
// 0x10f48138 ldr w14, [x12, #0x10]
{let address=(cpu.x[12]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[14]=(v0) as u32 as u64;}
// 0x10f4813c cmp w11, w14
{let a=((cpu.x[11] as u32 as u64));let b=((cpu.x[14] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f48140 b.lt #0x10f48124
if cpu.condition(11){pc=0x10f48124;continue;}pc=0x10f48144;continue;
}
0x10f48144=>{
// 0x10f48144 add x11, x0, x13
{let a=((cpu.x[0]));let b=((cpu.x[13]));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
// 0x10f48148 add x11, x11, #8
{let a=((cpu.x[11]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
// 0x10f4814c b #0x10f480ec
pc=0x10f480ec;continue;
}
0x10f48150=>{
// 0x10f48150 lsr x19, x3, #1
cpu.x[19]=(((((cpu.x[3]))) as u64).wrapping_shr(((((0x1u64))) & 63) as u32) as u64) as u64;
// 0x10f48154 mov x22, x1
cpu.x[22]=(((cpu.x[1]))) as u64;
// 0x10f48158 cmp x3, x5
{let a=((cpu.x[3]));let b=((cpu.x[5]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f4815c mov x23, x0
cpu.x[23]=(((cpu.x[0]))) as u64;
// 0x10f48160 add x21, x0, x19, lsl #3
{let a=((cpu.x[0]));let b=(((cpu.x[19])) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[21]=(result) as u64;}
// 0x10f48164 sub x20, x3, x19
{let a=((cpu.x[3]));let b=((cpu.x[19]));let result=a.wrapping_sub(b);cpu.x[20]=(result) as u64;}
// 0x10f48168 mov x1, x21
cpu.x[1]=(((cpu.x[21]))) as u64;
// 0x10f4816c b.le #0x10f481d8
if cpu.condition(13){pc=0x10f481d8;continue;}pc=0x10f48170;continue;
}
0x10f48170=>{
// 0x10f48170 mov x3, x19
cpu.x[3]=(((cpu.x[19]))) as u64;
// 0x10f48174 mov x24, x2
cpu.x[24]=(((cpu.x[2]))) as u64;
// 0x10f48178 mov x25, x4
cpu.x[25]=(((cpu.x[4]))) as u64;
// 0x10f4817c mov x26, x5
cpu.x[26]=(((cpu.x[5]))) as u64;
// 0x10f48180 bl #0x10f4804c
cpu.x[30]=0x10f48184;super::chunk9::f_10f4804c(cpu,rt,0x10f4804c)?;
// 0x10f48184 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f48188 mov x1, x22
cpu.x[1]=(((cpu.x[22]))) as u64;
// 0x10f4818c mov x2, x24
cpu.x[2]=(((cpu.x[24]))) as u64;
// 0x10f48190 mov x3, x20
cpu.x[3]=(((cpu.x[20]))) as u64;
// 0x10f48194 mov x4, x25
cpu.x[4]=(((cpu.x[25]))) as u64;
// 0x10f48198 mov x5, x26
cpu.x[5]=(((cpu.x[26]))) as u64;
// 0x10f4819c bl #0x10f4804c
cpu.x[30]=0x10f481a0;super::chunk9::f_10f4804c(cpu,rt,0x10f4804c)?;
// 0x10f481a0 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10f481a4 mov x1, x21
cpu.x[1]=(((cpu.x[21]))) as u64;
// 0x10f481a8 mov x2, x22
cpu.x[2]=(((cpu.x[22]))) as u64;
// 0x10f481ac mov x3, x24
cpu.x[3]=(((cpu.x[24]))) as u64;
// 0x10f481b0 mov x4, x19
cpu.x[4]=(((cpu.x[19]))) as u64;
// 0x10f481b4 mov x5, x20
cpu.x[5]=(((cpu.x[20]))) as u64;
// 0x10f481b8 mov x6, x25
cpu.x[6]=(((cpu.x[25]))) as u64;
// 0x10f481bc mov x7, x26
cpu.x[7]=(((cpu.x[26]))) as u64;
// 0x10f481c0 ldp x20, x19, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f481c4 ldp x22, x21, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f481c8 ldp x24, x23, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10f481cc ldp x26, x25, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10f481d0 ldp x29, x30, [sp], #0x50
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x50u64)))) as u64;}
// 0x10f481d4 b #0x10f48688
super::dispatch(cpu,rt,0x10f48688)?;return Ok(());
}
0x10f481d8=>{
// 0x10f481d8 mov x26, x3
cpu.x[26]=(((cpu.x[3]))) as u64;
// 0x10f481dc mov x3, x19
cpu.x[3]=(((cpu.x[19]))) as u64;
// 0x10f481e0 mov x25, x2
cpu.x[25]=(((cpu.x[2]))) as u64;
// 0x10f481e4 mov x24, x4
cpu.x[24]=(((cpu.x[4]))) as u64;
// 0x10f481e8 bl #0x10f48390
cpu.x[30]=0x10f481ec;super::dispatch(cpu,rt,0x10f48390)?;
// 0x10f481ec add x19, x24, x19, lsl #3
{let a=((cpu.x[24]));let b=(((cpu.x[19])) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[19]=(result) as u64;}
// 0x10f481f0 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f481f4 mov x1, x22
cpu.x[1]=(((cpu.x[22]))) as u64;
// 0x10f481f8 mov x2, x25
cpu.x[2]=(((cpu.x[25]))) as u64;
// 0x10f481fc mov x3, x20
cpu.x[3]=(((cpu.x[20]))) as u64;
// 0x10f48200 mov x4, x19
cpu.x[4]=(((cpu.x[19]))) as u64;
// 0x10f48204 bl #0x10f48390
cpu.x[30]=0x10f48208;super::dispatch(cpu,rt,0x10f48390)?;
// 0x10f48208 add x9, x24, x26, lsl #3
{let a=((cpu.x[24]));let b=(((cpu.x[26])) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
// 0x10f4820c mov x10, x19
cpu.x[10]=(((cpu.x[19]))) as u64;
// 0x10f48210 mov x8, x23
cpu.x[8]=(((cpu.x[23]))) as u64;
pc=0x10f48214;continue;
}
0x10f48214=>{
// 0x10f48214 cmp x10, x9
{let a=((cpu.x[10]));let b=((cpu.x[9]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f48218 b.eq #0x10f48308
if cpu.condition(0){pc=0x10f48308;continue;}pc=0x10f4821c;continue;
}
0x10f4821c=>{
// 0x10f4821c mov x11, x10
cpu.x[11]=(((cpu.x[10]))) as u64;
// 0x10f48220 ldr x10, [x10]
{let address=(cpu.x[10]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10f48224 ldr x14, [x24]
{let address=(cpu.x[24]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[14]=(v0) as u64;}
// 0x10f48228 add x23, x23, #8
{let a=((cpu.x[23]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[23]=(result) as u64;}
// 0x10f4822c ldr w12, [x10, #0x10]
{let address=(cpu.x[10]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[12]=(v0) as u32 as u64;}
// 0x10f48230 ldr w13, [x14, #0x10]
{let address=(cpu.x[14]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[13]=(v0) as u32 as u64;}
// 0x10f48234 cmp w12, w13
{let a=((cpu.x[12] as u32 as u64));let b=((cpu.x[13] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f48238 cset w15, ge
cpu.x[15]=((cpu.condition(10)) as u64) as u32 as u64;
// 0x10f4823c csel x14, x14, x10, ge
cpu.x[14]=(if cpu.condition(10){((cpu.x[14]))}else{((cpu.x[10]))}) as u64;
// 0x10f48240 add x24, x24, w15, uxtw #3
{let a=((cpu.x[24]));let b=((((cpu.x[15] as u32 as u64) as u32 as u64)) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[24]=(result) as u64;}
// 0x10f48244 cset w15, lt
cpu.x[15]=((cpu.condition(11)) as u64) as u32 as u64;
// 0x10f48248 add x10, x11, w15, uxtw #3
{let a=((cpu.x[11]));let b=((((cpu.x[15] as u32 as u64) as u32 as u64)) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x10f4824c str x14, [x8], #8
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=cpu.x[14];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);cpu.x[8]=((cpu.x[8]).wrapping_add(((0x8u64)))) as u64;}
// 0x10f48250 cmp x24, x19
{let a=((cpu.x[24]));let b=((cpu.x[19]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f48254 b.ne #0x10f48214
if cpu.condition(1){pc=0x10f48214;continue;}pc=0x10f48258;continue;
}
0x10f48258=>{
// 0x10f48258 cmp x10, x9
{let a=((cpu.x[10]));let b=((cpu.x[9]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f4825c b.eq #0x10f4806c
if cpu.condition(0){pc=0x10f4806c;continue;}pc=0x10f48260;continue;
}
0x10f48260=>{
// 0x10f48260 cmp w12, w13
{let a=((cpu.x[12] as u32 as u64));let b=((cpu.x[13] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f48264 mov x14, #-8
cpu.x[14]=(((0xfffffffffffffff8u64))) as u64;
// 0x10f48268 sub x15, x9, x11
{let a=((cpu.x[9]));let b=((cpu.x[11]));let result=a.wrapping_sub(b);cpu.x[15]=(result) as u64;}
// 0x10f4826c csel x16, xzr, x14, ge
cpu.x[16]=(if cpu.condition(10){((0u64))}else{((cpu.x[14]))}) as u64;
// 0x10f48270 add x15, x15, x16
{let a=((cpu.x[15]));let b=((cpu.x[16]));let result=a.wrapping_add(b);cpu.x[15]=(result) as u64;}
// 0x10f48274 sub x15, x15, #8
{let a=((cpu.x[15]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.x[15]=(result) as u64;}
// 0x10f48278 cmp x15, #0x78
{let a=((cpu.x[15]));let b=((0x78u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f4827c b.lo #0x10f482f4
if cpu.condition(3){pc=0x10f482f4;continue;}pc=0x10f48280;continue;
}
0x10f48280=>{
// 0x10f48280 cmp w12, w13
{let a=((cpu.x[12] as u32 as u64));let b=((cpu.x[13] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f48284 csel x14, xzr, x14, ge
cpu.x[14]=(if cpu.condition(10){((0u64))}else{((cpu.x[14]))}) as u64;
// 0x10f48288 sub x14, x14, x11
{let a=((cpu.x[14]));let b=((cpu.x[11]));let result=a.wrapping_sub(b);cpu.x[14]=(result) as u64;}
// 0x10f4828c add x14, x14, x23
{let a=((cpu.x[14]));let b=((cpu.x[23]));let result=a.wrapping_add(b);cpu.x[14]=(result) as u64;}
// 0x10f48290 cmp x14, #0x20
{let a=((cpu.x[14]));let b=((0x20u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f48294 b.lo #0x10f482f4
if cpu.condition(3){pc=0x10f482f4;continue;}pc=0x10f48298;continue;
}
0x10f48298=>{
// 0x10f48298 lsr x15, x15, #3
cpu.x[15]=(((((cpu.x[15]))) as u64).wrapping_shr(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10f4829c cmp w12, w13
{let a=((cpu.x[12] as u32 as u64));let b=((cpu.x[13] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f482a0 mov w13, #1
cpu.x[13]=(((0x1u64))) as u32 as u64;
// 0x10f482a4 cinc w13, w13, ge
cpu.x[13]=(if cpu.condition(10){(((cpu.x[13] as u32 as u64))).wrapping_add(1)}else{((cpu.x[13] as u32 as u64))}) as u32 as u64;
// 0x10f482a8 mov x14, xzr
cpu.x[14]=(((0u64))) as u64;
// 0x10f482ac add x15, x15, #1
{let a=((cpu.x[15]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[15]=(result) as u64;}
// 0x10f482b0 and x13, x13, #1
{let a=((cpu.x[13]));let b=((0x1u64));let result=a & b;cpu.x[13]=(result) as u64;}
// 0x10f482b4 and x12, x15, #0x3ffffffffffffffc
{let a=((cpu.x[15]));let b=((0x3ffffffffffffffcu64));let result=a & b;cpu.x[12]=(result) as u64;}
// 0x10f482b8 add x13, x11, x13, lsl #3
{let a=((cpu.x[11]));let b=(((cpu.x[13])) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[13]=(result) as u64;}
// 0x10f482bc lsl x16, x12, #3
cpu.x[16]=(((((cpu.x[12]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10f482c0 add x10, x10, x16
{let a=((cpu.x[10]));let b=((cpu.x[16]));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x10f482c4 add x11, x8, x16
{let a=((cpu.x[8]));let b=((cpu.x[16]));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
// 0x10f482c8 mov x16, x12
cpu.x[16]=(((cpu.x[12]))) as u64;
pc=0x10f482cc;continue;
}
0x10f482cc=>{
// 0x10f482cc add x17, x13, x14
{let a=((cpu.x[13]));let b=((cpu.x[14]));let result=a.wrapping_add(b);cpu.x[17]=(result) as u64;}
// 0x10f482d0 add x0, x8, x14
{let a=((cpu.x[8]));let b=((cpu.x[14]));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10f482d4 subs x16, x16, #4
{let a=((cpu.x[16]));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.x[16]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x10f482d8 ldp q0, q1, [x17]
{let address=(cpu.x[17]).wrapping_add(0x0u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));let v1=rt.memory.read::<u128>(address.wrapping_add(16));cpu.q[0]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;cpu.q[1]=(v1) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x10f482dc add x14, x14, #0x20
{let a=((cpu.x[14]));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[14]=(result) as u64;}
// 0x10f482e0 stp q0, q1, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[1] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10f482e4 b.ne #0x10f482cc
if cpu.condition(1){pc=0x10f482cc;continue;}pc=0x10f482e8;continue;
}
0x10f482e8=>{
// 0x10f482e8 cmp x15, x12
{let a=((cpu.x[15]));let b=((cpu.x[12]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f482ec mov x8, x11
cpu.x[8]=(((cpu.x[11]))) as u64;
// 0x10f482f0 b.eq #0x10f4806c
if cpu.condition(0){pc=0x10f4806c;continue;}pc=0x10f482f4;continue;
}
0x10f482f4=>{
// 0x10f482f4 ldr x11, [x10], #8
{let address=(cpu.x[10]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[11]=(v0) as u64;cpu.x[10]=((cpu.x[10]).wrapping_add(((0x8u64)))) as u64;}
// 0x10f482f8 cmp x10, x9
{let a=((cpu.x[10]));let b=((cpu.x[9]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f482fc str x11, [x8], #8
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=cpu.x[11];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);cpu.x[8]=((cpu.x[8]).wrapping_add(((0x8u64)))) as u64;}
// 0x10f48300 b.ne #0x10f482f4
if cpu.condition(1){pc=0x10f482f4;continue;}pc=0x10f48304;continue;
}
0x10f48304=>{
// 0x10f48304 b #0x10f4806c
pc=0x10f4806c;continue;
}
0x10f48308=>{
// 0x10f48308 cmp x24, x19
{let a=((cpu.x[24]));let b=((cpu.x[19]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f4830c b.eq #0x10f4806c
if cpu.condition(0){pc=0x10f4806c;continue;}pc=0x10f48310;continue;
}
0x10f48310=>{
// 0x10f48310 sub x9, x19, x24
{let a=((cpu.x[19]));let b=((cpu.x[24]));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u64;}
// 0x10f48314 sub x9, x9, #8
{let a=((cpu.x[9]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u64;}
// 0x10f48318 cmp x9, #0x38
{let a=((cpu.x[9]));let b=((0x38u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f4831c b.lo #0x10f4832c
if cpu.condition(3){pc=0x10f4832c;continue;}pc=0x10f48320;continue;
}
0x10f48320=>{
// 0x10f48320 sub x10, x23, x24
{let a=((cpu.x[23]));let b=((cpu.x[24]));let result=a.wrapping_sub(b);cpu.x[10]=(result) as u64;}
// 0x10f48324 cmp x10, #0x20
{let a=((cpu.x[10]));let b=((0x20u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f48328 b.hs #0x10f48348
if cpu.condition(2){pc=0x10f48348;continue;}pc=0x10f4832c;continue;
}
0x10f4832c=>{
// 0x10f4832c mov x9, x24
cpu.x[9]=(((cpu.x[24]))) as u64;
// 0x10f48330 mov x10, x8
cpu.x[10]=(((cpu.x[8]))) as u64;
pc=0x10f48334;continue;
}
0x10f48334=>{
// 0x10f48334 ldr x8, [x9], #8
{let address=(cpu.x[9]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;cpu.x[9]=((cpu.x[9]).wrapping_add(((0x8u64)))) as u64;}
// 0x10f48338 cmp x9, x19
{let a=((cpu.x[9]));let b=((cpu.x[19]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f4833c str x8, [x10], #8
{let address=(cpu.x[10]).wrapping_add(0x0u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);cpu.x[10]=((cpu.x[10]).wrapping_add(((0x8u64)))) as u64;}
// 0x10f48340 b.ne #0x10f48334
if cpu.condition(1){pc=0x10f48334;continue;}pc=0x10f48344;continue;
}
0x10f48344=>{
// 0x10f48344 b #0x10f4806c
pc=0x10f4806c;continue;
}
0x10f48348=>{
// 0x10f48348 lsr x9, x9, #3
cpu.x[9]=(((((cpu.x[9]))) as u64).wrapping_shr(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10f4834c mov x11, xzr
cpu.x[11]=(((0u64))) as u64;
// 0x10f48350 add x12, x9, #1
{let a=((cpu.x[9]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[12]=(result) as u64;}
// 0x10f48354 and x13, x12, #0x3ffffffffffffffc
{let a=((cpu.x[12]));let b=((0x3ffffffffffffffcu64));let result=a & b;cpu.x[13]=(result) as u64;}
// 0x10f48358 lsl x10, x13, #3
cpu.x[10]=(((((cpu.x[13]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10f4835c mov x14, x13
cpu.x[14]=(((cpu.x[13]))) as u64;
// 0x10f48360 add x9, x24, x10
{let a=((cpu.x[24]));let b=((cpu.x[10]));let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
// 0x10f48364 add x10, x8, x10
{let a=((cpu.x[8]));let b=((cpu.x[10]));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
pc=0x10f48368;continue;
}
0x10f48368=>{
// 0x10f48368 add x15, x24, x11
{let a=((cpu.x[24]));let b=((cpu.x[11]));let result=a.wrapping_add(b);cpu.x[15]=(result) as u64;}
// 0x10f4836c add x16, x8, x11
{let a=((cpu.x[8]));let b=((cpu.x[11]));let result=a.wrapping_add(b);cpu.x[16]=(result) as u64;}
// 0x10f48370 subs x14, x14, #4
{let a=((cpu.x[14]));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.x[14]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x10f48374 ldp q0, q1, [x15]
{let address=(cpu.x[15]).wrapping_add(0x0u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));let v1=rt.memory.read::<u128>(address.wrapping_add(16));cpu.q[0]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;cpu.q[1]=(v1) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x10f48378 add x11, x11, #0x20
{let a=((cpu.x[11]));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
// 0x10f4837c stp q0, q1, [x16]
{let address=(cpu.x[16]).wrapping_add(0x0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[1] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10f48380 b.ne #0x10f48368
if cpu.condition(1){pc=0x10f48368;continue;}pc=0x10f48384;continue;
}
0x10f48384=>{
// 0x10f48384 cmp x12, x13
{let a=((cpu.x[12]));let b=((cpu.x[13]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f48388 b.ne #0x10f48334
if cpu.condition(1){pc=0x10f48334;continue;}pc=0x10f4838c;continue;
}
0x10f4838c=>{
// 0x10f4838c b #0x10f4806c
pc=0x10f4806c;continue;
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f4804c")),}}}
#[inline]
pub(super) unsafe fn f_10f73ce4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f73ce4=>{
// 0x10f73ce4 ldrb w8, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f73ce8 cbz w8, #0x10f73cf0
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f73cf0;continue;}pc=0x10f73cec;continue;
}
0x10f73cec=>{
// 0x10f73cec ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f73cf0=>{
// 0x10f73cf0 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10f73cf4 ldr x9, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f73cf8 strb w8, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f73cfc ldr x1, [x9, #0xd8]
{let address=(cpu.x[9]).wrapping_add(0xd8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10f73d00 br x1
{let target=cpu.x[1];if target>=0x10f73ce4 && target<0x10f73d04{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f73ce4")),}}}
#[inline]
pub(super) unsafe fn f_10f73fd8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f73fd8=>{
// 0x10f73fd8 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f73fdc stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f73fe0 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f73fe4 ldr x8, [x1]
{let address=(cpu.x[1]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f73fe8 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f73fec mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x10f73ff0 mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x10f73ff4 ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f73ff8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f73ffc;super::dispatch(cpu,rt,target)?;}
pc=0x10f73ffc;continue;
}
0x10f73ffc=>{
// 0x10f73ffc and w8, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[8]=(result) as u32 as u64;}
pc=0x10f74000;continue;
}
0x10f74000=>{
// 0x10f74000 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f74004 strb w8, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f74008 bl #0x10f739a8
cpu.x[30]=0x10f7400c;super::chunk6::f_10f739a8(cpu,rt,0x10f739a8)?;
pc=0x10f7400c;continue;
}
0x10f7400c=>{
// 0x10f7400c str x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f74010 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f74014 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f74018 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7401c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f74020;super::dispatch(cpu,rt,target)?;}
pc=0x10f74020;continue;
}
0x10f74020=>{
// 0x10f74020 str xzr, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f74024 tbnz w0, #0x1f, #0x10f74054
if ((cpu.x[0] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x10f74054;continue;}pc=0x10f74028;continue;
}
0x10f74028=>{
// 0x10f74028 mov w20, w0
cpu.x[20]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f7402c ldr x0, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f74030 cbz x0, #0x10f74054
if (cpu.x[0]) == 0 {pc=0x10f74054;continue;}pc=0x10f74034;continue;
}
0x10f74034=>{
// 0x10f74034 bl #0x10f93f94
cpu.x[30]=0x10f74038;super::chunk3::f_10f93f94(cpu,rt,0x10f93f94)?;
// 0x10f74038 cbz x0, #0x10f74054
if (cpu.x[0]) == 0 {pc=0x10f74054;continue;}pc=0x10f7403c;continue;
}
0x10f7403c=>{
// 0x10f7403c ldr x0, [x0, #0xb8]
{let address=(cpu.x[0]).wrapping_add(0xb8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f74040 cbz x0, #0x10f74054
if (cpu.x[0]) == 0 {pc=0x10f74054;continue;}pc=0x10f74044;continue;
}
0x10f74044=>{
// 0x10f74044 mov w1, w20
cpu.x[1]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x10f74048 mov w2, #1
cpu.x[2]=(((0x1u64))) as u32 as u64;
// 0x10f7404c bl #0x10f748a4
cpu.x[30]=0x10f74050;super::dispatch(cpu,rt,0x10f748a4)?;
// 0x10f74050 str x0, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10f74054;continue;
}
0x10f74054=>{
// 0x10f74054 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f74058 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f7405c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f73fd8")),}}}
#[inline]
pub(super) unsafe fn f_10f74160(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f74160=>{
// 0x10f74160 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f74160")),}}}
#[inline]
pub(super) unsafe fn f_10f75830(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f75830=>{
// 0x10f75830 adrp x8, #0x11935000
cpu.x[8]=(0x11935000u64) as u64;
// 0x10f75834 add x8, x8, #0x688
{let a=((cpu.x[8]));let b=((0x688u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10f75838 stp xzr, xzr, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f7583c stp x8, xzr, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=cpu.x[8];let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f75840 stp xzr, xzr, [x0, #0x20]
{let address=(cpu.x[0]).wrapping_add(0x20u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f75844 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f75830")),}}}
#[inline]
pub(super) unsafe fn f_10f784b8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f784b8=>{
// 0x10f784b8 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f784bc stp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f784c0 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f784c4 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f784c8 ldr x8, [x1]
{let address=(cpu.x[1]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f784cc mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10f784d0 ldr w1, [x0, #0x5c]
{let address=(cpu.x[0]).wrapping_add(0x5cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f784d4 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10f784d8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f784dc ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f784e0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f784e4;super::dispatch(cpu,rt,target)?;}
pc=0x10f784e4;continue;
}
0x10f784e4=>{
// 0x10f784e4 ldr w8, [x20, #0x5c]
{let address=(cpu.x[20]).wrapping_add(0x5cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f784e8 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f784ec b.lt #0x10f78514
if cpu.condition(11){pc=0x10f78514;continue;}pc=0x10f784f0;continue;
}
0x10f784f0=>{
// 0x10f784f0 mov x21, xzr
cpu.x[21]=(((0u64))) as u64;
pc=0x10f784f4;continue;
}
0x10f784f4=>{
// 0x10f784f4 ldr x8, [x20, #0x50]
{let address=(cpu.x[20]).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f784f8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f784fc ldr x1, [x8, x21, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[21])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10f78500 bl #0x10f73d0c
cpu.x[30]=0x10f78504;super::chunk13::f_10f73d0c(cpu,rt,0x10f73d0c)?;
pc=0x10f78504;continue;
}
0x10f78504=>{
// 0x10f78504 ldrsw x8, [x20, #0x5c]
{let address=(cpu.x[20]).wrapping_add(0x5cu64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10f78508 add x21, x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[21]=(result) as u64;}
// 0x10f7850c cmp x21, x8
{let a=((cpu.x[21]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f78510 b.lt #0x10f784f4
if cpu.condition(11){pc=0x10f784f4;continue;}pc=0x10f78514;continue;
}
0x10f78514=>{
// 0x10f78514 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f78518 ldr w1, [x20, #0x1c]
{let address=(cpu.x[20]).wrapping_add(0x1cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f7851c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f78520 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f78524 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f78528;super::dispatch(cpu,rt,target)?;}
pc=0x10f78528;continue;
}
0x10f78528=>{
// 0x10f78528 ldr w8, [x20, #0x1c]
{let address=(cpu.x[20]).wrapping_add(0x1cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f7852c cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f78530 b.lt #0x10f785c4
if cpu.condition(11){pc=0x10f785c4;continue;}pc=0x10f78534;continue;
}
0x10f78534=>{
// 0x10f78534 mov x21, xzr
cpu.x[21]=(((0u64))) as u64;
// 0x10f78538 b #0x10f78560
pc=0x10f78560;continue;
}
0x10f7853c=>{
// 0x10f7853c mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
pc=0x10f78540;continue;
}
0x10f78540=>{
// 0x10f78540 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f78544 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f78548 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7854c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f78550;super::dispatch(cpu,rt,target)?;}
pc=0x10f78550;continue;
}
0x10f78550=>{
// 0x10f78550 ldrsw x8, [x20, #0x1c]
{let address=(cpu.x[20]).wrapping_add(0x1cu64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10f78554 add x21, x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[21]=(result) as u64;}
// 0x10f78558 cmp x21, x8
{let a=((cpu.x[21]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f7855c b.ge #0x10f785c4
if cpu.condition(10){pc=0x10f785c4;continue;}pc=0x10f78560;continue;
}
0x10f78560=>{
// 0x10f78560 ldr x8, [x20, #0x20]
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f78564 ldr x9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f78568 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f7856c ldr w1, [x8, x21, lsl #2]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[21])) as u64).wrapping_shl(2) as u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f78570 ldr x8, [x9, #0x88]
{let address=(cpu.x[9]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f78574 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f78578;super::dispatch(cpu,rt,target)?;}
pc=0x10f78578;continue;
}
0x10f78578=>{
// 0x10f78578 ldr x8, [x20, #0x10]
{let address=(cpu.x[20]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7857c lsl x22, x21, #3
cpu.x[22]=(((((cpu.x[21]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10f78580 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f78584 ldr x1, [x8, x22]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[22])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10f78588 bl #0x111dd6a4
cpu.x[30]=0x10f7858c;super::chunk3::f_111dd6a4(cpu,rt,0x111dd6a4)?;
pc=0x10f7858c;continue;
}
0x10f7858c=>{
// 0x10f7858c ldr x8, [x20, #0x30]
{let address=(cpu.x[20]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f78590 ldr x9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f78594 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f78598 ldr x8, [x8, x22]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[22])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f7859c ldr w1, [x8, #0xc]
{let address=(cpu.x[8]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f785a0 ldr x8, [x9, #0x88]
{let address=(cpu.x[9]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f785a4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f785a8;super::dispatch(cpu,rt,target)?;}
pc=0x10f785a8;continue;
}
0x10f785a8=>{
// 0x10f785a8 ldr x8, [x20, #0x40]
{let address=(cpu.x[20]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f785ac ldr x8, [x8, x22]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[22])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f785b0 cbz x8, #0x10f7853c
if (cpu.x[8]) == 0 {pc=0x10f7853c;continue;}pc=0x10f785b4;continue;
}
0x10f785b4=>{
// 0x10f785b4 ldr x9, [x8, #0x20]
{let address=(cpu.x[8]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f785b8 cbz x9, #0x10f7853c
if (cpu.x[9]) == 0 {pc=0x10f7853c;continue;}pc=0x10f785bc;continue;
}
0x10f785bc=>{
// 0x10f785bc ldr w1, [x8, #0x1c]
{let address=(cpu.x[8]).wrapping_add(0x1cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10f785c0 b #0x10f78540
pc=0x10f78540;continue;
}
0x10f785c4=>{
// 0x10f785c4 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f785c8 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f785cc ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f785d0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f784b8")),}}}
#[inline]
pub(super) unsafe fn f_10f7922c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f7922c=>{
// 0x10f7922c cbz x1, #0x10f792b0
if (cpu.x[1]) == 0 {pc=0x10f792b0;continue;}pc=0x10f79230;continue;
}
0x10f79230=>{
// 0x10f79230 stp x29, x30, [sp, #-0x50]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffb0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f79234 str x25, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f79238 stp x24, x23, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f7923c stp x22, x21, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f79240 stp x20, x19, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f79244 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f79248 mov x23, x0
cpu.x[23]=(((cpu.x[0]))) as u64;
// 0x10f7924c mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x10f79250 mov x19, x5
cpu.x[19]=(((cpu.x[5]))) as u64;
// 0x10f79254 mov w22, w4
cpu.x[22]=(((cpu.x[4] as u32 as u64))) as u32 as u64;
// 0x10f79258 mov w20, w3
cpu.x[20]=(((cpu.x[3] as u32 as u64))) as u32 as u64;
// 0x10f7925c mov x21, x2
cpu.x[21]=(((cpu.x[2]))) as u64;
// 0x10f79260 mov x24, x1
cpu.x[24]=(((cpu.x[1]))) as u64;
// 0x10f79264 bl #0x10f93f94
cpu.x[30]=0x10f79268;super::chunk3::f_10f93f94(cpu,rt,0x10f93f94)?;
pc=0x10f79268;continue;
}
0x10f79268=>{
// 0x10f79268 bl #0x10f9915c
cpu.x[30]=0x10f7926c;super::chunk11::f_10f9915c(cpu,rt,0x10f9915c)?;
pc=0x10f7926c;continue;
}
0x10f7926c=>{
// 0x10f7926c mov x1, x24
cpu.x[1]=(((cpu.x[24]))) as u64;
// 0x10f79270 mov x25, x0
cpu.x[25]=(((cpu.x[0]))) as u64;
// 0x10f79274 bl #0x10f879e0
cpu.x[30]=0x10f79278;super::chunk1::f_10f879e0(cpu,rt,0x10f879e0)?;
pc=0x10f79278;continue;
}
0x10f79278=>{
// 0x10f79278 and w4, w22, #1
{let a=((cpu.x[22] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[4]=(result) as u32 as u64;}
// 0x10f7927c mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10f79280 mov x1, x25
cpu.x[1]=(((cpu.x[25]))) as u64;
// 0x10f79284 mov x2, x21
cpu.x[2]=(((cpu.x[21]))) as u64;
// 0x10f79288 mov w3, w20
cpu.x[3]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x10f7928c mov x5, x19
cpu.x[5]=(((cpu.x[19]))) as u64;
// 0x10f79290 bl #0x10f792b4
cpu.x[30]=0x10f79294;super::chunk4::f_10f792b4(cpu,rt,0x10f792b4)?;
pc=0x10f79294;continue;
}
0x10f79294=>{
// 0x10f79294 mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10f79298 ldp x20, x19, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f7929c ldr x25, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[25]=(v0) as u64;}
// 0x10f792a0 ldp x22, x21, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f792a4 ldp x24, x23, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10f792a8 ldp x29, x30, [sp], #0x50
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x50u64)))) as u64;}
// 0x10f792ac b #0x10f878a0
super::chunk14::f_10f878a0(cpu,rt,0x10f878a0)?;return Ok(());
}
0x10f792b0=>{
// 0x10f792b0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f7922c")),}}}
#[inline]
pub(super) unsafe fn f_10f79ca0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f79ca0=>{
// 0x10f79ca0 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f79ca4 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f79ca8 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f79cac mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10f79cb0 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10f79cb4 bl #0x10f939f4
cpu.x[30]=0x10f79cb8;super::chunk14::f_10f939f4(cpu,rt,0x10f939f4)?;
pc=0x10f79cb8;continue;
}
0x10f79cb8=>{
// 0x10f79cb8 movi v0.2d, #0000000000000000
cpu.q[0]=(lane_dup(0u128,64,2)) & 0xffffffffffffffffffffffffffffffffu128;
// 0x10f79cbc mov x8, x20
cpu.x[8]=(((cpu.x[20]))) as u64;
// 0x10f79cc0 strh wzr, [x20, #0x118]
{let address=(cpu.x[20]).wrapping_add(0x118u64);let v0=0u64;rt.memory.write::<u16>(address.wrapping_add(0),v0 as u16);}
// 0x10f79cc4 str xzr, [x8, #0xfc]!
{let address=(cpu.x[8]).wrapping_add(0xfcu64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);cpu.x[8]=(address) as u64;}
// 0x10f79cc8 adrp x9, #0x11935000
cpu.x[9]=(0x11935000u64) as u64;
// 0x10f79ccc add x9, x9, #0x708
{let a=((cpu.x[9]));let b=((0x708u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
// 0x10f79cd0 stp xzr, xzr, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f79cd4 str x9, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=cpu.x[9];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f79cd8 mov w9, #-1
cpu.x[9]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10f79cdc stp q0, q0, [x20, #0x120]
{let address=(cpu.x[20]).wrapping_add(0x120u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10f79ce0 str q0, [x20, #0x140]
{let address=(cpu.x[20]).wrapping_add(0x140u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x10f79ce4 ldr x8, [x19, #0x198]
{let address=(cpu.x[19]).wrapping_add(0x198u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f79ce8 str w9, [x20, #0x114]
{let address=(cpu.x[20]).wrapping_add(0x114u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f79cec cmp x8, #0
{let a=((cpu.x[8]));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f79cf0 cset w8, ne
cpu.x[8]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x10f79cf4 strb w8, [x20, #0xf8]
{let address=(cpu.x[20]).wrapping_add(0xf8u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f79cf8 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f79cfc ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f79d00 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f79ca0")),}}}
#[inline]
pub(super) unsafe fn f_10f84c58(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f84c58=>{
// 0x10f84c58 b #0x10f944ac
super::chunk4::f_10f944ac(cpu,rt,0x10f944ac)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f84c58")),}}}
#[inline]
pub(super) unsafe fn f_10f85244(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f85244=>{
// 0x10f85244 sub sp, sp, #0x60
{let a=((cpu.sp));let b=((0x60u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x10f85248 stp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f8524c str x25, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f85250 stp x24, x23, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f85254 stp x22, x21, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f85258 stp x20, x19, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f8525c add x29, sp, #0x10
{let a=((cpu.sp));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x10f85260 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10f85264 mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x10f85268 mov w22, w2
cpu.x[22]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x10f8526c mov x21, x1
cpu.x[21]=(((cpu.x[1]))) as u64;
// 0x10f85270 bl #0x10f8e020
cpu.x[30]=0x10f85274;super::chunk1::f_10f8e020(cpu,rt,0x10f8e020)?;
pc=0x10f85274;continue;
}
0x10f85274=>{
// 0x10f85274 ldr x19, [x0, #0x348]
{let address=(cpu.x[0]).wrapping_add(0x348u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10f85278 cbnz x19, #0x10f85290
if (cpu.x[19]) != 0 {pc=0x10f85290;continue;}pc=0x10f8527c;continue;
}
0x10f8527c=>{
// 0x10f8527c ldr x0, [x20, #0x48]
{let address=(cpu.x[20]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f85280 and w1, w22, #1
{let a=((cpu.x[22] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[1]=(result) as u32 as u64;}
// 0x10f85284 bl #0x10de0438
cpu.x[30]=0x10f85288;super::chunk6::f_10de0438(cpu,rt,0x10de0438)?;
pc=0x10f85288;continue;
}
0x10f85288=>{
// 0x10f85288 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f8528c cbz x0, #0x10f85320
if (cpu.x[0]) == 0 {pc=0x10f85320;continue;}pc=0x10f85290;continue;
}
0x10f85290=>{
// 0x10f85290 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f85294 bl #0x10f93f94
cpu.x[30]=0x10f85298;super::chunk3::f_10f93f94(cpu,rt,0x10f93f94)?;
pc=0x10f85298;continue;
}
0x10f85298=>{
// 0x10f85298 ldr x22, [x0, #0x90]
{let address=(cpu.x[0]).wrapping_add(0x90u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[22]=(v0) as u64;}
// 0x10f8529c mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f852a0 bl #0x10f94298
cpu.x[30]=0x10f852a4;super::chunk2::f_10f94298(cpu,rt,0x10f94298)?;
pc=0x10f852a4;continue;
}
0x10f852a4=>{
// 0x10f852a4 mov w23, w0
cpu.x[23]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f852a8 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f852ac bl #0x10f942a0
cpu.x[30]=0x10f852b0;super::chunk6::f_10f942a0(cpu,rt,0x10f942a0)?;
pc=0x10f852b0;continue;
}
0x10f852b0=>{
// 0x10f852b0 mov w24, w0
cpu.x[24]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f852b4 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f852b8 bl #0x10f942a8
cpu.x[30]=0x10f852bc;super::chunk11::f_10f942a8(cpu,rt,0x10f942a8)?;
pc=0x10f852bc;continue;
}
0x10f852bc=>{
// 0x10f852bc mov w25, w0
cpu.x[25]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f852c0 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10f852c4 bl #0x10f93fc0
cpu.x[30]=0x10f852c8;super::chunk5::f_10f93fc0(cpu,rt,0x10f93fc0)?;
pc=0x10f852c8;continue;
}
0x10f852c8=>{
// 0x10f852c8 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10f852cc add x0, x20, #0xf8
{let a=((cpu.x[20]));let b=((0xf8u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10f852d0 bl #0x11294e04
cpu.x[30]=0x10f852d4;super::chunk4::f_11294e04(cpu,rt,0x11294e04)?;
pc=0x10f852d4;continue;
}
0x10f852d4=>{
// 0x10f852d4 ldr x8, [x22]
{let address=(cpu.x[22]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f852d8 mov w7, w0
cpu.x[7]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10f852dc mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f852e0 mov w1, w23
cpu.x[1]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10f852e4 mov w2, w24
cpu.x[2]=(((cpu.x[24] as u32 as u64))) as u32 as u64;
// 0x10f852e8 mov w3, w25
cpu.x[3]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10f852ec ldr x8, [x8, #0x48]
{let address=(cpu.x[8]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f852f0 mov x4, x19
cpu.x[4]=(((cpu.x[19]))) as u64;
// 0x10f852f4 mov x5, xzr
cpu.x[5]=(((0u64))) as u64;
// 0x10f852f8 mov x6, x21
cpu.x[6]=(((cpu.x[21]))) as u64;
// 0x10f852fc str wzr, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f85300 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f85304;super::dispatch(cpu,rt,target)?;}
pc=0x10f85304;continue;
}
0x10f85304=>{
// 0x10f85304 ldp x20, x19, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f85308 ldr x25, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[25]=(v0) as u64;}
// 0x10f8530c ldp x22, x21, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f85310 ldp x24, x23, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10f85314 ldp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10f85318 add sp, sp, #0x60
{let a=((cpu.sp));let b=((0x60u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10f8531c ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f85320=>{
// 0x10f85320 b #0x10f85304
pc=0x10f85304;continue;
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f85244")),}}}
#[inline]
pub(super) unsafe fn f_10f85eb0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f85eb0=>{
// 0x10f85eb0 ldrb w0, [x0, #0x172]
{let address=(cpu.x[0]).wrapping_add(0x172u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10f85eb4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f85eb0")),}}}
#[inline]
pub(super) unsafe fn f_10f86d50(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f86d50=>{
// 0x10f86d50 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f86d54 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f86d58 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f86d5c mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f86d60 cbz x1, #0x10f86d94
if (cpu.x[1]) == 0 {pc=0x10f86d94;continue;}pc=0x10f86d64;continue;
}
0x10f86d64=>{
// 0x10f86d64 ldr x8, [x1, #0x10]
{let address=(cpu.x[1]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f86d68 mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x10f86d6c cbz x8, #0x10f86da4
if (cpu.x[8]) == 0 {pc=0x10f86da4;continue;}pc=0x10f86d70;continue;
}
0x10f86d70=>{
// 0x10f86d70 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f86d74 bl #0x10f95c00
cpu.x[30]=0x10f86d78;super::chunk11::f_10f95c00(cpu,rt,0x10f95c00)?;
pc=0x10f86d78;continue;
}
0x10f86d78=>{
// 0x10f86d78 str w0, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f86d7c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f86d80 bl #0x10f95bec
cpu.x[30]=0x10f86d84;super::chunk6::f_10f95bec(cpu,rt,0x10f95bec)?;
pc=0x10f86d84;continue;
}
0x10f86d84=>{
// 0x10f86d84 str w0, [x19, #0x64]
{let address=(cpu.x[19]).wrapping_add(0x64u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f86d88 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f86d8c bl #0x10f95c08
cpu.x[30]=0x10f86d90;super::chunk13::f_10f95c08(cpu,rt,0x10f95c08)?;
pc=0x10f86d90;continue;
}
0x10f86d90=>{
// 0x10f86d90 b #0x10f86da0
pc=0x10f86da0;continue;
}
0x10f86d94=>{
// 0x10f86d94 movi v0.2d, #0xffffffffffffffff
cpu.q[0]=(lane_dup(18446744073709551615u128,64,2)) & 0xffffffffffffffffffffffffffffffffu128;
// 0x10f86d98 mov w0, #-1
cpu.x[0]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10f86d9c str d0, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=(cpu.q[0] & 0xffffffffffffffffu128) as u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10f86da0;continue;
}
0x10f86da0=>{
// 0x10f86da0 str w0, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10f86da4;continue;
}
0x10f86da4=>{
// 0x10f86da4 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f86da8 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f86dac ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f86d50")),}}}
#[inline]
pub(super) unsafe fn f_10f86f3c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f86f3c=>{
// 0x10f86f3c ldr x8, [x0, #0x18]
{let address=(cpu.x[0]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f86f40 ldr x0, [x8, #0x240]
{let address=(cpu.x[8]).wrapping_add(0x240u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f86f44 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f86f3c")),}}}
#[inline]
pub(super) unsafe fn f_10f871bc(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f871bc=>{
// 0x10f871bc b #0x10f93eb0
super::chunk6::f_10f93eb0(cpu,rt,0x10f93eb0)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f871bc")),}}}
#[inline]
pub(super) unsafe fn f_10f8915c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f8915c=>{
// 0x10f8915c str x1, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=cpu.x[1];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f89160 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f8915c")),}}}
#[inline]
pub(super) unsafe fn f_10f89440(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f89440=>{
// 0x10f89440 ldr w8, [x0, #0x24]
{let address=(cpu.x[0]).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f89444 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f89448 b.lt #0x10f894a8
if cpu.condition(11){pc=0x10f894a8;continue;}pc=0x10f8944c;continue;
}
0x10f8944c=>{
// 0x10f8944c stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f89450 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f89454 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f89458 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f8945c mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f89460 mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x10f89464 mov x21, xzr
cpu.x[21]=(((0u64))) as u64;
// 0x10f89468 b #0x10f89478
pc=0x10f89478;continue;
}
0x10f8946c=>{
// 0x10f8946c add x21, x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[21]=(result) as u64;}
// 0x10f89470 cmp x21, w8, sxtw
{let a=((cpu.x[21]));let b=(((cpu.x[8] as u32 as u64) as i32 as i64 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f89474 b.ge #0x10f8949c
if cpu.condition(10){pc=0x10f8949c;continue;}pc=0x10f89478;continue;
}
0x10f89478=>{
// 0x10f89478 ldr x9, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f8947c ldr x0, [x9, x21, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[21])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f89480 cbz x0, #0x10f8946c
if (cpu.x[0]) == 0 {pc=0x10f8946c;continue;}pc=0x10f89484;continue;
}
0x10f89484=>{
// 0x10f89484 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f89488 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10f8948c ldr x8, [x8, #0x60]
{let address=(cpu.x[8]).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f89490 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f89494;super::dispatch(cpu,rt,target)?;}
pc=0x10f89494;continue;
}
0x10f89494=>{
// 0x10f89494 ldr w8, [x19, #0x24]
{let address=(cpu.x[19]).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f89498 b #0x10f8946c
pc=0x10f8946c;continue;
}
0x10f8949c=>{
// 0x10f8949c ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f894a0 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f894a4 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
pc=0x10f894a8;continue;
}
0x10f894a8=>{
// 0x10f894a8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f89440")),}}}
#[inline]
pub(super) unsafe fn f_10f895a0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f895a0=>{
// 0x10f895a0 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f895a4 stp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f895a8 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f895ac mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f895b0 cbz x2, #0x10f895fc
if (cpu.x[2]) == 0 {pc=0x10f895fc;continue;}pc=0x10f895b4;continue;
}
0x10f895b4=>{
// 0x10f895b4 ldr x8, [x2]
{let address=(cpu.x[2]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f895b8 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10f895bc mov x0, x2
cpu.x[0]=(((cpu.x[2]))) as u64;
// 0x10f895c0 mov x19, x2
cpu.x[19]=(((cpu.x[2]))) as u64;
// 0x10f895c4 mov x21, x1
cpu.x[21]=(((cpu.x[1]))) as u64;
// 0x10f895c8 ldr x8, [x8, #0x100]
{let address=(cpu.x[8]).wrapping_add(0x100u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f895cc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f895d0;super::dispatch(cpu,rt,target)?;}
// 0x10f895d0 tbz w0, #0, #0x10f8960c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f8960c;continue;}pc=0x10f895d4;continue;
}
0x10f895d4=>{
// 0x10f895d4 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f895d8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10f895dc mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f895e0 mov x2, xzr
cpu.x[2]=(((0u64))) as u64;
// 0x10f895e4 ldr x4, [x8, #0x60]
{let address=(cpu.x[8]).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[4]=(v0) as u64;}
// 0x10f895e8 mov w3, #-1
cpu.x[3]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10f895ec ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f895f0 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f895f4 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f895f8 br x4
{let target=cpu.x[4];if target>=0x10f895a0 && target<0x10f89654{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
0x10f895fc=>{
// 0x10f895fc ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f89600 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f89604 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f89608 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f8960c=>{
// 0x10f8960c ldr x0, [x20, #0x28]
{let address=(cpu.x[20]).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f89610 cbnz x0, #0x10f89630
if (cpu.x[0]) != 0 {pc=0x10f89630;continue;}pc=0x10f89614;continue;
}
0x10f89614=>{
// 0x10f89614 mov w0, #0x68
cpu.x[0]=(((0x68u64))) as u32 as u64;
// 0x10f89618 bl #0x1187d2b0
cpu.x[30]=0x10f8961c;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
// 0x10f8961c mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10f89620 mov x22, x0
cpu.x[22]=(((cpu.x[0]))) as u64;
// 0x10f89624 bl #0x10f77db8
cpu.x[30]=0x10f89628;super::chunk10::f_10f77db8(cpu,rt,0x10f77db8)?;
// 0x10f89628 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f8962c str x22, [x20, #0x28]
{let address=(cpu.x[20]).wrapping_add(0x28u64);let v0=cpu.x[22];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10f89630;continue;
}
0x10f89630=>{
// 0x10f89630 mov x1, x21
cpu.x[1]=(((cpu.x[21]))) as u64;
// 0x10f89634 mov x2, x19
cpu.x[2]=(((cpu.x[19]))) as u64;
// 0x10f89638 mov w3, #-1
cpu.x[3]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10f8963c mov w4, wzr
cpu.x[4]=(((0u64))) as u32 as u64;
// 0x10f89640 mov x5, xzr
cpu.x[5]=(((0u64))) as u64;
// 0x10f89644 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f89648 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f8964c ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f89650 b #0x10f792b4
super::chunk4::f_10f792b4(cpu,rt,0x10f792b4)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f895a0")),}}}
#[inline]
pub(super) unsafe fn f_10f89efc(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f89efc=>{
// 0x10f89efc stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f89f00 stp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f89f04 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f89f08 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f89f0c ldr w21, [x0, #0xc]
{let address=(cpu.x[0]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[21]=(v0) as u32 as u64;}
// 0x10f89f10 cmp w1, w21
{let a=((cpu.x[1] as u32 as u64));let b=((cpu.x[21] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f89f14 b.lt #0x10f89fdc
if cpu.condition(11){pc=0x10f89fdc;continue;}pc=0x10f89f18;continue;
}
0x10f89f18=>{
// 0x10f89f18 ldr w8, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f89f1c mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f89f20 cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f89f24 b.ge #0x10f89f38
if cpu.condition(10){pc=0x10f89f38;continue;}pc=0x10f89f28;continue;
}
0x10f89f28=>{
// 0x10f89f28 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f89f2c mov w20, w1
cpu.x[20]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x10f89f30 bl #0x10f8b91c
cpu.x[30]=0x10f89f34;super::chunk9::f_10f8b91c(cpu,rt,0x10f8b91c)?;
pc=0x10f89f34;continue;
}
0x10f89f34=>{
// 0x10f89f34 mov w1, w20
cpu.x[1]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
pc=0x10f89f38;continue;
}
0x10f89f38=>{
// 0x10f89f38 subs w21, w1, w21
{let a=((cpu.x[1] as u32 as u64));let b=((cpu.x[21] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[21]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10f89f3c b.le #0x10f89fdc
if cpu.condition(13){pc=0x10f89fdc;continue;}pc=0x10f89f40;continue;
}
0x10f89f40=>{
// 0x10f89f40 mov w22, #5
cpu.x[22]=(((0x5u64))) as u32 as u64;
// 0x10f89f44 b #0x10f89f60
pc=0x10f89f60;continue;
}
0x10f89f48=>{
// 0x10f89f48 ldr x8, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f89f4c add w10, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10f89f50 subs w21, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[21]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10f89f54 str w10, [x19, #0x1c]
{let address=(cpu.x[19]).wrapping_add(0x1cu64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f89f58 str x20, [x8, w9, sxtw #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[9] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64);let v0=cpu.x[20];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f89f5c b.eq #0x10f89fdc
if cpu.condition(0){pc=0x10f89fdc;continue;}pc=0x10f89f60;continue;
}
0x10f89f60=>{
// 0x10f89f60 mov w0, #0x70
cpu.x[0]=(((0x70u64))) as u32 as u64;
// 0x10f89f64 bl #0x1187d2b0
cpu.x[30]=0x10f89f68;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
pc=0x10f89f68;continue;
}
0x10f89f68=>{
// 0x10f89f68 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10f89f6c bl #0x10f86f48
cpu.x[30]=0x10f89f70;super::chunk2::f_10f86f48(cpu,rt,0x10f86f48)?;
pc=0x10f89f70;continue;
}
0x10f89f70=>{
// 0x10f89f70 ldp w9, w8, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10f89f74 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f89f78 b.ne #0x10f89f9c
if cpu.condition(1){pc=0x10f89f9c;continue;}pc=0x10f89f7c;continue;
}
0x10f89f7c=>{
// 0x10f89f7c lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10f89f80 cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f89f84 csel w1, w22, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[22] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10f89f88 cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f89f8c b.ge #0x10f89f9c
if cpu.condition(10){pc=0x10f89f9c;continue;}pc=0x10f89f90;continue;
}
0x10f89f90=>{
// 0x10f89f90 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f89f94 bl #0x10f8b91c
cpu.x[30]=0x10f89f98;super::chunk9::f_10f8b91c(cpu,rt,0x10f8b91c)?;
// 0x10f89f98 ldr w8, [x19, #0xc]
{let address=(cpu.x[19]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
pc=0x10f89f9c;continue;
}
0x10f89f9c=>{
// 0x10f89f9c ldp w10, w9, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[10]=(v0) as u32 as u64;cpu.x[9]=(v1) as u32 as u64;}
// 0x10f89fa0 ldr x11, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[11]=(v0) as u64;}
// 0x10f89fa4 add w12, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[12]=(result) as u32 as u64;}
// 0x10f89fa8 str w12, [x19, #0xc]
{let address=(cpu.x[19]).wrapping_add(0xcu64);let v0=cpu.x[12] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f89fac cmp w9, w10
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f89fb0 str x20, [x11, w8, sxtw #3]
{let address=((cpu.x[11]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64);let v0=cpu.x[20];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f89fb4 b.ne #0x10f89f48
if cpu.condition(1){pc=0x10f89f48;continue;}pc=0x10f89fb8;continue;
}
0x10f89fb8=>{
// 0x10f89fb8 lsl w8, w9, #1
cpu.x[8]=(((((cpu.x[9] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10f89fbc cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f89fc0 csel w1, w22, w8, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[22] as u32 as u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10f89fc4 cmp w9, w1
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f89fc8 b.ge #0x10f89f48
if cpu.condition(10){pc=0x10f89f48;continue;}pc=0x10f89fcc;continue;
}
0x10f89fcc=>{
// 0x10f89fcc add x0, x19, #0x10
{let a=((cpu.x[19]));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10f89fd0 bl #0x10f8b91c
cpu.x[30]=0x10f89fd4;super::chunk9::f_10f8b91c(cpu,rt,0x10f8b91c)?;
pc=0x10f89fd4;continue;
}
0x10f89fd4=>{
// 0x10f89fd4 ldr w9, [x19, #0x1c]
{let address=(cpu.x[19]).wrapping_add(0x1cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f89fd8 b #0x10f89f48
pc=0x10f89f48;continue;
}
0x10f89fdc=>{
// 0x10f89fdc ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f89fe0 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f89fe4 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f89fe8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f89efc")),}}}
#[inline]
pub(super) unsafe fn f_10f8b91c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f8b91c=>{
// 0x10f8b91c stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f8b920 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f8b924 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f8b928 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f8b92c mov w19, w1
cpu.x[19]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x10f8b930 cmp w1, #0
{let a=((cpu.x[1] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8b934 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10f8b938 ubfiz x8, x19, #3, #0x20
cpu.x[8]=(((((cpu.x[19])) & 0xffffffffu64) << 3)) as u64;
// 0x10f8b93c csinv x0, x8, xzr, ge
cpu.x[0]=(if cpu.condition(10){((cpu.x[8]))}else{!(((0u64)))}) as u64;
// 0x10f8b940 bl #0x1187d520
cpu.x[30]=0x10f8b944;super::chunk8::f_1187d520(cpu,rt,0x1187d520)?;
pc=0x10f8b944;continue;
}
0x10f8b944=>{
// 0x10f8b944 ldr w9, [x20, #0xc]
{let address=(cpu.x[20]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f8b948 ldr x8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8b94c mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10f8b950 cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8b954 b.lt #0x10f8b9dc
if cpu.condition(11){pc=0x10f8b9dc;continue;}pc=0x10f8b958;continue;
}
0x10f8b958=>{
// 0x10f8b958 cmp w9, #4
{let a=((cpu.x[9] as u32 as u64));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8b95c b.lo #0x10f8b9bc
if cpu.condition(3){pc=0x10f8b9bc;continue;}pc=0x10f8b960;continue;
}
0x10f8b960=>{
// 0x10f8b960 sub x10, x21, x8
{let a=((cpu.x[21]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.x[10]=(result) as u64;}
// 0x10f8b964 mov x11, x21
cpu.x[11]=(((cpu.x[21]))) as u64;
// 0x10f8b968 cmp x10, #0x20
{let a=((cpu.x[10]));let b=((0x20u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f8b96c mov x10, x8
cpu.x[10]=(((cpu.x[8]))) as u64;
// 0x10f8b970 b.lo #0x10f8b9c4
if cpu.condition(3){pc=0x10f8b9c4;continue;}pc=0x10f8b974;continue;
}
0x10f8b974=>{
// 0x10f8b974 mov w12, w9
cpu.x[12]=(((cpu.x[9] as u32 as u64))) as u32 as u64;
// 0x10f8b978 add x14, x21, #0x10
{let a=((cpu.x[21]));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[14]=(result) as u64;}
// 0x10f8b97c add x15, x8, #0x10
{let a=((cpu.x[8]));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[15]=(result) as u64;}
// 0x10f8b980 and x13, x12, #0x7ffffffc
{let a=((cpu.x[12]));let b=((0x7ffffffcu64));let result=a & b;cpu.x[13]=(result) as u64;}
// 0x10f8b984 lsl x11, x13, #3
cpu.x[11]=(((((cpu.x[13]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10f8b988 sub w9, w9, w13
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[13] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10f8b98c mov x16, x13
cpu.x[16]=(((cpu.x[13]))) as u64;
// 0x10f8b990 add x10, x8, x11
{let a=((cpu.x[8]));let b=((cpu.x[11]));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x10f8b994 add x11, x21, x11
{let a=((cpu.x[21]));let b=((cpu.x[11]));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
pc=0x10f8b998;continue;
}
0x10f8b998=>{
// 0x10f8b998 ldp q0, q1, [x15, #-0x10]
{let address=(cpu.x[15]).wrapping_add(0xfffffffffffffff0u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));let v1=rt.memory.read::<u128>(address.wrapping_add(16));cpu.q[0]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;cpu.q[1]=(v1) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x10f8b99c subs x16, x16, #4
{let a=((cpu.x[16]));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.x[16]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x10f8b9a0 add x15, x15, #0x20
{let a=((cpu.x[15]));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[15]=(result) as u64;}
// 0x10f8b9a4 stp q0, q1, [x14, #-0x10]
{let address=(cpu.x[14]).wrapping_add(0xfffffffffffffff0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[1] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10f8b9a8 add x14, x14, #0x20
{let a=((cpu.x[14]));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[14]=(result) as u64;}
// 0x10f8b9ac b.ne #0x10f8b998
if cpu.condition(1){pc=0x10f8b998;continue;}pc=0x10f8b9b0;continue;
}
0x10f8b9b0=>{
// 0x10f8b9b0 cmp x13, x12
{let a=((cpu.x[13]));let b=((cpu.x[12]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f8b9b4 b.ne #0x10f8b9c4
if cpu.condition(1){pc=0x10f8b9c4;continue;}pc=0x10f8b9b8;continue;
}
0x10f8b9b8=>{
// 0x10f8b9b8 b #0x10f8b9dc
pc=0x10f8b9dc;continue;
}
0x10f8b9bc=>{
// 0x10f8b9bc mov x10, x8
cpu.x[10]=(((cpu.x[8]))) as u64;
// 0x10f8b9c0 mov x11, x21
cpu.x[11]=(((cpu.x[21]))) as u64;
pc=0x10f8b9c4;continue;
}
0x10f8b9c4=>{
// 0x10f8b9c4 add w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
pc=0x10f8b9c8;continue;
}
0x10f8b9c8=>{
// 0x10f8b9c8 ldr x12, [x10], #8
{let address=(cpu.x[10]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[12]=(v0) as u64;cpu.x[10]=((cpu.x[10]).wrapping_add(((0x8u64)))) as u64;}
// 0x10f8b9cc sub w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10f8b9d0 cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f8b9d4 str x12, [x11], #8
{let address=(cpu.x[11]).wrapping_add(0x0u64);let v0=cpu.x[12];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);cpu.x[11]=((cpu.x[11]).wrapping_add(((0x8u64)))) as u64;}
// 0x10f8b9d8 b.hi #0x10f8b9c8
if cpu.condition(8){pc=0x10f8b9c8;continue;}pc=0x10f8b9dc;continue;
}
0x10f8b9dc=>{
// 0x10f8b9dc cbz x8, #0x10f8b9e8
if (cpu.x[8]) == 0 {pc=0x10f8b9e8;continue;}pc=0x10f8b9e0;continue;
}
0x10f8b9e0=>{
// 0x10f8b9e0 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10f8b9e4 bl #0x1187d360
cpu.x[30]=0x10f8b9e8;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x10f8b9e8;continue;
}
0x10f8b9e8=>{
// 0x10f8b9e8 str x21, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f8b9ec str w19, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=cpu.x[19] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f8b9f0 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f8b9f4 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f8b9f8 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f8b9fc ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f8b91c")),}}}
#[inline]
pub(super) unsafe fn f_10f8eed8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f8eed8=>{
// 0x10f8eed8 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f8eedc str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f8eee0 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f8eee4 ldr x8, [x0, #0x48]
{let address=(cpu.x[0]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8eee8 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f8eeec ldr x2, [x8, #0x2b8]
{let address=(cpu.x[8]).wrapping_add(0x2b8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10f8eef0 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10f8eef4 strb w8, [x0, #0x170]
{let address=(cpu.x[0]).wrapping_add(0x170u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f8eef8 cbz x2, #0x10f8ef08
if (cpu.x[2]) == 0 {pc=0x10f8ef08;continue;}pc=0x10f8eefc;continue;
}
0x10f8eefc=>{
// 0x10f8eefc mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8ef00 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f8ef04 bl #0x10f89954
cpu.x[30]=0x10f8ef08;super::chunk11::f_10f89954(cpu,rt,0x10f89954)?;
pc=0x10f8ef08;continue;
}
0x10f8ef08=>{
// 0x10f8ef08 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8ef0c bl #0x10f94058
cpu.x[30]=0x10f8ef10;super::chunk6::f_10f94058(cpu,rt,0x10f94058)?;
pc=0x10f8ef10;continue;
}
0x10f8ef10=>{
// 0x10f8ef10 tbnz w0, #0, #0x10f8ef4c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10f8ef4c;continue;}pc=0x10f8ef14;continue;
}
0x10f8ef14=>{
// 0x10f8ef14 ldrb w8, [x19, #0x1dc]
{let address=(cpu.x[19]).wrapping_add(0x1dcu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f8ef18 cbz w8, #0x10f8ef4c
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f8ef4c;continue;}pc=0x10f8ef1c;continue;
}
0x10f8ef1c=>{
// 0x10f8ef1c ldr x1, [x19, #0x108]
{let address=(cpu.x[19]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10f8ef20 cbz x1, #0x10f8ef4c
if (cpu.x[1]) == 0 {pc=0x10f8ef4c;continue;}pc=0x10f8ef24;continue;
}
0x10f8ef24=>{
// 0x10f8ef24 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f8ef28 bl #0x10f8ef58
cpu.x[30]=0x10f8ef2c;super::chunk6::f_10f8ef58(cpu,rt,0x10f8ef58)?;
// 0x10f8ef2c ldr x8, [x19, #0x108]
{let address=(cpu.x[19]).wrapping_add(0x108u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f8ef30 neg w1, w0
cpu.x[1]=(0u64.wrapping_sub(((cpu.x[0] as u32 as u64)))) as u32 as u64;
// 0x10f8ef34 mov w2, #-1
cpu.x[2]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x10f8ef38 ldr x9, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f8ef3c mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x10f8ef40 ldr x9, [x9, #0x1b8]
{let address=(cpu.x[9]).wrapping_add(0x1b8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f8ef44 blr x9
{let target=cpu.x[9];cpu.x[30]=0x10f8ef48;super::dispatch(cpu,rt,target)?;}
// 0x10f8ef48 strb wzr, [x19, #0x1dc]
{let address=(cpu.x[19]).wrapping_add(0x1dcu64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
pc=0x10f8ef4c;continue;
}
0x10f8ef4c=>{
// 0x10f8ef4c ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x10f8ef50 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10f8ef54 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f8eed8")),}}}
#[inline]
pub(super) unsafe fn f_10f913f4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f913f4=>{
// 0x10f913f4 stp x29, x30, [sp, #-0x60]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffa0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f913f8 stp x28, x27, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[28];let v1=cpu.x[27];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f913fc stp x26, x25, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[26];let v1=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
pc=0x10f91400;continue;
}
0x10f91400=>{
// 0x10f91400 stp x24, x23, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f91404 stp x22, x21, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f91408 stp x20, x19, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f9140c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f91410 ldr x25, [x0, #0x48]
{let address=(cpu.x[0]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[25]=(v0) as u64;}
// 0x10f91414 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f91418 mov w21, w1
cpu.x[21]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x10f9141c mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10f91420 mov w22, w4
cpu.x[22]=(((cpu.x[4] as u32 as u64))) as u32 as u64;
// 0x10f91424 mov w23, w3
cpu.x[23]=(((cpu.x[3] as u32 as u64))) as u32 as u64;
// 0x10f91428 mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10f9142c mov w20, w2
cpu.x[20]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x10f91430 bl #0x10e5ac7c
cpu.x[30]=0x10f91434;super::chunk7::f_10e5ac7c(cpu,rt,0x10e5ac7c)?;
pc=0x10f91434;continue;
}
0x10f91434=>{
// 0x10f91434 tbnz w0, #0, #0x10f91768
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10f91768;continue;}pc=0x10f91438;continue;
}
0x10f91438=>{
// 0x10f91438 ldr w28, [x25, #0x9c]
{let address=(cpu.x[25]).wrapping_add(0x9cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[28]=(v0) as u32 as u64;}
// 0x10f9143c cmp w28, #1
{let a=((cpu.x[28] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f91440 b.lt #0x10f9144c
if cpu.condition(11){pc=0x10f9144c;continue;}pc=0x10f91444;continue;
}
0x10f91444=>{
// 0x10f91444 add x8, x25, #0xa0
{let a=((cpu.x[25]));let b=((0xa0u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10f91448 b #0x10f91454
pc=0x10f91454;continue;
}
0x10f9144c=>{
// 0x10f9144c ldr w28, [x25, #0x168]
{let address=(cpu.x[25]).wrapping_add(0x168u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[28]=(v0) as u32 as u64;}
// 0x10f91450 add x8, x25, #0x16c
{let a=((cpu.x[25]));let b=((0x16cu64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
pc=0x10f91454;continue;
}
0x10f91454=>{
// 0x10f91454 ldr w24, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[24]=(v0) as u32 as u64;}
// 0x10f91458 mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10f9145c mov w1, #0x10
cpu.x[1]=(((0x10u64))) as u32 as u64;
// 0x10f91460 bl #0x10e5ac7c
cpu.x[30]=0x10f91464;super::chunk7::f_10e5ac7c(cpu,rt,0x10e5ac7c)?;
pc=0x10f91464;continue;
}
0x10f91464=>{
// 0x10f91464 ldr x9, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f91468 ldrsw x8, [x9, #0x8c]
{let address=(cpu.x[9]).wrapping_add(0x8cu64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10f9146c cbz w8, #0x10f91768
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f91768;continue;}pc=0x10f91470;continue;
}
0x10f91470=>{
// 0x10f91470 ldr x26, [x9, #0x80]
{let address=(cpu.x[9]).wrapping_add(0x80u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[26]=(v0) as u64;}
// 0x10f91474 add w22, w28, w22
{let a=((cpu.x[28] as u32 as u64));let b=((cpu.x[22] as u32 as u64));let result=a.wrapping_add(b);cpu.x[22]=(result) as u32 as u64;}
// 0x10f91478 cmp w24, #0
{let a=((cpu.x[24] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9147c add x27, x26, x8, lsl #3
{let a=((cpu.x[26]));let b=(((cpu.x[8])) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[27]=(result) as u64;}
// 0x10f91480 tbz w0, #0, #0x10f915f8
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f915f8;continue;}pc=0x10f91484;continue;
}
0x10f91484=>{
// 0x10f91484 b.le #0x10f91568
if cpu.condition(13){pc=0x10f91568;continue;}pc=0x10f91488;continue;
}
0x10f91488=>{
// 0x10f91488 lsl w23, w22, #1
cpu.x[23]=(((((cpu.x[22] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10f9148c lsl w25, w24, #1
cpu.x[25]=(((((cpu.x[24] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10f91490 sub w21, w21, w22
{let a=((cpu.x[21] as u32 as u64));let b=((cpu.x[22] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[21]=(result) as u32 as u64;}
// 0x10f91494 sub w20, w20, w24
{let a=((cpu.x[20] as u32 as u64));let b=((cpu.x[24] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[20]=(result) as u32 as u64;}
// 0x10f91498 b #0x10f914b8
pc=0x10f914b8;continue;
}
0x10f9149c=>{
// 0x10f9149c mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f914a0 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f914a4 bl #0x10f7b46c
cpu.x[30]=0x10f914a8;super::dispatch(cpu,rt,0x10f7b46c)?;
// 0x10f914a8 tbnz w0, #0, #0x10f91788
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10f91788;continue;}pc=0x10f914ac;continue;
}
0x10f914ac=>{
// 0x10f914ac add x26, x26, #8
{let a=((cpu.x[26]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[26]=(result) as u64;}
// 0x10f914b0 cmp x26, x27
{let a=((cpu.x[26]));let b=((cpu.x[27]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f914b4 b.eq #0x10f91768
if cpu.condition(0){pc=0x10f91768;continue;}pc=0x10f914b8;continue;
}
0x10f914b8=>{
// 0x10f914b8 ldr x22, [x26]
{let address=(cpu.x[26]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[22]=(v0) as u64;}
// 0x10f914bc mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f914c0 bl #0x10f79d48
cpu.x[30]=0x10f914c4;super::chunk0::f_10f79d48(cpu,rt,0x10f79d48)?;
// 0x10f914c4 ldrb w8, [x0, #0x188]
{let address=(cpu.x[0]).wrapping_add(0x188u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f914c8 cbz w8, #0x10f914ac
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f914ac;continue;}pc=0x10f914cc;continue;
}
0x10f914cc=>{
// 0x10f914cc ldr x8, [x22]
{let address=(cpu.x[22]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f914d0 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f914d4 ldr x8, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f914d8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f914dc;super::dispatch(cpu,rt,target)?;}
// 0x10f914dc tbnz w0, #0, #0x10f914ac
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10f914ac;continue;}pc=0x10f914e0;continue;
}
0x10f914e0=>{
// 0x10f914e0 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f914e4 mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10f914e8 mov w2, w20
cpu.x[2]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x10f914ec mov w3, w23
cpu.x[3]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10f914f0 mov w4, w25
cpu.x[4]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
// 0x10f914f4 bl #0x10f95488
cpu.x[30]=0x10f914f8;super::chunk6::f_10f95488(cpu,rt,0x10f95488)?;
// 0x10f914f8 tbz w0, #0, #0x10f914ac
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f914ac;continue;}pc=0x10f914fc;continue;
}
0x10f914fc=>{
// 0x10f914fc ldr x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f91500 mov w1, #4
cpu.x[1]=(((0x4u64))) as u32 as u64;
// 0x10f91504 bl #0x10e5ac7c
cpu.x[30]=0x10f91508;super::chunk7::f_10e5ac7c(cpu,rt,0x10e5ac7c)?;
// 0x10f91508 tbz w0, #0, #0x10f9149c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f9149c;continue;}pc=0x10f9150c;continue;
}
0x10f9150c=>{
// 0x10f9150c ldr w8, [x19, #0x148]
{let address=(cpu.x[19]).wrapping_add(0x148u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f91510 cbnz w8, #0x10f9149c
if (cpu.x[8] as u32 as u64) != 0 {pc=0x10f9149c;continue;}pc=0x10f91514;continue;
}
0x10f91514=>{
// 0x10f91514 ldrb w8, [x19, #0x14c]
{let address=(cpu.x[19]).wrapping_add(0x14cu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f91518 cbz w8, #0x10f9149c
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f9149c;continue;}pc=0x10f9151c;continue;
}
0x10f9151c=>{
// 0x10f9151c mov x9, x19
cpu.x[9]=(((cpu.x[19]))) as u64;
pc=0x10f91520;continue;
}
0x10f91520=>{
// 0x10f91520 mov x8, x9
cpu.x[8]=(((cpu.x[9]))) as u64;
// 0x10f91524 ldr x9, [x9, #0x150]
{let address=(cpu.x[9]).wrapping_add(0x150u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f91528 cbnz x9, #0x10f91520
if (cpu.x[9]) != 0 {pc=0x10f91520;continue;}pc=0x10f9152c;continue;
}
0x10f9152c=>{
// 0x10f9152c ldr x8, [x8, #0x168]
{let address=(cpu.x[8]).wrapping_add(0x168u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f91530 cbz x8, #0x10f9149c
if (cpu.x[8]) == 0 {pc=0x10f9149c;continue;}pc=0x10f91534;continue;
}
0x10f91534=>{
// 0x10f91534 ldp w1, w2, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[1]=(v0) as u32 as u64;cpu.x[2]=(v1) as u32 as u64;}
// 0x10f91538 ldr w3, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10f9153c mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10f91540 bl #0x10f95084
cpu.x[30]=0x10f91544;super::chunk12::f_10f95084(cpu,rt,0x10f95084)?;
// 0x10f91544 tbnz w0, #0, #0x10f9149c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10f9149c;continue;}pc=0x10f91548;continue;
}
0x10f91548=>{
// 0x10f91548 b #0x10f914ac
pc=0x10f914ac;continue;
}
0x10f9154c=>{
// 0x10f9154c mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10f91550 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f91554 bl #0x10f7b46c
cpu.x[30]=0x10f91558;super::dispatch(cpu,rt,0x10f7b46c)?;
// 0x10f91558 tbnz w0, #0, #0x10f91788
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10f91788;continue;}pc=0x10f9155c;continue;
}
0x10f9155c=>{
// 0x10f9155c add x26, x26, #8
{let a=((cpu.x[26]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[26]=(result) as u64;}
// 0x10f91560 cmp x26, x27
{let a=((cpu.x[26]));let b=((cpu.x[27]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f91564 b.eq #0x10f91768
if cpu.condition(0){pc=0x10f91768;continue;}pc=0x10f91568;continue;
}
0x10f91568=>{
// 0x10f91568 ldr x23, [x26]
{let address=(cpu.x[26]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10f9156c mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10f91570 bl #0x10f79d48
cpu.x[30]=0x10f91574;super::chunk0::f_10f79d48(cpu,rt,0x10f79d48)?;
pc=0x10f91574;continue;
}
0x10f91574=>{
// 0x10f91574 ldrb w8, [x0, #0x188]
{let address=(cpu.x[0]).wrapping_add(0x188u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f91578 cbz w8, #0x10f9155c
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f9155c;continue;}pc=0x10f9157c;continue;
}
0x10f9157c=>{
// 0x10f9157c ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f91580 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10f91584 ldr x8, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f91588 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f9158c;super::dispatch(cpu,rt,target)?;}
// 0x10f9158c tbnz w0, #0, #0x10f9155c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10f9155c;continue;}pc=0x10f91590;continue;
}
0x10f91590=>{
// 0x10f91590 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10f91594 mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10f91598 mov w2, w20
cpu.x[2]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x10f9159c mov w3, w22
cpu.x[3]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10f915a0 bl #0x10f95434
cpu.x[30]=0x10f915a4;super::chunk7::f_10f95434(cpu,rt,0x10f95434)?;
// 0x10f915a4 tbz w0, #0, #0x10f9155c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f9155c;continue;}pc=0x10f915a8;continue;
}
0x10f915a8=>{
// 0x10f915a8 ldr x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f915ac mov w1, #4
cpu.x[1]=(((0x4u64))) as u32 as u64;
// 0x10f915b0 bl #0x10e5ac7c
cpu.x[30]=0x10f915b4;super::chunk7::f_10e5ac7c(cpu,rt,0x10e5ac7c)?;
// 0x10f915b4 tbz w0, #0, #0x10f9154c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f9154c;continue;}pc=0x10f915b8;continue;
}
0x10f915b8=>{
// 0x10f915b8 ldr w8, [x19, #0x148]
{let address=(cpu.x[19]).wrapping_add(0x148u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f915bc cbnz w8, #0x10f9154c
if (cpu.x[8] as u32 as u64) != 0 {pc=0x10f9154c;continue;}pc=0x10f915c0;continue;
}
0x10f915c0=>{
// 0x10f915c0 ldrb w8, [x19, #0x14c]
{let address=(cpu.x[19]).wrapping_add(0x14cu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f915c4 cbz w8, #0x10f9154c
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f9154c;continue;}pc=0x10f915c8;continue;
}
0x10f915c8=>{
// 0x10f915c8 mov x9, x19
cpu.x[9]=(((cpu.x[19]))) as u64;
pc=0x10f915cc;continue;
}
0x10f915cc=>{
// 0x10f915cc mov x8, x9
cpu.x[8]=(((cpu.x[9]))) as u64;
// 0x10f915d0 ldr x9, [x9, #0x150]
{let address=(cpu.x[9]).wrapping_add(0x150u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f915d4 cbnz x9, #0x10f915cc
if (cpu.x[9]) != 0 {pc=0x10f915cc;continue;}pc=0x10f915d8;continue;
}
0x10f915d8=>{
// 0x10f915d8 ldr x8, [x8, #0x168]
{let address=(cpu.x[8]).wrapping_add(0x168u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f915dc cbz x8, #0x10f9154c
if (cpu.x[8]) == 0 {pc=0x10f9154c;continue;}pc=0x10f915e0;continue;
}
0x10f915e0=>{
// 0x10f915e0 ldp w1, w2, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[1]=(v0) as u32 as u64;cpu.x[2]=(v1) as u32 as u64;}
// 0x10f915e4 ldr w3, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10f915e8 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10f915ec bl #0x10f95084
cpu.x[30]=0x10f915f0;super::chunk12::f_10f95084(cpu,rt,0x10f95084)?;
// 0x10f915f0 tbnz w0, #0, #0x10f9154c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10f9154c;continue;}pc=0x10f915f4;continue;
}
0x10f915f4=>{
// 0x10f915f4 b #0x10f9155c
pc=0x10f9155c;continue;
}
0x10f915f8=>{
// 0x10f915f8 b.gt #0x10f9161c
if cpu.condition(12){pc=0x10f9161c;continue;}pc=0x10f915fc;continue;
}
0x10f915fc=>{
// 0x10f915fc b #0x10f916d4
pc=0x10f916d4;continue;
}
0x10f91600=>{
// 0x10f91600 mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10f91604 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f91608 bl #0x10f7b46c
cpu.x[30]=0x10f9160c;super::dispatch(cpu,rt,0x10f7b46c)?;
// 0x10f9160c tbnz w0, #0, #0x10f91788
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10f91788;continue;}pc=0x10f91610;continue;
}
0x10f91610=>{
// 0x10f91610 add x26, x26, #8
{let a=((cpu.x[26]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[26]=(result) as u64;}
// 0x10f91614 cmp x26, x27
{let a=((cpu.x[26]));let b=((cpu.x[27]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f91618 b.eq #0x10f91768
if cpu.condition(0){pc=0x10f91768;continue;}pc=0x10f9161c;continue;
}
0x10f9161c=>{
// 0x10f9161c ldr x25, [x26]
{let address=(cpu.x[26]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[25]=(v0) as u64;}
// 0x10f91620 mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10f91624 bl #0x10f79d48
cpu.x[30]=0x10f91628;super::chunk0::f_10f79d48(cpu,rt,0x10f79d48)?;
pc=0x10f91628;continue;
}
0x10f91628=>{
// 0x10f91628 ldrb w8, [x0, #0x188]
{let address=(cpu.x[0]).wrapping_add(0x188u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f9162c cbz w8, #0x10f91610
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f91610;continue;}pc=0x10f91630;continue;
}
0x10f91630=>{
// 0x10f91630 ldr x8, [x25]
{let address=(cpu.x[25]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f91634 mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10f91638 ldr x8, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f9163c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f91640;super::dispatch(cpu,rt,target)?;}
// 0x10f91640 tbnz w0, #0, #0x10f91610
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10f91610;continue;}pc=0x10f91644;continue;
}
0x10f91644=>{
// 0x10f91644 mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10f91648 mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10f9164c mov w2, w20
cpu.x[2]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x10f91650 mov w3, w23
cpu.x[3]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10f91654 mov w4, w22
cpu.x[4]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10f91658 mov w5, w24
cpu.x[5]=(((cpu.x[24] as u32 as u64))) as u32 as u64;
// 0x10f9165c mov w6, w22
cpu.x[6]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10f91660 bl #0x10f957b8
cpu.x[30]=0x10f91664;super::dispatch(cpu,rt,0x10f957b8)?;
// 0x10f91664 tbz w0, #0, #0x10f91610
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f91610;continue;}pc=0x10f91668;continue;
}
0x10f91668=>{
// 0x10f91668 ldr x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f9166c mov w1, #4
cpu.x[1]=(((0x4u64))) as u32 as u64;
// 0x10f91670 bl #0x10e5ac7c
cpu.x[30]=0x10f91674;super::chunk7::f_10e5ac7c(cpu,rt,0x10e5ac7c)?;
// 0x10f91674 tbz w0, #0, #0x10f91600
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f91600;continue;}pc=0x10f91678;continue;
}
0x10f91678=>{
// 0x10f91678 ldr w8, [x19, #0x148]
{let address=(cpu.x[19]).wrapping_add(0x148u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f9167c cbnz w8, #0x10f91600
if (cpu.x[8] as u32 as u64) != 0 {pc=0x10f91600;continue;}pc=0x10f91680;continue;
}
0x10f91680=>{
// 0x10f91680 ldrb w8, [x19, #0x14c]
{let address=(cpu.x[19]).wrapping_add(0x14cu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f91684 cbz w8, #0x10f91600
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f91600;continue;}pc=0x10f91688;continue;
}
0x10f91688=>{
// 0x10f91688 mov x9, x19
cpu.x[9]=(((cpu.x[19]))) as u64;
pc=0x10f9168c;continue;
}
0x10f9168c=>{
// 0x10f9168c mov x8, x9
cpu.x[8]=(((cpu.x[9]))) as u64;
// 0x10f91690 ldr x9, [x9, #0x150]
{let address=(cpu.x[9]).wrapping_add(0x150u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f91694 cbnz x9, #0x10f9168c
if (cpu.x[9]) != 0 {pc=0x10f9168c;continue;}pc=0x10f91698;continue;
}
0x10f91698=>{
// 0x10f91698 ldr x8, [x8, #0x168]
{let address=(cpu.x[8]).wrapping_add(0x168u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f9169c cbz x8, #0x10f91600
if (cpu.x[8]) == 0 {pc=0x10f91600;continue;}pc=0x10f916a0;continue;
}
0x10f916a0=>{
// 0x10f916a0 ldp w1, w2, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[1]=(v0) as u32 as u64;cpu.x[2]=(v1) as u32 as u64;}
// 0x10f916a4 ldr w3, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10f916a8 mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10f916ac bl #0x10f95084
cpu.x[30]=0x10f916b0;super::chunk12::f_10f95084(cpu,rt,0x10f95084)?;
// 0x10f916b0 tbnz w0, #0, #0x10f91600
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10f91600;continue;}pc=0x10f916b4;continue;
}
0x10f916b4=>{
// 0x10f916b4 b #0x10f91610
pc=0x10f91610;continue;
}
0x10f916b8=>{
// 0x10f916b8 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10f916bc mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10f916c0 bl #0x10f7b46c
cpu.x[30]=0x10f916c4;super::dispatch(cpu,rt,0x10f7b46c)?;
// 0x10f916c4 tbnz w0, #0, #0x10f91788
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10f91788;continue;}pc=0x10f916c8;continue;
}
0x10f916c8=>{
// 0x10f916c8 add x26, x26, #8
{let a=((cpu.x[26]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[26]=(result) as u64;}
// 0x10f916cc cmp x26, x27
{let a=((cpu.x[26]));let b=((cpu.x[27]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f916d0 b.eq #0x10f91768
if cpu.condition(0){pc=0x10f91768;continue;}pc=0x10f916d4;continue;
}
0x10f916d4=>{
// 0x10f916d4 ldr x24, [x26]
{let address=(cpu.x[26]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[24]=(v0) as u64;}
// 0x10f916d8 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10f916dc bl #0x10f79d48
cpu.x[30]=0x10f916e0;super::chunk0::f_10f79d48(cpu,rt,0x10f79d48)?;
pc=0x10f916e0;continue;
}
0x10f916e0=>{
// 0x10f916e0 ldrb w8, [x0, #0x188]
{let address=(cpu.x[0]).wrapping_add(0x188u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f916e4 cbz w8, #0x10f916c8
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f916c8;continue;}pc=0x10f916e8;continue;
}
0x10f916e8=>{
// 0x10f916e8 ldr x8, [x24]
{let address=(cpu.x[24]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f916ec mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10f916f0 ldr x8, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f916f4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10f916f8;super::dispatch(cpu,rt,target)?;}
// 0x10f916f8 tbnz w0, #0, #0x10f916c8
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10f916c8;continue;}pc=0x10f916fc;continue;
}
0x10f916fc=>{
// 0x10f916fc mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10f91700 mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10f91704 mov w2, w20
cpu.x[2]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x10f91708 mov w3, w23
cpu.x[3]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10f9170c mov w4, w22
cpu.x[4]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10f91710 bl #0x10f95748
cpu.x[30]=0x10f91714;super::dispatch(cpu,rt,0x10f95748)?;
// 0x10f91714 tbz w0, #0, #0x10f916c8
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f916c8;continue;}pc=0x10f91718;continue;
}
0x10f91718=>{
// 0x10f91718 ldr x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f9171c mov w1, #4
cpu.x[1]=(((0x4u64))) as u32 as u64;
// 0x10f91720 bl #0x10e5ac7c
cpu.x[30]=0x10f91724;super::chunk7::f_10e5ac7c(cpu,rt,0x10e5ac7c)?;
// 0x10f91724 tbz w0, #0, #0x10f916b8
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f916b8;continue;}pc=0x10f91728;continue;
}
0x10f91728=>{
// 0x10f91728 ldr w8, [x19, #0x148]
{let address=(cpu.x[19]).wrapping_add(0x148u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f9172c cbnz w8, #0x10f916b8
if (cpu.x[8] as u32 as u64) != 0 {pc=0x10f916b8;continue;}pc=0x10f91730;continue;
}
0x10f91730=>{
// 0x10f91730 ldrb w8, [x19, #0x14c]
{let address=(cpu.x[19]).wrapping_add(0x14cu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f91734 cbz w8, #0x10f916b8
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f916b8;continue;}pc=0x10f91738;continue;
}
0x10f91738=>{
// 0x10f91738 mov x9, x19
cpu.x[9]=(((cpu.x[19]))) as u64;
pc=0x10f9173c;continue;
}
0x10f9173c=>{
// 0x10f9173c mov x8, x9
cpu.x[8]=(((cpu.x[9]))) as u64;
// 0x10f91740 ldr x9, [x9, #0x150]
{let address=(cpu.x[9]).wrapping_add(0x150u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f91744 cbnz x9, #0x10f9173c
if (cpu.x[9]) != 0 {pc=0x10f9173c;continue;}pc=0x10f91748;continue;
}
0x10f91748=>{
// 0x10f91748 ldr x8, [x8, #0x168]
{let address=(cpu.x[8]).wrapping_add(0x168u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f9174c cbz x8, #0x10f916b8
if (cpu.x[8]) == 0 {pc=0x10f916b8;continue;}pc=0x10f91750;continue;
}
0x10f91750=>{
// 0x10f91750 ldp w1, w2, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[1]=(v0) as u32 as u64;cpu.x[2]=(v1) as u32 as u64;}
// 0x10f91754 ldr w3, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[3]=(v0) as u32 as u64;}
// 0x10f91758 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10f9175c bl #0x10f95084
cpu.x[30]=0x10f91760;super::chunk12::f_10f95084(cpu,rt,0x10f95084)?;
// 0x10f91760 tbnz w0, #0, #0x10f916b8
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10f916b8;continue;}pc=0x10f91764;continue;
}
0x10f91764=>{
// 0x10f91764 b #0x10f916c8
pc=0x10f916c8;continue;
}
0x10f91768=>{
// 0x10f91768 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
pc=0x10f9176c;continue;
}
0x10f9176c=>{
// 0x10f9176c ldp x20, x19, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f91770 ldp x22, x21, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f91774 ldp x24, x23, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10f91778 ldp x26, x25, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10f9177c ldp x28, x27, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10f91780 ldp x29, x30, [sp], #0x60
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x60u64)))) as u64;}
// 0x10f91784 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10f91788=>{
// 0x10f91788 mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
// 0x10f9178c b #0x10f9176c
pc=0x10f9176c;continue;
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f913f4")),}}}
#[inline]
pub(super) unsafe fn f_10f935dc(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f935dc=>{
// 0x10f935dc mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
// 0x10f935e0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f935dc")),}}}
#[inline]
pub(super) unsafe fn f_10f940fc(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f940fc=>{
// 0x10f940fc ldrb w9, [x0, #0x30]
{let address=(cpu.x[0]).wrapping_add(0x30u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f94100 mov x8, x0
cpu.x[8]=(((cpu.x[0]))) as u64;
// 0x10f94104 mov x0, xzr
cpu.x[0]=(((0u64))) as u64;
// 0x10f94108 tbz w9, #3, #0x10f94120
if ((cpu.x[9] as u32 as u64) & (1u64 << 3)) == 0 {pc=0x10f94120;continue;}pc=0x10f9410c;continue;
}
0x10f9410c=>{
// 0x10f9410c ldr w9, [x8, #0x24]
{let address=(cpu.x[8]).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f94110 cmp w9, #4
{let a=((cpu.x[9] as u32 as u64));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f94114 b.lt #0x10f94120
if cpu.condition(11){pc=0x10f94120;continue;}pc=0x10f94118;continue;
}
0x10f94118=>{
// 0x10f94118 ldr x8, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f9411c ldr x0, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
pc=0x10f94120;continue;
}
0x10f94120=>{
// 0x10f94120 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f940fc")),}}}
#[inline]
pub(super) unsafe fn f_10f996f4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f996f4=>{
// 0x10f996f4 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f996f8 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f996fc stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f99700 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f99704 ldrsw x8, [x0, #0x1c]
{let address=(cpu.x[0]).wrapping_add(0x1cu64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10f99708 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10f9970c cbz w8, #0x10f99738
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f99738;continue;}pc=0x10f99710;continue;
}
0x10f99710=>{
// 0x10f99710 ldr x20, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10f99714 lsl x21, x8, #3
cpu.x[21]=(((((cpu.x[8]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10f99718 b #0x10f99728
pc=0x10f99728;continue;
}
0x10f9971c=>{
// 0x10f9971c subs x21, x21, #8
{let a=((cpu.x[21]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.x[21]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x10f99720 add x20, x20, #8
{let a=((cpu.x[20]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[20]=(result) as u64;}
// 0x10f99724 b.eq #0x10f99738
if cpu.condition(0){pc=0x10f99738;continue;}pc=0x10f99728;continue;
}
0x10f99728=>{
// 0x10f99728 ldr x0, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f9972c cbz x0, #0x10f9971c
if (cpu.x[0]) == 0 {pc=0x10f9971c;continue;}pc=0x10f99730;continue;
}
0x10f99730=>{
// 0x10f99730 bl #0x1187d2c0
cpu.x[30]=0x10f99734;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10f99734;continue;
}
0x10f99734=>{
// 0x10f99734 b #0x10f9971c
pc=0x10f9971c;continue;
}
0x10f99738=>{
// 0x10f99738 ldrsw x8, [x19, #0x2c]
{let address=(cpu.x[19]).wrapping_add(0x2cu64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10f9973c str wzr, [x19, #0x1c]
{let address=(cpu.x[19]).wrapping_add(0x1cu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f99740 cbz w8, #0x10f9976c
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10f9976c;continue;}pc=0x10f99744;continue;
}
0x10f99744=>{
// 0x10f99744 ldr x20, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10f99748 lsl x21, x8, #3
cpu.x[21]=(((((cpu.x[8]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10f9974c b #0x10f9975c
pc=0x10f9975c;continue;
}
0x10f99750=>{
// 0x10f99750 subs x21, x21, #8
{let a=((cpu.x[21]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.x[21]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x10f99754 add x20, x20, #8
{let a=((cpu.x[20]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[20]=(result) as u64;}
// 0x10f99758 b.eq #0x10f9976c
if cpu.condition(0){pc=0x10f9976c;continue;}pc=0x10f9975c;continue;
}
0x10f9975c=>{
// 0x10f9975c ldr x0, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10f99760 cbz x0, #0x10f99750
if (cpu.x[0]) == 0 {pc=0x10f99750;continue;}pc=0x10f99764;continue;
}
0x10f99764=>{
// 0x10f99764 bl #0x1187d2c0
cpu.x[30]=0x10f99768;super::dispatch(cpu,rt,0x1187d2c0)?;
// 0x10f99768 b #0x10f99750
pc=0x10f99750;continue;
}
0x10f9976c=>{
// 0x10f9976c str wzr, [x19, #0x2c]
{let address=(cpu.x[19]).wrapping_add(0x2cu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f99770 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f99774 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f99778 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f9977c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f996f4")),}}}
#[inline]
pub(super) unsafe fn f_10f9a064(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f9a064=>{
// 0x10f9a064 sub sp, sp, #0x80
{let a=((cpu.sp));let b=((0x80u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x10f9a068 stp x29, x30, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f9a06c stp x28, x27, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[28];let v1=cpu.x[27];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f9a070 stp x26, x25, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[26];let v1=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f9a074 stp x24, x23, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f9a078 stp x22, x21, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f9a07c stp x20, x19, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f9a080 add x29, sp, #0x20
{let a=((cpu.sp));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x10f9a084 mov x19, x6
cpu.x[19]=(((cpu.x[6]))) as u64;
// 0x10f9a088 stp x5, x4, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=cpu.x[5];let v1=cpu.x[4];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f9a08c stur w2, [x29, #-4]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffffcu64);let v0=cpu.x[2] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f9a090 cbz x6, #0x10f9a2f0
if (cpu.x[6]) == 0 {pc=0x10f9a2f0;continue;}pc=0x10f9a094;continue;
}
0x10f9a094=>{
// 0x10f9a094 sub w8, w1, w3
{let a=((cpu.x[1] as u32 as u64));let b=((cpu.x[3] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10f9a098 add w9, w3, w1
{let a=((cpu.x[3] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10f9a09c mov w22, w3
cpu.x[22]=(((cpu.x[3] as u32 as u64))) as u32 as u64;
// 0x10f9a0a0 asr w27, w8, #0xa
cpu.x[27]=(((((cpu.x[8] as u32 as u64))) as i32).wrapping_shr(((((0xau64))) & 31) as u32) as u64) as u32 as u64;
// 0x10f9a0a4 asr w8, w9, #0xa
cpu.x[8]=(((((cpu.x[9] as u32 as u64))) as i32).wrapping_shr(((((0xau64))) & 31) as u32) as u64) as u32 as u64;
// 0x10f9a0a8 mov w24, w1
cpu.x[24]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x10f9a0ac str wzr, [x19, #0xc]
{let address=(cpu.x[19]).wrapping_add(0xcu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f9a0b0 cmp w27, w8
{let a=((cpu.x[27] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9a0b4 b.gt #0x10f9a2f0
if cpu.condition(12){pc=0x10f9a2f0;continue;}pc=0x10f9a0b8;continue;
}
0x10f9a0b8=>{
// 0x10f9a0b8 ldur w10, [x29, #-4]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffffcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10f9a0bc sub w9, w10, w22
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[22] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10f9a0c0 add w10, w22, w10
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10f9a0c4 asr w9, w9, #0xa
cpu.x[9]=(((((cpu.x[9] as u32 as u64))) as i32).wrapping_shr(((((0xau64))) & 31) as u32) as u64) as u32 as u64;
// 0x10f9a0c8 asr w10, w10, #0xa
cpu.x[10]=(((((cpu.x[10] as u32 as u64))) as i32).wrapping_shr(((((0xau64))) & 31) as u32) as u64) as u32 as u64;
// 0x10f9a0cc cmp w9, w10
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9a0d0 str w9, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f9a0d4 stur w10, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f9a0d8 b.gt #0x10f9a2f0
if cpu.condition(12){pc=0x10f9a2f0;continue;}pc=0x10f9a0dc;continue;
}
0x10f9a0dc=>{
// 0x10f9a0dc cmp w27, w8
{let a=((cpu.x[27] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9a0e0 mov x25, x0
cpu.x[25]=(((cpu.x[0]))) as u64;
// 0x10f9a0e4 csel w8, w27, w8, gt
cpu.x[8]=(if cpu.condition(12){((cpu.x[27] as u32 as u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x10f9a0e8 str w8, [sp, #4]
{let address=(cpu.sp).wrapping_add(0x4u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f9a0ec b #0x10f9a100
pc=0x10f9a100;continue;
}
0x10f9a0f0=>{
// 0x10f9a0f0 ldr w8, [sp, #4]
{let address=(cpu.sp).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f9a0f4 cmp w27, w8
{let a=((cpu.x[27] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9a0f8 add w27, w27, #1
{let a=((cpu.x[27] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[27]=(result) as u32 as u64;}
// 0x10f9a0fc b.eq #0x10f9a28c
if cpu.condition(0){pc=0x10f9a28c;continue;}pc=0x10f9a100;continue;
}
0x10f9a100=>{
// 0x10f9a100 tbnz w27, #0x1f, #0x10f9a0f0
if ((cpu.x[27] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x10f9a0f0;continue;}pc=0x10f9a104;continue;
}
0x10f9a104=>{
// 0x10f9a104 ldr w8, [x25]
{let address=(cpu.x[25]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f9a108 ldr w20, [sp]
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[20]=(v0) as u32 as u64;}
// 0x10f9a10c cmp w8, w27
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[27] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9a110 b.gt #0x10f9a128
if cpu.condition(12){pc=0x10f9a128;continue;}pc=0x10f9a114;continue;
}
0x10f9a114=>{
// 0x10f9a114 b #0x10f9a0f0
pc=0x10f9a0f0;continue;
}
0x10f9a118=>{
// 0x10f9a118 ldur w8, [x29, #-8]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f9a11c cmp w20, w8
{let a=((cpu.x[20] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9a120 add w20, w20, #1
{let a=((cpu.x[20] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[20]=(result) as u32 as u64;}
// 0x10f9a124 b.eq #0x10f9a0f0
if cpu.condition(0){pc=0x10f9a0f0;continue;}pc=0x10f9a128;continue;
}
0x10f9a128=>{
// 0x10f9a128 ldr w8, [x25]
{let address=(cpu.x[25]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f9a12c cmp w8, w27
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[27] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9a130 b.le #0x10f9a118
if cpu.condition(13){pc=0x10f9a118;continue;}pc=0x10f9a134;continue;
}
0x10f9a134=>{
// 0x10f9a134 tbnz w20, #0x1f, #0x10f9a118
if ((cpu.x[20] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x10f9a118;continue;}pc=0x10f9a138;continue;
}
0x10f9a138=>{
// 0x10f9a138 ldr w9, [x25, #4]
{let address=(cpu.x[25]).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10f9a13c cmp w9, w20
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[20] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9a140 b.le #0x10f9a118
if cpu.condition(13){pc=0x10f9a118;continue;}pc=0x10f9a144;continue;
}
0x10f9a144=>{
// 0x10f9a144 madd w8, w8, w20, w27
cpu.x[8]=((((cpu.x[27] as u32 as u64))).wrapping_add((((cpu.x[8] as u32 as u64))).wrapping_mul(((cpu.x[20] as u32 as u64))))) as u32 as u64;
// 0x10f9a148 ldr x9, [x25, #0x10]
{let address=(cpu.x[25]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f9a14c ldr x21, [x9, w8, sxtw #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f9a150 cbz x21, #0x10f9a118
if (cpu.x[21]) == 0 {pc=0x10f9a118;continue;}pc=0x10f9a154;continue;
}
0x10f9a154=>{
// 0x10f9a154 ldr w8, [x21, #0xc]
{let address=(cpu.x[21]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f9a158 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9a15c b.lt #0x10f9a118
if cpu.condition(11){pc=0x10f9a118;continue;}pc=0x10f9a160;continue;
}
0x10f9a160=>{
// 0x10f9a160 lsl x28, x8, #3
cpu.x[28]=(((((cpu.x[8]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10f9a164 ldr x8, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f9a168 cbz x8, #0x10f9a204
if (cpu.x[8]) == 0 {pc=0x10f9a204;continue;}pc=0x10f9a16c;continue;
}
0x10f9a16c=>{
// 0x10f9a16c mov x23, xzr
cpu.x[23]=(((0u64))) as u64;
// 0x10f9a170 b #0x10f9a198
pc=0x10f9a198;continue;
}
0x10f9a174=>{
// 0x10f9a174 ldr x9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f9a178 add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10f9a17c str w10, [x19, #0xc]
{let address=(cpu.x[19]).wrapping_add(0xcu64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f9a180 str x26, [x9, w8, sxtw #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64);let v0=cpu.x[26];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f9a184 mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10f9a188 strb w8, [x26, #0x50]
{let address=(cpu.x[26]).wrapping_add(0x50u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
pc=0x10f9a18c;continue;
}
0x10f9a18c=>{
// 0x10f9a18c add x23, x23, #8
{let a=((cpu.x[23]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[23]=(result) as u64;}
// 0x10f9a190 cmp x28, x23
{let a=((cpu.x[28]));let b=((cpu.x[23]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f9a194 b.eq #0x10f9a118
if cpu.condition(0){pc=0x10f9a118;continue;}pc=0x10f9a198;continue;
}
0x10f9a198=>{
// 0x10f9a198 ldr x8, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f9a19c ldr x26, [x8, x23]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[23])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[26]=(v0) as u64;}
// 0x10f9a1a0 ldrb w8, [x26, #0x50]
{let address=(cpu.x[26]).wrapping_add(0x50u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f9a1a4 cbnz w8, #0x10f9a18c
if (cpu.x[8] as u32 as u64) != 0 {pc=0x10f9a18c;continue;}pc=0x10f9a1a8;continue;
}
0x10f9a1a8=>{
// 0x10f9a1a8 ldur w2, [x29, #-4]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffffcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[2]=(v0) as u32 as u64;}
// 0x10f9a1ac mov x0, x26
cpu.x[0]=(((cpu.x[26]))) as u64;
// 0x10f9a1b0 mov w1, w24
cpu.x[1]=(((cpu.x[24] as u32 as u64))) as u32 as u64;
// 0x10f9a1b4 mov w3, w22
cpu.x[3]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10f9a1b8 bl #0x10f95084
cpu.x[30]=0x10f9a1bc;super::chunk12::f_10f95084(cpu,rt,0x10f95084)?;
pc=0x10f9a1bc;continue;
}
0x10f9a1bc=>{
// 0x10f9a1bc tbz w0, #0, #0x10f9a18c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f9a18c;continue;}pc=0x10f9a1c0;continue;
}
0x10f9a1c0=>{
// 0x10f9a1c0 ldp x2, x0, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[2]=(v0) as u64;cpu.x[0]=(v1) as u64;}
// 0x10f9a1c4 mov x1, x26
cpu.x[1]=(((cpu.x[26]))) as u64;
// 0x10f9a1c8 bl #0x10e31fc8
cpu.x[30]=0x10f9a1cc;super::chunk7::f_10e31fc8(cpu,rt,0x10e31fc8)?;
pc=0x10f9a1cc;continue;
}
0x10f9a1cc=>{
// 0x10f9a1cc tbz w0, #0, #0x10f9a18c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f9a18c;continue;}pc=0x10f9a1d0;continue;
}
0x10f9a1d0=>{
// 0x10f9a1d0 ldp w9, w8, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10f9a1d4 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9a1d8 b.ne #0x10f9a174
if cpu.condition(1){pc=0x10f9a174;continue;}pc=0x10f9a1dc;continue;
}
0x10f9a1dc=>{
// 0x10f9a1dc lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10f9a1e0 mov w10, #5
cpu.x[10]=(((0x5u64))) as u32 as u64;
// 0x10f9a1e4 cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9a1e8 csel w1, w10, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[10] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10f9a1ec cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9a1f0 b.ge #0x10f9a174
if cpu.condition(10){pc=0x10f9a174;continue;}pc=0x10f9a1f4;continue;
}
0x10f9a1f4=>{
// 0x10f9a1f4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f9a1f8 bl #0x108d7c9c
cpu.x[30]=0x10f9a1fc;super::chunk10::f_108d7c9c(cpu,rt,0x108d7c9c)?;
pc=0x10f9a1fc;continue;
}
0x10f9a1fc=>{
// 0x10f9a1fc ldr w8, [x19, #0xc]
{let address=(cpu.x[19]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f9a200 b #0x10f9a174
pc=0x10f9a174;continue;
}
0x10f9a204=>{
// 0x10f9a204 mov x23, xzr
cpu.x[23]=(((0u64))) as u64;
// 0x10f9a208 b #0x10f9a230
pc=0x10f9a230;continue;
}
0x10f9a20c=>{
// 0x10f9a20c ldr x9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f9a210 add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10f9a214 str w10, [x19, #0xc]
{let address=(cpu.x[19]).wrapping_add(0xcu64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f9a218 str x26, [x9, w8, sxtw #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64);let v0=cpu.x[26];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f9a21c mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x10f9a220 strb w8, [x26, #0x50]
{let address=(cpu.x[26]).wrapping_add(0x50u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
pc=0x10f9a224;continue;
}
0x10f9a224=>{
// 0x10f9a224 add x23, x23, #8
{let a=((cpu.x[23]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[23]=(result) as u64;}
// 0x10f9a228 cmp x28, x23
{let a=((cpu.x[28]));let b=((cpu.x[23]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f9a22c b.eq #0x10f9a118
if cpu.condition(0){pc=0x10f9a118;continue;}pc=0x10f9a230;continue;
}
0x10f9a230=>{
// 0x10f9a230 ldr x8, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10f9a234 ldr x26, [x8, x23]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[23])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[26]=(v0) as u64;}
// 0x10f9a238 ldrb w8, [x26, #0x50]
{let address=(cpu.x[26]).wrapping_add(0x50u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f9a23c cbnz w8, #0x10f9a224
if (cpu.x[8] as u32 as u64) != 0 {pc=0x10f9a224;continue;}pc=0x10f9a240;continue;
}
0x10f9a240=>{
// 0x10f9a240 ldur w2, [x29, #-4]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffffcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[2]=(v0) as u32 as u64;}
// 0x10f9a244 mov x0, x26
cpu.x[0]=(((cpu.x[26]))) as u64;
// 0x10f9a248 mov w1, w24
cpu.x[1]=(((cpu.x[24] as u32 as u64))) as u32 as u64;
// 0x10f9a24c mov w3, w22
cpu.x[3]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10f9a250 bl #0x10f95084
cpu.x[30]=0x10f9a254;super::chunk12::f_10f95084(cpu,rt,0x10f95084)?;
// 0x10f9a254 tbz w0, #0, #0x10f9a224
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10f9a224;continue;}pc=0x10f9a258;continue;
}
0x10f9a258=>{
// 0x10f9a258 ldp w9, w8, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10f9a25c cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9a260 b.ne #0x10f9a20c
if cpu.condition(1){pc=0x10f9a20c;continue;}pc=0x10f9a264;continue;
}
0x10f9a264=>{
// 0x10f9a264 lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10f9a268 mov w10, #5
cpu.x[10]=(((0x5u64))) as u32 as u64;
// 0x10f9a26c cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9a270 csel w1, w10, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[10] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10f9a274 cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9a278 b.ge #0x10f9a20c
if cpu.condition(10){pc=0x10f9a20c;continue;}pc=0x10f9a27c;continue;
}
0x10f9a27c=>{
// 0x10f9a27c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f9a280 bl #0x108d7c9c
cpu.x[30]=0x10f9a284;super::chunk10::f_108d7c9c(cpu,rt,0x108d7c9c)?;
// 0x10f9a284 ldr w8, [x19, #0xc]
{let address=(cpu.x[19]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f9a288 b #0x10f9a20c
pc=0x10f9a20c;continue;
}
0x10f9a28c=>{
// 0x10f9a28c ldr w8, [x19, #0xc]
{let address=(cpu.x[19]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f9a290 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9a294 b.lt #0x10f9a2f0
if cpu.condition(11){pc=0x10f9a2f0;continue;}pc=0x10f9a298;continue;
}
0x10f9a298=>{
// 0x10f9a298 ldr x9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f9a29c cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9a2a0 b.ne #0x10f9a2ac
if cpu.condition(1){pc=0x10f9a2ac;continue;}pc=0x10f9a2a4;continue;
}
0x10f9a2a4=>{
// 0x10f9a2a4 mov x10, xzr
cpu.x[10]=(((0u64))) as u64;
// 0x10f9a2a8 b #0x10f9a2d8
pc=0x10f9a2d8;continue;
}
0x10f9a2ac=>{
// 0x10f9a2ac and x10, x8, #0x7ffffffe
{let a=((cpu.x[8]));let b=((0x7ffffffeu64));let result=a & b;cpu.x[10]=(result) as u64;}
// 0x10f9a2b0 add x11, x9, #8
{let a=((cpu.x[9]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
// 0x10f9a2b4 mov x12, x10
cpu.x[12]=(((cpu.x[10]))) as u64;
pc=0x10f9a2b8;continue;
}
0x10f9a2b8=>{
// 0x10f9a2b8 ldp x13, x14, [x11, #-8]
{let address=(cpu.x[11]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[13]=(v0) as u64;cpu.x[14]=(v1) as u64;}
// 0x10f9a2bc subs x12, x12, #2
{let a=((cpu.x[12]));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.x[12]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x10f9a2c0 add x11, x11, #0x10
{let a=((cpu.x[11]));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
// 0x10f9a2c4 strb wzr, [x13, #0x50]
{let address=(cpu.x[13]).wrapping_add(0x50u64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f9a2c8 strb wzr, [x14, #0x50]
{let address=(cpu.x[14]).wrapping_add(0x50u64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f9a2cc b.ne #0x10f9a2b8
if cpu.condition(1){pc=0x10f9a2b8;continue;}pc=0x10f9a2d0;continue;
}
0x10f9a2d0=>{
// 0x10f9a2d0 cmp x10, x8
{let a=((cpu.x[10]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10f9a2d4 b.eq #0x10f9a2f0
if cpu.condition(0){pc=0x10f9a2f0;continue;}pc=0x10f9a2d8;continue;
}
0x10f9a2d8=>{
// 0x10f9a2d8 add x9, x9, x10, lsl #3
{let a=((cpu.x[9]));let b=(((cpu.x[10])) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
// 0x10f9a2dc sub x8, x8, x10
{let a=((cpu.x[8]));let b=((cpu.x[10]));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u64;}
pc=0x10f9a2e0;continue;
}
0x10f9a2e0=>{
// 0x10f9a2e0 ldr x10, [x9], #8
{let address=(cpu.x[9]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;cpu.x[9]=((cpu.x[9]).wrapping_add(((0x8u64)))) as u64;}
// 0x10f9a2e4 subs x8, x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x10f9a2e8 strb wzr, [x10, #0x50]
{let address=(cpu.x[10]).wrapping_add(0x50u64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10f9a2ec b.ne #0x10f9a2e0
if cpu.condition(1){pc=0x10f9a2e0;continue;}pc=0x10f9a2f0;continue;
}
0x10f9a2f0=>{
// 0x10f9a2f0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10f9a2f4 ldp x20, x19, [sp, #0x70]
{let address=(cpu.sp).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f9a2f8 ldp x22, x21, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10f9a2fc ldp x24, x23, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10f9a300 ldp x26, x25, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10f9a304 ldp x28, x27, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10f9a308 ldp x29, x30, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10f9a30c add sp, sp, #0x80
{let a=((cpu.sp));let b=((0x80u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10f9a310 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f9a064")),}}}
#[inline]
pub(super) unsafe fn f_10f9abb8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10f9abb8=>{
// 0x10f9abb8 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10f9abbc str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f9abc0 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10f9abc4 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10f9abc8 ldr x21, [x1]
{let address=(cpu.x[1]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f9abcc cbz x21, #0x10f9ac40
if (cpu.x[21]) == 0 {pc=0x10f9ac40;continue;}pc=0x10f9abd0;continue;
}
0x10f9abd0=>{
// 0x10f9abd0 ldr w8, [x0, #0x24]
{let address=(cpu.x[0]).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f9abd4 mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10f9abd8 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10f9abdc cmp w8, #0xa
{let a=((cpu.x[8] as u32 as u64));let b=((0xau64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9abe0 b.lt #0x10f9abfc
if cpu.condition(11){pc=0x10f9abfc;continue;}pc=0x10f9abe4;continue;
}
0x10f9abe4=>{
// 0x10f9abe4 adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x10f9abe8 add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10f9abec mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10f9abf0 bl #0x1128df18
cpu.x[30]=0x10f9abf4;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10f9abf4 ldr w8, [x20, #0x24]
{let address=(cpu.x[20]).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10f9abf8 ldr x21, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
pc=0x10f9abfc;continue;
}
0x10f9abfc=>{
// 0x10f9abfc ldr w9, [x20, #0x20]
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
pc=0x10f9ac00;continue;
}
0x10f9ac00=>{
// 0x10f9ac00 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9ac04 b.ne #0x10f9ac2c
if cpu.condition(1){pc=0x10f9ac2c;continue;}pc=0x10f9ac08;continue;
}
0x10f9ac08=>{
// 0x10f9ac08 lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10f9ac0c mov w10, #5
cpu.x[10]=(((0x5u64))) as u32 as u64;
// 0x10f9ac10 cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9ac14 csel w1, w10, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[10] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10f9ac18 cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10f9ac1c b.ge #0x10f9ac2c
if cpu.condition(10){pc=0x10f9ac2c;continue;}pc=0x10f9ac20;continue;
}
0x10f9ac20=>{
// 0x10f9ac20 add x0, x20, #0x18
{let a=((cpu.x[20]));let b=((0x18u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10f9ac24 bl #0x10f9c37c
cpu.x[30]=0x10f9ac28;super::chunk12::f_10f9c37c(cpu,rt,0x10f9c37c)?;
// 0x10f9ac28 ldr w8, [x20, #0x24]
{let address=(cpu.x[20]).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
pc=0x10f9ac2c;continue;
}
0x10f9ac2c=>{
// 0x10f9ac2c ldr x9, [x20, #0x18]
{let address=(cpu.x[20]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10f9ac30 add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10f9ac34 str w10, [x20, #0x24]
{let address=(cpu.x[20]).wrapping_add(0x24u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10f9ac38 str x21, [x9, w8, sxtw #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10f9ac3c str xzr, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10f9ac40;continue;
}
0x10f9ac40=>{
// 0x10f9ac40 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10f9ac44 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x10f9ac48 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x10f9ac4c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10f9abb8")),}}}
#[inline]
pub(super) unsafe fn f_10fa1d08(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fa1d08=>{
// 0x10fa1d08 ldr w0, [x0, #0x2f8]
{let address=(cpu.x[0]).wrapping_add(0x2f8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[0]=(v0) as u32 as u64;}
// 0x10fa1d0c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fa1d08")),}}}
#[inline]
pub(super) unsafe fn f_10fa3e7c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fa3e7c=>{
// 0x10fa3e7c mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
// 0x10fa3e80 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fa3e7c")),}}}
#[inline]
pub(super) unsafe fn f_10fa4744(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fa4744=>{
// 0x10fa4744 cmp w1, #2
{let a=((cpu.x[1] as u32 as u64));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa4748 b.eq #0x10fa475c
if cpu.condition(0){pc=0x10fa475c;continue;}pc=0x10fa474c;continue;
}
0x10fa474c=>{
// 0x10fa474c cmp w1, #1
{let a=((cpu.x[1] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fa4750 b.ne #0x10fa4764
if cpu.condition(1){pc=0x10fa4764;continue;}pc=0x10fa4754;continue;
}
0x10fa4754=>{
// 0x10fa4754 add x8, x0, #0x3a0
{let a=((cpu.x[0]));let b=((0x3a0u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fa4758 b #0x10fa4760
pc=0x10fa4760;continue;
}
0x10fa475c=>{
// 0x10fa475c add x8, x0, #0x3a8
{let a=((cpu.x[0]));let b=((0x3a8u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
pc=0x10fa4760;continue;
}
0x10fa4760=>{
// 0x10fa4760 str x2, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=cpu.x[2];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10fa4764;continue;
}
0x10fa4764=>{
// 0x10fa4764 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fa4744")),}}}
#[inline]
pub(super) unsafe fn f_10fbbc58(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fbbc58=>{
// 0x10fbbc58 stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fbbc5c str x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fbbc60 stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fbbc64 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fbbc68 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fbbc6c mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10fbbc70 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10fbbc74 bl #0x10f73fd8
cpu.x[30]=0x10fbbc78;super::chunk9::f_10f73fd8(cpu,rt,0x10f73fd8)?;
pc=0x10fbbc78;continue;
}
0x10fbbc78=>{
// 0x10fbbc78 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fbbc7c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fbbc80 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fbbc84 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fbbc88;super::dispatch(cpu,rt,target)?;}
pc=0x10fbbc88;continue;
}
0x10fbbc88=>{
// 0x10fbbc88 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fbbc8c str w0, [x20, #0x68]
{let address=(cpu.x[20]).wrapping_add(0x68u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fbbc90 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fbbc94 str wzr, [x20, #0x7c]
{let address=(cpu.x[20]).wrapping_add(0x7cu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fbbc98 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fbbc9c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fbbca0;super::dispatch(cpu,rt,target)?;}
pc=0x10fbbca0;continue;
}
0x10fbbca0=>{
// 0x10fbbca0 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fbbca4 b.lt #0x10fbbd14
if cpu.condition(11){pc=0x10fbbd14;continue;}pc=0x10fbbca8;continue;
}
0x10fbbca8=>{
// 0x10fbbca8 mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fbbcac mov w22, #5
cpu.x[22]=(((0x5u64))) as u32 as u64;
// 0x10fbbcb0 b #0x10fbbccc
pc=0x10fbbccc;continue;
}
0x10fbbcb4=>{
// 0x10fbbcb4 ldr x9, [x20, #0x70]
{let address=(cpu.x[20]).wrapping_add(0x70u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fbbcb8 add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10fbbcbc subs w21, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[21]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10fbbcc0 str w10, [x20, #0x7c]
{let address=(cpu.x[20]).wrapping_add(0x7cu64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fbbcc4 str w0, [x9, w8, sxtw #2]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fbbcc8 b.eq #0x10fbbd14
if cpu.condition(0){pc=0x10fbbd14;continue;}pc=0x10fbbccc;continue;
}
0x10fbbccc=>{
// 0x10fbbccc ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fbbcd0 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fbbcd4 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fbbcd8 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fbbcdc;super::dispatch(cpu,rt,target)?;}
// 0x10fbbcdc ldp w9, w8, [x20, #0x78]
{let address=(cpu.x[20]).wrapping_add(0x78u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10fbbce0 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fbbce4 b.ne #0x10fbbcb4
if cpu.condition(1){pc=0x10fbbcb4;continue;}pc=0x10fbbce8;continue;
}
0x10fbbce8=>{
// 0x10fbbce8 lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10fbbcec cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fbbcf0 csel w1, w22, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[22] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10fbbcf4 cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fbbcf8 b.ge #0x10fbbcb4
if cpu.condition(10){pc=0x10fbbcb4;continue;}pc=0x10fbbcfc;continue;
}
0x10fbbcfc=>{
// 0x10fbbcfc mov w23, w0
cpu.x[23]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fbbd00 add x0, x20, #0x70
{let a=((cpu.x[20]));let b=((0x70u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fbbd04 bl #0x1078f4d4
cpu.x[30]=0x10fbbd08;super::chunk2::f_1078f4d4(cpu,rt,0x1078f4d4)?;
// 0x10fbbd08 ldr w8, [x20, #0x7c]
{let address=(cpu.x[20]).wrapping_add(0x7cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fbbd0c mov w0, w23
cpu.x[0]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10fbbd10 b #0x10fbbcb4
pc=0x10fbbcb4;continue;
}
0x10fbbd14=>{
// 0x10fbbd14 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fbbd18 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10fbbd1c ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fbbd20 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10fbbd24 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fbbc58")),}}}
#[inline]
pub(super) unsafe fn f_10fbc8b8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fbc8b8=>{
// 0x10fbc8b8 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fbc8bc stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fbc8c0 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fbc8c4 mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10fbc8c8 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10fbc8cc bl #0x10f73f5c
cpu.x[30]=0x10fbc8d0;super::chunk12::f_10f73f5c(cpu,rt,0x10f73f5c)?;
pc=0x10fbc8d0;continue;
}
0x10fbc8d0=>{
// 0x10fbc8d0 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fbc8d4 ldr w1, [x20, #0x60]
{let address=(cpu.x[20]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10fbc8d8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fbc8dc ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fbc8e0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fbc8e4;super::dispatch(cpu,rt,target)?;}
pc=0x10fbc8e4;continue;
}
0x10fbc8e4=>{
// 0x10fbc8e4 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fbc8e8 ldr w1, [x20, #0x64]
{let address=(cpu.x[20]).wrapping_add(0x64u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10fbc8ec mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fbc8f0 ldr x8, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fbc8f4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fbc8f8;super::dispatch(cpu,rt,target)?;}
pc=0x10fbc8f8;continue;
}
0x10fbc8f8=>{
// 0x10fbc8f8 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fbc8fc ldr w1, [x20, #0x68]
{let address=(cpu.x[20]).wrapping_add(0x68u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10fbc900 ldr x2, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10fbc904 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fbc908 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fbc90c ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10fbc910 br x2
{let target=cpu.x[2];if target>=0x10fbc8b8 && target<0x10fbc914{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fbc8b8")),}}}
#[inline]
pub(super) unsafe fn f_10fbfec4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fbfec4=>{
// 0x10fbfec4 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fbfec8 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fbfecc mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fbfed0 mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10fbfed4 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10fbfed8 bl #0x10f73f5c
cpu.x[30]=0x10fbfedc;super::chunk12::f_10f73f5c(cpu,rt,0x10f73f5c)?;
pc=0x10fbfedc;continue;
}
0x10fbfedc=>{
// 0x10fbfedc ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fbfee0 ldr w1, [x20, #0x60]
{let address=(cpu.x[20]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x10fbfee4 ldr x2, [x8, #0x88]
{let address=(cpu.x[8]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10fbfee8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fbfeec ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fbfef0 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10fbfef4 br x2
{let target=cpu.x[2];if target>=0x10fbfec4 && target<0x10fbfef8{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fbfec4")),}}}
#[inline]
pub(super) unsafe fn f_10fc259c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fc259c=>{
// 0x10fc259c stp x29, x30, [sp, #-0x50]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffb0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fc25a0 stp x26, x25, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[26];let v1=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc25a4 stp x24, x23, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc25a8 stp x22, x21, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc25ac stp x20, x19, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc25b0 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fc25b4 ldr w21, [x0, #0x24]
{let address=(cpu.x[0]).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[21]=(v0) as u32 as u64;}
// 0x10fc25b8 cmp w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc25bc b.lt #0x10fc26bc
if cpu.condition(11){pc=0x10fc26bc;continue;}pc=0x10fc25c0;continue;
}
0x10fc25c0=>{
// 0x10fc25c0 lsl x8, x21, #3
cpu.x[8]=(((((cpu.x[21]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10fc25c4 mvn w23, w21
cpu.x[23]=(!(((cpu.x[21] as u32 as u64)))) as u32 as u64;
// 0x10fc25c8 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10fc25cc mov w22, wzr
cpu.x[22]=(((0u64))) as u32 as u64;
// 0x10fc25d0 mov w25, w23
cpu.x[25]=(((cpu.x[23] as u32 as u64))) as u32 as u64;
// 0x10fc25d4 add x24, x8, #8
{let a=((cpu.x[8]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[24]=(result) as u64;}
// 0x10fc25d8 b #0x10fc2604
pc=0x10fc2604;continue;
}
0x10fc25dc=>{
// 0x10fc25dc mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fc25e0 bl #0x10f86100
cpu.x[30]=0x10fc25e4;super::chunk4::f_10f86100(cpu,rt,0x10f86100)?;
// 0x10fc25e4 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x10fc25e8 bl #0x1187d2c0
cpu.x[30]=0x10fc25ec;super::dispatch(cpu,rt,0x1187d2c0)?;
pc=0x10fc25ec;continue;
}
0x10fc25ec=>{
// 0x10fc25ec add w22, w22, #1
{let a=((cpu.x[22] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[22]=(result) as u32 as u64;}
// 0x10fc25f0 cmp x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fc25f4 add w25, w25, #1
{let a=((cpu.x[25] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[25]=(result) as u32 as u64;}
// 0x10fc25f8 sub x24, x24, #8
{let a=((cpu.x[24]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.x[24]=(result) as u64;}
// 0x10fc25fc mov x21, x26
cpu.x[21]=(((cpu.x[26]))) as u64;
// 0x10fc2600 b.le #0x10fc26bc
if cpu.condition(13){pc=0x10fc26bc;continue;}pc=0x10fc2604;continue;
}
0x10fc2604=>{
// 0x10fc2604 ldr x10, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10fc2608 sub x26, x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[26]=(result) as u64;}
// 0x10fc260c add x8, x10, x26, lsl #3
{let a=((cpu.x[10]));let b=(((cpu.x[26])) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fc2610 ldr x20, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[20]=(v0) as u64;}
// 0x10fc2614 ldr x9, [x20, #0x18]
{let address=(cpu.x[20]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fc2618 ldr w9, [x9, #0x1c4]
{let address=(cpu.x[9]).wrapping_add(0x1c4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fc261c cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc2620 b.lt #0x10fc25ec
if cpu.condition(11){pc=0x10fc25ec;continue;}pc=0x10fc2624;continue;
}
0x10fc2624=>{
// 0x10fc2624 ldrsw x13, [x19, #0x24]
{let address=(cpu.x[19]).wrapping_add(0x24u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[13]=(v0) as u64;}
// 0x10fc2628 sub w9, w13, #1
{let a=((cpu.x[13] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fc262c cmp x21, x13
{let a=((cpu.x[21]));let b=((cpu.x[13]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fc2630 str w9, [x19, #0x24]
{let address=(cpu.x[19]).wrapping_add(0x24u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fc2634 b.ge #0x10fc25dc
if cpu.condition(10){pc=0x10fc25dc;continue;}pc=0x10fc2638;continue;
}
0x10fc2638=>{
// 0x10fc2638 sub w9, w13, w21
{let a=((cpu.x[13] as u32 as u64));let b=((cpu.x[21] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fc263c cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc2640 b.lt #0x10fc25dc
if cpu.condition(11){pc=0x10fc25dc;continue;}pc=0x10fc2644;continue;
}
0x10fc2644=>{
// 0x10fc2644 add w11, w22, w23
{let a=((cpu.x[22] as u32 as u64));let b=((cpu.x[23] as u32 as u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u32 as u64;}
// 0x10fc2648 add w11, w13, w11
{let a=((cpu.x[13] as u32 as u64));let b=((cpu.x[11] as u32 as u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u32 as u64;}
// 0x10fc264c cmp w11, #3
{let a=((cpu.x[11] as u32 as u64));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc2650 b.lo #0x10fc2694
if cpu.condition(3){pc=0x10fc2694;continue;}pc=0x10fc2654;continue;
}
0x10fc2654=>{
// 0x10fc2654 add x11, x11, #1
{let a=((cpu.x[11]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
// 0x10fc2658 add w13, w13, w25
{let a=((cpu.x[13] as u32 as u64));let b=((cpu.x[25] as u32 as u64));let result=a.wrapping_add(b);cpu.x[13]=(result) as u32 as u64;}
// 0x10fc265c add x10, x10, x24
{let a=((cpu.x[10]));let b=((cpu.x[24]));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x10fc2660 and x12, x11, #0x1fffffffc
{let a=((cpu.x[11]));let b=((0x1fffffffcu64));let result=a & b;cpu.x[12]=(result) as u64;}
// 0x10fc2664 add x13, x13, #1
{let a=((cpu.x[13]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[13]=(result) as u64;}
// 0x10fc2668 add x8, x8, x12, lsl #3
{let a=((cpu.x[8]));let b=(((cpu.x[12])) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fc266c sub w9, w9, w12
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[12] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fc2670 and x13, x13, #0x1fffffffc
{let a=((cpu.x[13]));let b=((0x1fffffffcu64));let result=a & b;cpu.x[13]=(result) as u64;}
pc=0x10fc2674;continue;
}
0x10fc2674=>{
// 0x10fc2674 ldur q0, [x10, #-8]
{let address=(cpu.x[10]).wrapping_add(0xfffffffffffffff8u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));cpu.q[0]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x10fc2678 ldur q1, [x10, #8]
{let address=(cpu.x[10]).wrapping_add(0x8u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));cpu.q[1]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x10fc267c subs x13, x13, #4
{let a=((cpu.x[13]));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.x[13]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x10fc2680 stp q0, q1, [x10, #-0x10]
{let address=(cpu.x[10]).wrapping_add(0xfffffffffffffff0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[1] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x10fc2684 add x10, x10, #0x20
{let a=((cpu.x[10]));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x10fc2688 b.ne #0x10fc2674
if cpu.condition(1){pc=0x10fc2674;continue;}pc=0x10fc268c;continue;
}
0x10fc268c=>{
// 0x10fc268c cmp x11, x12
{let a=((cpu.x[11]));let b=((cpu.x[12]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fc2690 b.eq #0x10fc26b4
if cpu.condition(0){pc=0x10fc26b4;continue;}pc=0x10fc2694;continue;
}
0x10fc2694=>{
// 0x10fc2694 add w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fc2698 add x8, x8, #8
{let a=((cpu.x[8]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
pc=0x10fc269c;continue;
}
0x10fc269c=>{
// 0x10fc269c ldr x10, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10fc26a0 sub w9, w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fc26a4 cmp w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc26a8 stur x10, [x8, #-8]
{let address=(cpu.x[8]).wrapping_add(0xfffffffffffffff8u64);let v0=cpu.x[10];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fc26ac add x8, x8, #8
{let a=((cpu.x[8]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fc26b0 b.hi #0x10fc269c
if cpu.condition(8){pc=0x10fc269c;continue;}pc=0x10fc26b4;continue;
}
0x10fc26b4=>{
// 0x10fc26b4 cbnz x20, #0x10fc25dc
if (cpu.x[20]) != 0 {pc=0x10fc25dc;continue;}pc=0x10fc26b8;continue;
}
0x10fc26b8=>{
// 0x10fc26b8 b #0x10fc25ec
pc=0x10fc25ec;continue;
}
0x10fc26bc=>{
// 0x10fc26bc ldp x20, x19, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fc26c0 ldp x22, x21, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fc26c4 ldp x24, x23, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fc26c8 ldp x26, x25, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fc26cc ldp x29, x30, [sp], #0x50
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x50u64)))) as u64;}
// 0x10fc26d0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fc259c")),}}}
#[inline]
pub(super) unsafe fn f_10fc26d4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fc26d4=>{
// 0x10fc26d4 stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fc26d8 stp x24, x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc26dc stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc26e0 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc26e4 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fc26e8 ldr w8, [x0, #0x24]
{let address=(cpu.x[0]).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc26ec mov w19, w1
cpu.x[19]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x10fc26f0 mov w21, #0x64
cpu.x[21]=(((0x64u64))) as u32 as u64;
// 0x10fc26f4 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc26f8 b.lt #0x10fc2760
if cpu.condition(11){pc=0x10fc2760;continue;}pc=0x10fc26fc;continue;
}
0x10fc26fc=>{
// 0x10fc26fc mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10fc2700 mov x24, xzr
cpu.x[24]=(((0u64))) as u64;
// 0x10fc2704 mov w23, wzr
cpu.x[23]=(((0u64))) as u32 as u64;
// 0x10fc2708 mov w22, #0x64
cpu.x[22]=(((0x64u64))) as u32 as u64;
// 0x10fc270c b #0x10fc2728
pc=0x10fc2728;continue;
}
0x10fc2710=>{
// 0x10fc2710 cmp w0, w22
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[22] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc2714 csel w22, w0, w22, gt
cpu.x[22]=(if cpu.condition(12){((cpu.x[0] as u32 as u64))}else{((cpu.x[22] as u32 as u64))}) as u32 as u64;
pc=0x10fc2718;continue;
}
0x10fc2718=>{
// 0x10fc2718 ldrsw x8, [x20, #0x24]
{let address=(cpu.x[20]).wrapping_add(0x24u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x10fc271c add x24, x24, #1
{let a=((cpu.x[24]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[24]=(result) as u64;}
// 0x10fc2720 cmp x24, x8
{let a=((cpu.x[24]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fc2724 b.ge #0x10fc2754
if cpu.condition(10){pc=0x10fc2754;continue;}pc=0x10fc2728;continue;
}
0x10fc2728=>{
// 0x10fc2728 ldr x8, [x20, #0x18]
{let address=(cpu.x[20]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc272c ldr x8, [x8, x24, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[24])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc2730 ldr x0, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fc2734 bl #0x10ddab44
cpu.x[30]=0x10fc2738;super::chunk2::f_10ddab44(cpu,rt,0x10ddab44)?;
pc=0x10fc2738;continue;
}
0x10fc2738=>{
// 0x10fc2738 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc273c b.ge #0x10fc2710
if cpu.condition(10){pc=0x10fc2710;continue;}pc=0x10fc2740;continue;
}
0x10fc2740=>{
// 0x10fc2740 tbz w0, #0x1f, #0x10fc2718
if ((cpu.x[0] as u32 as u64) & (1u64 << 31)) == 0 {pc=0x10fc2718;continue;}pc=0x10fc2744;continue;
}
0x10fc2744=>{
// 0x10fc2744 neg w8, w0
cpu.x[8]=(0u64.wrapping_sub(((cpu.x[0] as u32 as u64)))) as u32 as u64;
// 0x10fc2748 cmp w23, w8
{let a=((cpu.x[23] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc274c csneg w23, w23, w0, gt
cpu.x[23]=(if cpu.condition(12){((cpu.x[23] as u32 as u64))}else{0u64.wrapping_sub(((cpu.x[0] as u32 as u64)))}) as u32 as u64;
// 0x10fc2750 b #0x10fc2718
pc=0x10fc2718;continue;
}
0x10fc2754=>{
// 0x10fc2754 mov w8, #0x64
cpu.x[8]=(((0x64u64))) as u32 as u64;
// 0x10fc2758 sub w8, w8, w23
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[23] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fc275c b #0x10fc2768
pc=0x10fc2768;continue;
}
0x10fc2760=>{
// 0x10fc2760 mov w8, #0x64
cpu.x[8]=(((0x64u64))) as u32 as u64;
// 0x10fc2764 mov w22, #0x64
cpu.x[22]=(((0x64u64))) as u32 as u64;
pc=0x10fc2768;continue;
}
0x10fc2768=>{
// 0x10fc2768 mul w9, w22, w19
cpu.x[9]=((((cpu.x[22] as u32 as u64))).wrapping_mul(((cpu.x[19] as u32 as u64)))) as u32 as u64;
// 0x10fc276c mov w10, #0x851f
cpu.x[10]=(((0x851fu64))) as u32 as u64;
// 0x10fc2770 cmp w8, #0x64
{let a=((cpu.x[8] as u32 as u64));let b=((0x64u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc2774 movk w10, #0x51eb, lsl #16
cpu.x[10]=((cpu.x[10] as u32 as u64 & !(0xffff0000u64)) | 0x51eb0000u64) as u32 as u64;
// 0x10fc2778 csel w8, w8, w21, lt
cpu.x[8]=(if cpu.condition(11){((cpu.x[8] as u32 as u64))}else{((cpu.x[21] as u32 as u64))}) as u32 as u64;
// 0x10fc277c bic w8, w8, w8, asr #31
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[8] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fc2780 smull x9, w9, w10
cpu.x[9]=(((((cpu.x[9] as u32 as u64))) as i32 as i64).wrapping_mul((((cpu.x[10] as u32 as u64))) as i32 as i64) as u64) as u64;
// 0x10fc2784 lsr x11, x9, #0x3f
cpu.x[11]=(((((cpu.x[9]))) as u64).wrapping_shr(((((0x3fu64))) & 63) as u32) as u64) as u64;
// 0x10fc2788 asr x9, x9, #0x25
cpu.x[9]=(((((cpu.x[9]))) as i64).wrapping_shr(((((0x25u64))) & 63) as u32) as u64) as u64;
// 0x10fc278c add w9, w9, w11
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[11] as u32 as u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fc2790 mul w8, w9, w8
cpu.x[8]=((((cpu.x[9] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64)))) as u32 as u64;
// 0x10fc2794 smull x8, w8, w10
cpu.x[8]=(((((cpu.x[8] as u32 as u64))) as i32 as i64).wrapping_mul((((cpu.x[10] as u32 as u64))) as i32 as i64) as u64) as u64;
// 0x10fc2798 lsr x9, x8, #0x3f
cpu.x[9]=(((((cpu.x[8]))) as u64).wrapping_shr(((((0x3fu64))) & 63) as u32) as u64) as u64;
// 0x10fc279c asr x8, x8, #0x25
cpu.x[8]=(((((cpu.x[8]))) as i64).wrapping_shr(((((0x25u64))) & 63) as u32) as u64) as u64;
// 0x10fc27a0 add w0, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u32 as u64;}
// 0x10fc27a4 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fc27a8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fc27ac ldp x24, x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fc27b0 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10fc27b4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fc26d4")),}}}
#[inline]
pub(super) unsafe fn f_10fc27b8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fc27b8=>{
// 0x10fc27b8 stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fc27bc stp x24, x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc27c0 stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc27c4 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc27c8 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fc27cc ldr w8, [x0, #0x24]
{let address=(cpu.x[0]).wrapping_add(0x24u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fc27d0 mov w19, w1
cpu.x[19]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x10fc27d4 mov w21, #0x64
cpu.x[21]=(((0x64u64))) as u32 as u64;
// 0x10fc27d8 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc27dc b.lt #0x10fc2844
if cpu.condition(11){pc=0x10fc2844;continue;}pc=0x10fc27e0;continue;
}
0x10fc27e0=>{
// 0x10fc27e0 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10fc27e4 mov x24, xzr
cpu.x[24]=(((0u64))) as u64;
// 0x10fc27e8 mov w23, wzr
cpu.x[23]=(((0u64))) as u32 as u64;
// 0x10fc27ec mov w22, #0x64
cpu.x[22]=(((0x64u64))) as u32 as u64;
// 0x10fc27f0 b #0x10fc280c
pc=0x10fc280c;continue;
}
0x10fc27f4=>{
// 0x10fc27f4 cmp w0, w22
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[22] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc27f8 csel w22, w0, w22, gt
cpu.x[22]=(if cpu.condition(12){((cpu.x[0] as u32 as u64))}else{((cpu.x[22] as u32 as u64))}) as u32 as u64;
pc=0x10fc27fc;continue;
}
0x10fc27fc=>{
// 0x10fc27fc ldrsw x8, [x20, #0x24]
{let address=(cpu.x[20]).wrapping_add(0x24u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
pc=0x10fc2800;continue;
}
0x10fc2800=>{
// 0x10fc2800 add x24, x24, #1
{let a=((cpu.x[24]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[24]=(result) as u64;}
// 0x10fc2804 cmp x24, x8
{let a=((cpu.x[24]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fc2808 b.ge #0x10fc2838
if cpu.condition(10){pc=0x10fc2838;continue;}pc=0x10fc280c;continue;
}
0x10fc280c=>{
// 0x10fc280c ldr x8, [x20, #0x18]
{let address=(cpu.x[20]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc2810 ldr x8, [x8, x24, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[24])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fc2814 ldr x0, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fc2818 bl #0x10ddab3c
cpu.x[30]=0x10fc281c;super::chunk0::f_10ddab3c(cpu,rt,0x10ddab3c)?;
pc=0x10fc281c;continue;
}
0x10fc281c=>{
// 0x10fc281c cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc2820 b.ge #0x10fc27f4
if cpu.condition(10){pc=0x10fc27f4;continue;}pc=0x10fc2824;continue;
}
0x10fc2824=>{
// 0x10fc2824 tbz w0, #0x1f, #0x10fc27fc
if ((cpu.x[0] as u32 as u64) & (1u64 << 31)) == 0 {pc=0x10fc27fc;continue;}pc=0x10fc2828;continue;
}
0x10fc2828=>{
// 0x10fc2828 neg w8, w0
cpu.x[8]=(0u64.wrapping_sub(((cpu.x[0] as u32 as u64)))) as u32 as u64;
// 0x10fc282c cmp w23, w8
{let a=((cpu.x[23] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc2830 csneg w23, w23, w0, gt
cpu.x[23]=(if cpu.condition(12){((cpu.x[23] as u32 as u64))}else{0u64.wrapping_sub(((cpu.x[0] as u32 as u64)))}) as u32 as u64;
// 0x10fc2834 b #0x10fc27fc
pc=0x10fc27fc;continue;
}
0x10fc2838=>{
// 0x10fc2838 mov w8, #0x64
cpu.x[8]=(((0x64u64))) as u32 as u64;
// 0x10fc283c sub w8, w8, w23
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[23] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fc2840 b #0x10fc284c
pc=0x10fc284c;continue;
}
0x10fc2844=>{
// 0x10fc2844 mov w8, #0x64
cpu.x[8]=(((0x64u64))) as u32 as u64;
// 0x10fc2848 mov w22, #0x64
cpu.x[22]=(((0x64u64))) as u32 as u64;
pc=0x10fc284c;continue;
}
0x10fc284c=>{
// 0x10fc284c mul w9, w22, w19
cpu.x[9]=((((cpu.x[22] as u32 as u64))).wrapping_mul(((cpu.x[19] as u32 as u64)))) as u32 as u64;
// 0x10fc2850 mov w10, #0x851f
cpu.x[10]=(((0x851fu64))) as u32 as u64;
// 0x10fc2854 cmp w8, #0x64
{let a=((cpu.x[8] as u32 as u64));let b=((0x64u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fc2858 movk w10, #0x51eb, lsl #16
cpu.x[10]=((cpu.x[10] as u32 as u64 & !(0xffff0000u64)) | 0x51eb0000u64) as u32 as u64;
// 0x10fc285c csel w8, w8, w21, lt
cpu.x[8]=(if cpu.condition(11){((cpu.x[8] as u32 as u64))}else{((cpu.x[21] as u32 as u64))}) as u32 as u64;
// 0x10fc2860 bic w8, w8, w8, asr #31
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[8] as u32 as u64)) as i32).wrapping_shr(31) as u64;let result=a & !b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fc2864 smull x9, w9, w10
cpu.x[9]=(((((cpu.x[9] as u32 as u64))) as i32 as i64).wrapping_mul((((cpu.x[10] as u32 as u64))) as i32 as i64) as u64) as u64;
// 0x10fc2868 lsr x11, x9, #0x3f
cpu.x[11]=(((((cpu.x[9]))) as u64).wrapping_shr(((((0x3fu64))) & 63) as u32) as u64) as u64;
// 0x10fc286c asr x9, x9, #0x25
cpu.x[9]=(((((cpu.x[9]))) as i64).wrapping_shr(((((0x25u64))) & 63) as u32) as u64) as u64;
// 0x10fc2870 add w9, w9, w11
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[11] as u32 as u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x10fc2874 mul w8, w9, w8
cpu.x[8]=((((cpu.x[9] as u32 as u64))).wrapping_mul(((cpu.x[8] as u32 as u64)))) as u32 as u64;
// 0x10fc2878 smull x8, w8, w10
cpu.x[8]=(((((cpu.x[8] as u32 as u64))) as i32 as i64).wrapping_mul((((cpu.x[10] as u32 as u64))) as i32 as i64) as u64) as u64;
// 0x10fc287c lsr x9, x8, #0x3f
cpu.x[9]=(((((cpu.x[8]))) as u64).wrapping_shr(((((0x3fu64))) & 63) as u32) as u64) as u64;
// 0x10fc2880 asr x8, x8, #0x25
cpu.x[8]=(((((cpu.x[8]))) as i64).wrapping_shr(((((0x25u64))) & 63) as u32) as u64) as u64;
// 0x10fc2884 add w0, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u32 as u64;}
// 0x10fc2888 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fc288c ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fc2890 ldp x24, x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fc2894 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x10fc2898 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fc27b8")),}}}
#[inline]
pub(super) unsafe fn f_10fc2e60(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fc2e60=>{
// 0x10fc2e60 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fc2e64 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc2e68 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fc2e6c mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10fc2e70 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10fc2e74 adrp x8, #0x1193b000
cpu.x[8]=(0x1193b000u64) as u64;
// 0x10fc2e78 add x8, x8, #0x4f8
{let a=((cpu.x[8]));let b=((0x4f8u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fc2e7c ldr x0, [x20, #0x50]!
{let address=(cpu.x[20]).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;cpu.x[20]=(address) as u64;}
// 0x10fc2e80 str x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fc2e84 cbz x0, #0x10fc2e8c
if (cpu.x[0]) == 0 {pc=0x10fc2e8c;continue;}pc=0x10fc2e88;continue;
}
0x10fc2e88=>{
// 0x10fc2e88 bl #0x1187d360
cpu.x[30]=0x10fc2e8c;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x10fc2e8c;continue;
}
0x10fc2e8c=>{
// 0x10fc2e8c ldr x0, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fc2e90 stp xzr, xzr, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=0u64;let v1=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fc2e94 cbz x0, #0x10fc2e9c
if (cpu.x[0]) == 0 {pc=0x10fc2e9c;continue;}pc=0x10fc2e98;continue;
}
0x10fc2e98=>{
// 0x10fc2e98 bl #0x1187d360
cpu.x[30]=0x10fc2e9c;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x10fc2e9c;continue;
}
0x10fc2e9c=>{
// 0x10fc2e9c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fc2ea0 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fc2ea4 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10fc2ea8 b #0x1187d2c0
super::dispatch(cpu,rt,0x1187d2c0)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fc2e60")),}}}
#[inline]
pub(super) unsafe fn f_10fca4d8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fca4d8=>{
// 0x10fca4d8 sub sp, sp, #0x70
{let a=((cpu.sp));let b=((0x70u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x10fca4dc stp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fca4e0 stp x28, x27, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[28];let v1=cpu.x[27];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fca4e4 stp x26, x25, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[26];let v1=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fca4e8 stp x24, x23, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fca4ec stp x22, x21, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fca4f0 stp x20, x19, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fca4f4 add x29, sp, #0x10
{let a=((cpu.sp));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x10fca4f8 ldrb w8, [x29, #0x68]
{let address=(cpu.x[29]).wrapping_add(0x68u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fca4fc ldr x26, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[26]=(v0) as u64;}
// 0x10fca500 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10fca504 ldrb w28, [x29, #0x70]
{let address=(cpu.x[29]).wrapping_add(0x70u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[28]=(v0) as u32 as u64;}
// 0x10fca508 mov x20, x7
cpu.x[20]=(((cpu.x[7]))) as u64;
// 0x10fca50c mov x21, x6
cpu.x[21]=(((cpu.x[6]))) as u64;
// 0x10fca510 str w8, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fca514 ldrb w8, [x29, #0x60]
{let address=(cpu.x[29]).wrapping_add(0x60u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fca518 mov x0, x26
cpu.x[0]=(((cpu.x[26]))) as u64;
// 0x10fca51c mov w22, w5
cpu.x[22]=(((cpu.x[5] as u32 as u64))) as u32 as u64;
// 0x10fca520 mov x23, x4
cpu.x[23]=(((cpu.x[4]))) as u64;
// 0x10fca524 mov w24, w3
cpu.x[24]=(((cpu.x[3] as u32 as u64))) as u32 as u64;
// 0x10fca528 mov w25, w2
cpu.x[25]=(((cpu.x[2] as u32 as u64))) as u32 as u64;
// 0x10fca52c stur w8, [x29, #-4]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffffcu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fca530 mov w27, w1
cpu.x[27]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x10fca534 bl #0x10f93f94
cpu.x[30]=0x10fca538;super::chunk3::f_10f93f94(cpu,rt,0x10f93f94)?;
pc=0x10fca538;continue;
}
0x10fca538=>{
// 0x10fca538 bl #0x10f9922c
cpu.x[30]=0x10fca53c;super::chunk0::f_10f9922c(cpu,rt,0x10f9922c)?;
pc=0x10fca53c;continue;
}
0x10fca53c=>{
// 0x10fca53c mov x1, x26
cpu.x[1]=(((cpu.x[26]))) as u64;
// 0x10fca540 mov w2, w27
cpu.x[2]=(((cpu.x[27] as u32 as u64))) as u32 as u64;
// 0x10fca544 mov x3, xzr
cpu.x[3]=(((0u64))) as u64;
// 0x10fca548 bl #0x10fd0738
cpu.x[30]=0x10fca54c;super::chunk14::f_10fd0738(cpu,rt,0x10fd0738)?;
pc=0x10fca54c;continue;
}
0x10fca54c=>{
// 0x10fca54c mov x1, x23
cpu.x[1]=(((cpu.x[23]))) as u64;
// 0x10fca550 mov x26, x0
cpu.x[26]=(((cpu.x[0]))) as u64;
// 0x10fca554 stp w25, w24, [x0, #4]
{let address=(cpu.x[0]).wrapping_add(0x4u64);let v0=cpu.x[25] as u32 as u64;let v1=cpu.x[24] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x10fca558 bl #0x10fd11f4
cpu.x[30]=0x10fca55c;super::chunk9::f_10fd11f4(cpu,rt,0x10fd11f4)?;
pc=0x10fca55c;continue;
}
0x10fca55c=>{
// 0x10fca55c str w22, [x26, #0xc]
{let address=(cpu.x[26]).wrapping_add(0xcu64);let v0=cpu.x[22] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fca560 str x21, [x26, #0x18]
{let address=(cpu.x[26]).wrapping_add(0x18u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fca564 cbz x20, #0x10fca570
if (cpu.x[20]) == 0 {pc=0x10fca570;continue;}pc=0x10fca568;continue;
}
0x10fca568=>{
// 0x10fca568 ldr d0, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.q[0]=(v0) as u128 & 0xffffffffffffffffu128;}
// 0x10fca56c str d0, [x26, #0x10]
{let address=(cpu.x[26]).wrapping_add(0x10u64);let v0=(cpu.q[0] & 0xffffffffffffffffu128) as u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10fca570;continue;
}
0x10fca570=>{
// 0x10fca570 ldr w9, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fca574 tst w28, #1
{let a=((cpu.x[28] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.logical_flags(result,32);}
// 0x10fca578 mov w8, #2
cpu.x[8]=(((0x2u64))) as u32 as u64;
// 0x10fca57c ldur w10, [x29, #-4]
{let address=(cpu.x[29]).wrapping_add(0xfffffffffffffffcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x10fca580 csel w8, w8, wzr, ne
cpu.x[8]=(if cpu.condition(1){((cpu.x[8] as u32 as u64))}else{((0u64))}) as u32 as u64;
// 0x10fca584 ldr w11, [x26, #0x20]
{let address=(cpu.x[26]).wrapping_add(0x20u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
// 0x10fca588 tst w9, #1
{let a=((cpu.x[9] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.logical_flags(result,32);}
// 0x10fca58c mov w9, #0x80
cpu.x[9]=(((0x80u64))) as u32 as u64;
// 0x10fca590 mov w12, #-0x8b
cpu.x[12]=(((0xffffffffffffff75u64))) as u32 as u64;
// 0x10fca594 csel w9, wzr, w9, ne
cpu.x[9]=(if cpu.condition(1){((0u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10fca598 tst w10, #1
{let a=((cpu.x[10] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.logical_flags(result,32);}
// 0x10fca59c mov w10, #8
cpu.x[10]=(((0x8u64))) as u32 as u64;
// 0x10fca5a0 csel w10, w10, wzr, ne
cpu.x[10]=(if cpu.condition(1){((cpu.x[10] as u32 as u64))}else{((0u64))}) as u32 as u64;
// 0x10fca5a4 mov x0, x26
cpu.x[0]=(((cpu.x[26]))) as u64;
// 0x10fca5a8 orr w9, w9, w10
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a | b;cpu.x[9]=(result) as u32 as u64;}
// 0x10fca5ac and w10, w11, w12
{let a=((cpu.x[11] as u32 as u64));let b=((cpu.x[12] as u32 as u64));let result=a & b;cpu.x[10]=(result) as u32 as u64;}
// 0x10fca5b0 orr w8, w9, w8
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fca5b4 orr w8, w8, w10
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x10fca5b8 str w8, [x26, #0x20]
{let address=(cpu.x[26]).wrapping_add(0x20u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fca5bc bl #0x10fd10f0
cpu.x[30]=0x10fca5c0;super::chunk7::f_10fd10f0(cpu,rt,0x10fd10f0)?;
pc=0x10fca5c0;continue;
}
0x10fca5c0=>{
// 0x10fca5c0 ldr w8, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fca5c4 add w8, w8, w0
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fca5c8 str w8, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fca5cc ldp x20, x19, [sp, #0x60]
{let address=(cpu.sp).wrapping_add(0x60u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fca5d0 ldp x22, x21, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fca5d4 ldp x24, x23, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fca5d8 ldp x26, x25, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fca5dc ldp x28, x27, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fca5e0 ldp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x10fca5e4 add sp, sp, #0x70
{let a=((cpu.sp));let b=((0x70u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x10fca5e8 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fca4d8")),}}}
#[inline]
pub(super) unsafe fn f_10fcaeec(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fcaeec=>{
// 0x10fcaeec stp x29, x30, [sp, #-0x60]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffa0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fcaef0 stp x28, x27, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[28];let v1=cpu.x[27];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fcaef4 stp x26, x25, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[26];let v1=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fcaef8 stp x24, x23, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fcaefc stp x22, x21, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fcaf00 stp x20, x19, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fcaf04 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fcaf08 ldr x23, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10fcaf0c mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10fcaf10 mov x22, x1
cpu.x[22]=(((cpu.x[1]))) as u64;
// 0x10fcaf14 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fcaf18 bl #0x10f93d2c
cpu.x[30]=0x10fcaf1c;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
pc=0x10fcaf1c;continue;
}
0x10fcaf1c=>{
// 0x10fcaf1c ldr w26, [x22]
{let address=(cpu.x[22]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[26]=(v0) as u32 as u64;}
// 0x10fcaf20 ldr x25, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[25]=(v0) as u64;}
// 0x10fcaf24 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x10fcaf28 ldr w27, [x22, #8]
{let address=(cpu.x[22]).wrapping_add(0x8u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[27]=(v0) as u32 as u64;}
// 0x10fcaf2c bl #0x10fd1174
cpu.x[30]=0x10fcaf30;super::chunk3::f_10fd1174(cpu,rt,0x10fd1174)?;
pc=0x10fcaf30;continue;
}
0x10fcaf30=>{
// 0x10fcaf30 ldr w28, [x22, #0x20]
{let address=(cpu.x[22]).wrapping_add(0x20u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[28]=(v0) as u32 as u64;}
// 0x10fcaf34 ldr w20, [x22, #0xc]
{let address=(cpu.x[22]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[20]=(v0) as u32 as u64;}
// 0x10fcaf38 mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x10fcaf3c tbnz w28, #3, #0x10fcaf68
if ((cpu.x[28] as u32 as u64) & (1u64 << 3)) != 0 {pc=0x10fcaf68;continue;}pc=0x10fcaf40;continue;
}
0x10fcaf40=>{
// 0x10fcaf40 and w1, w28, #1
{let a=((cpu.x[28] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[1]=(result) as u32 as u64;}
// 0x10fcaf44 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fcaf48 bl #0x10fca2b4
cpu.x[30]=0x10fcaf4c;super::chunk5::f_10fca2b4(cpu,rt,0x10fca2b4)?;
pc=0x10fcaf4c;continue;
}
0x10fcaf4c=>{
// 0x10fcaf4c tbnz w0, #0, #0x10fcb094
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x10fcb094;continue;}pc=0x10fcaf50;continue;
}
0x10fcaf50=>{
// 0x10fcaf50 adrp x8, #0x11a66000
cpu.x[8]=(0x11a66000u64) as u64;
// 0x10fcaf54 add x8, x8, #0x90
{let a=((cpu.x[8]));let b=((0x90u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fcaf58 ldr x9, [x23, #0xb0]
{let address=(cpu.x[23]).wrapping_add(0xb0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fcaf5c ldr x8, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcaf60 tst x9, x8
{let a=((cpu.x[9]));let b=((cpu.x[8]));let result=a & b;cpu.logical_flags(result,64);}
// 0x10fcaf64 b.ne #0x10fcb094
if cpu.condition(1){pc=0x10fcb094;continue;}pc=0x10fcaf68;continue;
}
0x10fcaf68=>{
// 0x10fcaf68 tbnz w26, #0x1f, #0x10fcb09c
if ((cpu.x[26] as u32 as u64) & (1u64 << 31)) != 0 {pc=0x10fcb09c;continue;}pc=0x10fcaf6c;continue;
}
0x10fcaf6c=>{
// 0x10fcaf6c cbz w26, #0x10fcb094
if (cpu.x[26] as u32 as u64) == 0 {pc=0x10fcb094;continue;}pc=0x10fcaf70;continue;
}
0x10fcaf70=>{
// 0x10fcaf70 cbz x21, #0x10fcaf84
if (cpu.x[21]) == 0 {pc=0x10fcaf84;continue;}pc=0x10fcaf74;continue;
}
0x10fcaf74=>{
// 0x10fcaf74 ldr x0, [x21, #0x58]
{let address=(cpu.x[21]).wrapping_add(0x58u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fcaf78 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcaf7c ldr x8, [x8, #0x18]
{let address=(cpu.x[8]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcaf80 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fcaf84;super::dispatch(cpu,rt,target)?;}
pc=0x10fcaf84;continue;
}
0x10fcaf84=>{
// 0x10fcaf84 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fcaf88 bl #0x10f940fc
cpu.x[30]=0x10fcaf8c;super::chunk9::f_10f940fc(cpu,rt,0x10f940fc)?;
pc=0x10fcaf8c;continue;
}
0x10fcaf8c=>{
// 0x10fcaf8c cbz x0, #0x10fcb04c
if (cpu.x[0]) == 0 {pc=0x10fcb04c;continue;}pc=0x10fcaf90;continue;
}
0x10fcaf90=>{
// 0x10fcaf90 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fcaf94 bl #0x10f940fc
cpu.x[30]=0x10fcaf98;super::chunk9::f_10f940fc(cpu,rt,0x10f940fc)?;
pc=0x10fcaf98;continue;
}
0x10fcaf98=>{
// 0x10fcaf98 mov w1, w26
cpu.x[1]=(((cpu.x[26] as u32 as u64))) as u32 as u64;
// 0x10fcaf9c mov x2, xzr
cpu.x[2]=(((0u64))) as u64;
// 0x10fcafa0 bl #0x10fc1e84
cpu.x[30]=0x10fcafa4;super::chunk4::f_10fc1e84(cpu,rt,0x10fc1e84)?;
pc=0x10fcafa4;continue;
}
0x10fcafa4=>{
// 0x10fcafa4 mov w24, w0
cpu.x[24]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fcafa8 tbz w0, #0x1f, #0x10fcafbc
if ((cpu.x[0] as u32 as u64) & (1u64 << 31)) == 0 {pc=0x10fcafbc;continue;}pc=0x10fcafac;continue;
}
0x10fcafac=>{
// 0x10fcafac adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x10fcafb0 add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fcafb4 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10fcafb8 bl #0x1128df18
cpu.x[30]=0x10fcafbc;super::dispatch(cpu,rt,0x1128df18)?;
pc=0x10fcafbc;continue;
}
0x10fcafbc=>{
// 0x10fcafbc mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fcafc0 bl #0x10f940fc
cpu.x[30]=0x10fcafc4;super::chunk9::f_10f940fc(cpu,rt,0x10f940fc)?;
pc=0x10fcafc4;continue;
}
0x10fcafc4=>{
// 0x10fcafc4 mov x1, xzr
cpu.x[1]=(((0u64))) as u64;
// 0x10fcafc8 bl #0x10fc1fc8
cpu.x[30]=0x10fcafcc;super::chunk14::f_10fc1fc8(cpu,rt,0x10fc1fc8)?;
pc=0x10fcafcc;continue;
}
0x10fcafcc=>{
// 0x10fcafcc cmp w24, #1
{let a=((cpu.x[24] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcafd0 csinc w26, w24, wzr, gt
cpu.x[26]=(if cpu.condition(12){((cpu.x[24] as u32 as u64))}else{(((0u64))).wrapping_add(1)}) as u32 as u64;
// 0x10fcafd4 cbz x21, #0x10fcb04c
if (cpu.x[21]) == 0 {pc=0x10fcb04c;continue;}pc=0x10fcafd8;continue;
}
0x10fcafd8=>{
// 0x10fcafd8 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fcafdc bl #0x10f940fc
cpu.x[30]=0x10fcafe0;super::chunk9::f_10f940fc(cpu,rt,0x10f940fc)?;
pc=0x10fcafe0;continue;
}
0x10fcafe0=>{
// 0x10fcafe0 cbz x0, #0x10fcb04c
if (cpu.x[0]) == 0 {pc=0x10fcb04c;continue;}pc=0x10fcafe4;continue;
}
0x10fcafe4=>{
// 0x10fcafe4 ldr w8, [x0, #0x38]
{let address=(cpu.x[0]).wrapping_add(0x38u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcafe8 mov w9, #0x851f
cpu.x[9]=(((0x851fu64))) as u32 as u64;
// 0x10fcafec ldr x10, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10fcaff0 movk w9, #0x51eb, lsl #16
cpu.x[9]=((cpu.x[9] as u32 as u64 & !(0xffff0000u64)) | 0x51eb0000u64) as u32 as u64;
// 0x10fcaff4 mov x24, x0
cpu.x[24]=(((cpu.x[0]))) as u64;
// 0x10fcaff8 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fcaffc mul w8, w8, w26
cpu.x[8]=((((cpu.x[8] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
pc=0x10fcb000;continue;
}
0x10fcb000=>{
// 0x10fcb000 smull x8, w8, w9
cpu.x[8]=(((((cpu.x[8] as u32 as u64))) as i32 as i64).wrapping_mul((((cpu.x[9] as u32 as u64))) as i32 as i64) as u64) as u64;
// 0x10fcb004 lsr x9, x8, #0x3f
cpu.x[9]=(((((cpu.x[8]))) as u64).wrapping_shr(((((0x3fu64))) & 63) as u32) as u64) as u64;
// 0x10fcb008 asr x8, x8, #0x25
cpu.x[8]=(((((cpu.x[8]))) as i64).wrapping_shr(((((0x25u64))) & 63) as u32) as u64) as u64;
// 0x10fcb00c add w8, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fcb010 ldr x9, [x10, #0x178]
{let address=(cpu.x[10]).wrapping_add(0x178u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fcb014 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcb018 csinc w26, w8, wzr, gt
cpu.x[26]=(if cpu.condition(12){((cpu.x[8] as u32 as u64))}else{(((0u64))).wrapping_add(1)}) as u32 as u64;
// 0x10fcb01c blr x9
{let target=cpu.x[9];cpu.x[30]=0x10fcb020;super::dispatch(cpu,rt,target)?;}
pc=0x10fcb020;continue;
}
0x10fcb020=>{
// 0x10fcb020 tbz w0, #0, #0x10fcb04c
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fcb04c;continue;}pc=0x10fcb024;continue;
}
0x10fcb024=>{
// 0x10fcb024 ldr w8, [x24, #0x3c]
{let address=(cpu.x[24]).wrapping_add(0x3cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcb028 mov w9, #0x851f
cpu.x[9]=(((0x851fu64))) as u32 as u64;
// 0x10fcb02c movk w9, #0x51eb, lsl #16
cpu.x[9]=((cpu.x[9] as u32 as u64 & !(0xffff0000u64)) | 0x51eb0000u64) as u32 as u64;
// 0x10fcb030 mul w8, w8, w26
cpu.x[8]=((((cpu.x[8] as u32 as u64))).wrapping_mul(((cpu.x[26] as u32 as u64)))) as u32 as u64;
// 0x10fcb034 smull x8, w8, w9
cpu.x[8]=(((((cpu.x[8] as u32 as u64))) as i32 as i64).wrapping_mul((((cpu.x[9] as u32 as u64))) as i32 as i64) as u64) as u64;
// 0x10fcb038 lsr x9, x8, #0x3f
cpu.x[9]=(((((cpu.x[8]))) as u64).wrapping_shr(((((0x3fu64))) & 63) as u32) as u64) as u64;
// 0x10fcb03c asr x8, x8, #0x25
cpu.x[8]=(((((cpu.x[8]))) as i64).wrapping_shr(((((0x25u64))) & 63) as u32) as u64) as u64;
// 0x10fcb040 add w8, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10fcb044 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcb048 csinc w26, w8, wzr, gt
cpu.x[26]=(if cpu.condition(12){((cpu.x[8] as u32 as u64))}else{(((0u64))).wrapping_add(1)}) as u32 as u64;
pc=0x10fcb04c;continue;
}
0x10fcb04c=>{
// 0x10fcb04c ldr x24, [x23, #0x28]
{let address=(cpu.x[23]).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[24]=(v0) as u64;}
// 0x10fcb050 str w26, [x22]
{let address=(cpu.x[22]).wrapping_add(0x0u64);let v0=cpu.x[26] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fcb054 cbz x24, #0x10fcb06c
if (cpu.x[24]) == 0 {pc=0x10fcb06c;continue;}pc=0x10fcb058;continue;
}
0x10fcb058=>{
// 0x10fcb058 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10fcb05c mov x1, x22
cpu.x[1]=(((cpu.x[22]))) as u64;
// 0x10fcb060 bl #0x10f78208
cpu.x[30]=0x10fcb064;super::chunk13::f_10f78208(cpu,rt,0x10f78208)?;
pc=0x10fcb064;continue;
}
0x10fcb064=>{
// 0x10fcb064 ldr w8, [x22]
{let address=(cpu.x[22]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcb068 cbz w8, #0x10fcb094
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10fcb094;continue;}pc=0x10fcb06c;continue;
}
0x10fcb06c=>{
// 0x10fcb06c adrp x8, #0x11a66000
cpu.x[8]=(0x11a66000u64) as u64;
// 0x10fcb070 add x8, x8, #0x18
{let a=((cpu.x[8]));let b=((0x18u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10fcb074 ldr x9, [x23, #0xb8]
{let address=(cpu.x[23]).wrapping_add(0xb8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fcb078 ldr x8, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcb07c orr x8, x9, x8
{let a=((cpu.x[9]));let b=((cpu.x[8]));let result=a | b;cpu.x[8]=(result) as u64;}
// 0x10fcb080 str x8, [x23, #0xb8]
{let address=(cpu.x[23]).wrapping_add(0xb8u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fcb084 tbnz w28, #3, #0x10fcb0b4
if ((cpu.x[28] as u32 as u64) & (1u64 << 3)) != 0 {pc=0x10fcb0b4;continue;}pc=0x10fcb088;continue;
}
0x10fcb088=>{
// 0x10fcb088 ldr w8, [x25, #0x1f4]
{let address=(cpu.x[25]).wrapping_add(0x1f4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcb08c cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcb090 b.le #0x10fcb0b4
if cpu.condition(13){pc=0x10fcb0b4;continue;}pc=0x10fcb094;continue;
}
0x10fcb094=>{
// 0x10fcb094 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10fcb098 b #0x10fcb244
pc=0x10fcb244;continue;
}
0x10fcb09c=>{
// 0x10fcb09c adrp x0, #0x102d1000
cpu.x[0]=(0x102d1000u64) as u64;
// 0x10fcb0a0 add x0, x0, #0xfd6
{let a=((cpu.x[0]));let b=((0xfd6u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fcb0a4 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10fcb0a8 bl #0x1128df18
cpu.x[30]=0x10fcb0ac;super::dispatch(cpu,rt,0x1128df18)?;
// 0x10fcb0ac mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10fcb0b0 b #0x10fcb244
pc=0x10fcb244;continue;
}
0x10fcb0b4=>{
// 0x10fcb0b4 cbz w27, #0x10fcb188
if (cpu.x[27] as u32 as u64) == 0 {pc=0x10fcb188;continue;}pc=0x10fcb0b8;continue;
}
0x10fcb0b8=>{
// 0x10fcb0b8 ldr w8, [x19, #0x64]
{let address=(cpu.x[19]).wrapping_add(0x64u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcb0bc cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcb0c0 b.lt #0x10fcb0f8
if cpu.condition(11){pc=0x10fcb0f8;continue;}pc=0x10fcb0c4;continue;
}
0x10fcb0c4=>{
// 0x10fcb0c4 ldur x9, [x19, #0x58]
{let address=(cpu.x[19]).wrapping_add(0x58u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fcb0c8 lsl x10, x8, #2
cpu.x[10]=(((((cpu.x[8]))) as u64).wrapping_shl(((((0x2u64))) & 63) as u32) as u64) as u64;
// 0x10fcb0cc mov x26, xzr
cpu.x[26]=(((0u64))) as u64;
pc=0x10fcb0d0;continue;
}
0x10fcb0d0=>{
// 0x10fcb0d0 ldr w11, [x9, x26]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[26])));let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[11]=(v0) as u32 as u64;}
// 0x10fcb0d4 cmp w11, w27
{let a=((cpu.x[11] as u32 as u64));let b=((cpu.x[27] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcb0d8 b.eq #0x10fcb0ec
if cpu.condition(0){pc=0x10fcb0ec;continue;}pc=0x10fcb0dc;continue;
}
0x10fcb0dc=>{
// 0x10fcb0dc add x26, x26, #4
{let a=((cpu.x[26]));let b=((0x4u64));let result=a.wrapping_add(b);cpu.x[26]=(result) as u64;}
// 0x10fcb0e0 cmp x10, x26
{let a=((cpu.x[10]));let b=((cpu.x[26]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fcb0e4 b.ne #0x10fcb0d0
if cpu.condition(1){pc=0x10fcb0d0;continue;}pc=0x10fcb0e8;continue;
}
0x10fcb0e8=>{
// 0x10fcb0e8 b #0x10fcb0f8
pc=0x10fcb0f8;continue;
}
0x10fcb0ec=>{
// 0x10fcb0ec mov x9, #0x3fffffffc
cpu.x[9]=(((0x3fffffffcu64))) as u64;
// 0x10fcb0f0 cmp x26, x9
{let a=((cpu.x[26]));let b=((cpu.x[9]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10fcb0f4 b.ne #0x10fcb280
if cpu.condition(1){pc=0x10fcb280;continue;}pc=0x10fcb0f8;continue;
}
0x10fcb0f8=>{
// 0x10fcb0f8 ldr w9, [x19, #0x60]
{let address=(cpu.x[19]).wrapping_add(0x60u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fcb0fc cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcb100 b.ne #0x10fcb128
if cpu.condition(1){pc=0x10fcb128;continue;}pc=0x10fcb104;continue;
}
0x10fcb104=>{
// 0x10fcb104 lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10fcb108 mov w10, #5
cpu.x[10]=(((0x5u64))) as u32 as u64;
// 0x10fcb10c cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcb110 csel w1, w10, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[10] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10fcb114 cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcb118 b.ge #0x10fcb128
if cpu.condition(10){pc=0x10fcb128;continue;}pc=0x10fcb11c;continue;
}
0x10fcb11c=>{
// 0x10fcb11c add x0, x19, #0x58
{let a=((cpu.x[19]));let b=((0x58u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fcb120 bl #0x1078f4d4
cpu.x[30]=0x10fcb124;super::chunk2::f_1078f4d4(cpu,rt,0x1078f4d4)?;
// 0x10fcb124 ldr w8, [x19, #0x64]
{let address=(cpu.x[19]).wrapping_add(0x64u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
pc=0x10fcb128;continue;
}
0x10fcb128=>{
// 0x10fcb128 ldr x9, [x19, #0x58]
{let address=(cpu.x[19]).wrapping_add(0x58u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fcb12c add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10fcb130 mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10fcb134 str w10, [x19, #0x64]
{let address=(cpu.x[19]).wrapping_add(0x64u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fcb138 str w27, [x9, w8, sxtw #2]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=cpu.x[27] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fcb13c bl #0x110ca998
cpu.x[30]=0x10fcb140;super::chunk3::f_110ca998(cpu,rt,0x110ca998)?;
// 0x10fcb140 ldp w9, w8, [x19, #0x70]
{let address=(cpu.x[19]).wrapping_add(0x70u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10fcb144 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcb148 b.ne #0x10fcb178
if cpu.condition(1){pc=0x10fcb178;continue;}pc=0x10fcb14c;continue;
}
0x10fcb14c=>{
// 0x10fcb14c lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10fcb150 mov w10, #5
cpu.x[10]=(((0x5u64))) as u32 as u64;
// 0x10fcb154 cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcb158 csel w1, w10, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[10] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10fcb15c cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcb160 b.ge #0x10fcb178
if cpu.condition(10){pc=0x10fcb178;continue;}pc=0x10fcb164;continue;
}
0x10fcb164=>{
// 0x10fcb164 mov w25, w0
cpu.x[25]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fcb168 add x0, x19, #0x68
{let a=((cpu.x[19]));let b=((0x68u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10fcb16c bl #0x1078f4d4
cpu.x[30]=0x10fcb170;super::chunk2::f_1078f4d4(cpu,rt,0x1078f4d4)?;
// 0x10fcb170 ldr w8, [x19, #0x74]
{let address=(cpu.x[19]).wrapping_add(0x74u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcb174 mov w0, w25
cpu.x[0]=(((cpu.x[25] as u32 as u64))) as u32 as u64;
pc=0x10fcb178;continue;
}
0x10fcb178=>{
// 0x10fcb178 ldr x9, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fcb17c add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10fcb180 str w10, [x19, #0x74]
{let address=(cpu.x[19]).wrapping_add(0x74u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fcb184 str w0, [x9, w8, sxtw #2]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10fcb188;continue;
}
0x10fcb188=>{
// 0x10fcb188 cbz x21, #0x10fcb1b8
if (cpu.x[21]) == 0 {pc=0x10fcb1b8;continue;}pc=0x10fcb18c;continue;
}
0x10fcb18c=>{
// 0x10fcb18c ldr x8, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcb190 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fcb194 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcb198 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fcb19c;super::dispatch(cpu,rt,target)?;}
pc=0x10fcb19c;continue;
}
0x10fcb19c=>{
// 0x10fcb19c cmp w0, #4
{let a=((cpu.x[0] as u32 as u64));let b=((0x4u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcb1a0 b.ne #0x10fcb260
if cpu.condition(1){pc=0x10fcb260;continue;}pc=0x10fcb1a4;continue;
}
0x10fcb1a4=>{
// 0x10fcb1a4 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fcb1a8 bl #0x10f8fc08
cpu.x[30]=0x10fcb1ac;super::chunk10::f_10f8fc08(cpu,rt,0x10f8fc08)?;
pc=0x10fcb1ac;continue;
}
0x10fcb1ac=>{
// 0x10fcb1ac cbz x0, #0x10fcb1b8
if (cpu.x[0]) == 0 {pc=0x10fcb1b8;continue;}pc=0x10fcb1b0;continue;
}
0x10fcb1b0=>{
// 0x10fcb1b0 mov x1, x23
cpu.x[1]=(((cpu.x[23]))) as u64;
// 0x10fcb1b4 bl #0x10f8585c
cpu.x[30]=0x10fcb1b8;super::chunk12::f_10f8585c(cpu,rt,0x10f8585c)?;
pc=0x10fcb1b8;continue;
}
0x10fcb1b8=>{
// 0x10fcb1b8 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fcb1bc bl #0x10fd04a8
cpu.x[30]=0x10fcb1c0;super::chunk2::f_10fd04a8(cpu,rt,0x10fd04a8)?;
pc=0x10fcb1c0;continue;
}
0x10fcb1c0=>{
// 0x10fcb1c0 bl #0x10e0b6b4
cpu.x[30]=0x10fcb1c4;super::chunk8::f_10e0b6b4(cpu,rt,0x10e0b6b4)?;
pc=0x10fcb1c4;continue;
}
0x10fcb1c4=>{
// 0x10fcb1c4 ldr x0, [x0, #0x78]
{let address=(cpu.x[0]).wrapping_add(0x78u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10fcb1c8 bl #0x10e16208
cpu.x[30]=0x10fcb1cc;super::chunk7::f_10e16208(cpu,rt,0x10e16208)?;
pc=0x10fcb1cc;continue;
}
0x10fcb1cc=>{
// 0x10fcb1cc ldrb w8, [x0, #0x439]
{let address=(cpu.x[0]).wrapping_add(0x439u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcb1d0 cbz w8, #0x10fcb214
if (cpu.x[8] as u32 as u64) == 0 {pc=0x10fcb214;continue;}pc=0x10fcb1d4;continue;
}
0x10fcb1d4=>{
// 0x10fcb1d4 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcb1d8 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fcb1dc ldr x8, [x8, #0x170]
{let address=(cpu.x[8]).wrapping_add(0x170u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcb1e0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fcb1e4;super::dispatch(cpu,rt,target)?;}
pc=0x10fcb1e4;continue;
}
0x10fcb1e4=>{
// 0x10fcb1e4 tbz w0, #0, #0x10fcb214
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fcb214;continue;}pc=0x10fcb1e8;continue;
}
0x10fcb1e8=>{
// 0x10fcb1e8 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fcb1ec bl #0x10fa3348
cpu.x[30]=0x10fcb1f0;super::chunk4::f_10fa3348(cpu,rt,0x10fa3348)?;
pc=0x10fcb1f0;continue;
}
0x10fcb1f0=>{
// 0x10fcb1f0 tbz w0, #0, #0x10fcb214
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fcb214;continue;}pc=0x10fcb1f4;continue;
}
0x10fcb1f4=>{
// 0x10fcb1f4 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10fcb1f8 bl #0x10f93d2c
cpu.x[30]=0x10fcb1fc;super::chunk11::f_10f93d2c(cpu,rt,0x10f93d2c)?;
// 0x10fcb1fc mov x1, x23
cpu.x[1]=(((cpu.x[23]))) as u64;
// 0x10fcb200 bl #0x10d3c2ac
cpu.x[30]=0x10fcb204;super::chunk1::f_10d3c2ac(cpu,rt,0x10d3c2ac)?;
// 0x10fcb204 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcb208 ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcb20c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fcb210;super::dispatch(cpu,rt,target)?;}
// 0x10fcb210 b #0x10fcb218
pc=0x10fcb218;continue;
}
0x10fcb214=>{
// 0x10fcb214 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
pc=0x10fcb218;continue;
}
0x10fcb218=>{
// 0x10fcb218 mov x1, x22
cpu.x[1]=(((cpu.x[22]))) as u64;
// 0x10fcb21c bl #0x10fcb29c
cpu.x[30]=0x10fcb220;super::chunk13::f_10fcb29c(cpu,rt,0x10fcb29c)?;
pc=0x10fcb220;continue;
}
0x10fcb220=>{
// 0x10fcb220 tbz w0, #0, #0x10fcb234
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10fcb234;continue;}pc=0x10fcb224;continue;
}
0x10fcb224=>{
// 0x10fcb224 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10fcb228 mov x1, x21
cpu.x[1]=(((cpu.x[21]))) as u64;
// 0x10fcb22c mov w2, w20
cpu.x[2]=(((cpu.x[20] as u32 as u64))) as u32 as u64;
// 0x10fcb230 bl #0x10fca5ec
cpu.x[30]=0x10fcb234;super::chunk14::f_10fca5ec(cpu,rt,0x10fca5ec)?;
pc=0x10fcb234;continue;
}
0x10fcb234=>{
// 0x10fcb234 cbz x24, #0x10fcb240
if (cpu.x[24]) == 0 {pc=0x10fcb240;continue;}pc=0x10fcb238;continue;
}
0x10fcb238=>{
// 0x10fcb238 mov x0, x24
cpu.x[0]=(((cpu.x[24]))) as u64;
// 0x10fcb23c bl #0x10f781b8
cpu.x[30]=0x10fcb240;super::chunk12::f_10f781b8(cpu,rt,0x10f781b8)?;
pc=0x10fcb240;continue;
}
0x10fcb240=>{
// 0x10fcb240 mov w0, #1
cpu.x[0]=(((0x1u64))) as u32 as u64;
pc=0x10fcb244;continue;
}
0x10fcb244=>{
// 0x10fcb244 ldp x20, x19, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fcb248 ldp x22, x21, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10fcb24c ldp x24, x23, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10fcb250 ldp x26, x25, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10fcb254 ldp x28, x27, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10fcb258 ldp x29, x30, [sp], #0x60
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x60u64)))) as u64;}
// 0x10fcb25c ret
cpu.pc=cpu.x[30];return Ok(());
}
0x10fcb260=>{
// 0x10fcb260 ldr x8, [x21]
{let address=(cpu.x[21]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcb264 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fcb268 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10fcb26c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10fcb270;super::dispatch(cpu,rt,target)?;}
pc=0x10fcb270;continue;
}
0x10fcb270=>{
// 0x10fcb270 cmp w0, #5
{let a=((cpu.x[0] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcb274 mov x0, x21
cpu.x[0]=(((cpu.x[21]))) as u64;
// 0x10fcb278 b.eq #0x10fcb1b0
if cpu.condition(0){pc=0x10fcb1b0;continue;}pc=0x10fcb27c;continue;
}
0x10fcb27c=>{
// 0x10fcb27c b #0x10fcb1b8
pc=0x10fcb1b8;continue;
}
0x10fcb280=>{
// 0x10fcb280 mov x0, x25
cpu.x[0]=(((cpu.x[25]))) as u64;
// 0x10fcb284 bl #0x110ca998
cpu.x[30]=0x10fcb288;super::chunk3::f_110ca998(cpu,rt,0x110ca998)?;
// 0x10fcb288 ldr x9, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10fcb28c mov w8, w0
cpu.x[8]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10fcb290 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x10fcb294 str w8, [x9, x26]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[26])));let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fcb298 b #0x10fcb244
pc=0x10fcb244;continue;
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fcaeec")),}}}
#[inline]
pub(super) unsafe fn f_10fcc040(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fcc040=>{
// 0x10fcc040 ldr w8, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10fcc044 ldr w9, [x0, #0x18]
{let address=(cpu.x[0]).wrapping_add(0x18u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10fcc048 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10fcc04c csel w0, wzr, w9, gt
cpu.x[0]=(if cpu.condition(12){((0u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10fcc050 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fcc040")),}}}
#[inline]
pub(super) unsafe fn f_10fce8f4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fce8f4=>{
// 0x10fce8f4 str wzr, [x0, #0x38]
{let address=(cpu.x[0]).wrapping_add(0x38u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10fce8f8 strb wzr, [x0, #0x215]
{let address=(cpu.x[0]).wrapping_add(0x215u64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x10fce8fc ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fce8f4")),}}}
#[inline]
pub(super) unsafe fn f_10fd11f4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fd11f4=>{
// 0x10fd11f4 cbz x1, #0x10fd122c
if (cpu.x[1]) == 0 {pc=0x10fd122c;continue;}pc=0x10fd11f8;continue;
}
0x10fd11f8=>{
// 0x10fd11f8 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10fd11fc stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10fd1200 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10fd1204 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x10fd1208 mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x10fd120c mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x10fd1210 bl #0x10f93f94
cpu.x[30]=0x10fd1214;super::chunk3::f_10f93f94(cpu,rt,0x10f93f94)?;
pc=0x10fd1214;continue;
}
0x10fd1214=>{
// 0x10fd1214 bl #0x10f9915c
cpu.x[30]=0x10fd1218;super::chunk11::f_10f9915c(cpu,rt,0x10f9915c)?;
pc=0x10fd1218;continue;
}
0x10fd1218=>{
// 0x10fd1218 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x10fd121c str x0, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10fd1220 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10fd1224 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x10fd1228 b #0x10f879e0
super::chunk1::f_10f879e0(cpu,rt,0x10f879e0)?;return Ok(());
}
0x10fd122c=>{
// 0x10fd122c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fd11f4")),}}}
#[inline]
pub(super) unsafe fn f_10fd3f88(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10fd3f88=>{
// 0x10fd3f88 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10fd3f88")),}}}
#[inline]
pub(super) unsafe fn f_10feadcc(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x10feadcc=>{
// 0x10feadcc stp x29, x30, [sp, #-0x60]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffa0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x10feadd0 stp x28, x27, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[28];let v1=cpu.x[27];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10feadd4 stp x26, x25, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[26];let v1=cpu.x[25];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10feadd8 stp x24, x23, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[24];let v1=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10feaddc stp x22, x21, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10feade0 stp x20, x19, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x10feade4 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x10feade8 ldr x8, [x1]
{let address=(cpu.x[1]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10feadec mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x10feadf0 mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x10feadf4 mov x19, x1
cpu.x[19]=(((cpu.x[1]))) as u64;
// 0x10feadf8 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10feadfc blr x8
{let target=cpu.x[8];cpu.x[30]=0x10feae00;super::dispatch(cpu,rt,target)?;}
pc=0x10feae00;continue;
}
0x10feae00=>{
// 0x10feae00 mov w22, w0
cpu.x[22]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10feae04 cmp w0, #0x29
{let a=((cpu.x[0] as u32 as u64));let b=((0x29u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10feae08 b.lt #0x10feae1c
if cpu.condition(11){pc=0x10feae1c;continue;}pc=0x10feae0c;continue;
}
0x10feae0c=>{
// 0x10feae0c adrp x0, #0x102f2000
cpu.x[0]=(0x102f2000u64) as u64;
// 0x10feae10 add x0, x0, #0x4bf
{let a=((cpu.x[0]));let b=((0x4bfu64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10feae14 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10feae18 bl #0x1128df18
cpu.x[30]=0x10feae1c;super::dispatch(cpu,rt,0x1128df18)?;
pc=0x10feae1c;continue;
}
0x10feae1c=>{
// 0x10feae1c ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10feae20 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10feae24 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10feae28 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10feae2c;super::dispatch(cpu,rt,target)?;}
pc=0x10feae2c;continue;
}
0x10feae2c=>{
// 0x10feae2c mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10feae30 cmp w0, #5
{let a=((cpu.x[0] as u32 as u64));let b=((0x5u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10feae34 b.lt #0x10feae48
if cpu.condition(11){pc=0x10feae48;continue;}pc=0x10feae38;continue;
}
0x10feae38=>{
// 0x10feae38 adrp x0, #0x1033a000
cpu.x[0]=(0x1033a000u64) as u64;
// 0x10feae3c add x0, x0, #0x254
{let a=((cpu.x[0]));let b=((0x254u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10feae40 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x10feae44 bl #0x1128df18
cpu.x[30]=0x10feae48;super::dispatch(cpu,rt,0x1128df18)?;
pc=0x10feae48;continue;
}
0x10feae48=>{
// 0x10feae48 ldr w8, [x20, #0x4c]
{let address=(cpu.x[20]).wrapping_add(0x4cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
pc=0x10feae4c;continue;
}
0x10feae4c=>{
// 0x10feae4c cmp w8, w21
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[21] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10feae50 b.le #0x10feae8c
if cpu.condition(13){pc=0x10feae8c;continue;}pc=0x10feae54;continue;
}
0x10feae54=>{
// 0x10feae54 ldr x9, [x20, #0x40]
{let address=(cpu.x[20]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10feae58 sub w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10feae5c str w8, [x20, #0x4c]
{let address=(cpu.x[20]).wrapping_add(0x4cu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10feae60 ldr x23, [x9, w8, sxtw #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10feae64 cbz x23, #0x10feae4c
if (cpu.x[23]) == 0 {pc=0x10feae4c;continue;}pc=0x10feae68;continue;
}
0x10feae68=>{
// 0x10feae68 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10feae6c mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10feae70 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10feae74 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10feae78;super::dispatch(cpu,rt,target)?;}
// 0x10feae78 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10feae7c mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10feae80 ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10feae84 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10feae88;super::dispatch(cpu,rt,target)?;}
// 0x10feae88 b #0x10feae48
pc=0x10feae48;continue;
}
0x10feae8c=>{
// 0x10feae8c cmp w8, w21
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[21] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10feae90 b.ge #0x10feaed8
if cpu.condition(10){pc=0x10feaed8;continue;}pc=0x10feae94;continue;
}
0x10feae94=>{
// 0x10feae94 ldr w9, [x20, #0x48]
{let address=(cpu.x[20]).wrapping_add(0x48u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10feae98 cmp w9, w21
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[21] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10feae9c b.ge #0x10feaeb0
if cpu.condition(10){pc=0x10feaeb0;continue;}pc=0x10feaea0;continue;
}
0x10feaea0=>{
// 0x10feaea0 add x0, x20, #0x40
{let a=((cpu.x[20]));let b=((0x40u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10feaea4 mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10feaea8 bl #0x10feda98
cpu.x[30]=0x10feaeac;super::chunk0::f_10feda98(cpu,rt,0x10feda98)?;
// 0x10feaeac ldr w8, [x20, #0x4c]
{let address=(cpu.x[20]).wrapping_add(0x4cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
pc=0x10feaeb0;continue;
}
0x10feaeb0=>{
// 0x10feaeb0 cmp w8, w21
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[21] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10feaeb4 b.ge #0x10feaed4
if cpu.condition(10){pc=0x10feaed4;continue;}pc=0x10feaeb8;continue;
}
0x10feaeb8=>{
// 0x10feaeb8 sxtw x8, w8
cpu.x[8]=((((cpu.x[8] as u32 as u64))) as i32 as i64 as u64) as u64;
// 0x10feaebc sxtw x9, w21
cpu.x[9]=((((cpu.x[21] as u32 as u64))) as i32 as i64 as u64) as u64;
pc=0x10feaec0;continue;
}
0x10feaec0=>{
// 0x10feaec0 ldur x10, [x20, #0x40]
{let address=(cpu.x[20]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10feaec4 str xzr, [x10, x8, lsl #3]
{let address=((cpu.x[10]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[8])) as u64).wrapping_shl(3) as u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10feaec8 add x8, x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10feaecc cmp x9, x8
{let a=((cpu.x[9]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10feaed0 b.ne #0x10feaec0
if cpu.condition(1){pc=0x10feaec0;continue;}pc=0x10feaed4;continue;
}
0x10feaed4=>{
// 0x10feaed4 str w21, [x20, #0x4c]
{let address=(cpu.x[20]).wrapping_add(0x4cu64);let v0=cpu.x[21] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10feaed8;continue;
}
0x10feaed8=>{
// 0x10feaed8 ldr w8, [x20, #0x2c]
{let address=(cpu.x[20]).wrapping_add(0x2cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
pc=0x10feaedc;continue;
}
0x10feaedc=>{
// 0x10feaedc cmp w8, w22
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[22] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10feaee0 b.le #0x10feaf1c
if cpu.condition(13){pc=0x10feaf1c;continue;}pc=0x10feaee4;continue;
}
0x10feaee4=>{
// 0x10feaee4 ldr x9, [x20, #0x20]
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10feaee8 sub w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;}
// 0x10feaeec str w8, [x20, #0x2c]
{let address=(cpu.x[20]).wrapping_add(0x2cu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10feaef0 ldr x23, [x9, w8, sxtw #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10feaef4 cbz x23, #0x10feaedc
if (cpu.x[23]) == 0 {pc=0x10feaedc;continue;}pc=0x10feaef8;continue;
}
0x10feaef8=>{
// 0x10feaef8 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10feaefc mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10feaf00 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10feaf04 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10feaf08;super::dispatch(cpu,rt,target)?;}
// 0x10feaf08 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10feaf0c mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10feaf10 ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10feaf14 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10feaf18;super::dispatch(cpu,rt,target)?;}
// 0x10feaf18 b #0x10feaed8
pc=0x10feaed8;continue;
}
0x10feaf1c=>{
// 0x10feaf1c cmp w8, w22
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[22] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10feaf20 b.ge #0x10feaf68
if cpu.condition(10){pc=0x10feaf68;continue;}pc=0x10feaf24;continue;
}
0x10feaf24=>{
// 0x10feaf24 ldr w9, [x20, #0x28]
{let address=(cpu.x[20]).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x10feaf28 cmp w9, w22
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[22] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10feaf2c b.ge #0x10feaf40
if cpu.condition(10){pc=0x10feaf40;continue;}pc=0x10feaf30;continue;
}
0x10feaf30=>{
// 0x10feaf30 add x0, x20, #0x20
{let a=((cpu.x[20]));let b=((0x20u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10feaf34 mov w1, w22
cpu.x[1]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10feaf38 bl #0x108c5574
cpu.x[30]=0x10feaf3c;super::chunk12::f_108c5574(cpu,rt,0x108c5574)?;
// 0x10feaf3c ldr w8, [x20, #0x2c]
{let address=(cpu.x[20]).wrapping_add(0x2cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
pc=0x10feaf40;continue;
}
0x10feaf40=>{
// 0x10feaf40 cmp w8, w22
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[22] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10feaf44 b.ge #0x10feaf64
if cpu.condition(10){pc=0x10feaf64;continue;}pc=0x10feaf48;continue;
}
0x10feaf48=>{
// 0x10feaf48 sxtw x8, w8
cpu.x[8]=((((cpu.x[8] as u32 as u64))) as i32 as i64 as u64) as u64;
// 0x10feaf4c sxtw x9, w22
cpu.x[9]=((((cpu.x[22] as u32 as u64))) as i32 as i64 as u64) as u64;
pc=0x10feaf50;continue;
}
0x10feaf50=>{
// 0x10feaf50 ldur x10, [x20, #0x20]
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;}
// 0x10feaf54 str xzr, [x10, x8, lsl #3]
{let address=((cpu.x[10]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[8])) as u64).wrapping_shl(3) as u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10feaf58 add x8, x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x10feaf5c cmp x9, x8
{let a=((cpu.x[9]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10feaf60 b.ne #0x10feaf50
if cpu.condition(1){pc=0x10feaf50;continue;}pc=0x10feaf64;continue;
}
0x10feaf64=>{
// 0x10feaf64 str w22, [x20, #0x2c]
{let address=(cpu.x[20]).wrapping_add(0x2cu64);let v0=cpu.x[22] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x10feaf68;continue;
}
0x10feaf68=>{
// 0x10feaf68 cmp w22, #1
{let a=((cpu.x[22] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10feaf6c mov w24, w22
cpu.x[24]=(((cpu.x[22] as u32 as u64))) as u32 as u64;
// 0x10feaf70 b.lt #0x10feafec
if cpu.condition(11){pc=0x10feafec;continue;}pc=0x10feaf74;continue;
}
0x10feaf74=>{
// 0x10feaf74 mov x26, xzr
cpu.x[26]=(((0u64))) as u64;
// 0x10feaf78 mov x25, xzr
cpu.x[25]=(((0u64))) as u64;
// 0x10feaf7c mov w27, #1
cpu.x[27]=(((0x1u64))) as u32 as u64;
// 0x10feaf80 b #0x10feaf98
pc=0x10feaf98;continue;
}
0x10feaf84=>{
// 0x10feaf84 str xzr, [x8, x26, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[26])) as u64).wrapping_shl(3) as u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10feaf88;continue;
}
0x10feaf88=>{
// 0x10feaf88 add x26, x26, #1
{let a=((cpu.x[26]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[26]=(result) as u64;}
// 0x10feaf8c orr x25, x28, x25
{let a=((cpu.x[28]));let b=((cpu.x[25]));let result=a | b;cpu.x[25]=(result) as u64;}
// 0x10feaf90 cmp x24, x26
{let a=((cpu.x[24]));let b=((cpu.x[26]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10feaf94 b.eq #0x10feaff0
if cpu.condition(0){pc=0x10feaff0;continue;}pc=0x10feaf98;continue;
}
0x10feaf98=>{
// 0x10feaf98 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10feaf9c mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10feafa0 ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10feafa4 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10feafa8;super::dispatch(cpu,rt,target)?;}
pc=0x10feafa8;continue;
}
0x10feafa8=>{
// 0x10feafa8 lsl x8, x27, x26
cpu.x[8]=(((((cpu.x[27]))) as u64).wrapping_shl(((((cpu.x[26]))) & 63) as u32) as u64) as u64;
// 0x10feafac ands w9, w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[9]=(result) as u32 as u64;cpu.logical_flags(result,32);}
// 0x10feafb0 csel x28, x8, xzr, ne
cpu.x[28]=(if cpu.condition(1){((cpu.x[8]))}else{((0u64))}) as u64;
// 0x10feafb4 cbnz w9, #0x10feaf88
if (cpu.x[9] as u32 as u64) != 0 {pc=0x10feaf88;continue;}pc=0x10feafb8;continue;
}
0x10feafb8=>{
// 0x10feafb8 ldur x8, [x20, #0x20]
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10feafbc ldr x23, [x8, x26, lsl #3]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[26])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10feafc0 cbz x23, #0x10feaf84
if (cpu.x[23]) == 0 {pc=0x10feaf84;continue;}pc=0x10feafc4;continue;
}
0x10feafc4=>{
// 0x10feafc4 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10feafc8 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10feafcc ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10feafd0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10feafd4;super::dispatch(cpu,rt,target)?;}
// 0x10feafd4 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10feafd8 mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10feafdc ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10feafe0 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10feafe4;super::dispatch(cpu,rt,target)?;}
// 0x10feafe4 ldur x8, [x20, #0x20]
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10feafe8 b #0x10feaf84
pc=0x10feaf84;continue;
}
0x10feafec=>{
// 0x10feafec mov x25, xzr
cpu.x[25]=(((0u64))) as u64;
pc=0x10feaff0;continue;
}
0x10feaff0=>{
// 0x10feaff0 cmp w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10feaff4 b.lt #0x10feb080
if cpu.condition(11){pc=0x10feb080;continue;}pc=0x10feaff8;continue;
}
0x10feaff8=>{
// 0x10feaff8 mov w8, w21
cpu.x[8]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x10feaffc mov x27, xzr
cpu.x[27]=(((0u64))) as u64;
pc=0x10feb000;continue;
}
0x10feb000=>{
// 0x10feb000 ubfiz x26, x8, #3, #0x20
cpu.x[26]=(((((cpu.x[8])) & 0xffffffffu64) << 3)) as u64;
// 0x10feb004 b #0x10feb01c
pc=0x10feb01c;continue;
}
0x10feb008=>{
// 0x10feb008 ldur x8, [x20, #0x40]
{let address=(cpu.x[20]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10feb00c str x23, [x8, x27]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[27])));let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x10feb010;continue;
}
0x10feb010=>{
// 0x10feb010 add x27, x27, #8
{let a=((cpu.x[27]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[27]=(result) as u64;}
// 0x10feb014 cmp x26, x27
{let a=((cpu.x[26]));let b=((cpu.x[27]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10feb018 b.eq #0x10feb080
if cpu.condition(0){pc=0x10feb080;continue;}pc=0x10feb01c;continue;
}
0x10feb01c=>{
// 0x10feb01c ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10feb020 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10feb024 ldr x8, [x8, #0xe0]
{let address=(cpu.x[8]).wrapping_add(0xe0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10feb028 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10feb02c;super::dispatch(cpu,rt,target)?;}
pc=0x10feb02c;continue;
}
0x10feb02c=>{
// 0x10feb02c ldur x8, [x20, #0x40]
{let address=(cpu.x[20]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10feb030 ldr x23, [x8, x27]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[27])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x10feb034 tbz w0, #0, #0x10feb054
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10feb054;continue;}pc=0x10feb038;continue;
}
0x10feb038=>{
// 0x10feb038 cbnz x23, #0x10feb010
if (cpu.x[23]) != 0 {pc=0x10feb010;continue;}pc=0x10feb03c;continue;
}
0x10feb03c=>{
// 0x10feb03c mov w0, #0x30
cpu.x[0]=(((0x30u64))) as u32 as u64;
// 0x10feb040 bl #0x1187d2b0
cpu.x[30]=0x10feb044;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
pc=0x10feb044;continue;
}
0x10feb044=>{
// 0x10feb044 ldr x1, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10feb048 mov x23, x0
cpu.x[23]=(((cpu.x[0]))) as u64;
// 0x10feb04c bl #0x11058f84
cpu.x[30]=0x10feb050;super::chunk4::f_11058f84(cpu,rt,0x11058f84)?;
pc=0x10feb050;continue;
}
0x10feb050=>{
// 0x10feb050 b #0x10feb008
pc=0x10feb008;continue;
}
0x10feb054=>{
// 0x10feb054 cbz x23, #0x10feb010
if (cpu.x[23]) == 0 {pc=0x10feb010;continue;}pc=0x10feb058;continue;
}
0x10feb058=>{
// 0x10feb058 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10feb05c mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10feb060 ldr x8, [x8, #0x10]
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10feb064 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10feb068;super::dispatch(cpu,rt,target)?;}
// 0x10feb068 ldr x8, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10feb06c mov x0, x23
cpu.x[0]=(((cpu.x[23]))) as u64;
// 0x10feb070 ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10feb074 blr x8
{let target=cpu.x[8];cpu.x[30]=0x10feb078;super::dispatch(cpu,rt,target)?;}
// 0x10feb078 mov x23, xzr
cpu.x[23]=(((0u64))) as u64;
// 0x10feb07c b #0x10feb008
pc=0x10feb008;continue;
}
0x10feb080=>{
// 0x10feb080 cmp w22, #1
{let a=((cpu.x[22] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10feb084 b.lt #0x10feb0c8
if cpu.condition(11){pc=0x10feb0c8;continue;}pc=0x10feb088;continue;
}
0x10feb088=>{
// 0x10feb088 mov x22, xzr
cpu.x[22]=(((0u64))) as u64;
// 0x10feb08c b #0x10feb09c
pc=0x10feb09c;continue;
}
0x10feb090=>{
// 0x10feb090 add x22, x22, #1
{let a=((cpu.x[22]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[22]=(result) as u64;}
// 0x10feb094 cmp x24, x22
{let a=((cpu.x[24]));let b=((cpu.x[22]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10feb098 b.eq #0x10feb0c8
if cpu.condition(0){pc=0x10feb0c8;continue;}pc=0x10feb09c;continue;
}
0x10feb09c=>{
// 0x10feb09c lsr x8, x25, x22
cpu.x[8]=(((((cpu.x[25]))) as u64).wrapping_shr(((((cpu.x[22]))) & 63) as u32) as u64) as u64;
// 0x10feb0a0 tbz w8, #0, #0x10feb090
if ((cpu.x[8] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x10feb090;continue;}pc=0x10feb0a4;continue;
}
0x10feb0a4=>{
// 0x10feb0a4 ldr x8, [x20, #0x20]
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10feb0a8 lsl x23, x22, #3
cpu.x[23]=(((((cpu.x[22]))) as u64).wrapping_shl(((((0x3u64))) & 63) as u32) as u64) as u64;
// 0x10feb0ac ldr x0, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10feb0b0 mov x2, x19
cpu.x[2]=(((cpu.x[19]))) as u64;
// 0x10feb0b4 ldr x1, [x8, x23]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[23])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x10feb0b8 bl #0x11056c0c
cpu.x[30]=0x10feb0bc;super::chunk2::f_11056c0c(cpu,rt,0x11056c0c)?;
pc=0x10feb0bc;continue;
}
0x10feb0bc=>{
// 0x10feb0bc ldr x8, [x20, #0x20]
{let address=(cpu.x[20]).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10feb0c0 str x0, [x8, x23]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[23])));let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10feb0c4 b #0x10feb090
pc=0x10feb090;continue;
}
0x10feb0c8=>{
// 0x10feb0c8 cmp w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10feb0cc b.lt #0x10feb100
if cpu.condition(11){pc=0x10feb100;continue;}pc=0x10feb0d0;continue;
}
0x10feb0d0=>{
// 0x10feb0d0 mov x22, xzr
cpu.x[22]=(((0u64))) as u64;
// 0x10feb0d4 ubfiz x21, x21, #3, #0x20
cpu.x[21]=(((((cpu.x[21])) & 0xffffffffu64) << 3)) as u64;
// 0x10feb0d8 b #0x10feb0e8
pc=0x10feb0e8;continue;
}
0x10feb0dc=>{
// 0x10feb0dc add x22, x22, #8
{let a=((cpu.x[22]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[22]=(result) as u64;}
// 0x10feb0e0 cmp x21, x22
{let a=((cpu.x[21]));let b=((cpu.x[22]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10feb0e4 b.eq #0x10feb100
if cpu.condition(0){pc=0x10feb100;continue;}pc=0x10feb0e8;continue;
}
0x10feb0e8=>{
// 0x10feb0e8 ldur x8, [x20, #0x40]
{let address=(cpu.x[20]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10feb0ec ldr x0, [x8, x22]
{let address=((cpu.x[8]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[22])));let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10feb0f0 cbz x0, #0x10feb0dc
if (cpu.x[0]) == 0 {pc=0x10feb0dc;continue;}pc=0x10feb0f4;continue;
}
0x10feb0f4=>{
// 0x10feb0f4 mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10feb0f8 bl #0x110591d4
cpu.x[30]=0x10feb0fc;super::chunk4::f_110591d4(cpu,rt,0x110591d4)?;
pc=0x10feb0fc;continue;
}
0x10feb0fc=>{
// 0x10feb0fc b #0x10feb0dc
pc=0x10feb0dc;continue;
}
0x10feb100=>{
// 0x10feb100 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10feb104 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10feb108 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10feb10c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10feb110;super::dispatch(cpu,rt,target)?;}
pc=0x10feb110;continue;
}
0x10feb110=>{
// 0x10feb110 ldr w8, [x20, #0x3c]
{let address=(cpu.x[20]).wrapping_add(0x3cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10feb114 str w0, [x20, #0x50]
{let address=(cpu.x[20]).wrapping_add(0x50u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10feb118 cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10feb11c b.lt #0x10feb14c
if cpu.condition(11){pc=0x10feb14c;continue;}pc=0x10feb120;continue;
}
0x10feb120=>{
// 0x10feb120 mov x21, xzr
cpu.x[21]=(((0u64))) as u64;
// 0x10feb124 b #0x10feb134
pc=0x10feb134;continue;
}
0x10feb128=>{
// 0x10feb128 add x21, x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[21]=(result) as u64;}
// 0x10feb12c cmp x21, w8, sxtw
{let a=((cpu.x[21]));let b=(((cpu.x[8] as u32 as u64) as i32 as i64 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x10feb130 b.ge #0x10feb14c
if cpu.condition(10){pc=0x10feb14c;continue;}pc=0x10feb134;continue;
}
0x10feb134=>{
// 0x10feb134 ldur x9, [x20, #0x30]
{let address=(cpu.x[20]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10feb138 ldr x0, [x9, x21, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[21])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x10feb13c cbz x0, #0x10feb128
if (cpu.x[0]) == 0 {pc=0x10feb128;continue;}pc=0x10feb140;continue;
}
0x10feb140=>{
// 0x10feb140 bl #0x1187d2c0
cpu.x[30]=0x10feb144;super::dispatch(cpu,rt,0x1187d2c0)?;
// 0x10feb144 ldr w8, [x20, #0x3c]
{let address=(cpu.x[20]).wrapping_add(0x3cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10feb148 b #0x10feb128
pc=0x10feb128;continue;
}
0x10feb14c=>{
// 0x10feb14c ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10feb150 str wzr, [x20, #0x3c]
{let address=(cpu.x[20]).wrapping_add(0x3cu64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10feb154 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x10feb158 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x10feb15c blr x8
{let target=cpu.x[8];cpu.x[30]=0x10feb160;super::dispatch(cpu,rt,target)?;}
pc=0x10feb160;continue;
}
0x10feb160=>{
// 0x10feb160 cmp w0, #1
{let a=((cpu.x[0] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10feb164 b.lt #0x10feb1e4
if cpu.condition(11){pc=0x10feb1e4;continue;}pc=0x10feb168;continue;
}
0x10feb168=>{
// 0x10feb168 mov w21, w0
cpu.x[21]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x10feb16c mov w23, #5
cpu.x[23]=(((0x5u64))) as u32 as u64;
// 0x10feb170 b #0x10feb18c
pc=0x10feb18c;continue;
}
0x10feb174=>{
// 0x10feb174 ldr x9, [x20, #0x30]
{let address=(cpu.x[20]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x10feb178 add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x10feb17c subs w21, w21, #1
{let a=((cpu.x[21] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[21]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x10feb180 str w10, [x20, #0x3c]
{let address=(cpu.x[20]).wrapping_add(0x3cu64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x10feb184 str x22, [x9, w8, sxtw #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(3) as u64);let v0=cpu.x[22];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10feb188 b.eq #0x10feb1e4
if cpu.condition(0){pc=0x10feb1e4;continue;}pc=0x10feb18c;continue;
}
0x10feb18c=>{
// 0x10feb18c mov w0, #0x18
cpu.x[0]=(((0x18u64))) as u32 as u64;
// 0x10feb190 bl #0x1187d2b0
cpu.x[30]=0x10feb194;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
// 0x10feb194 movi v0.2d, #0xffffffffffffffff
cpu.q[0]=(lane_dup(18446744073709551615u128,64,2)) & 0xffffffffffffffffffffffffffffffffu128;
// 0x10feb198 ldr x2, [x20, #8]
{let address=(cpu.x[20]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x10feb19c mov x1, x19
cpu.x[1]=(((cpu.x[19]))) as u64;
// 0x10feb1a0 mov x22, x0
cpu.x[22]=(((cpu.x[0]))) as u64;
// 0x10feb1a4 str xzr, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10feb1a8 strh wzr, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=0u64;rt.memory.write::<u16>(address.wrapping_add(0),v0 as u16);}
// 0x10feb1ac str d0, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=(cpu.q[0] & 0xffffffffffffffffu128) as u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x10feb1b0 bl #0x10f8775c
cpu.x[30]=0x10feb1b4;super::dispatch(cpu,rt,0x10f8775c)?;
// 0x10feb1b4 ldp w9, w8, [x20, #0x38]
{let address=(cpu.x[20]).wrapping_add(0x38u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x10feb1b8 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10feb1bc b.ne #0x10feb174
if cpu.condition(1){pc=0x10feb174;continue;}pc=0x10feb1c0;continue;
}
0x10feb1c0=>{
// 0x10feb1c0 lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x10feb1c4 cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10feb1c8 csel w1, w23, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[23] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x10feb1cc cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x10feb1d0 b.ge #0x10feb174
if cpu.condition(10){pc=0x10feb174;continue;}pc=0x10feb1d4;continue;
}
0x10feb1d4=>{
// 0x10feb1d4 add x0, x20, #0x30
{let a=((cpu.x[20]));let b=((0x30u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x10feb1d8 bl #0x10fedb7c
cpu.x[30]=0x10feb1dc;super::dispatch(cpu,rt,0x10fedb7c)?;
// 0x10feb1dc ldr w8, [x20, #0x3c]
{let address=(cpu.x[20]).wrapping_add(0x3cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x10feb1e0 b #0x10feb174
pc=0x10feb174;continue;
}
0x10feb1e4=>{
// 0x10feb1e4 ldp x20, x19, [sp, #0x50]
{let address=(cpu.sp).wrapping_add(0x50u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x10feb1e8 ldp x22, x21, [sp, #0x40]
{let address=(cpu.sp).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x10feb1ec ldp x24, x23, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[24]=(v0) as u64;cpu.x[23]=(v1) as u64;}
// 0x10feb1f0 ldp x26, x25, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[26]=(v0) as u64;cpu.x[25]=(v1) as u64;}
// 0x10feb1f4 ldp x28, x27, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[28]=(v0) as u64;cpu.x[27]=(v1) as u64;}
// 0x10feb1f8 ldp x29, x30, [sp], #0x60
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x60u64)))) as u64;}
// 0x10feb1fc ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x10feadcc")),}}}
#[inline]
pub(super) unsafe fn f_11057580(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x11057580=>{
// 0x11057580 str x1, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=cpu.x[1];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x11057584 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x11057580")),}}}
#[inline]
pub(super) unsafe fn f_11058fc0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x11058fc0=>{
// 0x11058fc0 ldr x0, [x0, #0x18]
{let address=(cpu.x[0]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x11058fc4 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x11058fc0")),}}}
#[inline]
pub(super) unsafe fn f_110cb66c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x110cb66c=>{
// 0x110cb66c stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x110cb670 str x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x110cb674 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x110cb678 mov w19, w1
cpu.x[19]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x110cb67c sub w1, w2, w1
{let a=((cpu.x[2] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[1]=(result) as u32 as u64;}
// 0x110cb680 add x0, x0, #0xa0
{let a=((cpu.x[0]));let b=((0xa0u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x110cb684 bl #0x11294bb4
cpu.x[30]=0x110cb688;super::chunk13::f_11294bb4(cpu,rt,0x11294bb4)?;
pc=0x110cb688;continue;
}
0x110cb688=>{
// 0x110cb688 add w0, w0, w19
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[19] as u32 as u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u32 as u64;}
// 0x110cb68c ldr x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[19]=(v0) as u64;}
// 0x110cb690 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x110cb694 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x110cb66c")),}}}
#[inline]
pub(super) unsafe fn f_111d7a98(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111d7a98=>{
// 0x111d7a98 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x111d7a9c stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d7aa0 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x111d7aa4 movi v1.2d, #0000000000000000
cpu.q[1]=(lane_dup(0u128,64,2)) & 0xffffffffffffffffffffffffffffffffu128;
// 0x111d7aa8 fmov d0, xzr
cpu.q[0]=(0u64) as u128 & 0xffffffffffffffffu128;
// 0x111d7aac mov w8, #1
cpu.x[8]=(((0x1u64))) as u32 as u64;
// 0x111d7ab0 str xzr, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111d7ab4 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x111d7ab8 strh w8, [x0, #0x74]
{let address=(cpu.x[0]).wrapping_add(0x74u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u16>(address.wrapping_add(0),v0 as u16);}
// 0x111d7abc mov v0.d[1], v0.d[0]
cpu.q[0]=(cpu.q[0] & !(0xffffffffffffffffu128 << 64)) | (((((cpu.q[0] >> 0) & 0xffffffffffffffffu128)) as u128 & 0xffffffffffffffffu128) << 64);
// 0x111d7ac0 str w8, [x0, #0x18]
{let address=(cpu.x[0]).wrapping_add(0x18u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111d7ac4 stur q1, [x0, #0x28]
{let address=(cpu.x[0]).wrapping_add(0x28u64);let v0=(cpu.q[1] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x111d7ac8 stur q1, [x0, #0x38]
{let address=(cpu.x[0]).wrapping_add(0x38u64);let v0=(cpu.q[1] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x111d7acc stur q1, [x0, #0x44]
{let address=(cpu.x[0]).wrapping_add(0x44u64);let v0=(cpu.q[1] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x111d7ad0 movi v1.2d, #0xffffffffffffffff
cpu.q[1]=(lane_dup(18446744073709551615u128,64,2)) & 0xffffffffffffffffffffffffffffffffu128;
// 0x111d7ad4 str q0, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x111d7ad8 strb wzr, [x0, #0x76]
{let address=(cpu.x[0]).wrapping_add(0x76u64);let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x111d7adc str wzr, [x0, #0x70]
{let address=(cpu.x[0]).wrapping_add(0x70u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111d7ae0 stur d1, [x0, #0x5c]
{let address=(cpu.x[0]).wrapping_add(0x5cu64);let v0=(cpu.q[1] & 0xffffffffffffffffu128) as u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111d7ae4 ldp w9, w10, [x1]
{let address=(cpu.x[1]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[10]=(v1) as u32 as u64;}
// 0x111d7ae8 str x1, [x0, #0x20]
{let address=(cpu.x[0]).wrapping_add(0x20u64);let v0=cpu.x[1];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111d7aec mul w8, w10, w9
cpu.x[8]=((((cpu.x[10] as u32 as u64))).wrapping_mul(((cpu.x[9] as u32 as u64)))) as u32 as u64;
// 0x111d7af0 stp w9, w10, [x0, #0x54]
{let address=(cpu.x[0]).wrapping_add(0x54u64);let v0=cpu.x[9] as u32 as u64;let v1=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);rt.memory.write::<u32>(address.wrapping_add(4),v1 as u32);}
// 0x111d7af4 sbfiz x11, x8, #2, #0x20
cpu.x[11]=((((((cpu.x[8])) << 32) as i64 >> 32) as u64).wrapping_shl(2)) as u64;
// 0x111d7af8 cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d7afc csinv x20, x11, xzr, ge
cpu.x[20]=(if cpu.condition(10){((cpu.x[11]))}else{!(((0u64)))}) as u64;
// 0x111d7b00 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111d7b04 bl #0x1187d520
cpu.x[30]=0x111d7b08;super::chunk8::f_1187d520(cpu,rt,0x1187d520)?;
pc=0x111d7b08;continue;
}
0x111d7b08=>{
// 0x111d7b08 str x0, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111d7b0c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111d7b10 bl #0x1187d520
cpu.x[30]=0x111d7b14;super::chunk8::f_1187d520(cpu,rt,0x1187d520)?;
pc=0x111d7b14;continue;
}
0x111d7b14=>{
// 0x111d7b14 str x0, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111d7b18 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111d7b1c bl #0x1187d520
cpu.x[30]=0x111d7b20;super::chunk8::f_1187d520(cpu,rt,0x1187d520)?;
pc=0x111d7b20;continue;
}
0x111d7b20=>{
// 0x111d7b20 str x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111d7b24 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111d7b28 bl #0x1187d520
cpu.x[30]=0x111d7b2c;super::chunk8::f_1187d520(cpu,rt,0x1187d520)?;
pc=0x111d7b2c;continue;
}
0x111d7b2c=>{
// 0x111d7b2c str x0, [x19, #0x30]
{let address=(cpu.x[19]).wrapping_add(0x30u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111d7b30 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111d7b34 bl #0x1187d520
cpu.x[30]=0x111d7b38;super::chunk8::f_1187d520(cpu,rt,0x1187d520)?;
pc=0x111d7b38;continue;
}
0x111d7b38=>{
// 0x111d7b38 str x0, [x19, #0x38]
{let address=(cpu.x[19]).wrapping_add(0x38u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111d7b3c mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111d7b40 bl #0x1187d520
cpu.x[30]=0x111d7b44;super::chunk8::f_1187d520(cpu,rt,0x1187d520)?;
pc=0x111d7b44;continue;
}
0x111d7b44=>{
// 0x111d7b44 str wzr, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111d7b48 str x0, [x19, #0x40]
{let address=(cpu.x[19]).wrapping_add(0x40u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111d7b4c ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d7b50 ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x111d7b54 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111d7a98")),}}}
#[inline]
pub(super) unsafe fn f_111d8694(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111d8694=>{
// 0x111d8694 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x111d8698 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111d869c mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x111d86a0 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x111d86a4 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x111d86a8 ldr x0, [x20, #0x28]!
{let address=(cpu.x[20]).wrapping_add(0x28u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;cpu.x[20]=(address) as u64;}
// 0x111d86ac cbz x0, #0x111d86b4
if (cpu.x[0]) == 0 {pc=0x111d86b4;continue;}pc=0x111d86b0;continue;
}
0x111d86b0=>{
// 0x111d86b0 bl #0x1187d360
cpu.x[30]=0x111d86b4;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x111d86b4;continue;
}
0x111d86b4=>{
// 0x111d86b4 ldr x0, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d86b8 str xzr, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111d86bc cbz x0, #0x111d86c4
if (cpu.x[0]) == 0 {pc=0x111d86c4;continue;}pc=0x111d86c0;continue;
}
0x111d86c0=>{
// 0x111d86c0 bl #0x1187d360
cpu.x[30]=0x111d86c4;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x111d86c4;continue;
}
0x111d86c4=>{
// 0x111d86c4 ldr x0, [x19, #0x30]
{let address=(cpu.x[19]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d86c8 str xzr, [x19, #0x48]
{let address=(cpu.x[19]).wrapping_add(0x48u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111d86cc cbz x0, #0x111d86d4
if (cpu.x[0]) == 0 {pc=0x111d86d4;continue;}pc=0x111d86d0;continue;
}
0x111d86d0=>{
// 0x111d86d0 bl #0x1187d360
cpu.x[30]=0x111d86d4;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x111d86d4;continue;
}
0x111d86d4=>{
// 0x111d86d4 ldr x0, [x19, #0x38]
{let address=(cpu.x[19]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d86d8 str xzr, [x19, #0x30]
{let address=(cpu.x[19]).wrapping_add(0x30u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111d86dc cbz x0, #0x111d86e4
if (cpu.x[0]) == 0 {pc=0x111d86e4;continue;}pc=0x111d86e0;continue;
}
0x111d86e0=>{
// 0x111d86e0 bl #0x1187d360
cpu.x[30]=0x111d86e4;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x111d86e4;continue;
}
0x111d86e4=>{
// 0x111d86e4 ldr x0, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d86e8 str xzr, [x19, #0x38]
{let address=(cpu.x[19]).wrapping_add(0x38u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111d86ec cbz x0, #0x111d86f4
if (cpu.x[0]) == 0 {pc=0x111d86f4;continue;}pc=0x111d86f0;continue;
}
0x111d86f0=>{
// 0x111d86f0 bl #0x1187d360
cpu.x[30]=0x111d86f4;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x111d86f4;continue;
}
0x111d86f4=>{
// 0x111d86f4 ldr x0, [x19, #0x40]
{let address=(cpu.x[19]).wrapping_add(0x40u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111d86f8 str xzr, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111d86fc cbz x0, #0x111d8704
if (cpu.x[0]) == 0 {pc=0x111d8704;continue;}pc=0x111d8700;continue;
}
0x111d8700=>{
// 0x111d8700 bl #0x1187d360
cpu.x[30]=0x111d8704;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x111d8704;continue;
}
0x111d8704=>{
// 0x111d8704 movi v0.2d, #0000000000000000
cpu.q[0]=(lane_dup(0u128,64,2)) & 0xffffffffffffffffffffffffffffffffu128;
// 0x111d8708 adrp x8, #0x102b2000
cpu.x[8]=(0x102b2000u64) as u64;
// 0x111d870c str xzr, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111d8710 ldr q1, [x8, #0x520]
{let address=(cpu.x[8]).wrapping_add(0x520u64);let v0=rt.memory.read::<u128>(address.wrapping_add(0));cpu.q[1]=(v0) as u128 & 0xffffffffffffffffffffffffffffffffu128;}
// 0x111d8714 str xzr, [x19, #0x68]
{let address=(cpu.x[19]).wrapping_add(0x68u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111d8718 stur q1, [x19, #0x54]
{let address=(cpu.x[19]).wrapping_add(0x54u64);let v0=(cpu.q[1] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x111d871c stur q0, [x20, #0x1c]
{let address=(cpu.x[20]).wrapping_add(0x1cu64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);}
// 0x111d8720 stp q0, q0, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);let v1=(cpu.q[0] & 0xffffffffffffffffffffffffffffffffu128);rt.memory.write::<u128>(address.wrapping_add(0),v0 as u128);rt.memory.write::<u128>(address.wrapping_add(16),v1 as u128);}
// 0x111d8724 str wzr, [x19, #0x70]
{let address=(cpu.x[19]).wrapping_add(0x70u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111d8728 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111d872c ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x111d8730 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111d8694")),}}}
#[inline]
pub(super) unsafe fn f_111d921c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111d921c=>{
// 0x111d921c cmp w1, #1
{let a=((cpu.x[1] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d9220 b.hi #0x111d9238
if cpu.condition(8){pc=0x111d9238;continue;}pc=0x111d9224;continue;
}
0x111d9224=>{
// 0x111d9224 add x8, x0, w1, uxtw
{let a=((cpu.x[0]));let b=(((cpu.x[1] as u32 as u64) as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x111d9228 ldrb w8, [x8, #0x7c]
{let address=(cpu.x[8]).wrapping_add(0x7cu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111d922c cmp w8, #0
{let a=((cpu.x[8] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111d9230 cset w0, ne
cpu.x[0]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x111d9234 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x111d9238=>{
// 0x111d9238 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x111d923c ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111d921c")),}}}
#[inline]
pub(super) unsafe fn f_111da048(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111da048=>{
// 0x111da048 ldr x9, [x1]
{let address=(cpu.x[1]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111da04c mov x8, x1
cpu.x[8]=(((cpu.x[1]))) as u64;
// 0x111da050 ldr w1, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[1]=(v0) as u32 as u64;}
// 0x111da054 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x111da058 ldr x2, [x9, #0x88]
{let address=(cpu.x[9]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x111da05c br x2
{let target=cpu.x[2];if target>=0x111da048 && target<0x111da060{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111da048")),}}}
#[inline]
pub(super) unsafe fn f_111db8a4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111db8a4=>{
// 0x111db8a4 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x111db8a8 str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111db8ac stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111db8b0 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x111db8b4 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x111db8b8 bl #0x111db9a0
cpu.x[30]=0x111db8bc;super::chunk7::f_111db9a0(cpu,rt,0x111db9a0)?;
pc=0x111db8bc;continue;
}
0x111db8bc=>{
// 0x111db8bc cbz x0, #0x111db954
if (cpu.x[0]) == 0 {pc=0x111db954;continue;}pc=0x111db8c0;continue;
}
0x111db8c0=>{
// 0x111db8c0 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111db8c4 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x111db8c8 ldr x0, [x8, #0x38]
{let address=(cpu.x[8]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111db8cc bl #0x10d76cb4
cpu.x[30]=0x111db8d0;super::dispatch(cpu,rt,0x10d76cb4)?;
// 0x111db8d0 ldr x2, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x111db8d4 ldr w21, [x0, #0xc]
{let address=(cpu.x[0]).wrapping_add(0xcu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[21]=(v0) as u32 as u64;}
// 0x111db8d8 cbnz x2, #0x111db8f0
if (cpu.x[2]) != 0 {pc=0x111db8f0;continue;}pc=0x111db8dc;continue;
}
0x111db8dc=>{
// 0x111db8dc mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x111db8e0 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x111db8e4 bl #0x111dba60
cpu.x[30]=0x111db8e8;super::dispatch(cpu,rt,0x111dba60)?;
// 0x111db8e8 ldr x2, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[2]=(v0) as u64;}
// 0x111db8ec cbz x2, #0x111db900
if (cpu.x[2]) == 0 {pc=0x111db900;continue;}pc=0x111db8f0;continue;
}
0x111db8f0=>{
// 0x111db8f0 ldr x0, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111db8f4 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x111db8f8 bl #0x110ce0c8
cpu.x[30]=0x111db8fc;super::dispatch(cpu,rt,0x110ce0c8)?;
// 0x111db8fc tbz w0, #0, #0x111db928
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x111db928;continue;}pc=0x111db900;continue;
}
0x111db900=>{
// 0x111db900 cmp w21, #0
{let a=((cpu.x[21] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111db904 b.gt #0x111db954
if cpu.condition(12){pc=0x111db954;continue;}pc=0x111db908;continue;
}
0x111db908=>{
// 0x111db908 ldr x0, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111db90c bl #0x110cb308
cpu.x[30]=0x111db910;super::chunk15::f_110cb308(cpu,rt,0x110cb308)?;
// 0x111db910 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111db914 ldr x1, [x8, #0x800]
{let address=(cpu.x[8]).wrapping_add(0x800u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[1]=(v0) as u64;}
// 0x111db918 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111db91c ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x111db920 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x111db924 br x1
{let target=cpu.x[1];if target>=0x111db8a4 && target<0x111db9a0{pc=target;continue;}super::dispatch(cpu,rt,target)?;return Ok(());}
}
0x111db928=>{
// 0x111db928 ldr w8, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111db92c cmp w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111db930 b.lt #0x111db964
if cpu.condition(11){pc=0x111db964;continue;}pc=0x111db934;continue;
}
0x111db934=>{
// 0x111db934 subs w8, w8, #0x32
{let a=((cpu.x[8] as u32 as u64));let b=((0x32u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u32 as u64;cpu.flags(a,b,result,32,true);}
// 0x111db938 str w8, [x19, #0x10]
{let address=(cpu.x[19]).wrapping_add(0x10u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111db93c b.hi #0x111db954
if cpu.condition(8){pc=0x111db954;continue;}pc=0x111db940;continue;
}
0x111db940=>{
// 0x111db940 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x111db944 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111db948 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x111db94c ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x111db950 b #0x111dbf98
super::dispatch(cpu,rt,0x111dbf98)?;return Ok(());
}
0x111db954=>{
// 0x111db954 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111db958 ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x111db95c ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x111db960 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x111db964=>{
// 0x111db964 ldr x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111db968 bl #0x10e9dd0c
cpu.x[30]=0x111db96c;super::dispatch(cpu,rt,0x10e9dd0c)?;
// 0x111db96c cbz w0, #0x111db988
if (cpu.x[0] as u32 as u64) == 0 {pc=0x111db988;continue;}pc=0x111db970;continue;
}
0x111db970=>{
// 0x111db970 mov w1, w0
cpu.x[1]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x111db974 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x111db978 mov x3, x20
cpu.x[3]=(((cpu.x[20]))) as u64;
// 0x111db97c bl #0x111dc04c
cpu.x[30]=0x111db980;super::dispatch(cpu,rt,0x111dc04c)?;
// 0x111db980 tbz w0, #0, #0x111db900
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x111db900;continue;}pc=0x111db984;continue;
}
0x111db984=>{
// 0x111db984 b #0x111db994
pc=0x111db994;continue;
}
0x111db988=>{
// 0x111db988 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x111db98c bl #0x111dc2ac
cpu.x[30]=0x111db990;super::dispatch(cpu,rt,0x111dc2ac)?;
// 0x111db990 tbnz w0, #0, #0x111db900
if ((cpu.x[0] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x111db900;continue;}pc=0x111db994;continue;
}
0x111db994=>{
// 0x111db994 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x111db998 bl #0x111dbf98
cpu.x[30]=0x111db99c;super::dispatch(cpu,rt,0x111dbf98)?;
// 0x111db99c b #0x111db900
pc=0x111db900;continue;
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111db8a4")),}}}
#[inline]
pub(super) unsafe fn f_111dd2e0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111dd2e0=>{
// 0x111dd2e0 stp x29, x30, [sp, #-0x20]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffe0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x111dd2e4 stp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111dd2e8 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x111dd2ec ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111dd2f0 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x111dd2f4 mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x111dd2f8 ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111dd2fc blr x8
{let target=cpu.x[8];cpu.x[30]=0x111dd300;super::dispatch(cpu,rt,target)?;}
pc=0x111dd300;continue;
}
0x111dd300=>{
// 0x111dd300 ldr x8, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111dd304 str w0, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111dd308 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x111dd30c ldr x8, [x8, #0x110]
{let address=(cpu.x[8]).wrapping_add(0x110u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111dd310 blr x8
{let target=cpu.x[8];cpu.x[30]=0x111dd314;super::dispatch(cpu,rt,target)?;}
pc=0x111dd314;continue;
}
0x111dd314=>{
// 0x111dd314 str w0, [x20, #4]
{let address=(cpu.x[20]).wrapping_add(0x4u64);let v0=cpu.x[0] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111dd318 ldp x20, x19, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111dd31c ldp x29, x30, [sp], #0x20
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x20u64)))) as u64;}
// 0x111dd320 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111dd2e0")),}}}
#[inline]
pub(super) unsafe fn f_111e68f0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111e68f0=>{
// 0x111e68f0 sub sp, sp, #0x40
{let a=((cpu.sp));let b=((0x40u64));let result=a.wrapping_sub(b);cpu.sp=(result) as u64;}
// 0x111e68f4 stp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111e68f8 str x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111e68fc stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111e6900 add x29, sp, #0x10
{let a=((cpu.sp));let b=((0x10u64));let result=a.wrapping_add(b);cpu.x[29]=(result) as u64;}
// 0x111e6904 ldrb w8, [x0, #0x80]
{let address=(cpu.x[0]).wrapping_add(0x80u64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111e6908 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x111e690c mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x111e6910 cbz w8, #0x111e6924
if (cpu.x[8] as u32 as u64) == 0 {pc=0x111e6924;continue;}pc=0x111e6914;continue;
}
0x111e6914=>{
// 0x111e6914 ldp w9, w8, [x19, #0xa0]
{let address=(cpu.x[19]).wrapping_add(0xa0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x111e6918 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111e691c b.eq #0x111e6968
if cpu.condition(0){pc=0x111e6968;continue;}pc=0x111e6920;continue;
}
0x111e6920=>{
// 0x111e6920 b #0x111e698c
pc=0x111e698c;continue;
}
0x111e6924=>{
// 0x111e6924 adrp x1, #0x1031c000
cpu.x[1]=(0x1031c000u64) as u64;
// 0x111e6928 add x1, x1, #0xb38
{let a=((cpu.x[1]));let b=((0xb38u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x111e692c mov x0, sp
cpu.x[0]=(((cpu.sp))) as u64;
// 0x111e6930 mov x21, sp
cpu.x[21]=(((cpu.sp))) as u64;
// 0x111e6934 bl #0x1142fde4
cpu.x[30]=0x111e6938;super::chunk6::f_1142fde4(cpu,rt,0x1142fde4)?;
// 0x111e6938 ldr w8, [sp, #4]
{let address=(cpu.sp).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111e693c ldr x9, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111e6940 add x10, x21, #8
{let a=((cpu.x[21]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x111e6944 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x111e6948 cmp w8, #8
{let a=((cpu.x[8] as u32 as u64));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111e694c csel x0, x10, x9, lt
cpu.x[0]=(if cpu.condition(11){((cpu.x[10]))}else{((cpu.x[9]))}) as u64;
// 0x111e6950 bl #0x1128df18
cpu.x[30]=0x111e6954;super::dispatch(cpu,rt,0x1128df18)?;
// 0x111e6954 mov x0, sp
cpu.x[0]=(((cpu.sp))) as u64;
// 0x111e6958 bl #0x1142fcc4
cpu.x[30]=0x111e695c;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
// 0x111e695c ldp w9, w8, [x19, #0xa0]
{let address=(cpu.x[19]).wrapping_add(0xa0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));let v1=rt.memory.read::<u32>(address.wrapping_add(4));cpu.x[9]=(v0) as u32 as u64;cpu.x[8]=(v1) as u32 as u64;}
// 0x111e6960 cmp w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111e6964 b.ne #0x111e698c
if cpu.condition(1){pc=0x111e698c;continue;}pc=0x111e6968;continue;
}
0x111e6968=>{
// 0x111e6968 lsl w9, w8, #1
cpu.x[9]=(((((cpu.x[8] as u32 as u64))) as u32).wrapping_shl(((((0x1u64))) & 31) as u32) as u64) as u32 as u64;
// 0x111e696c mov w10, #5
cpu.x[10]=(((0x5u64))) as u32 as u64;
// 0x111e6970 cmp w9, #0
{let a=((cpu.x[9] as u32 as u64));let b=((0x0u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111e6974 csel w1, w10, w9, eq
cpu.x[1]=(if cpu.condition(0){((cpu.x[10] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x111e6978 cmp w8, w1
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[1] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111e697c b.ge #0x111e698c
if cpu.condition(10){pc=0x111e698c;continue;}pc=0x111e6980;continue;
}
0x111e6980=>{
// 0x111e6980 add x0, x19, #0x98
{let a=((cpu.x[19]));let b=((0x98u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x111e6984 bl #0x1078f4d4
cpu.x[30]=0x111e6988;super::chunk2::f_1078f4d4(cpu,rt,0x1078f4d4)?;
// 0x111e6988 ldr w8, [x19, #0xa4]
{let address=(cpu.x[19]).wrapping_add(0xa4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
pc=0x111e698c;continue;
}
0x111e698c=>{
// 0x111e698c ldr x9, [x19, #0x98]
{let address=(cpu.x[19]).wrapping_add(0x98u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111e6990 add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x111e6994 str w10, [x19, #0xa4]
{let address=(cpu.x[19]).wrapping_add(0xa4u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111e6998 mov w10, #8
cpu.x[10]=(((0x8u64))) as u32 as u64;
// 0x111e699c str w10, [x9, w8, sxtw #2]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111e69a0 ldr x0, [x19, #0x88]
{let address=(cpu.x[19]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111e69a4 cbz x0, #0x111e6a74
if (cpu.x[0]) == 0 {pc=0x111e6a74;continue;}pc=0x111e69a8;continue;
}
0x111e69a8=>{
// 0x111e69a8 bl #0x1128bc0c
cpu.x[30]=0x111e69ac;super::chunk12::f_1128bc0c(cpu,rt,0x1128bc0c)?;
pc=0x111e69ac;continue;
}
0x111e69ac=>{
// 0x111e69ac ldr w8, [x19, #0x90]
{let address=(cpu.x[19]).wrapping_add(0x90u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111e69b0 sub w21, w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[21]=(result) as u32 as u64;}
// 0x111e69b4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x111e69b8 bl #0x1128bc0c
cpu.x[30]=0x111e69bc;super::chunk12::f_1128bc0c(cpu,rt,0x1128bc0c)?;
pc=0x111e69bc;continue;
}
0x111e69bc=>{
// 0x111e69bc cmp w21, w0
{let a=((cpu.x[21] as u32 as u64));let b=((cpu.x[0] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111e69c0 b.eq #0x111e6a1c
if cpu.condition(0){pc=0x111e6a1c;continue;}pc=0x111e69c4;continue;
}
0x111e69c4=>{
// 0x111e69c4 ldr x0, [x19, #0x88]
{let address=(cpu.x[19]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111e69c8 bl #0x1128bc0c
cpu.x[30]=0x111e69cc;super::chunk12::f_1128bc0c(cpu,rt,0x1128bc0c)?;
// 0x111e69cc ldr w8, [x19, #0x90]
{let address=(cpu.x[19]).wrapping_add(0x90u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111e69d0 sub w21, w0, w8
{let a=((cpu.x[0] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.x[21]=(result) as u32 as u64;}
// 0x111e69d4 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x111e69d8 bl #0x1128bc0c
cpu.x[30]=0x111e69dc;super::chunk12::f_1128bc0c(cpu,rt,0x1128bc0c)?;
// 0x111e69dc mov w2, w0
cpu.x[2]=(((cpu.x[0] as u32 as u64))) as u32 as u64;
// 0x111e69e0 adrp x0, #0x102ee000
cpu.x[0]=(0x102ee000u64) as u64;
// 0x111e69e4 add x0, x0, #0xdf4
{let a=((cpu.x[0]));let b=((0xdf4u64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x111e69e8 mov x8, sp
cpu.x[8]=(((cpu.sp))) as u64;
// 0x111e69ec mov w1, w21
cpu.x[1]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x111e69f0 mov x21, sp
cpu.x[21]=(((cpu.sp))) as u64;
// 0x111e69f4 bl #0x114328cc
cpu.x[30]=0x111e69f8;super::chunk0::f_114328cc(cpu,rt,0x114328cc)?;
// 0x111e69f8 ldr w8, [sp, #4]
{let address=(cpu.sp).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111e69fc ldr x9, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111e6a00 add x10, x21, #8
{let a=((cpu.x[21]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x111e6a04 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x111e6a08 cmp w8, #8
{let a=((cpu.x[8] as u32 as u64));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111e6a0c csel x0, x10, x9, lt
cpu.x[0]=(if cpu.condition(11){((cpu.x[10]))}else{((cpu.x[9]))}) as u64;
// 0x111e6a10 bl #0x1128df18
cpu.x[30]=0x111e6a14;super::dispatch(cpu,rt,0x1128df18)?;
// 0x111e6a14 mov x0, sp
cpu.x[0]=(((cpu.sp))) as u64;
// 0x111e6a18 bl #0x1142fcc4
cpu.x[30]=0x111e6a1c;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
pc=0x111e6a1c;continue;
}
0x111e6a1c=>{
// 0x111e6a1c ldr x0, [x19, #0x88]
{let address=(cpu.x[19]).wrapping_add(0x88u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111e6a20 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111e6a24 ldr x8, [x8, #0x120]
{let address=(cpu.x[8]).wrapping_add(0x120u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111e6a28 blr x8
{let target=cpu.x[8];cpu.x[30]=0x111e6a2c;super::dispatch(cpu,rt,target)?;}
pc=0x111e6a2c;continue;
}
0x111e6a2c=>{
// 0x111e6a2c subs x3, x20, x0
{let a=((cpu.x[20]));let b=((cpu.x[0]));let result=a.wrapping_sub(b);cpu.x[3]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x111e6a30 b.eq #0x111e6a74
if cpu.condition(0){pc=0x111e6a74;continue;}pc=0x111e6a34;continue;
}
0x111e6a34=>{
// 0x111e6a34 mov x2, x0
cpu.x[2]=(((cpu.x[0]))) as u64;
// 0x111e6a38 adrp x0, #0x102f7000
cpu.x[0]=(0x102f7000u64) as u64;
// 0x111e6a3c add x0, x0, #0x31f
{let a=((cpu.x[0]));let b=((0x31fu64));let result=a.wrapping_add(b);cpu.x[0]=(result) as u64;}
// 0x111e6a40 mov x8, sp
cpu.x[8]=(((cpu.sp))) as u64;
// 0x111e6a44 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x111e6a48 mov x21, sp
cpu.x[21]=(((cpu.sp))) as u64;
// 0x111e6a4c bl #0x114328cc
cpu.x[30]=0x111e6a50;super::chunk0::f_114328cc(cpu,rt,0x114328cc)?;
// 0x111e6a50 ldr w8, [sp, #4]
{let address=(cpu.sp).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111e6a54 ldr x9, [sp, #8]
{let address=(cpu.sp).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111e6a58 add x10, x21, #8
{let a=((cpu.x[21]));let b=((0x8u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u64;}
// 0x111e6a5c mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x111e6a60 cmp w8, #8
{let a=((cpu.x[8] as u32 as u64));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111e6a64 csel x0, x10, x9, lt
cpu.x[0]=(if cpu.condition(11){((cpu.x[10]))}else{((cpu.x[9]))}) as u64;
// 0x111e6a68 bl #0x1128df18
cpu.x[30]=0x111e6a6c;super::dispatch(cpu,rt,0x1128df18)?;
// 0x111e6a6c mov x0, sp
cpu.x[0]=(((cpu.sp))) as u64;
// 0x111e6a70 bl #0x1142fcc4
cpu.x[30]=0x111e6a74;super::chunk4::f_1142fcc4(cpu,rt,0x1142fcc4)?;
pc=0x111e6a74;continue;
}
0x111e6a74=>{
// 0x111e6a74 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x111e6a78 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x111e6a7c bl #0x1128ca38
cpu.x[30]=0x111e6a80;super::chunk5::f_1128ca38(cpu,rt,0x1128ca38)?;
pc=0x111e6a80;continue;
}
0x111e6a80=>{
// 0x111e6a80 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111e6a84 ldr x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x111e6a88 ldp x29, x30, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;}
// 0x111e6a8c add sp, sp, #0x40
{let a=((cpu.sp));let b=((0x40u64));let result=a.wrapping_add(b);cpu.sp=(result) as u64;}
// 0x111e6a90 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111e68f0")),}}}
#[inline]
pub(super) unsafe fn f_111fb574(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x111fb574=>{
// 0x111fb574 stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x111fb578 str x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111fb57c stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111fb580 stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111fb584 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x111fb588 ldr x21, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x111fb58c ldrsw x22, [x1]
{let address=(cpu.x[1]).wrapping_add(0x0u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[22]=(v0) as u64;}
// 0x111fb590 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x111fb594 mov x20, x3
cpu.x[20]=(((cpu.x[3]))) as u64;
// 0x111fb598 cbz x21, #0x111fb5c0
if (cpu.x[21]) == 0 {pc=0x111fb5c0;continue;}pc=0x111fb59c;continue;
}
0x111fb59c=>{
// 0x111fb59c fmov d0, x21
cpu.q[0]=(cpu.x[21]) as u128 & 0xffffffffffffffffu128;
// 0x111fb5a0 cnt v0.8b, v0.8b
cpu.q[0]=(lane_count((cpu.q[0] & 0xffffffffffffffffu128),8,8)) & 0xffffffffffffffffu128;
// 0x111fb5a4 uaddlv h0, v0.8b
cpu.q[0]=(lane_reduce((cpu.q[0] & 0xffffffffffffffffu128),8,8,"uaddlv")) & 0xffffu128;
// 0x111fb5a8 fmov w8, s0
cpu.x[8]=((cpu.q[0] & 0xffffffffu128) as u64) as u32 as u64;
// 0x111fb5ac cmp x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x111fb5b0 b.hi #0x111fb5c4
if cpu.condition(8){pc=0x111fb5c4;continue;}pc=0x111fb5b4;continue;
}
0x111fb5b4=>{
// 0x111fb5b4 sub x9, x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u64;}
// 0x111fb5b8 and x23, x9, x22
{let a=((cpu.x[9]));let b=((cpu.x[22]));let result=a & b;cpu.x[23]=(result) as u64;}
// 0x111fb5bc b #0x111fb5d8
pc=0x111fb5d8;continue;
}
0x111fb5c0=>{
// 0x111fb5c0 b #0x111fb680
pc=0x111fb680;continue;
}
0x111fb5c4=>{
// 0x111fb5c4 cmp x21, x22
{let a=((cpu.x[21]));let b=((cpu.x[22]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x111fb5c8 mov x23, x22
cpu.x[23]=(((cpu.x[22]))) as u64;
// 0x111fb5cc b.hi #0x111fb5d8
if cpu.condition(8){pc=0x111fb5d8;continue;}pc=0x111fb5d0;continue;
}
0x111fb5d0=>{
// 0x111fb5d0 udiv x9, x22, x21
{let a=((cpu.x[22])) as u64;let b=((cpu.x[21])) as u64;cpu.x[9]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u64;}
// 0x111fb5d4 msub x23, x9, x21, x22
cpu.x[23]=((((cpu.x[22]))).wrapping_sub((((cpu.x[9]))).wrapping_mul(((cpu.x[21]))))) as u64;
pc=0x111fb5d8;continue;
}
0x111fb5d8=>{
// 0x111fb5d8 ldr x9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111fb5dc ldr x9, [x9, x23, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[23])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111fb5e0 cbz x9, #0x111fb680
if (cpu.x[9]) == 0 {pc=0x111fb680;continue;}pc=0x111fb5e4;continue;
}
0x111fb5e4=>{
// 0x111fb5e4 ldr x0, [x9]
{let address=(cpu.x[9]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111fb5e8 cbz x0, #0x111fb680
if (cpu.x[0]) == 0 {pc=0x111fb680;continue;}pc=0x111fb5ec;continue;
}
0x111fb5ec=>{
// 0x111fb5ec cmp x8, #2
{let a=((cpu.x[8]));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x111fb5f0 b.hs #0x111fb60c
if cpu.condition(2){pc=0x111fb60c;continue;}pc=0x111fb5f4;continue;
}
0x111fb5f4=>{
// 0x111fb5f4 sub x8, x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u64;}
// 0x111fb5f8 b #0x111fb650
pc=0x111fb650;continue;
}
0x111fb5fc=>{
// 0x111fb5fc cmp x8, x23
{let a=((cpu.x[8]));let b=((cpu.x[23]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x111fb600 b.ne #0x111fb680
if cpu.condition(1){pc=0x111fb680;continue;}pc=0x111fb604;continue;
}
0x111fb604=>{
// 0x111fb604 ldr x0, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111fb608 cbz x0, #0x111fb680
if (cpu.x[0]) == 0 {pc=0x111fb680;continue;}pc=0x111fb60c;continue;
}
0x111fb60c=>{
// 0x111fb60c ldr x8, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111fb610 cmp x8, x22
{let a=((cpu.x[8]));let b=((cpu.x[22]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x111fb614 b.ne #0x111fb628
if cpu.condition(1){pc=0x111fb628;continue;}pc=0x111fb618;continue;
}
0x111fb618=>{
// 0x111fb618 ldr w8, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x111fb61c cmp w8, w22
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[22] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111fb620 b.ne #0x111fb604
if cpu.condition(1){pc=0x111fb604;continue;}pc=0x111fb624;continue;
}
0x111fb624=>{
// 0x111fb624 b #0x111fb668
pc=0x111fb668;continue;
}
0x111fb628=>{
// 0x111fb628 cmp x8, x21
{let a=((cpu.x[8]));let b=((cpu.x[21]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x111fb62c b.lo #0x111fb5fc
if cpu.condition(3){pc=0x111fb5fc;continue;}pc=0x111fb630;continue;
}
0x111fb630=>{
// 0x111fb630 udiv x9, x8, x21
{let a=((cpu.x[8])) as u64;let b=((cpu.x[21])) as u64;cpu.x[9]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u64;}
// 0x111fb634 msub x8, x9, x21, x8
cpu.x[8]=((((cpu.x[8]))).wrapping_sub((((cpu.x[9]))).wrapping_mul(((cpu.x[21]))))) as u64;
// 0x111fb638 b #0x111fb5fc
pc=0x111fb5fc;continue;
}
0x111fb63c=>{
// 0x111fb63c and x9, x9, x8
{let a=((cpu.x[9]));let b=((cpu.x[8]));let result=a & b;cpu.x[9]=(result) as u64;}
// 0x111fb640 cmp x9, x23
{let a=((cpu.x[9]));let b=((cpu.x[23]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x111fb644 b.ne #0x111fb680
if cpu.condition(1){pc=0x111fb680;continue;}pc=0x111fb648;continue;
}
0x111fb648=>{
// 0x111fb648 ldr x0, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x111fb64c cbz x0, #0x111fb680
if (cpu.x[0]) == 0 {pc=0x111fb680;continue;}pc=0x111fb650;continue;
}
0x111fb650=>{
// 0x111fb650 ldr x9, [x0, #8]
{let address=(cpu.x[0]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111fb654 cmp x9, x22
{let a=((cpu.x[9]));let b=((cpu.x[22]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x111fb658 b.ne #0x111fb63c
if cpu.condition(1){pc=0x111fb63c;continue;}pc=0x111fb65c;continue;
}
0x111fb65c=>{
// 0x111fb65c ldr w9, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x111fb660 cmp w9, w22
{let a=((cpu.x[9] as u32 as u64));let b=((cpu.x[22] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x111fb664 b.ne #0x111fb648
if cpu.condition(1){pc=0x111fb648;continue;}pc=0x111fb668;continue;
}
0x111fb668=>{
// 0x111fb668 mov x1, xzr
cpu.x[1]=(((0u64))) as u64;
// 0x111fb66c ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111fb670 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111fb674 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111fb678 ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x111fb67c ret
cpu.pc=cpu.x[30];return Ok(());
}
0x111fb680=>{
// 0x111fb680 mov w0, #0x20
cpu.x[0]=(((0x20u64))) as u32 as u64;
// 0x111fb684 bl #0x1187d2b0
cpu.x[30]=0x111fb688;super::chunk15::f_1187d2b0(cpu,rt,0x1187d2b0)?;
pc=0x111fb688;continue;
}
0x111fb688=>{
// 0x111fb688 ldr x8, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111fb68c ldr x9, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111fb690 ldr s1, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.q[1]=(v0) as u128 & 0xffffffffu128;}
// 0x111fb694 stp xzr, x22, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=0u64;let v1=cpu.x[22];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x111fb698 add x8, x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x111fb69c ldr w9, [x9]
{let address=(cpu.x[9]).wrapping_add(0x0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x111fb6a0 str xzr, [x0, #0x18]
{let address=(cpu.x[0]).wrapping_add(0x18u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111fb6a4 ucvtf s0, x8
cpu.q[0]=(((cpu.x[8]) as u64 as f32).to_bits()) as u128 & 0xffffffffu128;
// 0x111fb6a8 str w9, [x0, #0x10]
{let address=(cpu.x[0]).wrapping_add(0x10u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x111fb6ac cbz x21, #0x111fb6d8
if (cpu.x[21]) == 0 {pc=0x111fb6d8;continue;}pc=0x111fb6b0;continue;
}
0x111fb6b0=>{
// 0x111fb6b0 ucvtf s2, x21
cpu.q[2]=(((cpu.x[21]) as u64 as f32).to_bits()) as u128 & 0xffffffffu128;
// 0x111fb6b4 fmul s2, s1, s2
fp_domain(cpu,&[f32::from_bits(((cpu.q[1] & 0xffffffffu128) as u64) as u32) as f64,f32::from_bits(((cpu.q[2] & 0xffffffffu128) as u64) as u32) as f64])?;if f32::from_bits(((cpu.q[2] & 0xffffffffu128) as u64) as u32).is_subnormal(){return Err("subnormal f32 operand requires architectural handling".into());}if f32::from_bits(((cpu.q[1] & 0xffffffffu128) as u64) as u32).is_subnormal(){return Err("subnormal f32 operand requires architectural handling".into());}cpu.q[2]=((f32::from_bits(((cpu.q[1] & 0xffffffffu128) as u64) as u32)*f32::from_bits(((cpu.q[2] & 0xffffffffu128) as u64) as u32)).to_bits()) as u128 & 0xffffffffu128;fp_domain(cpu,&[f32::from_bits(((cpu.q[2] & 0xffffffffu128) as u64) as u32) as f64])?;
// 0x111fb6b8 fcmp s2, s0
fp_domain(cpu,&[f32::from_bits(((cpu.q[2] & 0xffffffffu128) as u64) as u32) as f64,f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32) as f64])?;if f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32).is_subnormal(){return Err("subnormal f32 operand requires architectural handling".into());}if f32::from_bits(((cpu.q[2] & 0xffffffffu128) as u64) as u32).is_subnormal(){return Err("subnormal f32 operand requires architectural handling".into());}{let a=f32::from_bits(((cpu.q[2] & 0xffffffffu128) as u64) as u32);let b=f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32);cpu.nzcv=if a.is_nan()||b.is_nan(){0x30000000}else if a==b{0x60000000}else if a<b{0x80000000}else{0x20000000};}
// 0x111fb6bc b.mi #0x111fb6d8
if cpu.condition(4){pc=0x111fb6d8;continue;}pc=0x111fb6c0;continue;
}
0x111fb6c0=>{
// 0x111fb6c0 ldr x9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111fb6c4 ldr x8, [x9, x23, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[23])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111fb6c8 cbz x8, #0x111fb80c
if (cpu.x[8]) == 0 {pc=0x111fb80c;continue;}pc=0x111fb6cc;continue;
}
0x111fb6cc=>{
// 0x111fb6cc ldr x9, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111fb6d0 str x9, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=cpu.x[9];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111fb6d4 b #0x111fb88c
pc=0x111fb88c;continue;
}
0x111fb6d8=>{
// 0x111fb6d8 lsl x8, x21, #1
cpu.x[8]=(((((cpu.x[21]))) as u64).wrapping_shl(((((0x1u64))) & 63) as u32) as u64) as u64;
// 0x111fb6dc cmp x21, #3
{let a=((cpu.x[21]));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x111fb6e0 b.lo #0x111fb6f4
if cpu.condition(3){pc=0x111fb6f4;continue;}pc=0x111fb6e4;continue;
}
0x111fb6e4=>{
// 0x111fb6e4 sub x9, x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u64;}
// 0x111fb6e8 tst x21, x9
{let a=((cpu.x[21]));let b=((cpu.x[9]));let result=a & b;cpu.logical_flags(result,64);}
// 0x111fb6ec cset w9, ne
cpu.x[9]=((cpu.condition(1)) as u64) as u32 as u64;
// 0x111fb6f0 b #0x111fb6f8
pc=0x111fb6f8;continue;
}
0x111fb6f4=>{
// 0x111fb6f4 mov w9, #1
cpu.x[9]=(((0x1u64))) as u32 as u64;
pc=0x111fb6f8;continue;
}
0x111fb6f8=>{
// 0x111fb6f8 fdiv s0, s0, s1
fp_domain(cpu,&[f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32) as f64,f32::from_bits(((cpu.q[1] & 0xffffffffu128) as u64) as u32) as f64])?;if f32::from_bits(((cpu.q[1] & 0xffffffffu128) as u64) as u32).is_subnormal(){return Err("subnormal f32 operand requires architectural handling".into());}if f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32).is_subnormal(){return Err("subnormal f32 operand requires architectural handling".into());}cpu.q[0]=((f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32)/f32::from_bits(((cpu.q[1] & 0xffffffffu128) as u64) as u32)).to_bits()) as u128 & 0xffffffffu128;fp_domain(cpu,&[f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32) as f64])?;
// 0x111fb6fc orr x8, x9, x8
{let a=((cpu.x[9]));let b=((cpu.x[8]));let result=a | b;cpu.x[8]=(result) as u64;}
// 0x111fb700 fcvtpu x10, s0
fp_domain(cpu,&[f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32) as f64])?;if f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32).is_subnormal(){return Err("subnormal f32 operand requires architectural handling".into());}cpu.x[10]=((f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32).ceil() as u64) as u64) as u64;
// 0x111fb704 cmp x8, x10
{let a=((cpu.x[8]));let b=((cpu.x[10]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x111fb708 csel x1, x8, x10, hi
cpu.x[1]=(if cpu.condition(8){((cpu.x[8]))}else{((cpu.x[10]))}) as u64;
// 0x111fb70c subs x8, x1, #1
{let a=((cpu.x[1]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u64;cpu.flags(a,b,result,64,true);}
// 0x111fb710 b.ne #0x111fb71c
if cpu.condition(1){pc=0x111fb71c;continue;}pc=0x111fb714;continue;
}
0x111fb714=>{
// 0x111fb714 mov w1, #2
cpu.x[1]=(((0x2u64))) as u32 as u64;
// 0x111fb718 b #0x111fb73c
pc=0x111fb73c;continue;
}
0x111fb71c=>{
// 0x111fb71c tst x1, x8
{let a=((cpu.x[1]));let b=((cpu.x[8]));let result=a & b;cpu.logical_flags(result,64);}
// 0x111fb720 b.eq #0x111fb73c
if cpu.condition(0){pc=0x111fb73c;continue;}pc=0x111fb724;continue;
}
0x111fb724=>{
// 0x111fb724 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x111fb728 mov x0, x1
cpu.x[0]=(((cpu.x[1]))) as u64;
// 0x111fb72c bl #0x1187d3f0
cpu.x[30]=0x111fb730;super::dispatch(cpu,rt,0x1187d3f0)?;
pc=0x111fb730;continue;
}
0x111fb730=>{
// 0x111fb730 ldr x21, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x111fb734 mov x1, x0
cpu.x[1]=(((cpu.x[0]))) as u64;
// 0x111fb738 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
pc=0x111fb73c;continue;
}
0x111fb73c=>{
// 0x111fb73c cmp x1, x21
{let a=((cpu.x[1]));let b=((cpu.x[21]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x111fb740 b.ls #0x111fb758
if cpu.condition(9){pc=0x111fb758;continue;}pc=0x111fb744;continue;
}
0x111fb744=>{
// 0x111fb744 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x111fb748 mov x0, x19
cpu.x[0]=(((cpu.x[19]))) as u64;
// 0x111fb74c bl #0x111fb8b4
cpu.x[30]=0x111fb750;super::chunk0::f_111fb8b4(cpu,rt,0x111fb8b4)?;
pc=0x111fb750;continue;
}
0x111fb750=>{
// 0x111fb750 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111fb754 b #0x111fb7ec
pc=0x111fb7ec;continue;
}
0x111fb758=>{
// 0x111fb758 b.hs #0x111fb7ec
if cpu.condition(2){pc=0x111fb7ec;continue;}pc=0x111fb75c;continue;
}
0x111fb75c=>{
// 0x111fb75c ldr x8, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111fb760 ldr s1, [x19, #0x20]
{let address=(cpu.x[19]).wrapping_add(0x20u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.q[1]=(v0) as u128 & 0xffffffffu128;}
// 0x111fb764 cmp x21, #3
{let a=((cpu.x[21]));let b=((0x3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x111fb768 ucvtf s0, x8
cpu.q[0]=(((cpu.x[8]) as u64 as f32).to_bits()) as u128 & 0xffffffffu128;
// 0x111fb76c fdiv s0, s0, s1
fp_domain(cpu,&[f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32) as f64,f32::from_bits(((cpu.q[1] & 0xffffffffu128) as u64) as u32) as f64])?;if f32::from_bits(((cpu.q[1] & 0xffffffffu128) as u64) as u32).is_subnormal(){return Err("subnormal f32 operand requires architectural handling".into());}if f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32).is_subnormal(){return Err("subnormal f32 operand requires architectural handling".into());}cpu.q[0]=((f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32)/f32::from_bits(((cpu.q[1] & 0xffffffffu128) as u64) as u32)).to_bits()) as u128 & 0xffffffffu128;fp_domain(cpu,&[f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32) as f64])?;
// 0x111fb770 fcvtpu x8, s0
fp_domain(cpu,&[f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32) as f64])?;if f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32).is_subnormal(){return Err("subnormal f32 operand requires architectural handling".into());}cpu.x[8]=((f32::from_bits(((cpu.q[0] & 0xffffffffu128) as u64) as u32).ceil() as u64) as u64) as u64;
// 0x111fb774 b.lo #0x111fb7c0
if cpu.condition(3){pc=0x111fb7c0;continue;}pc=0x111fb778;continue;
}
0x111fb778=>{
// 0x111fb778 fmov d0, x21
cpu.q[0]=(cpu.x[21]) as u128 & 0xffffffffffffffffu128;
// 0x111fb77c cnt v0.8b, v0.8b
cpu.q[0]=(lane_count((cpu.q[0] & 0xffffffffffffffffu128),8,8)) & 0xffffffffffffffffu128;
// 0x111fb780 uaddlv h0, v0.8b
cpu.q[0]=(lane_reduce((cpu.q[0] & 0xffffffffffffffffu128),8,8,"uaddlv")) & 0xffffu128;
// 0x111fb784 fmov w9, s0
cpu.x[9]=((cpu.q[0] & 0xffffffffu128) as u64) as u32 as u64;
// 0x111fb788 cmp x9, #1
{let a=((cpu.x[9]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x111fb78c b.hi #0x111fb7c0
if cpu.condition(8){pc=0x111fb7c0;continue;}pc=0x111fb790;continue;
}
0x111fb790=>{
// 0x111fb790 sub x9, x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u64;}
// 0x111fb794 mov w10, #1
cpu.x[10]=(((0x1u64))) as u32 as u64;
// 0x111fb798 cmp x8, #2
{let a=((cpu.x[8]));let b=((0x2u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x111fb79c clz x9, x9
cpu.x[9]=((((cpu.x[9])) as u64).leading_zeros() as u64) as u64;
// 0x111fb7a0 neg x9, x9
cpu.x[9]=(0u64.wrapping_sub(((cpu.x[9])))) as u64;
// 0x111fb7a4 lsl x9, x10, x9
cpu.x[9]=(((((cpu.x[10]))) as u64).wrapping_shl(((((cpu.x[9]))) & 63) as u32) as u64) as u64;
// 0x111fb7a8 csel x8, x8, x9, lo
cpu.x[8]=(if cpu.condition(3){((cpu.x[8]))}else{((cpu.x[9]))}) as u64;
// 0x111fb7ac cmp x1, x8
{let a=((cpu.x[1]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x111fb7b0 csel x1, x1, x8, hi
cpu.x[1]=(if cpu.condition(8){((cpu.x[1]))}else{((cpu.x[8]))}) as u64;
// 0x111fb7b4 cmp x1, x21
{let a=((cpu.x[1]));let b=((cpu.x[21]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x111fb7b8 b.lo #0x111fb744
if cpu.condition(3){pc=0x111fb744;continue;}pc=0x111fb7bc;continue;
}
0x111fb7bc=>{
// 0x111fb7bc b #0x111fb7ec
pc=0x111fb7ec;continue;
}
0x111fb7c0=>{
// 0x111fb7c0 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x111fb7c4 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x111fb7c8 mov x23, x1
cpu.x[23]=(((cpu.x[1]))) as u64;
// 0x111fb7cc bl #0x1187d3f0
cpu.x[30]=0x111fb7d0;super::dispatch(cpu,rt,0x1187d3f0)?;
// 0x111fb7d0 mov x1, x23
cpu.x[1]=(((cpu.x[23]))) as u64;
// 0x111fb7d4 mov x8, x0
cpu.x[8]=(((cpu.x[0]))) as u64;
// 0x111fb7d8 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x111fb7dc cmp x23, x8
{let a=((cpu.x[23]));let b=((cpu.x[8]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x111fb7e0 csel x1, x23, x8, hi
cpu.x[1]=(if cpu.condition(8){((cpu.x[23]))}else{((cpu.x[8]))}) as u64;
// 0x111fb7e4 cmp x1, x21
{let a=((cpu.x[1]));let b=((cpu.x[21]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x111fb7e8 b.lo #0x111fb744
if cpu.condition(3){pc=0x111fb744;continue;}pc=0x111fb7ec;continue;
}
0x111fb7ec=>{
// 0x111fb7ec ldr x21, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x111fb7f0 sub x8, x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u64;}
// 0x111fb7f4 tst x21, x8
{let a=((cpu.x[21]));let b=((cpu.x[8]));let result=a & b;cpu.logical_flags(result,64);}
// 0x111fb7f8 b.ne #0x111fb840
if cpu.condition(1){pc=0x111fb840;continue;}pc=0x111fb7fc;continue;
}
0x111fb7fc=>{
// 0x111fb7fc and x23, x8, x22
{let a=((cpu.x[8]));let b=((cpu.x[22]));let result=a & b;cpu.x[23]=(result) as u64;}
pc=0x111fb800;continue;
}
0x111fb800=>{
// 0x111fb800 ldr x9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111fb804 ldr x8, [x9, x23, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[23])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111fb808 cbnz x8, #0x111fb6cc
if (cpu.x[8]) != 0 {pc=0x111fb6cc;continue;}pc=0x111fb80c;continue;
}
0x111fb80c=>{
// 0x111fb80c mov x8, x19
cpu.x[8]=(((cpu.x[19]))) as u64;
// 0x111fb810 ldr x10, [x8, #0x10]!
{let address=(cpu.x[8]).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[10]=(v0) as u64;cpu.x[8]=(address) as u64;}
// 0x111fb814 str x0, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111fb818 str x10, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=cpu.x[10];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111fb81c str x8, [x9, x23, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[23])) as u64).wrapping_shl(3) as u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111fb820 ldr x8, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111fb824 cbz x8, #0x111fb890
if (cpu.x[8]) == 0 {pc=0x111fb890;continue;}pc=0x111fb828;continue;
}
0x111fb828=>{
// 0x111fb828 sub x9, x21, #1
{let a=((cpu.x[21]));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.x[9]=(result) as u64;}
// 0x111fb82c ldr x8, [x8, #8]
{let address=(cpu.x[8]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111fb830 tst x21, x9
{let a=((cpu.x[21]));let b=((cpu.x[9]));let result=a & b;cpu.logical_flags(result,64);}
// 0x111fb834 b.ne #0x111fb874
if cpu.condition(1){pc=0x111fb874;continue;}pc=0x111fb838;continue;
}
0x111fb838=>{
// 0x111fb838 and x8, x8, x9
{let a=((cpu.x[8]));let b=((cpu.x[9]));let result=a & b;cpu.x[8]=(result) as u64;}
// 0x111fb83c b #0x111fb884
pc=0x111fb884;continue;
}
0x111fb840=>{
// 0x111fb840 cmp x21, x22
{let a=((cpu.x[21]));let b=((cpu.x[22]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x111fb844 b.ls #0x111fb85c
if cpu.condition(9){pc=0x111fb85c;continue;}pc=0x111fb848;continue;
}
0x111fb848=>{
// 0x111fb848 mov x23, x22
cpu.x[23]=(((cpu.x[22]))) as u64;
// 0x111fb84c ldr x9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111fb850 ldr x8, [x9, x22, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[22])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111fb854 cbnz x8, #0x111fb6cc
if (cpu.x[8]) != 0 {pc=0x111fb6cc;continue;}pc=0x111fb858;continue;
}
0x111fb858=>{
// 0x111fb858 b #0x111fb80c
pc=0x111fb80c;continue;
}
0x111fb85c=>{
// 0x111fb85c udiv x8, x22, x21
{let a=((cpu.x[22])) as u64;let b=((cpu.x[21])) as u64;cpu.x[8]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u64;}
// 0x111fb860 msub x23, x8, x21, x22
cpu.x[23]=((((cpu.x[22]))).wrapping_sub((((cpu.x[8]))).wrapping_mul(((cpu.x[21]))))) as u64;
// 0x111fb864 ldr x9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111fb868 ldr x8, [x9, x23, lsl #3]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[23])) as u64).wrapping_shl(3) as u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111fb86c cbnz x8, #0x111fb6cc
if (cpu.x[8]) != 0 {pc=0x111fb6cc;continue;}pc=0x111fb870;continue;
}
0x111fb870=>{
// 0x111fb870 b #0x111fb80c
pc=0x111fb80c;continue;
}
0x111fb874=>{
// 0x111fb874 cmp x8, x21
{let a=((cpu.x[8]));let b=((cpu.x[21]));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x111fb878 b.lo #0x111fb884
if cpu.condition(3){pc=0x111fb884;continue;}pc=0x111fb87c;continue;
}
0x111fb87c=>{
// 0x111fb87c udiv x9, x8, x21
{let a=((cpu.x[8])) as u64;let b=((cpu.x[21])) as u64;cpu.x[9]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u64;}
// 0x111fb880 msub x8, x9, x21, x8
cpu.x[8]=((((cpu.x[8]))).wrapping_sub((((cpu.x[9]))).wrapping_mul(((cpu.x[21]))))) as u64;
pc=0x111fb884;continue;
}
0x111fb884=>{
// 0x111fb884 ldr x9, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x111fb888 add x8, x9, x8, lsl #3
{let a=((cpu.x[9]));let b=(((cpu.x[8])) as u64).wrapping_shl(3) as u64;let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
pc=0x111fb88c;continue;
}
0x111fb88c=>{
// 0x111fb88c str x0, [x8]
{let address=(cpu.x[8]).wrapping_add(0x0u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x111fb890;continue;
}
0x111fb890=>{
// 0x111fb890 ldr x8, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x111fb894 mov w1, #1
cpu.x[1]=(((0x1u64))) as u32 as u64;
// 0x111fb898 add x8, x8, #1
{let a=((cpu.x[8]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u64;}
// 0x111fb89c str x8, [x19, #0x18]
{let address=(cpu.x[19]).wrapping_add(0x18u64);let v0=cpu.x[8];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x111fb8a0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x111fb8a4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x111fb8a8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x111fb8ac ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x111fb8b0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x111fb574")),}}}
#[inline]
pub(super) unsafe fn f_1128c2f4(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x1128c2f4=>{
// 0x1128c2f4 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x1128c2f8 stp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x1128c2fc stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x1128c300 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x1128c304 mov w20, w1
cpu.x[20]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x1128c308 mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x1128c30c bl #0x1128dc00
cpu.x[30]=0x1128c310;super::chunk3::f_1128dc00(cpu,rt,0x1128dc00)?;
pc=0x1128c310;continue;
}
0x1128c310=>{
// 0x1128c310 ldr w8, [x19, #0x1c]
{let address=(cpu.x[19]).wrapping_add(0x1cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x1128c314 cbz w8, #0x1128c360
if (cpu.x[8] as u32 as u64) == 0 {pc=0x1128c360;continue;}pc=0x1128c318;continue;
}
0x1128c318=>{
// 0x1128c318 tbz w20, #0, #0x1128c340
if ((cpu.x[20] as u32 as u64) & (1u64 << 0)) == 0 {pc=0x1128c340;continue;}pc=0x1128c31c;continue;
}
0x1128c31c=>{
// 0x1128c31c ldr x9, [x19, #0x38]
{let address=(cpu.x[19]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x1128c320 ldrsw x10, [x19, #0x14]
{let address=(cpu.x[19]).wrapping_add(0x14u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[10]=(v0) as u64;}
// 0x1128c324 mov w11, #1
cpu.x[11]=(((0x1u64))) as u32 as u64;
// 0x1128c328 lsl w8, w11, w8
cpu.x[8]=(((((cpu.x[11] as u32 as u64))) as u32).wrapping_shl(((((cpu.x[8] as u32 as u64))) & 31) as u32) as u64) as u32 as u64;
// 0x1128c32c add x9, x9, x10
{let a=((cpu.x[9]));let b=((cpu.x[10]));let result=a.wrapping_add(b);cpu.x[9]=(result) as u64;}
// 0x1128c330 ldurb w10, [x9, #-1]
{let address=(cpu.x[9]).wrapping_add(0xffffffffffffffffu64);let v0=rt.memory.read::<u8>(address.wrapping_add(0));cpu.x[10]=(v0) as u32 as u64;}
// 0x1128c334 orr w8, w10, w8
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a | b;cpu.x[8]=(result) as u32 as u64;}
// 0x1128c338 sturb w8, [x9, #-1]
{let address=(cpu.x[9]).wrapping_add(0xffffffffffffffffu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x1128c33c ldr w8, [x19, #0x1c]
{let address=(cpu.x[19]).wrapping_add(0x1cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
pc=0x1128c340;continue;
}
0x1128c340=>{
// 0x1128c340 add w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x1128c344 and w0, w20, #1
{let a=((cpu.x[20] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.x[0]=(result) as u32 as u64;}
// 0x1128c348 and w8, w8, #7
{let a=((cpu.x[8] as u32 as u64));let b=((0x7u64));let result=a & b;cpu.x[8]=(result) as u32 as u64;}
// 0x1128c34c str w8, [x19, #0x1c]
{let address=(cpu.x[19]).wrapping_add(0x1cu64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128c350 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x1128c354 ldp x22, x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x1128c358 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x1128c35c ret
cpu.pc=cpu.x[30];return Ok(());
}
0x1128c360=>{
// 0x1128c360 ldr w21, [x19, #0x14]
{let address=(cpu.x[19]).wrapping_add(0x14u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[21]=(v0) as u32 as u64;}
// 0x1128c364 ldr w8, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x1128c368 cmp w21, w8
{let a=((cpu.x[21] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x1128c36c b.lt #0x1128c3bc
if cpu.condition(11){pc=0x1128c3bc;continue;}pc=0x1128c370;continue;
}
0x1128c370=>{
// 0x1128c370 add w9, w8, w8, lsr #1
{let a=((cpu.x[8] as u32 as u64));let b=(((cpu.x[8] as u32 as u64)) as u32).wrapping_shr(1) as u64;let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x1128c374 cmp w8, #0x100
{let a=((cpu.x[8] as u32 as u64));let b=((0x100u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x1128c378 mov w10, #0x64
cpu.x[10]=(((0x64u64))) as u32 as u64;
// 0x1128c37c ldr x22, [x19, #0x30]
{let address=(cpu.x[19]).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[22]=(v0) as u64;}
// 0x1128c380 csel w9, w10, w9, lt
cpu.x[9]=(if cpu.condition(11){((cpu.x[10] as u32 as u64))}else{((cpu.x[9] as u32 as u64))}) as u32 as u64;
// 0x1128c384 add w8, w8, w9
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x1128c388 add w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x1128c38c sxtw x0, w8
cpu.x[0]=((((cpu.x[8] as u32 as u64))) as i32 as i64 as u64) as u64;
// 0x1128c390 str w8, [x19, #0x28]
{let address=(cpu.x[19]).wrapping_add(0x28u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128c394 bl #0x1187d520
cpu.x[30]=0x1128c398;super::chunk8::f_1187d520(cpu,rt,0x1187d520)?;
// 0x1128c398 mov w1, wzr
cpu.x[1]=(((0u64))) as u32 as u64;
// 0x1128c39c mov x2, x22
cpu.x[2]=(((cpu.x[22]))) as u64;
// 0x1128c3a0 mov w3, wzr
cpu.x[3]=(((0u64))) as u32 as u64;
// 0x1128c3a4 mov w4, w21
cpu.x[4]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x1128c3a8 stp x0, x0, [x19, #0x30]
{let address=(cpu.x[19]).wrapping_add(0x30u64);let v0=cpu.x[0];let v1=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x1128c3ac bl #0x112aef80
cpu.x[30]=0x1128c3b0;super::chunk11::f_112aef80(cpu,rt,0x112aef80)?;
// 0x1128c3b0 cbz x22, #0x1128c3bc
if (cpu.x[22]) == 0 {pc=0x1128c3bc;continue;}pc=0x1128c3b4;continue;
}
0x1128c3b4=>{
// 0x1128c3b4 mov x0, x22
cpu.x[0]=(((cpu.x[22]))) as u64;
// 0x1128c3b8 bl #0x1187d360
cpu.x[30]=0x1128c3bc;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
pc=0x1128c3bc;continue;
}
0x1128c3bc=>{
// 0x1128c3bc ldrsw x8, [x19, #0x14]
{let address=(cpu.x[19]).wrapping_add(0x14u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x1128c3c0 ldr x9, [x19, #0x38]
{let address=(cpu.x[19]).wrapping_add(0x38u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x1128c3c4 add w10, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x1128c3c8 str w10, [x19, #0x14]
{let address=(cpu.x[19]).wrapping_add(0x14u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1128c3cc strb wzr, [x9, x8]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[8])));let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x1128c3d0 ldr w8, [x19, #0x1c]
{let address=(cpu.x[19]).wrapping_add(0x1cu64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x1128c3d4 tbnz w20, #0, #0x1128c31c
if ((cpu.x[20] as u32 as u64) & (1u64 << 0)) != 0 {pc=0x1128c31c;continue;}pc=0x1128c3d8;continue;
}
0x1128c3d8=>{
// 0x1128c3d8 b #0x1128c340
pc=0x1128c340;continue;
}
_=>return Err(format!("untranslated block {pc:#x} in 0x1128c2f4")),}}}
#[inline]
pub(super) unsafe fn f_112949ec(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x112949ec=>{
// 0x112949ec cmp w1, #1
{let a=((cpu.x[1] as u32 as u64));let b=((0x1u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x112949f0 b.lt #0x11294ad8
if cpu.condition(11){pc=0x11294ad8;continue;}pc=0x112949f4;continue;
}
0x112949f4=>{
// 0x112949f4 ldr w8, [x0, #0x9c0]
{let address=(cpu.x[0]).wrapping_add(0x9c0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x112949f8 cbnz w8, #0x11294a70
if (cpu.x[8] as u32 as u64) != 0 {pc=0x11294a70;continue;}pc=0x112949fc;continue;
}
0x112949fc=>{
// 0x112949fc mov w10, #0xb0df
cpu.x[10]=(((0xb0dfu64))) as u32 as u64;
// 0x11294a00 mov x11, xzr
cpu.x[11]=(((0u64))) as u64;
// 0x11294a04 mov w8, #-0xe3
cpu.x[8]=(((0xffffffffffffff1du64))) as u32 as u64;
// 0x11294a08 mov w9, #0x18d
cpu.x[9]=(((0x18du64))) as u32 as u64;
// 0x11294a0c movk w10, #0x9908, lsl #16
cpu.x[10]=((cpu.x[10] as u32 as u64 & !(0xffff0000u64)) | 0x99080000u64) as u32 as u64;
pc=0x11294a10;continue;
}
0x11294a10=>{
// 0x11294a10 lsl x12, x11, #2
cpu.x[12]=(((((cpu.x[11]))) as u64).wrapping_shl(((((0x2u64))) & 63) as u32) as u64) as u64;
// 0x11294a14 cmp x11, #0x26f
{let a=((cpu.x[11]));let b=((0x26fu64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x11294a18 csinc x13, xzr, x11, eq
cpu.x[13]=(if cpu.condition(0){((0u64))}else{(((cpu.x[11]))).wrapping_add(1)}) as u64;
// 0x11294a1c cmp x11, #0xe3
{let a=((cpu.x[11]));let b=((0xe3u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x11294a20 ldr w14, [x0, x12]
{let address=((cpu.x[0]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[12])));let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[14]=(v0) as u32 as u64;}
// 0x11294a24 ldr w13, [x0, x13, lsl #2]
{let address=((cpu.x[0]).wrapping_add(0x0u64)).wrapping_add((((cpu.x[13])) as u64).wrapping_shl(2) as u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[13]=(v0) as u32 as u64;}
// 0x11294a28 csel w15, w9, w8, lo
cpu.x[15]=(if cpu.condition(3){((cpu.x[9] as u32 as u64))}else{((cpu.x[8] as u32 as u64))}) as u32 as u64;
// 0x11294a2c add w15, w15, w11
{let a=((cpu.x[15] as u32 as u64));let b=((cpu.x[11] as u32 as u64));let result=a.wrapping_add(b);cpu.x[15]=(result) as u32 as u64;}
// 0x11294a30 add x11, x11, #1
{let a=((cpu.x[11]));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[11]=(result) as u64;}
// 0x11294a34 and w14, w14, #0x80000000
{let a=((cpu.x[14] as u32 as u64));let b=((0x80000000u64));let result=a & b;cpu.x[14]=(result) as u32 as u64;}
// 0x11294a38 and w16, w13, #0x7ffffffe
{let a=((cpu.x[13] as u32 as u64));let b=((0x7ffffffeu64));let result=a & b;cpu.x[16]=(result) as u32 as u64;}
// 0x11294a3c ldr w15, [x0, w15, uxtw #2]
{let address=((cpu.x[0]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[15] as u32 as u64) as u32 as u64)) as u64).wrapping_shl(2) as u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[15]=(v0) as u32 as u64;}
// 0x11294a40 orr w14, w16, w14
{let a=((cpu.x[16] as u32 as u64));let b=((cpu.x[14] as u32 as u64));let result=a | b;cpu.x[14]=(result) as u32 as u64;}
// 0x11294a44 tst w13, #1
{let a=((cpu.x[13] as u32 as u64));let b=((0x1u64));let result=a & b;cpu.logical_flags(result,32);}
// 0x11294a48 eor w14, w15, w14, asr #1
{let a=((cpu.x[15] as u32 as u64));let b=(((cpu.x[14] as u32 as u64)) as i32).wrapping_shr(1) as u64;let result=a ^ b;cpu.x[14]=(result) as u32 as u64;}
// 0x11294a4c eor w15, w14, w10
{let a=((cpu.x[14] as u32 as u64));let b=((cpu.x[10] as u32 as u64));let result=a ^ b;cpu.x[15]=(result) as u32 as u64;}
// 0x11294a50 csel w13, w14, w15, eq
cpu.x[13]=(if cpu.condition(0){((cpu.x[14] as u32 as u64))}else{((cpu.x[15] as u32 as u64))}) as u32 as u64;
// 0x11294a54 cmp x11, #0x270
{let a=((cpu.x[11]));let b=((0x270u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x11294a58 str w13, [x0, x12]
{let address=((cpu.x[0]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[12])));let v0=cpu.x[13] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x11294a5c b.ne #0x11294a10
if cpu.condition(1){pc=0x11294a10;continue;}pc=0x11294a60;continue;
}
0x11294a60=>{
// 0x11294a60 ldr w8, [x0, #0x9c4]
{let address=(cpu.x[0]).wrapping_add(0x9c4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x11294a64 add w9, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[9]=(result) as u32 as u64;}
// 0x11294a68 ldr w8, [x0, #0x9c0]
{let address=(cpu.x[0]).wrapping_add(0x9c0u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x11294a6c str w9, [x0, #0x9c4]
{let address=(cpu.x[0]).wrapping_add(0x9c4u64);let v0=cpu.x[9] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
pc=0x11294a70;continue;
}
0x11294a70=>{
// 0x11294a70 ldr w9, [x0, w8, sxtw #2]
{let address=((cpu.x[0]).wrapping_add(0x0u64)).wrapping_add(((((cpu.x[8] as u32 as u64) as i32 as i64 as u64)) as u64).wrapping_shl(2) as u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[9]=(v0) as u32 as u64;}
// 0x11294a74 mov w10, #0x5680
cpu.x[10]=(((0x5680u64))) as u32 as u64;
// 0x11294a78 add w8, w8, #1
{let a=((cpu.x[8] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x11294a7c movk w10, #0x9d2c, lsl #16
cpu.x[10]=((cpu.x[10] as u32 as u64 & !(0xffff0000u64)) | 0x9d2c0000u64) as u32 as u64;
// 0x11294a80 eor w9, w9, w9, asr #11
{let a=((cpu.x[9] as u32 as u64));let b=(((cpu.x[9] as u32 as u64)) as i32).wrapping_shr(11) as u64;let result=a ^ b;cpu.x[9]=(result) as u32 as u64;}
// 0x11294a84 and w10, w10, w9, lsl #7
{let a=((cpu.x[10] as u32 as u64));let b=(((cpu.x[9] as u32 as u64)) as u32).wrapping_shl(7) as u64;let result=a & b;cpu.x[10]=(result) as u32 as u64;}
// 0x11294a88 eor w9, w10, w9
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a ^ b;cpu.x[9]=(result) as u32 as u64;}
// 0x11294a8c mov w10, #-0x103a0000
cpu.x[10]=(((0xffffffffefc60000u64))) as u32 as u64;
// 0x11294a90 and w10, w10, w9, lsl #15
{let a=((cpu.x[10] as u32 as u64));let b=(((cpu.x[9] as u32 as u64)) as u32).wrapping_shl(15) as u64;let result=a & b;cpu.x[10]=(result) as u32 as u64;}
// 0x11294a94 eor w9, w10, w9
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a ^ b;cpu.x[9]=(result) as u32 as u64;}
// 0x11294a98 mov w10, #0x20d3
cpu.x[10]=(((0x20d3u64))) as u32 as u64;
// 0x11294a9c movk w10, #0xd20d, lsl #16
cpu.x[10]=((cpu.x[10] as u32 as u64 & !(0xffff0000u64)) | 0xd20d0000u64) as u32 as u64;
// 0x11294aa0 eor w9, w9, w9, asr #18
{let a=((cpu.x[9] as u32 as u64));let b=(((cpu.x[9] as u32 as u64)) as i32).wrapping_shr(18) as u64;let result=a ^ b;cpu.x[9]=(result) as u32 as u64;}
// 0x11294aa4 smull x10, w8, w10
cpu.x[10]=(((((cpu.x[8] as u32 as u64))) as i32 as i64).wrapping_mul((((cpu.x[10] as u32 as u64))) as i32 as i64) as u64) as u64;
// 0x11294aa8 and w9, w9, #0x7fffffff
{let a=((cpu.x[9] as u32 as u64));let b=((0x7fffffffu64));let result=a & b;cpu.x[9]=(result) as u32 as u64;}
// 0x11294aac udiv w11, w9, w1
{let a=((cpu.x[9] as u32 as u64)) as u32;let b=((cpu.x[1] as u32 as u64)) as u32;cpu.x[11]=(if b==0{0}else{a.wrapping_div(b) as u64}) as u32 as u64;}
// 0x11294ab0 lsr x10, x10, #0x20
cpu.x[10]=(((((cpu.x[10]))) as u64).wrapping_shr(((((0x20u64))) & 63) as u32) as u64) as u64;
// 0x11294ab4 add w10, w10, w8
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[8] as u32 as u64));let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x11294ab8 asr w12, w10, #9
cpu.x[12]=(((((cpu.x[10] as u32 as u64))) as i32).wrapping_shr(((((0x9u64))) & 31) as u32) as u64) as u32 as u64;
// 0x11294abc add w10, w12, w10, lsr #31
{let a=((cpu.x[12] as u32 as u64));let b=(((cpu.x[10] as u32 as u64)) as u32).wrapping_shr(31) as u64;let result=a.wrapping_add(b);cpu.x[10]=(result) as u32 as u64;}
// 0x11294ac0 mov w12, #0x270
cpu.x[12]=(((0x270u64))) as u32 as u64;
// 0x11294ac4 msub w10, w10, w12, w8
cpu.x[10]=((((cpu.x[8] as u32 as u64))).wrapping_sub((((cpu.x[10] as u32 as u64))).wrapping_mul(((cpu.x[12] as u32 as u64))))) as u32 as u64;
// 0x11294ac8 msub w8, w11, w1, w9
cpu.x[8]=((((cpu.x[9] as u32 as u64))).wrapping_sub((((cpu.x[11] as u32 as u64))).wrapping_mul(((cpu.x[1] as u32 as u64))))) as u32 as u64;
// 0x11294acc str w10, [x0, #0x9c0]
{let address=(cpu.x[0]).wrapping_add(0x9c0u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x11294ad0 mov w0, w8
cpu.x[0]=(((cpu.x[8] as u32 as u64))) as u32 as u64;
// 0x11294ad4 ret
cpu.pc=cpu.x[30];return Ok(());
}
0x11294ad8=>{
// 0x11294ad8 mov w0, wzr
cpu.x[0]=(((0u64))) as u32 as u64;
// 0x11294adc ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x112949ec")),}}}
#[inline]
pub(super) unsafe fn f_11294ae8(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x11294ae8=>{
// 0x11294ae8 stp x29, x30, [sp, #-0x30]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffd0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x11294aec str x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x11294af0 stp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x11294af4 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x11294af8 adrp x21, #0x11a90000
cpu.x[21]=(0x11a90000u64) as u64;
// 0x11294afc mov w19, w1
cpu.x[19]=(((cpu.x[1] as u32 as u64))) as u32 as u64;
// 0x11294b00 mov x20, x0
cpu.x[20]=(((cpu.x[0]))) as u64;
// 0x11294b04 ldr x8, [x21, #0xdb0]
{let address=(cpu.x[21]).wrapping_add(0xdb0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;}
// 0x11294b08 str wzr, [x0]
{let address=(cpu.x[0]).wrapping_add(0x0u64);let v0=0u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x11294b0c cbnz x8, #0x11294b20
if (cpu.x[8]) != 0 {pc=0x11294b20;continue;}pc=0x11294b10;continue;
}
0x11294b10=>{
// 0x11294b10 mov x0, xzr
cpu.x[0]=(((0u64))) as u64;
// 0x11294b14 bl #0x1187d8e0
cpu.x[30]=0x11294b18;super::dispatch(cpu,rt,0x1187d8e0)?;
// 0x11294b18 mov x8, x0
cpu.x[8]=(((cpu.x[0]))) as u64;
// 0x11294b1c str x0, [x21, #0xdb0]
{let address=(cpu.x[21]).wrapping_add(0xdb0u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x11294b20;continue;
}
0x11294b20=>{
// 0x11294b20 adrp x9, #0x11a90000
cpu.x[9]=(0x11a90000u64) as u64;
// 0x11294b24 ldr x9, [x9, #0xda8]
{let address=(cpu.x[9]).wrapping_add(0xda8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x11294b28 sub x8, x8, x9
{let a=((cpu.x[8]));let b=((cpu.x[9]));let result=a.wrapping_sub(b);cpu.x[8]=(result) as u64;}
// 0x11294b2c cmp x8, #0xf
{let a=((cpu.x[8]));let b=((0xfu64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x11294b30 cinc w8, w19, hi
cpu.x[8]=(if cpu.condition(8){(((cpu.x[19] as u32 as u64))).wrapping_add(1)}else{((cpu.x[19] as u32 as u64))}) as u32 as u64;
// 0x11294b34 str w8, [x20]
{let address=(cpu.x[20]).wrapping_add(0x0u64);let v0=cpu.x[8] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x11294b38 ldp x20, x19, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x11294b3c ldr x21, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[21]=(v0) as u64;}
// 0x11294b40 ldp x29, x30, [sp], #0x30
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x30u64)))) as u64;}
// 0x11294b44 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x11294ae8")),}}}
#[inline]
pub(super) unsafe fn f_112ada7c(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x112ada7c=>{
// 0x112ada7c add x1, x0, w1, sxtw
{let a=((cpu.x[0]));let b=(((cpu.x[1] as u32 as u64) as i32 as i64 as u64));let result=a.wrapping_add(b);cpu.x[1]=(result) as u64;}
// 0x112ada80 mov x0, x8
cpu.x[0]=(((cpu.x[8]))) as u64;
// 0x112ada84 b #0x1142fdc4
super::chunk0::f_1142fdc4(cpu,rt,0x1142fdc4)?;return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x112ada7c")),}}}
#[inline]
pub(super) unsafe fn f_1142fce0(cpu:&mut Cpu,rt:&mut Runtime,mut pc:u64)->Result<(),String>{loop{match pc{
0x1142fce0=>{
// 0x1142fce0 stp x29, x30, [sp, #-0x40]!
{let address=(cpu.sp).wrapping_add(0xffffffffffffffc0u64);let v0=cpu.x[29];let v1=cpu.x[30];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);cpu.sp=(address) as u64;}
// 0x1142fce4 str x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=cpu.x[23];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
// 0x1142fce8 stp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=cpu.x[22];let v1=cpu.x[21];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x1142fcec stp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=cpu.x[20];let v1=cpu.x[19];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);rt.memory.write::<u64>(address.wrapping_add(8),v1 as u64);}
// 0x1142fcf0 mov x29, sp
cpu.x[29]=(((cpu.sp))) as u64;
// 0x1142fcf4 mov x20, x1
cpu.x[20]=(((cpu.x[1]))) as u64;
// 0x1142fcf8 sxtw x1, w2
cpu.x[1]=((((cpu.x[2] as u32 as u64))) as i32 as i64 as u64) as u64;
// 0x1142fcfc mov x19, x0
cpu.x[19]=(((cpu.x[0]))) as u64;
// 0x1142fd00 mov x0, x20
cpu.x[0]=(((cpu.x[20]))) as u64;
// 0x1142fd04 bl #0x1187e630
cpu.x[30]=0x1142fd08;super::chunk12::f_1187e630(cpu,rt,0x1187e630)?;
pc=0x1142fd08;continue;
}
0x1142fd08=>{
// 0x1142fd08 ldr w8, [x19, #4]
{let address=(cpu.x[19]).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[8]=(v0) as u32 as u64;}
// 0x1142fd0c mov x21, x0
cpu.x[21]=(((cpu.x[0]))) as u64;
// 0x1142fd10 str w21, [x19, #4]
{let address=(cpu.x[19]).wrapping_add(0x4u64);let v0=cpu.x[21] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1142fd14 cmp w8, #8
{let a=((cpu.x[8] as u32 as u64));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x1142fd18 cset w9, lt
cpu.x[9]=((cpu.condition(11)) as u64) as u32 as u64;
// 0x1142fd1c cmp w21, #8
{let a=((cpu.x[21] as u32 as u64));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x1142fd20 cset w10, lt
cpu.x[10]=((cpu.condition(11)) as u64) as u32 as u64;
// 0x1142fd24 cmp w8, w21
{let a=((cpu.x[8] as u32 as u64));let b=((cpu.x[21] as u32 as u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x1142fd28 b.lt #0x1142fd3c
if cpu.condition(11){pc=0x1142fd3c;continue;}pc=0x1142fd2c;continue;
}
0x1142fd2c=>{
// 0x1142fd2c eor w9, w10, w9
{let a=((cpu.x[10] as u32 as u64));let b=((cpu.x[9] as u32 as u64));let result=a ^ b;cpu.x[9]=(result) as u32 as u64;}
// 0x1142fd30 cbnz w9, #0x1142fd3c
if (cpu.x[9] as u32 as u64) != 0 {pc=0x1142fd3c;continue;}pc=0x1142fd34;continue;
}
0x1142fd34=>{
// 0x1142fd34 mov w22, w21
cpu.x[22]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x1142fd38 b #0x1142fd78
pc=0x1142fd78;continue;
}
0x1142fd3c=>{
// 0x1142fd3c cmp w8, #8
{let a=((cpu.x[8] as u32 as u64));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x1142fd40 mov w22, w21
cpu.x[22]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x1142fd44 b.lt #0x1142fd60
if cpu.condition(11){pc=0x1142fd60;continue;}pc=0x1142fd48;continue;
}
0x1142fd48=>{
// 0x1142fd48 ldr x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[0]=(v0) as u64;}
// 0x1142fd4c mov w22, w21
cpu.x[22]=(((cpu.x[21] as u32 as u64))) as u32 as u64;
// 0x1142fd50 cbz x0, #0x1142fd5c
if (cpu.x[0]) == 0 {pc=0x1142fd5c;continue;}pc=0x1142fd54;continue;
}
0x1142fd54=>{
// 0x1142fd54 bl #0x1187d360
cpu.x[30]=0x1142fd58;super::chunk12::f_1187d360(cpu,rt,0x1187d360)?;
// 0x1142fd58 ldr w22, [x19, #4]
{let address=(cpu.x[19]).wrapping_add(0x4u64);let v0=rt.memory.read::<u32>(address.wrapping_add(0));cpu.x[22]=(v0) as u32 as u64;}
pc=0x1142fd5c;continue;
}
0x1142fd5c=>{
// 0x1142fd5c str xzr, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=0u64;rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x1142fd60;continue;
}
0x1142fd60=>{
// 0x1142fd60 cmp w21, #8
{let a=((cpu.x[21] as u32 as u64));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x1142fd64 b.lt #0x1142fd78
if cpu.condition(11){pc=0x1142fd78;continue;}pc=0x1142fd68;continue;
}
0x1142fd68=>{
// 0x1142fd68 add w8, w22, #1
{let a=((cpu.x[22] as u32 as u64));let b=((0x1u64));let result=a.wrapping_add(b);cpu.x[8]=(result) as u32 as u64;}
// 0x1142fd6c sxtw x0, w8
cpu.x[0]=((((cpu.x[8] as u32 as u64))) as i32 as i64 as u64) as u64;
// 0x1142fd70 bl #0x1187d520
cpu.x[30]=0x1142fd74;super::chunk8::f_1187d520(cpu,rt,0x1187d520)?;
// 0x1142fd74 str x0, [x19, #8]
{let address=(cpu.x[19]).wrapping_add(0x8u64);let v0=cpu.x[0];rt.memory.write::<u64>(address.wrapping_add(0),v0 as u64);}
pc=0x1142fd78;continue;
}
0x1142fd78=>{
// 0x1142fd78 mov x23, x19
cpu.x[23]=(((cpu.x[19]))) as u64;
// 0x1142fd7c cmp w22, #8
{let a=((cpu.x[22] as u32 as u64));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,32,true);}
// 0x1142fd80 sxtw x2, w21
cpu.x[2]=((((cpu.x[21] as u32 as u64))) as i32 as i64 as u64) as u64;
// 0x1142fd84 ldr x8, [x23, #8]!
{let address=(cpu.x[23]).wrapping_add(0x8u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[8]=(v0) as u64;cpu.x[23]=(address) as u64;}
// 0x1142fd88 mov x1, x20
cpu.x[1]=(((cpu.x[20]))) as u64;
// 0x1142fd8c csel x0, x23, x8, lt
cpu.x[0]=(if cpu.condition(11){((cpu.x[23]))}else{((cpu.x[8]))}) as u64;
// 0x1142fd90 bl #0x1187d3e0
cpu.x[30]=0x1142fd94;super::chunk3::f_1187d3e0(cpu,rt,0x1187d3e0)?;
pc=0x1142fd94;continue;
}
0x1142fd94=>{
// 0x1142fd94 ldrsw x8, [x19, #4]
{let address=(cpu.x[19]).wrapping_add(0x4u64);let v0=rt.memory.read::<i32>(address.wrapping_add(0)) as i64 as u64;cpu.x[8]=(v0) as u64;}
// 0x1142fd98 ldr x9, [x23]
{let address=(cpu.x[23]).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[9]=(v0) as u64;}
// 0x1142fd9c mov w10, #-1
cpu.x[10]=(((0xffffffffffffffffu64))) as u32 as u64;
// 0x1142fda0 cmp x8, #8
{let a=((cpu.x[8]));let b=((0x8u64));let result=a.wrapping_sub(b);cpu.flags(a,b,result,64,true);}
// 0x1142fda4 csel x9, x23, x9, lt
cpu.x[9]=(if cpu.condition(11){((cpu.x[23]))}else{((cpu.x[9]))}) as u64;
// 0x1142fda8 strb wzr, [x9, x8]
{let address=((cpu.x[9]).wrapping_add(0x0u64)).wrapping_add(((cpu.x[8])));let v0=0u64;rt.memory.write::<u8>(address.wrapping_add(0),v0 as u8);}
// 0x1142fdac str w10, [x19]
{let address=(cpu.x[19]).wrapping_add(0x0u64);let v0=cpu.x[10] as u32 as u64;rt.memory.write::<u32>(address.wrapping_add(0),v0 as u32);}
// 0x1142fdb0 ldp x20, x19, [sp, #0x30]
{let address=(cpu.sp).wrapping_add(0x30u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[20]=(v0) as u64;cpu.x[19]=(v1) as u64;}
// 0x1142fdb4 ldr x23, [sp, #0x10]
{let address=(cpu.sp).wrapping_add(0x10u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));cpu.x[23]=(v0) as u64;}
// 0x1142fdb8 ldp x22, x21, [sp, #0x20]
{let address=(cpu.sp).wrapping_add(0x20u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[22]=(v0) as u64;cpu.x[21]=(v1) as u64;}
// 0x1142fdbc ldp x29, x30, [sp], #0x40
{let address=(cpu.sp).wrapping_add(0x0u64);let v0=rt.memory.read::<u64>(address.wrapping_add(0));let v1=rt.memory.read::<u64>(address.wrapping_add(8));cpu.x[29]=(v0) as u64;cpu.x[30]=(v1) as u64;cpu.sp=((cpu.sp).wrapping_add(((0x40u64)))) as u64;}
// 0x1142fdc0 ret
cpu.pc=cpu.x[30];return Ok(());
}
_=>return Err(format!("untranslated block {pc:#x} in 0x1142fce0")),}}}
