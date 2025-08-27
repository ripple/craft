#!/bin/bash

# Set default profile to dev if not provided
PROFILE=${1:-dev}
PROFILE_FLAG=""

# If profile is not "dev", add the --profile flag
if [ "$PROFILE" != "dev" ]; then
  PROFILE_FLAG="--profile $PROFILE"
fi

printf "🔧 Building ALL workspace projects with profile: $PROFILE \n"

printf "🔧 Setting NOTARY_ACCOUNT_R to rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh\n\n"
export NOTARY_ACCOUNT_R=rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh

printf "🔧 Building entire workspace (native) \n"
cargo build --workspace $PROFILE_FLAG

printf "\n🔧 Testing entire workspace \n"
cargo test --workspace $PROFILE_FLAG

printf "\n🔧 Building WASM targets for applicable projects \n"
# Build WASM for xrpl-std and all example projects
cargo build -p xrpl-std $PROFILE_FLAG --target wasm32-unknown-unknown
cargo build -p decoder_tests $PROFILE_FLAG --target wasm32-unknown-unknown
cargo build -p float_tests $PROFILE_FLAG --target wasm32-unknown-unknown
cargo build -p kyc $PROFILE_FLAG --target wasm32-unknown-unknown
cargo build -p ledger_sqn $PROFILE_FLAG --target wasm32-unknown-unknown
cargo build -p nft_owner $PROFILE_FLAG --target wasm32-unknown-unknown
cargo build -p notary $PROFILE_FLAG --target wasm32-unknown-unknown
cargo build -p notary_macro_example $PROFILE_FLAG --target wasm32-unknown-unknown
cargo build -p oracle $PROFILE_FLAG --target wasm32-unknown-unknown
cargo build -p trace_escrow_account $PROFILE_FLAG --target wasm32-unknown-unknown
cargo build -p trace_escrow_finish $PROFILE_FLAG --target wasm32-unknown-unknown
cargo build -p trace_escrow_ledger_object $PROFILE_FLAG --target wasm32-unknown-unknown
cargo build -p codecov_tests $PROFILE_FLAG --target wasm32-unknown-unknown
cargo build -p host_functions_test $PROFILE_FLAG --target wasm32-unknown-unknown
cargo build -p keylet_example $PROFILE_FLAG --target wasm32-unknown-unknown

printf "\n✅  All WASM projects built successfully\n\n"

printf "🔧 Running clippy on entire workspace \n"
cargo clippy --workspace --all-targets --all-features -- -Dclippy::all

printf "\n✅  Clippy checks completed successfully\n\n"

printf "🔧 Running fmt check on entire workspace \n"
cargo fmt --all -- --check

printf "\n✅  Format checks completed successfully\n\n"
printf "✅  All workspace builds completed successfully\n"
