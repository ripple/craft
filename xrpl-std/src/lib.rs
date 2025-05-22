#![no_std]
use crate::core::types::account_id::AccountID;
use crate::core::types::contract_data::ContractData;

pub mod core;
pub mod host;
pub mod sfield;
// use keylet hash only (i.e. without 2-byte LedgerEntryType) for now.
// TODO Check rippled
pub const XRPL_KEYLET_SIZE: usize = 32;

pub type Keylet = [u8; XRPL_KEYLET_SIZE];

pub fn get_account_balance(aid: &AccountID) -> Option<u64> {
    let keylet = match account_keylet(aid) {
        None => return None,
        Some(keylet) => keylet,
    };
    // println!("std-lib keylet {:?}", keylet);
    let slot = unsafe { host::ledger_slot_set(keylet.as_ptr(), keylet.len(), 0) };
    if slot <= 0 {
        return None;
    }
    // println!("std-lib slot {:?}", slot);
    let mut balance = 0u64;
    if unsafe {
        host::get_ledger_obj_field(
            slot,
            sfield::Balance,
            (&mut balance) as *mut u64 as *mut u8,
            8,
        )
    } == 8
    {
        Some(balance)
    } else {
        None
    }
}

pub fn account_keylet(aid: &AccountID) -> Option<Keylet> {
    let mut key_let: Keylet = [0; XRPL_KEYLET_SIZE];
    if unsafe {
        host::account_keylet(
            aid.0.as_ptr(),
            aid.0.len(),
            key_let.as_mut_ptr(),
            key_let.len(),
        )
    } > 0
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
