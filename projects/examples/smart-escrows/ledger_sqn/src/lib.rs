#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_std::core::error_codes::match_result_code;
use xrpl_std::host;
use xrpl_std::host::trace::trace_num;

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    unsafe {
        let mut buffer = [0u8; 4]; // Enough to hold an u32

        let result_code = host::get_ledger_sqn(buffer.as_mut_ptr(), 8);

        let ledger_sequence = match_result_code(result_code, || {
            Some(u32::from_le_bytes(buffer)) // <-- Move the value into a buffer
        })
        .unwrap()
        .unwrap();

        let _ = trace_num("Ledger Sequence", ledger_sequence as i64);
        (ledger_sequence >= 5) as i32 // Return 1 if true (successful outcome), 0 if false (failed outcome)
    }
}
