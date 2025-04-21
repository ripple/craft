#![no_std]
#![no_main]
pub mod core;
mod field;
pub mod model;
pub mod utils;

// #[cfg(target_arch = "wasm32")]
pub mod host {
    #[link(wasm_import_module = "host")]
    unsafe extern "C" {
        /// Log a byte array of size `len` as a UTF-8 string.
        pub fn log(str_ptr: *const u8, len: usize);

        /// Log a byte array of size `len` as a UTF-8 string (with a trailing newline).
        pub fn log_ln(str_ptr: *const u8, len: usize);

        /// Log a byte array of size `len` as a hex string.
        pub fn log_hex(byte_ptr: *const u8, len: usize);

        /// Get the transaction id of the EscrowFinish transaction that instigated a Smart Escrow
        /// WASM execution.
        pub fn get_tx_hash(arr_ptr: *const u8);
    }
}

// TODO: For testing purposes, uncomment the cfg directive above and below, and implement Rust
// variants of the host functions. This would be for testing purposes only.
// #[cfg(not(target_arch = "wasm32"))]
