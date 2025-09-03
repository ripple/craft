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

printf "🔧 Building 'craft' ($PROFILE) \n"
cargo build $PROFILE_FLAG
cargo test $PROFILE_FLAG
cargo clippy --all-targets --all-features
cargo fmt --all -- --check

printf "\n🔧 Building WASM targets for projects \n"
# Build WASM for xrpl-std (library used by WASM projects)
cargo build -p xrpl-std $PROFILE_FLAG --target wasm32-unknown-unknown
(cd projects && cargo build --workspace $PROFILE_FLAG --target wasm32-unknown-unknown) || exit 1

printf "\n✅  All WASM projects built successfully\n\n"

printf "🔧 Running clippy on entire workspace \n"
cargo clippy --workspace --all-targets --all-features -- -Dclippy::all

printf "\n✅  Clippy checks completed successfully\n\n"

printf "🔧 Running fmt check on entire workspace \n"
cargo fmt --all -- --check

printf "\n✅  Format checks completed successfully\n\n"
printf "✅  All workspace builds completed successfully\n"
