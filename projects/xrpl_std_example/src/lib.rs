use xrpl_std_lib::core::amount::Amount;
use xrpl_std_lib::core::amount::xrp_amount::XrpAmount;
use xrpl_std_lib::core::constants::ACCOUNT_ONE;
use xrpl_std_lib::core::types::{AccountID, Hash256, TransactionType};
use xrpl_std_lib::host;
use xrpl_std_lib::host::trace::DataRepr;
use {
    host::trace::trace_msg, host::trace::trace_msg_with_data, host::trace::trace_num,
    xrpl_std_lib::utils::escrow_finish,
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
    let _ = trace_msg("$$$$$ STARTING WASM EXECUTION $$$$$");

    // TODO: Get a handle to the EscrowFinish as a Transaction?
    // let escrow_finish:EscrowFinish = apply_ctx.cur_tx;
    // let account:AccountID = escrow_finish.account_id();

    // ########################################
    // Step #1: Get fields from an EscrowFinish
    // ########################################
    let _ = trace_msg("## EscrowFinish Common Fields ##");
    // Field: TransactionID
    let escrow_finish_tx_id: Hash256 = escrow_finish::get_tx_id();
    let _ = trace_msg_with_data("EscrowFinish TxId:", &escrow_finish_tx_id.0, DataRepr::AsHex);

    // Field: Account
    let account: AccountID = escrow_finish::get_account();
    let _ = trace_msg_with_data("Account:", &account.0, DataRepr::AsHex);
    if account.0[0].eq(&ACCOUNT_ONE.0[0]) {
        let _ = trace_msg("AccountID == ACCOUNT_ONE => TRUE");
    } else {
        let _ = trace_msg("AccountID == ACCOUNT_ONE => FALSE");
        assert_eq!(account, ACCOUNT_ONE);
    }

    // Field: TransactionType
    let transaction_type: TransactionType = escrow_finish::get_transaction_type();
    let tx_type_bytes: [u8; 2] = transaction_type.into();
    let _ = trace_msg_with_data("TransactionType (EscrowFinish):", &tx_type_bytes, DataRepr::AsHex);

    // Field Fee
    let fee: Amount = escrow_finish::get_fee();
    let fee_as_xrp_amount: XrpAmount = match fee {
        Amount::Xrp(xrp_amount) => xrp_amount,
    };
    let _ = trace_num("Fee:", fee_as_xrp_amount.0 as i64);

    // etc.
    // EscrowFinish Fields
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

    let _ = trace_msg("## EscrowFinish Fields ##");
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
    let _ = trace_msg("$$$$$ WASM EXECUTION COMPLETE $$$$$");

    true
}
