# Groth16 Smart Escrow Example

This example shows a minimal smart-escrow that would verify Groth16 zk-SNARK proofs via host helper
functions. It's intentionally minimal: it doesn't bundle large proving/verifying keys.

Build and run (example):

```bash
# Build the wasm
cargo build --manifest-path projects/examples/smart-escrows/groth16/Cargo.toml --target wasm32-unknown-unknown --release

# Run with wasm-host (adjust path and project name as needed)
cargo run --package wasm-host --bin wasm-host -- --dir projects/examples/smart-escrows/groth16 --project groth16
```

Notes:
- To actually verify Groth16 proofs you'd typically implement a host function that depends on
  arkworks crates; for the example we keep the contract minimal and recommend adding host-side
  verification for realistic usage.
