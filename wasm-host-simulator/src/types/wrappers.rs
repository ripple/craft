// This crate exists to get around the "orphan rule" in Rust, which disallows implenting traits for
// foreign members.

use xrpl_std_lib::core::amount::Amount;
use xrpl_std_lib::core::types::Hash256;

pub struct AmountWrapper(pub Amount);

impl From<AmountWrapper> for Vec<u8> {
    fn from(wrapper: AmountWrapper) -> Self {
        match wrapper.0 {
            Amount::Xrp(amount) => {
                // 1. Cast the enum variant `self` to its underlying i16 value.
                let value: u64 = amount.0;
                // let value: i32 = value as i32;

                // 2. Convert the u64 value into a fixed-size byte array ([u8; 8]). XRP will never exceed
                // this value
                let bytes_array: [u8; 8] = value.to_be_bytes();

                // 3. Convert the byte array into a Vec<u8>.
                //    .to_vec() allocates a new Vec and copies the bytes.
                bytes_array.to_vec()
            }
        }
    }
}
