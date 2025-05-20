#![no_std]
#![allow(unused_imports)]
use xrpl_std::locator::LocatorPacker;
use xrpl_std::sfield::{SignerEntries, SignerEntry, SignerWeight};
use xrpl_std::{
    get_account_balance, get_current_escrow_account_id, get_current_escrow_destination,
    get_current_escrow_finish_after, get_tx_account_id, host_lib,
};

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
        // let keylet = [
        //     52, 47, 158, 13, 36, 46, 219, 67, 160, 251, 252, 103, 43, 48, 44, 200, 187, 144, 73,
        //     147, 23, 46, 87, 251, 255, 76, 93, 74, 30, 184, 90, 185,
        // ];
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
    {
        // let nft_id = [
        //     0, 8, 39, 16, 104, 7, 191, 132, 143, 172, 217, 114, 242, 246, 23, 226, 112, 3, 215, 91,
        //     44, 170, 201, 129, 108, 238, 20, 132, 5, 33, 209, 233,
        // ];
        // let owner = get_tx_account_id().unwrap();
        // if owner.len() != 20 {
        //     return -21;
        // }
        // let mut arr = [0u8; 256];
        // let res = unsafe {
        //     host_lib::get_NFT(
        //         owner.as_ptr(),
        //         owner.len(),
        //         nft_id.as_ptr(),
        //         nft_id.len(),
        //         arr.as_mut_ptr(),
        //         arr.len(),
        //     )
        // };
        // 
        // if res != 106 {
        //     return -22;
        // }
    }

    1
}
