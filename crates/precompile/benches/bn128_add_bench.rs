use criterion::{black_box, criterion_group, criterion_main, Criterion};
use revm_precompile::bn128::{run_add, ADD_INPUT_LEN, add::ISTANBUL_ADD_GAS_COST};
use primitives::hex;

fn bench_bn128_add(c: &mut Criterion) {
    // Test case from the official tests
    let input = hex::decode(
        "18b18acfb4c2c30276db5411368e7185b311dd124691610c5d3b74034e093dc9\
         063c909c4720840cb5134cb9f59fa749755796819658d32efc0d288198f37266\
         07c2b7f58a84bd6145f00c9c2bc0bb1a187f20ff2c92963a88019e7c6a014eed\
         06614e20c147e940f2d70da3f74c9a17df361706a4485c742bd6788478fa17d7",
    )
    .unwrap();

    let mut group = c.benchmark_group("bn128_add");
    
    // Print feature status
    #[cfg(feature = "gnark-optimized")]
    println!("Running with gnark-optimized feature enabled");
    #[cfg(not(feature = "gnark-optimized"))]
    println!("Running with default arkworks implementation");
    
    // Ensure input is properly padded
    let padded_input: [u8; ADD_INPUT_LEN] = {
        let mut buf = [0u8; ADD_INPUT_LEN];
        buf[..input.len()].copy_from_slice(&input);
        buf
    };

    group.bench_function("arkworks_default", |b| {
        b.iter(|| {
            run_add(black_box(&padded_input), ISTANBUL_ADD_GAS_COST, 10_000)
        })
    });

    // Also test with zero points (infinity)
    let zero_input = [0u8; ADD_INPUT_LEN];
    group.bench_function("arkworks_zero_points", |b| {
        b.iter(|| {
            run_add(black_box(&zero_input), ISTANBUL_ADD_GAS_COST, 10_000)
        })
    });

    // Test point doubling (same point added to itself)
    let double_input = hex::decode(
        "18b18acfb4c2c30276db5411368e7185b311dd124691610c5d3b74034e093dc9\
         063c909c4720840cb5134cb9f59fa749755796819658d32efc0d288198f37266\
         18b18acfb4c2c30276db5411368e7185b311dd124691610c5d3b74034e093dc9\
         063c909c4720840cb5134cb9f59fa749755796819658d32efc0d288198f37266",
    )
    .unwrap();
    
    let padded_double: [u8; ADD_INPUT_LEN] = {
        let mut buf = [0u8; ADD_INPUT_LEN];
        buf[..double_input.len()].copy_from_slice(&double_input);
        buf
    };

    group.bench_function("arkworks_point_doubling", |b| {
        b.iter(|| {
            run_add(black_box(&padded_double), ISTANBUL_ADD_GAS_COST, 10_000)
        })
    });

    group.finish();
}

criterion_group!(benches, bench_bn128_add);
criterion_main!(benches);