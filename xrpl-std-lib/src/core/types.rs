/// Represents a 256-bit hash (like transaction ID)
// #[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Hash256(pub [u8; 32]);

// Implement From<[u8; 32]> to create Hash256 from the array type
impl From<[u8; 32]> for Hash256 {
    fn from(bytes: [u8; 32]) -> Self {
        Self(bytes) // Access private field legally here
    }
}

// Optional: Implement TryFrom<&[u8]> for safe creation from slices
impl TryFrom<&[u8]> for Hash256 {
    type Error = &'static str; // Example error type

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        if slice.len() == 32 {
            // Convert slice to array (requires an intermediate step or unsafe)
            let mut array = [0u8; 32];
            array.copy_from_slice(slice);
            Ok(Self(array)) // Access private field legally here
        } else {
            Err("Slice must be 32 bytes long")
        }
    }
}

// #[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct AccountID(pub [u8; 20]);
