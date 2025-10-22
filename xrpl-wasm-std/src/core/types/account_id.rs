//! Account identifiers used throughout XRPL.
//!
//! This type wraps a 20-byte AccountID and is returned by many accessors.
//! See also: <https://xrpl.org/docs/references/protocol/common-fields#accountid-fields>

pub const ACCOUNT_ID_SIZE: usize = 20;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct AccountID(pub [u8; ACCOUNT_ID_SIZE]);

impl From<[u8; ACCOUNT_ID_SIZE]> for AccountID {
    fn from(value: [u8; ACCOUNT_ID_SIZE]) -> Self {
        AccountID(value)
    }
}

impl AccountID {
    /// Convert the AccountID to a hexadecimal representation as bytes
    /// Returns a 40-byte array (2 hex chars per byte)
    pub fn to_hex_bytes(&self) -> [u8; 40] {
        const HEX_CHARS: &[u8; 16] = b"0123456789abcdef";
        let mut result = [0u8; 40];
        
        for (i, &byte) in self.0.iter().enumerate() {
            result[i * 2] = HEX_CHARS[(byte >> 4) as usize];
            result[i * 2 + 1] = HEX_CHARS[(byte & 0x0f) as usize];
        }
        
        result
    }
    
    /// Get a reference to the underlying bytes
    pub const fn as_bytes(&self) -> &[u8; ACCOUNT_ID_SIZE] {
        &self.0
    }
}