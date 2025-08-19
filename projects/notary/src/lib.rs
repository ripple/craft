#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_std::core::current_tx::escrow_finish;
use xrpl_std::core::current_tx::traits::TransactionCommonFields;
use xrpl_std::host::trace::trace_num;
use xrpl_std::host::{Result::Err, Result::Ok};

// Two options for specifying the notary account:
//
// Option 1: Use the macro directly (compile-time constant)
// Uncomment this and comment out Option 2 to use:
// use xrpl_address_macro::r_address;
// const NOTARY_ACCOUNT: [u8; 20] = r_address!("rPPL...");
//
// Option 2: Use build script with environment variable (current implementation)
// Generated at build time from the classic (r...) address provided via NOTARY_ACCOUNT_R
// Fallback default is the XRPL master account if NOTARY_ACCOUNT_R is not set.
include!(concat!(env!("OUT_DIR"), "/notary_generated.rs"));

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    let escrow_finish = escrow_finish::get_current_escrow_finish();
    let tx_account = match escrow_finish.get_account() {
        Ok(v) => v,
        Err(e) => {
            let error_code = e.code();
            let _ = trace_num("Error in Notary contract", error_code as i64);
            return error_code; // Must return to short circuit.
        }
    };

    (tx_account.0 == NOTARY_ACCOUNT) as i32
}
