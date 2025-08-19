# XRPL Std Lib Example

This WebAssembly module is an example using the XRPL std lib to determine if an object identified by a keylet exists.

## Building

Build using:

```bash
# Navigate to the project directory
cd projects/keylet_exists

# Build the WASM file
cargo build --target wasm32-unknown-unknown --release
```

The resulting WASM file will be located at:

```
./target/wasm32-unknown-unknown/release/keylet_example.wasm
```

## Running with wasm-host

Run the contract using the wasm-host application:

```bash
cd ../../wasm-host
cargo run -- --wasm-file ../projects/keylet_exists/target/wasm32-unknown-unknown/release/keylet_exists.wasm --function finish
```

### Note

Please note that the wasm-host only has mock host functions. Please use the devnet (or a standalone rippled node) to
test with real data.
