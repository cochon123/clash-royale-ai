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
            "cmtst" => {
                if x & y != 0 {
                    u128::MAX
                } else {
                    0
                }
            }
            "sabd" => (signed(x, bits) - signed(y, bits)).unsigned_abs(),
            "uabd" => x.abs_diff(y),
            "uqsub" => x.saturating_sub(y),
            "umin" => {
                if x < y {
                    x
                } else {
                    y
                }
            }
            "umax" => {
                if x > y {
                    x
                } else {
                    y
                }
            }
            "neg" => 0u128.wrapping_sub(x),
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
    let mut result = match op {
        "smaxv" => 1u128 << (bits - 1),
        "uminv" => mask(bits),
        _ => 0,
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
            "uminv" => result.min(x),
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
#[inline(always)]
pub(super) fn lane_fp_tma(acc: u128, a: u128, b: u128, bits: u32, lanes: u32, sub: bool) -> u128 {
    // FMLA/FMLS are fused: one rounding for the multiply-accumulate.
    let mut result = 0;
    for i in 0..lanes {
        let sh = i * bits;
        let value = if bits == 32 {
            let x = f32::from_bits(((a >> sh) & mask(bits)) as u32);
            let y = f32::from_bits(((b >> sh) & mask(bits)) as u32);
            let d = f32::from_bits(((acc >> sh) & mask(bits)) as u32);
            (if sub { (-x).mul_add(y, d) } else { x.mul_add(y, d) }).to_bits() as u128
        } else {
            let x = f64::from_bits(((a >> sh) & mask(bits)) as u64);
            let y = f64::from_bits(((b >> sh) & mask(bits)) as u64);
            let d = f64::from_bits(((acc >> sh) & mask(bits)) as u64);
            (if sub { (-x).mul_add(y, d) } else { x.mul_add(y, d) }).to_bits() as u128
        };
        result |= (value & mask(bits)) << sh;
    }
    result
}
#[inline(always)]
pub(super) fn fp_minmax_f32(x: f32, y: f32, max: bool) -> f32 {
    // FMAXNM/FMINNM: a NaN operand yields the other operand, both-NaN yields a
    // quiet NaN, and ±0 ties favour +0 for FMAXNM and -0 for FMINNM.
    if x.is_nan() {
        if y.is_nan() { return x + y; }
        return y;
    }
    if y.is_nan() { return x; }
    if x == 0.0 && y == 0.0 {
        let xs = x.is_sign_negative();
        let ys = y.is_sign_negative();
        return if max { if !xs { x } else { y } } else { if xs { x } else { y } };
    }
    if max { if x > y { x } else { y } } else { if x < y { x } else { y } }
}
#[inline(always)]
pub(super) fn fp_minmax_f64(x: f64, y: f64, max: bool) -> f64 {
    if x.is_nan() {
        if y.is_nan() { return x + y; }
        return y;
    }
    if y.is_nan() { return x; }
    if x == 0.0 && y == 0.0 {
        let xs = x.is_sign_negative();
        let ys = y.is_sign_negative();
        return if max { if !xs { x } else { y } } else { if xs { x } else { y } };
    }
    if max { if x > y { x } else { y } } else { if x < y { x } else { y } }
}
#[inline(always)]
pub(super) fn lane_fp_minmax(a: u128, b: u128, bits: u32, lanes: u32, max: bool) -> u128 {
    let mut result = 0;
    for i in 0..lanes {
        let sh = i * bits;
        let value = if bits == 32 {
            let x = f32::from_bits(((a >> sh) & mask(bits)) as u32);
            let y = f32::from_bits(((b >> sh) & mask(bits)) as u32);
            fp_minmax_f32(x, y, max).to_bits() as u128
        } else {
            let x = f64::from_bits(((a >> sh) & mask(bits)) as u64);
            let y = f64::from_bits(((b >> sh) & mask(bits)) as u64);
            fp_minmax_f64(x, y, max).to_bits() as u128
        };
        result |= (value & mask(bits)) << sh;
    }
    result
}
#[inline(always)]
pub(super) fn lane_fp_binary(a: u128, b: u128, bits: u32, lanes: u32, op: &str) -> u128 {
    // Vector FP arithmetic. FMAX/FMIN (not NM) raise to a quiet NaN when either
    // operand is NaN, and ±0 ties favour +0 for FMAX and -0 for FMIN.
    let mut result = 0;
    for i in 0..lanes {
        let sh = i * bits;
        let value = if bits == 32 {
            let x = f32::from_bits(((a >> sh) & mask(bits)) as u32);
            let y = f32::from_bits(((b >> sh) & mask(bits)) as u32);
            let z = match op {
                "fmul" => x * y,
                "fadd" => x + y,
                "fsub" => x - y,
                "fdiv" => x / y,
                "fmax" => if x.is_nan() || y.is_nan() { x + y } else { fp_minmax_f32(x, y, true) },
                "fmin" => if x.is_nan() || y.is_nan() { x + y } else { fp_minmax_f32(x, y, false) },
                "fnmul" => -(x * y),
                "fabd" => (x - y).abs(),
                _ => unreachable!(),
            };
            z.to_bits() as u128
        } else {
            let x = f64::from_bits(((a >> sh) & mask(bits)) as u64);
            let y = f64::from_bits(((b >> sh) & mask(bits)) as u64);
            let z = match op {
                "fmul" => x * y,
                "fadd" => x + y,
                "fsub" => x - y,
                "fdiv" => x / y,
                "fmax" => if x.is_nan() || y.is_nan() { x + y } else { fp_minmax_f64(x, y, true) },
                "fmin" => if x.is_nan() || y.is_nan() { x + y } else { fp_minmax_f64(x, y, false) },
                "fnmul" => -(x * y),
                "fabd" => (x - y).abs(),
                _ => unreachable!(),
            };
            z.to_bits() as u128
        };
        result |= (value & mask(bits)) << sh;
    }
    result
}
#[inline(always)]
pub(super) fn lane_fp_cmp(a: u128, b: u128, bits: u32, lanes: u32, op: &str) -> u128 {
    let mut result = 0;
    for i in 0..lanes {
        let sh = i * bits;
        let (x, y) = if bits == 32 {
            (
                f32::from_bits(((a >> sh) & mask(bits)) as u32) as f64,
                f32::from_bits(((b >> sh) & mask(bits)) as u32) as f64,
            )
        } else {
            (
                f64::from_bits(((a >> sh) & mask(bits)) as u64),
                f64::from_bits(((b >> sh) & mask(bits)) as u64),
            )
        };
        let hit = match op {
            "fcmeq" => x == y,
            "fcmgt" => x > y,
            "fcmge" => x >= y,
            _ => unreachable!(),
        };
        if hit {
            result |= mask(bits) << sh;
        }
    }
    result
}
#[inline(always)]
pub(super) fn lane_fp_addp(a: u128, b: u128, bits: u32, lanes: u32) -> u128 {
    // FADDP Vd,T,Vn,Vm: adjacent lane pairs of Vn then Vm.
    let mut result = 0;
    let mut src = a;
    for i in 0..lanes {
        if lanes >= 2 && i == lanes / 2 {
            src = b;
        }
        let base = (i % (lanes / 2)) * 2 * bits;
        let value = if bits == 32 {
            let x = f32::from_bits(((src >> base) & mask(bits)) as u32);
            let y = f32::from_bits(((src >> (base + bits)) & mask(bits)) as u32);
            (x + y).to_bits() as u128
        } else {
            let x = f64::from_bits(((src >> base) & mask(bits)) as u64);
            let y = f64::from_bits(((src >> (base + bits)) & mask(bits)) as u64);
            (x + y).to_bits() as u128
        };
        result |= (value & mask(bits)) << (i * bits);
    }
    result
}
#[inline(always)]
pub(super) fn lane_ushl(a: u128, b: u128, bits: u32, lanes: u32) -> u128 {
    // Variable per-lane shift; signed byte selects direction, out-of-range clears.
    let mut result = 0;
    for i in 0..lanes {
        let sh = i * bits;
        let x = (a >> sh) & mask(bits);
        let s = ((b >> sh) & 0xff) as u8 as i8 as i32;
        let y = if s >= 0 {
            if (s as u32) < bits {
                x << s
            } else {
                0
            }
        } else {
            let r = s.unsigned_abs();
            if r < bits {
                x >> r
            } else {
                0
            }
        };
        result |= (y & mask(bits)) << sh;
    }
    result
}
#[inline(always)]
pub(super) fn lane_rev(a: u128, bits: u32, lanes: u32, group: u32) -> u128 {
    // REV64/REV32/REV16: reverse element order inside each container.
    let mut result = 0;
    let per = group / bits;
    for g in 0..(lanes * bits / group) {
        let chunk = (a >> (g * group)) & mask(group);
        for e in 0..per {
            let elem = (chunk >> (e * bits)) & mask(bits);
            result |= elem << (g * group + (per - 1 - e) * bits);
        }
    }
    result
}
#[inline(always)]
pub(super) fn lane_widen_add(
    a: u128,
    b: u128,
    bits: u32,
    sbits: u32,
    lanes: u32,
    upper: bool,
) -> u128 {
    // UADDW/UADDW2: wide accumulator plus zero-extended source lanes.
    let mut result = 0;
    let src = if upper { b >> 64 } else { b };
    for i in 0..lanes {
        let x = (a >> (i * bits)) & mask(bits);
        let y = (src >> (i * sbits)) & mask(sbits);
        result |= (x.wrapping_add(y) & mask(bits)) << (i * bits);
    }
    result
}
#[inline(always)]
pub(super) fn lane_fp_binary(a: u128, b: u128, bits: u32, lanes: u32, op: &str) -> u128 {
    let mut result = 0;
    for i in 0..lanes {
        let sh = i * bits;
        let x = (a >> sh) & mask(bits);
        let y = (b >> sh) & mask(bits);
        let value = if bits == 32 {
            let f = f32::from_bits(x as u32);
            let g = f32::from_bits(y as u32);
            let v = match op {
                "fmul" => f * g,
                "fadd" => f + g,
                "fsub" => f - g,
                "fdiv" => f / g,
                "fsqrt" => f.sqrt(),
                "fabs" => f.abs(),
                "fneg" => -f,
                "frintp" => f.ceil(),
                "frintm" => f.floor(),
                "frintz" => f.trunc(),
                "frinta" => f.round(),
                _ => unreachable!(),
            };
            v.to_bits() as u128
        } else {
            let f = f64::from_bits(x as u64);
            let g = f64::from_bits(y as u64);
            let v = match op {
                "fmul" => f * g,
                "fadd" => f + g,
                "fsub" => f - g,
                "fdiv" => f / g,
                "fsqrt" => f.sqrt(),
                "fabs" => f.abs(),
                "fneg" => -f,
                "frintp" => f.ceil(),
                "frintm" => f.floor(),
                "frintz" => f.trunc(),
                "frinta" => f.round(),
                _ => unreachable!(),
            };
            v.to_bits() as u128
        };
        result |= (value & mask(bits)) << sh;
    }
    result
}
#[inline(always)]
pub(super) fn lane_int_to_fp(a: u128, bits: u32, lanes: u32, signed: bool) -> u128 {
    let mut result = 0;
    for i in 0..lanes {
        let sh = i * bits;
        let x = (a >> sh) & mask(bits);
        let value = if bits == 32 {
            ((if signed { (x as u32) as i32 as f32 } else { x as u32 as f32 }).to_bits()) as u128
        } else {
            ((if signed { (x as u64) as i64 as f64 } else { x as u64 as f64 }).to_bits()) as u128
        };
        result |= (value & mask(bits)) << sh;
    }
    result
}
#[inline(always)]
pub(super) fn lane_fp_to_int(a: u128, bits: u32, lanes: u32, signed: bool) -> u128 {
    // FCVTZS/FCVTZU saturate and map NaN to zero; Rust `as` casts do exactly this.
    let mut result = 0;
    for i in 0..lanes {
        let sh = i * bits;
        let x = (a >> sh) & mask(bits);
        let value = if bits == 32 {
            let f = f32::from_bits(x as u32);
            (if signed { f as i32 as u32 } else { f as u32 }) as u128
        } else {
            let f = f64::from_bits(x as u64);
            (if signed { f as i64 as u64 } else { f as u64 }) as u128
        };
        result |= (value & mask(bits)) << sh;
    }
    result
}
#[inline(always)]
pub(super) fn lane_fp_cvtl(a: u128, lanes: u32, upper: bool) -> u128 {
    // FCVTL/FCVTL2: widen f32 lanes (low/high half) to f64.
    let src = if upper { a >> 64 } else { a };
    let mut result = 0;
    for i in 0..lanes {
        let f = f32::from_bits(((src >> (i * 32)) & mask(32)) as u32);
        result |= ((f as f64).to_bits() as u128) << (i * 64);
    }
    result
}
#[inline(always)]
pub(super) fn lane_fp_cvtn(d: u128, a: u128, lanes: u32, upper: bool) -> u128 {
    // FCVTN/FCVTN2: narrow f64 lanes to f32, into the low or high half.
    let mut result = d;
    for i in 0..lanes {
        let f = f64::from_bits(((a >> (i * 64)) & mask(64)) as u64);
        let bits32 = (f as f32).to_bits() as u128;
        let at = i * 32 + if upper { 64 } else { 0 };
        result = (result & !(mask(32) << at)) | (bits32 << at);
    }
    result
}
#[inline(always)]
pub(super) fn lane_tbl(
    indices: u128,
    tables: [u128; 4],
    len_bytes: u32,
) -> u128 {
    // TBL: per-byte lookup; out-of-range indices yield zero.
    let mut result = 0;
    for i in 0..16 {
        let idx = ((indices >> (8 * i)) & 0xff) as u32;
        let byte = if idx < len_bytes {
            ((tables[(idx / 16) as usize] >> ((idx % 16) * 8)) & 0xff)
        } else {
            0
        };
        result |= byte << (8 * i);
    }
    result
}
