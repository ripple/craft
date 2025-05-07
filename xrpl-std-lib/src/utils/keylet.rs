use crate::core::types::account_id::AccountID;

/// Entries in the ledger are located using 256-bit locators. The locators are  calculated using a
/// wide range of parameters specific to the entry whose locator we are calculating (e.g. an
/// account's locator is derived from the account's address, whereas the locator for an offer is
/// derived from the account and the offer sequence).
///
/// To enhance type safety during lookup and make the code more robust, XRPLd uses keylets, which
/// contain not only the locator of the object but also the type of the object being referenced.
pub struct Keylet(pub [u8; 34]);

/// Compute a `Keylet` for the specified `AccountID`.
pub fn account_keylet(account: AccountID) -> Keylet {
    
    
    // TODO: FIXME.
    return Keylet([0u8; 34]);
}
