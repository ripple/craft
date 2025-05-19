#![no_std]
#![allow(unused_imports)]
use xrpl_std::{get_account_balance, get_current_escrow_account_id, get_current_escrow_destination, get_current_escrow_finish_after, get_tx_account_id, host_lib};
use xrpl_std::locator::LocatorPacker;
use xrpl_std::sfield::{SignerEntries, SignerEntry, SignerWeight};

#[no_mangle]
pub extern "C" fn finish() -> i32 {
    {
        // let account_id_tx = match get_tx_account_id() {
        //     Some(v) => v,
        //     None => return -1,
        // };
        // println!("wasm finish {:?}", account_id_tx);
        // 
        // let account_id_clo = match get_current_escrow_account_id() {
        //     Some(v) => v,
        //     None => return -2,
        // };
        // 
        // let destination = match get_current_escrow_destination() {
        //     Some(v) => v,
        //     None => return -3,
        // };
        // if account_id_clo != account_id_tx {
        //     return -6;
        // }
        // if destination == account_id_tx {
        //     return -7;
        // }
    }
    {
        // let finish_after = match get_current_escrow_finish_after() {
        //     Some(v) => v,
        //     None => return -4,
        // };
        // if finish_after == 0 {
        //     return -8;
        // }
    }
    {
        let account_id_tx = match get_tx_account_id() {
            Some(v) => v,
            None => return -1,
        };
        // println!("wasm finish account_id_tx {:?}", account_id_tx);
        let balance = match get_account_balance(&account_id_tx) {
            Some(v) => v,
            None => return -5,
        };
        // println!("wasm finish balance {:?}", balance);
        if balance <= 0 {
            return -9;
        }
    }
    {
        // let mut ledger_sqn = 0i32;
        // if unsafe { xrpl_std::host_lib::get_ledger_sqn((&mut ledger_sqn) as *mut i32 as *mut u8, 4) }
        //     <= 0
        // {
        //     return -10;
        // }
    }
    {
        // let s = "342F9E0D242EDB43A0FBFC672B302CC8BB904993172E57FBFF4C5D4A1EB85AB9";
        // let keylet = hex::decode(s).unwrap();
        // println!("wasm finish keylet {:?}", keylet);
        // 
        // let slot = unsafe { host_lib::ledger_slot_set(keylet.as_ptr(), keylet.len(), 0) };
        // 
        // println!("wasm finish slot {:?}", slot);
        // 
        // let mut locator = LocatorPacker::new();
        // locator.pack(SignerEntries);
        // let array_len = unsafe {
        //     host_lib::get_ledger_obj_nested_array_len(slot, locator.get_addr(), locator.num_packed_bytes())
        // };
        // println!("wasm finish array_len {:?}", array_len);
        // 
        // locator.pack(0);
        // locator.pack(SignerEntry);
        // locator.pack(SignerWeight);
        // 
        // let mut weight = 0i32;
        // let nfr = unsafe {
        //     host_lib::get_ledger_obj_nested_field(
        //         slot, locator.get_addr(), locator.num_packed_bytes(),
        //         (&mut weight) as *mut i32 as *mut u8, 4
        //     )
        // };
        // 
        // println!("wasm finish get_ledger_obj_nested_field {:?} {}", nfr, weight);
    }
    
    1
}
