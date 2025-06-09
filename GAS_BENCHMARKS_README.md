# REVM Gas Benchmarks

This repository contains gas benchmarks for REVM opcodes, ported from Nethermind's gas benchmarks.

## Overview

The benchmarks simulate the gas costs and performance characteristics of various EVM opcodes at different gas limits (30M, 60M, 80M, 100M, 150M), matching the structure of Nethermind's gas benchmarks.

## Benchmarks

The benchmarks are located in `crates/interpreter/benches/` and include:

### 1. `gas_opcodes.rs` - Main gas opcode benchmarks
- **Stack Operations**: Simulating ADDRESS, CALLER, ORIGIN opcodes
- **Memory Operations**: MSIZE opcode simulation
- **Keccak256 Operations**: Hash operations with 32-byte data
- **Arithmetic Operations**: U256 addition operations
- **PUSH0 Operations**: Stack push operations

### 2. `opcodes.rs` - Component-level benchmarks
- **Stack Operations**: Push/pop, dup operations
- **Memory Operations**: Resize, set/get operations
- **Address Operations**: Address conversions
- **Keccak256 Operations**: Various data sizes (32, 64, 128, 256, 512, 1024 bytes)
- **U256 Operations**: Addition, multiplication, division
- **Bytes Operations**: Copy operations for various sizes

## Running the Benchmarks

### Prerequisites
- Rust toolchain (latest stable)
- Cargo

### Run all benchmarks
```bash
cd crates/interpreter
cargo bench
```

### Run specific benchmark suite
```bash
cd crates/interpreter
cargo bench --bench gas_opcodes
cargo bench --bench opcodes
```

### Run with specific filter
```bash
cd crates/interpreter
cargo bench --bench gas_opcodes -- push_address
```

### Quick test run (faster, less accurate)
```bash
cd crates/interpreter
cargo bench --bench gas_opcodes -- --test
```

## Benchmark Results

The benchmarks generate HTML reports in `target/criterion/` with detailed performance metrics including:
- Execution time per iteration
- Throughput measurements
- Statistical analysis (mean, median, standard deviation)
- Performance comparisons between runs

## Architecture

The benchmarks are structured to:
1. Match Nethermind's gas limit configurations (30M, 60M, 80M, 100M, 150M)
2. Simulate realistic opcode execution patterns
3. Measure performance at scale (10,000 iterations for most opcodes)
4. Use Criterion.rs for statistical rigor and reproducibility

## Implementation Notes

### Key Components Used:
- `revm_interpreter::Stack` - EVM stack implementation
- `revm_interpreter::SharedMemory` - EVM memory implementation
- `primitives::U256` - 256-bit unsigned integer operations
- `primitives::keccak256` - Keccak hash function

### Design Decisions:
1. **Isolated Operations**: Each benchmark tests a specific operation in isolation to measure its individual performance characteristics
2. **Realistic Workloads**: Operation counts (10,000 for most, 1,000 for expensive ops like keccak256) simulate realistic EVM execution patterns
3. **Gas Limit Parameterization**: All benchmarks are parameterized by gas limit to match Nethermind's approach
4. **Statistical Sampling**: Using Criterion's default sample size (100) with option to reduce to 10 for faster iteration

## Future Work

Potential additions:
- Storage operations (SLOAD, SSTORE)
- Call operations (CALL, DELEGATECALL, STATICCALL)
- Create operations (CREATE, CREATE2)
- Log operations (LOG0-LOG4)
- More arithmetic operations (SUB, MUL, DIV, MOD, EXP)
- Comparison operations (LT, GT, EQ)
- Bitwise operations (AND, OR, XOR, NOT)

## License

This project inherits the license from the REVM project.