#!/bin/bash
set -e

echo "🔥 Generating flamegraph profile..."
echo ""

cd sdk

# Install cargo-flamegraph if not present
if ! command -v cargo-flamegraph &> /dev/null; then
    echo "Installing cargo-flamegraph..."
    cargo install flamegraph
fi

# Build with optimizations and debug symbols
export CARGO_PROFILE_RELEASE_DEBUG=true

# Profile the skandha benchmark
echo "Profiling Skandha pipeline..."
cargo flamegraph --bench comprehensive_bench -- --bench "skandha_pipeline/full_cycle/100"

echo ""
echo "✅ Flamegraph generated: flamegraph.svg"
echo "   Open with: firefox flamegraph.svg"


