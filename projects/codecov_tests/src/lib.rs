#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use core::panic;
use xrpl_std::core::error_codes;
use xrpl_std::host;
use xrpl_std::host::trace::trace;
mod host_bindings_loose;
include!("host_bindings_loose.rs");

fn check_error(result: i32, expected: i32) -> () {
    match result {
        code if code == expected => return,
        code if code >= 0 => panic!("Unexpected success code: {}", code),
        code => panic!("Error code: {}", code),
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
        check_error(unsafe { host::get_ledger_sqn(ptr, len) }, 4)
    });
    let _ = with_buffer::<4, _, _>(|ptr, len| {
        check_error(unsafe { host::get_parent_ledger_time(ptr, len) }, 4)
    });
    let _ = with_buffer::<32, _, _>(|ptr, len| {
        check_error(unsafe { host::get_parent_ledger_hash(ptr, len) }, 32)
    });

    // ########################################
    // Step #2: Test all helper function edge cases
    // ########################################
    // check_error(
    //     unsafe { host::get_ledger_sqn(-1 as *mut u8, 4) },
    //     error_codes::INVALID_PARAMS,
    // );
    let _ = with_buffer::<3, _, _>(|ptr, _len| {
        check_error(
            unsafe { host_bindings_loose::get_ledger_sqn(ptr as i32, -1) },
            error_codes::INVALID_PARAMS,
        )
    });
    let _ = with_buffer::<3, _, _>(|ptr, len| {
        check_error(
            unsafe { host_bindings_loose::get_ledger_sqn(ptr as i32, len as i32) },
            error_codes::BUFFER_TOO_SMALL,
        )
    });

    false // <-- If we get here, don't finish the escrow.
}
