use sha2::{Digest, Sha512};

pub const HASH256_LEN: usize = 32;
pub type Hash256 = Vec<u8>;

#[repr(u16)]
#[allow(dead_code)]
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LedgerNameSpace {
    Account = b'a' as u16,
    DirNode = b'd' as u16,
    TrustLine = b'r' as u16,
    Offer = b'o' as u16,
    OwnerDir = b'O' as u16,
    BookDir = b'B' as u16,
    SkipList = b's' as u16,
    Escrow = b'u' as u16,
    Amendments = b'f' as u16,
    FeeSettings = b'e' as u16,
    Ticket = b'T' as u16,
    SignerList = b'S' as u16,
    XrpPaymentChannel = b'x' as u16,
    Check = b'C' as u16,
    DepositPreauth = b'p' as u16,
    DepositPreauthCredentials = b'P' as u16,
    NegativeUnl = b'N' as u16,
    NftokenOffer = b'q' as u16,
    NftokenBuyOffers = b'h' as u16,
    NftokenSellOffers = b'i' as u16,
    Amm = b'A' as u16,
    Bridge = b'H' as u16,
    XchainClaimId = b'Q' as u16,
    XchainCreateAccountClaimId = b'K' as u16,
    Did = b'I' as u16,
    Oracle = b'R' as u16,
    MptokenIssuance = b'~' as u16,
    Mptoken = b't' as u16,
    Credential = b'D' as u16,
    PermissionedDomain = b'm' as u16,

    #[deprecated]
    Contract = b'c' as u16,
    #[deprecated]
    Generator = b'g' as u16,
    #[deprecated]
    Nickname = b'n' as u16,
}

pub fn sha512_half(data: &[u8]) -> Hash256 {
    let mut hasher = Sha512::new();
    hasher.update(&data);
    let result = hasher.finalize();
    result[..32].to_vec()
}

pub fn index_hash(space: LedgerNameSpace, args: &[u8]) -> Hash256 {
    let mut data = Vec::with_capacity(2 + args.len());
    data.extend_from_slice(&(space as u16).to_le_bytes());
    data.extend_from_slice(args);
    sha512_half(&data)
}
