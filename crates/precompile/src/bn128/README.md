# BN254 Optimizations

This module implements optimized BN254 (alt-bn128) elliptic curve operations for Ethereum precompiles.

## Features

### `gnark-optimized`

When enabled, this feature provides performance optimizations inspired by gnark-crypto:

1. **Optimized field arithmetic** - Uses specialized implementations for BN254 field operations
2. **Assembly implementations (Linux/BSD only)** - Hand-written x86_64 assembly for critical operations
3. **Faster point addition** - Optimized Jacobian coordinate arithmetic
4. **Improved serialization** - Faster encoding/decoding of field elements

## Platform Support

| Platform | Assembly Support | Fallback |
|----------|-----------------|----------|
| Linux x86_64 | ✅ Full assembly optimization | - |
| BSD x86_64 | ✅ Full assembly optimization | - |
| macOS x86_64 | ❌ No assembly (AT&T syntax incompatible) | ✅ Rust implementation |
| Windows x86_64 | ❌ No assembly | ✅ Rust implementation |
| Other architectures | ❌ No assembly | ✅ Rust implementation |

## Usage

Enable the feature in your `Cargo.toml`:
```toml
[dependencies]
revm-precompile = { version = "22.0", features = ["gnark-optimized"] }
```

## Performance

On Linux x86_64 with assembly optimizations:
- BN254 point addition: ~2-3x faster than standard implementation
- Field multiplication: ~1.5-2x faster with ADX/BMI2 instructions

The optimizations are particularly effective for:
- Smart contracts that heavily use BN254 operations (e.g., zkSNARK verifiers)
- Batch verification of multiple proofs
- Pairing-heavy computations

## Implementation Notes

The assembly files use AT&T syntax and require:
- ADX instruction set (for carry-less multiplication)
- BMI2 instruction set (for efficient bit manipulation)

The implementation automatically detects CPU capabilities and falls back to portable code if required instructions are not available.