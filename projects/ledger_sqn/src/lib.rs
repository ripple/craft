#[no_mangle]
pub fn ready() -> bool {
    unsafe { host::get_ledger_sqn() >= 5}
}

pub mod host {
    #[link(wasm_import_module = "host")]
    extern "C" {
        pub fn get_ledger_sqn() -> i32;
    }
}