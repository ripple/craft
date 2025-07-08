use crate::core::types::account_id::AccountID;
use crate::core::types::amount::amount::Amount;
use crate::core::types::amount::currency_code::CurrencyCode;
use crate::core::types::amount::mpt_id::MptId;
use crate::host;
use crate::host::Error::InternalError;
use crate::host::trace::trace_num;

pub const TOKEN_AMOUNT_SIZE: usize = 48;

/// A zero-cost abstraction for XRPL tokens. Tokens conform to the following binary layout:
///
/// ```markdown
///              ┌────────────────────────────────────────────────────────────────────────────┐   
///              │                       XRP Amount (64 bits / 8 bytes)                       │   
///              ├────────────────────────────────────────────────────────────────────────────┤   
///              │                     ┌────────────────────────────────────────────────────┐ │   
///              │ ┌─┐┌─┐┌─┐ ┌─┬─┬─┬─┐ │ ┌────────────────────────────────────────────────┐ │ │   
///              │ │0││1││0│ │0│0│0│0│ │ │                      ...                       │ │ │   
///              │ └─┘└─┘└─┘ └─┴─┴─┴─┘ │ └────────────────────────────────────────────────┘ │ │   
///              │  ▲  ▲  ▲       ▲    │              Integer Drops (57 bits)               │ │   
///              │  │  │  │       │    └────────────────────────────────────────────────────┘ │   
///          ┌───┼──┘  │  └─────┐ └────────────────┐                                          │   
///          │   └─────┼────────┼──────────────────┼──────────────────────────────────────────┘   
///          │         │        │                  │                                              
/// ┌────────────────┐ │ ┌─────────────┐ ┌──────────────────┐                                     
/// │    Type Bit    │ │ │ Is MPT Bit  │ │     Reserved     │                                     
/// │(0=XRP/MPT;1=IOU│ │ │(1=MPT/0=XRP)│ └──────────────────┘                                     
/// └────────────────┘ │ └─────────────┘                                                          
///           ┌────────────────┐                                                                  
///           │    Sign bit    │                                                                  
///           │(1 for positive)│                                                                  
///           └────────────────┘                                                                  
///                                                                                               
///              ┌────────────────────────────────────────────────────────────────────────────┐   
///              │                       MPT Amount (264-bits/33-bytes)                       │   
///              ├────────────────────────────────────────────────────────────────────────────┤   
///              │                       ┌──────────┐ ┌────────────┐ ┌────────────────┐       │   
///              │ ┌─┐┌─┐┌─┐ ┌─┬─┬─┬─┬─┐ │┌────────┐│ │ ┌────────┐ │ │   ┌────────┐   │       │   
///              │ │0││1││1│ │0│0│0│0│0│ ││  ...   ││ │ │  ...   │ │ │   │  ...   │   │       │   
///              │ └─┘└─┘└─┘ └─┴─┴─┴─┴─┘ │└────────┘│ │ └────────┘ │ │   └────────┘   │       │   
///              │  ▲  ▲  ▲       ▲      │  Amount  │ │Sequence Num│ │Issuer AccountID│       │   
///              │  │  │  │       │      │(64 bits) │ │ (32 bits)  │ │   (160 bits)   │       │   
///          ┌───┼──┘  │  └────┐  │      └──────────┘ └────────────┘ └────────────────┘       │   
///          │   └─────┼───────┼──┼───────────────────────────────────────────────────────────┘   
///          │         │       │  └───────────────┐                                               
/// ┌─────────────────┐│┌─────────────┐           │                                               
/// │    Type Bit     │││ Is MPT Bit  │           │                                               
/// │(0=XRP/MPT;1=IOU)│││(1=MPT/0=XRP)│           │                                               
/// └─────────────────┘│└─────────────┘           │                                               
///           ┌────────────────┐        ┌──────────────────┐                                      
///           │    Sign bit    │        │     Reserved     │                                      
///           │(1 for positive)│        └──────────────────┘                                      
///           └────────────────┘                                                                  
///                                                                                               
///                                                                                               
///             ┌────────────────────────────────────────────────────────────────────────────────┐
///             │                         IOU Amount (384-bits/48-bytes)                         │
///             ├────────────────────────────────────────────────────────────────────────────────┤
///             │       ┌─────────────────┐  ┌──────────────┐ ┌──────────────┐ ┌────────────────┐│
///             │ ┌─┐┌─┐│┌─┬─┬─┬─┬─┬─┬─┬─┐│  │┌────────────┐│ │  ┌────────┐  │ │   ┌───────┐    ││
///             │ │1││1│││0│0│0│0│0│0│0│0││  ││    ...     ││ │  │  ...   │  │ │   │  ...  │    ││
///             │ └─┘└─┘│└─┴─┴─┴─┴─┴─┴─┴─┘│  │└────────────┘│ │  └────────┘  │ │   └───────┘    ││
///             │  ▲  ▲ │Exponent (8 Bits)│  │Mantissa Bits │ │Currency Code │ │Issuer AccountID││
///             │  │  │ └─────────────────┘  │  (54 Bits)   │ │  (160 bits)  │ │   (160 bits)   ││
///             │  │  └────────────────┐     └──────────────┘ └──────────────┘ └────────────────┘│
///             │  │                   │                                                         │
///             └──┴───────────────────┴─────────────────────────────────────────────────────────┘
///      ┌──────────────────┐┌──────────────────┐                                                 
///      │     Type Bit     ││     Sign bit     │                                                 
///      │(0=XRP/MPT;1=IOU) ││ (1 for positive) │                                                 
///      └──────────────────┘└──────────────────┘                                                 
/// ```
///
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub enum TokenAmount {
    XRP {
        amount: Amount,
    },
    IOU {
        amount: Amount,
        issuer: AccountID,
        currency_code: CurrencyCode,
    },
    MPT {
        amount: Amount,
        mpt_id: MptId,
    },
}

impl TokenAmount {
    /// Parses a TokenAmount from a byte array.
    ///
    /// The byte array can be one of three formats:
    /// - XRP: 8 bytes
    /// - MPT: 33 bytes
    /// - IOU: 48 bytes
    ///
    /// Returns None if the byte array is not a valid TokenAmount.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, host::Error> {
        if bytes.len() < 9 {
            return Err(InternalError);
        }

        let byte0 = bytes[0]; // Get the first byte for flag extraction

        // Extract flags using bitwise operations
        let is_iou = byte0 & 0x80 == 0x80; // Bit 7 (Most Significant Bit)
        let is_xrp_or_mpt = !is_iou;
        let is_xrp: bool = byte0 & 0x20 == 0x00; // Bit 5 (only used if type_bit is 0)

        if is_xrp_or_mpt {
            if is_xrp {
                if bytes.len() != 9 {
                    return Err(InternalError);
                }

                // Parse the Amount::XRP from the first 8 bytes
                let mut nine_amount_bytes = [0u8; 9];
                nine_amount_bytes[0] = byte0;
                nine_amount_bytes[1..9].copy_from_slice(&bytes[1..9]);

                let amount = Amount::from_bytes(nine_amount_bytes)?;
                match amount {
                    Amount::XRP { .. } => Ok(TokenAmount::XRP { amount }),
                    _ => Err(InternalError),
                }
            }
            // is_mpt
            else {
                // MPT amount: 33 bytes
                if bytes.len() != 33 {
                    return Err(InternalError);
                }

                // Parse the Amount::MPT from the first 9 bytes
                let mut amount_bytes = [0u8; 9];
                amount_bytes.copy_from_slice(&bytes[0..9]);

                // Parse the MptId from the remaining bytes
                let mut mpt_id_bytes = [0u8; 24];
                mpt_id_bytes.copy_from_slice(&bytes[9..33]);
                let mpt_id = MptId::from(mpt_id_bytes);

                let amount = Amount::from_bytes(amount_bytes)?;
                match amount {
                    Amount::MPT { .. } => Ok(TokenAmount::MPT {
                        amount: amount,
                        mpt_id,
                    }),
                    _ => Err(InternalError),
                }
            }
        }
        // is_iou
        else {
            // IOU amount: 48 bytes
            if bytes.len() != 48 {
                return Err(InternalError);
            }

            // Parse the Amount::IOU from the first 9 bytes
            let mut amount_bytes = [0u8; 9];
            amount_bytes.copy_from_slice(&bytes[0..9]);

            // Parse the CurrencyCode from the next 20 bytes
            let mut currency_code_bytes = [0u8; 20];
            currency_code_bytes.copy_from_slice(&bytes[8..28]);
            let currency_code = CurrencyCode::from(currency_code_bytes);

            // Parse the AccountID from the last 20 bytes
            let mut issuer_bytes = [0u8; 20];
            issuer_bytes.copy_from_slice(&bytes[28..48]);
            let issuer = AccountID::from(issuer_bytes);

            let amount = Amount::from_bytes(amount_bytes).unwrap();
            match amount {
                Amount::IOU { .. } => Ok(TokenAmount::IOU {
                    amount,
                    issuer,
                    currency_code,
                }),
                _ => Err(InternalError),
            }
        }
    }
}

impl From<[u8; TOKEN_AMOUNT_SIZE]> for TokenAmount {
    fn from(bytes: [u8; TOKEN_AMOUNT_SIZE]) -> Self {
        // Use the existing from_bytes method with a slice reference
        match Self::from_bytes(&bytes) {
            Ok(token_amount) => token_amount,
            Err(error) => {
                let _ = trace_num("Error parsing token_amount", error.code() as i64);
                panic!("Invalid TokenAmount byte array");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::amount::opaque_float::OpaqueFloat;

    #[test]
    fn test_parse_xrp_amount() {
        // Create a test XRP amount byte array
        // XRP amount: [0/type][1/sign][0/is-mpt][4/reserved][57/value]
        // First byte: 0b0100_0000 (0x40)
        // Value: 1,000,000 (0xF4240 in hex)
        let mut bytes = [0u8; 9];
        bytes[0] = 0x40; // XRP positive flag
        bytes[1..8].copy_from_slice(&1_000_000u64.to_be_bytes()[1..8]);

        // Parse the TokenAmount
        let token_amount = TokenAmount::from_bytes(&bytes).unwrap();

        // Verify it's an XRP amount with the correct value
        match token_amount {
            TokenAmount::XRP { amount: value } => match value {
                Amount::XRP { num_drops: value } => {
                    assert_eq!(value, 1_000_000);
                }
                _ => panic!("Expected Amount::XRP"),
            },
            _ => panic!("Expected TokenAmount::XRP"),
        }
    }

    #[test]
    fn test_parse_mpt_amount() {
        // Create a test MPT amount byte array
        // MPT amount: [0/type][1/sign][1/is-mpt][5/reserved][64/value][32/sequence][160/issuer]
        // First byte: 0b0110_0000 (0x60)
        const VALUE: u64 = 500_000; // 8 bytes
        const SEQUENCE_NUM: u32 = 12345; // 4 bytes
        const ISSUER_BYTES: [u8; 20] = [1u8; 20]; // 20 bytes

        let mut bytes = [0u8; 33];

        // Set the amount bytes
        bytes[0] = 0x60; // MPT positive flag
        bytes[1..9].copy_from_slice(&VALUE.to_be_bytes());

        // Set the MptId bytes
        bytes[9..13].copy_from_slice(&SEQUENCE_NUM.to_be_bytes());
        // Set the Issuer bytes.
        bytes[13..33].copy_from_slice(&ISSUER_BYTES);

        // Parse the TokenAmount
        let token_amount = TokenAmount::from_bytes(&bytes).unwrap();

        // Verify it's an MPT amount with the correct values
        match token_amount {
            TokenAmount::MPT {
                amount: value,
                mpt_id,
            } => match value {
                Amount::MPT {
                    num_units: value,
                    is_positive,
                } => {
                    assert_eq!(value, VALUE);
                    assert!(is_positive);
                    assert_eq!(mpt_id.get_sequence_num(), SEQUENCE_NUM);
                    assert_eq!(mpt_id.get_issuer(), AccountID::from(ISSUER_BYTES));
                }
                _ => panic!("Expected Amount::MPT"),
            },
            _ => panic!("Expected TokenAmount::MPT"),
        }
    }

    #[test]
    fn test_parse_iou_amount() {
        // IOU with exponent = 5, mantissa = 12345
        const EXPONENT: u8 = 5; // 1 byte
        const MANTISSA: u64 = 12345; // 57 bits (so need or 8 bytes)

        // First byte: 0b1100_0000 (0xC0, flags for IOU positive)
        // For exponent 5:
        // - We need to set the last 6 bits of the first byte and first 2 bits of the second byte
        // - 5 = 0b00000101, so we need 0b000001 in the last 6 bits of first byte
        // - and 0b01 in the first 2 bits of second byte

        // Create the input bytes
        let mut input = [0u8; 9];
        // Set the first byte: IOU positive flag (0xC0) with exponent bits
        input[0] = 0xC0 | ((EXPONENT >> 2) & 0x3F); // 5 >> 2 = 1, so this is 0xC1

        // Set the second byte: first 2 bits for exponent, rest will be part of mantissa
        input[1] = (EXPONENT & 0x03) << 6; // 5 & 0x03 = 1, 1 << 6 = 0x40

        let mantissa_bytes = MANTISSA.to_be_bytes();

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

        /////////////////
        // Add the rest of the TokenAmount Fields
        /////////////////

        // Create a test IOU amount byte array
        // IOU amount: [1/type][1/sign][8/exponent][54/mantissa][160/currency][160/issuer]
        // First byte: 0b1100_0000 (0xC0)

        let mut bytes = [0u8; 48];

        bytes[0..8].copy_from_slice(&eight_input_bytes[0..8]);

        // Set the currency code bytes
        const CURRENCY_BYTES: [u8; 20] = [2u8; 20]; // 20 bytes
        bytes[8..28].copy_from_slice(&CURRENCY_BYTES);

        // Set the issuer bytes
        const ISSUER_BYTES: [u8; 20] = [3u8; 20]; // 20 bytes
        bytes[28..48].copy_from_slice(&ISSUER_BYTES);

        // Parse the TokenAmount
        let token_amount = TokenAmount::from_bytes(&bytes).unwrap();

        // Verify it's an IOU amount with the correct values
        match token_amount {
            TokenAmount::IOU {
                amount: value,
                issuer,
                currency_code,
            } => match value {
                Amount::IOU { opaque_float, .. } => {
                    let opaque_float_value = u64::from_be_bytes(eight_input_bytes);
                    assert_eq!(opaque_float, OpaqueFloat(opaque_float_value));
                    assert_eq!(issuer, AccountID::from(ISSUER_BYTES));
                    assert_eq!(currency_code, CurrencyCode::from(CURRENCY_BYTES));
                }
                _ => panic!("Expected Amount::XRP"),
            },
            _ => panic!("Expected TokenAmount::IOU"),
        }
    }

    #[test]
    fn test_parse_invalid_amount() {
        // Test with an empty byte array
        assert!(TokenAmount::from_bytes(&[]).is_err());

        // Test with a byte array that's too short for XRP
        assert!(TokenAmount::from_bytes(&[0x40, 0, 0]).is_err());

        // Test with a byte array that's too short for MPT
        let mut mpt_bytes = [0u8; 20];
        mpt_bytes[0] = 0x60; // MPT positive flag
        assert!(TokenAmount::from_bytes(&mpt_bytes).is_err());

        // Test with a byte array that's too short for IOU
        let mut iou_bytes = [0u8; 30];
        iou_bytes[0] = 0xC0; // IOU positive flag
        assert!(TokenAmount::from_bytes(&iou_bytes).is_err());

        // Test with an invalid type bit pattern
        assert!(TokenAmount::from_bytes(&[0xA0, 0, 0, 0, 0, 0, 0, 0]).is_err());
    }
}
