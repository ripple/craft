use crate::core::types::account_id::AccountID;
use crate::core::types::contract_data::{ContractData, XRPL_CONTRACT_DATA_SIZE};
<<<<<<< HEAD
use crate::{host, sfield};
use crate::host::trace::trace;
=======
use crate::host::trace::trace;
use crate::{host, sfield};
>>>>>>> origin/main

pub fn get_current_escrow_destination() -> AccountID {
    get_account_id_field(sfield::Destination)
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

#[inline(always)]
fn get_account_id_field(field_code: i32) -> AccountID {
    let mut buffer = [0x00; 20];

    unsafe {
        let result_code =
            host::get_current_ledger_obj_field(field_code, buffer.as_mut_ptr(), buffer.len());

        if result_code < 0 {
            let _ = trace("Host function get_current_escrow_finish_field failed!");
            core::panic!(
                "Failed to get AccountID for field_code={} from host. Error code: {}",
                field_code,
                result_code
            );
        }

        let bytes_written = result_code as usize;
        assert_eq!(bytes_written, buffer.len());
    }

    buffer.into() // <-- Move the buffer into an AccountID
}
