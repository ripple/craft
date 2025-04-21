use xrpl_std_lib::core::types::Hash256;
use {xrpl_std_lib::utils::escrow_finish, xrpl_std_lib::utils::logging};

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
    logging::log("$$ STARTING WASM EXECUTION");
    
    // TODO: Get a handle to the EscrowFinish as a Transaction?
    // 1. Get otxn (EscrowFinish)?
    let escrow_finish_tx_id: Hash256 = escrow_finish::get_tx_id();
    logging::log_hash_ref("EscrowFinish TxId: ", &escrow_finish_tx_id);

    // TODO: Get fields from an EscrowFinish
    // let account:AccountID = escrow_finish::get_account();
    // let transaction_type:TransactionType = escrow_finish::get_transaction_type();
    // etc.
    // EscrowFinish Fields
    // TODO: Get Account
    // TODO: Get Owner
    // TODO: OfferSequence
    // TODO: Condition
    // TODO: CredentialIDs array
    // TODO: Fulfillment
    
    // TODO: Get arbitrary fields from an AccountRoot object.
    // let sender = get_tx_account_id();
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

    logging::log("$$ WASM EXECUTION COMPLETE!");

    return true;
    // }
}
