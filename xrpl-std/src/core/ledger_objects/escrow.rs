use crate::core::ledger_objects::traits::{EscrowFields, LedgerObjectCommonFields};

#[derive(Debug, Eq, PartialEq)]
pub struct Escrow {
    slot_num: i32,
}

impl LedgerObjectCommonFields for Escrow {
    fn get_slot_num(&self) -> i32 {
        self.slot_num
    }
}

impl EscrowFields for Escrow {}
