# XRPL Std Lib Example

This WebAssembly module is for testing the decoder in the simulated host.

## Building

Build using:

```bash
# Navigate to the project directory
cd projects/float_tests

# Build the WASM file
cargo build --target wasm32-unknown-unknown --release
```

The resulting WASM file will be located at:

```
./target/wasm32-unknown-unknown/release/float_tests.wasm
```

## Running with wasm-host

Run the contract using the wasm-host application:

```bash
cd ../../wasm-host
cargo run -- --wasm-file ../projects/float_tests/target/wasm32-unknown-unknown/release/float_tests.wasm --function finish
```

### Note

Please note that the wasm-host only has mock host functions. Please use the devnet (or a standalone rippled node) to
test with real data.  