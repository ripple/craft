#![no_std]
#![allow(unused_imports)]

// Use tracing API from xrpl_std
use xrpl_std::host::trace::{trace_data, trace_num, DataRepr};

// locator module doesn't exist in public API yet
//use xrpl_std::locator::LocatorPacker;

// Keeping sfield imports for future development
use xrpl_std::sfield::{SignerEntries, SignerEntry, SignerWeight};

// function paths: get_account_balance is available at root,
// but transaction functions are in core::tx::current_transaction module
use xrpl_std::get_account_balance;
use xrpl_std::core::tx::current_transaction::get_account;

// use unsafe attribute syntax required by Rust 2024 edition
#[unsafe(no_mangle)]

pub extern "C" fn finish(_reserved: u32) -> i32 {

        // get_account() returns AccountID directly
        let account_id_tx = get_account();

        // Access the inner bytes via .0 field of AccountID struct
        let _ = trace_data("  Account:", &account_id_tx.0, DataRepr::AsHex);

        // get_account_balance expects &AccountID, which we now have
        let balance = match get_account_balance(&account_id_tx) {
            Some(v) => v,
            None => return -5,
        };

        let _ = trace_num("  Balance:", balance as i64);

        if balance <= 0 {
            return -9;
        }

        1
}