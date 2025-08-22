# XRPL WASM Float Operations

## Overview

This document describes the floating-point operations available to XRPL WebAssembly smart contracts and the planned migration from the current BigDecimal implementation to rippled's native Number class.

The float operations in XRPL are specifically used for **fungible tokens** (also called IOUs), which is one of three amount types supported by the XRP Ledger.

## Table of Contents

- [XRPL Amount Types](#xrpl-amount-types)
- [Current Architecture](#current-architecture)
- [Fungible Token Float Format](#fungible-token-float-format)
- [Available Operations](#available-operations)
- [Migration Plan](#migration-plan)
- [Implementation Details](#implementation-details)
- [Testing Strategy](#testing-strategy)

## XRPL Amount Types

The XRPL Amount type (STAmount) can represent three different types of assets:

### 1. XRP
- 64-bit unsigned integer (big-endian)
- Most significant bit: always 0
- Second bit: 1 (positive indicator)
- Third bit: 0 (not an MPT)
- Remaining 61 bits: quantity in drops
- Standard format: 64-bit unsigned integer OR'd with `0x4000000000000000`

### 2. Fungible Tokens (IOUs)
- **First 64 bits**: Amount in custom float format (detailed below)
- **Next 160 bits**: Currency code
- **Last 160 bits**: Issuer's Account ID
- Total: 384 bits (48 bytes)

### 3. Multi-Purpose Tokens (MPTs)
- **First 8 bits**: `0x60` (indicates MPT)
- **Next 64 bits**: Quantity as unsigned integer
- **Last 192 bits**: MPT Issuance ID

You can identify the amount type by examining the first and third bits:
- First bit = 1: Fungible token (IOU)
- First bit = 0, third bit = 0: XRP
- First bit = 0, third bit = 1: MPT

## Current Architecture

### BigDecimal Implementation

The current implementation uses Rust's BigDecimal library for all floating-point computations:

```text
Current flow:
WASM Module -> Host Function -> BigDecimal -> Result
```

**Characteristics:**
- Arbitrary precision decimal arithmetic
- No native rounding mode support
- Slower than native operations
- May differ from rippled in edge cases

### Limitations

1. **Rounding Modes**: Currently ignored, always uses BigDecimal defaults
2. **Performance**: ~10x slower than native float operations
3. **Compatibility**: Not guaranteed to match rippled exactly
4. **Precision**: Different internal representation than XRPL

## Fungible Token Float Format

https://xrpl.org/docs/references/protocol/binary-format#amount-fields

XRPL uses a custom 64-bit floating-point encoding for fungible token amounts:

```text
Bit Layout:
[T][S][EEEEEEEE][MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM]
 │  │  └8 bits┘  └──────────────────54 bits───────────────────────────┘
 │  └─ Sign (1=positive, 0=negative)
 └─ Type (1=fungible token, 0=XRP/MPT)
```

### Encoding Details

1. **Type bit**: Always 1 for fungible tokens
2. **Sign bit**: 1 = positive, 0 = negative
3. **Exponent**: 8 bits, biased by 97
   - Actual exponent = stored value - 97
   - Range: approximately -97 to +158
4. **Mantissa**: 54 bits of precision
   - Normalized to 16 decimal digits

### Special Values

- **Zero**: Special encoding `0x8000000000000000`
- **Maximum**: ~9.999999999999999 × 10^80
- **Minimum positive**: ~1.0 × 10^-81
- **Precision**: 16 significant decimal digits

## Available Operations

### Creation Functions

```rust,ignore
// Create from integer
float_from_int(value: i64, out: *mut u8, rounding_mode: i32) -> i32

// Create from unsigned integer
float_from_uint(value: *const u8, out: *mut u8, rounding_mode: i32) -> i32

// Create from exponent and mantissa
float_set(exponent: i32, mantissa: i64, out: *mut u8, rounding_mode: i32) -> i32
```

### Arithmetic Operations

```rust,ignore
// Addition: out = a + b
float_add(a: *const u8, b: *const u8, out: *mut u8, rounding_mode: i32) -> i32

// Subtraction: out = a - b
float_subtract(a: *const u8, b: *const u8, out: *mut u8, rounding_mode: i32) -> i32

// Multiplication: out = a × b
float_multiply(a: *const u8, b: *const u8, out: *mut u8, rounding_mode: i32) -> i32

// Division: out = a ÷ b
float_divide(a: *const u8, b: *const u8, out: *mut u8, rounding_mode: i32) -> i32
```

### Mathematical Functions

```rust,ignore
// Nth power: out = aⁿ
float_pow(a: *const u8, n: i32, out: *mut u8, rounding_mode: i32) -> i32

// Nth root: out = ⁿ√a
float_root(a: *const u8, n: i32, out: *mut u8, rounding_mode: i32) -> i32

// Base-10 logarithm: out = log₁₀(a)
float_log(a: *const u8, out: *mut u8, rounding_mode: i32) -> i32
```

### Comparison

```rust,ignore
// Compare two floats
// Returns: 0 (equal), 1 (a > b), 2 (a < b)
float_compare(a: *const u8, b: *const u8) -> i32
```

## Migration Plan

### Phase 1: FFI Interface Design

Create Rust bindings to rippled's Number class:

```rust,ignore
// Example FFI declarations
extern "C" {
    // Number creation
    fn rippled_number_from_mantissa_exponent(
        mantissa: i64,
        exponent: i32,
        out: *mut u8
    ) -> i32;
    
    // Arithmetic with rounding
    fn rippled_number_add(
        a: *const u8,
        b: *const u8,
        out: *mut u8,
        rounding_mode: i32
    ) -> i32;
    
    // Comparison
    fn rippled_number_compare(
        a: *const u8,
        b: *const u8
    ) -> i32;
}
```

### Phase 2: Implementation Options

#### Option A: Dynamic Linking
- Link against rippled shared library
- Pros: Always up-to-date with rippled
- Cons: Deployment complexity

#### Option B: Static Extraction
- Extract Number class into separate library
- Pros: Simpler deployment
- Cons: Manual updates needed

#### Option C: Reimplementation
- Reimplement Number in Rust
- Pros: Pure Rust, no FFI
- Cons: Risk of divergence

Option B is likely the best starting point. The other options can be considered later on.

### Phase 3: Integration

1. **Wrapper Layer**: Create safe Rust wrappers around FFI calls
2. **Error Handling**: Map C++ exceptions to Rust Results
3. **Memory Management**: Ensure proper cleanup of C++ objects
4. **Testing**: Comprehensive comparison against current implementation

## Implementation Details

### Current BigDecimal Flow

```rust,ignore
fn float_add(env: wasm_exec_env_t, a: *const u8, b: *const u8, 
             out: *mut u8, rounding_mode: i32) -> i32 {
    // 1. Deserialize XRPL format to BigDecimal
    let val_a = _deserialize_issued_currency_amount(read_bytes(a))?;
    let val_b = _deserialize_issued_currency_amount(read_bytes(b))?;
    
    // 2. Perform operation (no rounding mode support)
    let result = val_a + val_b;
    
    // 3. Serialize back to XRPL format
    let bytes = _serialize_issued_currency_value(result)?;
    write_bytes(out, bytes)
}
```

### Future Rippled Number Flow

```rust,ignore
fn float_add(env: wasm_exec_env_t, a: *const u8, b: *const u8,
             out: *mut u8, rounding_mode: i32) -> i32 {
    // Direct pass-through to rippled with rounding support
    unsafe {
        rippled_number_add(a, b, out, rounding_mode)
    }
}
```

### Rounding Mode Mapping

| Mode | Name | Description | IEEE 754 Equivalent |
|------|------|-------------|-------------------|
| 0 | ToNearest | Round to nearest, ties to even | roundTiesToEven |
| 1 | TowardsZero | Truncate towards zero | roundTowardZero |
| 2 | Downward | Round towards -∞ | roundTowardNegative |
| 3 | Upward | Round towards +∞ | roundTowardPositive |

## Testing Strategy

### Compatibility Testing

1. **Golden Dataset**: Generate test vectors using rippled
2. **Comparison Suite**: Compare BigDecimal vs Number results
3. **Edge Cases**: Focus on rounding boundaries
4. **Regression Tests**: Ensure no breaking changes

### Performance Testing

```rust,ignore
#[bench]
fn bench_float_multiply_bigdecimal(b: &mut Bencher) {
    // Current implementation benchmark
}

#[bench]
fn bench_float_multiply_rippled(b: &mut Bencher) {
    // Future implementation benchmark
}
```

### Security Testing

1. **Fuzzing**: Random inputs to find crashes
2. **Bounds Checking**: Verify all memory accesses
3. **FFI Safety**: Audit C++/Rust boundary

## Important Notes

### Context: Float Operations vs Amount Types

The float operations described in this document specifically handle the 64-bit amount portion of fungible tokens. They do NOT handle:

1. **XRP amounts** - These use simple 64-bit integers
2. **MPT amounts** - These also use 64-bit integers
3. **Currency codes** - The 160-bit currency identifier
4. **Issuer IDs** - The 160-bit account identifier

When working with full Amount objects in XRPL, you need to consider which type you're dealing with and use the appropriate handling:

```rust,ignore
// Example: Checking amount type
fn get_amount_type(first_byte: u8) -> AmountType {
    let type_bit = (first_byte & 0x80) != 0;  // First bit
    let mpt_bit = (first_byte & 0x20) != 0;   // Third bit
    
    if type_bit {
        AmountType::FungibleToken  // Uses float operations
    } else if mpt_bit {
        AmountType::MPT            // Uses integer operations
    } else {
        AmountType::XRP            // Uses integer operations
    }
}
```

## References

1. [XRPL Amount Fields Specification](https://xrpl.org/docs/references/protocol/binary-format#amount-fields)
2. [rippled Number Implementation](https://github.com/XRPLF/rippled/blob/develop/src/ripple/basics/Number.h)
3. [XRPL Binary Codec Specification](https://xrpl.org/serialization.html)
4. [XLS-100d Specification](https://github.com/XRPLF/XRPL-Standards/discussions/100)

## Appendix: Example Migration

### Before (BigDecimal)
```rust,ignore
let val = BigDecimal::from_str("123.456").unwrap();
let result = val * BigDecimal::from(2);
// No control over rounding
```

### After (rippled Number)
```rust,ignore
let val = Number::from_string("123.456").unwrap();
let result = val.multiply(Number::from(2), RoundingMode::ToNearest);
// Explicit rounding control
```
