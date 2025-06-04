#![no_std]
use xrpl_std::get_tx_account_id;

// Notary account that is authorized to finish the escrow
const NOTARY_ACCOUNT: &[u8] = b"rPPLRQwB3KGvpfDMABZucA8ifJJcvQhHD3"; // Account 2 (example)

#[unsafe(no_mangle)]
pub fn finish() -> bool {
    let tx_account = match get_tx_account_id() {
        Some(v) => v,
        None => return false,
    };

    tx_account == NOTARY_ACCOUNT
}
