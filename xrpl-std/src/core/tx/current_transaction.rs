use crate::core::amount::Amount;
use crate::core::amount::Amount::Xrp;
use crate::core::amount::xrp_amount::XrpAmount;
use crate::core::field_codes::{
    SF_ACCOUNT, SF_ACCOUNT_TXN_ID, SF_CONDITION, SF_FEE, SF_FLAGS, SF_FULFILLMENT, SF_HASH,
    SF_LAST_LEDGER_SEQUENCE, SF_NETWORK_ID, SF_OFFER_SEQUENCE, SF_OWNER, SF_SEQUENCE,
    SF_SIGNING_PUB_KEY, SF_SOURCE_TAG, SF_TICKET_SEQUENCE, SF_TRANSACTION_TYPE, SF_TXN_SIGNATURE,
};
use crate::core::tx::transaction_helpers::{
    get_account_id_field, get_blob_field, get_hash_256_field, get_public_key_field, get_u32_field,
};
use crate::core::types::account_id::AccountID;
use crate::core::types::blob::Blob;
use crate::core::types::crypto_condition::{Condition, Fulfillment};
use crate::core::types::hash_256::Hash256;
use crate::core::types::public_key::PublicKey;
use crate::core::types::transaction_type::TransactionType;
use crate::host::get_tx_field;
use crate::sfield;

#[inline(always)]
pub fn get_account() -> AccountID {
    get_account_id_field(SF_ACCOUNT)
}

#[inline(always)]
pub fn get_id() -> Hash256 {
    let mut buffer = [0u8; 32]; // Allocate memory to read into (this is an i32)

    unsafe {
        get_tx_field(SF_HASH, buffer.as_mut_ptr(), buffer.len());
    }

    buffer.into()
}

#[inline(always)]
pub fn get_transaction_type() -> TransactionType {
    let mut buffer = [0u8; 2]; // Allocate memory to read into (this is an i32)

    unsafe {
        get_tx_field(SF_TRANSACTION_TYPE, buffer.as_mut_ptr(), buffer.len());
    }

    i16::from_le_bytes(buffer).into()
}

#[inline(always)]
pub fn get_computation_allowance() -> u32 {
    get_u32_field(sfield::ComputationAllowance)
}

#[inline(always)]
pub fn get_fee() -> Amount {
    let mut buffer = [0u8; 8]; // Enough to hold a u64

    unsafe {
        get_tx_field(SF_FEE, buffer.as_mut_ptr(), buffer.len());
    }

    let amount = i64::from_le_bytes(buffer);
    Xrp(XrpAmount(amount as u64))
}

#[inline(always)]
pub fn get_sequence() -> u32 {
    get_u32_field(SF_SEQUENCE)
}

#[inline(always)]
pub fn get_account_txn_id() -> Hash256 {
    get_hash_256_field(SF_ACCOUNT_TXN_ID)
}

#[inline(always)]
pub fn get_flags() -> u32 {
    get_u32_field(SF_FLAGS)
}

#[inline(always)]
pub fn get_last_ledger_sequence() -> u32 {
    get_u32_field(SF_LAST_LEDGER_SEQUENCE)
}

#[inline(always)]
pub fn get_network_id() -> u32 {
    get_u32_field(SF_NETWORK_ID)
}

#[inline(always)]
pub fn get_source_tag() -> u32 {
    get_u32_field(SF_SOURCE_TAG)
}

#[inline(always)]
pub fn get_signing_pub_key() -> PublicKey {
    get_public_key_field(SF_SIGNING_PUB_KEY)
}

#[inline(always)]
pub fn get_ticket_sequence() -> u32 {
    get_u32_field(SF_TICKET_SEQUENCE)
}

#[inline(always)]
pub fn get_txn_signature() -> Blob {
    get_blob_field(SF_TXN_SIGNATURE)
}

#[inline(always)]
pub fn get_owner() -> AccountID {
    get_account_id_field(SF_OWNER)
}

#[inline(always)]
pub fn get_offer_sequence() -> u32 {
    get_u32_field(SF_OFFER_SEQUENCE)
}

#[inline(always)]
pub fn get_condition() -> Condition {
    let mut buffer = [0u8; 32];

    unsafe {
        get_tx_field(SF_CONDITION, buffer.as_mut_ptr(), buffer.len());
    }

    buffer.into()
}

#[inline(always)]
pub fn get_fulfillment() -> Fulfillment {
    let blob = get_blob_field(SF_FULFILLMENT);

    let mut data = [0u8; 256];
    let len_to_copy = blob.len;
    if len_to_copy > 0 {
        data[0..len_to_copy].copy_from_slice(&blob.data[0..len_to_copy]);
    }

    Fulfillment {
        data,
        len: blob.len,
    }
}

// TODO: credential IDS
