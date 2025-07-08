/// This file exists as a host_binding stand-in for non WASM targets. For example, this file will
/// be used during unit tests.

#[allow(unused)]
pub unsafe fn get_ledger_sqn(_out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32 {
    -1
}

#[allow(unused)]
pub unsafe fn get_parent_ledger_time(_out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32 {
    -1
}

#[allow(unused)]
pub unsafe fn get_parent_ledger_hash(__out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32 {
    -1
}
#[allow(unused)]
pub unsafe fn cache_ledger_obj(
    _keylet_ptr: *const u8,
    _keylet_len: usize,
    __cache_num: i32,
) -> i32 {
    -1
}

#[allow(unused)]
pub unsafe fn get_tx_field(_field: i32, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32 {
    -1
}
#[allow(unused)]
pub unsafe fn get_tx_field2(
    _field: i32,
    _field2: i32,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    -1
}
#[allow(unused)]
pub unsafe fn get_tx_field3(
    _field: i32,
    _field2: i32,
    _field3: i32,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    -1
}
#[allow(unused)]
pub unsafe fn get_tx_field4(
    _field: i32,
    _field2: i32,
    _field3: i32,
    _field4: i32,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    -1
}
#[allow(unused)]
pub unsafe fn get_tx_field5(
    _field: i32,
    _field2: i32,
    _field3: i32,
    _field4: i32,
    _field5: i32,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    -1
}
#[allow(unused)]
pub unsafe fn get_tx_field6(
    _field: i32,
    _field2: i32,
    _field3: i32,
    _field4: i32,
    _field5: i32,
    _field6: i32,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    -1
}

#[allow(unused)]
pub unsafe fn get_current_ledger_obj_field(
    _field: i32,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    -1
}

#[allow(unused)]
pub unsafe fn get_ledger_obj_field(
    _cache_num: i32,
    _field: i32,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    -1
}

#[allow(unused)]
pub unsafe fn get_tx_nested_field(
    _locator_ptr: *const u8,
    _locator_len: usize,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    -1
}

#[allow(unused)]
pub unsafe fn get_current_ledger_obj_nested_field(
    _locator_ptr: *const u8,
    _locator_len: usize,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    -1
}

#[allow(unused)]
pub unsafe fn get_ledger_obj_nested_field(
    _cache_num: i32,
    _locator_ptr: *const u8,
    _locator_len: usize,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    -1
}

#[allow(unused)]
pub unsafe fn get_tx_array_len(_field: i32) -> i32 {
    -1
}

#[allow(unused)]
pub unsafe fn get_current_ledger_obj_array_len(_field: i32) -> i32 {
    -1
}

#[allow(unused)]
pub unsafe fn get_ledger_obj_array_len(_cache_num: i32, _field: i32) -> i32 {
    -1
}

#[allow(unused)]
pub unsafe fn get_tx_nested_array_len(_locator_ptr: *const u8, _locator_len: usize) -> i32 {
    -1
}

#[allow(unused)]
pub unsafe fn get_current_ledger_obj_nested_array_len(
    _locator_ptr: *const u8,
    _locator_len: usize,
) -> i32 {
    -1
}

#[allow(unused)]
pub unsafe fn get_ledger_obj_nested_array_len(
    _cache_num: i32,
    _locator_ptr: *const u8,
    _locator_len: usize,
) -> i32 {
    -1
}

#[allow(unused)]
pub unsafe fn update_data(_data_ptr: *const u8, _data_len: usize) -> i32 {
    -1
}

#[allow(unused)]
pub unsafe fn compute_sha512_half(
    _data_ptr: *const u8,
    _data_len: usize,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused)]
pub unsafe fn account_keylet(
    _account_ptr: *const u8,
    _account_len: usize,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused)]
pub unsafe fn credential_keylet(
    _subject_ptr: *const u8,
    _subject_len: usize,
    _issuer_ptr: *const u8,
    _issuer_len: usize,
    _cred_type_ptr: *const u8,
    _cred_type_len: usize,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused)]
pub unsafe fn escrow_keylet(
    _account_ptr: *const u8,
    _account_len: usize,
    _sequence: i32,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused)]
pub unsafe fn oracle_keylet(
    _account_ptr: *const u8,
    _account_len: usize,
    _document_id: i32,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    32
}

// TODO: This should be called `get_nft`
#[allow(unused)]
pub unsafe fn get_nft(
    _account_ptr: *const u8,
    _account_len: usize,
    _nft_id_ptr: *const u8,
    _nft_id_len: usize,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    -1
}

#[allow(unused)]
pub unsafe fn trace(
    _msg_read_ptr: u32,
    _msg_read_len: usize,
    _data_read_ptr: u32,
    _data_read_len: usize,
    _as_hex: u32,
) -> i32 {
    -1
}

#[allow(unused)]
pub unsafe fn trace_num(_msg_read_ptr: u32, _msg_read_len: usize, _number: i64) -> i32 {
    -1
}

#[allow(unused)]
pub unsafe fn trace_opaque_float(
    _msg_read_ptr: u32,
    _msg_read_len: usize,
    _opaque_float_ptr: u32,
) -> i32 {
    -1
}
