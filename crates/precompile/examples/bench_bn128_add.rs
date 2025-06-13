use revm_precompile::bn128::{run_add, ADD_INPUT_LEN, add::ISTANBUL_ADD_GAS_COST};
use primitives::hex;
use std::time::Instant;

fn main() {
    // Test case from the official tests
    let input = hex::decode(
        "18b18acfb4c2c30276db5411368e7185b311dd124691610c5d3b74034e093dc9\
         063c909c4720840cb5134cb9f59fa749755796819658d32efc0d288198f37266\
         07c2b7f58a84bd6145f00c9c2bc0bb1a187f20ff2c92963a88019e7c6a014eed\
         06614e20c147e940f2d70da3f74c9a17df361706a4485c742bd6788478fa17d7",
    )
    .unwrap();
    
    // Ensure input is properly padded
    let padded_input: [u8; ADD_INPUT_LEN] = {
        let mut buf = [0u8; ADD_INPUT_LEN];
        buf[..input.len()].copy_from_slice(&input);
        buf
    };
    
    // Print feature status
    #[cfg(feature = "gnark-optimized")]
    {
        println!("Running with gnark-optimized feature enabled");
        #[cfg(no_asm)]
        println!("Assembly optimizations disabled (platform: {})", std::env::consts::OS);
        #[cfg(not(no_asm))]
        println!("Assembly optimizations enabled");
    }
    #[cfg(not(feature = "gnark-optimized"))]
    println!("Running with default arkworks implementation");
    
    // Warmup
    for _ in 0..1000 {
        let _ = run_add(&padded_input, ISTANBUL_ADD_GAS_COST, 10_000);
    }
    
    // Benchmark
    let iterations = 100_000;
    let start = Instant::now();
    
    for _ in 0..iterations {
        let _ = run_add(&padded_input, ISTANBUL_ADD_GAS_COST, 10_000).unwrap();
    }
    
    let duration = start.elapsed();
    let per_op = duration / iterations;
    
    println!("Iterations: {}", iterations);
    println!("Total time: {:?}", duration);
    println!("Time per operation: {:?}", per_op);
    println!("Operations per second: {:.0}", 1_000_000_000.0 / per_op.as_nanos() as f64);
    
    // Also test with zero points
    let zero_input = [0u8; ADD_INPUT_LEN];
    let start = Instant::now();
    
    for _ in 0..iterations {
        let _ = run_add(&zero_input, ISTANBUL_ADD_GAS_COST, 10_000).unwrap();
    }
    
    let duration = start.elapsed();
    let per_op = duration / iterations;
    
    println!("\nZero points (infinity):");
    println!("Time per operation: {:?}", per_op);
    println!("Operations per second: {:.0}", 1_000_000_000.0 / per_op.as_nanos() as f64);
    
    // Test point doubling
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
    
    let start = Instant::now();
    
    for _ in 0..iterations {
        let _ = run_add(&padded_double, ISTANBUL_ADD_GAS_COST, 10_000).unwrap();
    }
    
    let duration = start.elapsed();
    let per_op = duration / iterations;
    
    println!("\nPoint doubling:");
    println!("Time per operation: {:?}", per_op);
    println!("Operations per second: {:.0}", 1_000_000_000.0 / per_op.as_nanos() as f64);
}