/// Represents an amount of XRP in Drops.
#[derive(Copy, Debug, PartialEq, Eq, Clone, Ord, PartialOrd)]
pub struct XrpAmount(pub u64);

impl XrpAmount {
    pub fn new(drops: u64) -> Self {
        Self(drops)
    }
}
impl Default for XrpAmount {
    fn default() -> Self {
        Self(0u64)
    }
}

impl From<u64> for XrpAmount {
    fn from(value: u64) -> Self {
        XrpAmount(value)
    }
}

impl From<XrpAmount> for [u8; 8] {
    fn from(value: XrpAmount) -> Self {
        let value: u64 = value.0;
        value.to_le_bytes()
    }
}
