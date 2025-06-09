# REVM Gas Benchmarks

This repository contains gas benchmarks for REVM opcodes, ported from Nethermind's gas benchmarks.

## Overview

The benchmarks simulate the gas costs and performance characteristics of various EVM opcodes at different gas limits (30M, 60M, 80M, 100M, 150M), matching the structure of Nethermind's gas benchmarks.

## Benchmarks

The benchmarks are located in `crates/interpreter/benches/` and include:

### 1. `nethermind_opcodes.rs` - Complete port of Nethermind's VM opcodes
This file contains ALL opcodes from Nethermind's tests-vm directory:
- **ADDRESS**: Contract address retrieval
- **BASEFEE**: EIP-1559 base fee
- **BLOBBASEFEE**: EIP-4844 blob base fee
- **BLOBHASH**: Blob hash retrieval
- **CALLER/CALLERPOP**: Message sender operations
- **CHAINID**: Chain identifier
- **COINBASE**: Block beneficiary
- **GAS/GASPOP**: Remaining gas operations
- **GASLIMIT**: Block gas limit
- **MSIZE**: Memory size
- **NUMBER**: Block number
- **ORIGIN**: Transaction origin
- **PREVRANDAO**: Previous randomness
- **PUSH0**: Push zero to stack
- **SELFBALANCE**: Contract balance
- **TIMESTAMP**: Block timestamp
- **Keccak256**: Hash operations (1, 8, 32 bytes)
- **Identity**: Identity precompile
- **Transfers**: Value transfer simulation

### 2. `nethermind_precompiles.rs` - Complete port of Nethermind's precompile benchmarks
This file contains ALL precompile benchmarks from Nethermind:
- **Blake2 Rounds**: 1K and 1M round variants
- **EC Operations**:
  - EcAdd (12-byte and 32-byte coordinates)
  - EcMul (various coordinate and scalar sizes)
  - EcPairing (2 sets)
- **EcRecover**: Multiple cachable and uncachable variants
- **ModExp**: Comprehensive modular exponentiation tests
  - 208 gas balanced variants
  - 215 gas exp-heavy variants
  - 298 gas exp-heavy variants
  - Minimum gas exp-heavy variants
  - Pawel's test cases
  - Vulnerability test cases (Guido, Pawel 1-4)

### 3. `gas_opcodes.rs` - Simplified gas opcode benchmarks
- **Stack Operations**: ADDRESS, CALLER, ORIGIN simulation
- **Memory Operations**: MSIZE opcode
- **Keccak256 Operations**: 32-byte hash operations
- **Arithmetic Operations**: U256 addition
- **PUSH0 Operations**: Stack push operations

### 4. `opcodes.rs` - Component-level benchmarks
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
# Run complete Nethermind opcode benchmarks
cargo bench --bench nethermind_opcodes

# Run complete Nethermind precompile benchmarks
cargo bench --bench nethermind_precompiles

# Run simplified benchmarks
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
1. **Exactly match Nethermind's test cases**: Every single test from tests-vm is represented
2. **Match Nethermind's gas limit configurations**: 30M, 50M, 60M, 80M, 100M, 150M (note: 50M is included)
3. **Simulate realistic opcode execution patterns**: 10,000 iterations for simple ops, fewer for expensive ones
4. **Use Criterion.rs for statistical rigor**: Provides detailed performance metrics and comparisons
5. **Separate opcodes from precompiles**: Organized by computation type for clarity

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