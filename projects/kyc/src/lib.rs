#![no_std]

use xrpl_std::get_current_escrow_destination;
use xrpl_std::host::trace::{DataRepr, trace_data, trace_num};
use xrpl_std::keylet::credential_keylet;

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> bool {
    let account = match get_current_escrow_destination() {
        Some(v) => v,
        None => return false,
    };

    // "termsandconditions" in hex
    let cred_type: &[u8] = b"termsandconditions";
    let cred_keylet = match credential_keylet(&account, &account, &cred_type) {
        Some(v) => v,
        None => return false,
    };
    let _ = trace_data("cred_keylet", &cred_keylet, DataRepr::AsHex);

    let slot = xrpl_std::host::cache_ledger_obj(cred_keylet.as_ptr(), cred_keylet.len(), 0);
    if slot < 0 {
        let _ = trace_num("CACHE ERROR", i64::from(slot));
        return false;
    };
    return true;
}
