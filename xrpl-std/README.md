# xrpl-std

Standard library for XRPL WebAssembly smart contracts.

This crate provides the core functionality needed to develop smart contracts for the XRP Ledger, including:
- Host function bindings for interacting with the XRPL
- Type definitions for XRPL objects and transactions
- No-std compatible implementation for WASM environments

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
xrpl-std = "0.6.0"
```

## Example

```rust
#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_std::core::current_tx::escrow_finish;
use xrpl_std::core::current_tx::traits::TransactionCommonFields;
use xrpl_std::host::trace::trace_num;
use xrpl_std::host::{Result::Err, Result::Ok};

// Example authorized account (20-byte account ID)
// This represents a real XRPL account address in its raw 20-byte format
const AUTHORIZED_ACCOUNT: [u8; 20] = [
    0xd5, 0xb9, 0x84, 0x56, 0x50, 0x9f, 0x20, 0xb5, 0x27, 0x9d,
    0x1e, 0x4a, 0x2e, 0xe8, 0xb2, 0xaa, 0x82, 0xae, 0x63, 0xe3
];

#[no_mangle]
pub extern "C" fn finish() -> i32 {
    // Get the current escrow finish transaction
    let escrow_finish = escrow_finish::get_current_escrow_finish();

    // Access transaction fields
    let account = match escrow_finish.get_account() {
        Ok(v) => v,
        Err(e) => {
            let _ = trace_num("Error getting account:", e.code() as i64);
            return e.code(); // Return error code on failure
        }
    };

    // Make escrow release decision based on account
    if account.0 == AUTHORIZED_ACCOUNT {
        1  // Release escrow
    } else {
        0  // Keep escrow locked
    }
}
```

### Using classic (r...) addresses

Contracts compare 20-byte AccountID values. If you have a classic XRPL address (r...) during development, convert it to 20 bytes at build time and embed it as a constant. See `projects/notary` which accepts `NOTARY_ACCOUNT_R=r...` and generates a `[u8; 20]` at build time for comparison inside the WASM.

### Build and run your contract

Build a contract for WASM and run it with the host:

```bash
cargo build --target wasm32-unknown-unknown --release
# From the wasm-host crate:
cargo run -p wasm-host -- --wasm-file path/to/your.wasm --function finish
```

## Features

- **No-std compatible**: Designed for WebAssembly environments
- **Type-safe API**: Strongly typed interfaces for XRPL objects
- **Host function access**: Direct bindings to XRPL validator functions
- **Memory safety**: Built-in panic handler and allocation management

## Documentation

For more information about developing XRPL smart contracts, see the [craft repository](https://github.com/XRPLF/craft).

## License

This project is licensed under the ISC License.
