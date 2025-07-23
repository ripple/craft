#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use core::panic;
use xrpl_std::core::current_tx::escrow_finish::{EscrowFinish, get_current_escrow_finish};
use xrpl_std::core::current_tx::traits::TransactionCommonFields;
use xrpl_std::core::error_codes;
use xrpl_std::core::locator::Locator;
use xrpl_std::core::types::keylets;
use xrpl_std::host;
use xrpl_std::host::trace::{trace, trace_num};
use xrpl_std::sfield;

mod host_bindings_loose;
include!("host_bindings_loose.rs");

fn check_error(result: i32, expected: i32, test_name: &'static str) -> () {
    match result {
        code if code == expected => {
            let _ = trace_num(test_name, code.into());
            return;
        }
        code if code >= 0 => {
            let _ = trace_num("TEST FAILED", code.into());
            panic!("Unexpected success code: {}", code);
        }
        code => {
            let _ = trace_num("TEST FAILED", code.into());
            panic!("Error code: {}", code);
        }
    }
}

fn with_buffer<const N: usize, F, R>(mut f: F) -> R
where
    F: FnMut(*mut u8, usize) -> R,
{
    let mut buf = [0u8; N];
    f(buf.as_mut_ptr(), buf.len())
}

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> bool {
    let _ = trace("$$$$$ STARTING WASM EXECUTION $$$$$");

    // ########################################
    // Step #1: Test all host function happy paths
    // Note: not testing all the keylet functions,
    // that's in a separate test file.
    // ########################################
    let _ = with_buffer::<4, _, _>(|ptr, len| {
        check_error(
            unsafe { host::get_ledger_sqn(ptr, len) },
            4,
            "get_ledger_sqn",
        )
    });
    let _ = with_buffer::<4, _, _>(|ptr, len| {
        check_error(
            unsafe { host::get_parent_ledger_time(ptr, len) },
            4,
            "get_parent_ledger_time",
        );
    });
    let _ = with_buffer::<32, _, _>(|ptr, len| {
        check_error(
            unsafe { host::get_parent_ledger_hash(ptr, len) },
            32,
            "get_parent_ledger_hash",
        );
    });
    let tx: EscrowFinish = get_current_escrow_finish();
    let account = tx.get_account().unwrap_or_panic(); // get_tx_field under the hood
    let keylet = keylets::account_keylet(&account).unwrap_or_panic(); // account_keylet under the hood
    check_error(
        unsafe { host::cache_ledger_obj(keylet.as_ptr(), keylet.len(), 0) },
        1,
        "cache_ledger_obj",
    );
    let _ = with_buffer::<20, _, _>(|ptr, len| {
        check_error(
            unsafe { host::get_current_ledger_obj_field(sfield::Account, ptr, len) },
            20,
            "get_current_ledger_obj_field",
        );
    });
    let _ = with_buffer::<20, _, _>(|ptr, len| {
        check_error(
            unsafe { host::get_ledger_obj_field(1, sfield::Account, ptr, len) },
            20,
            "get_ledger_obj_field",
        );
    });
    let mut locator = Locator::new();
    locator.pack(sfield::Account);
    let _ = with_buffer::<20, _, _>(|ptr, len| {
        check_error(
            unsafe { host::get_tx_nested_field(locator.as_ptr(), locator.len(), ptr, len) },
            20,
            "get_tx_nested_field",
        );
    });
    let _ = with_buffer::<20, _, _>(|ptr, len| {
        check_error(
            unsafe {
                host::get_current_ledger_obj_nested_field(locator.as_ptr(), locator.len(), ptr, len)
            },
            20,
            "get_current_ledger_obj_nested_field",
        );
    });
    let _ = with_buffer::<20, _, _>(|ptr, len| {
        check_error(
            unsafe {
                host::get_ledger_obj_nested_field(1, locator.as_ptr(), locator.len(), ptr, len)
            },
            20,
            "get_ledger_obj_nested_field",
        );
    });
    check_error(
        unsafe { host::get_tx_array_len(sfield::Memos) },
        32,
        "get_tx_array_len",
    );
    check_error(
        unsafe { host::get_current_ledger_obj_array_len(sfield::Memos) },
        32,
        "get_current_ledger_obj_array_len",
    );
    check_error(
        unsafe { host::get_ledger_obj_array_len(1, sfield::Memos) },
        32,
        "get_ledger_obj_array_len",
    );
    check_error(
        unsafe { host::get_nested_tx_array_len(locator.as_ptr(), locator.len()) },
        32,
        "get_nested_tx_array_len",
    );
    check_error(
        unsafe { host::get_nested_current_ledger_obj_array_len(locator.as_ptr(), locator.len()) },
        32,
        "get_nested_current_ledger_obj_array_len",
    );
    check_error(
        unsafe { host::get_nested_ledger_obj_array_len(1, locator.as_ptr(), locator.len()) },
        32,
        "get_nested_ledger_obj_array_len",
    );

    // ########################################
    // Step #2: Test set_data edge cases
    // ########################################
    check_error(
        unsafe { host_bindings_loose::get_ledger_sqn(-1 as i32, 4) },
        error_codes::INVALID_PARAMS,
        "get_ledger_sqn_neg_ptr",
    );
    let _ = with_buffer::<4, _, _>(|ptr, _len| {
        check_error(
            unsafe { host_bindings_loose::get_ledger_sqn(ptr as i32, -1) },
            error_codes::INVALID_PARAMS,
            "get_ledger_sqn_neg_len",
        )
    });
    let _ = with_buffer::<3, _, _>(|ptr, len| {
        check_error(
            unsafe { host_bindings_loose::get_ledger_sqn(ptr as i32, len as i32) },
            error_codes::BUFFER_TOO_SMALL,
            "get_ledger_sqn_buf_too_small",
        )
    });
    let _ = with_buffer::<4, _, _>(|ptr, _len| {
        check_error(
            unsafe { host_bindings_loose::get_ledger_sqn(ptr as i32, 1_000_000_000) },
            error_codes::POINTER_OUT_OF_BOUNDS,
            "get_ledger_sqn_len_too_long",
        )
    });

    true // <-- If we get here, finish the escrow.
}
