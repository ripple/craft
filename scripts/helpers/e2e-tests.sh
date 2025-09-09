#!/bin/bash
# End-to-end tests script
# Mirrors the e2e-tests job from GitHub Actions

set -euo pipefail

echo "🔧 Running end-to-end tests..."

# Set RUSTFLAGS to match CI environment
export RUSTFLAGS="${RUSTFLAGS:-"-Dwarnings"}"

# Ensure wasm32 target is available
echo "📦 Ensuring wasm32-unknown-unknown target is installed..."
rustup target add wasm32-unknown-unknown

echo "🏗️  Building projects..."
../build.sh
../build.sh release

echo "🧪 Running integration tests..."
find ./projects -name "Cargo.toml" -type f | while read -r cargo_file; do
    dir=$(dirname "$cargo_file")
    contract_name=$(basename "$dir")
    if [ -d "$dir/fixtures" ]; then
        echo "🔧 Running integration test for $contract_name in $dir"
        cargo run --package wasm-host --bin wasm-host -- -p "$contract_name" --dir $dir || exit 1
    fi
done

echo "✅ End-to-end tests completed successfully!"
