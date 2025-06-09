use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use revm_interpreter::{SharedMemory, Stack, interpreter_types::MemoryTr};
use primitives::{Address, U256};
use std::hint::black_box;

// Benchmark configurations matching Nethermind's gas limits
const GAS_LIMITS: &[u64] = &[30_000_000, 60_000_000, 80_000_000, 100_000_000, 150_000_000];

// Stack operations - simulating ADDRESS, CALLER, ORIGIN opcodes
fn bench_stack_ops(c: &mut Criterion) {
    let mut group = c.benchmark_group("stack_opcodes");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::new("push_address", format!("{}M", gas_limit / 1_000_000)),
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

// Memory operations - simulating MSIZE opcode
fn bench_memory_ops(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_opcodes");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::new("msize", format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                b.iter(|| {
                    let mut memory = SharedMemory::new();
                    memory.resize(1024);
                    for _ in 0..10000 {
                        black_box(memory.size());
                    }
                });
            },
        );
    }
    
    group.finish();
}

// Keccak256 operations with different data sizes
fn bench_keccak256_ops(c: &mut Criterion) {
    let mut group = c.benchmark_group("keccak256_opcodes");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::new("keccak256_32bytes", format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, _| {
                let data = vec![0u8; 32];
                b.iter(|| {
                    for _ in 0..1000 {
                        let hash = primitives::keccak256(&data);
                        black_box(hash);
                    }
                });
            },
        );
    }
    
    group.finish();
}

// U256 arithmetic operations - simulating gas costs for arithmetic opcodes
fn bench_arithmetic_ops(c: &mut Criterion) {
    let mut group = c.benchmark_group("arithmetic_opcodes");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::new("add", format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |bencher, _| {
                let a = U256::from(u128::MAX);
                let b = U256::from(42);
                bencher.iter(|| {
                    for _ in 0..10000 {
                        black_box(a.saturating_add(b));
                    }
                });
            },
        );
    }
    
    group.finish();
}

// Simulating PUSH0 opcode
fn bench_push0(c: &mut Criterion) {
    let mut group = c.benchmark_group("push0_opcode");
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::new("push0", format!("{}M", gas_limit / 1_000_000)),
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

criterion_group!(
    name = gas_opcode_benches;
    config = Criterion::default().sample_size(10);
    targets = bench_stack_ops,
              bench_memory_ops,
              bench_keccak256_ops,
              bench_arithmetic_ops,
              bench_push0
);

criterion_main!(gas_opcode_benches);