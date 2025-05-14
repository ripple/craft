#![no_std]
use xrpl_std::{get_current_escrow_account_id, get_current_escrow_destination, get_tx_account_id,
               get_current_escrow_finish_after, get_account_balance};

#[no_mangle]
pub extern "C" fn ready() -> bool {
    let account_id_tx = match get_tx_account_id() {
        Some(v) => v,
        None => return false,
    };

    let account_id_clo = match get_current_escrow_account_id() {
        Some(v) => v,
        None => return false,
    };

    let destination = match get_current_escrow_destination() {
        Some(v) => v,
        None => return false,
    };

    let finish_after = match get_current_escrow_finish_after() {
        Some(v) => v,
        None => return false,
    };

    let balance = match get_account_balance(&account_id_tx) {
        Some(v) => v,
        None => return false,
    };

    account_id_clo == account_id_tx && destination == account_id_tx && finish_after == 0 && balance > 0
}
