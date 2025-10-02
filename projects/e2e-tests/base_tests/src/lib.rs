#![allow(unused_imports)]
#![allow(unused_variables)]
#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_std::host::trace::{trace, DataRepr, trace_data};
use xrpl_std::core::types::amount::token_amount::TokenAmount;

#[unsafe(no_mangle)]
pub extern "C" fn base() -> i32 {
    let _ = trace("\n$$$ test_float_invert $$$");

   const FEE: [u8; 8] = [
        0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
    ];
    let fee = match TokenAmount::from_bytes(&FEE) {
        Ok(f) => f,
        Err(_) => {
            let _ = trace("Failed to parse fee TokenAmount");
            return -11;
        }
    };
    0 // <-- Finish the escrow to indicate a successful outcome
}