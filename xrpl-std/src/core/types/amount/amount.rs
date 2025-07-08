use crate::core::types::amount::opaque_float::OpaqueFloat;
use crate::core::types::amount::token_amount::TokenAmount;
use crate::host;
use crate::host::Error::InternalError;
use crate::host::trace::trace_num;

pub const MAX_XRP: i64 = 100 * 1_000_000_000;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub enum Amount {
    /// Represents an XRP amount
    XRP {
        /// Design decision note: Per the pattern in `Amount`, we considered having this be an
        /// unsigned u64 and adding an `is_positve` boolean to this variant. However, we decided to
        /// break that patter and instead use an i64 here for two reasonse. First, this allows
        /// simple math like `add`, `sub`, etc. to be performed in WASM without having to check for
        /// negative values. Second, the total supply of XRP is capped at
        /// 100B XRP (100B * 1M Drops), which fits just fine into an i64
        num_drops: i64,
    },
    /// Represents an IOU amount
    IOU {
        opaque_float: OpaqueFloat,
        is_positive: bool,
    },
    /// Represents an MPT amount
    MPT { num_units: u64, is_positive: bool },
}

pub const AMOUNT_SIZE: usize = 9;

impl From<[u8; 9]> for Amount {
    fn from(bytes: [u8; 9]) -> Self {
        Amount::from_bytes(bytes).unwrap_or_else(|error| {
            let _ = trace_num("Invalid bytes for Amount", error.code() as i64);
            panic!("Invalid bytes for Amount")
        })
    }
}

impl From<TokenAmount> for Amount {
    fn from(token_amount: TokenAmount) -> Self {
        match token_amount {
            TokenAmount::XRP { amount, .. } => amount,
            TokenAmount::IOU { amount, .. } => amount,
            TokenAmount::MPT { amount, .. } => amount,
        }
    }
}

const MASK_57_BIT: u64 = 0x01FFFFFFFFFFFFFFu64;

impl Amount {
    /// Creates an Amount from raw bytes based on the bit pattern in the first byte
    ///
    /// The byte layout follows these patterns:
    /// - XRP: [0/type][1/sign][0/is-mpt][4/reserved][57/value] --> 8 total bytes
    /// - MPT: [0/type][1/sign][1/is-mpt][5/reserved][64/value] --> 9 total bytes
    /// - IOU: [1/type][1/sign][8/exponent][54/mantissa]  --> 8 total bytes
    pub fn from_bytes(bytes: [u8; 9]) -> Result<Self, host::Error> {
        // All Amount bytes must be 9 bytes to accommodate MPT.
        if bytes.len() != 9 {
            return Err(InternalError);
        }

        let byte0 = bytes[0]; // Get the first byte for flag extraction

        // Extract flags using bitwise operations

        let is_iou = byte0 & 0x80 == 0x80; // Bit 7 (Most Significant Bit)
        let is_xrp_or_mpt = !is_iou;
        let is_xrp: bool = byte0 & 0x20 == 0x00; // Bit 5 (only used if type_bit is 0)

        let is_positive: bool = byte0 & 0x40 == 0x40; // Bit 6

        // Based on the extracted flags, determine the type of amount
        if is_xrp_or_mpt {
            if is_xrp {
                // Despite have 9 bytes, XRP values only use the first 8. So, we can copy those
                // into a new buffer and then operate from there.
                let value_bytes: [u8; 8] = bytes[0..8].try_into().unwrap();

                // XRP amount: [0/type][1/sign][0/is-mpt][4/reserved][57/value]

                // For XRP, we need to handle the first byte specially to mask out the flag bits
                // and then use the remaining 7 bytes as is.
                let value = u64::from_be_bytes(value_bytes) & MASK_57_BIT;

                Ok(Amount::XRP {
                    num_drops: match is_positive {
                        true => value as i64,
                        false => value as i64 * -1,
                    },
                })
            }
            // is_mpt
            else {
                // MPT amounts are encoded into 9 bytes, with the actual unit value being in the
                // final 8 bytes.

                // MPT amount: [0/type][1/sign][1/is-mpt][5/reserved][64/value]
                let value_bytes: [u8; 8] = bytes[1..9].try_into().unwrap();
                let value = u64::from_be_bytes(value_bytes);

                Ok(Amount::MPT {
                    num_units: value,
                    is_positive,
                })
            }
        }
        // is_iou
        else {
            // Despite have 9 bytes, IOU amounts only use the first 8 bytes. So, we can copy those
            // into a new buffer and then operate from there.
            let value_bytes: [u8; 8] = bytes[0..8].try_into().unwrap();

            // IOU amount: [1/type][1/sign][8/exponent][54/mantissa]
            let opaque_float: OpaqueFloat = value_bytes.into();
            Ok(Amount::IOU {
                opaque_float,
                is_positive,
            })
        }
    }

    /// Extracts XRP amount data, panicking if this is not an XRP amount
    pub fn into_xrp(self) -> i64 {
        match self {
            Amount::XRP { num_drops } => num_drops,
            _ => panic!("Expected XRP amount, got {:?}", self),
        }
    }

    /// Safely extracts XRP amount data, returning None if this is not an XRP amount
    pub fn as_xrp(self) -> Option<i64> {
        match self {
            Amount::XRP { num_drops } => Some(num_drops),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to create a byte array from a slice
    // fn bytes_from_slice(slice: &[u8]) -> [u8; 9] {
    //     let mut arr = [0u8; 9];
    //     arr[..slice.len()].copy_from_slice(slice);
    //     arr
    // }

    #[test]
    fn test_parse_xrp_zero() {
        // Example XRP: [0/type][1/sign][0/is-mpt][4/reserved][57/value]
        // First byte: 0b0100_0000 (0x40)
        // Value: 0
        let input: [u8; 9] = [0x40, 0, 0, 0, 0, 0, 0, 0, 0];
        let expected = Amount::XRP { num_drops: 0 };
        assert_eq!(Amount::from(input), expected);
    }

    #[test]
    fn test_parse_xrp_positive_value() {
        // Test with the maximum u64 value
        let mut input = [0u8; 9];

        // Use BigEndian for more obvious test creation.
        // Max XRP
        let num_drops: i64 = 1_000_000;
        let num_drops_bytes: [u8; 8] = num_drops.to_be_bytes();

        // Set the first byte to XRP positive flag
        input[0] = 0x40;

        // Copy the last 57 bits by masking and copying the bytes
        // Set the XRP positive flag (0x40) and preserve the last bit for the 57-bit value
        input[0] = 0x40 | (num_drops_bytes[0] & 0x1F); // Keep only the last 5 bits from the value
        input[1..8].copy_from_slice(&num_drops_bytes[1..8]);

        let expected = Amount::XRP {
            num_drops: 1_000_000,
        };
        assert_eq!(Amount::from(input), expected);
    }

    #[test]
    fn test_parse_xrp_negative_value() {
        // Test with the maximum u64 value
        let mut input = [0u8; 9];

        // XRP with a negative sign
        let num_drops: i64 = -1_000_000;
        // The bytes of the negative number must be positive (absolute value) because the negative
        // indicator for an encoded amount is not part of the number.
        let num_drops_bytes: [u8; 8] = (num_drops * -1).to_be_bytes();

        // Set the first byte to XRP positive flag
        input[0] = 0x40;

        // Set the XRP negative flag (0x00) and preserve the last bit for the 57-bit value
        // Keep only the last bits from the first byte
        input[0] = 0x00 | (num_drops_bytes[0] & 0x01);
        input[1..8].copy_from_slice(&num_drops_bytes[1..8]);

        let expected = Amount::XRP { num_drops };
        assert_eq!(Amount::from(input), expected);
    }

    #[test]
    fn test_parse_xrp_max_value() {
        // Test with the maximum u64 value
        let mut input = [0u8; 9];

        // Use BigEndian for more obvious test creation.
        // Max XRP
        let num_drops: i64 = MAX_XRP;
        let num_drops_bytes: [u8; 8] = num_drops.to_be_bytes();

        // Set the first byte to XRP positive flag
        input[0] = 0x40;

        // Copy the last 57 bits by masking and copying the bytes
        // Set the XRP positive flag (0x40) and preserve the last bit for the 57-bit value
        input[0] = 0x40 | (num_drops_bytes[0] & 0x1F); // Keep only the last 5 bits from the value
        input[1..8].copy_from_slice(&num_drops_bytes[1..8]);

        let expected = Amount::XRP { num_drops: MAX_XRP };
        assert_eq!(Amount::from(input), expected);
    }
    #[test]
    fn test_parse_iou_zero() {
        // Example IOU: [1/type][1/sign][8/exponent][54/mantissa]
        // First byte: 0b1100_0000 (0xC0)
        // Exponent: 0, Mantissa: 0
        let input: [u8; 9] = [0xC0, 0, 0, 0, 0, 0, 0, 0, 0];
        let expected = Amount::IOU {
            opaque_float: OpaqueFloat(0xC0_000000_00000000),
            is_positive: true,
        };

        let actual = Amount::from(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_iou_with_values() {
        // IOU with exponent = 5, mantissa = 12345
        let exponent: u8 = 5;
        let mantissa: u64 = 12345;

        // First byte: 0b1100_0000 (0xC0, flags for IOU positive)
        // For exponent 5:
        // - We need to set the last 6 bits of the first byte and first 2 bits of the second byte
        // - 5 = 0b00000101, so we need 0b000001 in the last 6 bits of first byte
        // - and 0b01 in the first 2 bits of second byte

        // Create the input bytes
        let mut input = [0u8; 9];
        // Set the first byte: IOU positive flag (0xC0) with exponent bits
        input[0] = 0xC0 | ((exponent >> 2) & 0x3F); // 5 >> 2 = 1, so this is 0xC1

        // Set the second byte: first 2 bits for exponent, rest will be part of mantissa
        input[1] = (exponent & 0x03) << 6; // 5 & 0x03 = 1, 1 << 6 = 0x40

        let mantissa_bytes = mantissa.to_be_bytes();

        // Copy the mantissa bytes to the input array, preserving the exponent bits in input[1]
        // The mantissa starts from the last 6 bits of input[1], then goes for 6 more bytes.
        input[1] |= mantissa_bytes[0] & 0x3F; // Keep first 2 bits for exponent, set last 6 bits from mantissa
        input[2] = mantissa_bytes[1];
        input[3] = mantissa_bytes[2];
        input[4] = mantissa_bytes[3];
        input[5] = mantissa_bytes[4];
        input[6] = mantissa_bytes[5];
        input[7] = mantissa_bytes[6];
        // input[8] = mantissa_bytes[7]; // <-- Not necessary.

        let mut eight_input_bytes: [u8; 8] = [0u8; 8];
        eight_input_bytes.copy_from_slice(&input[..8]);

        let value = u64::from_be_bytes(eight_input_bytes);

        let expected = Amount::IOU {
            opaque_float: OpaqueFloat(value),
            is_positive: true,
        };

        let actual = Amount::from(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_iou_negative() {
        // IOU with exponent = 5, mantissa = 12345
        let exponent: u8 = 5;
        let mantissa: u64 = 12345;

        // First byte: 0b1000_0000 (0x80, flags for IOU negative)
        // For exponent 5:
        // - We need to set the last 6 bits of the first byte and first 2 bits of the second byte
        // - 5 = 0b00000101, so we need 0b000001 in the last 6 bits of first byte
        // - and 0b01 in the first 2 bits of second byte

        // Create the input bytes
        let mut input = [0u8; 9];
        // Set the first byte: IOU positive flag (0x80) with exponent bits
        input[0] = 0x80 | ((exponent >> 2) & 0x3F); // 5 >> 2 = 1, so this is 0xC1

        // Set the second byte: first 2 bits for exponent, rest will be part of mantissa
        input[1] = (exponent & 0x03) << 6; // 5 & 0x03 = 1, 1 << 6 = 0x40

        let mantissa_bytes = mantissa.to_be_bytes();

        // Copy the mantissa bytes to the input array, preserving the exponent bits in input[1]
        // The mantissa starts from the last 6 bits of input[1], then goes for 6 more bytes.
        input[1] |= mantissa_bytes[0] & 0x3F; // Keep first 2 bits for exponent, set last 6 bits from mantissa
        input[2] = mantissa_bytes[1];
        input[3] = mantissa_bytes[2];
        input[4] = mantissa_bytes[3];
        input[5] = mantissa_bytes[4];
        input[6] = mantissa_bytes[5];
        input[7] = mantissa_bytes[6];
        // input[8] = mantissa_bytes[7]; // <-- Not necessary.

        let mut eight_input_bytes: [u8; 8] = [0u8; 8];
        eight_input_bytes.copy_from_slice(&input[..8]);

        let value = u64::from_be_bytes(eight_input_bytes);

        let expected = Amount::IOU {
            opaque_float: OpaqueFloat(value),
            is_positive: false,
        };

        let actual = Amount::from(input);
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_parse_mpt_zero() {
        // Example MPT: [0/type][1/sign][1/is-mpt][5/reserved][64/value]
        // First byte: 0b0110_0000 (0x60)
        // Value: 0
        let input: [u8; 9] = [0x60, 0, 0, 0, 0, 0, 0, 0, 0];
        let expected = Amount::MPT {
            num_units: 0,
            is_positive: true,
        };
        assert_eq!(Amount::from(input), expected);
    }

    #[test]
    fn test_parse_mpt_positive_value() {
        // MPT with a positive value
        // First byte: 0b0110_0000 (0x60, flags for MPT positive)
        // Value: 500,000

        let val: u64 = 500_000;
        let mut input = [0u8; 9];
        input[0] = 0x60; // MPT positive flag
        // Uses little-endian as the implementation expects
        input[1..9].copy_from_slice(&val.to_be_bytes());

        let expected = Amount::MPT {
            num_units: val,
            is_positive: true,
        };

        assert_eq!(Amount::from(input), expected);
    }

    #[test]
    fn test_parse_mpt_negative_value() {
        // MPT with a negative sign and some value
        // First byte: 0b0010_0000 (0x20, flags for MPT negative)
        // Value: 100

        let val: u64 = 100;
        let mut input = [0u8; 9];
        input[0] = 0x20; // MPT negative flag
        // Use little-endian as the implementation expects
        input[1..9].copy_from_slice(&val.to_be_bytes());

        let expected = Amount::MPT {
            num_units: val,
            is_positive: false,
        };

        assert_eq!(Amount::from(input), expected);
    }
}
