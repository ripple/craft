use crate::core::types::account_id::AccountID;
use crate::core::types::keylets::account_keylet;
use crate::host::Error;
use crate::host::{Result, Result::Err, Result::Ok};
use crate::types::{ContractData, NFT, XRPL_CONTRACT_DATA_SIZE};
use crate::{host, sfield};
use crate::core::error_codes::match_result_code;
use crate::locator::LocatorPacker;

// TODO: Adapt this to support STAmount for all token types.
// TODO: Add documentation and examples.
pub fn get_account_balance(account_id: &AccountID) -> Result<Option<u64>> {
    // Construct the account keylet. This calls a host function, so propagate the error via `?`
    let account_keylet = match account_keylet(account_id) {
        Ok(keylet) => keylet,
        Err(e) => return Err(e),
    };

    // Try to cache the ledger object inside of rippled
    let slot = unsafe { host::cache_ledger_obj(account_keylet.as_ptr(), account_keylet.len(), 0) };
    if slot <= 0 {
        return Ok(None);
    }

    // Get the balance.
    let mut balance = 0u64;
    let result_code = unsafe {
        host::get_ledger_obj_field(
            slot,
            sfield::Balance,
            (&mut balance) as *mut u64 as *mut u8,
            8,
        )
    };

    match result_code {
        8 => Ok(Some(balance)),
        code if code < 0 => Err(Error::from_code(code)),
        _ => Err(Error::InternalError), // <-- Used for an unexpected result.
    }
}

// TODO: Add documentation and examples.
pub fn get_nft(owner: &AccountID, nft: &NFT) -> Result<Option<ContractData>> {
    let mut data: ContractData = [0; XRPL_CONTRACT_DATA_SIZE];
    let result_code = unsafe {
        host::get_NFT(
            owner.0.as_ptr(),
            owner.0.len(),
            nft.as_ptr(),
            nft.len(),
            data.as_mut_ptr(),
            data.len(),
        )
    };

    match result_code {
        code if code > 0 => Ok(Some(data)),
        code => {
            // let _ = trace_num("get_nft error", i64::from(code));
            Err(Error::from_code(code))
        }
    }
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