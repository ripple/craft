#!/bin/bash

# Set default profile to dev if not provided
PROFILE=${1:-dev}
PROFILE_FLAG=""

# If profile is not "dev", add the --profile flag
if [ "$PROFILE" != "dev" ]; then
  PROFILE_FLAG="--profile $PROFILE"
fi

echo "🔧 Building ALL with profile: $PROFILE"

cd xrpl-std || exit
echo "🔧 Building 'xrpl-std' ($PROFILE)"
cargo build $PROFILE_FLAG
cargo test $PROFILE_FLAG
cargo build $PROFILE_FLAG --target wasm32-unknown-unknown
cargo clippy --all-targets --all-features
cargo fmt --all -- --check
echo "✅ 'xrpl-std' project built successfully"

cd ..
cd ./wasm-host || exit
echo "🔧 Building 'xrpl-host' ($PROFILE)"
cargo build $PROFILE_FLAG
cargo test $PROFILE_FLAG
cargo clippy --all-targets --all-features
cargo fmt --all -- --check
echo "✅  'xrpl-host' project built successfully"

cd .. || exit

echo "🔧 Setting NOTARY_ACCOUNT_R to rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh"
export NOTARY_ACCOUNT_R=rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh

find ./projects -name "Cargo.toml" -type f | while read -r cargo_file; do
  dir=$(dirname "$cargo_file")
  echo "🔧 Building example WASM: $dir"
  (cd "$dir" && cargo build $PROFILE_FLAG --target wasm32-unknown-unknown) || exit 1
done

echo "✅  All WASM examples built successfully"

find ./projects -name "Cargo.toml" -type f | while read -r cargo_file; do
  dir=$(dirname "$cargo_file")
  echo "🔧 Building example Rust: $dir"
  (cd "$dir" && cargo build $PROFILE_FLAG) || exit 1
done

echo "✅  All Rust examples built successfully"

find ./projects -name "Cargo.toml" -type f | while read -r cargo_file; do
  dir=$(dirname "$cargo_file")
  echo "🔧 cargo fmt for $dir"
  (cd "$dir" && cargo fmt --all -- --check) || exit 1
done

echo "✅  All 'cargo fmt' checks completed successfully"

find ./projects -name "Cargo.toml" -type f | while read -r cargo_file; do
  dir=$(dirname "$cargo_file")
  echo "🔧 'cargo clippy' for $dir"
  (cd "$dir" && cargo clippy --all-targets --all-features) || exit 1
done

echo "✅  All 'cargo clippy' checks completed successfully"

cd ../.. || exit
