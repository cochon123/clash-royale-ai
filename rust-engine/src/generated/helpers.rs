//! Scalar definitions of original SIMD lane operations; LLVM lowers these to host SIMD.
use crate::cpu::Cpu;
#[inline(always)]
pub(super) fn fp_domain(cpu: &Cpu, values: &[f64]) -> Result<(), String> {
    if cpu.fpcr != 0 || cpu.fpsr & 0x10 == 0 {
        return Err("FP control/status outside the supported checkpoint domain".into());
    }
    if values
        .iter()
        .any(|x| !x.is_finite() || (x.abs() > 0.0 && x.abs() < f64::MIN_POSITIVE))
    {
        return Err("nonfinite/subnormal FP operand needs explicit architectural handling".into());
    }
    Ok(())
}
#[inline(always)]
fn mask(bits: u32) -> u128 {
    if bits == 128 {
        u128::MAX
    } else {
        (1u128 << bits) - 1
    }
}
#[inline(always)]
fn signed(x: u128, bits: u32) -> i128 {
    ((x << (128 - bits)) as i128) >> (128 - bits)
}
#[inline(always)]
pub(super) fn lane_dup(x: u128, bits: u32, lanes: u32) -> u128 {
    let mut result = 0;
    for i in 0..lanes {
        result |= (x & mask(bits)) << (i * bits);
    }
    result
}
#[inline(always)]
pub(super) fn lane_binary(a: u128, b: u128, bits: u32, lanes: u32, op: &str) -> u128 {
    let mut result = 0;
    for i in 0..lanes {
        let x = (a >> (i * bits)) & mask(bits);
        let y = (b >> (i * bits)) & mask(bits);
        let value = match op {
            "add" => x.wrapping_add(y),
            "sub" => x.wrapping_sub(y),
            "mul" => x.wrapping_mul(y),
            "cmeq" => {
                if x == y {
                    u128::MAX
                } else {
                    0
                }
            }
            "cmgt" => {
                if signed(x, bits) > signed(y, bits) {
                    u128::MAX
                } else {
                    0
                }
            }
            "cmge" => {
                if signed(x, bits) >= signed(y, bits) {
                    u128::MAX
                } else {
                    0
                }
            }
            "cmlt" => {
                if signed(x, bits) < signed(y, bits) {
                    u128::MAX
                } else {
                    0
                }
            }
            "cmhi" => {
                if x > y {
                    u128::MAX
                } else {
                    0
                }
            }
            "cmhs" => {
                if x >= y {
                    u128::MAX
                } else {
                    0
                }
            }
            "sabd" => (signed(x, bits) - signed(y, bits)).unsigned_abs(),
            "uabd" => x.abs_diff(y),
            "smin" => {
                if signed(x, bits) < signed(y, bits) {
                    x
                } else {
                    y
                }
            }
            "smax" => {
                if signed(x, bits) > signed(y, bits) {
                    x
                } else {
                    y
                }
            }
            _ => unreachable!("vector binary {op}"),
        };
        result |= (value & mask(bits)) << (i * bits);
    }
    result
}
#[inline(always)]
pub(super) fn lane_count(a: u128, bits: u32, lanes: u32) -> u128 {
    let mut result = 0;
    for i in 0..lanes {
        result |= (((a >> (i * bits)) & mask(bits)).count_ones() as u128) << (i * bits);
    }
    result
}
#[inline(always)]
pub(super) fn lane_shift(a: u128, bits: u32, lanes: u32, shift: u32, op: &str) -> u128 {
    let mut result = 0;
    for i in 0..lanes {
        let x = (a >> (i * bits)) & mask(bits);
        let y = match op {
            "shl" => x << shift,
            "ushr" => x >> shift,
            "sshr" => (signed(x, bits) >> shift) as u128,
            _ => unreachable!(),
        };
        result |= (y & mask(bits)) << (i * bits);
    }
    result
}
#[inline(always)]
pub(super) fn lane_reduce(a: u128, bits: u32, lanes: u32, op: &str) -> u128 {
    let mut result = if op == "smaxv" {
        1u128 << (bits - 1)
    } else {
        0
    };
    for i in 0..lanes {
        let x = (a >> (i * bits)) & mask(bits);
        result = match op {
            "addv" | "uaddlv" => result.wrapping_add(x),
            "saddlv" => result.wrapping_add(signed(x, bits) as u128),
            "smaxv" => {
                if signed(x, bits) > signed(result, bits) {
                    x
                } else {
                    result
                }
            }
            "umaxv" => result.max(x),
            _ => unreachable!(),
        };
    }
    result
}
#[inline(always)]
pub(super) fn lane_shuffle(a: u128, b: u128, bits: u32, lanes: u32, op: &str) -> u128 {
    let mut result = 0;
    for i in 0..lanes {
        let (source, index) = match op {
            "uzp1" => (if i < lanes / 2 { a } else { b }, (i % (lanes / 2)) * 2),
            "uzp2" => (if i < lanes / 2 { a } else { b }, (i % (lanes / 2)) * 2 + 1),
            "zip1" => (if i % 2 == 0 { a } else { b }, i / 2),
            "zip2" => (if i % 2 == 0 { a } else { b }, i / 2 + lanes / 2),
            "trn1" => (if i % 2 == 0 { a } else { b }, (i / 2) * 2),
            "trn2" => (if i % 2 == 0 { a } else { b }, (i / 2) * 2 + 1),
            _ => unreachable!(),
        };
        result |= ((source >> (index * bits)) & mask(bits)) << (i * bits);
    }
    result
}
#[inline(always)]
pub(super) fn lane_resize(
    a: u128,
    source_bits: u32,
    bits: u32,
    lanes: u32,
    shift: u32,
    op: &str,
) -> u128 {
    let mut result = 0;
    for i in 0..lanes {
        let x = (a >> (i * source_bits)) & mask(source_bits);
        let y = match op {
            "xtn" => x,
            "shrn" => x >> shift,
            "ushll" | "shll" => x << shift,
            "sshll" => (signed(x, source_bits) as u128) << shift,
            _ => unreachable!(),
        };
        result |= (y & mask(bits)) << (i * bits);
    }
    result
}
