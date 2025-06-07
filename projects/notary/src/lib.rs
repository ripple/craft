#![no_std]
use xrpl_std::core::tx::current_transaction;
use xrpl_std::core::constants::ACCOUNT_ONE;

// For testing, use ACCOUNT_ONE as the notary
// In production, this would be the actual notary account bytes

#[unsafe(no_mangle)]
pub fn finish() -> bool {
    let tx_account = current_transaction::get_account();
    
    // Compare AccountID directly (both are 20-byte arrays)
    tx_account == ACCOUNT_ONE
}
