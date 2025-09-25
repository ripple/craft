# XRPL WASM Projects

This directory contains various Craft projects including end-to-end tests, integration test suites, and smart escrow examples.

## Building

### Automatic Smart Escrow Building

When you run `cargo build` in this directory, all smart escrow examples in `examples/smart-escrows/` are automatically built via the `build.rs` script. Each smart escrow project maintains its own `target` directory with individual build artifacts.

```bash
# Build everything (workspace + smart escrows)
cargo build

# Build for WASM target
cargo build --target wasm32-unknown-unknown

# Build release versions
cargo build --release
cargo build --target wasm32-unknown-unknown --release
```

### Smart Escrow Projects

The following smart escrow examples are automatically built:

- `kyc` - Unlocks based on KYC credentials on the XRPL
- `ledger_sqn` - Unlocks when ledger sequence number reaches threshold
- `nft_owner` - Unlocks when destination account owns specific NFT
- `notary` - Unlocks when EscrowFinish is signed by designated notary
- `notary_macro_example` - Demonstrates compile-time XRPL address conversion
- `oracle` - Unlocks when oracle price data meets threshold conditions

Each project is standalone and can also be built individually by navigating to its directory and running `cargo build`.

### WASM Output

WASM files are generated in each project's individual `target/wasm32-unknown-unknown/debug/` and `target/wasm32-unknown-unknown/release/` directories.

### Adding New Projects

Add your own projects here by creating a new directory and adding a project layout similar to the existing examples.
