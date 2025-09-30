/// This file exists as a host_binding stand-in/override to allow calls that Rust wouldn't ordinarily allow (for testing
/// purposes) because this contract is actually executed against `rippled` in that project's build (see here:
/// https://github.com/XRPLF/rippled/blob/ripple/smart-escrow/src/test/app/TestHostFunctions.h), which is a C++ project
/// and therefore allows different pointer types from what Rust would allow.
#[allow(unused_variables)]
// #[link(wasm_import_module = "host_lib")]
pub fn get_parent_ledger_hash(out_buff_ptr: i32, out_buff_len: i32) -> i32 {
    32
}

#[allow(unused_variables)]
// #[link(wasm_import_module = "host_lib")]
pub fn cache_ledger_obj(keylet_ptr: i32, keylet_len: i32, cache_num: i32) -> i32 {
    32
}

#[allow(unused_variables)]
// #[link(wasm_import_module = "host_lib")]
pub fn get_tx_nested_array_len(locator_ptr: i32, locator_len: i32) -> i32 {
    32
}

#[allow(unused_variables)]
// #[link(wasm_import_module = "host_lib")]
pub fn account_keylet(
    account_ptr: i32,
    account_len: i32,
    out_buff_ptr: *mut u8,
    out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused_variables)]
// #[link(wasm_import_module = "host_lib")]
pub fn amm_keylet(
    issue1_ptr: *const u8,
    issue1_len: usize,
    issue2_ptr: i32,
    issue2_len: i32,
    out_buff_ptr: *mut u8,
    out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
// #[link(wasm_import_module = "host_lib")]
pub fn line_keylet(
    account1_ptr: *const u8,
    account1_len: usize,
    account2_ptr: *const u8,
    account2_len: usize,
    currency_ptr: i32,
    currency_len: i32,
    out_buff_ptr: *mut u8,
    out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused_variables)]
// #[link(wasm_import_module = "host_lib")]
pub fn trace_num(msg_read_ptr: i32, msg_read_len: i32, number: i64) -> i32 {
    0
}
