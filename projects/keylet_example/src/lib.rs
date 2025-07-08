#![no_std]

use crate::host::{Result::Err, Result::Ok};
use xrpl_std::core::amount::Amount;
use xrpl_std::core::amount::xrp_amount::XrpAmount;
use xrpl_std::core::current_tx::escrow_finish::{EscrowFinish, get_current_escrow_finish};
use xrpl_std::core::current_tx::traits::{EscrowFinishFields, TransactionCommonFields};
use xrpl_std::core::keylets;
use xrpl_std::core::ledger_objects::current_escrow::{CurrentEscrow, get_current_escrow};
use xrpl_std::core::ledger_objects::traits::CurrentEscrowFields;
use xrpl_std::core::locator::Locator;
use xrpl_std::core::types::account_id::AccountID;
use xrpl_std::core::types::blob::Blob;
use xrpl_std::core::types::hash_256::Hash256;
use xrpl_std::core::types::transaction_type::TransactionType;
use xrpl_std::host;
use xrpl_std::host::trace::{DataRepr, trace, trace_data, trace_num};
use xrpl_std::locator::LocatorPacker;
use xrpl_std::sfield;

#[unsafe(no_mangle)]
pub fn object_exists(keylet: &KeyletBytes, slot: &u32, field: u32) -> Result<u32, host::Error> {
    slot = unsafe { host::cache_ledger_obj(keylet.as_ptr(), keylet.len(), slot) };
    if slot <= 0 {
        let _ = trace_num("Error: ", slot, DataRepr::AsHex);
        return Err(host::Error::NoFreeSlots);
    }
    let mut data = [0u8; 32]; // Adjust size as needed for the field
}

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> bool {
    let _ = trace("$$$$$ STARTING WASM EXECUTION $$$$$");
    let slot = 0;

    let escrow_finish: EscrowFinish = get_current_escrow_finish();
    let current_tx_id: Hash256 = escrow_finish.get_id().unwrap_or_panic();
    let _ = trace_data("  EscrowFinish TxId:", &current_tx_id.0, DataRepr::AsHex);

    let account = escrow_finish.get_account().unwrap_or_panic();
    let _ = trace_data("  Account:", &account.0, DataRepr::AsHex);

    let account_keylet = keylets::account_keylet(&account);
    let _ = trace_data("  Account Keylet:", &account_keylet, DataRepr::AsHex);
    match object_field() {
        Ok(opt_last_ledger_sequence) => {
            if let Some(last_ledger_sequence) = opt_last_ledger_sequence {
                let _ = trace_num("  LastLedgerSequence:", last_ledger_sequence as i64);
            }
        }
        Err(error) => {
            let _ = trace_num(
                "  Error getting LastLedgerSequence. error_code = ",
                error.code() as i64,
            );
        }
    };

    false // <-- If we get here, don't finish the escrow.
}
