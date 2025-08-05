#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use crate::host::{Result::Err, Result::Ok};
use xrpl_std::core::constants::{ACCOUNT_ONE, ACCOUNT_ZERO};
use xrpl_std::core::current_tx::escrow_finish::{EscrowFinish, get_current_escrow_finish};
use xrpl_std::core::current_tx::traits::{EscrowFinishFields, TransactionCommonFields};
use xrpl_std::core::ledger_objects::account::{Account, get_account_balance};
use xrpl_std::core::ledger_objects::current_escrow::{CurrentEscrow, get_current_escrow};
use xrpl_std::core::ledger_objects::traits::{
    AccountFields, CurrentEscrowFields, CurrentLedgerObjectCommonFields, LedgerObjectCommonFields,
};
use xrpl_std::core::locator::Locator;
use xrpl_std::core::types::account_id::AccountID;
use xrpl_std::core::types::amount::token_amount::TokenAmount;
use xrpl_std::core::types::blob::Blob;
use xrpl_std::core::types::hash_256::Hash256;
use xrpl_std::core::types::public_key::PublicKey;
use xrpl_std::core::types::transaction_type::TransactionType;
use xrpl_std::host;
use xrpl_std::host::cache_ledger_obj;
use xrpl_std::host::trace::{DataRepr, trace, trace_amount, trace_data, trace_num};
use xrpl_std::sfield;
use xrpl_std::{assert_eq, decode_hex_32};

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
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
                let _ = trace_num("  cannot get Account, error:", output_len as i64);
                panic!()
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
                panic!()
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
        let mut signature_bytes = [0u8; 71];
        signature_bytes.copy_from_slice(&txn_signature.data[..71]);
        assert_eq!(signature_bytes, EXPECTED_TXN_SIGNATURE);
        let _ = trace_data("  TxnSignature:", &signature_bytes, DataRepr::AsHex);

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
            assert_eq!(condition.0, EXPECTED_ESCROW_FINISH_CONDITION);
            let _ = trace_data("  Condition:", &condition.0, DataRepr::AsHex);
        }

        let opt_fulfillment = escrow_finish.get_fulfillment().unwrap();
        if let Some(fulfillment) = opt_fulfillment {
            assert_eq!(
                &fulfillment.data[..fulfillment.len],
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
            if i == 0 {
                assert_eq!(buf, EXPECTED_CURRENT_ESCROW_CREDENTIAL1);
            } else if i == 1 {
                assert_eq!(buf, EXPECTED_CURRENT_ESCROW_CREDENTIAL2);
            } else if i == 2 {
                assert_eq!(buf, EXPECTED_CURRENT_ESCROW_CREDENTIAL3);
            } else {
                panic!()
            }

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
        let _ = trace_data("  Account:", &account.0, DataRepr::AsHex);

        // Trace Field: Amount
        let amount = current_escrow.get_amount().unwrap();
        let _ = trace_amount("  Amount:", &amount);

        // Trace Field: LedgerEntryType
        let ledger_entry_type = current_escrow.get_ledger_entry_type().unwrap();
        assert_eq!(ledger_entry_type, 117);
        let _ = trace_num("  LedgerEntryType:", ledger_entry_type as i64);

        // Trace Field: CancelAfter
        let opt_cancel_after = current_escrow.get_cancel_after().unwrap();
        if let Some(cancel_after) = opt_cancel_after {
            assert_eq!(cancel_after, 545440232);
            let _ = trace_num("  CancelAfter:", cancel_after as i64);
        }

        // Trace Field: Condition
        let opt_condition = current_escrow.get_condition().unwrap();
        if let Some(condition) = opt_condition {
            assert_eq!(condition.0, EXPECTED_ESCROW_CONDITION);
            let _ = trace_data("  Condition:", &condition.0, DataRepr::AsHex);
        }

        // Trace Field: Destination
        let destination = current_escrow.get_destination().unwrap();
        const EXPECTED_DESTINATION: [u8; 20] = [
            0x3E, 0x9D, 0x4A, 0x2B, 0x8A, 0xA0, 0x78, 0x0F, 0x68, 0x2D, 0x13, 0x6F, 0x7A, 0x56,
            0xD6, 0x72, 0x4E, 0xF5, 0x37, 0x54,
        ];
        assert_eq!(destination.0, EXPECTED_DESTINATION);
        let _ = trace_data("  Destination:", &destination.0, DataRepr::AsHex);

        // Trace Field: DestinationTag
        let opt_destination_tag = current_escrow.get_destination_tag().unwrap();
        if let Some(destination_tag) = opt_destination_tag {
            assert_eq!(destination_tag, 23480);
            let _ = trace_num("  DestinationTag:", destination_tag as i64);
        }

        // Trace Field: FinishAfter
        let opt_finish_after = current_escrow.get_finish_after().unwrap();
        if let Some(finish_after) = opt_finish_after {
            assert_eq!(finish_after, 545354132);
            let _ = trace_num("  FinishAfter:", finish_after as i64);
        }

        // Trace Field: Flags
        let result = current_escrow.get_get_flags();
        if let Ok(flags) = result {
            assert_eq!(flags, 0);
            let _ = trace_num("  Flags:", flags as i64);
        } else if let Err(error) = result {
            let _ = trace_num("  Error getting Flags. error_code = ", error.code() as i64);
        }

        // Trace Field: FinishFunction
        let opt_finish_function = current_escrow.get_finish_function().unwrap();
        if let Some(finish_function) = opt_finish_function {
            assert_eq!(finish_function.len, 172);
            let _ = trace_data(
                "  FinishFunction:",
                &finish_function.data[..finish_function.len],
                DataRepr::AsHex,
            );
        }

        // Trace Field: OwnerNode
        let owner_node = current_escrow.get_owner_node().unwrap();
        assert_eq!(owner_node, 0);
        let _ = trace_num("  OwnerNode:", owner_node as i64);

        // Trace Field: DestinationNode
        let opt_destination_node = current_escrow.get_destination_node().unwrap();
        if let Some(destination_node) = opt_destination_node {
            assert_eq!(destination_node, 0);
            let _ = trace_num("  DestinationNode:", destination_node as i64);
        }

        // Trace Field: PreviousTxnID
        let previous_txn_id = current_escrow.get_previous_txn_id().unwrap();
        assert_eq!(
            previous_txn_id.0,
            [
                0xC4, 0x4F, 0x2E, 0xB8, 0x41, 0x96, 0xB9, 0xAD, 0x82, 0x03, 0x13, 0xDB, 0xEB, 0xA6,
                0x31, 0x6A, 0x15, 0xC9, 0xA2, 0xD3, 0x57, 0x87, 0x57, 0x9E, 0xD1, 0x72, 0xB8, 0x7A,
                0x30, 0x13, 0x1D, 0xA7,
            ]
        );
        let _ = trace_data("  PreviousTxnID:", &previous_txn_id.0, DataRepr::AsHex);

        // Trace Field: PreviousTxnLgrSeq
        let previous_txn_lgr_seq = current_escrow.get_previous_txn_lgr_seq().unwrap();
        assert_eq!(previous_txn_lgr_seq, 28991004);
        let _ = trace_num("  PreviousTxnLgrSeq:", previous_txn_lgr_seq as i64);

        // Trace Field: SourceTag
        let opt_source_tag = current_escrow.get_source_tag().unwrap();
        if let Some(source_tag) = opt_source_tag {
            assert_eq!(source_tag, 11747);
            let _ = trace_num("  SourceTag:", 11747);
        }

        // Trace Field: `index` or `LedgerIndex`
        // The current decision is that this field should not be accessible from the ledger object.
        // let ledger_index = current_escrow.get_ledger_index().unwrap();
        // let _ = trace_data("  index:", &ledger_index.0, DataRepr::AsHex);

        let _ = trace("}");
        let _ = trace("");
    }

    // ########################################
    // Step #3 [EscrowFinish Account]: Trace Current Balance
    // ########################################
    {
        let _ = trace("### Step #3: Trace Account Balance for Account Finishing the Escrow");
        let _ = trace("{ ");
        let account: AccountID = escrow_finish.get_account().unwrap();
        let balance = match get_account_balance(&account).unwrap() {
            TokenAmount::XRP { num_drops } => num_drops,
            TokenAmount::IOU { .. } => {
                panic!("IOU Balance encountered, but should have been XRP.")
            }
            TokenAmount::MPT { .. } => {
                panic!("MPT Balance encountered, but should have been XRP.")
            }
        };

        assert_eq!(balance, 55426479402);
        let _ = trace_num("  Balance of Account Finishing the Escrow:", balance);
        let _ = trace("}");
        let _ = trace("");
    }

    // ########################################
    // Step #4 [Arbitrary Ledger Object]: Trace AccountRoot Fields.
    // ########################################
    {
        // Slot the account
        // "Account": "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh",
        let account_keylet =
            decode_hex_32(b"2B6AC232AA4C4BE41BF49D2459FA4A0347E1B543A4C92FCEE0821C0201E2E9A8")
                .unwrap();

        // Try to cache the ledger object inside rippled
        let slot = unsafe { cache_ledger_obj(account_keylet.as_ptr(), 32, 0) };
        if slot <= 0 {
            let _ = trace_num("Error slotting Account object", slot as i64);
            panic!()
        } else {
            let _ = trace_num("Account object slotted at", slot as i64);
        }

        // We use the trait-bound implementation so as not to duplicate accessor logic.
        let account = Account;

        let _ = trace("### Step #4: Trace AccountRoot Ledger Object");
        let _ = trace("{ ");
        let _ = trace("  -- Common Fields");

        // Trace the `Flags`
        let flags = account.get_flags(slot).unwrap();
        assert_eq!(flags, 1703936);
        let _ = trace_num("  Flags:", flags as i64);

        // Trace the `LedgerEntryType`
        let ledger_entry_type = account.ledger_entry_type(slot).unwrap();
        assert_eq!(ledger_entry_type, 97); // 97 is the code for "AccountRoot"
        let _ = trace_num("  LedgerEntryType (AccountRoot):", ledger_entry_type as i64);
        let _ = trace("} ");

        let _ = trace("{ ");
        let _ = trace("  -- Account Specific Fields");

        // Trace the `Account`
        let account_id = account.get_account(slot).unwrap();
        assert_eq!(
            account_id.0,
            [
                0xB5, 0xF7, 0x62, 0x79, 0x8A, 0x53, 0xD5, 0x43, 0xA0, 0x14, 0xCA, 0xF8, 0xB2, 0x97,
                0xCF, 0xF8, 0xF2, 0xF9, 0x37, 0xE8
            ]
        );
        let _ = trace_data("  Account:", &account_id.0, DataRepr::AsHex);

        // Trace the `AccountTxnID`
        let account_txn_id = account.account_txn_id(slot).unwrap();
        assert_eq!(
            account_txn_id.0,
            [
                0xBC, 0x8E, 0x8B, 0x46, 0xD1, 0xC4, 0x03, 0xB1, 0x68, 0xEE, 0x64, 0x02, 0x76, 0x90,
                0x65, 0xEB, 0xDA, 0xD7, 0x8E, 0x5E, 0xA3, 0xA0, 0x43, 0xD8, 0xE0, 0x41, 0x37, 0x2E,
                0xDF, 0x14, 0xA1, 0x1E
            ]
        );
        let _ = trace_data("  AccountTxnID:", &account_txn_id.0, DataRepr::AsHex);

        // Trace `AMMID`
        let amm_id = account.ammid(slot).unwrap();
        assert_eq!(
            amm_id.0,
            [
                0xBC, 0x8E, 0x8B, 0x46, 0xD1, 0xC4, 0x03, 0xB1, 0x68, 0xEE, 0x64, 0x02, 0x76, 0x90,
                0x65, 0xEB, 0xDA, 0xD7, 0x8E, 0x5E, 0xA3, 0xA0, 0x43, 0xD8, 0xE0, 0x41, 0x37, 0x2E,
                0xDF, 0x14, 0xA1, 0x1E
            ]
        );
        let _ = trace_data("  AMMID:", &amm_id.0, DataRepr::AsHex);

        // Trace the `Balance`
        let balance = match account.balance(slot).unwrap() {
            TokenAmount::XRP { num_drops } => num_drops,
            TokenAmount::IOU { .. } => {
                panic!("IOU Balance encountered, but should have been XRP.")
            }
            TokenAmount::MPT { .. } => {
                panic!("MPT Balance encountered, but should have been XRP.")
            }
        };
        assert_eq!(balance, 55426479402);
        let _ = trace_num("  Balance of arbitrary Account:", balance);

        // Trace the `BurnedNFTokens`
        let burned_nf_tokens = account.burned_nf_tokens(slot).unwrap();
        assert_eq!(burned_nf_tokens, 20);
        let _ = trace_num("  BurnedNFTokens:", burned_nf_tokens as i64);

        // Trace the `Domain`
        let domain = account.domain(slot).unwrap();
        assert_eq!(&domain.data[..domain.len], &[0xC8, 0xE8, 0xB4, 0x6E]);
        let _ = trace_data("  Domain:", &domain.data[..domain.len], DataRepr::AsHex);

        // Trace the `EmailHash`
        let email_hash = account.email_hash(slot).unwrap();
        assert_eq!(
            email_hash.0,
            [
                0xBC, 0x8E, 0x8B, 0x46, 0xD1, 0xC4, 0x03, 0xB1, 0x68, 0xEE, 0x64, 0x02, 0x76, 0x90,
                0x65, 0xEB
            ]
        );
        let _ = trace_data("  EmailHash:", &email_hash.0, DataRepr::AsHex);

        // Trace the `FirstNFTokenSequence`
        let first_nf_token_sequence = account.first_nf_token_sequence(slot).unwrap();
        assert_eq!(first_nf_token_sequence, 21);
        let _ = trace_num("  FirstNFTokenSequence:", first_nf_token_sequence as i64);

        // Trace the `MessageKey`
        let message_key = account.message_key(slot).unwrap();
        assert_eq!(
            &message_key.data[..message_key.len],
            &[0xC8, 0xE8, 0xB4, 0x6D]
        );
        let _ = trace_data(
            "  MessageKey:",
            &message_key.data[..message_key.len],
            DataRepr::AsHex,
        );

        // Trace the `MintedNFTokens`
        let minted_nf_tokens = account.minted_nf_tokens(slot).unwrap();
        assert_eq!(minted_nf_tokens, 22);
        let _ = trace_num("  MintedNFTokens:", minted_nf_tokens as i64);

        // Trace the `NFTokenMinter`
        let nf_token_minter = account.nf_token_minter(slot).unwrap();
        assert_eq!(
            nf_token_minter.0,
            [
                0xB5, 0xF7, 0x62, 0x79, 0x8A, 0x53, 0xD5, 0x43, 0xA0, 0x14, 0xCA, 0xF8, 0xB2, 0x97,
                0xCF, 0xF8, 0xF2, 0xF9, 0x37, 0xE8
            ]
        );
        let _ = trace_data("  NFTokenMinter:", &nf_token_minter.0, DataRepr::AsHex);

        // Trace the `OwnerCount`
        let owner_count = account.owner_count(slot).unwrap();
        assert_eq!(owner_count, 1);
        let _ = trace_num("  OwnerCount:", owner_count as i64);

        // Trace the `PreviousTxnID`
        let previous_txn_id = account.previous_txn_id(slot).unwrap();
        assert_eq!(
            previous_txn_id.0,
            [
                0xBC, 0x8E, 0x8B, 0x46, 0xD1, 0xC4, 0x03, 0xB1, 0x68, 0xEE, 0x64, 0x02, 0x76, 0x90,
                0x65, 0xEB, 0xDA, 0xD7, 0x8E, 0x5E, 0xA3, 0xA0, 0x43, 0xD8, 0xE0, 0x41, 0x37, 0x2E,
                0xDF, 0x14, 0xA1, 0x1F,
            ]
        );
        let _ = trace_data("  PreviousTxnID:", &previous_txn_id.0, DataRepr::AsHex);

        // Trace the `PreviousTxnLgrSeq`
        let previous_txn_lgr_seq = account.previous_txn_lgr_seq(slot).unwrap();
        assert_eq!(previous_txn_lgr_seq, 95945324);
        let _ = trace_num("  PreviousTxnLgrSeq:", previous_txn_lgr_seq as i64);

        // Trace the `RegularKey`
        let regular_key = account.regular_key(slot).unwrap();
        assert_eq!(
            regular_key.0,
            [
                0x76, 0x1B, 0x18, 0xF3, 0x46, 0x11, 0x2D, 0xFC, 0xD6, 0xA9, 0x95, 0x92, 0x94, 0xE9,
                0xE9, 0x5D, 0x02, 0xDB, 0x7E, 0xE1
            ]
        );
        let _ = trace_data("  RegularKey:", &regular_key.0, DataRepr::AsHex);

        // Trace the `Sequence`
        let sequence = account.sequence(slot).unwrap();
        assert_eq!(sequence, 44196);
        let _ = trace_num("  Sequence:", sequence as i64);

        // Trace the `TicketCount`
        let ticket_count = account.ticket_count(slot).unwrap();
        assert_eq!(ticket_count, 23);
        let _ = trace_num("  TicketCount:", ticket_count as i64);

        // Trace the `TickSize`
        let tick_size = account.tick_size(slot).unwrap();
        assert_eq!(tick_size, 24);
        let _ = trace_num("  TickSize:", tick_size as i64);

        // Trace the `TransferRate`
        let transfer_rate = account.transfer_rate(slot).unwrap();
        assert_eq!(transfer_rate, 1220000000);
        let _ = trace_num("  TransferRate:", transfer_rate as i64);

        // Trace the `WalletLocator`
        let wallet_locator = account.wallet_locator(slot).unwrap();
        assert_eq!(
            &wallet_locator.0,
            &[
                0xBC, 0x8E, 0x8B, 0x46, 0xD1, 0xC4, 0x03, 0xB1, 0x68, 0xEE, 0x64, 0x02, 0x76, 0x90,
                0x65, 0xEB, 0xDA, 0xD7, 0x8E, 0x5E, 0xA3, 0xA0, 0x43, 0xD8, 0xE0, 0x41, 0x37, 0x2E,
                0xDF, 0x14, 0xA1, 0x1D,
            ]
        );
        let _ = trace_data("  WalletLocator:", &wallet_locator.0, DataRepr::AsHex);

        // Trace the `WalletSize`
        let wallet_size = account.wallet_size(slot).unwrap();
        assert_eq!(wallet_size, 25);
        let _ = trace_num("  WalletSize:", wallet_size as i64);

        let _ = trace("}");
        let _ = trace("");
    }

    let _ = trace("$$$$$ WASM EXECUTION COMPLETE $$$$$");
    -1 // <-- Don't finish the escrow; this example merely traces.
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

const EXPECTED_ESCROW_CONDITION: [u8; 32] = [
    0xA0, 0x25, 0x80, 0x20, 0xA8, 0x2A, 0x88, 0xB2, 0xDF, 0x84, 0x3A, 0x54, 0xF5, 0x87, 0x72, 0xE4,
    0xA3, 0x86, 0x18, 0x66, 0xEC, 0xDB, 0x41, 0x57, 0x64, 0x5D, 0xD9, 0xAE, 0x52, 0x8C, 0x1D, 0x3A,
];

/// Represents rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn
const EXPECTED_CURRENT_ESCROW_ACCOUNT_ID: [u8; 20] = [
    0x4B, 0x4E, 0x9C, 0x06, 0xF2, 0x42, 0x96, 0x07, 0x4F, 0x7B, 0xC4, 0x8F, 0x92, 0xA9, 0x79, 0x16,
    0xC6, 0xDC, 0x5E, 0xA9,
];

const EXPECTED_CURRENT_ESCROW_CREDENTIAL1: [u8; 32] = [
    0x0A, 0xBA, 0x05, 0xA3, 0x49, 0x49, 0xF2, 0xCE, 0xD4, 0x10, 0x25, 0x91, 0x4F, 0xC4, 0xF2, 0x67,
    0x88, 0x3F, 0x1D, 0x38, 0x8A, 0x65, 0x45, 0xAF, 0xB4, 0x86, 0x34, 0x66, 0xFA, 0xA6, 0xF2, 0x8C,
];

const EXPECTED_CURRENT_ESCROW_CREDENTIAL2: [u8; 32] = [
    0xD0, 0xA0, 0x63, 0xDE, 0xE0, 0xB0, 0xEC, 0x95, 0x22, 0xCF, 0x35, 0xCD, 0x55, 0x77, 0x1B, 0x5D,
    0xCA, 0xFA, 0x19, 0xA1, 0x33, 0xEE, 0x46, 0xA0, 0x29, 0x5E, 0x4D, 0x08, 0x9A, 0xF8, 0x64, 0x38,
];

const EXPECTED_CURRENT_ESCROW_CREDENTIAL3: [u8; 32] = [
    0xD2, 0xEF, 0xD3, 0x85, 0x89, 0x60, 0x9A, 0xE5, 0x70, 0xD1, 0x7E, 0x99, 0x57, 0xCE, 0x60, 0x02,
    0xE7, 0x64, 0xA6, 0x3E, 0xE6, 0x6F, 0xE8, 0xCA, 0xA2, 0x76, 0x89, 0x76, 0xAB, 0xD6, 0x0B, 0xFF,
];
