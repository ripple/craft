# XRPL Field Access and Locators

This document explains how to access fields in XRPL transactions and ledger objects.

The locator system is used to navigate nested data structures.

## Table of Contents

- [Overview](#overview)
- [Field Types and Codes](#field-types-and-codes)
- [Simple Field Access](#simple-field-access)
- [Nested Field Access](#nested-field-access)
- [Understanding Locators](#understanding-locators)
- [JSON vs Internal Representation](#json-vs-internal-representation)
- [Common Patterns](#common-patterns)
- [Field Code Reference](#field-code-reference)
- [Troubleshooting](#troubleshooting)

## Overview

XRPL data structures can contain:
- **Simple fields**: Direct values (Account, Balance, Sequence)
- **Object fields**: Nested structures (Memo, Signer)
- **Array fields**: Lists of objects (Memos, Signers)

Accessing these fields involves:
1. Field codes (SFIELD constants)
2. Direct vs nested access patterns
3. The locator system for nested paths
4. Differences between JSON and internal representations

## Field Types and Codes

Every field has a unique field code that combines:
- **Type ID**: The data type (1-19)
- **Field ID**: The specific field (0-255)

```rust
// Field code calculation
const FIELD_CODE: i32 = (TYPE_ID << 16) | FIELD_ID;

// Examples
const SFIELD_ACCOUNT: i32 = 524289;      // (8 << 16) | 1
const SFIELD_BALANCE: i32 = 393222;      // (6 << 16) | 6
const SFIELD_MEMOS: i32 = 983049;        // (15 << 16) | 9
```

## Simple Field Access

For top-level fields, use direct access functions:

### Transaction Fields

```rust
use xrpl_std::host::get_tx_field;
use xrpl_std::sfield;

// Get account from transaction
let mut account_buf = [0u8; 20];
let len = get_tx_field(
    sfield::Account,
    0,  // slot (always 0)
    &mut account_buf
)?;

// Get fee amount
let mut fee_buf = [0u8; 8];
let len = get_tx_field(
    sfield::Fee,
    0,
    &mut fee_buf
)?;
```

### Ledger Object Fields

```rust
use xrpl_std::host::get_ledger_obj_field;

// First load object into cache
let slot = cache_ledger_obj(&keylet)?;

// Then access fields
let mut balance_buf = [0u8; 8];
let len = get_ledger_obj_field(
    slot,
    sfield::Balance,
    0,  // field_slot (always 0)
    &mut balance_buf
)?;
```

## Nested Field Access

Complex objects require locators to navigate to nested fields.

### Building Locators

```rust
use xrpl_std::locator::LocatorPacker;

let mut locator = LocatorPacker::new();

// Add fields to the path
locator.pack(field1);
locator.pack(field2);
locator.pack(field3);

// Use the packed locator
let result = get_tx_nested_field(&locator.data, &mut buffer)?;
```

### Locator Encoding Rules

1. **First field**: 6 bytes (2-byte type/field + 4-byte extended)
2. **Subsequent fields**: 5 bytes each (1-byte marker + 4-byte data)
3. **Maximum depth**: 12 levels
4. **Maximum size**: 64 bytes

```rust
// Internal structure
pub struct LocatorPacker {
    pub data: [u8; 64],  // Packed locator data
    pub len: usize,      // Current length
}
```

## JSON vs Internal Representation

### Critical Difference

**JSON representation** includes wrapper objects for type safety:
```json
{
  "Memos": [
    {
      "Memo": {
        "MemoType": "test",
        "MemoData": "data"
      }
    }
  ]
}
```

**Internal representation** omits the wrapper:
```
Memos[0].MemoType  // Direct access after array index
```

### Why This Matters

When building locators, use the **internal path**, not the JSON path:

```rust
// CORRECT: Internal representation
let mut locator = LocatorPacker::new();
locator.pack(sfield::Memos);     // Array field
locator.pack(0);                  // Array index
locator.pack(sfield::MemoType);   // Target field

// WRONG: Attempting to follow JSON structure
let mut locator = LocatorPacker::new();
locator.pack(sfield::Memos);     // Array field
locator.pack(0);                  // Array index
locator.pack(sfield::Memo);       // INCORRECT: wrapper
locator.pack(sfield::MemoType);   // Target field
```

### The Wrapper's Purpose

The JSON wrapper exists because:
1. **Encoding requirement**: Every STObject needs a field identifier
2. **Type safety**: Explicitly declares the object type
3. **Consistency**: All arrays follow this pattern
4. **Serialization**: Required for binary ↔ JSON conversion

However, internally, once you index into an array, you're already at the object level.

## Common Patterns

### Accessing Memo Fields

```rust
// Access first memo's type
let mut locator = LocatorPacker::new();
locator.pack(sfield::Memos);
locator.pack(0);
locator.pack(sfield::MemoType);

let mut buffer = [0u8; 256];
let len = get_tx_nested_field(&locator.data, &mut buffer)?;
```

### Accessing Signer Fields

```rust
// Access second signer's account
let mut locator = LocatorPacker::new();
locator.pack(sfield::Signers);
locator.pack(1);  // Second signer (0-indexed)
locator.pack(sfield::Account);

let mut account = [0u8; 20];
let len = get_tx_nested_field(&locator.data, &mut account)?;
```

### Accessing Oracle Data

```rust
// Access price from oracle document
let mut locator = LocatorPacker::new();
locator.pack(sfield::PriceDataSeries);
locator.pack(0);
locator.pack(sfield::AssetPrice);

let mut price_buf = [0u8; 8];
let len = get_ledger_obj_nested_field(
    oracle_slot,
    &locator.data,
    &mut price_buf
)?;
```

### Iterating Arrays

```rust
// Get array length
let memo_count = get_tx_array_len(sfield::Memos)?;

// Process each memo
for i in 0..memo_count {
    // Access MemoType
    let mut type_locator = LocatorPacker::new();
    type_locator.pack(sfield::Memos);
    type_locator.pack(i as i32);
    type_locator.pack(sfield::MemoType);
    
    // Access MemoData
    let mut data_locator = LocatorPacker::new();
    data_locator.pack(sfield::Memos);
    data_locator.pack(i as i32);
    data_locator.pack(sfield::MemoData);
    
    // Process memo...
}
```

## Field Code Reference

### Common Transaction Fields

| Field | Code | Type | Size |
|-------|------|------|------|
| Account | 524289 | AccountID | 20 |
| TransactionType | 131074 | UInt16 | 2 |
| Fee | 524296 | Amount | 8 |
| Sequence | 262148 | UInt32 | 4 |
| Flags | 131076 | UInt32 | 4 |
| SourceTag | 196611 | UInt32 | 4 |
| DestinationTag | 917508 | UInt32 | 4 |
| SigningPubKey | 196627 | Blob | 33 |
| TxnSignature | 262148 | Blob | Variable |

### Escrow Fields

| Field | Code | Type | Size |
|-------|------|------|------|
| Owner | 524289 | AccountID | 20 |
| Destination | 524291 | AccountID | 20 |
| Amount | 393222 | Amount | 8 |
| Condition | 851975 | Blob | 32 |
| CancelAfter | 262156 | UInt32 | 4 |
| FinishAfter | 262157 | UInt32 | 4 |
| SourceTag | 196624 | UInt32 | 4 |
| DestinationTag | 196622 | UInt32 | 4 |

### Array Fields

| Field | Code | Type |
|-------|------|------|
| Memos | 983049 | Array of Memo |
| Signers | 720899 | Array of Signer |
| SignerEntries | 720900 | Array of SignerEntry |
| Paths | 65537 | Array of Path |

### Nested Object Fields

| Parent | Field | Code | Type |
|--------|-------|------|------|
| Memo | MemoType | 458764 | Blob |
| Memo | MemoData | 458765 | Blob |
| Memo | MemoFormat | 458766 | Blob |
| Signer | Account | 524291 | AccountID |
| Signer | TxnSignature | 524292 | Blob |
| Signer | SigningPubKey | 524293 | Blob |

## Troubleshooting

### Common Errors

#### FieldNotFound (-2)
- Field doesn't exist in the object
- Optional field not present
- Wrong field code used

**Solution**: Check if field is optional, verify field code

#### NotLeafField (-5)
- Trying to read an object/array as a value
- Missing array index in locator

**Solution**: Add array index or access sub-fields

#### LocatorMalformed (-6)
- Locator exceeds 64 bytes
- Invalid packing sequence
- Corrupted locator data

**Solution**: Check locator depth, verify packing order

#### NoArray (-4)
- Expected array field not found
- Wrong field code for array

**Solution**: Verify field is actually an array type

### Debugging Tips

1. **Use trace functions** to output field values during development
2. **Check array lengths** before accessing elements
3. **Verify field existence** with proper error handling
4. **Test with fixtures** that have all optional fields
5. **Compare with JSON** to understand structure (but follow internal paths!)

### Best Practices

1. **Cache array lengths** to avoid repeated calls
2. **Reuse locators** when accessing multiple fields at same level
3. **Handle optional fields** gracefully
4. **Use constants** for field codes
5. **Document** expected fields

## Examples

### Complete Memo Processing

```rust
use xrpl_std::locator::LocatorPacker;
use xrpl_std::host::{get_tx_array_len, get_tx_nested_field};
use xrpl_std::sfield;

fn process_memos() -> Result<()> {
    // Get memo count
    let count = get_tx_array_len(sfield::Memos)?;
    
    for i in 0..count {
        // Build locator for MemoType
        let mut type_loc = LocatorPacker::new();
        type_loc.pack(sfield::Memos);
        type_loc.pack(i);
        type_loc.pack(sfield::MemoType);
        
        // Read MemoType
        let mut type_buf = [0u8; 256];
        let type_len = get_tx_nested_field(
            &type_loc.data,
            &mut type_buf
        )?;
        
        // Build locator for MemoData
        let mut data_loc = LocatorPacker::new();
        data_loc.pack(sfield::Memos);
        data_loc.pack(i);
        data_loc.pack(sfield::MemoData);
        
        // Read MemoData
        let mut data_buf = [0u8; 256];
        let data_len = get_tx_nested_field(
            &data_loc.data,
            &mut data_buf
        )?;
        
        // Process memo
        let memo_type = &type_buf[..type_len as usize];
        let memo_data = &data_buf[..data_len as usize];
        
        // Your logic here...
    }
    
    Ok(())
}
```

### Oracle Price Access

```rust
fn get_oracle_price(oracle_slot: i32) -> Result<u64> {
    // Build locator for first price entry
    let mut locator = LocatorPacker::new();
    locator.pack(sfield::PriceDataSeries);
    locator.pack(0);
    locator.pack(sfield::AssetPrice);
    
    // Read price
    let mut price_buf = [0u8; 8];
    let len = get_ledger_obj_nested_field(
        oracle_slot,
        &locator.data,
        0,  // field_slot
        &mut price_buf
    )?;
    
    // Convert to u64
    Ok(u64::from_le_bytes(price_buf))
}
```

## See Also

- [Binary Format Reference](https://xrpl.org/docs/references/protocol/binary-format)
- [Ledger Entry Types](https://xrpl.org/docs/references/protocol/ledger-data/ledger-entry-types)
