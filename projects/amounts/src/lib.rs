#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_std::core::ledger_objects::current_escrow::{CurrentEscrow, get_current_escrow};
use xrpl_std::core::ledger_objects::traits::CurrentEscrowFields;
use xrpl_std::core::types::amount::amount::Amount;
use xrpl_std::host::trace::trace_amount;

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> bool {
    let current_escrow: CurrentEscrow = get_current_escrow();
    let token_amount = current_escrow.get_amount().unwrap();
    let amount: Amount = token_amount.into();

    let _ = trace_amount("Amount", &amount);
    true
}
