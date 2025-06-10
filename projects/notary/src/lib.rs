#![no_std]

use xrpl_std::core::tx::current_transaction;
use xrpl_std::host::trace::trace_num;
use xrpl_std::host::{Result::Err, Result::Ok};

// Notary account that is authorized to finish the escrow
const NOTARY_ACCOUNT: &[u8] = b"rPPLRQwB3KGvpfDMABZucA8ifJJcvQhHD3"; // Account 2 (example)

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> bool {
    let tx_account = match current_transaction::get_account() {
        Ok(v) => v,
        Err(e) => {
            let _ = trace_num("Error in Notary contract", e.code() as i64);
            return false; // Must return to short circuit.
        }
    };

    tx_account.0 == NOTARY_ACCOUNT
}
