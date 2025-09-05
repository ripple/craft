# XRPL Std Lib Example

This WebAssembly module is for testing the decoder in the simulated host.

## Building

Build using:

```bash
cargo build --target wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
```

The resulting WASM file will be located at:

```
./target/wasm32-unknown-unknown/release/decoder_tests.wasm
```

## Running with wasm-host

Run the contract using the wasm-host application:

```bash
cd ../../../
cargo run --package wasm-host --bin wasm-host -- --dir projects/e2e-tests/decoder_tests --project decoder_tests
```

### Note

Please note that the wasm-host only has mock host functions. Please use the devnet (or a standalone rippled node) to
test with real data.
