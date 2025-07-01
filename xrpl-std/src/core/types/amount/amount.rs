use crate::core::types::amount::token_amount::TokenAmount;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub enum Amount {
    /// Represents an XRP amount
    XRP {
        num_drops: u64, // Raw u64 can be used directly (drops)
        is_positive: bool,
    },
    /// Represents an IOU amount
    IOU {
        exponent: u8,  // 8 bits
        mantissa: u64, // 54 bits
        is_positive: bool,
    },
    /// Represents an MPT amount
    MPT { num_units: u64, is_positive: bool },
    /// Represents an unknown or invalid amount
    UNKNOWN,
}

pub const AMOUNT_SIZE: usize = 9;

impl From<[u8; 9]> for Amount {
    fn from(bytes: [u8; 9]) -> Self {
        Amount::from_bytes(bytes)
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

impl Amount {
    /// Creates an Amount from raw bytes based on the bit pattern in the first byte
    ///
    /// The byte layout follows these patterns:
    /// - XRP: [0/type][1/sign][0/is-mpt][4/reserved][57/value] --> 8 total bytes
    /// - MPT: [0/type][1/sign][1/is-mpt][5/reserved][64/value] --> 9 total bytes
    /// - IOU: [1/type][1/sign][8/exponent][54/mantissa]  --> 8 total bytes
    pub fn from_bytes(bytes: [u8; 9]) -> Self {
        let byte0 = bytes[0]; // Get the first byte for flag extraction

        // Extract flags using bitwise operations
        let type_bit = byte0 & 0x80 & 1; // Bit 7 (Most Significant Bit)
        let is_xrp_or_mpt = type_bit == 0;
        let is_iou = !is_xrp_or_mpt; // type_bit == 1

        let is_positive: bool = byte0 & 0x40 & 1 == 1; // Bit 6

        let is_xrp: bool = byte0 & 0x20 & 1 == 0; // Bit 5 (only used if type_bit is 0)
        let is_mpt: bool = byte0 & 0x20 & 1 == 1; // Bit 5 (only used if type_bit is 0)

        // Based on the extracted flags, determine the type of amount
        if is_xrp_or_mpt {
            if is_xrp {
                // XRP amount: [0/type][1/sign][0/is-mpt][4/reserved][57/value]
                let mut value_bytes = [0u8; 8];
                value_bytes.copy_from_slice(&bytes[1..9]);
                let value = u64::from_be_bytes(value_bytes);

                Amount::XRP {
                    num_drops: value,
                    is_positive,
                }
            } else if is_mpt {
                // MPT amount: [0/type][1/sign][1/is-mpt][5/reserved][64/value]
                let mut value_bytes = [0u8; 8];
                value_bytes.copy_from_slice(&bytes[1..9]);
                let value = u64::from_be_bytes(value_bytes);

                Amount::MPT {
                    num_units: value,
                    is_positive,
                }
            } else {
                // This should never happen with the current bit patterns, but included for
                // completeness
                Amount::UNKNOWN
            }
        } else if is_iou {
            // <-- The IOU case.
            // IOU amount: [1/type][1/sign][8/exponent][54/mantissa]
            // Extract the 8-bit exponent from the second byte

            // Extract the 8-bit exponent: last 6 bits of the first byte + first 2 bits of the second byte
            let exponent = ((bytes[0] & 0x3F) << 2) | ((bytes[1] & 0xC0) >> 6);

            // Extract the 54-bit mantissa from the remaining bytes
            // The mantissa starts from the last 6 bits of the second byte
            // and continues through the remaining bytes

            // Create a buffer for the mantissa (8 bytes for u64)
            let mut mantissa_bytes = [0u8; 8];

            // Copy bytes 1-8 into positions 0-7 of the mantissa buffer
            mantissa_bytes[0..7].copy_from_slice(&bytes[1..8]);

            // Clear the top 2 bits of the first byte of the mantissa (which are part of the
            // exponent) and keep only the last 6 bits
            mantissa_bytes[1] = mantissa_bytes[1] & 0x3F;

            // Convert to u64 and ensure we only use 54 bits total
            let mantissa = u64::from_be_bytes(mantissa_bytes) & 0x003FFFFFFFFFFFFF;

            Amount::IOU {
                exponent,
                mantissa,
                is_positive,
            }
        } else {
            // This should never happen with the current bit patterns, but included for
            // completeness
            Amount::UNKNOWN
        }
    }

    /// Extracts XRP amount data, panicking if this is not an XRP amount
    pub fn into_xrp(self) -> (u64, bool) {
        match self {
            Amount::XRP {
                num_drops,
                is_positive,
            } => (num_drops, is_positive),
            _ => panic!("Expected XRP amount, got {:?}", self),
        }
    }

    /// Safely extracts XRP amount data, returning None if this is not an XRP amount
    pub fn as_xrp(self) -> Option<(u64, bool)> {
        match self {
            Amount::XRP {
                num_drops,
                is_positive,
            } => Some((num_drops, is_positive)),
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
        let expected = Amount::XRP {
            num_drops: 0,
            is_positive: true,
        };
        assert_eq!(Amount::from(input), expected);
    }

    #[test]
    fn test_parse_xrp_positive_value() {
        // XRP with a specific value (e.g., 1 XRP = 1_000_000 drops)
        // Value: 1_000_000 (0xF4240 in hex)
        // First byte: 0b0100_0000 (0x40)

        // Create a byte array with 1,000,000 drops
        let val: u64 = 1_000_000;
        let value_bytes = val.to_le_bytes();

        let mut input = [0u8; 9];
        input[0] = 0x40; // XRP positive flag
        input[1..9].copy_from_slice(&value_bytes);

        let expected = Amount::XRP {
            num_drops: val,
            is_positive: true,
        };

        assert_eq!(Amount::from(input), expected);
    }

    #[test]
    fn test_parse_xrp_negative_value() {
        // XRP with a negative sign
        // First byte: 0b0000_0000 (0x00)
        // Value: 1,000,000

        let val: u64 = 1_000_000;
        let value_bytes = val.to_le_bytes();

        let mut input = [0u8; 9];
        input[0] = 0x00; // XRP negative flag
        input[1..9].copy_from_slice(&value_bytes);

        let expected = Amount::XRP {
            num_drops: val,
            is_positive: false,
        };

        assert_eq!(Amount::from(input), expected);
    }

    #[test]
    fn test_parse_xrp_max_value() {
        // Test with the maximum u64 value
        let max_u64_val = u64::MAX;
        let mut input = [0u8; 9];
        input[0] = 0x40; // XRP positive flag
        input[1..9].copy_from_slice(&max_u64_val.to_le_bytes());

        let expected = Amount::XRP {
            num_drops: max_u64_val,
            is_positive: true,
        };

        assert_eq!(Amount::from(input), expected);
    }
    #[test]
    fn test_parse_iou_zero() {
        // Example IOU: [1/type][1/sign][8/exponent][54/mantissa]
        // First byte: 0b1100_0000 (0xC0)
        // Exponent: 0, Mantissa: 0
        let input: [u8; 9] = [0xC0, 0, 0, 0, 0, 0, 0, 0, 0];
        let expected = Amount::IOU {
            exponent: 0,
            mantissa: 0,
            is_positive: true,
        };
        assert_eq!(Amount::from(input), expected);
    }

    #[test]
    fn test_parse_iou_with_values() {
        // IOU with exponent = 5, mantissa = 12345
        // First byte: 0b1100_0000 (0xC0, flags for IOU positive)
        // Exponent: 5 (0x05)
        // Mantissa: 12345 (0x3039 in hex)

        // Create the input bytes
        let mut input = [0u8; 9];
        input[0] = 0xC0; // IOU positive flag
        input[1] = 5; // Exponent

        // Set the mantissa in the remaining 7 bytes (54 bits)
        let mantissa: u64 = 12345;
        let mantissa_bytes = mantissa.to_le_bytes();
        // Copy the mantissa bytes, but ensure we only use 54 bits (not the full 64)
        input[2..9].copy_from_slice(&mantissa_bytes[1..8]);

        let expected = Amount::IOU {
            exponent: 5,
            mantissa: mantissa & 0x003FFFFFFFFFFFFF, // Ensure only 54 bits are used
            is_positive: true,
        };

        assert_eq!(Amount::from(input), expected);
    }

    #[test]
    fn test_parse_iou_negative() {
        // IOU with negative sign
        // First byte: 0b1000_0000 (0x80, flags for IOU negative)

        let mut input = [0u8; 9];
        input[0] = 0x80; // IOU negative flag
        input[1] = 3; // Exponent

        // Set the mantissa
        let mantissa: u64 = 9876;
        let mantissa_bytes = mantissa.to_le_bytes();
        input[2..9].copy_from_slice(&mantissa_bytes[1..8]);

        let expected = Amount::IOU {
            exponent: 3,
            mantissa: mantissa & 0x003FFFFFFFFFFFFF,
            is_positive: false,
        };

        assert_eq!(Amount::from(input), expected);
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
        input[1..9].copy_from_slice(&val.to_le_bytes());

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
        input[1..9].copy_from_slice(&val.to_le_bytes());

        let expected = Amount::MPT {
            num_units: val,
            is_positive: false,
        };

        assert_eq!(Amount::from(input), expected);
    }

    #[test]
    fn test_parse_unknown_type() {
        // An invalid combination of flags that should result in UNKNOWN
        // First byte: 0b1010_0000 (0xA0)
        let input: [u8; 9] = [0xA0, 0, 0, 0, 0, 0, 0, 0, 0];
        let expected = Amount::UNKNOWN;
        assert_eq!(Amount::from(input), expected);
    }
}
