#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_std::core::current_tx::escrow_finish;
use xrpl_std::core::current_tx::traits::TransactionCommonFields;
use xrpl_std::host::trace::trace_num;
use xrpl_std::host::{Result::Err, Result::Ok};

// Generated at build time from the classic (r...) address provided via NOTARY_ACCOUNT_R
// Fallback default is the XRPL master account if NOTARY_ACCOUNT_R is not set.
include!(concat!(env!("OUT_DIR"), "/notary_generated.rs"));

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    let escrow_finish = escrow_finish::get_current_escrow_finish();
    let tx_account = match escrow_finish.get_account() {
        Ok(v) => v,
        Err(e) => {
            let _ = trace_num("Error in Notary contract", e.code() as i64);
            return e.code(); // Must return to short circuit.
        }
    };

    (tx_account.0 == NOTARY_ACCOUNT) as i32
}
