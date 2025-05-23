#![no_std]
#![allow(unused_imports)]
use xrpl_std::host::trace::{trace_data, trace_num, DataRepr};
use xrpl_std::locator::LocatorPacker;
use xrpl_std::sfield::{SignerEntries, SignerEntry, SignerWeight};
use xrpl_std::{
    get_account_balance, get_current_escrow_account_id, get_current_escrow_destination,
    get_current_escrow_finish_after, get_tx_account_id,
};

#[no_mangle]
pub extern "C" fn finish() -> i32 {
    {
        let account_id_tx = match get_tx_account_id() {
            Some(v) => v,
            None => return -1,
        };
        let _ = trace_data("  Account:", &account_id_tx, DataRepr::AsHex);

        let balance = match get_account_balance(&account_id_tx) {
            Some(v) => v,
            None => return -5,
        };
        let _ = trace_num("  Fee:", balance as i64);

        if balance <= 0 {
            return -9;
        }
    }

    1
}
