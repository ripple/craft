pub const XRPL_ACCOUNT_ID_SIZE: usize = 20;
pub const XRPL_NFTID_SIZE: usize = 32;
pub const XRPL_HASH256_SIZE: usize = 32;
pub const XRPL_CONTRACT_DATA_SIZE: usize = 4096; //TODO size??
pub type AccountID = [u8; XRPL_ACCOUNT_ID_SIZE];
pub type NFT = [u8; XRPL_NFTID_SIZE];
pub type Hash256 = [u8; XRPL_HASH256_SIZE];
pub type ContractData = [u8; XRPL_CONTRACT_DATA_SIZE];
// use keylet hash only (i.e. without 2-byte LedgerEntryType) for now.
// TODO Check rippled
pub const XRPL_KEYLET_SIZE: usize = 32;

pub type Keylet = [u8; XRPL_KEYLET_SIZE];
