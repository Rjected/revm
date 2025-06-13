//! Optimized BN254 implementation inspired by gnark-crypto
//! This module implements efficient point addition using techniques from gnark:
//! - Extended Jacobian coordinates (X, Y, ZZ, ZZZ) where ZZ = Z² and ZZZ = Z³
//! - Mixed addition for affine + jacobian points
//! - Optimized field operations

use super::{FQ_LEN, G1_LEN};
use crate::PrecompileError;
use ark_bn254::{Fq, G1Affine};
use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::{AdditiveGroup, Field, One, Zero};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};

/// Extended Jacobian coordinates for more efficient point operations
/// Coordinates: (X, Y, ZZ, ZZZ) where ZZ = Z² and ZZZ = Z³
#[derive(Clone, Debug)]
pub struct G1JacobianExtended {
    x: Fq,
    y: Fq,
    zz: Fq,
    zzz: Fq,
}

impl G1JacobianExtended {
    /// Create a new point in extended Jacobian coordinates
    fn new(x: Fq, y: Fq, zz: Fq, zzz: Fq) -> Self {
        Self { x, y, zz, zzz }
    }

    /// Convert from affine coordinates to extended Jacobian
    fn from_affine(p: &G1Affine) -> Self {
        if p.is_zero() {
            Self {
                x: Fq::zero(),
                y: Fq::one(),
                zz: Fq::zero(),
                zzz: Fq::zero(),
            }
        } else {
            let (x, y) = p.xy().unwrap();
            Self {
                x,
                y,
                zz: Fq::one(),
                zzz: Fq::one(),
            }
        }
    }

    /// Convert back to affine coordinates
    fn to_affine(&self) -> G1Affine {
        if self.zz.is_zero() {
            G1Affine::zero()
        } else {
            // x = X/ZZ, y = Y/ZZZ
            let zz_inv = self.zz.inverse().unwrap();
            let zzz_inv = self.zzz.inverse().unwrap();
            
            let x = self.x * zz_inv;
            let y = self.y * zzz_inv;
            
            G1Affine::new_unchecked(x, y)
        }
    }

    /// Check if point is at infinity
    fn is_infinity(&self) -> bool {
        self.zz.is_zero()
    }

    /// Mixed addition: self + affine point
    /// This is more efficient when one point is in affine coordinates
    /// Based on: http://www.hyperelliptic.org/EFD/g1p/auto-shortw-xyzz.html#addition-madd-2008-s
    fn add_mixed(&mut self, a: &G1Affine) {
        // If a is infinity, return self
        if a.is_zero() {
            return;
        }

        // If self is infinity, set to a
        if self.is_infinity() {
            *self = Self::from_affine(a);
            return;
        }

        let (ax, ay) = a.xy().unwrap();

        // P = ax * ZZ - X
        let mut p = ax * &self.zz;
        p -= &self.x;

        // R = ay * ZZZ - Y
        let mut r = ay * &self.zzz;
        r -= &self.y;

        // Check if points are equal or negatives
        if p.is_zero() {
            if r.is_zero() {
                // Points are equal, double instead
                self.double_mixed(a);
                return;
            } else {
                // Points are negatives, result is infinity
                self.zz = Fq::zero();
                self.zzz = Fq::zero();
                return;
            }
        }

        // Compute the addition
        let pp = p.square();
        let ppp = p * &pp;
        let q = self.x * &pp;
        let rr = r.square();
        
        // X3 = R² - PPP - 2Q
        let mut x3 = rr;
        x3 -= &ppp;
        x3 -= &q;
        x3 -= &q;

        // Y3 = R(Q - X3) - Y*PPP
        let mut y3 = q;
        y3 -= &x3;
        y3 *= &r;
        let y_ppp = self.y * &ppp;
        y3 -= &y_ppp;

        // Update coordinates
        self.x = x3;
        self.y = y3;
        self.zz *= &pp;
        self.zzz *= &ppp;
    }

    /// Double a point when it's in affine coordinates
    /// More efficient than general doubling
    fn double_mixed(&mut self, a: &G1Affine) {
        if a.is_zero() {
            return;
        }

        let (ax, ay) = a.xy().unwrap();

        // Following https://www.hyperelliptic.org/EFD/g1p/auto-shortw-xyzz.html#doubling-dbl-2008-s-1
        let u = ay.double();
        let v = u.square();
        let w = u * &v;
        let s = ax * &v;
        let xx = ax.square();
        let m = xx.double() + &xx; // 3*X² for BN254 (a=0)
        
        let u_w_y = w * ay;

        // X3 = M² - 2S
        let mut x3 = m.square();
        x3 -= &s;
        x3 -= &s;

        // Y3 = M(S - X3) - U*W*Y
        let mut y3 = s;
        y3 -= &x3;
        y3 *= &m;
        y3 -= &u_w_y;

        self.x = x3;
        self.y = y3;
        self.zz = v;
        self.zzz = w;
    }
}

/// Optimized G1 point addition using extended Jacobian coordinates
pub fn g1_point_add_optimized(p1: G1Affine, p2: G1Affine) -> G1Affine {
    // Handle special cases
    if p1.is_zero() {
        return p2;
    }
    if p2.is_zero() {
        return p1;
    }

    // Check if points are equal
    if p1 == p2 {
        // Use doubling which is more efficient
        let mut result = G1JacobianExtended::from_affine(&p1);
        result.double_mixed(&p1);
        return result.to_affine();
    }

    // Use mixed addition for efficiency
    // Convert p1 to extended Jacobian and add p2 as affine
    let mut result = G1JacobianExtended::from_affine(&p1);
    result.add_mixed(&p2);
    result.to_affine()
}

/// Batch point addition using Montgomery batch inversion
/// This is more efficient when adding multiple points
pub fn g1_point_add_batch(points: &[(G1Affine, G1Affine)]) -> Vec<G1Affine> {
    if points.is_empty() {
        return vec![];
    }

    // For now, just use individual additions
    // A full implementation would use batch inversion for coordinate conversions
    points.iter()
        .map(|(p1, p2)| g1_point_add_optimized(*p1, *p2))
        .collect()
}

/// Read field element with optimized deserialization
#[inline]
fn read_fq_optimized(input_be: &[u8]) -> Result<Fq, PrecompileError> {
    debug_assert_eq!(input_be.len(), FQ_LEN);

    // Convert from big-endian to little-endian
    let mut input_le = [0u8; FQ_LEN];
    for i in 0..FQ_LEN {
        input_le[i] = input_be[FQ_LEN - 1 - i];
    }

    Fq::deserialize_uncompressed(&input_le[..])
        .map_err(|_| PrecompileError::Bn128FieldPointNotAMember)
}

/// Read G1 point with validation
#[inline]
pub fn read_g1_point_optimized(input: &[u8]) -> Result<G1Affine, PrecompileError> {
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
#[inline]
pub fn encode_g1_point_optimized(point: G1Affine) -> [u8; G1_LEN] {
    let mut output = [0u8; G1_LEN];
    
    if let Some((x, y)) = point.xy() {
        // Serialize x coordinate
        x.serialize_uncompressed(&mut output[0..FQ_LEN])
            .expect("Failed to serialize x coordinate");
        
        // Serialize y coordinate
        y.serialize_uncompressed(&mut output[FQ_LEN..G1_LEN])
            .expect("Failed to serialize y coordinate");
        
        // Convert to big-endian
        output[0..FQ_LEN].reverse();
        output[FQ_LEN..G1_LEN].reverse();
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