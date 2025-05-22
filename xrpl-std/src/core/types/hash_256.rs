/// Represents a 256-bit hash (like transaction ID)
#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Hash256(pub [u8; 32]);

// Implement From<[u8; 32]> to create Hash256 from the array type
impl From<[u8; 32]> for Hash256 {
    fn from(bytes: [u8; 32]) -> Self {
        Self(bytes) // Access private field legally here
    }
}
