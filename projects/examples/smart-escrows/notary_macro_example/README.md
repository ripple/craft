# NFT Owner Smart Escrow

Smart Escrow example demonstrating compile-time XRPL r-address to hex conversion using procedural macros.

## Overview

This smart escrow unlocks when the account finishing the escrow is the same as the pre-programmaed notary account.
Otherwise, the escrow does not unlock.

## Building

### Build Commands

```bash
cargo build --target wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
```

The resulting WASM file will be located at:

```
./target/wasm32-unknown-unknown/release/notary_macro_example.wasm
```

## Running with wasm-host

Run the contract using the wasm-host application:

```bash
cd ../../../../wasm-host
cargo run -- --wasm-file ../projects/examples/smart-escrows/notary_macro_example/target/wasm32-unknown-unknown/release/notary_macro_example.wasm --project examples/smart-escrows/notary_macro_example
```
