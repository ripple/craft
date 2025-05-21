#![no_std]

use xrpl_std::core::amount::Amount;
use xrpl_std::core::amount::xrp_amount::XrpAmount;
use xrpl_std::core::constants::{ACCOUNT_ONE, ACCOUNT_ZERO};
use xrpl_std::core::tx::current_transaction;
use xrpl_std::core::types::account_id::AccountID;
use xrpl_std::core::types::blob::Blob;
use xrpl_std::core::types::crypto_condition::{Condition, Fulfillment};
use xrpl_std::core::types::hash_256::Hash256;
use xrpl_std::core::types::public_key::PublicKey;
use xrpl_std::core::types::transaction_type::TransactionType;
use xrpl_std::host::trace::{DataRepr, trace, trace_data, trace_num};

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    let _ = trace("$$$$$ STARTING WASM EXECUTION $$$$$");

    // TODO: It would be nice to get a handle to the EscrowFinish as a Transaction. How does the
    // std-lib facilitate that?
    // let current_transaction:EscrowFinish = apply_ctx.cur_tx;
    // let account:AccountID = current_transaction.account_id();

    // ########################################
    // Step #1: Access & Emit Common Transaction fields from an EscrowFinish
    // ########################################
    let _ = trace("{");
    let _ = trace("  -- EscrowFinish Common Fields");

    // TODO: FIXME!
    // Field: TransactionID
    // let current_transaction_tx_id: Hash256 = current_transaction::get_tx_id();
    // let _ = trace_data("  EscrowFinish TxId:", &current_transaction_tx_id.0, DataRepr::AsHex);

    // Field: Account
    let account: AccountID = current_transaction::get_account();
    let _ = trace_data("  Account:", &account.0, DataRepr::AsHex);
    if account.0.eq(&ACCOUNT_ONE.0) {
        let _ = trace("    AccountID == ACCOUNT_ONE => TRUE");
    } else {
        let _ = trace("    AccountID == ACCOUNT_ONE => FALSE");
        assert_eq!(account, ACCOUNT_ONE);
    }

    // Field: TransactionType
    let transaction_type: TransactionType = current_transaction::get_transaction_type();
    let tx_type_bytes: [u8; 2] = transaction_type.into();
    let _ = trace_data(
        "  TransactionType (EscrowFinish):",
        &tx_type_bytes,
        DataRepr::AsHex,
    );

    // Field: Fee
    let fee: Amount = current_transaction::get_fee();
    let fee_as_xrp_amount: XrpAmount = match fee {
        Amount::Xrp(xrp_amount) => xrp_amount,
    };
    let _ = trace_num("  Fee:", fee_as_xrp_amount.0 as i64);

    // Field: Sequence
    let sequence: u32 = current_transaction::get_sequence();
    let _ = trace_num("  Sequence:", sequence as i64);

    // Field: AccountTxnID
    let account_txn_id: Hash256 = current_transaction::get_account_txn_id();
    let _ = trace_data("  AccountTxnID:", &account_txn_id.0, DataRepr::AsHex);

    // Field: Flags
    let flags: u32 = current_transaction::get_flags();
    let _ = trace_num("  Flags:", flags as i64);

    // Field: LastLedgerSequence
    let last_ledger_sequence: u32 = current_transaction::get_last_ledger_sequence();
    let _ = trace_num("  LastLedgerSequence:", last_ledger_sequence as i64);

    // Field: NetworkID
    let network_id: u32 = current_transaction::get_network_id();
    let _ = trace_num("  NetworkID:", network_id as i64);

    // Field: SourceTag
    let source_tag: u32 = current_transaction::get_source_tag();
    let _ = trace_num("  SourceTag:", source_tag as i64);

    // Field: SigningPubKey
    let signing_pub_key: PublicKey = current_transaction::get_signing_pub_key();
    let _ = trace_data("  SigningPubKey:", &signing_pub_key.0, DataRepr::AsHex);

    // Field: TicketSequence
    let ticket_sequence: u32 = current_transaction::get_ticket_sequence();
    let _ = trace_num("  TicketSequence:", ticket_sequence as i64);

    // TODO: Memos (Array)
    // TODO: Signers (Array)

    let txn_signature: Blob = current_transaction::get_txn_signature();
    let sliced_data_len: usize = txn_signature.len;
    let sliced_data: &[u8] = &txn_signature.data[..sliced_data_len];
    let _ = trace_data("  TxnSignature:", &sliced_data, DataRepr::AsHex);

    // ########################################
    // Step #2: Access & Emit Specific fields from the connected Escrow Object
    // ########################################
    let _ = trace("  -- EscrowFinish Fields");

    // Field: Account
    let owner: AccountID = current_transaction::get_owner();
    let _ = trace_data("  Owner:", &owner.0, DataRepr::AsHex);
    if owner.0[0].eq(&ACCOUNT_ZERO.0[0]) {
        let _ = trace("    AccountID == ACCOUNT_ZERO => TRUE");
    } else {
        let _ = trace("    AccountID == ACCOUNT_ZERO => FALSE");
        assert_eq!(owner, ACCOUNT_ZERO);
    }

    // Field: OfferSequence
    let offer_sequence: u32 = current_transaction::get_offer_sequence();
    let _ = trace_num("  OfferSequence:", offer_sequence as i64);

    // Field: Condition
    let condition: Condition = current_transaction::get_condition();
    let _ = trace_data("  Condition:", &condition.0, DataRepr::AsHex);

    // TODO: CredentialIDs (Array of Strings)

    // Field: Fulfillment
    let fulfillment: Fulfillment = current_transaction::get_fulfillment();
    let _ = trace_data("  Fulfillment:", &fulfillment.0, DataRepr::AsHex);

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
    // let e_time = get_current_current_transaction_after();

    let _ = trace("}");
    // sender == owner && dest_balance <= threshold_balance && pl_time >= e_time
    let _ = trace("$$$$$ WASM EXECUTION COMPLETE $$$$$");

    // TODO: Uncomment and Ensure each of these work!
    {
        // let account_id_clo = match get_current_escrow_account_id() {
        //     Some(v) => v,
        //     None => return -2,
        // };
        //
        // let destination = match get_current_escrow_destination() {
        //     Some(v) => v,
        //     None => return -3,
        // };
        // if account_id_clo != account_id_tx {
        //     return -6;
        // }
        // if destination == account_id_tx {
        //     return -7;
        // }
    }
    {
        // let finish_after = match get_current_current_transaction_after() {
        //     Some(v) => v,
        //     None => return -4,
        // };
        // if finish_after == 0 {
        //     return -8;
        // }
    }
    {
        // let mut ledger_sqn = 0i32;
        // if unsafe { xrpl_std::host_lib::get_ledger_sqn((&mut ledger_sqn) as *mut i32 as *mut u8, 4) }
        //     <= 0
        // {
        //     return -10;
        // }
    }
    {
        // let keylet = [
        //     52, 47, 158, 13, 36, 46, 219, 67, 160, 251, 252, 103, 43, 48, 44, 200, 187, 144, 73,
        //     147, 23, 46, 87, 251, 255, 76, 93, 74, 30, 184, 90, 185,
        // ];
        // println!("wasm finish keylet {:?}", keylet);
        //
        // let slot = unsafe { host_lib::ledger_slot_set(keylet.as_ptr(), keylet.len(), 0) };
        //
        // println!("wasm finish slot {:?}", slot);
        //
        // let mut locator = LocatorPacker::new();
        // locator.pack(SignerEntries);
        // let array_len = unsafe {
        //     host_lib::get_ledger_obj_nested_array_len(slot, locator.get_addr(), locator.num_packed_bytes())
        // };
        // println!("wasm finish array_len {:?}", array_len);
        //
        // locator.pack(0);
        // locator.pack(SignerEntry);
        // locator.pack(SignerWeight);
        //
        // let mut weight = 0i32;
        // let nfr = unsafe {
        //     host_lib::get_ledger_obj_nested_field(
        //         slot, locator.get_addr(), locator.num_packed_bytes(),
        //         (&mut weight) as *mut i32 as *mut u8, 4
        //     )
        // };
        //
        // println!("wasm finish get_ledger_obj_nested_field {:?} {}", nfr, weight);
    }
    {
        // let nft_id = [
        //     0, 8, 39, 16, 104, 7, 191, 132, 143, 172, 217, 114, 242, 246, 23, 226, 112, 3, 215, 91,
        //     44, 170, 201, 129, 108, 238, 20, 132, 5, 33, 209, 233,
        // ];
        // let owner = get_tx_account_id().unwrap();
        // if owner.len() != 20 {
        //     return -21;
        // }
        // let mut arr = [0u8; 256];
        // let res = unsafe {
        //     host_lib::get_NFT(
        //         owner.as_ptr(),
        //         owner.len(),
        //         nft_id.as_ptr(),
        //         nft_id.len(),
        //         arr.as_mut_ptr(),
        //         arr.len(),
        //     )
        // };
        //
        // if res != 106 {
        //     return -22;
        // }
    }

    1
}
