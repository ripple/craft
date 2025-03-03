# JSON AccountID Comparison Prototype

This is a simple WebAssembly prorotype that compares account IDs in two JSON strings.

## Purpose

This smart contract demonstrates:
1. How to parse JSON data in a WebAssembly environment
2. Memory management between host and WASM
3. The contract interface for account ID validation

## Functions

The contract exposes two main functions:

### `allocate(size: usize) -> *mut u8`

Allocates memory of the specified size and returns a pointer to it.

### `compare_accountID(tx_json_ptr: *mut u8, tx_json_size: usize, lo_json_ptr: *mut u8, lo_json_size: usize) -> bool`

Compares the "Account" field in two JSON objects:
- `tx_json_ptr`: Pointer to the transaction JSON data
- `tx_json_size`: Size of the transaction JSON data
- `lo_json_ptr`: Pointer to the ledger object JSON data
- `lo_json_size`: Size of the ledger object JSON data

Returns `true` if the accounts match, otherwise `false`.

## Project Structure

This project is intentionally kept as an independent Rust project, separate from the main workspace. This allows:
- Independent building and testing
- Project-specific target directory
- Clear separation of the smart contract from the host application

## Building

Build using:

```bash
# Navigate to the project directory
cd projects/json_account_id_compare

# Build the WASM file
cargo build --target wasm32-unknown-unknown --release
```

The resulting WASM file will be located at:
```
./target/wasm32-unknown-unknown/release/json_account_id_compare.wasm
```

## Running with wasm-host

Run the contract using the wasm-host application:

```bash
cd ../../wasm-host
cargo run -- --wasm-file ../projects/json_account_id_compare/target/wasm32-unknown-unknown/release/json_account_id_compare.wasm --function compare_accountID
```