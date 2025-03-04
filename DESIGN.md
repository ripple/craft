# Smart Escrow Design

## 1. Introduction

This document outlines the design and implementation of programmable conditions for XRPL Escrow objects. Smart Escrows enable conditional logic for releasing escrowed funds, extending the capabilities of time-based and crypto-condition escrows.

## 2. `FinishFunction`

The field contains compiled WebAssembly (WASM) code that is uploaded to the XRPL. This code implements conditional logic that determines whether an escrow can be finished.

**Requirements:**
- The compiled WASM code must contain a function `finish` with a standardized signature that evaluates the escrow condition
- The function must return a boolean value:
  - `true`: The escrow can be finished
  - `false`: The escrow cannot be finished
- The function is triggered by an EscrowFinish transaction

## 3. Technical Implementation

The XRPL validators will execute the WASM code in a sandboxed environment with the following characteristics:

- Restricted execution environment (limited instruction count)
- Memory-safe execution
- Deterministic outcomes across all validators
- Minimal resource consumption

### 3.1. WebAssembly Interface

#### Required Functions

The WebAssembly module must expose the following functions:

##### `allocate(size: usize) -> *mut u8`

This function is called by the host environment to request memory allocation within the WebAssembly module's memory space. It returns a pointer to the allocated memory region.

```rust
#[no_mangle]
pub extern fn allocate(size: usize) -> *mut u8 {
    let mut buffer = Vec::with_capacity(size);
    let pointer = buffer.as_mut_ptr();
    std::mem::forget(buffer);
    pointer
}
```

##### `finish(tx_json_ptr: *mut u8, tx_json_size: usize, lo_json_ptr: *mut u8, lo_json_size: usize) -> bool`

This function evaluates whether an escrow can be finished based on the transaction and ledger object data provided.

Parameters:
- `tx_json_ptr`: Pointer to the JSON data of the EscrowFinish transaction
- `tx_json_size`: Size in bytes of the transaction JSON data
- `lo_json_ptr`: Pointer to the JSON data of the Escrow ledger object
- `lo_json_size`: Size in bytes of the ledger object JSON data

Returns a boolean indicating whether the escrow can be finished.

#### Execution Flow

1. The host environment calls `allocate` to request memory for transaction data.
2. The host writes transaction JSON data to the allocated memory.
3. The host calls `allocate` again to request memory for ledger object data.
4. The host writes ledger object JSON data to the allocated memory.
5. The host calls `finish` with the memory pointers and sizes.
6. The `finish` function processes the data and returns a boolean result.

### 3.2. Memory Management

The WebAssembly module manages memory explicitly through the `allocate` function, which allows the host to request memory regions where it can write data. This pattern provides:

- **Safe data exchange**: Complex data structures like JSON can be safely passed between the host and WebAssembly module.
- **Memory control**: The WebAssembly module maintains control over its own memory.
- **Explicit ownership**: Memory allocation and access patterns are clearly defined for both the host and the module.

Since WebAssembly modules have their own linear memory, this explicit allocation mechanism creates a standard protocol for hosts to interact with the module's memory space.

## 4. Restrictions and Limitations

To ensure security, performance, and determinism, the following restrictions apply to WASM code in the `FinishFunction`:

| Category | Restriction |
|----------|-------------|
| Execution | Super restricted (<X instructions allowed, for some defined-in-future value of X) |
| Storage | Minimal data storage allowed |
| Interface | Simple ABI - a function that takes the transaction and returns a true/false value |
| Functions | No helper functions (standard library only) |
| Data Access | Read-access of ledger objects allowed |
| Modifications | No write access to other ledger objects (or other fields of the Escrow) |
| Transactions | No transaction emission or creation |

These restrictions may be relaxed in future versions as the system matures and security is further established.
