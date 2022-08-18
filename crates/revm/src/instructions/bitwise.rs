use core::cmp::Ordering;

use super::i256::{i256_cmp, i256_sign, two_compl, Sign};
use ruint::Uint;

pub fn slt(op1: Uint<256, 4>, op2: Uint<256, 4>) -> Uint<256, 4> {
    if i256_cmp(op1, op2) == Ordering::Less {
        Uint::from(1)
    } else {
        Uint::ZERO
    }
}

pub fn sgt(op1: Uint<256, 4>, op2: Uint<256, 4>) -> Uint<256, 4> {
    if i256_cmp(op1, op2) == Ordering::Greater {
        Uint::from(1)
    } else {
        Uint::ZERO
    }
}

pub fn iszero(op1: Uint<256, 4>) -> Uint<256, 4> {
    if op1 == Uint::ZERO {
        Uint::from(1)
    } else {
        Uint::ZERO
    }
}

pub fn not(op1: Uint<256, 4>) -> Uint<256, 4> {
    !op1
}

pub fn byte(op1: Uint<256, 4>, op2: Uint<256, 4>) -> Uint<256, 4> {
    let mut ret = Uint::ZERO;

    for i in 0..256 {
        if i < 8 && op1 < Uint::from(32) {
            let o: usize = u64::from_le_bytes(op1.to_le_bytes()) as usize;
            let t = 255 - (7 - i + 8 * o);
            let bit_mask = Uint::from(1) << t;
            let value = (op2 & bit_mask) >> t;
            ret = ret.overflowing_add(value << i).0;
        }
    }

    ret
}

pub fn shl(shift: Uint<256, 4>, value: Uint<256, 4>) -> Uint<256, 4> {
    if value == Uint::ZERO || shift >= Uint::from(256) {
        Uint::ZERO
    } else {
        // TODO: is there an easier or faster way to convert from a (known < 256) Uint<256, 4> into a u64?
        let shift: u64 = u64::from_le_bytes(shift.to_le_bytes());
        value << shift as usize
    }
}

pub fn shr(shift: Uint<256, 4>, value: Uint<256, 4>) -> Uint<256, 4> {
    if value == Uint::ZERO || shift >= Uint::from(256) {
        Uint::ZERO
    } else {
        // TODO: is there an easier or faster way to convert from a (known < 256) Uint<256, 4> into a u64?
        let shift: u64 = u64::from_le_bytes(shift.to_le_bytes());
        value >> shift as usize
    }
}

pub fn sar(shift: Uint<256, 4>, mut value: Uint<256, 4>) -> Uint<256, 4> {
    let value_sign = i256_sign::<true>(&mut value);

    if value == Uint::ZERO || shift >= Uint::from(256) {
        match value_sign {
            // value is 0 or >=1, pushing 0
            Sign::Plus | Sign::Zero => Uint::ZERO,
            // value is <0, pushing -1
            Sign::Minus => two_compl(Uint::from(1)),
        }
    } else {
        // TODO: is there an easier or faster way to convert from a (known < 256) Uint<256, 4> into a u64?
        let shift: u64 = u64::from_le_bytes(shift.to_le_bytes());

        match value_sign {
            Sign::Plus | Sign::Zero => value >> shift as usize,
            Sign::Minus => {
                let shifted = ((value.overflowing_sub(Uint::from(1)).0) >> shift as usize)
                    .overflowing_add(Uint::from(1))
                    .0;
                two_compl(shifted)
            }
        }
    }
}
