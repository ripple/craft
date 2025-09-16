// This file exists as a host_binding stand-in for non-WASM targets. For example, this file will
// be used during unit tests.

//TODO add docs after discussing the interface
//Note that Craft currently does not honor the rounding modes
//TODO discuss reason for this file
#[allow(unused)]
pub const FLOAT_ROUNDING_MODES_TO_NEAREST: i32 = 0;
#[allow(unused)]
pub const FLOAT_ROUNDING_MODES_TOWARDS_ZERO: i32 = 1;
#[allow(unused)]
pub const FLOAT_ROUNDING_MODES_DOWNWARD: i32 = 2;
#[allow(unused)]
pub const FLOAT_ROUNDING_MODES_UPWARD: i32 = 3;

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_ledger_sqn(_out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_parent_ledger_time(_out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_parent_ledger_hash(_out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_ledger_account_hash(_out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_ledger_tx_hash(_out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_base_fee() -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn amendment_enabled(_amendment_ptr: *const u8, _amendment_len: usize) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn cache_ledger_obj(_keylet_ptr: *const u8, _keylet_len: usize, _cache_num: i32) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_tx_field(_field: i32, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_current_ledger_obj_field(
    _field: i32,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_ledger_obj_field(
    _cache_num: i32,
    _field: i32,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_tx_nested_field(
    _locator_ptr: *const u8,
    _locator_len: usize,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_current_ledger_obj_nested_field(
    _locator_ptr: *const u8,
    _locator_len: usize,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
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
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_tx_array_len(_field: i32) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_current_ledger_obj_array_len(_field: i32) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_ledger_obj_array_len(_cache_num: i32, _field: i32) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_tx_nested_array_len(_locator_ptr: *const u8, _locator_len: usize) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_current_ledger_obj_nested_array_len(
    _locator_ptr: *const u8,
    _locator_len: usize,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_ledger_obj_nested_array_len(
    _cache_num: i32,
    _locator_ptr: *const u8,
    _locator_len: usize,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn update_data(_data_ptr: *const u8, _data_len: usize) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn compute_sha512_half(
    _data_ptr: *const u8,
    _data_len: usize,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn check_sig(
    _message_ptr: *const u8,
    _message_len: usize,
    _signature_ptr: *const u8,
    _signature_len: usize,
    _pubkey_ptr: *const u8,
    _pubkey_len: usize,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn account_keylet(
    _account_ptr: *const u8,
    _account_len: usize,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn check_keylet(
    _account_ptr: *const u8,
    _account_len: usize,
    _sequence: i32,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::missing_safety_doc)]
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
#[allow(clippy::missing_safety_doc)]
pub unsafe fn delegate_keylet(
    _account_ptr: *const u8,
    _account_len: usize,
    _authorize_ptr: *const u8,
    _authorize_len: usize,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn deposit_preauth_keylet(
    _account_ptr: *const u8,
    _account_len: usize,
    _authorize_ptr: *const u8,
    _authorize_len: usize,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn did_keylet(
    _account_ptr: *const u8,
    _account_len: usize,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
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
#[allow(clippy::too_many_arguments)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn line_keylet(
    _account1_ptr: *const u8,
    _account1_len: usize,
    _account2_ptr: *const u8,
    _account2_len: usize,
    _currency_ptr: *const u8,
    _currency_len: usize,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn nft_offer_keylet(
    _account_ptr: *const u8,
    _account_len: usize,
    _sequence: i32,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn offer_keylet(
    _account_ptr: *const u8,
    _account_len: usize,
    _sequence: i32,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn oracle_keylet(
    _account_ptr: *const u8,
    _account_len: usize,
    _document_id: i32,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn paychan_keylet(
    _account_ptr: *const u8,
    _account_len: usize,
    _destination_ptr: *const u8,
    _destination_len: usize,
    _sequence: i32,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn signers_keylet(
    _account_ptr: *const u8,
    _account_len: usize,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn ticket_keylet(
    _account_ptr: *const u8,
    _account_len: usize,
    _sequence: i32,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
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
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_nft_issuer(
    _nft_id_ptr: *const u8,
    _nft_id_len: usize,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_nft_taxon(
    _nft_id_ptr: *const u8,
    _nft_id_len: usize,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_nft_flags(_nft_id_ptr: *const u8, _nft_id_len: usize) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_nft_transfer_fee(_nft_id_ptr: *const u8, _nft_id_len: usize) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_nft_serial(
    _nft_id_ptr: *const u8,
    _nft_id_len: usize,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn trace(
    _msg_read_ptr: *const u8,
    _msg_read_len: usize,
    _data_read_ptr: *const u8,
    _data_read_len: usize,
    _as_hex: i32,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn trace_num(_msg_read_ptr: *const u8, _msg_read_len: usize, _number: i64) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn float_from_int(
    in_int: i64,
    out_buff: *mut u8,
    out_buff_len: usize,
    rounding_mode: i32,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn float_from_uint(
    in_uint_ptr: *const u8,
    in_uint_len: usize,
    out_buff: *mut u8,
    out_buff_len: usize,
    rounding_mode: i32,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn float_set(
    exponent: i32,
    mantissa: i64,
    out_buff: *mut u8,
    out_buff_len: usize,
    rounding_mode: i32,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn float_compare(
    in_buff1: *const u8,
    in_buff1_len: usize,
    in_buff2: *const u8,
    in_buff2_len: usize,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn float_add(
    in_buff1: *const u8,
    in_buff1_len: usize,
    in_buff2: *const u8,
    in_buff2_len: usize,
    out_buff: *mut u8,
    out_buff_len: usize,
    rounding_mode: i32,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn float_subtract(
    in_buff1: *const u8,
    in_buff1_len: usize,
    in_buff2: *const u8,
    in_buff2_len: usize,
    out_buff: *mut u8,
    out_buff_len: usize,
    rounding_mode: i32,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn float_multiply(
    in_buff1: *const u8,
    in_buff1_len: usize,
    in_buff2: *const u8,
    in_buff2_len: usize,
    out_buff: *mut u8,
    out_buff_len: usize,
    rounding_mode: i32,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn float_divide(
    in_buff1: *const u8,
    in_buff1_len: usize,
    in_buff2: *const u8,
    in_buff2_len: usize,
    out_buff: *mut u8,
    out_buff_len: usize,
    rounding_mode: i32,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn float_pow(
    in_buff: *const u8,
    in_buff_len: usize,
    in_int: i32,
    out_buff: *mut u8,
    out_buff_len: usize,
    rounding_mode: i32,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn float_root(
    in_buff: *const u8,
    in_buff_len: usize,
    in_int: i32,
    out_buff: *mut u8,
    out_buff_len: usize,
    rounding_mode: i32,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn float_log(
    in_buff: *const u8,
    in_buff_len: usize,
    out_buff: *mut u8,
    out_buff_len: usize,
    rounding_mode: i32,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn trace_opaque_float(
    msg_read_ptr: *const u8,
    msg_read_len: usize,
    opaque_float_ptr: *const u8,
    opaque_float_len: usize,
) -> i32 {
    -1
}
