#!/bin/bash

# Set default profile to dev if not provided
PROFILE=${1:-dev}
PROFILE_FLAG=""

# If profile is not "dev", add the --profile flag
if [ "$PROFILE" != "dev" ]; then
  PROFILE_FLAG="--profile $PROFILE"
fi

echo "🔧 Building with profile: $PROFILE"

cd xrpl-std || exit
cargo build $PROFILE_FLAG
cargo build $PROFILE_FLAG --target wasm32-unknown-unknown
cd ..
cd ./wasm-host || exit
cargo build $PROFILE_FLAG
cd .. || exit
for dir in ./projects/*/; do
  if [ -f "$dir/Cargo.toml" ]; then
    echo "🔧 Building in $dir"
    (cd "$dir" && cargo build $PROFILE_FLAG --target wasm32-unknown-unknown) || exit 1
  fi
done

echo "✅  All WASM builds completed successfully"

for dir in ./projects/*/; do
  if [ -f "$dir/Cargo.toml" ]; then
    echo "🔧 Building in $dir"
    (cd "$dir" && cargo build $PROFILE_FLAG) || exit 1
  fi
done

echo "✅  All Rust builds completed successfully"

for dir in ./projects/*/; do
  if [ -f "$dir/Cargo.toml" ]; then
    echo "🔧 Building in $dir"
    (cd "$dir" && cargo fmt --all -- --check) || exit 1
  fi
done

echo "✅  All 'cargo fmt' checks completed successfully"

for dir in ./projects/*/; do
  if [ -f "$dir/Cargo.toml" ]; then
    echo "🔧 Building in $dir"
    (cd "$dir" && cargo clippy --all-targets --all-features) || exit 1
  fi
done

echo "✅  All 'cargo clippy' checks completed successfully"

cd ../.. || exit
