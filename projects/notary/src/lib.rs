use serde_json::Value;
use std::mem;
use xrpl_std_lib::host;
use xrpl_std_lib::host::log;
// Notary account that is authorized to finish the escrow
const NOTARY_ACCOUNT: &str = "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh";

// #[no_mangle]
// pub fn allocate(size: usize) -> *mut u8 {
//     log("allocate called");
//     log(&format!("allocating {} bytes", size));
//     let mut buffer = Vec::with_capacity(size);
//     let pointer = buffer.as_mut_ptr();
//     log(&format!("allocation address: {:p}", pointer));
//     mem::forget(buffer);
//     pointer
// }

#[no_mangle]
pub fn finish(escrow_ptr: *mut usize, finish_tx_ptr: *mut usize) -> bool {

    // xrpl_std_lib::core::()


    // return false;
}

// #[no_mangle]
// pub fn finish2(tx_json_ptr: *mut u8, tx_json_size: usize, lo_json_ptr: *mut u8, lo_json_size: usize) -> bool {
//     log("finish function called");
//     log(&format!(
//         "tx_json_ptr: {:p}, tx_json_size: {}",
//         tx_json_ptr, tx_json_size
//     ));
//
//     let tx_data;
//     unsafe {
//         tx_data = Vec::from_raw_parts(tx_json_ptr, tx_json_size, tx_json_size);
//     }
//
//     log("Parsing transaction JSON");
//     let tx_json_value: Value = match serde_json::from_slice(tx_data.as_slice()) {
//         Ok(v) => v,
//         Err(e) => {
//             log(&format!("Error parsing transaction JSON: {:?}", e));
//             return false;
//         }
//     };
//
//     log("Extracting Account field");
//     let tx_account = match tx_json_value.get("Account") {
//         Some(v) => {
//             let account_str = v.as_str().unwrap_or("");
//             log(&format!("Transaction Account: {}", account_str));
//             account_str
//         }
//         None => {
//             log("Transaction JSON has no Account field");
//             return false;
//         }
//     };
//
//     // Check if the transaction account matches the notary account
//     let result = tx_account == NOTARY_ACCOUNT;
//     debug(&format!("Notary check result: {}", result));
//
//     result
// }