#![no_std]

use xrpl_std::host::trace::{trace_data, DataRepr};
use xrpl_std::{credential_keylet, get_current_escrow_account_id};

#[no_mangle]
pub extern "C" fn finish() -> bool {
    unsafe {
        let account = match get_current_escrow_account_id() {
            Some(v) => v,
            None => return false,
        };
        let credential_keylet = match credential_keylet(&account, &account, &account) {
            Some(v) => v,
            None => return false,
        };

        trace_data("Credential Keylet", &credential_keylet, DataRepr::AsHex);

        true
    }
}
