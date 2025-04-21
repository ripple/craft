#![no_std]
#![no_main]
extern crate alloc;

pub mod core;
mod field;
pub mod model;
pub mod utils;

// #[cfg(target_arch = "wasm32")]
pub mod host {
    #[link(wasm_import_module = "host")]
    unsafe extern "C" {
        /// Log a string to std_out using the host for actual emission.
        pub fn log(str_ptr: *const u8, len: usize);

        // pub fn getLedgerSqn() -> i32;
        // pub fn getTxField(fname_ptr: i32, fname_len: i32) -> i32;
        // pub fn print(str_ptr: i32, str_len: i32);

        pub fn add(a: i32, b: i32) -> i32;

        /// Get the transaction id of the EscrowFinish transaction that instigated a Smart Escrow
        /// Execution.
        pub fn get_tx_hash(arr_ptr: *const u8);

        // Obtain the specified account's current IOU balance.
        // pub fn getAccountBalanceSTAmount() -> u64;
    }
}
