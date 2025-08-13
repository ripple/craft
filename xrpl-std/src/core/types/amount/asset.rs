use crate::core::types::account_id::AccountID;
use crate::core::types::amount::currency_code::CurrencyCode;
use crate::core::types::amount::mpt_id::MptId;

/// Struct to represent an Asset of type XRP. Exists so that other structs can restrict type
/// information to XRP in their declarations (this is not possible with just the `Asset` enum below).
#[derive(Debug, Eq, PartialEq)]
pub struct XrpAsset {}

/// Defines an asset for IOUs.
#[derive(Debug, Eq, PartialEq)]
pub struct IouAsset {
    issuer: AccountID,
    currency_code: CurrencyCode,
}

/// Struct to represent an Asset of type MPT. Exists so that other structs can restrict type
/// information to XRP in their declarations (this is not possible with just the `Asset` enum below).
#[derive(Debug, Eq, PartialEq)]
pub struct MptAsset {
    mpt_id: MptId,
}

/// Represents an asset withoout a value, such as reading `Asset1` and `Asset2` in AMM ledger
/// objects.
#[derive(Debug, Eq, PartialEq)]
pub enum Asset {
    XRP(XrpAsset),
    IOU(IouAsset),
    MPT(MptAsset),
}
