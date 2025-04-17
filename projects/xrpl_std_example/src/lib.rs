use xrpl_std_lib::host;
use xrpl_std_lib::model::ledger_objects::Escrow;
use xrpl_std_lib::model::transactions::EscrowFinish;

/// This function is the low-level WASM entry point for Smart Escrows. It assumes:
/// 1. `escrow_ptr` is a valid pointer to a mutable `Escrow` struct instance
///    in the WASM module's linear memory.
/// 2. `finish_tx_ptr` is a valid pointer to a mutable `FinishTransaction` struct instance
///    in the WASM module's linear memory.
/// 3. The memory pointed to by these pointers is valid and properly aligned for
///    the respective struct types for the duration of this function call.
/// 4. The caller (WASM host or other WASM code) ensures that these pointers
///    originate from valid allocations of these Rust types.
///
/// The `*mut usize` type is often used in FFI/WASM as a way to pass an opaque pointer
/// (memory address) which needs to be cast back to the actual type.
#[no_mangle]
pub extern "C" fn finish() -> bool {
    unsafe {
        // 1. Get otxn (EscrowFinish)?

        // Access a field from the current EscrowFinish transaction that triggered this WASM
        // execution.
        // pub fn getCurrentTxField(
        //
        //
        // fname_ptr: i32, fname_len: i32
        // ) -> i32;

        // Option1: Static allocation (with "Sugar")
        // const accountId:[u8,32];
        // getCurrentTxField(&accountId, len, field);

        // Option2: Dynamic allocation
        // let accountId:AccountID = getCurrentTxField(field);

        // TODO: What allocator would we use?
        // TODO: Does WasmGC do allocation, or just Garbage Collection/cleanup.

        // accountId:AccountID = getCurrentTxField(Account); -->

        let accountId = host::getCurrentTxField(1);
        let s_ref: &str = &format!("{}", accountId);
        xrpl_std_lib::util::log(s_ref);

        // TODO: Get fields from EscrowFinish TX (the otxn)
        // TODO: 1) `Owner` (source AccountID of the account that funded the escrow).
        // TODO: Tx common fields and other EscrowFinish fields.
        // TODO: Get fields from Escrow ledger object
        // TODO: Get arbitrary fields from an AccountRoot object.

        // let _result = host::add(1, 2);
        // let sender = get_tx_account_id();
        // let owner = get_current_escrow_account_id();
        // let dest = get_current_escrow_destination();
        // let dest_balance = get_account_balance(&dest);
        // let escrow_data = get_current_escrow_data();
        // let ed_str = String::from_utf8(escrow_data.clone()).unwrap();
        // let threshold_balance = ed_str.parse::<u64>().unwrap();
        // let pl_time = host::getParentLedgerTime();
        // let e_time = get_current_escrow_finish_after();
        //
        // print_data(&sender.to_vec());
        // print_data(&owner);
        // print_data(&dest);
        // print_data(&escrow_data);
        // print_number(&dest_balance);
        // print_number(&pl_time);
        // print_number(result);
        // print(100i32, 99i32);

        // sender == owner && dest_balance <= threshold_balance && pl_time >= e_time
        return true;
    }
}

// This function contains the actual, safe Rust logic for finishing the escrow.
// It operates on references to the structs.
// fn process_finish(escrow: &mut Escrow, finish_tx: &mut EscrowFinish) -> bool {
//     println!("Processing Escrow ID: {}", escrow.id);
//     println!("Finish Transaction Nonce: {}", finish_tx.nonce);
//
//     // --- Your actual logic goes here ---
//     // Example: Check conditions, modify the escrow status, etc.
//     if escrow.status == 0 {
//         // Assuming 0 means 'active'
//         println!("Finishing active escrow...");
//         // Perform checks using finish_tx data...
//         // If valid:
//         escrow.status = 1; // Assuming 1 means 'finished'
//         println!("Escrow finished successfully.");
//         true // Indicate success
//     } else {
//         eprintln!("Escrow cannot be finished (status: {})", escrow.status);
//         false // Indicate failure
//     }
//     // --- End of your logic ---
// }
