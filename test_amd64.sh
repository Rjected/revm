#!/bin/bash
# Test script for verifying BN254 assembly optimizations on Linux x86_64

set -e

echo "Testing BN254 optimizations on Linux x86_64"
echo "==========================================="

# Check if we're on Linux x86_64
if [[ "$OSTYPE" != "linux-gnu"* ]] || [[ "$(uname -m)" != "x86_64" ]]; then
    echo "Warning: This script is intended for Linux x86_64 systems"
    echo "Current system: $OSTYPE on $(uname -m)"
fi

# Check for required CPU features
echo -e "\nChecking CPU features:"
if grep -q "adx" /proc/cpuinfo; then
    echo "✓ ADX instruction set supported"
else
    echo "✗ ADX instruction set not supported (assembly will use fallback)"
fi

if grep -q "bmi2" /proc/cpuinfo; then
    echo "✓ BMI2 instruction set supported"
else
    echo "✗ BMI2 instruction set not supported (assembly will use fallback)"
fi

# Run tests with gnark-optimized feature
echo -e "\nRunning tests with gnark-optimized feature:"
cargo test -p revm-precompile --features gnark-optimized test_alt_bn128_add

# Run benchmarks
echo -e "\nRunning benchmarks:"
echo -e "\n1. With gnark-optimized feature:"
cargo run --release --example bench_bn128_add --features gnark-optimized -p revm-precompile

echo -e "\n2. Without optimizations (baseline):"
cargo run --release --example bench_bn128_add -p revm-precompile

# Run criterion benchmarks if available
if command -v cargo-criterion &> /dev/null; then
    echo -e "\nRunning criterion benchmarks:"
    cargo bench --features gnark-optimized -p revm-precompile -- bn128_add
fi

echo -e "\nTest complete!"