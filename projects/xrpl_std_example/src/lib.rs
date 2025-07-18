#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use crate::host::{Result::Err, Result::Ok};
use xrpl_std::core::constants::{ACCOUNT_ONE, ACCOUNT_ZERO};
use xrpl_std::core::current_tx::escrow_finish::{EscrowFinish, get_current_escrow_finish};
use xrpl_std::core::current_tx::traits::{EscrowFinishFields, TransactionCommonFields};
use xrpl_std::core::ledger_objects::account;
use xrpl_std::core::ledger_objects::current_escrow::{CurrentEscrow, get_current_escrow};
use xrpl_std::core::ledger_objects::traits::{
    CurrentEscrowFields, CurrentLedgerObjectCommonFields,
};
use xrpl_std::core::locator::Locator;
use xrpl_std::core::types::account_id::AccountID;
use xrpl_std::core::types::blob::Blob;
use xrpl_std::core::types::hash_256::Hash256;
use xrpl_std::core::types::public_key::PublicKey;
use xrpl_std::core::types::transaction_type::TransactionType;
use xrpl_std::host;
use xrpl_std::host::trace::{DataRepr, trace, trace_amount, trace_data, trace_num};
use xrpl_std::sfield;

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> bool {
    let _ = trace("$$$$$ STARTING WASM EXECUTION $$$$$");
    let _ = trace("");

    // The transaction prompting execution of this contract.
    let escrow_finish: EscrowFinish = get_current_escrow_finish();

    // ########################################
    // Step #1: Trace All EscrowFinish Fields
    // ########################################
    {
        let _ = trace("### Step #1: Trace All EscrowFinish Fields");
        let _ = trace("{ ");
        let _ = trace("  -- Common Fields");

        // Trace Field: TransactionID
        let current_tx_id: Hash256 = escrow_finish.get_id().unwrap();
        let _ = trace_data("  EscrowFinish TxId:", &current_tx_id.0, DataRepr::AsHex);
        assert_eq!(current_tx_id, EXPECTED_TX_ID.into());

        // Trace Field: Account
        let account = escrow_finish.get_account().unwrap();
        let _ = trace_data("  Account:", &account.0, DataRepr::AsHex);
        if account.0.eq(&ACCOUNT_ONE.0) {
            let _ = trace("    AccountID == ACCOUNT_ONE => TRUE");
        } else {
            let _ = trace("    AccountID == ACCOUNT_ONE => FALSE");
            assert_eq!(account, ACCOUNT_ONE);
        }

        // Trace Field: TransactionType
        let transaction_type: TransactionType = escrow_finish.get_transaction_type().unwrap();
        assert_eq!(transaction_type, TransactionType::EscrowFinish);
        let tx_type_bytes: [u8; 2] = transaction_type.into();
        let _ = trace_data(
            "  TransactionType (EscrowFinish):",
            &tx_type_bytes,
            DataRepr::AsHex,
        );

        // Trace Field: ComputationAllowance
        let computation_allowance: u32 = escrow_finish.get_computation_allowance().unwrap();
        assert_eq!(computation_allowance, 1000001);
        let _ = trace_num("  ComputationAllowance:", computation_allowance as i64);

        // Trace Field: Fee
        let fee = escrow_finish.get_fee().unwrap();
        let _ = trace_amount("  Fee:", &fee);

        // Trace Field: Sequence
        let sequence: u32 = escrow_finish.get_sequence().unwrap();
        assert_eq!(sequence, 4294967295);
        let _ = trace_num("  Sequence:", sequence as i64);

        // Trace Field: AccountTxnID
        let opt_account_txn_id = escrow_finish.get_account_txn_id().unwrap();
        if let Some(account_txn_id) = opt_account_txn_id {
            assert_eq!(account_txn_id.0, EXPECTED_ACCOUNT_TXN_ID);
            let _ = trace_data("  AccountTxnID:", &account_txn_id.0, DataRepr::AsHex);
        }

        // Trace Field: Flags
        let opt_flags = escrow_finish.get_flags().unwrap();
        if let Some(flags) = opt_flags {
            assert_eq!(flags, 4294967294);
            let _ = trace_num("  Flags:", flags as i64);
        }

        // Trace Field: LastLedgerSequence
        let opt_last_ledger_sequence = escrow_finish.get_last_ledger_sequence().unwrap();
        if let Some(last_ledger_sequence) = opt_last_ledger_sequence {
            assert_eq!(last_ledger_sequence, 4294967292);
            let _ = trace_num("  LastLedgerSequence:", last_ledger_sequence as i64);
        }

        // Trace Field: NetworkID
        let opt_network_id = escrow_finish.get_network_id().unwrap();
        if let Some(network_id) = opt_network_id {
            assert_eq!(network_id, 4294967291);
            let _ = trace_num("  NetworkID:", network_id as i64);
        }

        // Trace Field: SourceTag
        let opt_source_tag = escrow_finish.get_source_tag().unwrap();
        if let Some(source_tag) = opt_source_tag {
            assert_eq!(source_tag, 4294967290);
            let _ = trace_num("  SourceTag:", source_tag as i64);
        }

        // Trace Field: SigningPubKey
        let signing_pub_key = escrow_finish.get_signing_pub_key().unwrap();
        assert_eq!(signing_pub_key.0, EXPECTED_TX_SIGNING_PUB_KEY);
        let _ = trace_data("  SigningPubKey:", &signing_pub_key.0, DataRepr::AsHex);

        // Trace Field: TicketSequence
        let opt_ticket_sequence = escrow_finish.get_ticket_sequence().unwrap();
        if let Some(ticket_sequence) = opt_ticket_sequence {
            assert_eq!(ticket_sequence, 4294967289);
            let _ = trace_num("  TicketSequence:", ticket_sequence as i64);
        }

        let array_len = unsafe { host::get_tx_array_len(sfield::Memos) };
        assert_eq!(array_len, 1);
        let _ = trace_num("  Memos array len:", array_len as i64);

        let mut memo_buf = [0u8; 1024];
        let mut locator = Locator::new();
        locator.pack(sfield::Memos);
        locator.pack(0);
        locator.pack(sfield::Memo);
        locator.pack(sfield::MemoType);
        let output_len = unsafe {
            host::get_tx_nested_field(
                locator.get_addr(),
                locator.num_packed_bytes(),
                memo_buf.as_mut_ptr(),
                memo_buf.len(),
            )
        };
        let _ = trace("    Memo #: 1");
        let _ = trace_data(
            "      MemoType:",
            &memo_buf[..output_len as usize],
            DataRepr::AsHex,
        );

        locator.repack_last(sfield::MemoData);
        let output_len = unsafe {
            host::get_tx_nested_field(
                locator.get_addr(),
                locator.num_packed_bytes(),
                memo_buf.as_mut_ptr(),
                memo_buf.len(),
            )
        };
        let _ = trace_data(
            "      MemoData:",
            &memo_buf[..output_len as usize],
            DataRepr::AsHex,
        );

        locator.repack_last(sfield::MemoFormat);
        let output_len = unsafe {
            host::get_tx_nested_field(
                locator.get_addr(),
                locator.num_packed_bytes(),
                memo_buf.as_mut_ptr(),
                memo_buf.len(),
            )
        };
        let _ = trace_data(
            "      MemoFormat:",
            &memo_buf[..output_len as usize],
            DataRepr::AsHex,
        );

        let array_len = unsafe { host::get_tx_array_len(sfield::Signers) };
        assert_eq!(array_len, 2);
        let _ = trace_num("  Signers array len:", array_len as i64);

        for i in 0..array_len {
            let mut buf = [0x00; 64];
            let mut locator = Locator::new();
            locator.pack(sfield::Signers);
            locator.pack(i);
            locator.pack(sfield::Account);
            let output_len = unsafe {
                host::get_tx_nested_field(
                    locator.get_addr(),
                    locator.num_packed_bytes(),
                    buf.as_mut_ptr(),
                    buf.len(),
                )
            };
            if output_len < 0 {
                //TODO rebase on to devnet3, there is an error code commit
                let _ = trace_num("  cannot get Account, error:", output_len as i64);
                break;
            }
            let _ = trace_num("    Signer #:", i as i64);
            let _ = trace_data(
                "     Account:",
                &buf[..output_len as usize],
                DataRepr::AsHex,
            );

            locator.repack_last(sfield::TxnSignature);
            let output_len = unsafe {
                host::get_tx_nested_field(
                    locator.get_addr(),
                    locator.num_packed_bytes(),
                    buf.as_mut_ptr(),
                    buf.len(),
                )
            };
            if output_len < 0 {
                let _ = trace_num("  cannot get TxnSignature, error:", output_len as i64);
                break;
            }
            let _ = trace_data(
                "     TxnSignature:",
                &buf[..output_len as usize],
                DataRepr::AsHex,
            );

            locator.repack_last(sfield::SigningPubKey);
            let output_len = unsafe {
                host::get_tx_nested_field(
                    locator.get_addr(),
                    locator.num_packed_bytes(),
                    buf.as_mut_ptr(),
                    buf.len(),
                )
            };
            let signing_pub_key: PublicKey = buf.into();
            assert_eq!(signing_pub_key.0, EXPECTED_TX_SIGNING_PUB_KEY);

            if output_len < 0 {
                let _ = trace_num(
                    "  Error getting SigningPubKey. error_code = ",
                    output_len as i64,
                );
                break;
            }
            let _ = trace_data(
                "     SigningPubKey:",
                &buf[..output_len as usize],
                DataRepr::AsHex,
            );
        }

        let txn_signature: Blob = escrow_finish.get_txn_signature().unwrap();
        assert_eq!(txn_signature.data[..71], EXPECTED_TXN_SIGNATURE);
        let _ = trace_data(
            "  TxnSignature:",
            &txn_signature.data[..txn_signature.len],
            DataRepr::AsHex,
        );

        let _ = trace("  -- EscrowFinish Fields");

        // Trace Field: Account
        let owner: AccountID = escrow_finish.get_owner().unwrap();
        let _ = trace_data("  Owner:", &owner.0, DataRepr::AsHex);
        if owner.0[0].eq(&ACCOUNT_ZERO.0[0]) {
            let _ = trace("    AccountID == ACCOUNT_ZERO => TRUE");
        } else {
            let _ = trace("    AccountID == ACCOUNT_ZERO => FALSE");
            assert_eq!(owner, ACCOUNT_ZERO);
        }

        // Trace Field: OfferSequence
        let offer_sequence: u32 = escrow_finish.get_offer_sequence().unwrap();
        assert_eq!(offer_sequence, 4294967293);
        let _ = trace_num("  OfferSequence:", offer_sequence as i64);

        // Trace Field: Condition
        let opt_condition = escrow_finish.get_condition().unwrap();
        if let Some(condition) = opt_condition {
            debug_assert_eq!(condition.0, EXPECTED_ESCROW_FINISH_CONDITION);
            let _ = trace_data("  Condition:", &condition.0, DataRepr::AsHex);
        }

        let opt_fulfillment = escrow_finish.get_fulfillment().unwrap();
        if let Some(fulfillment) = opt_fulfillment {
            assert_eq!(
                fulfillment.data[..fulfillment.len],
                EXPECTED_ESCROW_FINISH_FULFILLMENT
            );
            let _ = trace_data(
                "  Fulfillment:",
                &fulfillment.data[..fulfillment.len],
                DataRepr::AsHex,
            );
        }

        // CredentialIDs (Array of Hashes)
        let array_len = unsafe { host::get_tx_array_len(sfield::CredentialIDs) };
        let _ = trace_num("  CredentialIDs array len:", array_len as i64);
        for i in 0..array_len {
            let mut buf = [0x00; 32];
            let mut locator = Locator::new();
            locator.pack(sfield::CredentialIDs);
            locator.pack(i);
            let output_len = unsafe {
                host::get_tx_nested_field(
                    locator.get_addr(),
                    locator.num_packed_bytes(),
                    buf.as_mut_ptr(),
                    buf.len(),
                )
            };
            let _ = trace_data(
                "  CredentialID:",
                &buf[..output_len as usize],
                DataRepr::AsHex,
            );
        }

        let _ = trace("}");
        let _ = trace(""); // Newline
    }

    // ########################################
    // Step #2: Trace All Current Escrow Ledger Object Fields
    // ########################################
    {
        let _ = trace("### Step #2: Trace Current Escrow Ledger Object Fields");
        let _ = trace("{ ");
        let _ = trace("  -- Common Fields");
        let current_escrow: CurrentEscrow = get_current_escrow();

        // Trace Field: Account
        let account = current_escrow.get_account().unwrap();
        assert_eq!(account.0, EXPECTED_CURRENT_ESCROW_ACCOUNT_ID);
        let _ = trace_data("  Escrow Account:", &account.0, DataRepr::AsHex);

        // Trace Field: Amount
        let amount = current_escrow.get_amount().unwrap();
        let _ = trace_amount("  Escrow Amount:", &amount);

        // Trace Field: LedgerEntryType
        let ledger_entry_type = current_escrow.get_ledger_entry_type().unwrap();
        assert_eq!(ledger_entry_type, 117);
        let _ = trace_num("  Escrow LedgerEntryType:", ledger_entry_type as i64);

        // Trace Field: CancelAfter
        let opt_cancel_after = current_escrow.get_cancel_after().unwrap();
        if let Some(cancel_after) = opt_cancel_after {
            let _ = trace_num("  Escrow CancelAfter:", cancel_after as i64);
        }

        // Trace Field: Condition
        let opt_condition = current_escrow.get_condition().unwrap();
        if let Some(condition) = opt_condition {
            let _ = trace_data("  Escrow Condition:", &condition.0, DataRepr::AsHex);
        }

        // Trace Field: Destination
        let destination = current_escrow.get_destination().unwrap();
        let _ = trace_data("  Escrow Destination:", &destination.0, DataRepr::AsHex);

        // Trace Field: DestinationTag
        let opt_destination_tag = current_escrow.get_destination_tag().unwrap();
        if let Some(destination_tag) = opt_destination_tag {
            let _ = trace_num("  Escrow DestinationTag:", destination_tag as i64);
        }

        // Trace Field: FinishAfter
        let opt_finish_after = current_escrow.get_finish_after().unwrap();
        if let Some(finish_after) = opt_finish_after {
            let _ = trace_num("  Escrow FinishAfter:", finish_after as i64);
        }

        // Trace Field: Flags
        let result = current_escrow.get_get_flags();
        if let Ok(flags) = result {
            let _ = trace_num("  Escrow Flags:", flags as i64);
        } else if let Err(error) = result {
            let _ = trace_num("  Error getting Flags. error_code = ", error.code() as i64);
        }

        // Trace Field: FinishFunction
        let opt_finish_function = current_escrow.get_finish_function().unwrap();
        if let Some(finish_function) = opt_finish_function {
            let _ = trace_data(
                "  Escrow FinishFunction:",
                &finish_function.data[..finish_function.len],
                DataRepr::AsHex,
            );
        }

        // Trace Field: OwnerNode
        let owner_node = current_escrow.get_owner_node().unwrap();
        let _ = trace_num("  Escrow OwnerNode:", owner_node as i64);

        // Trace Field: DestinationNode
        let opt_destination_node = current_escrow.get_destination_node().unwrap();
        if let Some(destination_node) = opt_destination_node {
            let _ = trace_num("  Escrow DestinationNode:", destination_node as i64);
        }

        // Trace Field: PreviousTxnID
        let previous_txn_id = current_escrow.get_previous_txn_id().unwrap();
        let _ = trace_data(
            "  Escrow PreviousTxnID:",
            &previous_txn_id.0,
            DataRepr::AsHex,
        );

        // Trace Field: PreviousTxnLgrSeq
        let previous_txn_lgr_seq = current_escrow.get_previous_txn_lgr_seq().unwrap();
        let _ = trace_num("  Escrow PreviousTxnLgrSeq:", previous_txn_lgr_seq as i64);

        // Trace Field: SourceTag
        let opt_source_tag = current_escrow.get_source_tag().unwrap();
        if let Some(source_tag) = opt_source_tag {
            let _ = trace_num("  Escrow SourceTag:", source_tag as i64);
        }

        // TODO: Provide access?
        // Trace Field: index
        // let ledger_index = current_escrow.get_ledger_index().unwrap();
        // let _ = trace_data("  Escrow index:", &ledger_index.0, DataRepr::AsHex);

        let _ = trace("}");
        let _ = trace("");
    }

    // ########################################
    // Step #3 [EscrowFinish Account]: Trace Current Balance
    // ########################################
    {
        let _ = trace("### Step #3: Trace EscrowFinish Account Balance");
        let _ = trace("{ ");
        let account: AccountID = escrow_finish.get_account().unwrap();
        let balance = account::get_account_balance(&account).unwrap();
        let _ = trace_num("  Balance of Account Finishing the Escrow:", balance as i64);
        if balance == 0 {
            let _ = trace("  Balance of Account Finishing the Escrow was 0");
            return false;
        }
        let _ = trace("}");
        let _ = trace("");
    }

    // ########################################
    // Step #4 [Arbitrary Ledger Object]: Get arbitrary fields from an AccountRoot object.
    // ########################################
    {
        let _ = trace("### Step #4a: Trace AccountRoot Ledger Object");
        let _ = trace("{ ");
        let _ = trace("  -- Common Fields");
        let _ = trace("    -- TODO: Finish tracing all fields");
        let _ = trace("  -- Specific Fields");
        let _ = trace("    -- TODO: Finish tracing all fields");
        let _ = trace("}");
        let _ = trace("");
        // TODO: Implement these.
        // let sender = get_tx_account_id();
        // let dest_balance = get_account_balance(&dest);
        // let escrow_data = get_current_escrow_data();
        // let ed_str = String::from_utf8(escrow_data.clone()).unwrap();
        // let threshold_balance = ed_str.parse::<u64>().unwrap();
        // let pl_time = host::getParentLedgerTime();
        // let e_time = get_current_current_transaction_after();

        // ########################################
        // Step #4 [NFT]: Trace all fields from an NFT
        // ########################################
        let _ = trace("### Step #4b: Trace Nft Ledger Object");
        let _ = trace("{ ");
        let _ = trace("  -- Common Fields");
        let _ = trace("    -- TODO: Finish tracing all fields");
        let _ = trace("  -- Specific Fields");
        let _ = trace("    -- TODO: Finish tracing all fields");
        let _ = trace("}");
    }

    // ########################################
    // Step #5 [Ledger Headers]: Emit all ledger headers.
    // ########################################
    {
        let _ = trace("### Step #5: Trace Ledger Headers");
        let _ = trace("{ ");
        // TODO: Implement this.
        let _ = trace("    -- TODO: Finish tracing all fields");
        let _ = trace("}");

        // TODO: Remove these examples once the above TODOs are completed.
        // Keep the commented out validation code from main branch
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
            // let slot = unsafe { host_lib::cache_ledger_obj(keylet.as_ptr(), keylet.len(), 0) };
            //
            // println!("wasm finish slot {:?}", slot);
            //
            // let mut locator = Locator::new();
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
    }

    let _ = trace("$$$$$ WASM EXECUTION COMPLETE $$$$$");
    true // <-- Finish the escrow.
}

/// The following are private constants used for testing purposes to enforce value checks in this
/// contract (to ensure that code changes don't break this contract).
const EXPECTED_TX_ID: [u8; 32] = [
    0x74, 0x46, 0x51, 0x21, 0x37, 0x28, 0x13, 0xCB, 0xA4, 0xC7, 0x7E, 0x31, 0xF1, 0x2E, 0x13, 0x71,
    0x63, 0xF5, 0xB2, 0x50, 0x9B, 0x16, 0xAC, 0x17, 0x03, 0xEC, 0xF0, 0xDA, 0x19, 0x4B, 0x2D, 0xD4,
];

const EXPECTED_ACCOUNT_TXN_ID: [u8; 32] = [0xDD; 32];

const EXPECTED_TX_SIGNING_PUB_KEY: [u8; 33] = [
    0x03, 0x30, 0xE7, 0xFC, 0x9D, 0x56, 0xBB, 0x25, 0xD6, 0x89, 0x3B, 0xA3, 0xF3, 0x17, 0xAE, 0x5B,
    0xCF, 0x33, 0xB3, 0x29, 0x1B, 0xD6, 0x3D, 0xB3, 0x26, 0x54, 0xA3, 0x13, 0x22, 0x2F, 0x7F, 0xD0,
    0x20,
];

const EXPECTED_TXN_SIGNATURE: [u8; 71] = [
    0x30, 0x45, 0x02, 0x21, 0x00, 0x8A, 0xD5, 0xEE, 0x48, 0xF7, 0xF1, 0x04, 0x78, 0x13, 0xE7, 0x9C,
    0x17, 0x4F, 0xE4, 0x01, 0xD0, 0x23, 0xA4, 0xB4, 0xA7, 0xB9, 0x9A, 0xF8, 0x26, 0xE0, 0x81, 0xDB,
    0x1D, 0xFF, 0x7B, 0x9C, 0x51, 0x02, 0x20, 0x13, 0x3F, 0x05, 0xB7, 0xFD, 0x3D, 0x7D, 0x7F, 0x16,
    0x3E, 0x8C, 0x77, 0xEE, 0x0A, 0x49, 0xD0, 0x26, 0x19, 0xAB, 0x6C, 0x77, 0xCC, 0x34, 0x87, 0xD0,
    0x09, 0x5C, 0x9B, 0x34, 0x03, 0x3C, 0x1C,
];

const EXPECTED_ESCROW_FINISH_CONDITION: [u8; 32] = [0x33; 32];
const EXPECTED_ESCROW_FINISH_FULFILLMENT: [u8; 32] = [0x21; 32];

/// Represents rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn
const EXPECTED_CURRENT_ESCROW_ACCOUNT_ID: [u8; 20] = [
    0x4B, 0x4E, 0x9C, 0x06, 0xF2, 0x42, 0x96, 0x07, 0x4F, 0x7B, 0xC4, 0x8F, 0x92, 0xA9, 0x79, 0x16,
    0xC6, 0xDC, 0x5E, 0xA9,
];
