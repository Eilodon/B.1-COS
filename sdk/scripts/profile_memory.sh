#!/bin/bash
set -e

echo "💾 Profiling memory usage with valgrind..."
echo ""

cd sdk

# Build test binary
cargo build --tests --release

# Find the test binary
TEST_BINARY=$(find target/release/deps -name "load_scenarios*" -type f -executable | head -1)

if [ -z "$TEST_BINARY" ]; then
    echo "❌ Could not find test binary"
    exit 1
fi

echo "Using test binary: $TEST_BINARY"

# Run with valgrind
valgrind \
    --tool=massif \
    --massif-out-file=massif.out \
    --time-unit=B \
    $TEST_BINARY --test test_memory_stability_under_load --ignored

echo ""
echo "✅ Memory profile generated: massif.out"
echo "   Analyze with: ms_print massif.out"


