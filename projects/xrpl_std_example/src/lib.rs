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

    // Types of objects WASM needs access to:
    // # Special Access
    // 1. Current transaction being processed (like `otxn` in hooks) --> Type: EscrowFinish
    // 2. Current ledger object associated with WASM code --> Type: Escrow (#2)
    // # Ledger Objects
    // 1. Ledger objects (by Keylet) --> High Gas (?) [Limited to 256 in hooks]
    // # Ledger Metadata/Header Info
    // 1. Ledger headers (or: Specific useful values like https://xrpl-hooks.readme.io/reference/ledger_seq)

    // (Not Yet, but Later)
    // 1. Emitted transaction (maybe)

    // # XRPL Programmabilty Data Pipeline
    /// returns a slot # (renamed from `fetch`) from 3 to 255
    /// fn slot(keylet_ptr: i32, keylet_len: i32) -> i32
    // TODO: This returns a slot # from 0 to 255 (in Hashtable in rippled).

    /// return an i32 and an i64. i32 is error code; i64 is the data len for data.
    /// i64 is value when returning numbers, in this case, output_len is 0.
    /// [NO] fn read(locator_ptr: i32, locator_len: i32, output_ptr: i32, output_len: i32) -> (i64)

    // Peng: Have four different groups of function for retrieving data.
    // 1: Originating TX
    // 2. Associated Ledger Object
    // 3. Info about current ledger.
    // 4. Ledger Object (by keylet)
    // For #1,2,3 --> No datasource required.
    // For #4, need a register number (0,1,2 are pre-allocated above; from 4 - 256 it's up to WASM to slot these).
    // Datasource is a register number.
    // A. Call fetch with a keylet; this returns a "slot" to the programmer.
    // B.

    /// fn read_by_keylet(locator_ptr: i32, locator_len: i32, output_ptr: i32, output_len: i32) -> (i64)
    /// fn read_otxn(locator_ptr: i32, locator_len: i32, output_ptr: i32, output_len: i32) -> (i64)
    /// fn read_hosting_lob(locator_ptr: i32, locator_len: i32, output_ptr: i32, output_len: i32) -> (i64)
    /// [NO] fn read_ledger_info(locator_ptr: i32, locator_len: i32, output_ptr: i32, output_len: i32) -> (i64)
    /// fn ledger_hash
    /// fn ledger_last_time
    /// ...etc.

    ///
    /// let status:i32 = fetch(ptr,len);
    /// const locator:string = "[keylet][sField/index]"; --> Max 256 keylets in any single contract enforced in rippled
    /// const output = [u8;64];
    /// let escrowAmountHandle = read(&locator, locator.len, &output, &output.len);

    /// handle(locator_ptr: i32, locator_len: i32);
    ///
    /// let locator:string = "keylet1:sfAccount";
    /// let handle:Handle = host::handle(&locator;locator.len);
    /// host::trace(handle)
    /// host::equals(handle1,handle2) --> true/false
    /// let field = host::get_field(handle, sField)
    // Access one field in Locator (Escrow2.Account)
    // 33 bytes --> datasource (ledger, tx, keylet etc)
    // 1 byte (locator field type) -> index or sfield
    // 4 bytes (sfield or index)
    // --> 38 bytes
    let _ = trace_msg("}");
    // sender == owner && dest_balance <= threshold_balance && pl_time >= e_time
    let _ = trace_msg("$$$$$ WASM EXECUTION COMPLETE $$$$$");

    true
}
