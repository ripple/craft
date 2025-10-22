pub const UINT192_SIZE: usize = 24;

#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct UInt192(pub [u8; UINT192_SIZE]);

impl UInt192 {
    /// Create a new UInt192 from raw bytes
    pub const fn new(bytes: [u8; UINT192_SIZE]) -> Self {
        Self(bytes)
    }

    /// Get a reference to the raw bytes
    pub const fn as_bytes(&self) -> &[u8; UINT192_SIZE] {
        &self.0
    }

    /// Get a pointer to the raw bytes
    pub const fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }

    /// Get the length in bytes
    pub const fn len(&self) -> usize {
        UINT192_SIZE
    }

    /// Check if all bytes are zero
    pub fn is_zero(&self) -> bool {
        let mut i = 0;
        while i < UINT192_SIZE {
            if self.0[i] != 0 {
                return false;
            }
            i += 1;
        }
        true
    }
}

impl From<[u8; UINT192_SIZE]> for UInt192 {
    fn from(bytes: [u8; UINT192_SIZE]) -> Self {
        Self(bytes)
    }
}

impl Default for UInt192 {
    fn default() -> Self {
        Self([0u8; UINT192_SIZE])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uint192_creation() {
        let bytes = [1u8; UINT192_SIZE];
        let uint = UInt192::new(bytes);
        assert_eq!(uint.as_bytes(), &bytes);
    }

    #[test]
    fn test_uint192_from_bytes() {
        let bytes = [2u8; UINT192_SIZE];
        let uint = UInt192::from(bytes);
        assert_eq!(uint.as_bytes(), &bytes);
    }

    #[test]
    fn test_uint192_zero() {
        let zero = UInt192::default();
        assert!(zero.is_zero());

        let non_zero = UInt192::from([1u8; UINT192_SIZE]);
        assert!(!non_zero.is_zero());
    }

    #[test]
    fn test_uint192_size() {
        let uint = UInt192::default();
        assert_eq!(uint.len(), 24);
    }
}