use core::fmt::Display;

/// Represents an amount of XRP in Drops.

#[derive(Copy, Default, Debug, PartialEq, Eq, Clone, Ord, PartialOrd)]
pub struct XrpAmount(pub u64);

impl XrpAmount {
    pub fn new(drops: u64) -> Self {
        Self(drops)
    }
}

impl Display for XrpAmount {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u64> for XrpAmount {
    fn from(value: u64) -> Self {
        XrpAmount(value)
    }
}

impl From<XrpAmount> for [u8; 8] {
    fn from(value: XrpAmount) -> Self {
        // 1. Cast the enum variant `self` to its underlying i16 value.
        let value: u64 = value.0;
        // let value: i32 = value as i32;

        // 2. Convert the u64 value into a fixed-size byte array ([u8; 8]). XRP will never exceed
        // this value
        let bytes_array: [u8; 8] = value.to_be_bytes();

        // 3. Convert the byte array into a Vec<u8>.
        //    .to_vec() allocates a new Vec and copies the bytes.
        bytes_array
    }
}
