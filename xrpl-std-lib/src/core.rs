// Represents the type of Ledger Entry.
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// #[repr(u16)]
// pub enum LedgerEntryType {
//     AccountRoot = 0x0061,
//     // Add other ledger entry types as needed...
// }

// Represents the field codes.
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// #[repr(u16)]
// pub enum FieldCode {
//     Account = 1,
//     Sequence = 2,
//     Balance = 3,
//     OwnerCount = 4,
//     PreviousTxnID = 5,
//     PreviousTxnLgrSeq = 6,
//     AccountTxnID = 7,
//     RegularKey = 8,
//     EmailHash = 9,
//     WalletLocator = 10,
//     WalletSize = 11,
//     MessageKey = 12,
//     TransferRate = 13,
//     Domain = 14,
//     TickSize = 15,
//     TicketCount = 16,
//     NFTokenMinter = 17,
//     MintedNFTokens = 18,
//     BurnedNFTokens = 19,
//     FirstNFTokenSequence = 20,
//     AMMID = 21,
//     // Add other field codes as needed...
// }