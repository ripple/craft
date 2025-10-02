//! Represents a 192-bit number

pub const UINT192_SIZE: usize = 24;

#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct UInt192(pub [u8; UINT192_SIZE]);

// Implement From<[u8; 24]> to create UInt192 from the array type
impl From<[u8; UINT192_SIZE]> for UInt192 {
    fn from(bytes: [u8; UINT192_SIZE]) -> Self {
        Self(bytes) // Access private field legally here
    }
}

impl UInt192 {
    /// Returns the inner 24 bytes as a reference to the inner array.
    pub fn as_bytes(&self) -> &[u8; UINT192_SIZE] {
        &self.0
    }
}