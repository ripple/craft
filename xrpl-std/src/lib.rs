#![no_std]

use crate::host::trace::trace;

pub mod core;
pub mod host;
pub mod locator;
pub mod sfield;

pub const XRPL_ACCOUNT_ID_SIZE: usize = 20;
// use keylet hash only (i.e. without 2-byte LedgerEntryType) for now.
// TODO Check rippled
pub const XRPL_KEYLET_SIZE: usize = 32;
pub const XRPL_HASH256_SIZE: usize = 32;
pub const XRPL_CONTRACT_DATA_SIZE: usize = 4096; //TODO size??
pub type AccountID = [u8; XRPL_ACCOUNT_ID_SIZE];
pub type Keylet = [u8; XRPL_KEYLET_SIZE];
pub type Hash256 = [u8; XRPL_HASH256_SIZE];
pub type ContractData = [u8; XRPL_CONTRACT_DATA_SIZE];

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

pub fn account_keylet(aid: &AccountID) -> Option<Keylet> {
    let mut key_let: Keylet = [0; XRPL_KEYLET_SIZE];
    if unsafe { host::account_keylet(aid.as_ptr(), aid.len(), key_let.as_mut_ptr(), key_let.len()) }
        > 0
    {
        Some(key_let)
    } else {
        None
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
