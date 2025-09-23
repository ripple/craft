# NFT Owner Smart Escrow

Smart Escrow example demonstrating compile-time XRPL r-address to hex conversion using procedural macros.

## Overview

This smart escrow unlocks when the account finishing the escrow is the same as the pre-programmaed notary account.
Otherwise, the escrow does not unlock.

## Building

### Build Commands

```bash
cargo build
cargo build --target wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
```

The resulting WASM file will be located at:

```
./target/wasm32-unknown-unknown/release/notary_macro_example.wasm
```

## Running with wasm-host-simulator

Run the contract using the wasm-host-simulator application:

```bash
cd ../../../..
cargo run --package wasm-host-simulator --bin wasm-host-simulator -- --dir projects/examples/smart-escrows/notary_macro_example --project notary_macro_example
```
