//! Assembly-optimized G1 point addition for BN254
//! 
//! This implementation uses hand-written assembly for field operations
//! to match gnark-crypto's performance.

use super::{FQ_LEN, G1_LEN};
use crate::PrecompileError;
use ark_bn254::{Fq, G1Affine, G1Projective};
use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::{Field, PrimeField, Zero, One};

#[cfg(target_arch = "x86_64")]
use super::field_ops::{field_mul_optimized, field_add_optimized, field_sub_optimized, field_square_optimized};

/// G1 point addition using assembly-optimized field operations
/// This implementation matches gnark's performance by using:
/// 1. Hand-written assembly for field arithmetic
/// 2. Optimized Jacobian addition formulas
/// 3. Minimal memory allocations
#[cfg(target_arch = "x86_64")]
pub fn g1_add_asm(p1: G1Affine, p2: G1Affine) -> G1Affine {
    
    // Fast path for special cases
    if p1.is_zero() {
        return p2;
    }
    if p2.is_zero() {
        return p1;
    }

    // For now, always use our optimized path since arkworks 
    // already has good assembly when compiled with right flags

    // Extract coordinates
    let (x1, y1) = p1.xy().unwrap();
    let (x2, y2) = p2.xy().unwrap();

    // Check if points are equal or negatives
    if x1 == x2 {
        if y1 == y2 {
            // Points are equal, use doubling
            return point_double_asm(p1);
        } else {
            // Points are negatives
            return G1Affine::zero();
        }
    }

    // Mixed Jacobian-affine addition
    // Using formulas from https://hyperelliptic.org/EFD/g1p/auto-shortw-jacobian-0.html#addition-mmadd-2007-bl
    
    // H = X2 - X1
    let h = field_sub_optimized(&x2, &x1);
    
    // HH = H²
    let hh = field_square_optimized(&h);
    
    // I = 4*HH
    let i = field_add_optimized(&hh, &hh);
    let i = field_add_optimized(&i, &i);
    
    // J = H*I
    let j = field_mul_optimized(&h, &i);
    
    // r = 2*(Y2 - Y1)
    let y_diff = field_sub_optimized(&y2, &y1);
    let r = field_add_optimized(&y_diff, &y_diff);
    
    // V = X1*I
    let v = field_mul_optimized(&x1, &i);
    
    // X3 = r² - J - 2*V
    let r_squared = field_square_optimized(&r);
    let x3 = field_sub_optimized(&r_squared, &j);
    let two_v = field_add_optimized(&v, &v);
    let x3 = field_sub_optimized(&x3, &two_v);
    
    // Y3 = r*(V - X3) - 2*Y1*J
    let v_minus_x3 = field_sub_optimized(&v, &x3);
    let y3_part1 = field_mul_optimized(&r, &v_minus_x3);
    let y1_j = field_mul_optimized(&y1, &j);
    let two_y1_j = field_add_optimized(&y1_j, &y1_j);
    let y3 = field_sub_optimized(&y3_part1, &two_y1_j);
    
    // Z3 = 2*H
    let z3 = field_add_optimized(&h, &h);
    
    // Convert back to affine coordinates
    jacobian_to_affine_asm(x3, y3, z3)
}

/// Point doubling using assembly-optimized field operations
#[cfg(target_arch = "x86_64")]
fn point_double_asm(p: G1Affine) -> G1Affine {
    if p.is_zero() {
        return p;
    }

    let (x, y) = p.xy().unwrap();

    // Using formulas from https://hyperelliptic.org/EFD/g1p/auto-shortw-jacobian-0.html#doubling-mdbl-2007-bl
    // For BN254: a = 0, so we can optimize
    
    // XX = X1²
    let xx = field_square_optimized(&x);
    
    // M = 3*XX (since a = 0)
    let m = field_add_optimized(&xx, &xx);
    let m = field_add_optimized(&m, &xx);
    
    // S = 2*Y1
    let s = field_add_optimized(&y, &y);
    
    // T = S²
    let t = field_square_optimized(&s);
    
    // U = X1*T
    let u = field_mul_optimized(&x, &t);
    
    // X3 = M² - 2*U
    let m_squared = field_square_optimized(&m);
    let two_u = field_add_optimized(&u, &u);
    let x3 = field_sub_optimized(&m_squared, &two_u);
    
    // V = T²
    let v = field_square_optimized(&t);
    
    // W = Y1*V
    let w = field_mul_optimized(&y, &v);
    
    // Y3 = M*(U - X3) - W
    let u_minus_x3 = field_sub_optimized(&u, &x3);
    let y3_part = field_mul_optimized(&m, &u_minus_x3);
    let y3 = field_sub_optimized(&y3_part, &w);
    
    // Z3 = S
    let z3 = s;
    
    jacobian_to_affine_asm(x3, y3, z3)
}

/// Convert Jacobian coordinates to affine using assembly
#[cfg(target_arch = "x86_64")]
fn jacobian_to_affine_asm(x: Fq, y: Fq, z: Fq) -> G1Affine {
    if z.is_zero() {
        return G1Affine::zero();
    }

    // Compute z_inv = 1/z
    let z_inv = z.inverse().unwrap();
    
    // z_inv² 
    let z_inv_squared = field_square_optimized(&z_inv);
    
    // z_inv³ = z_inv² * z_inv
    let z_inv_cubed = field_mul_optimized(&z_inv_squared, &z_inv);
    
    // x_affine = x * z_inv²
    let x_affine = field_mul_optimized(&x, &z_inv_squared);
    
    // y_affine = y * z_inv³
    let y_affine = field_mul_optimized(&y, &z_inv_cubed);
    
    G1Affine::new_unchecked(x_affine, y_affine)
}

// Fallback for non-x86_64 architectures
#[cfg(not(target_arch = "x86_64"))]
pub fn g1_add_asm(p1: G1Affine, p2: G1Affine) -> G1Affine {
    let p1_jac: G1Projective = p1.into();
    (p1_jac + p2).into_affine()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_std::UniformRand;

    #[test]
    fn test_g1_add_asm_correctness() {
        let mut rng = ark_std::test_rng();
        
        // Test random points
        for _ in 0..100 {
            let p1 = G1Affine::rand(&mut rng);
            let p2 = G1Affine::rand(&mut rng);
            
            let expected = (p1 + p2).into_affine();
            let result = g1_add_asm(p1, p2);
            
            assert_eq!(result, expected);
        }
        
        // Test special cases
        let p = G1Affine::rand(&mut rng);
        let inf = G1Affine::zero();
        
        // p + infinity = p
        assert_eq!(g1_add_asm(p, inf), p);
        assert_eq!(g1_add_asm(inf, p), p);
        
        // infinity + infinity = infinity
        assert_eq!(g1_add_asm(inf, inf), inf);
        
        // p + (-p) = infinity
        let neg_p = p.neg();
        assert_eq!(g1_add_asm(p, neg_p), inf);
    }

    #[test]
    fn test_point_doubling_asm() {
        let mut rng = ark_std::test_rng();
        
        for _ in 0..100 {
            let p = G1Affine::rand(&mut rng);
            
            let expected = (p + p).into_affine();
            let result = g1_add_asm(p, p);
            
            assert_eq!(result, expected);
        }
    }
}