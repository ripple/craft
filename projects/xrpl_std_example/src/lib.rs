#![no_std]
use xrpl_std::{get_current_escrow_account_id, get_current_escrow_destination, get_tx_account_id,
               get_current_escrow_finish_after, get_account_balance};

#[no_mangle]
pub extern "C" fn finish() -> i32 {
    let account_id_tx = match get_tx_account_id() {
        Some(v) => v,
        None => return -1,
    };
    
    let account_id_clo = match get_current_escrow_account_id() {
        Some(v) => v,
        None => return -2,
    };
    
    let destination = match get_current_escrow_destination() {
        Some(v) => v,
        None => return -3,
    };
    
    let finish_after = match get_current_escrow_finish_after() {
        Some(v) => v,
        None => return -4,
    };
    
    let balance = match get_account_balance(&account_id_tx) {
        Some(v) => v,
        None => return -5,
    };
    
    let mut ledger_sqn = 0i32;
    if unsafe {
        xrpl_std::host_lib::get_ledger_sqn((&mut ledger_sqn) as *mut i32 as *mut u8, 4)
    } <= 0 {
        return -10;
    }     
    
    if account_id_clo != account_id_tx {
        return -6;
    } 
    if destination == account_id_tx {
        return -7;
    }
    if finish_after == 0 {
        return -8;
    }
    if balance <= 0 {
        return -9;
    }
    
    1
}
