#![no_std]
#![no_main]

pub mod core;
mod field;
mod mocks;
pub mod model;
pub mod string;
pub mod util;

// #[cfg(target_arch = "wasm32")]
pub mod host {
    #[link(wasm_import_module = "host")]
    unsafe extern "C" {
        /// Log a string to std_out using the host for actual emission.
        pub fn log(str_ptr: *const u8, len: usize);

        // pub fn getLedgerSqn() -> i32;
        pub fn getCurrentTxField(sfield: i32) -> i32;
        // pub fn print(str_ptr: i32, str_len: i32);

        pub fn add(a: i32, b: i32) -> i32;

        // Obtain the specified account's current IOU balance.
        // pub fn getAccountBalanceSTAmount() -> u64;
    }
}
