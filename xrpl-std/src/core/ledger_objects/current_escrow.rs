use crate::core::ledger_objects::traits::{CurrentEscrowFields, CurrentLedgerObjectCommonFields};

#[derive(Debug, Eq, PartialEq)]
pub struct CurrentEscrow;

impl CurrentLedgerObjectCommonFields for CurrentEscrow {}

impl CurrentEscrowFields for CurrentEscrow {}

#[inline]
pub fn get_current_escrow() -> CurrentEscrow {
    CurrentEscrow
}
