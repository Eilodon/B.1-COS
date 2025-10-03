#!/bin/bash

# Performance profiling script using flamegraph
set -e

echo "🔥 Starting Pandora Performance Profiling with Flamegraph..."

# Install flamegraph if not present
if ! command -v flamegraph &> /dev/null; then
    echo "Installing flamegraph..."
    cargo install flamegraph
fi

# Set up profiling environment
export RUSTFLAGS="-C force-frame-pointers"
export CARGO_PROFILE_DEV_DEBUG=1

# Create output directory
mkdir -p profiles/flamegraph

echo "📊 Running automatic scientist test with flamegraph profiling..."

# Run the automatic scientist test with flamegraph
cd sdk
cargo flamegraph --test automatic_scientist_test --output ../profiles/flamegraph/automatic_scientist_flamegraph.svg

echo "📊 Running load test scenarios with flamegraph profiling..."

# Run load test scenarios  
cargo flamegraph --test load_test_scenarios --output ../profiles/flamegraph/load_test_flamegraph.svg

echo "✅ Flamegraph profiling complete!"
echo "📁 Results saved to:"
echo "   - profiles/flamegraph/automatic_scientist_flamegraph.svg"
echo "   - profiles/flamegraph/load_test_flamegraph.svg"
