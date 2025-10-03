#!/bin/bash

# Memory profiling script
set -e

echo "🧠 Starting Pandora Memory Profiling..."

# Install memory profiler if not present
if ! command -v valgrind &> /dev/null; then
    echo "⚠️  Valgrind not found. Installing..."
    sudo apt-get update && sudo apt-get install -y valgrind
fi

# Create output directory
mkdir -p profiles/memory

echo "📊 Running memory profiling on automatic scientist test..."

# Run memory profiling
cd sdk
valgrind --tool=massif --pages-as-heap=yes --massif-out-file=../profiles/memory/automatic_scientist_memory.massif \
    cargo test --test automatic_scientist_test test_automatic_scientist_orchestrator

echo "📊 Running memory profiling on load test scenarios..."

valgrind --tool=massif --pages-as-heap=yes --massif-out-file=../profiles/memory/load_test_memory.massif \
    cargo test --test load_test_scenarios

echo "✅ Memory profiling complete!"
echo "📁 Results saved to:"
echo "   - profiles/memory/automatic_scientist_memory.massif"
echo "   - profiles/memory/load_test_memory.massif"

# Generate memory reports
echo "📈 Generating memory reports..."
ms_print ../profiles/memory/automatic_scientist_memory.massif > ../profiles/memory/automatic_scientist_report.txt
ms_print ../profiles/memory/load_test_memory.massif > ../profiles/memory/load_test_report.txt

echo "📊 Memory reports generated!"
