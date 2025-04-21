use xrpl_std_lib::core::types::Hash256;

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
pub extern "C" fn ready() -> bool {
    // unsafe {
    xrpl_std_lib::utils::logging::log("$$ STARTING WASM EXECUTION");

    // TODO: Get a handle to the EscrowFinish as a Transaction?
    // 1. Get otxn (EscrowFinish)?
    let escrow_finish_tx_id: Hash256 = xrpl_std_lib::utils::escrow_finish::get_tx_id();
    // unsafe {
    //         let value_via_pointer: &Hash256 = &*address_ptr;
    //         println!("Value accessed via pointer: {:?}", value_via_pointer);
    //         assert_eq!(escrow_finish_tx_id, *value_via_pointer);
    // }
    let address_ptr: *const Hash256 = &escrow_finish_tx_id;

    xrpl_std_lib::utils::logging::log_hash_ref("EscrowFinish TxId: ", &escrow_finish_tx_id);
    // xrpl_std_lib::utils::logging::log_hash_ptr("EscrowFinish TxId: ", address_ptr);

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

    xrpl_std_lib::utils::logging::log("$$ WASM EXECUTION COMPLETE!");

    return true;
    // }
}
