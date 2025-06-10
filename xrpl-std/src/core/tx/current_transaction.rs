use crate::core::amount::Amount;
use crate::core::amount::Amount::Xrp;
use crate::core::amount::xrp_amount::XrpAmount;
use crate::core::error_codes::{match_result_code, match_result_code_with_expected_bytes};
use crate::core::field_codes::{
    SF_ACCOUNT, SF_ACCOUNT_TXN_ID, SF_CONDITION, SF_FEE, SF_FLAGS, SF_FULFILLMENT, SF_HASH,
    SF_LAST_LEDGER_SEQUENCE, SF_NETWORK_ID, SF_OFFER_SEQUENCE, SF_OWNER, SF_SEQUENCE,
    SF_SIGNING_PUB_KEY, SF_SOURCE_TAG, SF_TICKET_SEQUENCE, SF_TRANSACTION_TYPE, SF_TXN_SIGNATURE,
};
use crate::core::types::account_id::{ACCOUNT_ID_SIZE, AccountID};
use crate::core::types::blob::Blob;
use crate::core::types::crypto_condition::{Condition, Fulfillment};
use crate::core::types::hash_256::{HASH256_SIZE, Hash256};
use crate::core::types::public_key::PublicKey;
use crate::core::types::transaction_type::TransactionType;
use crate::host::{Result, Result::Err, Result::Ok, get_tx_field};
use crate::sfield;

/// Retrieves the account field from the current transaction.
///
/// # Returns
///
/// Returns a `Result<AccountID>` where:
/// * `Ok(AccountID)` - The account identifier from the transaction
/// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
pub fn get_account() -> Result<AccountID> {
    get_account_id_field(SF_ACCOUNT)
}

/// Retrieves the account field from the current transaction, panicking on failure.
///
/// # Returns
///
/// Returns the transaction account `AccountID`.
///
/// # Panics
///
/// Panics if the account field cannot be retrieved from the host.
pub fn get_account_safe() -> AccountID {
    get_account_id_field_safe(SF_ACCOUNT)
}

/// Retrieves the transaction ID (hash) from the current transaction.
///
/// # Returns
///
/// Returns a `Result<Hash256>` where:
/// * `Ok(Hash256)` - The transaction hash identifier
/// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
pub fn get_id() -> Result<Hash256> {
    get_hash_256_field(SF_HASH)
}

/// Retrieves the transaction ID (hash) from the current transaction, panicking on failure.
///
/// # Returns
///
/// Returns the transaction `Hash256` identifier.
///
/// # Panics
///
/// Panics if the transaction ID field cannot be retrieved from the host.
pub fn get_id_safe() -> Hash256 {
    get_hash_256_field_safe(SF_HASH)
}

/// Retrieves the transaction type from the current transaction.
///
/// # Returns
///
/// Returns a `Result<TransactionType>` where:
/// * `Ok(TransactionType)` - The type of the transaction
/// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
pub fn get_transaction_type() -> Result<TransactionType> {
    let mut buffer = [0u8; 2]; // Allocate memory to read into (this is an i32)

    let result_code =
        unsafe { get_tx_field(SF_TRANSACTION_TYPE, buffer.as_mut_ptr(), buffer.len()) };

    match_result_code_with_expected_bytes(result_code, 2, || i16::from_le_bytes(buffer).into())
}

/// Retrieves the transaction type from the current transaction, panicking on failure.
///
/// # Returns
///
/// Returns the `TransactionType` of the transaction.
///
/// # Panics
///
/// Panics if the transaction type field cannot be retrieved from the host.
pub fn get_transaction_type_safe() -> TransactionType {
    get_transaction_type()
        .unwrap_or_panic_traced("current_ledger_object::get_transaction_type_safe")
}

/// Retrieves the computation allowance from the current transaction.
///
/// # Returns
///
/// Returns a `Result<u32>` where:
/// * `Ok(u32)` - The computation allowance value
/// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
pub fn get_computation_allowance() -> Result<u32> {
    get_u32_field(sfield::ComputationAllowance)
}

/// Retrieves the computation allowance from the current transaction, panicking on failure.
///
/// # Returns
///
/// Returns the computation allowance as `u32`.
///
/// # Panics
///
/// Panics if the computation allowance field cannot be retrieved from the host.
pub fn get_computation_allowance_safe() -> u32 {
    get_u32_field_safe(sfield::ComputationAllowance)
}

/// Retrieves the fee amount from the current transaction.
///
/// # Returns
///
/// Returns a `Result<Amount>` where:
/// * `Ok(Amount)` - The fee amount as an XRP amount
/// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
pub fn get_fee() -> Result<Amount> {
    let mut buffer = [0u8; 8]; // Enough to hold a u64

    let result_code = unsafe { get_tx_field(SF_FEE, buffer.as_mut_ptr(), buffer.len()) };

    match_result_code_with_expected_bytes(result_code, 8, || {
        let amount = i64::from_le_bytes(buffer);
        Xrp(XrpAmount(amount as u64))
    })
}

/// Retrieves the fee amount from the current transaction, panicking on failure.
///
/// # Returns
///
/// Returns the fee `Amount`.
///
/// # Panics
///
/// Panics if the fee field cannot be retrieved from the host.
pub fn get_fee_safe() -> Amount {
    get_fee().unwrap_or_panic_traced("current_ledger_object::get_fee_safe")
}

/// Retrieves the sequence number from the current transaction.
///
/// # Returns
///
/// Returns a `Result<u32>` where:
/// * `Ok(u32)` - The transaction sequence number
/// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
pub fn get_sequence() -> Result<u32> {
    get_u32_field(SF_SEQUENCE)
}

/// Retrieves the sequence number from the current transaction, panicking on failure.
///
/// # Returns
///
/// Returns the transaction sequence number as `u32`.
///
/// # Panics
///
/// Panics if the sequence field cannot be retrieved from the host.
pub fn get_sequence_safe() -> u32 {
    get_u32_field_safe(SF_SEQUENCE)
}

/// Retrieves the account transaction ID from the current transaction.
///
/// # Returns
///
/// Returns a `Result<Hash256>` where:
/// * `Ok(Hash256)` - The account transaction ID hash
/// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
pub fn get_account_txn_id() -> Result<Hash256> {
    get_hash_256_field(SF_ACCOUNT_TXN_ID)
}

/// Retrieves the account transaction ID from the current transaction, panicking on failure.
///
/// # Returns
///
/// Returns the account transaction ID `Hash256`.
///
/// # Panics
///
/// Panics if the account transaction ID field cannot be retrieved from the host.
pub fn get_account_txn_id_safe() -> Hash256 {
    get_hash_256_field_safe(SF_ACCOUNT_TXN_ID)
}

/// Retrieves the flags field from the current transaction.
///
/// # Returns
///
/// Returns a `Result<u32>` where:
/// * `Ok(u32)` - The transaction flags value
/// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
pub fn get_flags() -> Result<u32> {
    get_u32_field(SF_FLAGS)
}

/// Retrieves the flags field from the current transaction, panicking on failure.
///
/// # Returns
///
/// Returns the transaction flags as `u32`.
///
/// # Panics
///
/// Panics if the flags field cannot be retrieved from the host.
pub fn get_flags_safe() -> u32 {
    get_u32_field_safe(SF_FLAGS)
}

/// Retrieves the last ledger sequence from the current transaction.
///
/// # Returns
///
/// Returns a `Result<u32>` where:
/// * `Ok(u32)` - The last ledger sequence number
/// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
pub fn get_last_ledger_sequence() -> Result<u32> {
    get_u32_field(SF_LAST_LEDGER_SEQUENCE)
}

/// Retrieves the last ledger sequence from the current transaction, panicking on failure.
///
/// # Returns
///
/// Returns the last ledger sequence as `u32`.
///
/// # Panics
///
/// Panics if the last ledger sequence field cannot be retrieved from the host.
pub fn get_last_ledger_sequence_safe() -> u32 {
    get_u32_field_safe(SF_LAST_LEDGER_SEQUENCE)
}

/// Retrieves the network ID from the current transaction.
///
/// # Returns
///
/// Returns a `Result<u32>` where:
/// * `Ok(u32)` - The network ID value
/// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
pub fn get_network_id() -> Result<u32> {
    get_u32_field(SF_NETWORK_ID)
}
/// Retrieves the network ID from the current transaction, panicking on failure.
///
/// # Returns
///
/// Returns the network ID as `u32`.
///
/// # Panics
///
/// Panics if the network ID field cannot be retrieved from the host.
pub fn get_network_id_safe() -> u32 {
    get_u32_field_safe(SF_NETWORK_ID)
}

/// Retrieves the source tag from the current transaction.
///
/// # Returns
///
/// Returns a `Result<u32>` where:
/// * `Ok(u32)` - The source tag value
/// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
pub fn get_source_tag() -> Result<u32> {
    get_u32_field(SF_SOURCE_TAG)
}
/// Retrieves the source tag from the current transaction, panicking on failure.
///
/// # Returns
///
/// Returns the source tag as `u32`.
///
/// # Panics
///
/// Panics if the source tag field cannot be retrieved from the host.
pub fn get_source_tag_safe() -> u32 {
    get_u32_field_safe(SF_SOURCE_TAG)
}

/// Retrieves the signing public key from the current transaction.
///
/// # Returns
///
/// Returns a `Result<PublicKey>` where:
/// * `Ok(PublicKey)` - The signing public key
/// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
pub fn get_signing_pub_key() -> Result<PublicKey> {
    get_public_key_field(SF_SIGNING_PUB_KEY)
}

/// Retrieves the signing public key from the current transaction, panicking on failure.
///
/// # Returns
///
/// Returns the signing `PublicKey`.
///
/// # Panics
///
/// Panics if the signing public key field cannot be retrieved from the host.
pub fn get_signing_pub_key_safe() -> PublicKey {
    get_public_key_field_safe(SF_SIGNING_PUB_KEY)
}

/// Retrieves the ticket sequence from the current transaction.
///
/// # Returns
///
/// Returns a `Result<u32>` where:
/// * `Ok(u32)` - The ticket sequence number
/// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
pub fn get_ticket_sequence() -> Result<u32> {
    get_u32_field(SF_TICKET_SEQUENCE)
}

/// Retrieves the ticket sequence from the current transaction, panicking on failure.
///
/// # Returns
///
/// Returns the ticket sequence as `u32`.
///
/// # Panics
///
/// Panics if the ticket sequence field cannot be retrieved from the host.
pub fn get_ticket_sequence_safe() -> u32 {
    get_u32_field_safe(SF_TICKET_SEQUENCE)
}

/// Retrieves the transaction signature from the current transaction.
///
/// # Returns
///
/// Returns a `Result<Blob>` where:
/// * `Ok(Blob)` - The transaction signature as a blob
/// * `Err(Error)` - If the field cannot be retrieved
pub fn get_txn_signature() -> Result<Blob> {
    get_blob_field(SF_TXN_SIGNATURE)
}

/// Retrieves the transaction signature from the current transaction, panicking on failure.
///
/// # Returns
///
/// Returns the transaction signature `Blob`.
///
/// # Panics
///
/// Panics if the transaction signature field cannot be retrieved from the host.
pub fn get_txn_signature_safe() -> Blob {
    get_blob_field_safe(SF_TXN_SIGNATURE)
}

/// Retrieves the owner account ID from the current transaction.
///
/// # Returns
///
/// Returns a `Result<AccountID>` where:
/// * `Ok(AccountID)` - The owner account identifier
/// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
pub fn get_owner() -> Result<AccountID> {
    get_account_id_field(SF_OWNER)
}

/// Retrieves the owner account ID from the current transaction, panicking on failure.
///
/// # Returns
///
/// Returns the owner `AccountID`.
///
/// # Panics
///
/// Panics if the owner field cannot be retrieved from the host.
pub fn get_owner_safe() -> AccountID {
    get_account_id_field_safe(SF_OWNER)
}

/// Retrieves the offer sequence from the current transaction.
///
/// # Returns
///
/// Returns a `Result<u32>` where:
/// * `Ok(u32)` - The offer sequence number
/// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
pub fn get_offer_sequence() -> Result<u32> {
    get_u32_field(SF_OFFER_SEQUENCE)
}

/// Retrieves the offer sequence from the current transaction, panicking on failure.
///
/// # Returns
///
/// Returns the offer sequence as `u32`.
///
/// # Panics
///
/// Panics if the offer sequence field cannot be retrieved from the host.
pub fn get_offer_sequence_safe() -> u32 {
    get_u32_field_safe(SF_OFFER_SEQUENCE)
}

/// Retrieves the condition from the current transaction.
///
/// # Returns
///
/// Returns a `Result<Condition>` where:
/// * `Ok(Condition)` - The cryptographic condition
/// * `Err(Error)` - If the field cannot be retrieved or has unexpected s
pub fn get_condition() -> Result<Condition> {
    let mut buffer = [0u8; 32];

    let result_code = unsafe { get_tx_field(SF_CONDITION, buffer.as_mut_ptr(), buffer.len()) };

    match_result_code_with_expected_bytes(result_code, 32, || buffer.into())
}

/// Retrieves the condition from the current transaction, panicking on failure.
///
/// # Returns
///
/// Returns the cryptographic `Condition`.
///
/// # Panics
///
/// Panics if the condition field cannot be retrieved from the host.
pub fn get_condition_safe() -> Condition {
    get_condition().unwrap_or_panic_traced("current_ledger_object::get_condition_safe")
}

/// Retrieves the fulfillment from the current transaction.
///
/// # Returns
///
/// Returns a `Result<Fulfillment>` where:
/// * `Ok(Fulfillment)` - The cryptographic fulfillment with data and length
/// * `Err(Error)` - If the field cannot be retrieved
///
/// # Note
///
/// The fulfillment data is copied into a fixed-size buffer (256 bytes) and the
/// actual length is preserved in the returned `Fulfillment` structure.
pub fn get_fulfillment() -> Result<Fulfillment> {
    let blob = get_blob_field(SF_FULFILLMENT);

    match blob {
        Ok(blob) => {
            let mut data = [0u8; 256];
            let len_to_copy = blob.len;
            if len_to_copy > 0 {
                data[0..len_to_copy].copy_from_slice(&blob.data[0..len_to_copy]);
            }

            Ok(Fulfillment {
                data,
                len: blob.len,
            })
        }
        // If we get here, just return the error that came from get_blob_field.
        Err(error) => Err(error),
    }
}

/// Retrieves the fulfillment from the current transaction, panicking on failure.
///
/// # Returns
///
/// Returns the cryptographic `Fulfillment`.
///
/// # Panics
///
/// Panics if the fulfillment field cannot be retrieved from the host.
pub fn get_fulfillment_safe() -> Fulfillment {
    get_fulfillment().unwrap_or_panic_traced("current_ledger_object::get_fulfillment_safe")
}

// TODO: credential IDS

//////////////////////////////
// Current Transaction Helpers
//////////////////////////////

/// Retrieves a u32 field from the current transaction.
///
/// # Arguments
///
/// * `field_code` - The field code identifying which u32 field to retrieve
///
/// # Returns
///
/// Returns a `Result<u32>` where:
/// * `Ok(u32)` - The u32 value for the specified field
/// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
#[inline]
fn get_u32_field(field_code: i32) -> Result<u32> {
    let mut buffer = [0u8; 4]; // Enough to hold an u32

    let result_code = unsafe { get_tx_field(field_code, buffer.as_mut_ptr(), buffer.len()) };

    match_result_code_with_expected_bytes(result_code, 4, || {
        u32::from_le_bytes(buffer) // <-- Move the buffer into an AccountID
    })
}

/// Retrieves a u32 field from the current transaction, panicking on failure.
///
/// # Arguments
///
/// * `field_code` - The field code identifying which u32 field to retrieve
///
/// # Returns
///
/// Returns the `u32` value for the specified field.
///
/// # Panics
///
/// Panics if the field cannot be retrieved from the host.
#[inline]
fn get_u32_field_safe(field_code: i32) -> u32 {
    get_u32_field(field_code).unwrap_or_panic_traced("current_ledger_object::get_u32_field_safe")
}

/// Retrieves a Hash256 field from the current transaction.
///
/// # Arguments
///
/// * `field_code` - The field code identifying which Hash256 field to retrieve
///
/// # Returns
///
/// Returns a `Result<Hash256>` where:
/// * `Ok(Hash256)` - The 256-bit hash for the specified field
/// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
#[inline]
fn get_hash_256_field(field_code: i32) -> Result<Hash256> {
    let mut buffer = [0u8; HASH256_SIZE]; // Enough to hold 256 bits (32 bytes)

    let result_code = unsafe { get_tx_field(field_code, buffer.as_mut_ptr(), buffer.len()) };

    match_result_code_with_expected_bytes(result_code, HASH256_SIZE, || {
        buffer.into() // <-- Move the buffer into an Hash256
    })
}

/// Retrieves a Hash256 field from the current transaction, panicking on failure.
///
/// # Arguments
///
/// * `field_code` - The field code identifying which Hash256 field to retrieve
///
/// # Returns
///
/// Returns the `Hash256` for the specified field.
///
/// # Panics
///
/// Panics if the field cannot be retrieved from the host.
#[inline]
fn get_hash_256_field_safe(field_code: i32) -> Hash256 {
    get_hash_256_field(field_code)
        .unwrap_or_panic_traced("current_ledger_object::get_hash_256_field_safe")
}

/// Retrieves a PublicKey field from the current transaction.
///
/// # Arguments
///
/// * `field_code` - The field code identifying which PublicKey field to retrieve
///
/// # Returns
///
/// Returns a `Result<PublicKey>` where:
/// * `Ok(PublicKey)` - The public key for the specified field
/// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
#[inline]
fn get_public_key_field(field_code: i32) -> Result<PublicKey> {
    let mut buffer = [0u8; 33];

    let result_code = unsafe { get_tx_field(field_code, buffer.as_mut_ptr(), buffer.len()) };

    match_result_code_with_expected_bytes(result_code, 33, || buffer.into())
}

/// Retrieves a PublicKey field from the current transaction, panicking on failure.
///
/// # Arguments
///
/// * `field_code` - The field code identifying which PublicKey field to retrieve
///
/// # Returns
///
/// Returns the `PublicKey` for the specified field.
///
/// # Panics
///
/// Panics if the field cannot be retrieved from the host.
#[inline]
fn get_public_key_field_safe(field_code: i32) -> PublicKey {
    get_public_key_field(field_code)
        .unwrap_or_panic_traced("current_ledger_object::get_public_key_field_safe")
}

/// Retrieves a variable-length blob field from the current transaction.
///
/// # Arguments
///
/// * `field_code` - The field code identifying which blob field to retrieve
///
/// # Returns
///
/// Returns a `Result<Blob>` where:
/// * `Ok(Blob)` - The blob data with its actual length
/// * `Err(Error)` - If the field cannot be retrieved
///
/// # Note
///
/// Uses a 1024-byte buffer to accommodate the largest possible field (memo).
/// The actual length is returned in the `Blob` structure.
#[inline]
fn get_blob_field(field_code: i32) -> Result<Blob> {
    let mut buffer = [0u8; 1024]; // Enough to hold the largest field, which is a memo.

    let result_code = unsafe { get_tx_field(field_code, buffer.as_mut_ptr(), buffer.len()) };

    match_result_code(result_code, || Blob {
        data: buffer,
        len: result_code as usize,
    })
}

/// Retrieves a variable-length blob field from the current transaction, panicking on failure.
///
/// # Arguments
///
/// * `field_code` - The field code identifying which blob field to retrieve
///
/// # Returns
///
/// Returns the `Blob` for the specified field.
///
/// # Panics
///
/// Panics if the field cannot be retrieved from the host.
#[inline]
fn get_blob_field_safe(field_code: i32) -> Blob {
    get_blob_field(field_code).unwrap_or_panic_traced("current_ledger_object::get_blob_field_safe")
}

/// Retrieves an AccountID field from the current transaction.
///
/// # Arguments
///
/// * `field_code` - The field code identifying which AccountID field to retrieve
///
/// # Returns
///
/// Returns a `Result<AccountID>` where:
/// * `Ok(AccountID)` - The account identifier for the specified field
/// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
#[inline]
fn get_account_id_field(field_code: i32) -> Result<AccountID> {
    let mut buffer = [0x00; ACCOUNT_ID_SIZE];

    let result_code = unsafe { get_tx_field(field_code, buffer.as_mut_ptr(), buffer.len()) };

    match_result_code_with_expected_bytes(result_code, ACCOUNT_ID_SIZE, || buffer.into())
}

/// Retrieves an AccountID field from the current transaction, panicking on failure.
///
/// # Arguments
///
/// * `field_code` - The field code identifying which AccountID field to retrieve
///
/// # Returns
///
/// Returns the `AccountID` for the specified field.
///
/// # Panics
///
/// Panics if the field cannot be retrieved from the host.
#[inline]
fn get_account_id_field_safe(field_code: i32) -> AccountID {
    get_account_id_field(field_code)
        .unwrap_or_panic_traced("current_ledger_object::get_account_id_field_safe")
}
