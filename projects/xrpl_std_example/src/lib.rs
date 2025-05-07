use xrpl_std_lib::core::amount::Amount;
use xrpl_std_lib::core::amount::xrp_amount::XrpAmount;
use xrpl_std_lib::core::constants::{ACCOUNT_ONE, ACCOUNT_ZERO};
use xrpl_std_lib::core::types::account_id::AccountID;
use xrpl_std_lib::core::types::blob::Blob;
use xrpl_std_lib::core::types::crypto_condition::{Condition, Fulfillment};
use xrpl_std_lib::core::types::hash_256::Hash256;
use xrpl_std_lib::core::types::public_key::PublicKey;
use xrpl_std_lib::core::types::transaction_type::TransactionType;
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
    // Step #1: Access & Emit Common Transaction fields from an EscrowFinish
    // ########################################
    let _ = trace_msg("{");
    let _ = trace_msg("  -- EscrowFinish Common Fields");

    // Questions:
    // 1. What if we don't want to actually draw the bytes of a field across the boundary? It would
    // be nice to get a "handle" to a field. E.g.,
    // It would be nice to trace fields without actually copying bytes across the WASM boundary.
    // WASM needs to be able to do things with bytes (e.g., eq, less-than, etc.
    // Things like CredentialIDs require a way to access them without loading/copying the entire
    // array of IDS. E.g., just get the 5th one.

    // Field: TransactionID
    let escrow_finish_tx_id: Hash256 = escrow_finish::get_tx_id();
    let _ = trace_msg_with_data("  EscrowFinish TxId:", &escrow_finish_tx_id.0, DataRepr::AsHex);

    // Field: Account
    let account: AccountID = escrow_finish::get_account();
    let _ = trace_msg_with_data("  Account:", &account.0, DataRepr::AsHex);
    if account.0.eq(&ACCOUNT_ONE.0) {
        let _ = trace_msg("    AccountID == ACCOUNT_ONE => TRUE");
    } else {
        let _ = trace_msg("    AccountID == ACCOUNT_ONE => FALSE");
        assert_eq!(account, ACCOUNT_ONE);
    }

    // Field: TransactionType
    let transaction_type: TransactionType = escrow_finish::get_transaction_type();
    let tx_type_bytes: [u8; 2] = transaction_type.into();
    let _ = trace_msg_with_data("  TransactionType (EscrowFinish):", &tx_type_bytes, DataRepr::AsHex);

    // Field: Fee
    let fee: Amount = escrow_finish::get_fee();
    let fee_as_xrp_amount: XrpAmount = match fee {
        Amount::Xrp(xrp_amount) => xrp_amount,
    };
    let _ = trace_num("  Fee:", fee_as_xrp_amount.0 as i64);

    // Field: Sequence
    let sequence: u32 = escrow_finish::get_sequence();
    let _ = trace_num("  Sequence:", sequence as i64);

    // Field: AccountTxnID
    let account_txn_id: Hash256 = escrow_finish::get_account_txn_id();
    let _ = trace_msg_with_data("  AccountTxnID:", &account_txn_id.0, DataRepr::AsHex);

    // Field: Flags
    let flags: u32 = escrow_finish::get_flags();
    let _ = trace_num("  Flags:", flags as i64);

    // Field: LastLedgerSequence
    let last_ledger_sequence: u32 = escrow_finish::get_last_ledger_sequence();
    let _ = trace_num("  LastLedgerSequence:", last_ledger_sequence as i64);

    // Field: NetworkID
    let network_id: u32 = escrow_finish::get_network_id();
    let _ = trace_num("  NetworkID:", network_id as i64);

    // Field: SourceTag
    let source_tag: u32 = escrow_finish::get_source_tag();
    let _ = trace_num("  SourceTag:", source_tag as i64);

    // Field: SigningPubKey
    let signing_pub_key: PublicKey = escrow_finish::get_signing_pub_key();
    let _ = trace_msg_with_data("  SigningPubKey:", &signing_pub_key.0, DataRepr::AsHex);

    // Field: TicketSequence
    let ticket_sequence: u32 = escrow_finish::get_ticket_sequence();
    let _ = trace_num("  TicketSequence:", ticket_sequence as i64);

    // TODO: Memos (Array)
    // TODO: Signers (Array)

    let txn_signature: Blob = escrow_finish::get_txn_signature();
    let sliced_data_len: usize = txn_signature.len;
    let sliced_data: &[u8] = &txn_signature.data[..sliced_data_len];
    let _ = trace_msg_with_data("  TxnSignature:", &sliced_data, DataRepr::AsHex);

    // ########################################
    // Step #2: Access & Emit Specific fields from the connected Escrow Object
    // ########################################
    let _ = trace_msg("  -- EscrowFinish Fields");

    // Field: Account
    let owner: AccountID = escrow_finish::get_owner();
    let _ = trace_msg_with_data("  Owner:", &owner.0, DataRepr::AsHex);
    if owner.0[0].eq(&ACCOUNT_ZERO.0[0]) {
        let _ = trace_msg("    AccountID == ACCOUNT_ZERO => TRUE");
    } else {
        let _ = trace_msg("    AccountID == ACCOUNT_ZERO => FALSE");
        assert_eq!(owner, ACCOUNT_ZERO);
    }

    // Field: OfferSequence
    let offer_sequence: u32 = escrow_finish::get_offer_sequence();
    let _ = trace_num("  OfferSequence:", offer_sequence as i64);

    // Field: Condition
    let condition: Condition = escrow_finish::get_condition();
    let _ = trace_msg_with_data("  Condition:", &condition.0, DataRepr::AsHex);

    // TODO: CredentialIDs (Array of Strings)

    // Field: Fulfillment
    let fulfillment: Fulfillment = escrow_finish::get_fulfillment();
    let _ = trace_msg_with_data("  Fulfillment:", &fulfillment.0, DataRepr::AsHex);

    // Step #3: Get fields from the Escrow being finished....
    // TODO:

    // Step #4: Get arbitrary fields from an AccountRoot object.
    // TODO:
    // let sender = get_tx_account_id();
    // let dest_balance = get_account_balance(&dest);
    // let escrow_data = get_current_escrow_data();
    // let ed_str = String::from_utf8(escrow_data.clone()).unwrap();
    // let threshold_balance = ed_str.parse::<u64>().unwrap();
    // let pl_time = host::getParentLedgerTime();
    // let e_time = get_current_escrow_finish_after();
    
    let _ = trace_msg("}");
    // sender == owner && dest_balance <= threshold_balance && pl_time >= e_time
    let _ = trace_msg("$$$$$ WASM EXECUTION COMPLETE $$$$$");

    true
}
