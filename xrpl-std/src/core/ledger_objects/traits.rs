use crate::core::amount::Amount;
use crate::core::amount::Amount::Xrp;
use crate::core::amount::xrp_amount::XrpAmount;
use crate::core::error_codes::{
    match_result_code_with_expected_bytes, match_result_code_with_expected_bytes_optional,
};
use crate::core::ledger_objects::{current_ledger_object, ledger_object};
use crate::core::types::account_id::AccountID;
use crate::core::types::blob::Blob;
use crate::core::types::contract_data::{ContractData, XRPL_CONTRACT_DATA_SIZE};
use crate::core::types::crypto_condition::Condition;
use crate::core::types::hash_256::Hash256;
use crate::host::{Error, get_current_ledger_obj_field, get_ledger_obj_field, update_data};
use crate::host::{Result, Result::Err, Result::Ok};
use crate::sfield;

pub trait LedgerObjectCommonFields {
    fn get_ledger_index(&self, register_num: i32) -> Result<Hash256> {
        ledger_object::get_hash_256_field(register_num, sfield::LedgerIndex)
    }
    fn get_flags(&self, register_num: i32) -> Result<u32> {
        ledger_object::get_u32_field(register_num, sfield::Flags)
    }

    // TODO: Add LedgerEntryType struct and implement this function.
    // fn get_ledger_entry_type(&self, register_num: i32) -> &LedgerEntryType;
}

pub trait CurrentLedgerObjectCommonFields {
    fn get_ledger_index(&self) -> Result<Hash256> {
        current_ledger_object::get_hash_256_field(sfield::LedgerIndex)
    }
    fn get_get_flags(&self) -> Result<u32> {
        current_ledger_object::get_u32_field(sfield::Flags)
    }

    // TODO: Add LedgerEntryType struct and implement this function.
    // get_fn get_ledger_entry_type(&self, register_num: i32) -> &LedgerEntryType;
}

pub trait CurrentEscrowFields: CurrentLedgerObjectCommonFields {
    /// The address of the owner (sender) of this escrow. This is the account that provided the XRP
    /// and gets it back if the escrow is canceled.
    fn get_account(&self) -> Result<AccountID> {
        current_ledger_object::get_account_id_field(sfield::Account)
    }

    /// The amount of XRP, in drops, currently held in the escrow.
    fn get_amount(&self) -> Result<Amount> {
        // TODO: Use get_amount_field from mod.rs
        let mut buffer = [0u8; 8]; // Enough to hold a u64

        let result_code = unsafe {
            get_current_ledger_obj_field(sfield::Amount, buffer.as_mut_ptr(), buffer.len())
        };

        match_result_code_with_expected_bytes(result_code, 8, || {
            let amount = i64::from_le_bytes(buffer);
            Xrp(XrpAmount(amount as u64))
        })
    }

    /// The escrow can be canceled if and only if this field is present and the time it specifies
    /// has passed. Specifically, this is specified as seconds since the Ripple Epoch and it
    /// "has passed" if it's earlier than the close time of the previous validated ledger.
    fn get_cancel_after(&self) -> Result<Option<u32>> {
        current_ledger_object::get_u32_field_optional(sfield::CancelAfter)
    }

    /// A PREIMAGE-SHA-256 crypto-condition, as hexadecimal. If present, the EscrowFinish
    /// transaction must contain a fulfillment that satisfies this condition.
    fn get_condition(&self) -> Result<Option<Condition>> {
        let mut buffer = [0u8; 32];

        let result_code = unsafe {
            get_current_ledger_obj_field(sfield::Condition, buffer.as_mut_ptr(), buffer.len())
        };

        match_result_code_with_expected_bytes_optional(result_code, 32, || Some(buffer.into()))
    }

    /// The destination address where the XRP is paid if the escrow is successful.
    fn get_destination(&self) -> Result<AccountID> {
        current_ledger_object::get_account_id_field(sfield::Destination)
    }

    /// A hint indicating which page of the destination's owner directory links to this object, in
    /// case the directory consists of multiple pages. Omitted on escrows created before enabling the fix1523 amendment.
    fn get_destination_node(&self) -> Result<Option<Hash256>> {
        current_ledger_object::get_hash_256_field_optional(sfield::DestinationNode)
    }

    /// An arbitrary tag to further specify the destination for this escrow, such as a hosted
    /// recipient at the destination address.
    fn get_destination_tag(&self) -> Result<Option<u32>> {
        current_ledger_object::get_u32_field_optional(sfield::DestinationTag)
    }

    /// The time, in seconds since the Ripple Epoch, after which this escrow can be finished. Any
    /// EscrowFinish transaction before this time fails. (Specifically, this is compared with the
    /// close time of the previous validated ledger.)
    fn get_finish_after(&self) -> Result<Option<u32>> {
        current_ledger_object::get_u32_field_optional(sfield::FinishAfter)
    }

    // TODO: Implement this function.
    /// The value 0x0075, mapped to the string Escrow, indicates that this is an Escrow entry.
    // fn get_ledger_entry_type(&self) -> Result<LedgerEntryType> {
    //     return Ok(LedgerEntryType::Escrow);
    // }

    /// A hint indicating which page of the sender's owner directory links to this entry, in case
    /// the directory consists of multiple pages.
    fn get_owner_node(&self) -> Result<Hash256> {
        current_ledger_object::get_hash_256_field(sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self) -> Result<Hash256> {
        current_ledger_object::get_hash_256_field(sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self) -> Result<u32> {
        current_ledger_object::get_u32_field(sfield::PreviousTxnLgrSeq)
    }

    /// An arbitrary tag to further specify the source for this escrow, such as a hosted recipient
    /// at the owner's address.
    fn get_source_tag(&self) -> Result<Option<u32>> {
        current_ledger_object::get_u32_field_optional(sfield::SourceTag)
    }

    /// The WASM code that is executing.
    fn get_finish_function(&self) -> Result<Option<Blob>> {
        current_ledger_object::get_blob_field_optional(sfield::FinishFunction)
    }

    // TODO: Redo docs.
    /// Retrieves the contract data from the current ledger object.
    fn get_data(&self) -> Result<ContractData> {
        let mut data: [u8; XRPL_CONTRACT_DATA_SIZE] = [0; XRPL_CONTRACT_DATA_SIZE];

        let result_code =
            unsafe { get_current_ledger_obj_field(sfield::Data, data.as_mut_ptr(), data.len()) };

        match result_code {
            code if code >= 0 => Ok(ContractData {
                data,
                len: code as usize,
            }),
            code => Err(Error::from_code(code)),
        }
    }

    /// Updates the contract data in the current escrow object.
    ///
    /// # Arguments
    ///
    /// * `data` - The contract data to update
    ///
    /// # Returns
    ///
    /// Returns a `Result<()>` where:
    /// * `Ok(())` - The data was successfully updated
    /// * `Err(Error)` - If the update operation failed
    fn get_update_current_escrow_data(data: ContractData) -> Result<()> {
        // TODO: Make sure Olek always deletes any existing data bytes in rippled, and sets the new
        // length to be `data.len` (e.g., if the developer writes 2 bytes, then that's the new
        // length and any old bytes are lost.
        let result_code = unsafe { update_data(data.data.as_ptr(), data.len) };
        match_result_code_with_expected_bytes(result_code, data.len, || ())
    }
}

pub trait EscrowFields: LedgerObjectCommonFields {
    /// The address of the owner (sender) of this escrow. This is the account that provided the XRP
    /// and gets it back if the escrow is canceled.
    fn get_account(&self, register_num: i32) -> Result<AccountID> {
        ledger_object::get_account_id_field(register_num, sfield::Account)
    }

    /// The amount of XRP, in drops, currently held in the escrow.
    fn get_amount(&self, register_num: i32) -> Result<Amount> {
        // TODO: Use get_amount_field from mod.rs
        let mut buffer = [0u8; 8]; // Enough to hold a u64

        let result_code = unsafe {
            get_ledger_obj_field(
                register_num,
                sfield::Amount,
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };

        match_result_code_with_expected_bytes(result_code, 8, || {
            let amount = i64::from_le_bytes(buffer);
            Xrp(XrpAmount(amount as u64))
        })
    }

    /// The escrow can be canceled if and only if this field is present and the time it specifies
    /// has passed. Specifically, this is specified as seconds since the Ripple Epoch and it
    /// "has passed" if it's earlier than the close time of the previous validated ledger.
    fn get_cancel_after(&self, register_num: i32) -> Result<Option<u32>> {
        ledger_object::get_u32_field_optional(register_num, sfield::CancelAfter)
    }

    /// A PREIMAGE-SHA-256 crypto-condition, as hexadecimal. If present, the EscrowFinish
    /// transaction must contain a fulfillment that satisfies this condition.
    fn get_condition(&self, register_num: i32) -> Result<Option<Condition>> {
        let mut buffer = [0u8; 32];

        let result_code = unsafe {
            get_ledger_obj_field(
                register_num,
                sfield::Condition,
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };

        match_result_code_with_expected_bytes_optional(result_code, 32, || Some(buffer.into()))
    }

    /// The destination address where the XRP is paid if the escrow is successful.
    fn get_destination(&self, register_num: i32) -> Result<AccountID> {
        ledger_object::get_account_id_field(register_num, sfield::Destination)
    }

    /// A hint indicating which page of the destination's owner directory links to this object, in
    /// case the directory consists of multiple pages. Omitted on escrows created before enabling the fix1523 amendment.
    fn get_destination_node(&self, register_num: i32) -> Result<Option<Hash256>> {
        ledger_object::get_hash_256_field_optional(register_num, sfield::DestinationNode)
    }

    /// An arbitrary tag to further specify the destination for this escrow, such as a hosted
    /// recipient at the destination address.
    fn get_destination_tag(&self, register_num: i32) -> Result<Option<u32>> {
        ledger_object::get_u32_field_optional(register_num, sfield::DestinationTag)
    }

    /// The time, in seconds since the Ripple Epoch, after which this escrow can be finished. Any
    /// EscrowFinish transaction before this time fails. (Specifically, this is compared with the
    /// close time of the previous validated ledger.)
    fn get_finish_after(&self, register_num: i32) -> Result<Option<u32>> {
        ledger_object::get_u32_field_optional(register_num, sfield::FinishAfter)
    }

    // TODO: Implement this function.
    /// The value 0x0075, mapped to the string Escrow, indicates that this is an Escrow entry.
    // fn get_ledger_entry_type(&self, register_num: i32) -> Result<LedgerEntryType> {
    //     return Ok(LedgerEntryType::Escrow);
    // }

    /// A hint indicating which page of the sender's owner directory links to this entry, in case
    /// the directory consists of multiple pages.
    fn get_owner_node(&self, register_num: i32) -> Result<Hash256> {
        ledger_object::get_hash_256_field(register_num, sfield::OwnerNode)
    }

    /// The identifying hash of the transaction that most recently modified this entry.
    fn get_previous_txn_id(&self, register_num: i32) -> Result<Hash256> {
        ledger_object::get_hash_256_field(register_num, sfield::PreviousTxnID)
    }

    /// The index of the ledger that contains the transaction that most recently modified this
    /// entry.
    fn get_previous_txn_lgr_seq(&self, register_num: i32) -> Result<u32> {
        ledger_object::get_u32_field(register_num, sfield::PreviousTxnLgrSeq)
    }

    /// An arbitrary tag to further specify the source for this escrow, such as a hosted recipient
    /// at the owner's address.
    fn get_source_tag(&self, register_num: i32) -> Result<Option<u32>> {
        ledger_object::get_u32_field_optional(register_num, sfield::SourceTag)
    }

    /// The WASM code that is executing.
    fn get_finish_function(&self, register_num: i32) -> Result<Option<Blob>> {
        ledger_object::get_blob_field_optional(register_num, sfield::FinishFunction)
    }

    // TODO: Redo docs.
    /// Retrieves the contract data from the current ledger object.
    fn get_data(&self, register_num: i32) -> Result<ContractData> {
        let mut data: [u8; XRPL_CONTRACT_DATA_SIZE] = [0; XRPL_CONTRACT_DATA_SIZE];

        let result_code = unsafe {
            get_ledger_obj_field(register_num, sfield::Data, data.as_mut_ptr(), data.len())
        };

        match result_code {
            code if code >= 0 => Ok(ContractData {
                data,
                len: code as usize,
            }),
            code => Err(Error::from_code(code)),
        }
    }
}
