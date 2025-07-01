use crate::core::types::account_id::AccountID;
use crate::core::types::amount::amount::Amount;
use crate::core::types::amount::{CurrencyCode, MptId};
use crate::host;
use crate::host::Error::InternalError;
use crate::host::trace::trace_num;

pub const TOKEN_AMOUNT_SIZE: usize = 48;

/// A zero-cost abstraction for XRPL tokens. Tokens conform to the following binary layout:
///
/// ```markdown
///              ┌─────────────────────────────────────────────────────────────────────────────┐
///              │                       XRP Amount (64 bits / 8 bytes)                        │
///              ├─────────────────────────────────────────────────────────────────────────────┤
///              │                     ┌──────────────────────────────────────────────────────┐│
///              │ ┌─┐┌─┐┌─┐ ┌─┬─┬─┬─┐ │┌─┬─┬────────────────────────────────────────────┬─┬─┐││
///              │ │0││1││0│ │0│0│0│0│ ││0│1│                    ...                     │1│1│││
///              │ └─┘└─┘└─┘ └─┴─┴─┴─┘ │└─┴─┴────────────────────────────────────────────┴─┴─┘││
///              │  ▲  ▲  ▲       ▲    │               Integer Drops (57 bits)                ││
///              │  │  │  │       │    └──────────────────────────────────────────────────────┘│
///          ┌───┼──┘  │  └─────┐ └────────────────┐                                           │
///          │   └─────┼────────┼──────────────────┼───────────────────────────────────────────┘
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
///
///              ┌──────────────────────────────────────────────────────────────────────────────┐
///              │                        MPT Amount (264-bits/33-bytes)                        │
///              ├──────────────────────────────────────────────────────────────────────────────┤
///              │         ┌─────────────────┐    ┌──────────┐ ┌────────────┐ ┌────────────────┐│
///              │ ┌─┐┌─┐  │┌─┬─┬─┬─┬─┬─┬─┬─┐│    │┌─┬────┬─┐│ │ ┌─┬────┬─┐ │ │   ┌─┬────┬─┐   ││
///              │ │1││1│  ││0│0│0│0│0│0│0│0││    ││0│... │1││ │ │0│... │1│ │ │   │0│... │1│   ││
///              │ └─┘└─┘  │└─┴─┴─┴─┴─┴─┴─┴─┘│    │└─┴────┴─┘│ │ └─┴────┴─┘ │ │   └─┴────┴─┘   ││
///              │  ▲  ▲   │Exponent (8 Bits)│    │  Amount  │ │Sequence Num│ │Issuer AccountID││
///              │  │  │   └─────────────────┘    │(64 bits) │ │ (32 bits)  │ │   (160 bits)   ││
///              │  │  └───────────────────────┐  └──────────┘ └────────────┘ └────────────────┘│
///              │  │                          │                                                │
///              └──┴──────────────────────────┴────────────────────────────────────────────────┘
///       ┌──────────────────┐       ┌──────────────────┐
///       │     Type Bit     │       │     Sign bit     │
///       │(0=XRP/MPT;1=IOU) │       │ (1 for positive) │
///       └──────────────────┘       └──────────────────┘
///
///
///
///             ┌────────────────────────────────────────────────────────────────────────────────┐
///             │                         IOU Amount (384-bits/48-bytes)                         │
///             ├────────────────────────────────────────────────────────────────────────────────┤
///             │         ┌─────────────────┐   ┌──────────┐ ┌──────────────┐ ┌────────────────┐ │
///             │ ┌─┐┌─┐  │┌─┬─┬─┬─┬─┬─┬─┬─┐│   │┌─┬────┬─┐│ │    ┌─┬────┬─┐│ │   ┌─┬────┬─┐   │ │
///             │ │1││1│  ││0│0│0│0│0│0│0│0││   ││0│... │1││ │    │0│... │1││ │   │0│... │1│   │ │
///             │ └─┘└─┘  │└─┴─┴─┴─┴─┴─┴─┴─┘│   │└─┴────┴─┘│ │    └─┴────┴─┘│ │   └─┴────┴─┘   │ │
///             │  ▲  ▲   │Exponent (8 Bits)│   │  Amount  │ │Currency Code │ │Issuer AccountID│ │
///             │  │  │   └─────────────────┘   │(64 bits) │ │  (160 bits)  │ │   (160 bits)   │ │
///             │  │  └───────────────────────┐ └──────────┘ └──────────────┘ └────────────────┘ │
///             │  │                          │                                                  │
///             └──┴──────────────────────────┴──────────────────────────────────────────────────┘
///      ┌──────────────────┐       ┌──────────────────┐
///      │     Type Bit     │       │     Sign bit     │
///      │(0=XRP/MPT;1=IOU) │       │ (1 for positive) │
///      └──────────────────┘       └──────────────────┘
/// ```
///
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
        if bytes.is_empty() {
            return Err(InternalError);
        }

        let byte0 = bytes[0]; // Get the first byte for flag extraction

        // Extract flags using bitwise operations
        let type_bit = byte0 & 0x80 & 1; // Bit 7 (Most Significant Bit)
        let is_xrp_or_mpt = type_bit == 0;
        let is_iou = !is_xrp_or_mpt; // type_bit == 1

        let is_xrp: bool = byte0 & 0x20 & 1 == 0; // Bit 5 (only used if type_bit is 0)
        let is_mpt: bool = byte0 & 0x20 & 1 == 1; // Bit 5 (only used if type_bit is 0)

        if is_xrp_or_mpt {
            if is_xrp {
                // Parse the Amount::XRP from the first 8 bytes
                let mut amount_bytes = [0u8; 9];
                amount_bytes[0] = byte0;
                amount_bytes[1..9].copy_from_slice(&bytes[1..9]);

                let amount = Amount::from_bytes(amount_bytes);
                match amount {
                    Amount::XRP { .. } => Ok(TokenAmount::XRP { amount }),
                    _ => Err(InternalError),
                }
            } else if is_mpt {
                // MPT amount: 33 bytes
                if bytes.len() < 33 {
                    return Err(InternalError);
                }

                // Parse the Amount::MPT from the first 9 bytes
                let mut amount_bytes = [0u8; 9];
                amount_bytes.copy_from_slice(&bytes[0..9]);

                let amount = Amount::from_bytes(amount_bytes);

                // Parse the MptId from the remaining bytes
                let mut mpt_id_bytes = [0u8; 24];
                mpt_id_bytes.copy_from_slice(&bytes[9..33]);
                let mpt_id = MptId::from(mpt_id_bytes);

                match amount {
                    Amount::MPT { .. } => Ok(TokenAmount::MPT {
                        amount: amount,
                        mpt_id,
                    }),
                    _ => Err(InternalError),
                }
            } else {
                Err(InternalError)
            }
        } else if is_iou {
            // IOU amount: 48 bytes
            if bytes.len() < 48 {
                return Err(InternalError);
            }

            // Parse the Amount::IOU from the first 9 bytes
            let mut amount_bytes = [0u8; 9];
            amount_bytes.copy_from_slice(&bytes[0..9]);

            let amount = Amount::from_bytes(amount_bytes);

            // Parse the CurrencyCode from the next 20 bytes
            let mut currency_code_bytes = [0u8; 20];
            currency_code_bytes.copy_from_slice(&bytes[9..29]);
            let currency_code = CurrencyCode::from(currency_code_bytes);

            // Parse the AccountID from the last 20 bytes
            let mut issuer_bytes = [0u8; 20];
            issuer_bytes.copy_from_slice(&bytes[29..49]);
            let issuer = AccountID::from(issuer_bytes);

            match amount {
                Amount::IOU { .. } => Ok(TokenAmount::IOU {
                    amount,
                    issuer,
                    currency_code,
                }),
                _ => Err(InternalError),
            }
        } else {
            Err(InternalError)
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

    #[test]
    fn test_parse_xrp_amount() {
        // Create a test XRP amount byte array
        // XRP amount: [0/type][1/sign][0/is-mpt][4/reserved][57/value]
        // First byte: 0b0100_0000 (0x40)
        // Value: 1,000,000 (0xF4240 in hex)
        let mut bytes = [0u8; 8];
        bytes[0] = 0x40; // XRP positive flag
        bytes[1..8].copy_from_slice(&1_000_000u64.to_le_bytes()[1..8]);

        // Parse the TokenAmount
        let token_amount = TokenAmount::from_bytes(&bytes).unwrap();

        // Verify it's an XRP amount with the correct value
        match token_amount {
            TokenAmount::XRP { amount: value } => match value {
                Amount::XRP {
                    num_drops: value,
                    is_positive,
                } => {
                    assert_eq!(value, 1_000_000);
                    assert!(is_positive);
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
        // Value: 500,000
        // Sequence: 12345
        // Issuer: [1u8; 20]
        let mut bytes = [0u8; 33];

        // Set the amount bytes
        bytes[0] = 0x60; // MPT positive flag
        bytes[1..9].copy_from_slice(&500_000u64.to_le_bytes());

        // Set the MptId bytes
        let sequence_num = 12345u32;
        bytes[9..13].copy_from_slice(&sequence_num.to_le_bytes());

        let issuer_bytes = [1u8; 20];
        bytes[13..33].copy_from_slice(&issuer_bytes);

        // Parse the TokenAmount
        let token_amount = TokenAmount::from_bytes(&bytes).unwrap();

        // Verify it's an MPT amount with the correct values
        match token_amount {
            TokenAmount::MPT {
                amount: value,
                mpt_id,
            } => {
                match value {
                    Amount::MPT {
                        num_units: value,
                        is_positive,
                    } => {
                        assert_eq!(value, 500_000);
                        assert!(is_positive);
                    }
                    _ => panic!("Expected Amount::MPT"),
                }

                assert_eq!(mpt_id.get_sequence_num(), sequence_num);
                assert_eq!(mpt_id.get_issuer(), AccountID::from(issuer_bytes));
            }
            _ => panic!("Expected TokenAmount::MPT"),
        }
    }

    #[test]
    fn test_parse_iou_amount() {
        // Create a test IOU amount byte array
        // IOU amount: [1/type][1/sign][8/exponent][54/mantissa][160/currency][160/issuer]
        // First byte: 0b1100_0000 (0xC0)
        // Exponent: 5
        // Mantissa: 12345
        // Currency: [2u8; 20]
        // Issuer: [3u8; 20]
        let mut bytes = [0u8; 48];

        // Set the amount bytes
        bytes[0] = 0xC0; // IOU positive flag
        bytes[1] = 5; // Exponent

        // Set the mantissa in the remaining 7 bytes (54 bits)
        let mantissa: u64 = 12345;
        let mantissa_bytes = mantissa.to_le_bytes();
        bytes[2..9].copy_from_slice(&mantissa_bytes[1..8]);

        // Set the currency code bytes
        let currency_bytes = [2u8; 20];
        bytes[9..29].copy_from_slice(&currency_bytes);

        // Set the issuer bytes
        let issuer_bytes = [3u8; 20];
        bytes[29..49].copy_from_slice(&issuer_bytes);

        // Parse the TokenAmount
        let token_amount = TokenAmount::from_bytes(&bytes).unwrap();

        // Verify it's an IOU amount with the correct values
        match token_amount {
            TokenAmount::IOU {
                amount: value,
                issuer,
                currency_code,
            } => {
                match value {
                    Amount::XRP {
                        num_drops: value,
                        is_positive,
                    } => {
                        assert_eq!(value, mantissa & 0x003FFFFFFFFFFFFF); // Ensure only 54 bits are used
                        assert!(is_positive);
                    }
                    _ => panic!("Expected Amount::XRP"),
                }

                assert_eq!(issuer, AccountID::from(issuer_bytes));
                assert_eq!(currency_code, CurrencyCode::from(currency_bytes));
            }
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
