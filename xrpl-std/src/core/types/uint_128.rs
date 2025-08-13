//! Represents a 128-bit number

pub const UINT128_SIZE: usize = 16;

#[derive(Debug, Eq, PartialEq)]
pub struct UInt128(pub [u8; UINT128_SIZE]);

// Implement From<[u8; 16]> to create UInt128 from the array type
impl From<[u8; UINT128_SIZE]> for UInt128 {
    fn from(bytes: [u8; UINT128_SIZE]) -> Self {
        Self(bytes) // Access private field legally here
    }
}
