use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use revm_interpreter::{SharedMemory, Stack, interpreter_types::MemoryTr};
use primitives::{Address, Bytes, U256, B256, keccak256};
use std::hint::black_box;

// All gas limits from Nethermind benchmarks
const GAS_LIMITS: &[u64] = &[30_000_000, 50_000_000, 60_000_000, 80_000_000, 100_000_000, 150_000_000];

// ADDRESS opcode benchmark
fn bench_address(c: &mut Criterion) {
    let mut group = c.benchmark_group("Address");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let addr = Address::from([0x42; 20]);
                let addr_u256 = U256::from_be_bytes(addr.into_word().0);
                b.iter(|| {
                    let mut stack = Stack::new();
                    for _ in 0..10000 {
                        let _ = stack.push(addr_u256);
                        black_box(&stack);
                    }
                });
            },
        );
    }
    group.finish();
}

// BASEFEE opcode benchmark
fn bench_basefee(c: &mut Criterion) {
    let mut group = c.benchmark_group("BaseFee");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let basefee = U256::from(1000u64);
                b.iter(|| {
                    let mut stack = Stack::new();
                    for _ in 0..10000 {
                        let _ = stack.push(basefee);
                        black_box(&stack);
                    }
                });
            },
        );
    }
    group.finish();
}

// BLOBBASEFEE opcode benchmark
fn bench_blobbasefee(c: &mut Criterion) {
    let mut group = c.benchmark_group("BlobBaseFee");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let blobbasefee = U256::from(1u64);
                b.iter(|| {
                    let mut stack = Stack::new();
                    for _ in 0..10000 {
                        let _ = stack.push(blobbasefee);
                        black_box(&stack);
                    }
                });
            },
        );
    }
    group.finish();
}

// BLOBHASH opcode benchmark (index 0)
fn bench_blobhash_zero(c: &mut Criterion) {
    let mut group = c.benchmark_group("BlobHashZero");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let blob_hash = B256::default();
                b.iter(|| {
                    let mut stack = Stack::new();
                    for _ in 0..10000 {
                        let _ = stack.push(U256::ZERO); // index
                        let _ = stack.pop(); // simulate reading index
                        let _ = stack.push(U256::from_be_bytes(blob_hash.0));
                        black_box(&stack);
                    }
                });
            },
        );
    }
    group.finish();
}

// CALLER opcode benchmark
fn bench_caller(c: &mut Criterion) {
    let mut group = c.benchmark_group("Caller");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let caller = Address::from([0x01; 20]);
                let caller_u256 = U256::from_be_bytes(caller.into_word().0);
                b.iter(|| {
                    let mut stack = Stack::new();
                    for _ in 0..10000 {
                        let _ = stack.push(caller_u256);
                        black_box(&stack);
                    }
                });
            },
        );
    }
    group.finish();
}

// CALLER with POP opcode benchmark
fn bench_caller_pop(c: &mut Criterion) {
    let mut group = c.benchmark_group("CallerPop");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let caller = Address::from([0x01; 20]);
                let caller_u256 = U256::from_be_bytes(caller.into_word().0);
                b.iter(|| {
                    let mut stack = Stack::new();
                    for _ in 0..10000 {
                        let _ = stack.push(caller_u256);
                        let _ = stack.pop();
                        black_box(&stack);
                    }
                });
            },
        );
    }
    group.finish();
}

// CHAINID opcode benchmark
fn bench_chainid(c: &mut Criterion) {
    let mut group = c.benchmark_group("ChainId");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let chain_id = U256::from(1u64);
                b.iter(|| {
                    let mut stack = Stack::new();
                    for _ in 0..10000 {
                        let _ = stack.push(chain_id);
                        black_box(&stack);
                    }
                });
            },
        );
    }
    group.finish();
}

// COINBASE opcode benchmark
fn bench_coinbase(c: &mut Criterion) {
    let mut group = c.benchmark_group("CoinBase");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let coinbase = Address::from([0x02; 20]);
                let coinbase_u256 = U256::from_be_bytes(coinbase.into_word().0);
                b.iter(|| {
                    let mut stack = Stack::new();
                    for _ in 0..10000 {
                        let _ = stack.push(coinbase_u256);
                        black_box(&stack);
                    }
                });
            },
        );
    }
    group.finish();
}

// GAS opcode benchmark
fn bench_gas(c: &mut Criterion) {
    let mut group = c.benchmark_group("Gas");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, &limit| {
                let gas_remaining = U256::from(limit);
                b.iter(|| {
                    let mut stack = Stack::new();
                    for _ in 0..10000 {
                        let _ = stack.push(gas_remaining);
                        black_box(&stack);
                    }
                });
            },
        );
    }
    group.finish();
}

// GASLIMIT opcode benchmark
fn bench_gaslimit(c: &mut Criterion) {
    let mut group = c.benchmark_group("GasLimit");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, &limit| {
                let gas_limit_u256 = U256::from(limit);
                b.iter(|| {
                    let mut stack = Stack::new();
                    for _ in 0..10000 {
                        let _ = stack.push(gas_limit_u256);
                        black_box(&stack);
                    }
                });
            },
        );
    }
    group.finish();
}

// GAS with POP opcode benchmark
fn bench_gas_pop(c: &mut Criterion) {
    let mut group = c.benchmark_group("GasPop");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, &limit| {
                let gas_remaining = U256::from(limit);
                b.iter(|| {
                    let mut stack = Stack::new();
                    for _ in 0..10000 {
                        let _ = stack.push(gas_remaining);
                        let _ = stack.pop();
                        black_box(&stack);
                    }
                });
            },
        );
    }
    group.finish();
}

// MSIZE opcode benchmark
fn bench_msize(c: &mut Criterion) {
    let mut group = c.benchmark_group("MSize");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                b.iter(|| {
                    let mut memory = SharedMemory::new();
                    memory.resize(1024);
                    let mut stack = Stack::new();
                    for _ in 0..10000 {
                        let _ = stack.push(U256::from(memory.size()));
                        black_box(&stack);
                    }
                });
            },
        );
    }
    group.finish();
}

// NUMBER opcode benchmark
fn bench_number(c: &mut Criterion) {
    let mut group = c.benchmark_group("Number");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let block_number = U256::from(1000000u64);
                b.iter(|| {
                    let mut stack = Stack::new();
                    for _ in 0..10000 {
                        let _ = stack.push(block_number);
                        black_box(&stack);
                    }
                });
            },
        );
    }
    group.finish();
}

// ORIGIN opcode benchmark
fn bench_origin(c: &mut Criterion) {
    let mut group = c.benchmark_group("Origin");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let origin = Address::from([0x03; 20]);
                let origin_u256 = U256::from_be_bytes(origin.into_word().0);
                b.iter(|| {
                    let mut stack = Stack::new();
                    for _ in 0..10000 {
                        let _ = stack.push(origin_u256);
                        black_box(&stack);
                    }
                });
            },
        );
    }
    group.finish();
}

// PREVRANDAO opcode benchmark
fn bench_prevrandao(c: &mut Criterion) {
    let mut group = c.benchmark_group("PrevRandao");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let prevrandao = B256::from([0x44; 32]);
                let prevrandao_u256 = U256::from_be_bytes(prevrandao.0);
                b.iter(|| {
                    let mut stack = Stack::new();
                    for _ in 0..10000 {
                        let _ = stack.push(prevrandao_u256);
                        black_box(&stack);
                    }
                });
            },
        );
    }
    group.finish();
}

// PUSH0 opcode benchmark
fn bench_push0(c: &mut Criterion) {
    let mut group = c.benchmark_group("Push0");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                b.iter(|| {
                    let mut stack = Stack::new();
                    for _ in 0..10000 {
                        let _ = stack.push(U256::ZERO);
                        black_box(&stack);
                    }
                });
            },
        );
    }
    group.finish();
}

// SELFBALANCE opcode benchmark
fn bench_selfbalance(c: &mut Criterion) {
    let mut group = c.benchmark_group("SelfBalance");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let balance = U256::from(1000000u64);
                b.iter(|| {
                    let mut stack = Stack::new();
                    for _ in 0..10000 {
                        let _ = stack.push(balance);
                        black_box(&stack);
                    }
                });
            },
        );
    }
    group.finish();
}

// TIMESTAMP opcode benchmark
fn bench_timestamp(c: &mut Criterion) {
    let mut group = c.benchmark_group("Timestamp");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let timestamp = U256::from(1700000000u64);
                b.iter(|| {
                    let mut stack = Stack::new();
                    for _ in 0..10000 {
                        let _ = stack.push(timestamp);
                        black_box(&stack);
                    }
                });
            },
        );
    }
    group.finish();
}

// Keccak256 benchmarks with different sizes
fn bench_keccak256_1byte(c: &mut Criterion) {
    let mut group = c.benchmark_group("Keccak256From1Byte");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let data = vec![0x42u8; 1];
                b.iter(|| {
                    for _ in 0..1000 {
                        let hash = keccak256(&data);
                        black_box(hash);
                    }
                });
            },
        );
    }
    group.finish();
}

fn bench_keccak256_8bytes(c: &mut Criterion) {
    let mut group = c.benchmark_group("Keccak256From8Bytes");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let data = vec![0x42u8; 8];
                b.iter(|| {
                    for _ in 0..1000 {
                        let hash = keccak256(&data);
                        black_box(hash);
                    }
                });
            },
        );
    }
    group.finish();
}

fn bench_keccak256_32bytes(c: &mut Criterion) {
    let mut group = c.benchmark_group("Keccak256From32Bytes");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let data = vec![0x42u8; 32];
                b.iter(|| {
                    for _ in 0..1000 {
                        let hash = keccak256(&data);
                        black_box(hash);
                    }
                });
            },
        );
    }
    group.finish();
}

// Identity precompile benchmark (simulated)
fn bench_identity_1byte(c: &mut Criterion) {
    let mut group = c.benchmark_group("IdentityFrom1ByteCACHABLE");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let data = Bytes::from(vec![0x42u8; 1]);
                b.iter(|| {
                    for _ in 0..10000 {
                        // Identity precompile just returns the input
                        let result = data.clone();
                        black_box(result);
                    }
                });
            },
        );
    }
    group.finish();
}

// Simulate transfers (simple value movements)
fn bench_transfers(c: &mut Criterion) {
    let mut group = c.benchmark_group("Transfers");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let value = U256::from(1000u64);
                b.iter(|| {
                    for _ in 0..1000 {
                        // Simulate balance updates
                        let from_balance = U256::from(10000u64);
                        let to_balance = U256::from(5000u64);
                        let new_from = from_balance.saturating_sub(value);
                        let new_to = to_balance.saturating_add(value);
                        black_box((new_from, new_to));
                    }
                });
            },
        );
    }
    group.finish();
}

// Note: The following precompiles would require actual implementations:
// - Blake2 rounds (Blake1KRoundsCACHABLE, Blake1MRoundsCACHABLE)
// - EC operations (EcAdd, EcMul, EcPairing)
// - EcRecover
// - Modexp variants
// For now, we'll create placeholder benchmarks

fn bench_ecrecover_cachable(c: &mut Criterion) {
    let mut group = c.benchmark_group("EcRecoverCACHABLE");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                // Simulate ecrecover computation cost
                let hash = B256::from([0x42; 32]);
                b.iter(|| {
                    for _ in 0..100 {
                        // Placeholder for actual ecrecover
                        let result = keccak256(&hash.0);
                        black_box(result);
                    }
                });
            },
        );
    }
    group.finish();
}

criterion_group! {
    name = nethermind_opcode_benches;
    config = Criterion::default().sample_size(10);
    targets = 
        bench_address,
        bench_basefee,
        bench_blobbasefee,
        bench_blobhash_zero,
        bench_caller,
        bench_caller_pop,
        bench_chainid,
        bench_coinbase,
        bench_gas,
        bench_gaslimit,
        bench_gas_pop,
        bench_msize,
        bench_number,
        bench_origin,
        bench_prevrandao,
        bench_push0,
        bench_selfbalance,
        bench_timestamp,
        bench_keccak256_1byte,
        bench_keccak256_8bytes,
        bench_keccak256_32bytes,
        bench_identity_1byte,
        bench_transfers,
        bench_ecrecover_cachable
}

criterion_main!(nethermind_opcode_benches);