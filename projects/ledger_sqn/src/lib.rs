#[no_mangle]
pub fn ready() -> bool {
    unsafe { host_lib::get_ledger_sqn() >= 5}
}

pub mod host_lib {
    #[link(wasm_import_module = "host_lib")]
    extern "C" {
        pub fn get_ledger_sqn() -> i32;
    }
}