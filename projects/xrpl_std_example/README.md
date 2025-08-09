# XRPL Std Lib Example

This WebAssembly module demonstrates how to use the XRPL std lib to access every field in the current escrow ledger
object, the escrow finish transaction, and arbitrary account root object in the ledger. The WASM program is meant to
both execute every known host function, but also validate that object access is working correctly by asserting each
value that craft makes available. In this way, this Smart Escrow can be used as a type of canary that can indicate if
anything in Craft is not operating according to expectations.

## Building

Build using:

```bash
# Navigate to the project directory
cd projects/xrpl_std_example

# Build the WASM file
cargo build --target wasm32-unknown-unknown --release
```

The resulting WASM file will be located at:

```
./target/wasm32-unknown-unknown/release/xrpl_std_example.wasm
```

## Running with wasm-host

Run the contract using the wasm-host application:

```bash
cd ../../wasm-host
cargo run -- --wasm-file ../projects/xrpl_std_example/target/wasm32-unknown-unknown/release/xrpl_std_example.wasm --project xrpl_std_example
```

### Note

Please note that the wasm-host only has mock host functions. Please use the devnet (or a standalone rippled node) to
test with a real implementation and real data.
