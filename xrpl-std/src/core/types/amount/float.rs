/// Represents an opaque floating point number that conforms to the XRPL binary encoding of a Token Amount.
///
/// This struct wraps a 64-bit unsigned integer that encodes a floating-point value according to
/// the XRP Ledger's token amount format specification. The encoding uses a custom format that
/// allows for precise representation of currency amounts in the XRPL ecosystem.
///
/// # Format
/// The underlying `u64` follows the XRPL token amount binary format:
/// - Bit 63: Not XRP flag (always 1 for tokens)
/// - Bit 62: Sign flag
/// - Bits 61-54: 8-bit exponent (biased by 97)
/// - Bits 53-0: 54-bit mantissa
///
/// # Examples
/// ```rust
/// let amount = XrplFloat(0x8000000000000000); // Minimum positive token amount
/// let zero = XrplFloat(0x8000000000000000);   // Zero amount representation
/// ```
///
/// # See Also
/// - [XRPL Token Amount Format](https://xrpl.org/docs/references/protocol/binary-format#token-amount-format)
/// - [Currency Amount Specification](https://xrpl.org/docs/references/protocol/data-types#currency-amount)

#[derive(Copy, Debug, PartialEq, Eq, Clone, Ord, PartialOrd)]
pub struct XrplFloat(pub u64);