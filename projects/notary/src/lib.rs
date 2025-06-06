#![no_std]
use xrpl_std::core::tx::current_transaction;

// Notary account that is authorized to finish the escrow
const NOTARY_ACCOUNT: &str = "rPPLRQwB3KGvpfDMABZucA8ifJJcvQhHD3"; // Account 2 (example)

#[unsafe(no_mangle)]
pub fn finish() -> bool {
    let tx_account = current_transaction::get_account();
    
    // Convert AccountID to string for comparison
    let tx_account_str = tx_account.to_string();
    
    tx_account_str == NOTARY_ACCOUNT
}
