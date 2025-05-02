use xrpl_std_lib::core::amount::Amount;
use xrpl_std_lib::core::types::account_id::AccountID;
use xrpl_std_lib::core::types::blob::Blob;
use xrpl_std_lib::core::types::credentials::CredentialIDs;
use xrpl_std_lib::core::types::crypto_condition::{Condition, Fulfillment};
use xrpl_std_lib::core::types::hash_256::Hash256;
use xrpl_std_lib::core::types::public_key::PublicKey;
use xrpl_std_lib::core::types::transaction_type::TransactionType;
// Represents the context retrieved by HOOK_SETUP
pub(crate) struct ApplyContext {
    // Example: Assuming 'tx' is a field holding transaction info
    pub tx: EscrowFinish,
    // ... other fields ...
}

// pub trait Transaction<'a>
// where
//     Self: Sized,
// {
//     fn get_transaction_type(&self) -> TransactionType;
// }

// impl Transaction<'_> for EscrowFinish {
//     fn get_transaction_type(&self) -> TransactionType {
//         TransactionType::EscrowFinish
//     }
// }

pub(crate) struct CommonFields {
    pub transaction_id: Hash256,
    pub account_id: AccountID,
    pub transaction_type: TransactionType,
    pub fee: Amount,
    pub sequence: u32,
    pub account_txn_id: Option<Hash256>,
    pub flags: u32,
    pub last_ledger_sequence: Option<u32>,
    pub network_id: Option<u32>,
    pub source_tag: Option<u32>,
    pub signing_pub_key: Option<PublicKey>,
    pub ticket_sequence: Option<u32>,
    pub txn_signature: Option<Blob>,
}

pub(crate) struct EscrowFinish {
    pub common_fields: CommonFields,
    pub owner: AccountID,
    pub offer_sequence: u32,
    pub condition: Option<Condition>,
    pub fulfillment: Option<Fulfillment>,
    pub credential_ids: Option<CredentialIDs>,
}
