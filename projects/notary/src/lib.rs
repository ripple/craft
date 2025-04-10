use std::str;
use xrpl_std::get_tx_account_id;

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