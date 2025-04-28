use xrpl_std_lib::core::types::{AccountID, Hash256};
use xrpl_std_lib::host::trace::DataRepr;
use {
    host::trace::trace_msg, host::trace::trace_msg_with_data, xrpl_std_lib::host, xrpl_std_lib::utils::escrow_finish,
};

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
    let _ = trace_msg("$$ STARTING WASM EXECUTION $$");

    // TODO: Get a handle to the EscrowFinish as a Transaction?

    // Step #1: Get fields from an EscrowFinish
    let escrow_finish_tx_id: Hash256 = escrow_finish::get_tx_id();
    let _ = trace_msg_with_data("EscrowFinish TxId: ", &escrow_finish_tx_id.0, DataRepr::AsHex);

    let account: AccountID = escrow_finish::get_account_id();
    let _ = trace_msg_with_data("EscrowFinish TxId: ", &account.0, DataRepr::AsHex);

    // let transaction_type:TransactionType = escrow_finish::get_transaction_type();
    // etc.
    // EscrowFinish Fields
    // TODO: Get Account
    // TODO: TransactionType?
    // TODO: Fee
    // TODO: Sequence
    // TODO: AccountTxnID
    // TODO: Flags
    // TODO: LastLedgerSequence
    // TODO: Memos
    // TODO: NetworkID
    // TODO: Signers
    // TODO: SourceTag
    // TODO: SigningPubKey
    // TODO: TicketSequence
    // TODO: TxnSignature
    //
    // TODO: Get Owner
    // TODO: OfferSequence
    // TODO: Condition
    // TODO: CredentialIDs array
    // TODO: Fulfillment

    // Step #2: Get fields from the Escrow being finished....

    // Step #3: Get arbitrary fields from an AccountRoot object.
    // let sender = get_tx_account_id();
    // let dest_balance = get_account_balance(&dest);
    // let escrow_data = get_current_escrow_data();
    // let ed_str = String::from_utf8(escrow_data.clone()).unwrap();
    // let threshold_balance = ed_str.parse::<u64>().unwrap();
    // let pl_time = host::getParentLedgerTime();
    // let e_time = get_current_escrow_finish_after();

    // sender == owner && dest_balance <= threshold_balance && pl_time >= e_time
    let _ = trace_msg("$$ WASM EXECUTION COMPLETE $$");

    true
}
