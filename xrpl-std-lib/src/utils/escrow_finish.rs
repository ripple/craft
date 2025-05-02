use crate::core::amount::Amount;
use crate::core::amount::Amount::Xrp;
use crate::core::amount::xrp_amount::XrpAmount;
use crate::core::field_codes::{
    SF_ACCOUNT, SF_ACCOUNT_TXN_ID, SF_CONDITION, SF_FEE, SF_FLAGS, SF_FULFILLMENT, SF_LAST_LEDGER_SEQUENCE,
    SF_NETWORK_ID, SF_OFFER_SEQUENCE, SF_OWNER, SF_SEQUENCE, SF_SIGNING_PUB_KEY, SF_SOURCE_TAG, SF_TICKET_SEQUENCE,
    SF_TRANSACTION_TYPE, SF_TXN_SIGNATURE,
};
use crate::core::types::account_id::AccountID;
use crate::core::types::blob::Blob;
use crate::core::types::crypto_condition::{Condition, Fulfillment};
use crate::core::types::hash_256::Hash256;
use crate::core::types::public_key::PublicKey;
use crate::core::types::transaction_type::TransactionType;
use crate::host;
use crate::host::get_current_escrow_finish_field;
use crate::host::trace::trace_msg;

#[inline(always)]
pub fn get_tx_id() -> Hash256 {
    // 1. Allocate a buffer on the stack
    let buffer = [0u8; 32]; // Allocate memory to read into.

    // 2. Call the actual host function with a pointer to the above array.
    unsafe {
        host::get_tx_hash(buffer.as_ptr());
    }

    // 3. Return the transactionId as a Hash256.
    buffer.into()
}

#[inline(always)]
pub fn get_account() -> AccountID {
    get_account_id_field(SF_ACCOUNT)
}

#[inline(always)]
pub fn get_transaction_type() -> TransactionType {
    // 1. Allocate a buffer on the stack
    let buffer = [0u8; 2]; // Allocate memory to read into (this is an i32)

    // 2. Call the actual host function with a pointer to the above array.
    unsafe {
        get_current_escrow_finish_field(buffer.as_ptr(), buffer.len(), SF_TRANSACTION_TYPE);
    }

    i16::from_be_bytes(buffer).into()
}

#[inline(always)]
pub fn get_fee() -> Amount {
    // 1. Allocate a buffer on the stack
    let buffer = [0u8; 8]; // Enough to hold an u64

    // 2. Call the actual host function with a pointer to the above array.
    unsafe {
        get_current_escrow_finish_field(buffer.as_ptr(), buffer.len(), SF_FEE);
    }

    let amount = i64::from_be_bytes(buffer);
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
pub fn get_owner() -> AccountID {
    get_account_id_field(SF_OWNER)
}

#[inline(always)]
pub fn get_offer_sequence() -> u32 {
    get_u32_field(SF_OFFER_SEQUENCE)
}

#[inline(always)]
pub fn get_condition() -> Condition {
    // 1. Allocate a buffer on the stack
    let buffer = [0u8; 32]; // Enough to hold the largest field, which is a memo.

    // 2. Call the actual host function with a pointer to the above array.
    unsafe {
        get_current_escrow_finish_field(buffer.as_ptr(), buffer.len(), SF_CONDITION);
    }

    buffer.into()
}

// TODO: credential IDS
#[inline(always)]
pub fn get_fulfillment() -> Fulfillment {
    // 1. Allocate a buffer on the stack
    let buffer = [0u8; 256]; // Enough to hold the largest field, which is a memo.

    // 2. Call the actual host function with a pointer to the above array.
    unsafe {
        get_current_escrow_finish_field(buffer.as_ptr(), buffer.len(), SF_FULFILLMENT);
    }

    buffer.into()
}

#[inline(always)]
fn get_u32_field(field_code: i32) -> u32 {
    // 1. Allocate a buffer on the stack
    let buffer = [0u8; 4]; // Enough to hold an u32

    // 2. Call the actual host function with a pointer to the above array.
    unsafe {
        get_current_escrow_finish_field(buffer.as_ptr(), buffer.len(), field_code);
    }

    u32::from_be_bytes(buffer)
}

fn get_hash_256_field(field_code: i32) -> Hash256 {
    // 1. Allocate a buffer on the stack
    let buffer = [0u8; 32]; // Allocate memory to read into.

    // 2. Call the actual host function with a pointer to the above array.
    unsafe {
        get_current_escrow_finish_field(buffer.as_ptr(), buffer.len(), field_code);
    }

    // 3. Return the transactionId as a Hash256.
    buffer.into()
}

#[inline(always)]
fn get_public_key_field(field_code: i32) -> PublicKey {
    // 1. Allocate a buffer on the stack
    let buffer = [0u8; 33]; // Enough to hold the largest field, which is a memo.

    // 2. Call the actual host function with a pointer to the above array.
    unsafe {
        get_current_escrow_finish_field(buffer.as_ptr(), buffer.len(), field_code);
    }

    buffer.into()
}

#[inline(always)]
fn get_blob_field(field_code: i32) -> Blob {
    // 1. Allocate a buffer on the stack
    let buffer = [0u8; 1024]; // Enough to hold the largest field, which is a memo.

    // 2. Call the actual host function with a pointer to the above array.
    unsafe {
        let len = get_current_escrow_finish_field(buffer.as_ptr(), buffer.len(), field_code);
        Blob {
            data: buffer,
            len: len as usize,
        }
    }
}

#[inline(always)]
fn get_account_id_field(field_code: i32) -> AccountID {
    // Allocate a buffer
    let buffer = [0x00; 20]; // Allocate memory to read into.

    unsafe {
        // 2. Call the actual host function with a pointer to the above array.
        let result_code = get_current_escrow_finish_field(buffer.as_ptr(), buffer.len(), field_code);

        // 3. Check the result code from the host
        //    (This requires the host to return meaningful codes!)
        if result_code < 0 {
            // Assuming negative means error
            let _ = trace_msg("Host function get_current_escrow_finish_field failed!");
            // Handle error appropriately - maybe panic or return Err(...)
            panic!(
                "Failed to get AccountID for field_code={} from host. Error code: {}",
                field_code, result_code
            );
        }

        // Optional: check if bytes written matches expected, if host returns that
        let bytes_written = result_code as usize;
        assert_eq!(bytes_written, buffer.len());
    }

    // 4. Construct the AccountID from the buffer filled by the host
    let buffer_copy = buffer.clone();
    AccountID(buffer_copy) // Or AccountID::from_bytes(&buffer) etc.
}
