#![no_std]

use crate::host::trace::trace;

pub mod core;
pub mod host;
pub mod keylet;
pub mod locator;
pub mod sfield;
pub mod types;
use host::trace::trace_num;

use crate::keylet::account_keylet;
use crate::locator::LocatorPacker;
use crate::types::{AccountID, ContractData, NFT, XRPL_ACCOUNT_ID_SIZE, XRPL_CONTRACT_DATA_SIZE};

//TODO replace some of the helper functions with Objects, e.g. AccountRoot, Escrow, Tx

pub fn get_tx_account_id() -> Option<AccountID> {
    let mut account_id: AccountID = [0; XRPL_ACCOUNT_ID_SIZE];
    if unsafe { host::get_tx_field(sfield::Account, account_id.as_mut_ptr(), account_id.len()) } > 0
    {
        Some(account_id)
    } else {
        None
    }
}

pub fn get_current_escrow_account_id() -> Option<AccountID> {
    let mut account_id: AccountID = [0; XRPL_ACCOUNT_ID_SIZE];
    if unsafe {
        host::get_current_ledger_obj_field(
            sfield::Account,
            account_id.as_mut_ptr(),
            account_id.len(),
        )
    } > 0
    {
        Some(account_id)
    } else {
        None
    }
}

pub fn get_current_escrow_destination() -> Option<AccountID> {
    let mut account_id: AccountID = [0; XRPL_ACCOUNT_ID_SIZE];
    if unsafe {
        host::get_current_ledger_obj_field(
            sfield::Destination,
            account_id.as_mut_ptr(),
            account_id.len(),
        )
    } > 0
    {
        Some(account_id)
    } else {
        None
    }
}

pub fn get_current_escrow_data() -> Option<ContractData> {
    let mut data: ContractData = [0; XRPL_CONTRACT_DATA_SIZE];
    if unsafe { host::get_current_ledger_obj_field(sfield::Data, data.as_mut_ptr(), data.len()) }
        > 0
    {
        Some(data)
    } else {
        None
    }
}

pub fn get_current_escrow_finish_after() -> Option<i32> {
    let mut after = 0i32;
    if unsafe {
        host::get_current_ledger_obj_field(
            sfield::FinishAfter,
            (&mut after) as *mut i32 as *mut u8,
            4,
        )
    } > 0
    {
        Some(after)
    } else {
        None
    }
}

pub fn get_account_balance(aid: &AccountID) -> Option<u64> {
    let keylet = match account_keylet(aid) {
        None => return None,
        Some(keylet) => keylet,
    };
    // let _ = trace_data("std-lib keylet ", &keylet, DataRepr::AsHex);
    let slot = unsafe { host::cache_ledger_obj(keylet.as_ptr(), keylet.len(), 0) };
    if slot <= 0 {
        return None;
    }
    // let _ = trace("std-lib slot ");
    let mut balance = 0u64;
    let result_code;
    unsafe {
        result_code = host::get_ledger_obj_field(
            slot,
            sfield::Balance,
            (&mut balance) as *mut u64 as *mut u8,
            8,
        );
    }

    if result_code == 8 {
        Some(balance)
    } else {
        let _ = trace("Host function get_current_escrow_finish_field failed!");
        panic!(
            "Failed to get Account Balance for field_code={} from host. Error code: {}",
            sfield::Balance,
            result_code
        );
    }
}

pub fn get_nft(owner: &AccountID, nft: &NFT) -> Option<ContractData> {
    let mut data: ContractData = [0; XRPL_CONTRACT_DATA_SIZE];
    unsafe {
        let retcode = host::get_NFT(
            owner.as_ptr(),
            owner.len(),
            nft.as_ptr(),
            nft.len(),
            data.as_mut_ptr(),
            data.len(),
        );
        if retcode > 0 {
            Some(data)
        } else {
            let _ = trace_num("get_nft error", i64::from(retcode));
            None
        }
    }
}

pub fn get_ledger_obj_nested_field(slot: i32, locator: &LocatorPacker) -> Option<ContractData> {
    let mut data: ContractData = [0; XRPL_CONTRACT_DATA_SIZE];
    unsafe {
        let retcode = host::get_ledger_obj_nested_field(
            slot,
            locator.get_addr(),
            locator.num_packed_bytes(),
            data.as_mut_ptr(),
            data.len(),
        );
        if retcode > 0 {
            Some(data)
        } else {
            let _ = trace_num("get_ledger_obj_nested_field error", i64::from(retcode));
            None
        }
    }
}

pub fn update_current_escrow_data(data: ContractData) {
    unsafe {
        host::update_data(data.as_ptr(), data.len());
    }
}

/// This function is called on panic, but only in the WASM architecture. In non-WASM (e.g., in the
/// Host Simulator) the standard lib is available, which includes a panic handler.
#[cfg(target_arch = "wasm32")]
#[panic_handler]
fn panic(_info: &::core::panic::PanicInfo) -> ! {
    // This instruction will halt execution of the WASM module.
    // It's the WASM equivalent of a trap or an unrecoverable error.
    ::core::arch::wasm32::unreachable();
}
