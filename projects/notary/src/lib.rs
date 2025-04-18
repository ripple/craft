use std::str;
use xrpl_std_lib::get_tx_account_id;

// Notary account that is authorized to finish the escrow
const NOTARY_ACCOUNT: &str = "rPPLRQwB3KGvpfDMABZucA8ifJJcvQhHD3"; // Account 2 (example)

#[no_mangle]
pub fn ready() -> bool {
    unsafe {
        let tx_account = get_tx_account_id();

        // Convert account bytes to string for comparison
        let tx_account_str = match str::from_utf8(&tx_account) {
            Ok(s) => s,
            Err(_) => return false
        };

        tx_account_str == NOTARY_ACCOUNT
    }
}

// pub fn finish(escrow_ptr: *mut usize, finish_tx_ptr: *mut usize) -> bool {
//
//     // xrpl_std_lib::core::()
//
//
//     // return false;
// }

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