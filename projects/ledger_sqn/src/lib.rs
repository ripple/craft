#[no_mangle]
pub fn finish() -> bool {
    unsafe { host_lib::getLedgerSqn() >= 5 }
}

pub mod host_lib {
    #[link(wasm_import_module = "host_lib")]
    unsafe extern "C" {
        pub fn getLedgerSqn() -> i32;
    }
}
