#!/bin/bash
# Build and test script
# Mirrors the build_and_test job from GitHub Actions

set -euo pipefail

echo "🔧 Running build and test workflow..."

# Set RUSTFLAGS to match CI environment
export RUSTFLAGS="${RUSTFLAGS:-"-Dwarnings"}"

# Parse command line arguments for release mode
RELEASE_ARG=""
if [[ "${1:-}" == "release" ]]; then
    RELEASE_ARG="release"
    echo "🔧 Running in release mode..."
fi

echo "🏗️  Building projects..."
# Use the dedicated build script for consistency
./scripts/build.sh $RELEASE_ARG

echo "🧪 Running native workspace tests..."
# Run tests on the native workspace
cargo test --workspace

echo "🔧 Installing and running craft..."
cargo install --path craft --force
find ./projects/examples -name "Cargo.toml" -type f | while read -r cargo_file; do
    dir=$(dirname "$cargo_file")
    contract_name=$(basename "$dir")
    if [ -d "$dir/fixtures" ]; then
        echo "   Building contract: $contract_name"
        craft build $contract_name -r -O aggressive || exit 1
    fi
done

echo "✅ Build and test workflow completed successfully!"
