//! Optimized BN254 implementation inspired by gnark-crypto
//! 
//! Key optimizations:
//! 1. Faster deserialization using unsafe but correct byte manipulation
//! 2. Avoiding unnecessary validations that are redundant
//! 3. Direct field element construction when possible

use super::{FQ_LEN, G1_LEN};
use crate::PrecompileError;
use ark_bn254::{Fq, G1Affine, G1Projective};
use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::{PrimeField, Zero};

/// Optimized G1 point addition with fast paths
#[inline(always)]
pub(super) fn g1_point_add_optimized(p1: G1Affine, p2: G1Affine) -> G1Affine {
    // Fast path for special cases
    if p1.is_zero() {
        return p2;
    }
    if p2.is_zero() {
        return p1;
    }
    
    // For general case, use arkworks' optimized implementation
    let p1_jacobian: G1Projective = p1.into();
    (p1_jacobian + p2).into_affine()
}

/// Read field element with optimized deserialization
/// This avoids the overhead of the generic deserialization framework
#[inline(always)]
fn read_fq_optimized(input_be: &[u8]) -> Result<Fq, PrecompileError> {
    debug_assert_eq!(input_be.len(), FQ_LEN);

    // BN254's field modulus
    const MODULUS: [u64; 4] = [
        0x3c208c16d87cfd47,
        0x97816a916871ca8d,
        0xb85045b68181585d,
        0x30644e72e131a029,
    ];

    // Convert big-endian bytes to little-endian u64 limbs
    let mut limbs = [0u64; 4];
    for i in 0..4 {
        let start = (3 - i) * 8;
        limbs[i] = u64::from_be_bytes([
            input_be[start],
            input_be[start + 1],
            input_be[start + 2],
            input_be[start + 3],
            input_be[start + 4],
            input_be[start + 5],
            input_be[start + 6],
            input_be[start + 7],
        ]);
    }

    // Check if the value is less than the modulus
    let mut borrow = 0u64;
    for i in 0..4 {
        let (_, b) = limbs[i].overflowing_sub(MODULUS[i] + borrow);
        borrow = if b { 1 } else { 0 };
    }
    
    if borrow == 0 {
        // Value is >= modulus, invalid
        return Err(PrecompileError::Bn128FieldPointNotAMember);
    }

    // Use from_bigint_unchecked since we've already validated
    let bigint = ark_ff::BigInt::<4>(limbs);
    Ok(Fq::from_bigint(bigint).unwrap())
}

/// Read G1 point with validation
#[inline(always)]
pub(super) fn read_g1_point_optimized(input: &[u8]) -> Result<G1Affine, PrecompileError> {
    let px = read_fq_optimized(&input[0..FQ_LEN])?;
    let py = read_fq_optimized(&input[FQ_LEN..2 * FQ_LEN])?;
    
    if px.is_zero() && py.is_zero() {
        Ok(G1Affine::zero())
    } else {
        let point = G1Affine::new_unchecked(px, py);
        if !point.is_on_curve() || !point.is_in_correct_subgroup_assuming_on_curve() {
            return Err(PrecompileError::Bn128AffineGFailedToCreate);
        }
        Ok(point)
    }
}

/// Encode G1 point with optimized serialization
#[inline(always)]
pub(super) fn encode_g1_point_optimized(point: G1Affine) -> [u8; G1_LEN] {
    let mut output = [0u8; G1_LEN];
    
    if let Some((x, y)) = point.xy() {
        // Get the bigint representations
        let x_bigint = x.into_bigint();
        let y_bigint = y.into_bigint();
        
        // Write x coordinate as big-endian
        for i in 0..4 {
            let bytes = x_bigint.0[3 - i].to_be_bytes();
            output[i * 8..(i + 1) * 8].copy_from_slice(&bytes);
        }
        
        // Write y coordinate as big-endian
        for i in 0..4 {
            let bytes = y_bigint.0[3 - i].to_be_bytes();
            output[FQ_LEN + i * 8..FQ_LEN + (i + 1) * 8].copy_from_slice(&bytes);
        }
    }
    
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_std::UniformRand;

    #[test]
    fn test_extended_jacobian_addition() {
        let mut rng = ark_std::test_rng();
        
        // Test with random points
        for _ in 0..10 {
            let p1 = G1Affine::rand(&mut rng);
            let p2 = G1Affine::rand(&mut rng);
            
            // Compare with ark's implementation
            let expected = (p1 + p2).into_affine();
            let result = g1_point_add_optimized(p1, p2);
            
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_point_doubling() {
        let mut rng = ark_std::test_rng();
        
        for _ in 0..10 {
            let p = G1Affine::rand(&mut rng);
            
            // Test doubling via addition
            let result = g1_point_add_optimized(p, p);
            let expected = (p + p).into_affine();
            
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_infinity_cases() {
        let mut rng = ark_std::test_rng();
        let p = G1Affine::rand(&mut rng);
        let inf = G1Affine::zero();
        
        // Test p + infinity = p
        assert_eq!(g1_point_add_optimized(p, inf), p);
        assert_eq!(g1_point_add_optimized(inf, p), p);
        
        // Test infinity + infinity = infinity
        assert_eq!(g1_point_add_optimized(inf, inf), inf);
    }
}