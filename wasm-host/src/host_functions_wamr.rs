#![allow(unused)]
use crate::data_provider::{DataProvider, HostError, XRPL_CONTRACT_DATA_SIZE, unpack_locator};
use crate::decoding::ACCOUNT_ID_LEN;
use crate::hashing::{HASH256_LEN, LedgerNameSpace, index_hash, sha512_half};
use crate::mock_data::{DataSource, Keylet};
use log::debug;
// use bls12_381::{Scalar, pairing, G1Projective, G1Affine, G2Affine, Gt, G2Prepared, multi_miller_loop};
// use group::{Curve};

use ark_bn254::{G1Affine, G2Affine, Bn254, Fr, Fq, Fq2};
use ark_ff::{One, PrimeField};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_ec::{pairing::Pairing, AffineRepr, CurveGroup};

use wamr_rust_sdk::sys::{wasm_exec_env_t, wasm_runtime_get_function_attachment};

const MAX_WASM_PARAM_LENGTH: usize = 1024;
const FQ_LEN: usize = 32;
const G1_LEN: usize = 2 * FQ_LEN;
const G2_LEN: usize = 128;
const SCALAR_LEN: usize = 32;

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
    let e = get_dp(env);
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

pub fn ec_add_helper_bn254(
    _env: wasm_exec_env_t,
    p1_ptr: *mut u8,
    p1_len: usize,
    p2_ptr: *mut u8,
    p2_len: usize,
    out_buff_ptr: *mut u8,
    out_buff_len: usize,
) -> i32 {
    const G1_LEN: usize = 64;
    const ERROR: i32 = -99;

    if p1_ptr.is_null() || p1_len != G1_LEN 
        || p2_ptr.is_null() || p2_len != G1_LEN 
        || out_buff_ptr.is_null() || out_buff_len < G1_LEN 
    {
        return ERROR;
    }
    let mut p1_slice = get_data(p1_ptr, p1_len);
    let mut p2_slice = get_data(p2_ptr, p2_len);

    // To little-endian
    let (p1_x_slice, p1_y_slice) = p1_slice.split_at_mut(G1_LEN/2);
    p1_x_slice.reverse();
    p1_y_slice.reverse();
    let (p2_x_slice, p2_y_slice) = p2_slice.split_at_mut(G1_LEN/2);
    p2_x_slice.reverse();
    p2_y_slice.reverse();

    let p1_x = Fq::deserialize_uncompressed(&*p1_x_slice).expect("Invalid BN254 field element");
    let p1_y = Fq::deserialize_uncompressed(&*p1_y_slice).expect("Invalid BN254 field element");
    let p1 = G1Affine::new_unchecked(p1_x, p1_y);

    let p2_x = Fq::deserialize_uncompressed(&*p2_x_slice).expect("Invalid BN254 field element");
    let p2_y = Fq::deserialize_uncompressed(&*p2_y_slice).expect("Invalid BN254 field element");
    let p2 = G1Affine::new_unchecked(p2_x, p2_y);

    let sum = p1.into_group() + p2;
    let result = sum.into_affine();

    let mut result_bytes = [0u8; G1_LEN];
    let Some((x, y)) = result.xy() else {
        return ERROR; 
    };

    // To big-endian
    let mut x_bytes = [0u8; FQ_LEN];
    x.serialize_uncompressed(&mut x_bytes[..]).expect("serialize x");
    x_bytes.reverse(); 
    let mut y_bytes = [0u8; FQ_LEN];
    y.serialize_uncompressed(&mut y_bytes[..]).expect("serialize y");
    y_bytes.reverse(); 

    result_bytes[0..FQ_LEN].copy_from_slice(&x_bytes);
    result_bytes[FQ_LEN..(2 * FQ_LEN)].copy_from_slice(&y_bytes);
    if result_bytes.len() > out_buff_len {
        return ERROR;
    }
    let output_size: i32 = result_bytes.len() as i32;
    set_data(output_size, out_buff_ptr, result_bytes.to_vec());

    output_size
}

pub fn ec_mul_helper_bn254(
    _env: wasm_exec_env_t,
    p1_ptr: *mut u8,
    p1_len: usize,
    scalar_ptr: *mut u8,
    scalar_len: usize,
    out_buff_ptr: *mut u8,
    out_buff_len: usize,
) -> i32 {
    const ERROR: i32 = -98;

    if p1_ptr.is_null() || p1_len != G1_LEN 
        || scalar_ptr.is_null() || scalar_len != SCALAR_LEN  
        || out_buff_ptr.is_null() || out_buff_len < G1_LEN 
    {
        return ERROR;
    }
    
    let mut p_slice = get_data(p1_ptr, p1_len);
    let mut s_slice = get_data(scalar_ptr, scalar_len);

    // To little-endian
    let (p_x_slice, p_y_slice) = p_slice.split_at_mut(G1_LEN/2);
    p_x_slice.reverse();
    p_y_slice.reverse();
    s_slice.reverse();

    let p1_x = Fq::deserialize_uncompressed(&*p_x_slice).expect("Invalid BN254 field element");
    let p1_y = Fq::deserialize_uncompressed(&*p_y_slice).expect("Invalid BN254 field element");
    let p = G1Affine::new_unchecked(p1_x, p1_y);
    // Can directly use from_be_bytes_mod_order without reversing s_slice first
    let s = Fr::from_le_bytes_mod_order(&*s_slice);

    let result = p.mul_bigint(s.into_bigint()).into_affine();

    let mut result_bytes = [0u8; G1_LEN];
    let Some((x, y)) = result.xy() else {
        return ERROR; 
    };
    // To big-endian
    let mut x_bytes = [0u8; FQ_LEN];
    x.serialize_uncompressed(&mut x_bytes[..]).expect("serialize x");
    x_bytes.reverse(); 
    let mut y_bytes = [0u8; FQ_LEN];
    y.serialize_uncompressed(&mut y_bytes[..]).expect("serialize y");
    y_bytes.reverse(); 

    result_bytes[0..FQ_LEN].copy_from_slice(&x_bytes);
    result_bytes[FQ_LEN..(2 * FQ_LEN)].copy_from_slice(&y_bytes);

    if result_bytes.len() > out_buff_len {
        return ERROR;
    }

    let output_size: i32 = result_bytes.len() as i32;
    set_data(output_size, out_buff_ptr, result_bytes.to_vec());

    output_size
}

// ec_pairing for bn254
pub fn ec_pairing_check_helper_bn254(
    _env: wasm_exec_env_t,
    pair_ptr: *mut u8, 
    pair_len: usize
) -> i32 {
    const PAIR_SIZE: usize = G1_LEN + G2_LEN;
    const ERROR: i32 = -97;

    if pair_ptr.is_null() || pair_len % PAIR_SIZE != 0 {
        return ERROR;
    }

    let mut bytes = get_data(pair_ptr, pair_len);
    println!("{:?}", bytes.len());
    let mut g1_points = Vec::new();
    let mut g2_points = Vec::new();

    for chunk in bytes.chunks_mut(PAIR_SIZE) {
        let (g1_bytes, g2_bytes) = chunk.split_at_mut(G1_LEN);

        // To little-endian
        let (g1_x_slice, g1_y_slice) = g1_bytes.split_at_mut(G1_LEN/2);
        g1_x_slice.reverse();
        g1_y_slice.reverse();
        let g1_x = Fq::deserialize_uncompressed(&*g1_x_slice).expect("Invalid BN254 field element");
        let g1_y = Fq::deserialize_uncompressed(&*g1_y_slice).expect("Invalid BN254 field element");
        let g1 = G1Affine::new_unchecked(g1_x, g1_y);
        
        // To little-endian
        // For each coordinate, c1 is encoded before c0
        let (g2_x_c1_slice, rest) = g2_bytes.split_at_mut(G2_LEN/4);
        let (g2_x_c0_slice, rest) = rest.split_at_mut(G2_LEN/4);
        let (g2_y_c1_slice, g2_y_c0_slice) = rest.split_at_mut(G2_LEN/4);
        g2_x_c0_slice.reverse();
        g2_x_c1_slice.reverse();
        g2_y_c0_slice.reverse();
        g2_y_c1_slice.reverse();
        let g2_x_c0 = Fq::deserialize_uncompressed(&*g2_x_c0_slice).expect("Invalid BN254 field element");
        let g2_x_c1 = Fq::deserialize_uncompressed(&*g2_x_c1_slice).expect("Invalid BN254 field element");
        let g2_y_c0 = Fq::deserialize_uncompressed(&*g2_y_c0_slice).expect("Invalid BN254 field element");
        let g2_y_c1 = Fq::deserialize_uncompressed(&*g2_y_c1_slice).expect("Invalid BN254 field element");

        let g2_x = Fq2::new(g2_x_c0, g2_x_c1);
        let g2_y = Fq2::new(g2_y_c0, g2_y_c1);
        let g2 = G2Affine::new_unchecked(g2_x, g2_y);

        g1_points.push(g1);
        g2_points.push(g2);
    }

    let result = Bn254::multi_pairing(g1_points, g2_points);
    if result.0.is_one() {
        return 1
    }
    -1
}

pub fn ec_negation_helper_bn254(
    _env: wasm_exec_env_t,
    p_ptr: *mut u8,
    p_len: usize,
    out_buff_ptr: *mut u8,
    out_buff_len: usize, 
) -> i32 {
    const ERROR: i32 = -97;

    if p_ptr.is_null() || p_len != G1_LEN {
        return ERROR;
    }
    let mut input_slice = get_data(p_ptr, p_len);

    // To little-endian
    let (p_x_slice, p_y_slice) = input_slice.split_at_mut(G1_LEN/2);
    p_x_slice.reverse();
    p_y_slice.reverse();

    let p_x = Fq::deserialize_uncompressed(&*p_x_slice).expect("Invalid BN254 field element");
    let p_y = Fq::deserialize_uncompressed(&*p_y_slice).expect("Invalid BN254 field element");
    let p = G1Affine::new_unchecked(p_x, p_y);
    let neg_point = -p;

    let mut result_bytes = [0u8; G1_LEN];
    let Some((x, y)) = neg_point.xy() else {
        return ERROR; 
    };
    // to big-endian
    let mut x_bytes = [0u8; FQ_LEN];
    x.serialize_uncompressed(&mut x_bytes[..]).expect("serialize x");
    x_bytes.reverse(); 
    let mut y_bytes = [0u8; FQ_LEN];
    y.serialize_uncompressed(&mut y_bytes[..]).expect("serialize y");
    y_bytes.reverse(); 

    result_bytes[0..FQ_LEN].copy_from_slice(&x_bytes);
    result_bytes[FQ_LEN..(2 * FQ_LEN)].copy_from_slice(&y_bytes);

    let output_size: i32 = result_bytes.len() as i32;
    set_data(output_size, out_buff_ptr, result_bytes.to_vec());

    output_size
}

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

    let number: u64 = number as u64;
    debug!(
        "trace() params: msg_read_ptr={:?} msg_read_len={} number={} ",
        msg_read_ptr, msg_read_len, number
    );
    let Some(message) = read_utf8_from_wasm(msg_read_ptr, msg_read_len) else {
        return HostError::DecodingError as i32;
    };

    println!("WASM TRACE: {message} {number}");
    0
}
