#![no_std]

use crate::host::{Result, Result::Err, Result::Ok};
use xrpl_std::core::ledger_objects::current_escrow::CurrentEscrow;
use xrpl_std::core::ledger_objects::current_escrow::get_current_escrow;
use xrpl_std::core::ledger_objects::ledger_object;
use xrpl_std::core::ledger_objects::traits::CurrentEscrowFields;
use xrpl_std::core::types::keylets;
use xrpl_std::host;
use xrpl_std::host::trace::{DataRepr, trace, trace_data, trace_num};
use xrpl_std::sfield;

#[unsafe(no_mangle)]
pub fn object_exists(
    keylet_result: Result<keylets::KeyletBytes>,
    keylet_type: &str,
    field: i32,
) -> Result<bool> {
    match keylet_result {
        Ok(keylet) => {
            let _ = trace_data(keylet_type, &keylet, DataRepr::AsHex);

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

    let escrow: CurrentEscrow = get_current_escrow();

    let account = escrow.get_account().unwrap_or_panic();
    let _ = trace_data("  Account:", &account.0, DataRepr::AsHex);

    let destination = escrow.get_destination().unwrap_or_panic();
    let _ = trace_data("  Destination:", &destination.0, DataRepr::AsHex);

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

    // created with sequence 3
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

    // created with sequence 4
    let cred_type: &[u8] = b"termsandconditions";
    let credential_keylet = keylets::credential_keylet(&account, &account, &cred_type);
    match object_exists(credential_keylet, "Credential", sfield::Subject) {
        Ok(exists) => {
            if exists {
                let _ = trace("  Credential object exists, proceeding with escrow finish.");
            } else {
                let _ = trace("  Credential object does not exist, aborting escrow finish.");
                return false;
            }
        }
        Err(_error) => return false,
    };

    // created with sequence 5
    let delegate_keylet = keylets::delegate_keylet(&account, &destination);
    match object_exists(delegate_keylet, "Delegate", sfield::Account) {
        Ok(exists) => {
            if exists {
                let _ = trace("  Delegate object exists, proceeding with escrow finish.");
            } else {
                let _ = trace("  Delegate object does not exist, aborting escrow finish.");
                return false;
            }
        }
        Err(_error) => return false,
    };

    // created with sequence 6
    let did_keylet = keylets::did_keylet(&account);
    match object_exists(did_keylet, "Account", sfield::Account) {
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

    // created with sequence 7
    let escrow_keylet = keylets::escrow_keylet(&account, 7);
    match object_exists(escrow_keylet, "Escrow", sfield::Account) {
        Ok(exists) => {
            if exists {
                let _ = trace("  Escrow object exists, proceeding with escrow finish.");
            } else {
                let _ = trace("  Escrow object does not exist, aborting escrow finish.");
                return false;
            }
        }
        Err(_error) => return false,
    };

    // created with sequence 8
    let nft_offer_keylet = keylets::nft_offer_keylet(&account, 8);
    match object_exists(nft_offer_keylet, "NFTokenOffer", sfield::Owner) {
        Ok(exists) => {
            if exists {
                let _ = trace("  NFTokenOffer object exists, proceeding with escrow finish.");
            } else {
                let _ = trace("  NFTokenOffer object does not exist, aborting escrow finish.");
                return false;
            }
        }
        Err(_error) => return false,
    };

    // created with sequence 9
    let paychan_keylet = keylets::paychan_keylet(&account, &destination, 9);
    match object_exists(paychan_keylet, "PayChannel", sfield::Account) {
        Ok(exists) => {
            if exists {
                let _ = trace("  PayChannel object exists, proceeding with escrow finish.");
            } else {
                let _ = trace("  PayChannel object does not exist, aborting escrow finish.");
                return false;
            }
        }
        Err(_error) => return false,
    };

    // created with sequence 10
    let signers_keylet = keylets::signers_keylet(&account);
    match object_exists(signers_keylet, "SignerList", sfield::Owner) {
        Ok(exists) => {
            if exists {
                let _ = trace("  SignerList object exists, proceeding with escrow finish.");
            } else {
                let _ = trace("  SignerList object does not exist, aborting escrow finish.");
                return false;
            }
        }
        Err(_error) => return false,
    };

    // created with sequence 11
    let ticket_keylet = keylets::ticket_keylet(&account, 11);
    match object_exists(ticket_keylet, "Ticket", sfield::Account) {
        Ok(exists) => {
            if exists {
                let _ = trace("  Ticket object exists, proceeding with escrow finish.");
            } else {
                let _ = trace("  Ticket object does not exist, aborting escrow finish.");
                return false;
            }
        }
        Err(_error) => return false,
    };

    true // All keylets exist, finish the escrow.
}
