#![no_std]

use crate::host::{Result, Result::Err, Result::Ok};
use xrpl_std::core::current_tx::escrow_finish::{EscrowFinish, get_current_escrow_finish};
use xrpl_std::core::current_tx::traits::{EscrowFinishFields, TransactionCommonFields};
use xrpl_std::core::ledger_objects::ledger_object;
use xrpl_std::core::types::hash_256::Hash256;
use xrpl_std::core::types::keylets;
use xrpl_std::host;
use xrpl_std::host::trace::{DataRepr, trace, trace_data, trace_num};
use xrpl_std::sfield;

#[unsafe(no_mangle)]
pub fn object_exists(
    keyletResult: Result<keylets::KeyletBytes>,
    keyletType: &str,
    field: i32,
) -> Result<bool> {
    match keyletResult {
        Ok(keylet) => {
            let _ = trace_data(keyletType, &keylet, DataRepr::AsHex);

            let slot = unsafe { host::cache_ledger_obj(keylet.as_ptr(), keylet.len(), 0) };
            if slot <= 0 {
                let _ = trace_num("Error: ", slot.into());
                return Err(host::Error::NoFreeSlots);
            }
            let _ = trace_num("Getting field: ", field.into());
            match ledger_object::get_account_id_field(slot, field) {
                Ok(data) => {
                    let _ = trace_data("Field data: ", &data.0, DataRepr::AsHex);
                }
                Err(result_code) => {
                    let _ = trace_num("Error getting field: ", result_code.into());
                    return Err(result_code);
                }
            }

            Ok(true)
        }
        Err(error) => {
            return Err(error.into());
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> bool {
    let _ = trace("$$$$$ STARTING WASM EXECUTION $$$$$");

    let escrow_finish: EscrowFinish = get_current_escrow_finish();
    let current_tx_id: Hash256 = escrow_finish.get_id().unwrap_or_panic();
    let _ = trace_data("  EscrowFinish TxId:", &current_tx_id.0, DataRepr::AsHex);

    let account = escrow_finish.get_account().unwrap_or_panic();
    let _ = trace_data("  Account:", &account.0, DataRepr::AsHex);

    let account_keylet = keylets::account_keylet(&account);
    match object_exists(account_keylet, "Account", sfield::Account) {
        Ok(exists) => {
            if exists {
                let _ = trace("  Check object exists, proceeding with escrow finish.");
            } else {
                let _ = trace("  Check object does not exist, aborting escrow finish.");
                return false;
            }
        }
        Err(_error) => return false,
    };

    let check_keylet = keylets::check_keylet(&account, 3);
    match object_exists(check_keylet, "Check", sfield::Account) {
        Ok(exists) => {
            if exists {
                let _ = trace("  Check object exists, proceeding with escrow finish.");
            } else {
                let _ = trace("  Check object does not exist, aborting escrow finish.");
                return false;
            }
        }
        Err(_error) => return false,
    };

    false // <-- If we get here, don't finish the escrow.
}
