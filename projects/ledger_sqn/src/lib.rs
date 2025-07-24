#![no_std]
use xrpl_std::host::trace::trace_num;

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> bool {
    unsafe {
        let ledger_sequence = host_lib::getLedgerSqn();
        let _ = trace_num("Ledger Sequence", ledger_sequence as i64);
        ledger_sequence >= 5
    }
}

pub mod host_lib {
    #[link(wasm_import_module = "host_lib")]
    unsafe extern "C" {
        pub fn getLedgerSqn() -> i32;
    }
}
