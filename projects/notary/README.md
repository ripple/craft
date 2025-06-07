# Notary Escrow FinishFunction

This WebAssembly module implements a notary-based escrow finish condition. It verifies that only a designated notary account is allowed to finish the escrow.

## Purpose

This module demonstrates implementation of a notary pattern where only a trusted third party can release funds.

## How it Works

The contract checks if the account attempting to finish the escrow matches a predefined notary account. Pseudo-code:

```
function finish() {
    let account = current_transaction::get_account();
    return account == "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh"
}
```

In a real implementation, this notary account could be a trusted entity responsible for validating that off-ledger conditions have been met before releasing the escrow.

## Functions

The contract exposes a single function as defined by the XLS-100d specification:

### `finish() -> bool`

Validates that the account in the current transaction matches the predefined notary account. Uses host functions to access transaction data rather than receiving it as parameters.

Returns `true` if the account attempting to finish the escrow is the authorized notary, otherwise `false`.

## Use Cases

This notary pattern can be used for:
1. **Trade escrows** - where a trusted third party verifies that goods have been delivered before releasing payment
2. **Escrow services** - where professional escrow agents manage the release of funds
3. **Regulatory compliance** - where a regulated entity must approve certain transactions

## Project Structure

This project is intentionally kept as an independent Rust project, separate from the main workspace. This allows:
- Independent building and testing
- Project-specific target directory
- Clear separation of the WASM module from the host application

## Building

Build using:

```bash
# Navigate to the project directory
cd projects/notary

# Build the WASM file
cargo build --target wasm32-unknown-unknown --release
```

The resulting WASM file will be located at:
```
./target/wasm32-unknown-unknown/release/notary.wasm
```

## Running with wasm-host

Run the contract using the wasm-host application:

```bash
cd ../../wasm-host
cargo run -- --wasm-file ../projects/notary/target/wasm32-unknown-unknown/release/notary.wasm --test-case success
```

## Modifying the Notary Account

To use a different notary account, modify the `NOTARY_ACCOUNT` constant in `src/lib.rs`:

```rust
// Notary account that is authorized to finish the escrow
const NOTARY_ACCOUNT: &str = "your_notary_account_here";
```

Then rebuild the WASM file. 
