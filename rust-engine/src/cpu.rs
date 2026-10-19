//! Architectural values at recovered function boundaries, kept for exact audits.
#[derive(Clone, Debug)]
pub struct Cpu {
    pub x: [u64; 31],
    pub q: [u128; 32],
    pub sp: u64,
    pub pc: u64,
    pub nzcv: u64,
    pub tpidr_el0: u64,
    pub fpcr: u64,
    pub fpsr: u64,
    pub cpacr_el1: u64,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            x: [0; 31],
            q: [0; 32],
            sp: 0,
            pc: 0,
            nzcv: 0,
            tpidr_el0: 0,
            fpcr: 0,
            fpsr: 0,
            cpacr_el1: 0,
        }
    }
}
impl Cpu {
    #[inline(always)]
    pub fn condition(&self, condition: u8) -> bool {
        let n = self.nzcv & (1 << 31) != 0;
        let z = self.nzcv & (1 << 30) != 0;
        let c = self.nzcv & (1 << 29) != 0;
        let v = self.nzcv & (1 << 28) != 0;
        match condition {
            0 => z,
            1 => !z,
            2 => c,
            3 => !c,
            4 => n,
            5 => !n,
            6 => v,
            7 => !v,
            8 => c && !z,
            9 => !c || z,
            10 => n == v,
            11 => n != v,
            12 => !z && n == v,
            13 => z || n != v,
            14 | 15 => true,
            _ => unreachable!(),
        }
    }
    #[inline(always)]
    pub fn flags(&mut self, a: u64, b: u64, result: u64, bits: u32, subtract: bool) {
        let mask = if bits == 32 {
            u32::MAX as u64
        } else {
            u64::MAX
        };
        let a = a & mask;
        let b = b & mask;
        let r = result & mask;
        let carry = if subtract {
            a >= b
        } else {
            (a as u128 + b as u128) >> (bits) != 0
        };
        let overflow = if subtract {
            ((a ^ b) & (a ^ r)) >> (bits - 1) & 1
        } else {
            (!(a ^ b) & (a ^ r)) >> (bits - 1) & 1
        };
        self.nzcv = ((r >> (bits - 1)) << 31)
            | ((r == 0) as u64) << 30
            | (carry as u64) << 29
            | overflow << 28;
    }
    #[inline(always)]
    pub fn logical_flags(&mut self, result: u64, bits: u32) {
        let r = if bits == 32 {
            result as u32 as u64
        } else {
            result
        };
        self.nzcv = ((r >> (bits - 1)) << 31) | ((r == 0) as u64) << 30;
    }
}
