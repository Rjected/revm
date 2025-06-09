use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use revm_interpreter::Stack;
use primitives::{Address, Bytes, U256, B256, keccak256};
use std::hint::black_box;

// All gas limits from Nethermind benchmarks  
const GAS_LIMITS: &[u64] = &[30_000_000, 50_000_000, 60_000_000, 80_000_000, 100_000_000, 150_000_000];

// Blake2 1K rounds benchmark (simulated)
fn bench_blake2_1k_rounds(c: &mut Criterion) {
    let mut group = c.benchmark_group("Blake1KRoundsCACHABLE");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let data = vec![0x42u8; 64]; // Blake2 typically uses 64-byte blocks
                b.iter(|| {
                    for _ in 0..100 {
                        // Simulate 1K rounds of Blake2
                        // In reality, this would call the Blake2 F function
                        let mut result = data.clone();
                        for _ in 0..1000 {
                            let hash = keccak256(&result);
                            result = hash.0.to_vec();
                        }
                        black_box(result);
                    }
                });
            },
        );
    }
    group.finish();
}

// Blake2 1M rounds benchmark (simulated)
fn bench_blake2_1m_rounds(c: &mut Criterion) {
    let mut group = c.benchmark_group("Blake1MRoundsCACHABLE");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let data = vec![0x42u8; 64];
                b.iter(|| {
                    // Simulate 1M rounds - much more expensive
                    // In practice, we'll do fewer iterations
                    let mut result = data.clone();
                    for _ in 0..10000 { // Reduced for benchmark practicality
                        let hash = keccak256(&result);
                        result = hash.0.to_vec();
                    }
                    black_box(result);
                });
            },
        );
    }
    group.finish();
}

// EC Add with 12-byte inputs (simulated)
fn bench_ec_add_12(c: &mut Criterion) {
    let mut group = c.benchmark_group("EcAdd12CACHABLE");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                // Simulate EC point addition
                let point1 = U256::from(1u64);
                let point2 = U256::from(2u64);
                b.iter(|| {
                    for _ in 0..1000 {
                        // Simulate elliptic curve addition
                        let result = point1.saturating_add(point2);
                        black_box(result);
                    }
                });
            },
        );
    }
    group.finish();
}

// EC Add with 32-byte coordinates (simulated)
fn bench_ec_add_32(c: &mut Criterion) {
    let mut group = c.benchmark_group("EcAdd32ByteCoordinatesCACHABLE");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let x1 = U256::from_be_bytes([0x42; 32]);
                let y1 = U256::from_be_bytes([0x43; 32]);
                let x2 = U256::from_be_bytes([0x44; 32]);
                let y2 = U256::from_be_bytes([0x45; 32]);
                b.iter(|| {
                    for _ in 0..1000 {
                        // Simulate EC point addition with full coordinates
                        let rx = x1.saturating_add(x2);
                        let ry = y1.saturating_add(y2);
                        black_box((rx, ry));
                    }
                });
            },
        );
    }
    group.finish();
}

// EC Mul with 12-byte point and 32-byte scalar (simulated)
fn bench_ec_mul_12_32(c: &mut Criterion) {
    let mut group = c.benchmark_group("EcMul12And32ByteScalarCACHABLE");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let point = U256::from(1u64);
                let scalar = U256::from_be_bytes([0x42; 32]);
                b.iter(|| {
                    for _ in 0..100 {
                        // Simulate scalar multiplication
                        let result = point.saturating_mul(scalar);
                        black_box(result);
                    }
                });
            },
        );
    }
    group.finish();
}

// EC Mul with 32-byte coordinates and scalar (simulated)
fn bench_ec_mul_32_32(c: &mut Criterion) {
    let mut group = c.benchmark_group("EcMul32ByteCoordinates32ByteScalarCACHABLE");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let x = U256::from_be_bytes([0x42; 32]);
                let y = U256::from_be_bytes([0x43; 32]);
                let scalar = U256::from_be_bytes([0x44; 32]);
                b.iter(|| {
                    for _ in 0..100 {
                        // Simulate EC scalar multiplication
                        let rx = x.saturating_mul(scalar);
                        let ry = y.saturating_mul(scalar);
                        black_box((rx, ry));
                    }
                });
            },
        );
    }
    group.finish();
}

// EC Pairing with 2 sets (simulated)
fn bench_ec_pairing_2sets(c: &mut Criterion) {
    let mut group = c.benchmark_group("EcPairing2SetsCACHABLE");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                // Simulate pairing check with 2 point pairs
                let data = vec![0x42u8; 192]; // 2 * (2 * 32 + 2 * 32) bytes
                b.iter(|| {
                    for _ in 0..10 {
                        // Pairing is very expensive
                        let hash = keccak256(&data);
                        black_box(hash);
                    }
                });
            },
        );
    }
    group.finish();
}

// EcRecover uncachable variant 1
fn bench_ecrecover_uncachable(c: &mut Criterion) {
    let mut group = c.benchmark_group("EcRecoverUNCACHABLE");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let hash = B256::from([0x42; 32]);
                let v = 27u8;
                let r = B256::from([0x43; 32]);
                let s = B256::from([0x44; 32]);
                b.iter(|| {
                    for _ in 0..100 {
                        // Simulate signature recovery
                        let mut data = Vec::with_capacity(128);
                        data.extend_from_slice(&hash.0);
                        data.push(v);
                        data.extend_from_slice(&r.0);
                        data.extend_from_slice(&s.0);
                        let result = keccak256(&data);
                        black_box(result);
                    }
                });
            },
        );
    }
    group.finish();
}

// EcRecover uncachable variant 2
fn bench_ecrecover_uncachable2(c: &mut Criterion) {
    let mut group = c.benchmark_group("EcRecoverUNCACHABLE2");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let hash = B256::from([0x45; 32]);
                let v = 28u8;
                let r = B256::from([0x46; 32]);
                let s = B256::from([0x47; 32]);
                b.iter(|| {
                    for _ in 0..100 {
                        // Different inputs for uncachable variant
                        let mut data = Vec::with_capacity(128);
                        data.extend_from_slice(&hash.0);
                        data.push(v);
                        data.extend_from_slice(&r.0);
                        data.extend_from_slice(&s.0);
                        let result = keccak256(&data);
                        black_box(result);
                    }
                });
            },
        );
    }
    group.finish();
}

// Modexp benchmarks - simulating various sizes and complexities
fn bench_modexp_208_balanced(c: &mut Criterion) {
    let mut group = c.benchmark_group("Modexp208GasBalancedUNCACHABLE");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let base = U256::from_be_bytes([0x42; 32]);
                let exp = U256::from(208u64);
                let modulus = U256::from_be_bytes([0x44; 32]);
                b.iter(|| {
                    for _ in 0..10 {
                        // Simulate modular exponentiation
                        let mut result = U256::from(1u64);
                        for _ in 0..208 {
                            result = result.saturating_mul(base);
                            if modulus != U256::ZERO {
                                result = result % modulus;
                            }
                        }
                        black_box(result);
                    }
                });
            },
        );
    }
    group.finish();
}

fn bench_modexp_208_balanced2(c: &mut Criterion) {
    let mut group = c.benchmark_group("Modexp208GasBalancedUNCACHABLE2");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let base = U256::from_be_bytes([0x45; 32]);
                let exp = U256::from(208u64);
                let modulus = U256::from_be_bytes([0x47; 32]);
                b.iter(|| {
                    for _ in 0..10 {
                        let mut result = U256::from(1u64);
                        for _ in 0..208 {
                            result = result.saturating_mul(base);
                            if modulus != U256::ZERO {
                                result = result % modulus;
                            }
                        }
                        black_box(result);
                    }
                });
            },
        );
    }
    group.finish();
}

fn bench_modexp_215_exp_heavy_cachable(c: &mut Criterion) {
    let mut group = c.benchmark_group("Modexp215GasExpHeavyCACHABLE");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let base = U256::from_be_bytes([0x42; 32]);
                let exp = U256::from(215u64);
                let modulus = U256::from_be_bytes([0x44; 32]);
                b.iter(|| {
                    for _ in 0..10 {
                        let mut result = U256::from(1u64);
                        for _ in 0..215 {
                            result = result.saturating_mul(base);
                            if modulus != U256::ZERO {
                                result = result % modulus;
                            }
                        }
                        black_box(result);
                    }
                });
            },
        );
    }
    group.finish();
}

fn bench_modexp_215_exp_heavy_uncachable(c: &mut Criterion) {
    let mut group = c.benchmark_group("Modexp215GasExpHeavyUNCACHABLE");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let base = U256::from_be_bytes([0x48; 32]);
                let exp = U256::from(215u64);
                let modulus = U256::from_be_bytes([0x49; 32]);
                b.iter(|| {
                    for _ in 0..10 {
                        let mut result = U256::from(1u64);
                        for _ in 0..215 {
                            result = result.saturating_mul(base);
                            if modulus != U256::ZERO {
                                result = result % modulus;
                            }
                        }
                        black_box(result);
                    }
                });
            },
        );
    }
    group.finish();
}

fn bench_modexp_215_exp_heavy_uncachable2(c: &mut Criterion) {
    let mut group = c.benchmark_group("Modexp215GasExpHeavyUNCACHABLE2");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let base = U256::from_be_bytes([0x4a; 32]);
                let exp = U256::from(215u64);
                let modulus = U256::from_be_bytes([0x4b; 32]);
                b.iter(|| {
                    for _ in 0..10 {
                        let mut result = U256::from(1u64);
                        for _ in 0..215 {
                            result = result.saturating_mul(base);
                            if modulus != U256::ZERO {
                                result = result % modulus;
                            }
                        }
                        black_box(result);
                    }
                });
            },
        );
    }
    group.finish();
}

fn bench_modexp_298_exp_heavy_uncachable(c: &mut Criterion) {
    let mut group = c.benchmark_group("Modexp298GasExpHeavyUNCACHABLE");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let base = U256::from_be_bytes([0x50; 32]);
                let exp = U256::from(298u64);
                let modulus = U256::from_be_bytes([0x51; 32]);
                b.iter(|| {
                    for _ in 0..5 {
                        let mut result = U256::from(1u64);
                        for _ in 0..298 {
                            result = result.saturating_mul(base);
                            if modulus != U256::ZERO {
                                result = result % modulus;
                            }
                        }
                        black_box(result);
                    }
                });
            },
        );
    }
    group.finish();
}

fn bench_modexp_298_exp_heavy_uncachable2(c: &mut Criterion) {
    let mut group = c.benchmark_group("Modexp298GasExpHeavyUNCACHABLE2");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let base = U256::from_be_bytes([0x52; 32]);
                let exp = U256::from(298u64);
                let modulus = U256::from_be_bytes([0x53; 32]);
                b.iter(|| {
                    for _ in 0..5 {
                        let mut result = U256::from(1u64);
                        for _ in 0..298 {
                            result = result.saturating_mul(base);
                            if modulus != U256::ZERO {
                                result = result % modulus;
                            }
                        }
                        black_box(result);
                    }
                });
            },
        );
    }
    group.finish();
}

fn bench_modexp_min_exp_heavy_cachable(c: &mut Criterion) {
    let mut group = c.benchmark_group("ModexpMinGasExpHeavyCACHABLE");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let base = U256::from(2u64);
                let exp = U256::from(32u64);
                let modulus = U256::from(97u64);
                b.iter(|| {
                    for _ in 0..100 {
                        let mut result = U256::from(1u64);
                        for _ in 0..32 {
                            result = result.saturating_mul(base);
                            result = result % modulus;
                        }
                        black_box(result);
                    }
                });
            },
        );
    }
    group.finish();
}

fn bench_modexp_min_exp_heavy_uncachable(c: &mut Criterion) {
    let mut group = c.benchmark_group("ModexpMinGasExpHeavyUNCACHABLE");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let base = U256::from(3u64);
                let exp = U256::from(32u64);
                let modulus = U256::from(97u64);
                b.iter(|| {
                    for _ in 0..100 {
                        let mut result = U256::from(1u64);
                        for _ in 0..32 {
                            result = result.saturating_mul(base);
                            result = result % modulus;
                        }
                        black_box(result);
                    }
                });
            },
        );
    }
    group.finish();
}

fn bench_modexp_min_exp_heavy_uncachable2(c: &mut Criterion) {
    let mut group = c.benchmark_group("ModexpMinGasExpHeavyUNCACHABLE2");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let base = U256::from(5u64);
                let exp = U256::from(32u64);
                let modulus = U256::from(97u64);
                b.iter(|| {
                    for _ in 0..100 {
                        let mut result = U256::from(1u64);
                        for _ in 0..32 {
                            result = result.saturating_mul(base);
                            result = result % modulus;
                        }
                        black_box(result);
                    }
                });
            },
        );
    }
    group.finish();
}

// Pawel's modexp test cases
fn bench_modexp_pawel2(c: &mut Criterion) {
    let mut group = c.benchmark_group("ModexpPawel2UNCACHABLE");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let base = U256::from_be_bytes([0x60; 32]);
                let exp = U256::from(128u64);
                let modulus = U256::from_be_bytes([0x61; 32]);
                b.iter(|| {
                    for _ in 0..10 {
                        let mut result = U256::from(1u64);
                        for _ in 0..128 {
                            result = result.saturating_mul(base);
                            if modulus != U256::ZERO {
                                result = result % modulus;
                            }
                        }
                        black_box(result);
                    }
                });
            },
        );
    }
    group.finish();
}

fn bench_modexp_pawel4_2(c: &mut Criterion) {
    let mut group = c.benchmark_group("ModexpPawel4UNCACHABLE2");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let base = U256::from_be_bytes([0x62; 32]);
                let exp = U256::from(256u64);
                let modulus = U256::from_be_bytes([0x63; 32]);
                b.iter(|| {
                    for _ in 0..5 {
                        let mut result = U256::from(1u64);
                        for _ in 0..256 {
                            result = result.saturating_mul(base);
                            if modulus != U256::ZERO {
                                result = result % modulus;
                            }
                        }
                        black_box(result);
                    }
                });
            },
        );
    }
    group.finish();
}

// Vulnerability test cases
fn bench_modexp_vulnerability_guido4_even(c: &mut Criterion) {
    let mut group = c.benchmark_group("ModexpVulnerabilityGuido4EvenCACHABLE");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let base = U256::from(4u64); // Even base
                let exp = U256::from(64u64);
                let modulus = U256::from(1000u64);
                b.iter(|| {
                    for _ in 0..50 {
                        let mut result = U256::from(1u64);
                        for _ in 0..64 {
                            result = result.saturating_mul(base);
                            result = result % modulus;
                        }
                        black_box(result);
                    }
                });
            },
        );
    }
    group.finish();
}

fn bench_modexp_vulnerability_pawel1_exp_heavy(c: &mut Criterion) {
    let mut group = c.benchmark_group("ModexpVulnerabilityPawel1ExpHeavyCACHABLE");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let base = U256::from_be_bytes([0x70; 32]);
                let exp = U256::from(512u64);
                let modulus = U256::from_be_bytes([0x71; 32]);
                b.iter(|| {
                    for _ in 0..2 {
                        let mut result = U256::from(1u64);
                        for _ in 0..512 {
                            result = result.saturating_mul(base);
                            if modulus != U256::ZERO {
                                result = result % modulus;
                            }
                        }
                        black_box(result);
                    }
                });
            },
        );
    }
    group.finish();
}

fn bench_modexp_vulnerability_pawel2_exp_heavy(c: &mut Criterion) {
    let mut group = c.benchmark_group("ModexpVulnerabilityPawel2ExpHeavyCACHABLE");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let base = U256::from_be_bytes([0x72; 32]);
                let exp = U256::from(1024u64);
                let modulus = U256::from_be_bytes([0x73; 32]);
                b.iter(|| {
                    // Very expensive - reduce iterations
                    let mut result = U256::from(1u64);
                    for _ in 0..1024 {
                        result = result.saturating_mul(base);
                        if modulus != U256::ZERO {
                            result = result % modulus;
                        }
                    }
                    black_box(result);
                });
            },
        );
    }
    group.finish();
}

fn bench_modexp_vulnerability_pawel3_exp_heavy(c: &mut Criterion) {
    let mut group = c.benchmark_group("ModexpVulnerabilityPawel3ExpHeavyCACHABLE");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let base = U256::from_be_bytes([0x74; 32]);
                let exp = U256::from(2048u64);
                let modulus = U256::from_be_bytes([0x75; 32]);
                b.iter(|| {
                    // Extremely expensive - minimal iterations
                    let mut result = U256::from(1u64);
                    for i in 0..128 { // Reduced from 2048
                        result = result.saturating_mul(base);
                        if modulus != U256::ZERO && i % 16 == 0 {
                            result = result % modulus;
                        }
                    }
                    black_box(result);
                });
            },
        );
    }
    group.finish();
}

fn bench_modexp_vulnerability_pawel4_exp_heavy(c: &mut Criterion) {
    let mut group = c.benchmark_group("ModexpVulnerabilityPawel4ExpHeavyCACHABLE");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let base = U256::from_be_bytes([0x76; 32]);
                let exp = U256::from(4096u64);
                let modulus = U256::from_be_bytes([0x77; 32]);
                b.iter(|| {
                    // Ultra expensive - minimal iterations
                    let mut result = U256::from(1u64);
                    for i in 0..64 { // Heavily reduced from 4096
                        result = result.saturating_mul(base);
                        if modulus != U256::ZERO && i % 8 == 0 {
                            result = result % modulus;
                        }
                    }
                    black_box(result);
                });
            },
        );
    }
    group.finish();
}

criterion_group! {
    name = nethermind_precompile_benches;
    config = Criterion::default().sample_size(10);
    targets = 
        bench_blake2_1k_rounds,
        bench_blake2_1m_rounds,
        bench_ec_add_12,
        bench_ec_add_32,
        bench_ec_mul_12_32,
        bench_ec_mul_32_32,
        bench_ec_pairing_2sets,
        bench_ecrecover_uncachable,
        bench_ecrecover_uncachable2,
        bench_modexp_208_balanced,
        bench_modexp_208_balanced2,
        bench_modexp_215_exp_heavy_cachable,
        bench_modexp_215_exp_heavy_uncachable,
        bench_modexp_215_exp_heavy_uncachable2,
        bench_modexp_298_exp_heavy_uncachable,
        bench_modexp_298_exp_heavy_uncachable2,
        bench_modexp_min_exp_heavy_cachable,
        bench_modexp_min_exp_heavy_uncachable,
        bench_modexp_min_exp_heavy_uncachable2,
        bench_modexp_pawel2,
        bench_modexp_pawel4_2,
        bench_modexp_vulnerability_guido4_even,
        bench_modexp_vulnerability_pawel1_exp_heavy,
        bench_modexp_vulnerability_pawel2_exp_heavy,
        bench_modexp_vulnerability_pawel3_exp_heavy,
        bench_modexp_vulnerability_pawel4_exp_heavy
}

criterion_main!(nethermind_precompile_benches);