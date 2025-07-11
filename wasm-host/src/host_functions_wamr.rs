#![allow(unused)]
use crate::data_provider::{
    DataProvider, HostError, XRPL_CONTRACT_DATA_SIZE, error_code_to_string, unpack_locator,
};
use crate::decoding::ACCOUNT_ID_LEN;

use crate::data_provider::{DataProvider, HostError, XRPL_CONTRACT_DATA_SIZE, unpack_locator};
use crate::decoding::{
    _deserialize_issued_currency_amount, _serialize_issued_currency_value, ACCOUNT_ID_LEN,
};
use crate::hashing::{HASH256_LEN, LedgerNameSpace, index_hash, sha512_half};
use crate::mock_data::{DataSource, Keylet};
use bigdecimal::num_bigint::{BigInt, ToBigInt};
use bigdecimal::num_traits::real::Real;
use bigdecimal::{BigDecimal, ToPrimitive};
use log::{debug, warn};
use wamr_rust_sdk::sys::{
    wasm_exec_env_t, wasm_runtime_get_function_attachment, wasm_runtime_get_module_inst,
    wasm_runtime_validate_native_addr,
};

const MAX_WASM_PARAM_LENGTH: usize = 1024;

pub fn get_dp(env: wasm_exec_env_t) -> &'static mut DataProvider {
    unsafe { &mut *(wasm_runtime_get_function_attachment(env) as *mut DataProvider) }
}

fn get_data(in_buf_ptr: *mut u8, in_buf_len: usize) -> Vec<u8> {
    let mut buffer = vec![0u8; in_buf_len];
    unsafe {
        std::ptr::copy_nonoverlapping(in_buf_ptr, buffer.as_mut_ptr(), in_buf_len);
    }
    buffer
}

fn get_keylet(in_buf_ptr: *mut u8, in_buf_len: usize) -> Keylet {
    get_data(in_buf_ptr, in_buf_len)
}

fn set_data(dp_res: i32, out_buf_ptr: *mut u8, data_to_write: Vec<u8>) {
    if dp_res > 0 {
        unsafe {
            std::ptr::copy_nonoverlapping(data_to_write.as_ptr(), out_buf_ptr, data_to_write.len());
        }
    }
}

pub fn get_ledger_sqn(env: wasm_exec_env_t, out_buf_ptr: *mut u8, out_buf_cap: usize) -> i32 {
    let data_provider = get_dp(env);
    let dp_res = data_provider.get_ledger_sqn(out_buf_cap);
    set_data(dp_res.0, out_buf_ptr, dp_res.1);
    dp_res.0
}

pub fn get_parent_ledger_time(
    env: wasm_exec_env_t,
    out_buf_ptr: *mut u8,
    out_buf_cap: usize,
) -> i32 {
    let data_provider = get_dp(env);
    let dp_res = data_provider.get_parent_ledger_time(out_buf_cap);
    set_data(dp_res.0, out_buf_ptr, dp_res.1);
    dp_res.0
}

pub fn get_parent_ledger_hash(
    env: wasm_exec_env_t,
    out_buf_ptr: *mut u8,
    out_buf_cap: usize,
) -> i32 {
    let data_provider = get_dp(env);
    let dp_res = data_provider.get_parent_ledger_hash(out_buf_cap);
    set_data(dp_res.0, out_buf_ptr, dp_res.1);
    dp_res.0
}

pub fn cache_ledger_obj(
    env: wasm_exec_env_t,
    in_buf_ptr: *mut u8,
    in_buf_cap: usize,
    cache_num: i32,
) -> i32 {
    let data_provider = get_dp(env);
    let keylet = get_keylet(in_buf_ptr, in_buf_cap);
    data_provider.slot_set(keylet, cache_num as usize)
}

pub fn get_tx_field(
    env: wasm_exec_env_t,
    field: i32,
    out_buf_ptr: *mut u8,
    out_buf_cap: usize,
) -> i32 {
    let data_provider = get_dp(env);
    let dp_res = data_provider.get_field_value(DataSource::Tx, vec![field], out_buf_cap);
    set_data(dp_res.0, out_buf_ptr, dp_res.1);
    dp_res.0
}

pub fn get_current_ledger_obj_field(
    env: wasm_exec_env_t,
    field: i32,
    out_buf_ptr: *mut u8,
    out_buf_cap: usize,
) -> i32 {
    let data_provider = get_dp(env);
    let dp_res =
        data_provider.get_field_value(DataSource::CurrentLedgerObj, vec![field], out_buf_cap);
    set_data(dp_res.0, out_buf_ptr, dp_res.1);
    dp_res.0
}

pub fn get_ledger_obj_field(
    env: wasm_exec_env_t,
    slot: i32,
    field: i32,
    out_buf_ptr: *mut u8,
    out_buf_cap: usize,
) -> i32 {
    let data_provider = get_dp(env);
    let keylet = match data_provider.slot_get(slot as usize) {
        None => return HostError::EmptySlot as i32,
        Some(key) => key.clone(),
    };
    let dp_res = data_provider.get_field_value(
        DataSource::KeyletLedgerObj(keylet),
        vec![field],
        out_buf_cap,
    );

    set_data(dp_res.0, out_buf_ptr, dp_res.1);
    dp_res.0
}

pub fn get_tx_nested_field(
    env: wasm_exec_env_t,
    in_buf_ptr: *mut u8,
    in_buf_len: usize,
    out_buf_ptr: *mut u8,
    out_buf_cap: usize,
) -> i32 {
    let data_provider = get_dp(env);
    let data = get_data(in_buf_ptr, in_buf_len);
    let idx_fields: Vec<i32> = match unpack_locator(data) {
        Ok(fields) => fields,
        Err(host_err) => return host_err as i32,
    };

    let dp_res = data_provider.get_field_value(DataSource::Tx, idx_fields, out_buf_cap);
    set_data(dp_res.0, out_buf_ptr, dp_res.1);
    dp_res.0
}

pub fn get_current_ledger_obj_nested_field(
    env: wasm_exec_env_t,
    in_buf_ptr: *mut u8,
    in_buf_len: usize,
    out_buf_ptr: *mut u8,
    out_buf_cap: usize,
) -> i32 {
    let data_provider = get_dp(env);
    let data = get_data(in_buf_ptr, in_buf_len);
    let idx_fields: Vec<i32> = match unpack_locator(data) {
        Ok(fields) => fields,
        Err(host_err) => return host_err as i32,
    };

    let dp_res =
        data_provider.get_field_value(DataSource::CurrentLedgerObj, idx_fields, out_buf_cap);
    set_data(dp_res.0, out_buf_ptr, dp_res.1);
    dp_res.0
}

pub fn get_ledger_obj_nested_field(
    env: wasm_exec_env_t,
    slot: i32,
    in_buf_ptr: *mut u8,
    in_buf_len: usize,
    out_buf_ptr: *mut u8,
    out_buf_cap: usize,
) -> i32 {
    let data_provider = get_dp(env);
    let keylet = match data_provider.slot_get(slot as usize) {
        None => return HostError::EmptySlot as i32,
        Some(key) => key.clone(),
    };

    let data = get_data(in_buf_ptr, in_buf_len);
    let idx_fields: Vec<i32> = match unpack_locator(data) {
        Ok(fields) => fields,
        Err(host_err) => return host_err as i32,
    };

    let dp_res =
        data_provider.get_field_value(DataSource::KeyletLedgerObj(keylet), idx_fields, out_buf_cap);
    set_data(dp_res.0, out_buf_ptr, dp_res.1);
    dp_res.0
}

pub fn get_tx_array_len(env: wasm_exec_env_t, field: i32) -> i32 {
    let data_provider = get_dp(env);
    data_provider.get_array_len(DataSource::Tx, vec![field])
}
pub fn get_current_ledger_obj_array_len(env: wasm_exec_env_t, field: i32) -> i32 {
    let data_provider = get_dp(env);
    data_provider.get_array_len(DataSource::CurrentLedgerObj, vec![field])
}
pub fn get_ledger_obj_array_len(env: wasm_exec_env_t, slot: i32, field: i32) -> i32 {
    let data_provider = get_dp(env);
    let keylet = match data_provider.slot_get(slot as usize) {
        None => return HostError::EmptySlot as i32,
        Some(key) => key.clone(),
    };
    data_provider.get_array_len(DataSource::KeyletLedgerObj(keylet), vec![field])
}
pub fn get_tx_nested_array_len(
    env: wasm_exec_env_t,
    in_buf_ptr: *mut u8,
    in_buf_len: usize,
) -> i32 {
    let data_provider = get_dp(env);
    let data = get_data(in_buf_ptr, in_buf_len);
    let idx_fields: Vec<i32> = match unpack_locator(data) {
        Ok(fields) => fields,
        Err(host_err) => return host_err as i32,
    };
    data_provider.get_array_len(DataSource::Tx, idx_fields)
}
pub fn get_current_ledger_obj_nested_array_len(
    env: wasm_exec_env_t,
    in_buf_ptr: *mut u8,
    in_buf_len: usize,
) -> i32 {
    let data_provider = get_dp(env);
    let data = get_data(in_buf_ptr, in_buf_len);
    let idx_fields: Vec<i32> = match unpack_locator(data) {
        Ok(fields) => fields,
        Err(host_err) => return host_err as i32,
    };
    data_provider.get_array_len(DataSource::CurrentLedgerObj, idx_fields)
}
pub fn get_ledger_obj_nested_array_len(
    env: wasm_exec_env_t,
    slot: i32,
    in_buf_ptr: *mut u8,
    in_buf_len: usize,
) -> i32 {
    let data_provider = get_dp(env);
    let keylet = match data_provider.slot_get(slot as usize) {
        None => return HostError::EmptySlot as i32,
        Some(key) => key.clone(),
    };

    let data = get_data(in_buf_ptr, in_buf_len);
    let idx_fields: Vec<i32> = match unpack_locator(data) {
        Ok(fields) => fields,
        Err(host_err) => return host_err as i32,
    };

    data_provider.get_array_len(DataSource::KeyletLedgerObj(keylet), idx_fields)
}
pub fn update_data(env: wasm_exec_env_t, in_buf_ptr: *mut u8, in_buf_len: usize) -> i32 {
    let data_provider = get_dp(env);
    if in_buf_len > XRPL_CONTRACT_DATA_SIZE {
        return HostError::DataFieldTooLarge as i32;
    }
    let data = get_data(in_buf_ptr, in_buf_len);
    data_provider.set_current_ledger_obj_data(data);
    0
}
pub fn compute_sha512_half(
    _env: wasm_exec_env_t,
    in_buf_ptr: *mut u8,
    in_buf_len: usize,
    out_buf_ptr: *mut u8,
    out_buf_cap: usize,
) -> i32 {
    if HASH256_LEN > out_buf_cap {
        return HostError::BufferTooSmall as i32;
    }
    if in_buf_len > MAX_WASM_PARAM_LENGTH {
        return HostError::DataFieldTooLarge as i32;
    }
    let data = get_data(in_buf_ptr, in_buf_len);
    let hash_half = sha512_half(&data);
    set_data(hash_half.len() as i32, out_buf_ptr, hash_half);
    HASH256_LEN as i32
}

pub fn account_keylet(
    _env: wasm_exec_env_t,
    in_buf_ptr: *mut u8,
    in_buf_len: usize,
    out_buf_ptr: *mut u8,
    out_buf_cap: usize,
) -> i32 {
    if HASH256_LEN > out_buf_cap {
        return HostError::BufferTooSmall as i32;
    }
    let data = get_data(in_buf_ptr, in_buf_len);
    if ACCOUNT_ID_LEN != data.len() {
        return HostError::InvalidAccount as i32;
    }
    let keylet_hash = index_hash(LedgerNameSpace::Account, &data);
    // let hex_str = hex::encode(&keylet_hash);
    // println!("Data (keylet_hash): {:?}", hex_str);
    set_data(keylet_hash.len() as i32, out_buf_ptr, keylet_hash);
    HASH256_LEN as i32
}

#[allow(clippy::too_many_arguments)]
pub fn credential_keylet(
    _env: wasm_exec_env_t,
    subject_ptr: *mut u8,
    subject_len: usize,
    issuer_ptr: *mut u8,
    issuer_len: usize,
    cred_type_ptr: *mut u8,
    cred_type_len: usize,
    out_buf_ptr: *mut u8,
    out_buf_cap: usize,
) -> i32 {
    if HASH256_LEN > out_buf_cap {
        return HostError::BufferTooSmall as i32;
    }
    let subject = get_data(subject_ptr, subject_len); // check length?
    let mut issuer = get_data(issuer_ptr, issuer_len);
    if ACCOUNT_ID_LEN != issuer.len() {
        return HostError::InvalidAccount as i32;
    }
    let mut cred_type = get_data(cred_type_ptr, cred_type_len); // check length?
    let mut data = subject;
    data.append(&mut issuer);
    data.append(&mut cred_type);
    let keylet_hash = index_hash(LedgerNameSpace::Credential, &data);
    set_data(keylet_hash.len() as i32, out_buf_ptr, keylet_hash);
    HASH256_LEN as i32
}
pub fn escrow_keylet(
    _env: wasm_exec_env_t,
    account_ptr: *mut u8,
    account_len: usize,
    sequence: u32,
    out_buf_ptr: *mut u8,
    out_buf_cap: usize,
) -> i32 {
    if HASH256_LEN > out_buf_cap {
        return HostError::BufferTooSmall as i32;
    }
    let mut data = get_data(account_ptr, account_len);
    if ACCOUNT_ID_LEN != data.len() {
        return HostError::InvalidAccount as i32;
    }
    let sqn_data = sequence.to_be_bytes();
    data.extend_from_slice(&sqn_data);
    let keylet_hash = index_hash(LedgerNameSpace::Escrow, &data);
    set_data(keylet_hash.len() as i32, out_buf_ptr, keylet_hash);
    HASH256_LEN as i32
}
pub fn oracle_keylet(
    _env: wasm_exec_env_t,
    account_ptr: *mut u8,
    account_len: usize,
    document_id: u32,
    out_buf_ptr: *mut u8,
    out_buf_cap: usize,
) -> i32 {
    if HASH256_LEN > out_buf_cap {
        return HostError::BufferTooSmall as i32;
    }
    let mut data = get_data(account_ptr, account_len);
    if ACCOUNT_ID_LEN != data.len() {
        return HostError::InvalidAccount as i32;
    }
    let sqn_data = document_id.to_be_bytes();
    data.extend_from_slice(&sqn_data);
    let keylet_hash = index_hash(LedgerNameSpace::Oracle, &data);
    set_data(keylet_hash.len() as i32, out_buf_ptr, keylet_hash);
    HASH256_LEN as i32
}

pub fn get_nft(
    env: wasm_exec_env_t,
    owner_ptr: *mut u8,
    owner_len: usize,
    nft_id_ptr: *mut u8,
    nft_id_len: usize,
    out_buf_ptr: *mut u8,
    out_buf_cap: usize,
) -> i32 {
    let data_provider = get_dp(env);
    let owner_id = get_data(owner_ptr, owner_len);
    if ACCOUNT_ID_LEN != owner_id.len() {
        return HostError::InvalidAccount as i32;
    }
    let nft_id = get_data(nft_id_ptr, nft_id_len);
    if HASH256_LEN != nft_id.len() {
        return HostError::InvalidParams as i32;
    }
    let dp_res = data_provider.get_nft_uri(&nft_id, &owner_id, out_buf_cap);
    set_data(dp_res.0, out_buf_ptr, dp_res.1);
    dp_res.0
}

fn unpack_in_float(env: wasm_exec_env_t, in_buf: *const u8) -> Result<BigDecimal, HostError> {
    let bytes: [u8; 8] = unsafe {
        let inst = wasm_runtime_get_module_inst(env);
        if !wasm_runtime_validate_native_addr(inst, in_buf as *mut ::core::ffi::c_void, 8) {
            return Err(HostError::PointerOutOfBound);
        }
        match std::slice::from_raw_parts(in_buf, 8).try_into() {
            Ok(bytes) => bytes,
            Err(_) => return Err(HostError::FloatInputMalformed),
        }
    };
    _deserialize_issued_currency_amount(bytes).map_err(|_| HostError::FloatInputMalformed)
}

fn pack_out_float(decimal: BigDecimal, env: wasm_exec_env_t, out_buf: *mut u8) -> i32 {
    let bytes: [u8; 8] = match _serialize_issued_currency_value(decimal) {
        Ok(bytes) => bytes,
        Err(_) => return HostError::FloatComputationError as i32,
    };

    unsafe {
        let inst = wasm_runtime_get_module_inst(env);
        if !wasm_runtime_validate_native_addr(inst, out_buf as *mut ::core::ffi::c_void, 8) {
            return HostError::PointerOutOfBound as i32;
        }
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), out_buf, 8);
    }
    8
}

pub fn float_from_int(env: wasm_exec_env_t, in_int: i64, out_buf: *mut u8) -> i32 {
    let a = BigDecimal::from(in_int);
    pack_out_float(a, env, out_buf)
}

pub fn float_set(env: wasm_exec_env_t, exponent: i32, mantissa: i64, out_buf: *mut u8) -> i32 {
    let value = BigDecimal::from_bigint(BigInt::from(mantissa), -exponent as i64);
    // warn!("float_set {value}");
    pack_out_float(value, env, out_buf)
}

pub fn float_compare(env: wasm_exec_env_t, in_buf1: *const u8, in_buf2: *const u8) -> i32 {
    let f1 = match unpack_in_float(env, in_buf1) {
        Ok(val) => val,
        Err(e) => return e as i32,
    };
    let f2 = match unpack_in_float(env, in_buf2) {
        Ok(val) => val,
        Err(e) => return e as i32,
    };
    // warn!("float_compare {f1} {f2}");
    if f1 == f2 {
        0
    } else if f1 > f2 {
        1
    } else {
        2
    }
}
pub fn float_add(
    env: wasm_exec_env_t,
    in_buf1: *const u8,
    in_buf2: *const u8,
    out_buf: *mut u8,
) -> i32 {
    let f1 = match unpack_in_float(env, in_buf1) {
        Ok(val) => val,
        Err(e) => return e as i32,
    };
    let f2 = match unpack_in_float(env, in_buf2) {
        Ok(val) => val,
        Err(e) => return e as i32,
    };
    let r = f1 + f2;
    pack_out_float(r, env, out_buf)
}
pub fn float_subtract(
    env: wasm_exec_env_t,
    in_buf1: *const u8,
    in_buf2: *const u8,
    out_buf: *mut u8,
) -> i32 {
    let f1 = match unpack_in_float(env, in_buf1) {
        Ok(val) => val,
        Err(e) => return e as i32,
    };
    let f2 = match unpack_in_float(env, in_buf2) {
        Ok(val) => val,
        Err(e) => return e as i32,
    };
    let r = f1 - f2;
    pack_out_float(r, env, out_buf)
}
pub fn float_multiply(
    env: wasm_exec_env_t,
    in_buf1: *const u8,
    in_buf2: *const u8,
    out_buf: *mut u8,
) -> i32 {
    let f1 = match unpack_in_float(env, in_buf1) {
        Ok(val) => val,
        Err(e) => return e as i32,
    };
    let f2 = match unpack_in_float(env, in_buf2) {
        Ok(val) => val,
        Err(e) => return e as i32,
    };
    let r = f1 * f2;
    pack_out_float(r, env, out_buf)
}
pub fn float_divide(
    env: wasm_exec_env_t,
    in_buf1: *const u8,
    in_buf2: *const u8,
    out_buf: *mut u8,
) -> i32 {
    let f1 = match unpack_in_float(env, in_buf1) {
        Ok(val) => val,
        Err(e) => return e as i32,
    };
    let f2 = match unpack_in_float(env, in_buf2) {
        Ok(val) => val,
        Err(e) => return e as i32,
    };
    let r = f1 / f2;
    pack_out_float(r, env, out_buf)
}
pub fn float_root(env: wasm_exec_env_t, in_buf: *const u8, in_int: i32, out_buf: *mut u8) -> i32 {
    let f = match unpack_in_float(env, in_buf) {
        Ok(val) => match val.to_f64() {
            Some(f) => f,
            None => return HostError::FloatComputationError as i32,
        },
        Err(e) => return e as i32,
    };

    if in_int <= 0 {
        return HostError::InvalidParams as i32;
    }

    let f = f.powf(1.0 / in_int as f64);
    let r = match BigDecimal::try_from(f) {
        Ok(val) => val,
        Err(_) => return HostError::FloatComputationError as i32,
    };
    // warn!("float_root {r}");
    pack_out_float(r, env, out_buf)
}
pub fn float_log(env: wasm_exec_env_t, in_buf: *const u8, out_buf: *mut u8) -> i32 {
    let f = match unpack_in_float(env, in_buf) {
        Ok(val) => match val.to_f64() {
            Some(f) => f,
            None => return HostError::FloatComputationError as i32,
        },
        Err(e) => return e as i32,
    };

    let f = f.log10();
    let r = match BigDecimal::try_from(f) {
        Ok(val) => val,
        Err(_) => return HostError::FloatComputationError as i32,
    };

    pack_out_float(r, env, out_buf)
}
///////////////////////////////////////////////////////////////////////////////

fn read_utf8_from_wasm(msg_read_ptr: *mut u8, msg_read_len: usize) -> Option<String> {
    String::from_utf8(get_data(msg_read_ptr, msg_read_len)).ok()
}
fn read_hex_from_wasm(
    data_read_ptr: *mut u8,
    data_read_len: usize,
    data_as_hex: bool,
) -> Option<String> {
    if data_as_hex {
        // Read the data from memory.
        let bytes_vec: Vec<u8> = get_data(data_read_ptr, data_read_len);
        let mut final_hex_string = "0x".to_owned();
        let hex_data = hex::encode_upper(&bytes_vec);
        final_hex_string.push_str(hex_data.as_str());
        Some(final_hex_string)
    } else {
        read_utf8_from_wasm(data_read_ptr, data_read_len)
    }
}

pub fn trace(
    _env: wasm_exec_env_t,
    msg_read_ptr: *mut u8,
    msg_read_len: usize,
    data_read_ptr: *mut u8,
    data_read_len: usize,
    data_as_hex: i32,
) -> i32 {
    // Don't need to check number of inputs or types since these will manifest at runtime and
    // cancel execution of the contract.

    if msg_read_len > MAX_WASM_PARAM_LENGTH || data_read_len > MAX_WASM_PARAM_LENGTH {
        return HostError::DataFieldTooLarge as i32;
    }

    let data_as_hex = {
        match data_as_hex {
            0 => false,
            1 => true,
            // If an invalid value is supplied, assume `true`
            _ => true,
        }
    };

    debug!(
        "trace() params: msg_read_ptr={:?} msg_read_len={} data_read_ptr={:?} data_read_len={}",
        msg_read_ptr, msg_read_len, data_read_ptr, data_read_len
    );

    let Some(message) = read_utf8_from_wasm(msg_read_ptr, msg_read_len) else {
        return HostError::DecodingError as i32;
    };

    let Some(data_string) = read_hex_from_wasm(data_read_ptr, data_read_len, data_as_hex) else {
        return HostError::DecodingError as i32;
    };

    if data_read_len > 0 {
        println!(
            "WASM TRACE: {message} ({data_string} | {} data bytes)",
            data_read_len
        );
    } else {
        println!("WASM TRACE: {message}");
    }

    (data_read_len + msg_read_len + 1) as i32
}

pub fn trace_num(
    _env: wasm_exec_env_t,
    msg_read_ptr: *mut u8,
    msg_read_len: usize,
    number: i64,
) -> i32 {
    // Don't need to check number of inputs or types since these will manifest at runtime and
    // cancel execution of the contract.

    if msg_read_len > MAX_WASM_PARAM_LENGTH {
        return HostError::DataFieldTooLarge as i32;
    }

    debug!(
        "trace() params: msg_read_ptr={:?} msg_read_len={} number={} ",
        msg_read_ptr, msg_read_len, number
    );
    let Some(message) = read_utf8_from_wasm(msg_read_ptr, msg_read_len) else {
        return HostError::DecodingError as i32;
    };

    if (number < 0) {
        let error_code_str = error_code_to_string(number);
        println!("WASM TRACE[ERROR]: {message} {error_code_str}");
    } else {
        println!("WASM TRACE: {message} {number}");
    }
    0
}

pub fn trace_opaque_float(
    _env: wasm_exec_env_t,
    msg_read_ptr: *mut u8,
    msg_read_len: usize,
    opaque_float_ptr: *mut u8,
    opaque_float_len: usize,
) -> i32 {
    debug!(
        "trace() params: msg_read_ptr={:#?} msg_read_len={:#?} _opaque_float_ptr={:#?} _opaque_float_len={:#?}",
        msg_read_ptr, msg_read_len, opaque_float_ptr, opaque_float_len
    );
    let Some(message) = read_utf8_from_wasm(msg_read_ptr, msg_read_len) else {
        return HostError::DecodingError as i32;
    };

    let data = get_data(opaque_float_ptr, opaque_float_len);
    let array: [u8; 8] = data
        .try_into()
        .map_err(|v: Vec<u8>| format!("Expected 8 bytes, got {}", v.len()))
        .unwrap();
    let opaque_float: u64 = u64::from_be_bytes(array);

    // TODO: call trace_num when it supports u64?
    println!("WASM TRACE: {message} {opaque_float}");
    0
}

pub fn trace_opaque_float(
    _env: wasm_exec_env_t,
    msg_read_ptr: *mut u8,
    msg_read_len: usize,
    op_float: *const u8,
) -> i32 {
    let f = match unpack_in_float(_env, op_float) {
        Ok(val) => val,
        Err(e) => return e as i32,
    };
    debug!(
        "trace() params: msg_read_ptr={:?} msg_read_len={} float={} ",
        msg_read_ptr, msg_read_len, f
    );
    let Some(message) = read_utf8_from_wasm(msg_read_ptr, msg_read_len) else {
        return HostError::DecodingError as i32;
    };

    println!("WASM TRACE: {message} {f}");
    0
}
