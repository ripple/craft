#!/bin/bash
# Build script - replaces the missing build.sh referenced in e2e-tests
# Based on the build steps from build_and_test job

set -euo pipefail

# Parse command line arguments
RELEASE_MODE=""
if [[ "${1:-}" == "release" ]]; then
    RELEASE_MODE="--release"
    echo "🔧 Building in release mode..."
else
    echo "🔧 Building in debug mode..."
fi

# Set RUSTFLAGS to match CI environment
export RUSTFLAGS="${RUSTFLAGS:-"-Dwarnings"}"

# Ensure wasm32 target is available
echo "📦 Ensuring wasm32-unknown-unknown target is installed..."
rustup target add wasm32-unknown-unknown

echo "🏗️  Building Native Workspace..."
cargo build --workspace $RELEASE_MODE

echo "🏗️  Building xrpl-std for WASM..."
cargo build -p xrpl-std --target wasm32-unknown-unknown $RELEASE_MODE
cargo rustc -p xrpl-std --target wasm32-unknown-unknown $RELEASE_MODE -- -D warnings

echo "🏗️  Building WASM Projects Workspace..."
cd projects
echo "🔧 Building projects workspace for WASM"
if [[ -n "$RELEASE_MODE" ]]; then
    # Only build release if specifically requested
    cargo build --workspace --target wasm32-unknown-unknown $RELEASE_MODE
else
    # Build both debug and release when no specific mode is requested
    cargo build --workspace --target wasm32-unknown-unknown
    cargo build --workspace --target wasm32-unknown-unknown --release
fi
cd ..

echo "✅ Build completed successfully!"
