use crate::core::error_codes::match_result_code_with_expected_bytes;
use crate::core::types::account_id::AccountID;
use crate::host;
use crate::host::Result;
use crate::host::trace::{DataRepr, trace_data, trace_num};

pub const XRPL_KEYLET_SIZE: usize = 32;
// Type aliases for specific keylets, all currently using the same underlying array type.
pub type KeyletBytes = [u8; XRPL_KEYLET_SIZE];
pub type AccountKeylet = KeyletBytes;
pub type CredentialKeylet = KeyletBytes;
pub type OracleKeylet = KeyletBytes;

/// Generates an account keylet for a given XRP Ledger account.
///
/// Account keylets are used to reference account entries in the XRP Ledger's state data.
/// This function uses the generic `create_keylet_from_host_call` helper to manage the FFI interaction.
///
/// # Arguments
///
/// * `account_id` - Reference to an `AccountID` representing the XRP Ledger account
///
/// # Returns
///
/// * `Result<AccountKeylet>` - On success, returns a 32-byte account keylet.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::account_keylet` function, though the unsafe code is contained
/// within the closure passed to `create_keylet_from_host_call`.
///
/// # Example
///
/// ```rust
///
/// use xrpl_std::core::types::account_id::AccountID;
/// use xrpl_std::core::types::keylets::account_keylet;
/// use xrpl_std::host::trace::{DataRepr, trace_data, trace_num};
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///   let account:AccountID = AccountID::from(
///     *b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3"
///   );
///   match account_keylet(&account){
///     xrpl_std::host::Result::Ok(keylet) => {
///       let _ = trace_data("Generated keylet", &keylet, DataRepr::AsHex);
///     }
///     xrpl_std::host::Result::Err(e) => {
///       let _ = trace_num("Error assembling keylet", e.code() as i64);
///     }
///   }
///   Ok(())
/// }
/// ```
pub fn account_keylet(account_id: &AccountID) -> Result<AccountKeylet> {
    create_keylet_from_host_call(|keylet_buffer_ptr, keylet_buffer_len| unsafe {
        host::account_keylet(
            account_id.0.as_ptr(), // Assuming AccountID is a tuple struct like AccountID(bytes)
            account_id.0.len(),
            keylet_buffer_ptr,
            keylet_buffer_len,
        )
    })
}

/// Generates a credential keylet for a given subject, issuer, and credential type.
///
/// A credential keylet is used to reference credential entries in the XRP Ledger.
///
/// # Arguments
///
/// * `subject` - The AccountID of the subject for whom the credential is issued
/// * `issuer` - The AccountID of the entity issuing the credential
/// * `credential_type` - A byte slice representing the type of credential
///
/// # Returns
///
/// * `Result<CredentialKeylet>` - On success, returns a 32-byte credential keylet.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::credential_keylet` function.
///
/// # Example
///
/// ```rust
/// use xrpl_std::core::types::account_id::AccountID;
/// use xrpl_std::core::types::keylets::credential_keylet;
/// use xrpl_std::host::trace::{DataRepr, trace_data, trace_num};
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let subject: AccountID =
///         AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///     let issuer: AccountID =
///         AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///     let cred_type: &[u8] = b"termsandconditions";
///     match credential_keylet(&subject, &issuer, cred_type) {
///       xrpl_std::host::Result::Ok(keylet) => {
///         let _ = trace_data("Generated keylet", &keylet, DataRepr::AsHex);
///       }
///       xrpl_std::host::Result::Err(e) => {
///         let _ = trace_num("Error assembling keylet", e.code() as i64);
///       }
///     }
///     Ok(())
/// }
/// ```
pub fn credential_keylet(
    subject: &AccountID,
    issuer: &AccountID,
    credential_type: &[u8],
) -> Result<CredentialKeylet> {
    create_keylet_from_host_call(|keylet_buffer_ptr, keylet_buffer_len| unsafe {
        host::credential_keylet(
            subject.0.as_ptr(),
            subject.0.len(),
            issuer.0.as_ptr(),
            issuer.0.len(),
            credential_type.as_ptr(),
            credential_type.len(),
            keylet_buffer_ptr,
            keylet_buffer_len,
        )
    })
}

/// Generates an oracle keylet for a given owner and document ID in the XRP Ledger.
///
/// Oracle keylets are used to reference oracle entries in the XRP Ledger's state data.
/// This function uses the generic `create_keylet_from_host_call` helper to manage the FFI interaction.
///
/// # Arguments
///
/// * `owner` - Reference to an `AccountID` representing the oracle owner's account
/// * `document_id` - An integer identifier for the oracle document
///
/// # Returns
///
/// * `Result<OracleKeylet>` - On success, returns a 32-byte oracle keylet.
///   On failure, returns an `Error` with the corresponding error code.
///
/// # Safety
///
/// This function makes unsafe FFI calls to the host environment through
/// the `host::oracle_keylet` function, though the unsafe code is contained
/// within the closure passed to `create_keylet_from_host_call`.
///
/// # Example
///
/// ```rust
/// use xrpl_std::core::types::account_id::AccountID;
///use xrpl_std::core::types::keylets::oracle_keylet;
///use xrpl_std::host::trace::{DataRepr, trace_data, trace_num};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///   let owner: AccountID =
///       AccountID::from(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
///   let document_id = 12345;
///   match oracle_keylet(&owner, document_id) {
///     xrpl_std::host::Result::Ok(keylet) => {
///       let _ = trace_data("Generated keylet", &keylet, DataRepr::AsHex);
///     }
///     xrpl_std::host::Result::Err(e) => {
///       let _ = trace_num("Error assembling keylet", e.code() as i64);
///     }
///   }
///   Ok(())
///}
/// ```
pub fn oracle_keylet(owner: &AccountID, document_id: i32) -> Result<OracleKeylet> {
    create_keylet_from_host_call(|keylet_buffer_ptr, keylet_buffer_len| unsafe {
        host::oracle_keylet(
            owner.0.as_ptr(),
            owner.0.len(),
            document_id,
            keylet_buffer_ptr,
            keylet_buffer_len,
        )
    })
}

/// Creates an Oracle keylet with panic-on-failure behavior for safe Oracle object lookups.
///
/// This function wraps the `oracle_keylet` function and provides guaranteed success by panicking
/// if the keylet creation fails. It's designed for use cases where Oracle keylet creation
/// is expected to always succeed, and failure indicates a critical error condition.
///
/// # Arguments
///
/// * `owner` - A reference to the `AccountID` that owns the Oracle object
/// * `document_id` - The document identifier for the specific Oracle instance
///
/// # Returns
///
/// Returns an `OracleKeylet` that can be used to locate and access the Oracle object
/// in the ledger.
///
/// # Panics
///
/// This function will panic if:
/// - The underlying `oracle_keylet` function fails to create a valid keylet
/// - Invalid account ID or document ID parameters are provided
///
/// When a panic occurs, the function will:
/// - Log the account ID as hexadecimal using `trace_data`
/// - Log the document ID using `trace_num`
/// - Panic with a descriptive error message including the error details
///
/// # Safety
///
/// This function is marked as "safe" because it guarantees a valid keylet return value
/// or program termination. Use this when Oracle keylet creation failure should be
/// treated as an unrecoverable error.
///
/// # Example
///
/// ```rust
/// use crate::xrpl_std::core::types::account_id::AccountID;
/// use xrpl_std::core::types::keylets::oracle_keylet;
/// use xrpl_std::host::trace::{trace_data, DataRepr};
/// use xrpl_std::core::types::keylets::oracle_keylet_safe;
/// use xrpl_std::core::types::keylets::OracleKeylet;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///   let owner: AccountID = AccountID::from(
///       *b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3"
///   );
///   let document_id = 12345;
///   let keylet:OracleKeylet = oracle_keylet_safe(&owner, document_id);
///   Ok(())
/// }
/// ```
pub fn oracle_keylet_safe(owner: &AccountID, document_id: i32) -> OracleKeylet {
    let keylet = oracle_keylet(owner, document_id);
    match keylet {
        Result::Ok(keylet) => keylet,
        Result::Err(error) => {
            let _ = trace_data(
                "Failed to get oracle_keylet for account_id=",
                &owner.0,
                DataRepr::AsHex,
            );
            let _ = trace_num(
                "Failed to get oracle_keylet for document_id=",
                document_id as i64,
            );
            core::panic!("Failed to get oracle_keylet (error_code={})", error.code())
        }
    }
}

/// Generic helper function to create a keylet by calling a host function.
///
/// This function handles the common tasks of:
/// - Initializing the keylet output buffer.
/// - Invoking the provided `host_call` closure (which performs the unsafe host FFI call).
/// - Converting the host call's `i32` result code into a `Result<KeyletBytes, Error>`.
///
/// # Arguments
///
/// * `host_call`: A closure that takes a mutable pointer to the output buffer (`*mut u8`)
///   and its length (`usize`), performs the specific host FFI call, and returns an `i32` status
///   code.
fn create_keylet_from_host_call<F>(host_call: F) -> Result<KeyletBytes>
where
    F: FnOnce(*mut u8, usize) -> i32,
{
    let mut keylet_buffer: KeyletBytes = [0; XRPL_KEYLET_SIZE];
    let result_code: i32 = host_call(keylet_buffer.as_mut_ptr(), keylet_buffer.len());

    match_result_code_with_expected_bytes(result_code, XRPL_KEYLET_SIZE, || keylet_buffer)
}
