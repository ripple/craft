use crate::core::ledger_objects::traits::{EscrowFields, LedgerObjectCommonFields};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct Escrow;

impl LedgerObjectCommonFields for Escrow {}

impl EscrowFields for Escrow {}
