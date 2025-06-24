//! # MPT Amount Module
//!
/// Represents the amount of MPT in the core units of the MPT issuance (similar to how XRP is
/// represented in drops).
///
/// This struct wraps a 64-bit unsigned integer that holds the amount in the smallest
/// divisible unit of a Multi-Purpose Token.
///
/// # Examples
/// ```rust
/// use mpt_amount::MptAmount;
///
/// // Create from raw units
/// let amount = MptAmount::new(1000000);
///
/// // Create from u64
/// let amount2: MptAmount = 500000u64.into();
///
/// // Default is zero
/// let zero_amount = MptAmount::default();
/// assert_eq!(zero_amount, MptAmount::new(0));
///
/// // Convert to byte array for serialization
/// let bytes: [u8; 8] = amount.into();
/// ```
///
/// # Precision
/// The underlying value represents the amount in the MPT's smallest unit. For example,
/// if an MPT has 6 decimal places, then a value of 1,000,000 represents 1.0 tokens.
///
/// # Serialization
/// The amount can be converted to a big-endian byte array for transmission to a host function.
#[derive(Copy, Debug, PartialEq, Eq, Clone, Ord, PartialOrd)]
pub struct MptAmount(pub u64);

impl MptAmount {
    pub fn new(mpt_units: u64) -> Self {
        Self(mpt_units)
    }
}
impl Default for MptAmount {
    fn default() -> Self {
        Self(0u64)
    }
}

impl From<u64> for MptAmount {
    fn from(value: u64) -> Self {
        MptAmount(value)
    }
}

impl From<MptAmount> for [u8; 8] {
    fn from(value: MptAmount) -> Self {
        let value: u64 = value.0;
        value.to_le_bytes()
    }
}
