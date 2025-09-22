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

The WebAssembly module must expose the following function:

##### `finish() -> i32`

This is the main entry point that evaluates whether an escrow can be finished. The function accesses transaction and ledger data through host functions provided by the XRPL runtime.

Returns:
- `1`: The escrow can be finished (release funds)
- `0`: The escrow cannot be finished (keep funds locked)

```rust
#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_std::host;

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    // Get the current ledger sequence number
    let ledger_seq = unsafe { host::get_ledger_sqn() };

    // Get the parent ledger time
    let ledger_time = unsafe { host::get_parent_ledger_time() };

    // Example logic: Release escrow after ledger 100 and after timestamp 750000000
    // (These are example values - implement your actual business logic here)
    if ledger_seq > 100 && ledger_time > 750000000 {
        1  // Release escrow
    } else {
        0  // Keep escrow locked
    }
}
```

#### Host Functions

The WASM module can import and use various host functions to access blockchain data:

- **Ledger access**: `get_ledger_sqn()`, `get_parent_ledger_time()`, `get_parent_ledger_hash()`
- **Transaction data**: `get_tx_field()`, `get_tx_array_len()`, `get_tx_nested_field()`
- **Ledger objects**: `cache_ledger_obj()`, `get_ledger_obj_field()`, `get_ledger_obj_array_len()`
- **Keylets**: Various keylet functions for accessing specific object types
- **Utilities**: `trace()`, `compute_sha512_half()`, `update_data()`

#### Execution Flow

1. The XRPL validator loads the WASM module
2. The validator calls the `finish()` function
3. The function uses host functions to access necessary data
4. The function returns 1 (finish) or 0 (don't finish)
5. The validator uses this result to determine escrow outcome

### 3.2. Memory Management

The WebAssembly module uses a stack-based memory model with pre-allocated buffers:

- **No dynamic allocation**: The module does not export an `allocate` function
- **Stack buffers**: All data is passed through fixed-size stack buffers
- **Host-managed**: The host environment manages data transfer through function parameters
- **Deterministic**: Fixed memory usage ensures predictable execution

This approach provides:
- **Safety**: No memory leaks or allocation failures
- **Simplicity**: No complex memory management code needed
- **Performance**: Stack allocation is faster than heap allocation
- **Determinism**: Memory usage is predictable across all validators

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
