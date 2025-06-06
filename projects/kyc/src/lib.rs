#![no_std]

use xrpl_std::get_current_escrow_account_id;
use xrpl_std::keylet::credential_keylet;

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> bool {
    let account = match get_current_escrow_account_id() {
        Some(v) => v,
        None => return false,
    };
    let cred_type: &[u8] = b"termsandconditions";
    match credential_keylet(&account, &account, &cred_type) {
        Some(_v) => return true,
        None => return false,
    };
}
