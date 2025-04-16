#!/bin/bash
echo "==> Build xrpl-std-lib..."
cd ./xrpl-std-lib || exit
cargo build
cargo build --target wasm32-unknown-unknown
echo "==> Build Craft and wasm-host..."
cd ..
cargo build
echo "==> Build xrpl_std_example..."
cd ./projects/xrpl_std_example || exit
cargo build --target wasm32-unknown-unknown
echo "==> Build COMPLETE!"