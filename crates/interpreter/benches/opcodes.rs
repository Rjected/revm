use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use revm_interpreter::{SharedMemory, Stack};
use primitives::{Address, Bytes, U256};
use std::hint::black_box;

// Simple stack operation benchmarks
fn bench_push_pop(c: &mut Criterion) {
    let mut group = c.benchmark_group("stack_operations");
    
    group.bench_function("push_pop", |b| {
        b.iter_batched(
            || Stack::new(),
            |mut stack| {
                for i in 0..1000 {
                    let _ = stack.push(U256::from(i));
                    let _ = black_box(stack.pop());
                }
            },
            BatchSize::SmallInput,
        );
    });
    
    group.bench_function("push_dup_pop", |b| {
        b.iter_batched(
            || {
                let mut stack = Stack::new();
                let _ = stack.push(U256::from(42));
                stack
            },
            |mut stack| {
                for _ in 0..1000 {
                    let _ = stack.dup(0);
                    let _ = black_box(stack.pop());
                }
            },
            BatchSize::SmallInput,
        );
    });
    
    group.finish();
}

// Memory operation benchmarks
fn bench_memory_ops(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_operations");
    
    group.bench_function("memory_resize", |b| {
        b.iter_batched(
            || SharedMemory::new(),
            |mut memory| {
                for size in (0..10000).step_by(32) {
                    memory.resize(size);
                }
            },
            BatchSize::SmallInput,
        );
    });
    
    group.bench_function("memory_set_get", |b| {
        b.iter_batched(
            || {
                let mut memory = SharedMemory::new();
                memory.resize(1024);
                memory
            },
            |mut memory| {
                let data = [0u8; 32];
                for offset in (0..992).step_by(32) {
                    memory.set(offset, &data);
                    black_box(memory.slice_len(offset, 32));
                }
            },
            BatchSize::SmallInput,
        );
    });
    
    group.finish();
}

// Simulate address-related operations
fn bench_address_ops(c: &mut Criterion) {
    let mut group = c.benchmark_group("address_operations");
    
    group.bench_function("address_conversions", |b| {
        let addresses: Vec<Address> = (0..100).map(|i| Address::from([i as u8; 20])).collect();
        
        b.iter(|| {
            for addr in &addresses {
                let word = addr.into_word();
                black_box(word);
            }
        });
    });
    
    group.finish();
}

// Simulate keccak256 operations with varying data sizes
fn bench_keccak256_ops(c: &mut Criterion) {
    let mut group = c.benchmark_group("keccak256_operations");
    
    for size in [32, 64, 128, 256, 512, 1024] {
        let name = format!("keccak256_{}_bytes", size);
        group.bench_function(&name, |b| {
            let data = vec![0u8; size];
            b.iter(|| {
                let hash = primitives::keccak256(&data);
                black_box(hash);
            });
        });
    }
    
    group.finish();
}

// U256 arithmetic operations
fn bench_u256_ops(c: &mut Criterion) {
    let mut group = c.benchmark_group("u256_operations");
    
    group.bench_function("u256_add", |b| {
        let val_a = U256::from(u128::MAX);
        let val_b = U256::from(42);
        b.iter(|| {
            black_box(val_a.saturating_add(val_b));
        });
    });
    
    group.bench_function("u256_mul", |b| {
        let val_a = U256::from(u128::MAX);
        let val_b = U256::from(42);
        b.iter(|| {
            black_box(val_a.saturating_mul(val_b));
        });
    });
    
    group.bench_function("u256_div", |b| {
        let val_a = U256::from(u128::MAX);
        let val_b = U256::from(42);
        b.iter(|| {
            black_box(val_a / val_b);
        });
    });
    
    group.finish();
}

// Bytes operations
fn bench_bytes_ops(c: &mut Criterion) {
    let mut group = c.benchmark_group("bytes_operations");
    
    for size in [32, 64, 128, 256, 512, 1024] {
        let name = format!("bytes_copy_{}", size);
        group.bench_function(&name, |b| {
            let data = vec![0u8; size];
            b.iter(|| {
                let bytes = Bytes::copy_from_slice(&data);
                black_box(bytes);
            });
        });
    }
    
    group.finish();
}

criterion_group!(
    name = opcode_benches;
    config = Criterion::default();
    targets = bench_push_pop,
              bench_memory_ops,
              bench_address_ops,
              bench_keccak256_ops,
              bench_u256_ops,
              bench_bytes_ops
);

criterion_main!(opcode_benches);