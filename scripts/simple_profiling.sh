#!/bin/bash

# Simple performance profiling script
set -e

echo "🚀 Starting Pandora Performance Analysis..."

# Create output directory
mkdir -p profiles

echo "📊 Running performance benchmarks..."

cd sdk

# Run tests with timing
echo "⏱️  Running automatic scientist tests with timing..."
time cargo test --test automatic_scientist_test --release 2>&1 | tee ../profiles/automatic_scientist_timing.log

echo "⏱️  Running CWM tests with timing..."
time cargo test --package pandora_cwm --release 2>&1 | tee ../profiles/cwm_timing.log

echo "⏱️  Running SIE tests with timing..."
time cargo test --package pandora_sie --release 2>&1 | tee ../profiles/sie_timing.log

echo "✅ Performance analysis complete!"
echo "📁 Results saved to:"
echo "   - profiles/automatic_scientist_timing.log"
echo "   - profiles/cwm_timing.log"
echo "   - profiles/sie_timing.log"
