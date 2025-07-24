#![allow(unused)]

use crate::data_provider::{DataProvider, HostError, XRPL_CONTRACT_DATA_SIZE, unpack_locator};
use crate::hashing::{HASH256_LEN, Hash256, LedgerNameSpace, index_hash, sha512_half};
use crate::host_function_utils::{read_hex_from_wasm, read_utf8_from_wasm};
use crate::mock_data::{DataSource, Keylet};
use log::debug;
use wasmedge_sdk::error::{CoreError, CoreExecutionError};
use wasmedge_sdk::{CallingFrame, Instance, WasmValue};

fn get_data(
    in_buf_ptr: i32,
    in_buf_len: i32,
    _caller: &mut CallingFrame,
) -> Result<Vec<u8>, CoreError> {
    let memory = _caller.memory_mut(0).ok_or_else(|| {
        eprintln!("get_tx_hash_helper: Error: Failed to get memory instance");
        CoreError::Execution(CoreExecutionError::MemoryOutOfBounds)
    })?;
    let buffer = memory
        .get_data(in_buf_ptr as u32, in_buf_len as u32)
        .map_err(|e| {
            eprintln!(
                "get_tx_hash_helper: Error: Failed to get memory data: {}",
                e
            );
            CoreError::Execution(CoreExecutionError::MemoryOutOfBounds)
        })?;
    Ok(buffer)
}

fn get_keylet(
    in_buf_ptr: i32,
    in_buf_len: i32,
    _caller: &mut CallingFrame,
) -> Result<Keylet, CoreError> {
    get_data(in_buf_ptr, in_buf_len, _caller)
}

fn set_data(
    dp_res: i32,
    out_buf_ptr: i32,
    data_to_write: Vec<u8>,
    _caller: &mut CallingFrame,
) -> Result<(), CoreError> {
    if dp_res > 0 {
        let mut memory = _caller.memory_mut(0).ok_or_else(|| {
            eprintln!("get_tx_hash_helper: Error: Failed to get memory instance");
            CoreError::Execution(CoreExecutionError::MemoryOutOfBounds)
        })?;
        memory
            .set_data(&data_to_write, out_buf_ptr as u32)
            .map_err(|e| {
                eprintln!(
                    "get_tx_hash_helper: Error: Failed to set memory data: {}",
                    e
                );
                CoreError::Execution(CoreExecutionError::MemoryOutOfBounds)
            })?;
    }
    Ok(())
}

pub fn get_ledger_sqn(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let out_buf_ptr: i32 = _inputs[0].to_i32();
    let out_buf_cap: i32 = _inputs[1].to_i32();
    let dp_res = _data_provider.get_ledger_sqn(out_buf_cap as usize);
    set_data(dp_res.0, out_buf_ptr, dp_res.1, _caller)?;
    Ok(vec![WasmValue::from_i32(dp_res.0)])
}

pub fn get_parent_ledger_time(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let out_buf_ptr: i32 = _inputs[0].to_i32();
    let out_buf_cap: i32 = _inputs[1].to_i32();
    let dp_res = _data_provider.get_parent_ledger_time(out_buf_cap as usize);
    set_data(dp_res.0, out_buf_ptr, dp_res.1, _caller)?;
    Ok(vec![WasmValue::from_i32(dp_res.0)])
}

pub fn get_parent_ledger_hash(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let out_buf_ptr: i32 = _inputs[0].to_i32();
    let out_buf_cap: i32 = _inputs[1].to_i32();
    let dp_res = _data_provider.get_parent_ledger_hash(out_buf_cap as usize);
    set_data(dp_res.0, out_buf_ptr, dp_res.1, _caller)?;
    Ok(vec![WasmValue::from_i32(dp_res.0)])
}

pub fn cache_ledger_obj(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let in_buf_ptr: i32 = _inputs[0].to_i32();
    let in_buf_cap: i32 = _inputs[1].to_i32();
    let cache_num: i32 = _inputs[2].to_i32();
    let keylet = get_keylet(in_buf_ptr, in_buf_cap, _caller)?;
    let dp_res = _data_provider.slot_set(keylet, cache_num as usize);
    Ok(vec![WasmValue::from_i32(dp_res)])
}

pub fn get_tx_field(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let field: i32 = _inputs[0].to_i32();
    let out_buf_ptr: i32 = _inputs[1].to_i32();
    let out_buf_cap: i32 = _inputs[2].to_i32();
    let dp_res = _data_provider.get_field_value(DataSource::Tx, vec![field], out_buf_cap as usize);
    set_data(dp_res.0, out_buf_ptr, dp_res.1, _caller)?;
    Ok(vec![WasmValue::from_i32(dp_res.0)])
}

pub fn get_tx_field2(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let field: i32 = _inputs[0].to_i32();
    let field2: i32 = _inputs[1].to_i32();
    let out_buf_ptr: i32 = _inputs[2].to_i32();
    let out_buf_cap: i32 = _inputs[3].to_i32();
    let dp_res =
        _data_provider.get_field_value(DataSource::Tx, vec![field, field2], out_buf_cap as usize);
    set_data(dp_res.0, out_buf_ptr, dp_res.1, _caller)?;
    Ok(vec![WasmValue::from_i32(dp_res.0)])
}

pub fn get_tx_field3(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let field: i32 = _inputs[0].to_i32();
    let field2: i32 = _inputs[1].to_i32();
    let field3: i32 = _inputs[2].to_i32();
    let out_buf_ptr: i32 = _inputs[3].to_i32();
    let out_buf_cap: i32 = _inputs[4].to_i32();
    let dp_res = _data_provider.get_field_value(
        DataSource::Tx,
        vec![field, field2, field3],
        out_buf_cap as usize,
    );
    set_data(dp_res.0, out_buf_ptr, dp_res.1, _caller)?;
    Ok(vec![WasmValue::from_i32(dp_res.0)])
}

pub fn get_tx_field4(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let field: i32 = _inputs[0].to_i32();
    let field2: i32 = _inputs[1].to_i32();
    let field3: i32 = _inputs[2].to_i32();
    let field4: i32 = _inputs[3].to_i32();
    let out_buf_ptr: i32 = _inputs[4].to_i32();
    let out_buf_cap: i32 = _inputs[5].to_i32();
    let dp_res = _data_provider.get_field_value(
        DataSource::Tx,
        vec![field, field2, field3, field4],
        out_buf_cap as usize,
    );
    set_data(dp_res.0, out_buf_ptr, dp_res.1, _caller)?;
    Ok(vec![WasmValue::from_i32(dp_res.0)])
}

pub fn get_tx_field5(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let field: i32 = _inputs[0].to_i32();
    let field2: i32 = _inputs[1].to_i32();
    let field3: i32 = _inputs[2].to_i32();
    let field4: i32 = _inputs[3].to_i32();
    let field5: i32 = _inputs[4].to_i32();
    let out_buf_ptr: i32 = _inputs[5].to_i32();
    let out_buf_cap: i32 = _inputs[6].to_i32();
    let dp_res = _data_provider.get_field_value(
        DataSource::Tx,
        vec![field, field2, field3, field4, field5],
        out_buf_cap as usize,
    );
    set_data(dp_res.0, out_buf_ptr, dp_res.1, _caller)?;
    Ok(vec![WasmValue::from_i32(dp_res.0)])
}

pub fn get_tx_field6(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let field: i32 = _inputs[0].to_i32();
    let field2: i32 = _inputs[1].to_i32();
    let field3: i32 = _inputs[2].to_i32();
    let field4: i32 = _inputs[3].to_i32();
    let field5: i32 = _inputs[4].to_i32();
    let field6: i32 = _inputs[5].to_i32();

    let out_buf_ptr: i32 = _inputs[6].to_i32();
    let out_buf_cap: i32 = _inputs[7].to_i32();
    let dp_res = _data_provider.get_field_value(
        DataSource::Tx,
        vec![field, field2, field3, field4, field5, field6],
        out_buf_cap as usize,
    );
    set_data(dp_res.0, out_buf_ptr, dp_res.1, _caller)?;
    Ok(vec![WasmValue::from_i32(dp_res.0)])
}

pub fn get_current_ledger_obj_field(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let field: i32 = _inputs[0].to_i32();
    let out_buf_ptr: i32 = _inputs[1].to_i32();
    let out_buf_cap: i32 = _inputs[2].to_i32();
    let dp_res = _data_provider.get_field_value(
        DataSource::CurrentLedgerObj,
        vec![field],
        out_buf_cap as usize,
    );
    set_data(dp_res.0, out_buf_ptr, dp_res.1, _caller)?;
    Ok(vec![WasmValue::from_i32(dp_res.0)])
}

pub fn get_ledger_obj_field(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let slot: i32 = _inputs[0].to_i32();
    let field: i32 = _inputs[1].to_i32();
    let out_buf_ptr: i32 = _inputs[2].to_i32();
    let out_buf_cap: i32 = _inputs[3].to_i32();
    let keylet = match _data_provider.slot_get(slot as usize) {
        None => return Ok(vec![WasmValue::from_i32(HostError::SlotOutRange as i32)]),
        Some(key) => key.clone(),
    };
    let dp_res = _data_provider.get_field_value(
        DataSource::KeyletLedgerObj(keylet),
        vec![field],
        out_buf_cap as usize,
    );
    set_data(dp_res.0, out_buf_ptr, dp_res.1, _caller)?;
    Ok(vec![WasmValue::from_i32(dp_res.0)])
}

pub fn get_tx_nested_field(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let in_buf_ptr: i32 = _inputs[0].to_i32();
    let in_buf_len: i32 = _inputs[1].to_i32();
    let out_buf_ptr: i32 = _inputs[2].to_i32();
    let out_buf_cap: i32 = _inputs[3].to_i32();

    let data = get_data(in_buf_ptr, in_buf_len, _caller)?;
    let idx_fields: Vec<i32> = match unpack_locator(data) {
        Ok(fields) => fields,
        Err(host_err) => return Ok(vec![WasmValue::from_i32(host_err as i32)]),
    };

    let dp_res = _data_provider.get_field_value(DataSource::Tx, idx_fields, out_buf_cap as usize);
    set_data(dp_res.0, out_buf_ptr, dp_res.1, _caller)?;
    Ok(vec![WasmValue::from_i32(dp_res.0)])
}

pub fn get_current_ledger_obj_nested_field(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let in_buf_ptr: i32 = _inputs[0].to_i32();
    let in_buf_len: i32 = _inputs[1].to_i32();
    let out_buf_ptr: i32 = _inputs[2].to_i32();
    let out_buf_cap: i32 = _inputs[3].to_i32();

    let data = get_data(in_buf_ptr, in_buf_len, _caller)?;
    let idx_fields: Vec<i32> = match unpack_locator(data) {
        Ok(fields) => fields,
        Err(host_err) => return Ok(vec![WasmValue::from_i32(host_err as i32)]),
    };

    let dp_res = _data_provider.get_field_value(
        DataSource::CurrentLedgerObj,
        idx_fields,
        out_buf_cap as usize,
    );
    set_data(dp_res.0, out_buf_ptr, dp_res.1, _caller)?;
    Ok(vec![WasmValue::from_i32(dp_res.0)])
}

pub fn get_ledger_obj_nested_field(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let slot: i32 = _inputs[0].to_i32();
    let in_buf_ptr: i32 = _inputs[1].to_i32();
    let in_buf_len: i32 = _inputs[2].to_i32();
    let out_buf_ptr: i32 = _inputs[3].to_i32();
    let out_buf_cap: i32 = _inputs[4].to_i32();
    let keylet = match _data_provider.slot_get(slot as usize) {
        None => return Ok(vec![WasmValue::from_i32(HostError::SlotOutRange as i32)]),
        Some(key) => key.clone(),
    };

    let data = get_data(in_buf_ptr, in_buf_len, _caller)?;
    let idx_fields: Vec<i32> = match unpack_locator(data) {
        Ok(fields) => fields,
        Err(host_err) => return Ok(vec![WasmValue::from_i32(host_err as i32)]),
    };

    let dp_res = _data_provider.get_field_value(
        DataSource::KeyletLedgerObj(keylet),
        idx_fields,
        out_buf_cap as usize,
    );
    set_data(dp_res.0, out_buf_ptr, dp_res.1, _caller)?;
    Ok(vec![WasmValue::from_i32(dp_res.0)])
}

pub fn get_tx_array_len(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let field: i32 = _inputs[0].to_i32();
    Ok(vec![WasmValue::from_i32(
        _data_provider.get_array_len(DataSource::Tx, vec![field]),
    )])
}

pub fn get_current_ledger_obj_array_len(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let field: i32 = _inputs[0].to_i32();
    Ok(vec![WasmValue::from_i32(
        _data_provider.get_array_len(DataSource::CurrentLedgerObj, vec![field]),
    )])
}

pub fn get_ledger_obj_array_len(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let slot: i32 = _inputs[0].to_i32();
    let field: i32 = _inputs[1].to_i32();

    let keylet = match _data_provider.slot_get(slot as usize) {
        None => return Ok(vec![WasmValue::from_i32(HostError::SlotOutRange as i32)]),
        Some(key) => key.clone(),
    };
    Ok(vec![WasmValue::from_i32(_data_provider.get_array_len(
        DataSource::KeyletLedgerObj(keylet),
        vec![field],
    ))])
}

pub fn get_tx_nested_array_len(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let in_buf_ptr: i32 = _inputs[0].to_i32();
    let in_buf_len: i32 = _inputs[1].to_i32();

    let data = get_data(in_buf_ptr, in_buf_len, _caller)?;
    let idx_fields: Vec<i32> = match unpack_locator(data) {
        Ok(fields) => fields,
        Err(host_err) => return Ok(vec![WasmValue::from_i32(host_err as i32)]),
    };
    Ok(vec![WasmValue::from_i32(
        _data_provider.get_array_len(DataSource::Tx, idx_fields),
    )])
}

pub fn get_current_ledger_obj_nested_array_len(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let in_buf_ptr: i32 = _inputs[0].to_i32();
    let in_buf_len: i32 = _inputs[1].to_i32();

    let data = get_data(in_buf_ptr, in_buf_len, _caller)?;
    let idx_fields: Vec<i32> = match unpack_locator(data) {
        Ok(fields) => fields,
        Err(host_err) => return Ok(vec![WasmValue::from_i32(host_err as i32)]),
    };
    Ok(vec![WasmValue::from_i32(
        _data_provider.get_array_len(DataSource::CurrentLedgerObj, idx_fields),
    )])
}

pub fn get_ledger_obj_nested_array_len(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let slot: i32 = _inputs[0].to_i32();
    let in_buf_ptr: i32 = _inputs[1].to_i32();
    let in_buf_len: i32 = _inputs[2].to_i32();
    let keylet = match _data_provider.slot_get(slot as usize) {
        None => return Ok(vec![WasmValue::from_i32(HostError::SlotOutRange as i32)]),
        Some(key) => key.clone(),
    };

    let data = get_data(in_buf_ptr, in_buf_len, _caller)?;
    let idx_fields: Vec<i32> = match unpack_locator(data) {
        Ok(fields) => fields,
        Err(host_err) => return Ok(vec![WasmValue::from_i32(host_err as i32)]),
    };
    Ok(vec![WasmValue::from_i32(_data_provider.get_array_len(
        DataSource::KeyletLedgerObj(keylet),
        idx_fields,
    ))])
}

pub fn update_data(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let in_buf_ptr: i32 = _inputs[0].to_i32();
    let in_buf_len: i32 = _inputs[1].to_i32();
    if in_buf_len as usize > XRPL_CONTRACT_DATA_SIZE {
        return Ok(vec![WasmValue::from_i32(
            HostError::DataFieldTooLarge as i32,
        )]);
    }
    let data = get_data(in_buf_ptr, in_buf_len, _caller)?;
    _data_provider.set_current_ledger_obj_data(data);
    Ok(vec![WasmValue::from_i32(0)])
}

pub fn compute_sha512_half(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let in_buf_ptr: i32 = _inputs[0].to_i32();
    let in_buf_len: i32 = _inputs[1].to_i32();
    let out_buf_ptr: i32 = _inputs[2].to_i32();
    let out_buf_cap: i32 = _inputs[3].to_i32();

    if HASH256_LEN > out_buf_cap as usize {
        return Ok(vec![WasmValue::from_i32(HostError::BufferTooSmall as i32)]);
    }
    let data = get_data(in_buf_ptr, in_buf_len, _caller)?;
    let hash_half = sha512_half(&data);
    set_data(hash_half.len() as i32, out_buf_ptr, hash_half, _caller)?;
    Ok(vec![WasmValue::from_i32(HASH256_LEN as i32)])
}

pub fn account_keylet(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let in_buf_ptr: i32 = _inputs[0].to_i32();
    let in_buf_len: i32 = _inputs[1].to_i32();
    let out_buf_ptr: i32 = _inputs[2].to_i32();
    let out_buf_cap: i32 = _inputs[3].to_i32();

    if HASH256_LEN > out_buf_cap as usize {
        return Ok(vec![WasmValue::from_i32(HostError::BufferTooSmall as i32)]);
    }
    let data = get_data(in_buf_ptr, in_buf_len, _caller)?;
    let keylet_hash: Hash256 = index_hash(LedgerNameSpace::Account, &data);

    let hex_str = hex::encode(&keylet_hash);
    println!("Data (keylet_hash): {:?}", hex_str);
    set_data(keylet_hash.len() as i32, out_buf_ptr, keylet_hash, _caller)?;
    Ok(vec![WasmValue::from_i32(HASH256_LEN as i32)])
}

pub fn credential_keylet(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let subject_ptr: i32 = _inputs[0].to_i32();
    let subject_len: i32 = _inputs[1].to_i32();
    let issuer_ptr: i32 = _inputs[2].to_i32();
    let issuer_len: i32 = _inputs[3].to_i32();
    let cred_type_ptr: i32 = _inputs[4].to_i32();
    let cred_type_len: i32 = _inputs[5].to_i32();
    let out_buf_ptr: i32 = _inputs[6].to_i32();
    let out_buf_cap: i32 = _inputs[7].to_i32();

    if HASH256_LEN > out_buf_cap as usize {
        return Ok(vec![WasmValue::from_i32(HostError::BufferTooSmall as i32)]);
    }
    let subject = get_data(subject_ptr, subject_len, _caller)?;
    let mut issuer = get_data(issuer_ptr, issuer_len, _caller)?;
    let mut cred_type = get_data(cred_type_ptr, cred_type_len, _caller)?;
    let mut data = subject;
    data.append(&mut issuer);
    data.append(&mut cred_type);

    let keylet_hash = index_hash(LedgerNameSpace::Credential, &data);
    set_data(keylet_hash.len() as i32, out_buf_ptr, keylet_hash, _caller)?;
    Ok(vec![WasmValue::from_i32(HASH256_LEN as i32)])
}

pub fn escrow_keylet(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let account_ptr: i32 = _inputs[0].to_i32();
    let account_len: i32 = _inputs[1].to_i32();
    let sequence: u32 = _inputs[2].to_i32() as u32;
    let out_buf_ptr: i32 = _inputs[3].to_i32();
    let out_buf_cap: i32 = _inputs[4].to_i32();

    if HASH256_LEN > out_buf_cap as usize {
        return Ok(vec![WasmValue::from_i32(HostError::BufferTooSmall as i32)]);
    }
    let mut data = get_data(account_ptr, account_len, _caller)?;
    let sqn_data = sequence.to_be_bytes();
    data.extend_from_slice(&sqn_data);

    let keylet_hash = index_hash(LedgerNameSpace::Escrow, &data);
    set_data(keylet_hash.len() as i32, out_buf_ptr, keylet_hash, _caller)?;
    Ok(vec![WasmValue::from_i32(HASH256_LEN as i32)])
}

pub fn oracle_keylet(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let account_ptr: i32 = _inputs[0].to_i32();
    let account_len: i32 = _inputs[1].to_i32();
    let document_id: u32 = _inputs[2].to_i32() as u32;
    let out_buf_ptr: i32 = _inputs[3].to_i32();
    let out_buf_cap: i32 = _inputs[4].to_i32();

    if HASH256_LEN > out_buf_cap as usize {
        return Ok(vec![WasmValue::from_i32(HostError::BufferTooSmall as i32)]);
    }
    let mut data = get_data(account_ptr, account_len, _caller)?;
    let sqn_data = document_id.to_be_bytes();
    data.extend_from_slice(&sqn_data);

    let keylet_hash = index_hash(LedgerNameSpace::Oracle, &data);
    set_data(keylet_hash.len() as i32, out_buf_ptr, keylet_hash, _caller)?;
    Ok(vec![WasmValue::from_i32(HASH256_LEN as i32)])
}

pub fn get_nft(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let owner_ptr: i32 = _inputs[0].to_i32();
    let owner_len: i32 = _inputs[1].to_i32();
    let nft_id_ptr: i32 = _inputs[2].to_i32();
    let nft_id_len: i32 = _inputs[3].to_i32();
    let out_buf_ptr: i32 = _inputs[4].to_i32();
    let out_buf_cap: i32 = _inputs[5].to_i32();

    let owner_id = get_data(owner_ptr, owner_len, _caller)?;
    let nft_id = get_data(nft_id_ptr, nft_id_len, _caller)?;
    let dp_res = _data_provider.get_nft_uri(&nft_id, &owner_id, out_buf_cap as usize);
    set_data(dp_res.0, out_buf_ptr, dp_res.1, _caller)?;
    Ok(vec![WasmValue::from_i32(dp_res.0)])
}

pub fn trace(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    // Don't need to check number of inputs or types since these will manifest at runtime and
    // cancel execution of the contract.

    let msg_read_ptr: u32 = inputs[0].to_i32() as u32;
    let msg_read_len: u32 = inputs[1].to_i32() as u32;
    let data_read_ptr: u32 = inputs[2].to_i32() as u32;
    let data_read_len: u32 = inputs[3].to_i32() as u32;
    let data_as_hex = {
        match inputs[4].to_i32() {
            0 => false,
            1 => true,
            // If an invalid value is supplied, assume `true`
            _ => true,
        }
    };

    debug!(
        "trace() params: msg_read_ptr={} msg_read_len={} data_read_ptr={} data_read_len={}",
        msg_read_ptr, msg_read_len, data_read_ptr, data_read_len
    );

    let message = read_utf8_from_wasm(_caller, msg_read_ptr as i32, msg_read_len as i32)?;
    let data_string = read_hex_from_wasm(
        _caller,
        data_read_ptr as i32,
        data_read_len as i32,
        data_as_hex,
    )?;
    if data_read_len > 0 {
        println!(
            "WASM TRACE: {message} ({data_string} | {} data bytes)",
            data_read_len
        );
    } else {
        println!("WASM TRACE: {message}");
    }

    Ok(vec![WasmValue::from_i32(
        (data_read_len + msg_read_len + 1) as i32,
    )])
}

pub fn trace_num(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    // Don't need to check number of inputs or types since these will manifest at runtime and
    // cancel execution of the contract.

    let msg_read_ptr: u32 = inputs[0].to_i32() as u32;
    let msg_read_len: u32 = inputs[1].to_i32() as u32;
    let number: u64 = inputs[2].to_i64() as u64;

    debug!(
        "trace() params: msg_read_ptr={} msg_read_len={} number={} ",
        msg_read_ptr, msg_read_len, number
    );

    let message = read_utf8_from_wasm(_caller, msg_read_ptr as i32, msg_read_len as i32)?;
    println!("WASM TRACE: {message} {number}");

    Ok(vec![WasmValue::from_i32(0)])
}
