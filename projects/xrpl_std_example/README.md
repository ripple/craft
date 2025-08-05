# XRPL Std Lib Example

This WebAssembly module is an example using the XRPL std lib. The WASM program in this example attempts to execute every
known host function, and then verify the results against expected values. This contract can be used as a type of canary
that can indicate if anything in Craft is not operating according to expectations.

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
