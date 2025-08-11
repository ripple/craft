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
xrpl-std = "0.5.0-devnet5"
```

## Example

```rust
#![no_std]
use xrpl_std::core::current_tx::escrow_finish;
use xrpl_std::core::current_tx::traits::TransactionCommonFields;

#[no_mangle]
pub extern "C" fn finish() -> i32 {
    // Get the current escrow finish transaction
    let escrow_finish = escrow_finish::get_current_escrow_finish();

    // Access transaction fields
    let account = match escrow_finish.get_account() {
        Ok(v) => v,
        Err(_) => return 0,  // Keep escrow locked on error
    };

    // Make escrow release decision based on account
    if account.0 == b"rAuthorizedAccount..." {
        1  // Release escrow
    } else {
        0  // Keep escrow locked
    }
}
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
