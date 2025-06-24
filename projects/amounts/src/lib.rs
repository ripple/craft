#![no_std]

use xrpl_std::core::ledger_objects::current_escrow::get_current_escrow;

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> bool {
    let _current_escrow = get_current_escrow();
    true
}
