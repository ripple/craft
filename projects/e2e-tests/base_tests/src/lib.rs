#![allow(unused_imports)]
#![allow(unused_variables)]
#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_wasm_std::host::trace::{trace, DataRepr, trace_data};
use xrpl_wasm_std::core::types::amount::token_amount::TokenAmount;

#[unsafe(no_mangle)]
pub extern "C" fn base() -> i32 {
    let _ = trace("\n$$$ test_float_invert $$$");
    0 // <-- Finish the escrow to indicate a successful outcome
}