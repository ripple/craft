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

printf "🔧 Building default workspace members (native projects) \n"
cargo build $PROFILE_FLAG

printf "\n🔧 Testing default workspace members \n"
cargo test $PROFILE_FLAG

printf "\n🔧 Building WASM targets for smart contract projects \n"
# Build WASM for xrpl-std (library used by WASM projects)
cargo build -p xrpl-std $PROFILE_FLAG --target wasm32-unknown-unknown

# Build all smart contract projects for WASM target
WASM_PROJECTS=(
    "decoder_tests"
    "float_tests"
    "kyc"
    "ledger_sqn"
    "nft_owner"
    "notary"
    "notary_macro_example"
    "oracle"
    "trace_escrow_account"
    "trace_escrow_finish"
    "trace_escrow_ledger_object"
    "codecov_tests"
    "host_functions_test"
    "keylet_example"
)

for project in "${WASM_PROJECTS[@]}"; do
    printf "  Building WASM: $project\n"
    cargo build -p "$project" $PROFILE_FLAG --target wasm32-unknown-unknown || {
        printf "  ❌ Failed to build $project for WASM\n"
        exit 1
    }
done

printf "\n✅  All WASM projects built successfully\n\n"

printf "🔧 Running clippy on entire workspace \n"
cargo clippy --workspace --all-targets --all-features -- -Dclippy::all

printf "\n✅  Clippy checks completed successfully\n\n"

printf "🔧 Running fmt check on entire workspace \n"
cargo fmt --all -- --check

printf "\n✅  Format checks completed successfully\n\n"
printf "✅  All workspace builds completed successfully\n"
