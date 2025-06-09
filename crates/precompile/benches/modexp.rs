//! Benchmarks for the modexp precompile
use criterion::{measurement::Measurement, BenchmarkGroup};
use primitives::{hex, Bytes, U256};
use revm_precompile::modexp::{berlin_run, byzantium_run, osaka_run};

/// Helper function to create modexp input
fn create_modexp_input(base: &[u8], exponent: &[u8], modulus: &[u8]) -> Bytes {
    let mut input = vec![0u8; 96];
    
    // Write lengths as 32-byte padded values
    let base_len = U256::from(base.len());
    let exp_len = U256::from(exponent.len());
    let mod_len = U256::from(modulus.len());
    
    input[0..32].copy_from_slice(&base_len.to_be_bytes::<32>());
    input[32..64].copy_from_slice(&exp_len.to_be_bytes::<32>());
    input[64..96].copy_from_slice(&mod_len.to_be_bytes::<32>());
    
    // Append the actual values
    input.extend_from_slice(base);
    input.extend_from_slice(exponent);
    input.extend_from_slice(modulus);
    
    Bytes::from(input)
}

/// Add benches for the modexp precompile
pub fn add_benches<M: Measurement>(group: &mut BenchmarkGroup<'_, M>) {
    // Small modexp: 32-byte values
    let small_base = hex::decode("1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef").unwrap();
    let small_exp = hex::decode("0000000000000000000000000000000000000000000000000000000000000003").unwrap();
    let small_mod = hex::decode("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap();
    let small_input = create_modexp_input(&small_base, &small_exp, &small_mod);
    
    group.bench_function("modexp small (32-byte) - byzantium", |b| {
        b.iter(|| byzantium_run(&small_input, u64::MAX).unwrap())
    });
    
    group.bench_function("modexp small (32-byte) - berlin", |b| {
        b.iter(|| berlin_run(&small_input, u64::MAX).unwrap())
    });
    
    group.bench_function("modexp small (32-byte) - osaka", |b| {
        b.iter(|| osaka_run(&small_input, u64::MAX).unwrap())
    });
    
    // Medium modexp: 128-byte values
    let medium_base = vec![0xAB; 128];
    let medium_exp = vec![0x02; 128];
    let medium_mod = vec![0xFF; 128];
    let medium_input = create_modexp_input(&medium_base, &medium_exp, &medium_mod);
    
    group.bench_function("modexp medium (128-byte) - byzantium", |b| {
        b.iter(|| byzantium_run(&medium_input, u64::MAX).unwrap())
    });
    
    group.bench_function("modexp medium (128-byte) - berlin", |b| {
        b.iter(|| berlin_run(&medium_input, u64::MAX).unwrap())
    });
    
    group.bench_function("modexp medium (128-byte) - osaka", |b| {
        b.iter(|| osaka_run(&medium_input, u64::MAX).unwrap())
    });
    
    // Large modexp: 256-byte values
    let large_base = vec![0xDE; 256];
    let large_exp = vec![0x03; 256];
    let large_mod = vec![0xEF; 256];
    let large_input = create_modexp_input(&large_base, &large_exp, &large_mod);
    
    group.bench_function("modexp large (256-byte) - byzantium", |b| {
        b.iter(|| byzantium_run(&large_input, u64::MAX).unwrap())
    });
    
    group.bench_function("modexp large (256-byte) - berlin", |b| {
        b.iter(|| berlin_run(&large_input, u64::MAX).unwrap())
    });
    
    group.bench_function("modexp large (256-byte) - osaka", |b| {
        b.iter(|| osaka_run(&large_input, u64::MAX).unwrap())
    });
    
    // Edge case: Very large exponent (worst case for gas calculation)
    let worst_base = vec![0xFF; 32];
    let worst_exp = vec![0xFF; 512];  // Large exponent
    let worst_mod = vec![0xFF; 32];
    let worst_input = create_modexp_input(&worst_base, &worst_exp, &worst_mod);
    
    group.bench_function("modexp worst case (512-byte exp) - byzantium", |b| {
        b.iter(|| byzantium_run(&worst_input, u64::MAX).unwrap())
    });
    
    group.bench_function("modexp worst case (512-byte exp) - berlin", |b| {
        b.iter(|| berlin_run(&worst_input, u64::MAX).unwrap())
    });
    
    group.bench_function("modexp worst case (512-byte exp) - osaka", |b| {
        b.iter(|| osaka_run(&worst_input, u64::MAX).unwrap())
    });
    
    // Real-world case: RSA-2048 style operation
    let rsa_base = vec![0x42; 256];  // 2048 bits
    let rsa_exp = hex::decode("010001").unwrap();  // Common RSA exponent (65537)
    let rsa_mod = vec![0xFF; 256];  // 2048-bit modulus
    let rsa_input = create_modexp_input(&rsa_base, &rsa_exp, &rsa_mod);
    
    group.bench_function("modexp RSA-2048 style - byzantium", |b| {
        b.iter(|| byzantium_run(&rsa_input, u64::MAX).unwrap())
    });
    
    group.bench_function("modexp RSA-2048 style - berlin", |b| {
        b.iter(|| berlin_run(&rsa_input, u64::MAX).unwrap())
    });
    
    group.bench_function("modexp RSA-2048 style - osaka", |b| {
        b.iter(|| osaka_run(&rsa_input, u64::MAX).unwrap())
    });
}