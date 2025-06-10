use crate::core::error_codes::{match_result_code, match_result_code_with_expected_bytes};
use crate::core::types::account_id::{ACCOUNT_ID_SIZE, AccountID};
use crate::core::types::contract_data::{ContractData, XRPL_CONTRACT_DATA_SIZE};
use crate::host::Result;
use crate::locator::LocatorPacker;
use crate::{host, sfield};

/// Retrieves the destination AccountID from the current ledger object.
///
/// # Returns
///
/// Returns a `Result<AccountID>` where:
/// * `Ok(AccountID)` - The destination account identifier
/// * `Err(Error)` - If the field cannot be retrieved or doesn't exist
///
/// # Note
/// This function may be made optional in the future to accommodate ledger objects that don't have
/// a destination field (TODO).
pub fn get_destination() -> Result<AccountID> {
    get_account_id_field(sfield::Destination)
}

/// Retrieves the destination AccountID from the current ledger object, panicking on failure.
///
/// # Returns
///
/// Returns the destination `AccountID`.
///
/// # Panics
///
/// Panics if the destination field cannot be retrieved from the host.
pub fn get_destination_safe() -> AccountID {
    get_account_id_field_safe(sfield::Destination)
}

/// Retrieves the AccountID from the current ledger object.
///
/// # Returns
///
/// Returns a `Result<AccountID>` where:
/// * `Ok(AccountID)` - The account identifier
/// * `Err(Error)` - If the field cannot be retrieved or doesn't exist
pub fn get_account_id() -> Result<AccountID> {
    get_account_id_field(sfield::Account)
}

/// Retrieves the account AccountID from the current ledger object, panicking on failure.
///
/// # Returns
///
/// Returns the account `AccountID`.
///
/// # Panics
///
/// Panics if the account field cannot be retrieved from the host.
pub fn get_account_id_safe() -> AccountID {
    get_account_id_field_safe(sfield::Account)
}

/// Retrieves the contract data from the current ledger object.
///
/// # Returns
///
/// Returns a `Result<Option<ContractData>>` where:
/// * `Ok(Some(ContractData))` - The contract data if present
/// * `Ok(None)` - If no contract data is available
/// * `Err(Error)` - If an error occurs while retrieving the data
pub fn get_data() -> Result<Option<ContractData>> {
    let mut data: ContractData = [0; XRPL_CONTRACT_DATA_SIZE];
    let result_code =
        unsafe { host::get_current_ledger_obj_field(sfield::Data, data.as_mut_ptr(), data.len()) };

    match_result_code_with_expected_bytes(result_code, data.len(), || Some(data))
}

/// Retrieves the contract data from the current ledger object, panicking on failure.
///
/// # Returns
///
/// Returns `Option<ContractData>` where:
/// * `Some(ContractData)` - The contract data if present
/// * `None` - If no contract data is available
///
/// # Panics
///
/// Panics if an error occurs while retrieving the data from the host.
pub fn get_data_safe() -> Option<ContractData> {
    get_data().unwrap_or_panic_traced("current_ledger_object::get_data_safe")
}

/// Retrieves the FinishAfter value from the current ledger object.
///
/// # Returns
///
/// Returns a `Result<Option<i32>>` where:
/// * `Ok(Some(i32))` - The FinishAfter timestamp if present
/// * `Ok(None)` - If no FinishAfter value is set
/// * `Err(Error)` - If an error occurs while retrieving the value
pub fn get_finish_after() -> Result<Option<i32>> {
    let mut after = 0i32;

    let result_code = unsafe {
        host::get_current_ledger_obj_field(
            sfield::FinishAfter,
            (&mut after) as *mut i32 as *mut u8,
            4,
        )
    };

    match_result_code_with_expected_bytes(result_code, 16, || Some(after))
}

/// Retrieves the FinishAfter value from the current ledger object, panicking on failure.
///
/// # Returns
///
/// Returns `Option<i32>` where:
/// * `Some(i32)` - The FinishAfter timestamp if present
/// * `None` - If no FinishAfter value is set
///
/// # Panics
///
/// Panics if an error occurs while retrieving the value from the host.
pub fn get_finish_after_safe() -> Option<i32> {
    get_finish_after().unwrap_or_panic_traced("current_ledger_object::get_finish_after_safe")
}

/// Retrieves an AccountID field from the current ledger object.
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
#[inline(always)]
fn get_account_id_field(field_code: i32) -> Result<AccountID> {
    let mut buffer = [0x00; ACCOUNT_ID_SIZE];

    let result_code = unsafe {
        host::get_current_ledger_obj_field(field_code, buffer.as_mut_ptr(), buffer.len())
    };

    match_result_code_with_expected_bytes(result_code, buffer.len(), || buffer.into())
}

/// Retrieves an AccountID field from the current ledger object, panicking on failure.
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
#[inline(always)]
fn get_account_id_field_safe(field_code: i32) -> AccountID {
    get_account_id_field(field_code)
        .unwrap_or_panic_traced("current_ledger_object::get_account_id_field_safe")
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
pub fn update_current_escrow_data(data: ContractData) -> Result<()> {
    let result_code = unsafe { host::update_data(data.as_ptr(), data.len()) };

    match_result_code_with_expected_bytes(result_code, data.len(), || ())
}

/// Retrieves contract data from a nested field in a ledger object.
///
/// # Arguments
///
/// * `slot` - The slot number of the ledger object
/// * `locator` - The locator that specifies the path to the nested field
///
/// # Returns
///
/// Returns a `Result<Option<ContractData>>` where:
/// * `Ok(Some(ContractData))` - The nested field data if present
/// * `Ok(None)` - If the nested field doesn't exist or is empty
/// * `Err(Error)` - If an error occurs while retrieving the data
pub fn get_contract_data(slot: i32, locator: &LocatorPacker) -> Result<Option<ContractData>> {
    let mut data: ContractData = [0; XRPL_CONTRACT_DATA_SIZE];

    let result_code = unsafe {
        host::get_ledger_obj_nested_field(
            slot,
            locator.get_addr(),
            locator.num_packed_bytes(),
            data.as_mut_ptr(),
            data.len(),
        )
    };

    match_result_code(result_code, || Some(data))
}

/// Retrieves contract data from a nested field in a ledger object.
///
/// # Arguments
///
/// * `slot` - The slot number of the ledger object
/// * `locator` - The locator that specifies the path to the nested field
///
/// # Returns
///
/// Returns a `Result<Option<ContractData>>` where:
/// * `Ok(Some(ContractData))` - The nested field data if present
/// * `Ok(None)` - If the nested field doesn't exist or is empty
/// * `Err(Error)` - If an error occurs while retrieving the data
pub fn get_nested_field(slot: i32, locator: &LocatorPacker) -> Result<Option<ContractData>> {
    let mut data: ContractData = [0; XRPL_CONTRACT_DATA_SIZE];

    let result_code = unsafe {
        host::get_ledger_obj_nested_field(
            slot,
            locator.get_addr(),
            locator.num_packed_bytes(),
            data.as_mut_ptr(),
            data.len(),
        )
    };

    match_result_code(result_code, || Some(data))
}
