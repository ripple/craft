// TODO: Define functions for accessing data from any account on the ledger.
// TODO: Adapt this to support STAmount for all token types.

use crate::core::types::account_id::AccountID;
use crate::core::types::keylets::account_keylet;
use crate::{host, sfield};
use host::Error;

pub fn get_account_balance(account_id: &AccountID) -> host::Result<u64> {
    // Construct the account keylet. This calls a host function, so propagate the error via `?`
    let account_keylet = match account_keylet(account_id) {
        host::Result::Ok(keylet) => keylet,
        host::Result::Err(e) => return host::Result::Err(e),
    };

    // Try to cache the ledger object inside rippled
    let slot = unsafe { host::cache_ledger_obj(account_keylet.as_ptr(), account_keylet.len(), 0) };
    if slot <= 0 {
        return host::Result::Err(Error::NoFreeSlots);
    }

    // Get the balance.
    let mut balance = 0u64;
    let result_code = unsafe {
        host::get_ledger_obj_field(
            slot,
            sfield::Balance,
            (&mut balance) as *mut u64 as *mut u8,
            8,
        )
    };

    match result_code {
        8 => host::Result::Ok(balance), // <-- 8 bytes were written
        code if code < 0 => host::Result::Err(Error::from_code(code)),
        _ => host::Result::Err(Error::InternalError), // <-- Used for an unexpected result.
    }
}
