# XRPL Std Lib Example

This WebAssembly module demonstrates how to use the XRPL std lib to access every field in an `AccountRoot` ledger object
associated with a Smart Escrow. The WASM program is meant to both execute known host functions required for accessing
the fields of an `AccountRoot` and also validate that field access is working correctly by asserting each value that
craft makes available. In this way, this Smart Escrow can be used as a type of canary that can indicate if anything in
Craft is not operating according to expectations.

## Building

Build using:

```bash
cargo build
cargo build --target wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
```

The resulting WASM file will be located at:

```
./target/wasm32-unknown-unknown/release/trace_escrow_account.wasm
```

## Running with wasm-host

Run the contract using the wasm-host application:

```bash
cd ../../../wasm-host
cargo run -- --dir ../projects/e2e-tests/trace_escrow_account --project trace_escrow_account
```

### Note

Please note that the wasm-host only has mock host functions. Please use the devnet (or a standalone rippled node) to
test with a real implementation and real data.
