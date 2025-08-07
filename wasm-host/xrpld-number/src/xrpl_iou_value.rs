//! # XRP Ledger Token Amount Format
//!
//! The XRP Ledger uses 64 bits to serialize the numeric amount of a (fungible) token.
//! In JSON format, the numeric amount is the `value` field of a currency amount object.
//! In binary format, the numeric amount consists of a "not XRP" bit, a sign bit,
//! significant digits, and an exponent, in the following order:
//!
//! ## Bit Layout
//!
//! ```text
//! [T][S][EEEEEEEE][MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM]
//!  │  │  └─8 bits─┘└──────────────────54 bits───────────────────────────┘
//!  │  └─ Sign (1=positive, 0=negative)
//!  └─ Type (1=fungible token, 0=XRP/MPT)
//! ```
//!
//! ## Field Descriptions
//!
//! ### Type Bit (T)
//! - The first (most significant) bit for a token amount is `1` to indicate that it is not an XRP amount
//! - XRP amounts always have the most significant bit set to `0` to distinguish them from this format
//!
//! ### Sign Bit (S)
//! - The sign bit indicates whether the amount is positive or negative
//! - Unlike standard two's complement integers, `1` indicates positive in the XRP Ledger format,
//!   and `0` indicates negative
//!
//! ### Exponent (8 bits)
//! - The next 8 bits represent the exponent as an unsigned integer
//! - The exponent indicates the scale (what power of 10 the significant digits should be multiplied by)
//! - Valid range: -96 to +80 (inclusive)
//! - During serialization, we add 97 to the exponent to make it possible to serialize as an unsigned integer
//! - Examples:
//!   - Serialized value of `1` indicates an exponent of `-96`
//!   - Serialized value of `177` indicates an exponent of `80`
//!
//! ### Mantissa (54 bits)
//! - The remaining 54 bits represent the significant digits (mantissa) as an unsigned integer
//! - During serialization, this value is normalized to the range:
//!   - Minimum: 10^15 (`1000000000000000`)
//!   - Maximum: 10^16-1 (`9999999999999999`)
//! - **Special case for zero**: When the value is 0, the sign bit, exponent, and significant digits
//!   are all zeroes, so the 64-bit value is serialized as `0x8000000000000000`
