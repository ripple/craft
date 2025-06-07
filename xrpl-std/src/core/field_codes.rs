// TODO: Decided if we prefer this style or sfield.rs
pub const SF_ACCOUNT: i32 = 524289; // 0x80001
pub const SF_TRANSACTION_TYPE: i32 = 65538; // 0x10002
pub const SF_FEE: i32 = 393224; // 0x60008
pub const SF_SEQUENCE: i32 = 131076;
pub const SF_ACCOUNT_TXN_ID: i32 = 327689;
pub const SF_FLAGS: i32 = 131074;
pub const SF_LAST_LEDGER_SEQUENCE: i32 = 131099;
pub const SF_MEMOS: i32 = 983049;
pub const SF_NETWORK_ID: i32 = (2 << 16) + 1;
pub const SF_HASH: i32 = 327937;
pub const SF_LEDGER_ENTRY_TYPE: i32 = 65537; // 0x10001
pub const SF_SIGNERS: i32 = 983043;
pub const SF_SOURCE_TAG: i32 = 131075;
pub const SF_SIGNING_PUB_KEY: i32 = 458755;
pub const SF_TXN_SIGNATURE: i32 = 458756;
pub const SF_TICKET_SEQUENCE: i32 = 131113; // 0x20029

/// EscrowFinish Fields
pub const SF_OWNER: i32 = 524290;
pub const SF_OFFER_SEQUENCE: i32 = 131097;
pub const SF_CONDITION: i32 = 458769;
pub const SF_CREDENTIAL_IDS: i32 = 1245189; // TODO: Check this value.
pub const SF_FULFILLMENT: i32 = 458768;

// Transaction Field Codes
// #[repr(i32)]
// pub enum TransactionFieldCodes {
//     Account = SF_ACCOUNT as i32,
//     TransactionType = SF_TRANSACTION_TYPE as i32,
//     Fee = SF_FEE as i32,
//     Sequence = SF_SEQUENCE as i32,
//     AccountTxnID = SF_ACCOUNT_TXN_ID as i32,
//     Flags = SF_FLAGS as i32,
//     LastLedgerSequence = SF_LAST_LEDGER_SEQUENCE as i32,
//     Memos = SF_MEMOS as i32,
//     NetworkID = SF_NETWORK_ID as i32,
//     Signers = SF_SIGNERS as i32,
//     SourceTag = SF_SOURCE_TAG as i32,
//     SigningPubKey = SF_SIGNING_PUB_KEY as i32,
//     TicketSequence = SF_TICKET_SEQUENCE as i32,
//     TxnSignature = SF_TXN_SIGNATURE as i32,
//     LedgerEntryType = SF_LEDGER_ENTRY_TYPE as i32,
// }

// /// Usage:
// /// ```rust
// /// // match some_escrow_finish_field_code {
// ///  //   TransactionFieldCodes(Account) => ...,
// ///  //   TransactionFieldCodes(TransactionType) => ...,
// ///  //   Owner => ...
// /// //}
// /// ```
// #[repr(i32)]
// pub enum EscrowFinishFieldCodes {
//     TransactionFieldCodes, // For extension without copy/paste.
//     Owner = SF_OWNER as i32,
//     OfferSequence = SF_OFFER_SEQUENCE as i32,
//     Condition = SF_CONDITION as i32,
//     CredentialIDs = SF_CREDENTIAL_IDS as i32,
//     Fulfillment = SF_FULFILLMENT as i32,
// }
