use crate::types::core::{ApplyContext, Hash256};
use log::info;
use wasmedge_sdk::error::CoreError;
use wasmedge_sdk::{CallingFrame, ValType, WasmValue};

// --- Constants for return codes ---
// Assuming these are defined elsewhere as int64_t
// const RC_ROLLBACK: i64 = -1; // Example value
// const RC_ACCEPT: i64 = -2; // Example value
const TOO_SMALL: i64 = -3; // Example value
const OUT_OF_BOUNDS: i64 = -4; // Example value
// const UNKNOWN_ERROR: i64 = -5;
// const SUCCESS_PLACEHOLDER: i64 = 0; // A generic success code if needed, though returning size is common

// The signature matches what `wasmedge-sdk` expects for host functions.
// We assume HookContext is retrievable as mutable data associated with the Caller/Instance.
// If HookContext is immutable or globally accessible, adjust the signature.

/// Copies the transaction hash of the EscrowFinish transaction into WASM memory for access by the
/// finish contract.
///

// Needed: WASM Address of the array to write bytes into.
// Needed: # of bytes to write (32 for now, hard-coded).

pub(crate) fn get_tx_hash_helper(
    apply_ctx: ApplyContext, // <-- In rippled this is accessible via PreclaimResult or otherwise.
    // _inst: &mut Instance,
    _caller: &mut CallingFrame, // Provides access to memory, instance data, etc.
    inputs: Vec<WasmValue>,     // <-- Pointer to the array declared in WASM.
) -> Result<Vec<WasmValue>, CoreError> {
    // check the number of inputs
    if inputs.len() != 1 {
        return Err(CoreError::Execution(
            wasmedge_sdk::error::CoreExecutionError::FuncSigMismatch,
        ));
    }

    println!("Input1: {:?}", inputs[0]);

    // parse the first input of WebAssembly value type into Rust built-in value type
    let guest_write_ptr = if inputs[0].ty() == ValType::I32 {
        inputs[0].to_i32()
    } else {
        return Err(CoreError::Execution(
            wasmedge_sdk::error::CoreExecutionError::FuncSigMismatch,
        ));
    };

    let guest_write_ptr_u32: u32 = guest_write_ptr as u32;
    let write_len = 32; // <-- hard-coded for now, for Escrow TXN ID, but may chnage for other fields.
    // let flags = inputs[2].to_i32() as u32; // Assuming i32 in Wasm maps to u32 flags
    println!("guest_write_ptr: {}", guest_write_ptr_u32);

    // 2. Context Retrieval (Replaces reinterpret_cast<hook::HookContext*>(data_ptr) and HOOK_SETUP)
    // Get memory. Assumes memory index 0.
    let mut memory = _caller.memory_mut(0).ok_or_else(|| {
        eprintln!("otxn_id: Error: Failed to get memory instance");
        CoreError::Execution(wasmedge_sdk::error::CoreExecutionError::MemoryOutOfBounds)
    })?;
    println!("memory.size (pages): {}", memory.size());

    // Get HookContext. This depends heavily on how you associate data.
    // Option 1: Data associated with the module instance.
    // let hook_ctx = caller.instance_data().downcast_ref::<HookContext>().ok_or(...) ?;
    // Option 2: Data passed during function registration (less common for instance-specific data).
    // let hook_ctx = caller.data().downcast_ref::<HookContext>().ok_or(...) ? ;

    // *** Using a placeholder HookContext for demonstration ***
    // In a real scenario, retrieve it properly from the caller/instance.
    // let hook_ctx = HookContext { emit_failure: false }; // Example: Replace with actual retrieval
    // let apply_ctx = get_apply_context(&hook_ctx); // Simulates HOOK_SETUP part

    // 3. Get Transaction ID (Matches C++ logic)
    let tx_id: Hash256 = apply_ctx.tx.get_transaction_id();
    // match write!(writer, "{:02X}", byte) {
    info!("Simulated tx_id from apply_ctx: {:02X}", tx_id);

    let tx_id_size = tx_id.size() as u32;
    // let tx_id_data = tx_id.data();

    // // 4. Size Check
    // if tx_id_size > write_len {
    //     println!("otxn_id: Buffer too small (need {}, got {})", tx_id_size, write_len);
    //     // Return TOO_SMALL as i64
    //     return Ok(vec![WasmValue::from_i64(TOO_SMALL)]);
    // }

    // 5. Bounds Check (Matches NOT_IN_BOUNDS)
    let memory_size = memory.size() * 65536; // Memory size is in pages (64KiB)
    println!("memory_size (KiB): {}", memory_size);

    // let memory_size = memory // Get Option<&MemoryType> without consuming memory
    //     .expect("Memory should not be None") // Panic if None, returns &MemoryType
    //     .size() // Call size() on the borrowed &MemoryType
    //     .wrapping_mul(65536); // Use wrapping_mul for potential overflow safety

    // Check if write_ptr + tx_id_size overflows or goes out of bounds
    // Using checked_add to prevent overflow issues during check.
    let end_ptr = match guest_write_ptr_u32.checked_add(tx_id_size) {
        Some(end) => end,
        None => {
            println!("otxn_id: Out of bounds (pointer + size overflow)");
            return Ok(vec![WasmValue::from_i64(OUT_OF_BOUNDS)]);
        }
    };

    if end_ptr > memory_size {
        println!(
            "otxn_id: Out of bounds (ptr {} + size {} > memory {})",
            guest_write_ptr_u32, tx_id_size, memory_size
        );
        // Return OUT_OF_BOUNDS as i64
        return Ok(vec![WasmValue::from_i64(OUT_OF_BOUNDS)]);
    }
    // The original check also seemed to validate write_ptr + write_len.
    // This might be redundant if tx_id_size <= write_len, but we can add it for safety.
    // let end_ptr_alloc = match guest_write_ptr_u32.checked_add(write_len) {
    //     Some(end) => end,
    //     None => {
    //         println!("otxn_id: Out of bounds (pointer + alloc size overflow)");
    //         return Ok(vec![WasmValue::from_i64(OUT_OF_BOUNDS)]);
    //     }
    // };
    // if end_ptr_alloc > memory_size {
    //     println!(
    //         "otxn_id: Out of bounds (allocated buffer ptr {} + len {} > memory {})",
    //         guest_write_ptr_u32, write_len, memory_size
    //     );
    //     return Ok(vec![WasmValue::from_i64(OUT_OF_BOUNDS)]);
    // }

    // 6. Write to Memory (Matches WRITE_WASM_MEMORY_AND_RETURN)
    println!(
        "otxn_id: Writing {} bytes to pointer {}",
        tx_id_size, guest_write_ptr_u32
    );

    // The `write` method takes a slice `&[u8]`.
    // You can easily get a slice from your array `[u8; 32]` using `&data[..]` or just `data`.
    // Rust often coerces `&[u8; N]` to `&[u8]` automatically in function calls.
    let data: [u8; 32] = tx_id.data()[..32].try_into().unwrap();
    let result = memory.write(guest_write_ptr_u32 as usize, data);
    match result {
        Some(()) => {
            println!("Wasm memory write succeeded!");
            // Proceed
        }
        None => {
            eprintln!("Error: Wasm memory write failed. Check address and bounds.");
            // Handle error
            // return Err(HostFuncError::User( /* some error code */ ));
        }
    }

    // 7. Result Handling (Matches C++ return logic)
    // The C++ returns the number of bytes written (txID.size()) on success.
    let return_code = tx_id_size as i64;
    println!("otxn_id: Success, wrote {return_code} bytes");
    // TODO: What should WASM Get here? Hooks uses return codes, so we probably need something that's close to `OK`?
    // Ok(vec![WasmValue::from_i64(return_code)])
    Ok(vec![])
}
