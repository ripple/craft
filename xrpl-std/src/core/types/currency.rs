//! Represents an XRPL currency (160 bits)

pub const CURRENCY_SIZE: usize = 20;
pub const STANDARD_CURRENCY_SIZE: usize = 3; // For standard currencies like USD, EUR, etc.

#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Currency(pub [u8; CURRENCY_SIZE]);

// Implement From<[u8; 20]> to create Currency from the array type
impl From<[u8; CURRENCY_SIZE]> for Currency {
    fn from(bytes: [u8; CURRENCY_SIZE]) -> Self {
        Self(bytes) // Access private field legally here
    }
}

// Implement From<[u8; 3]> to create Currency from the standard currency array type
impl From<[u8; STANDARD_CURRENCY_SIZE]> for Currency {
    fn from(bytes: [u8; STANDARD_CURRENCY_SIZE]) -> Self {
        let mut arr = [0u8; CURRENCY_SIZE];
        arr[12..15].copy_from_slice(&bytes);
        Self(arr)
    }
}
