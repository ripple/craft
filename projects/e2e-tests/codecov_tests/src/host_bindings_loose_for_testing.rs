// This file exists as a host_binding stand-in for non-WASM targets. For example, this file will
// be used during unit tests.

//TODO discuss reason for this file
#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub fn get_ledger_sqn(_out_buff_ptr: i32, _out_buff_len: i32) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub fn cache_ledger_obj(_keylet_ptr: i32, _keylet_len: i32, _cache_num: i32) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub fn get_tx_nested_array_len(locator_ptr: i32, locator_len: i32) -> i32 {
    32
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub fn account_keylet(
    _account_ptr: i32,
    _account_len: i32,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
#[allow(clippy::too_many_arguments)]
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

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub fn trace_num(msg_read_ptr: i32, msg_read_len: i32, number: i64) -> i32 {
    32
}
