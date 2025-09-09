#!/bin/bash
# Clippy linting script
# Mirrors the clippy_linting job from GitHub Actions

set -euo pipefail

echo "🔧 Running Clippy linting..."

# Set RUSTFLAGS to match CI environment
export RUSTFLAGS="${RUSTFLAGS:-"-Dwarnings"}"

# Ensure wasm32 target is available
echo "📦 Ensuring wasm32-unknown-unknown target is installed..."
rustup target add wasm32-unknown-unknown

echo "🔍 Running Clippy on Native Workspace..."
cargo clippy --workspace --all-targets --all-features -- -Dclippy::all

echo "🔍 Running Clippy on WASM Projects Workspace..."
cd projects
cargo clippy --workspace --target wasm32-unknown-unknown --all-features -- -Dclippy::all
cd ..

echo "🔍 Checking WASM contract exports..."
# Check that all WASM projects export the required finish function
find ./projects -type d -name "src" | while read -r src_dir; do
    dir=$(dirname "$src_dir")
    echo "🔧 Checking exports in $dir"
    if [[ -f "$src_dir/lib.rs" ]]; then
        grep -q "finish() -> i32" "$src_dir/lib.rs" || {
            echo "❌ Missing required finish() -> i32 export in $dir"
            exit 1
        }
    else
        echo "❌ Missing lib.rs in $src_dir"
        exit 1
    fi
done

echo "✅ Clippy linting passed!"
