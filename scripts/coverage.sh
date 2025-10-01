#!/bin/bash

# Coverage script for Pandora Genesis SDK
set -e

echo "🔍 Generating coverage report for Pandora Genesis SDK..."

# Check if cargo-llvm-cov is installed
if ! command -v cargo-llvm-cov &> /dev/null; then
    echo "❌ cargo-llvm-cov not found. Installing..."
    cargo install cargo-llvm-cov
fi

# Change to SDK directory
cd sdk

# Clean previous coverage data
echo "🧹 Cleaning previous coverage data..."
cargo clean

# Generate coverage report
echo "📊 Generating coverage report..."
cargo llvm-cov \
    --workspace \
    --all-features \
    --html \
    --output-dir ../coverage

# Generate LCOV report for CI
echo "📈 Generating LCOV report..."
cargo llvm-cov \
    --workspace \
    --all-features \
    --lcov \
    --output-path ../coverage/lcov.info

echo "✅ Coverage report generated in ../coverage/"
echo "🌐 Open ../coverage/index.html in your browser to view the report"

# Show summary
echo ""
echo "📋 Coverage Summary:"
cargo llvm-cov \
    --workspace \
    --all-features \
    --summary-only
