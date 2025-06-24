#![no_std]

use xrpl_std::core::ledger_objects::current_escrow;
use xrpl_std::core::ledger_objects::current_escrow::CurrentEscrow;
use xrpl_std::core::ledger_objects::traits::CurrentEscrowFields;
use xrpl_std::core::types::keylets::credential_keylet;
use xrpl_std::host::trace::{trace_data, trace_num, DataRepr};
use xrpl_std::host::{Result::Err, Result::Ok};

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> bool {
    let current_escrow: CurrentEscrow = current_escrow::get_current_escrow();

    let account_id = match current_escrow.get_account() {
        Ok(account_id) => account_id,
        Err(e) => {
            let _ = trace_num("Error getting account_id", e.code() as i64);
            return false; // <-- Do not execute the escrow.
        }
    };

    // "termsandconditions" in hex
    let cred_type: &[u8] = b"termsandconditions";
    match credential_keylet(&account_id, &account_id, &cred_type) {
        Ok(cred_keylet) => {
            let _ = trace_data("cred_keylet", &cred_keylet, DataRepr::AsHex);

            let slot =
                unsafe { xrpl_std::host::cache_ledger_obj(cred_keylet.as_ptr(), cred_keylet.len(), 0) };
            if slot < 0 {
                let _ = trace_num("CACHE ERROR", i64::from(slot));
                return false;
            };
            true
        },
        Err(e) => {
            let _ = trace_num("Error getting account_id", e.code() as i64);
            false // <-- Do not execute the escrow.
        }
    }
}
