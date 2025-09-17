#!/bin/bash
# Clippy linting script
# Mirrors the clippy_linting job from GitHub Actions

set -euo pipefail

# Change to the repository root directory (where this script's grandparent directory is located)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

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

echo "✅ Clippy linting passed!"
