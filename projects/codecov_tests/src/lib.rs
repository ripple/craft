#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use core::panic;
use xrpl_std::core::error_codes;
use xrpl_std::host;
use xrpl_std::host::trace::{trace, trace_num};
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
            unsafe { host_bindings_loose::get_ledger_sqn(ptr as i32, 1_000_000) },
            error_codes::POINTER_OUT_OF_BOUNDS,
            "get_ledger_sqn_len_too_long",
        )
    });

    true // <-- If we get here, finish the escrow.
}
