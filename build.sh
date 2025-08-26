#!/bin/bash

# Set default profile to dev if not provided
PROFILE=${1:-dev}
PROFILE_FLAG=""

# If profile is not "dev", add the --profile flag
if [ "$PROFILE" != "dev" ]; then
  PROFILE_FLAG="--profile $PROFILE"
fi


printf "🔧 Building ALL with profile: $PROFILE \n"

printf "🔧 Building 'craft' ($PROFILE) \n"
cargo build $PROFILE_FLAG
cargo test $PROFILE_FLAG
cargo clippy --all-targets --all-features
cargo fmt --all -- --check

printf "\n✅ 'Craft' project built successfully\n\n"


cd xrpl-std || exit
printf "🔧 Building 'xrpl-std' ($PROFILE) \n"
cargo build $PROFILE_FLAG
cargo test $PROFILE_FLAG
cargo build $PROFILE_FLAG --target wasm32-unknown-unknown
cargo clippy --all-targets --all-features
cargo fmt --all -- --check

printf "\n✅ 'xrpl-std' project built successfully\n\n"

cd ..
cd ./wasm-host || exit
printf "🔧 Building 'wasm-host' ($PROFILE) \n"
cargo build $PROFILE_FLAG
cargo test $PROFILE_FLAG
cargo clippy --all-targets --all-features
cargo fmt --all -- --check

printf "\n✅  'wasm-host' project built successfully\n\n"

cd .. || exit

printf "🔧 Setting NOTARY_ACCOUNT_R to rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh\n\n"
export NOTARY_ACCOUNT_R=rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh

find ./projects -name "Cargo.toml" -type f | while read -r cargo_file; do
  dir=$(dirname "$cargo_file")
  printf "🔧 Building example WASM: $dir \n"
  (cd "$dir" && cargo build $PROFILE_FLAG --target wasm32-unknown-unknown) || exit 1
done

printf "\n✅  All WASM examples built successfully\n\n"

find ./projects -name "Cargo.toml" -type f | while read -r cargo_file; do
  dir=$(dirname "$cargo_file")
  printf "🔧 Building example Rust: $dir \n"
  (cd "$dir" && cargo build $PROFILE_FLAG) || exit 1
done

printf "\n✅  All Rust examples built successfully\n\n"

find ./projects -name "Cargo.toml" -type f | while read -r cargo_file; do
  dir=$(dirname "$cargo_file")
  printf "🔧 cargo fmt for $dir \n"
  (cd "$dir" && cargo fmt --all -- --check) || exit 1
done

printf "\n✅  All 'cargo fmt' checks completed successfully\n\n"

find ./projects -name "Cargo.toml" -type f | while read -r cargo_file; do
  dir=$(dirname "$cargo_file")
  printf "🔧 'cargo clippy' for $dir"
  (cd "$dir" && cargo clippy --all-targets --all-features -- -Dclippy::all) || exit 1
done

printf "\n✅  All 'cargo clippy' checks completed successfully\n\n"

cd ../.. || exit
