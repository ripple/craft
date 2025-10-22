//! Represents a 160-bit number

pub const UINT160_SIZE: usize = 20;

#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct UInt160(pub [u8; UINT160_SIZE]);

// Implement From<[u8; 20]> to create UInt160 from the array type
impl From<[u8; UINT160_SIZE]> for UInt160 {
    fn from(bytes: [u8; UINT160_SIZE]) -> Self {
        Self(bytes) // Access private field legally here
    }
}

impl UInt160 {
    /// Returns the inner 20 bytes as a reference to the inner array.
    pub fn as_bytes(&self) -> &[u8; UINT160_SIZE] {
        &self.0
    }
}