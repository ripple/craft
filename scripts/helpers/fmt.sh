#!/bin/bash
# Rust formatting check script
# Mirrors the rustfmt job from GitHub Actions

set -euo pipefail

echo "🔧 Running Rust formatting check..."

# Set RUSTFLAGS to match CI environment
export RUSTFLAGS="${RUSTFLAGS:-"-Dwarnings"}"

echo "📝 Checking formatting for entire workspace..."
cargo fmt --all -- --check

echo "✅ Formatting check passed!"
