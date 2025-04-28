use crate::types::core::{ApplyContext, Hash256};
use wasmedge_sdk::error::CoreError;
use wasmedge_sdk::{CallingFrame, ValType, WasmValue};

// --- Constants for return codes ---
// Assuming these are defined elsewhere as int64_t
// const RC_ROLLBACK: i64 = -1; // Example value
// const RC_ACCEPT: i64 = -2; // Example value
// const TOO_SMALL: i64 = -3; // Example value
pub const OUT_OF_BOUNDS: i64 = -4; // Example value
// const UNKNOWN_ERROR: i64 = -5;
// const SUCCESS_PLACEHOLDER: i64 = 0; // A generic success code if needed, though returning size is common

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

    // parse the first input of WebAssembly value type into Rust built-in value type
    let guest_write_ptr = if inputs[0].ty() == ValType::I32 {
        inputs[0].to_i32()
    } else {
        return Err(CoreError::Execution(
            wasmedge_sdk::error::CoreExecutionError::FuncSigMismatch,
        ));
    };

    let guest_write_ptr_u32: u32 = guest_write_ptr as u32;

    // 2. Context Retrieval (Replaces reinterpret_cast<hook::HookContext*>(data_ptr) and HOOK_SETUP)
    // Get memory. Assumes memory index 0.
    let mut memory = _caller.memory_mut(0).ok_or_else(|| {
        eprintln!("get_tx_hash_helper: Error: Failed to get memory instance");
        CoreError::Execution(wasmedge_sdk::error::CoreExecutionError::MemoryOutOfBounds)
    })?;
    // println!("memory.size (pages): {}", memory.size());

    // 3. Get Transaction ID (Matches C++ logic)
    let tx_id: Hash256 = apply_ctx.tx.get_transaction_id();
    // match write!(writer, "{:02X}", byte) {
    // info!("Simulated tx_id from apply_ctx: {:02X}", tx_id);

    let tx_id_size = tx_id.size() as u32;

    // 4. Bounds Check (Matches NOT_IN_BOUNDS)
    let memory_size = memory.size() * 65536; // Memory size is in pages (64KiB)
    // println!("memory_size (KiB): {}", memory_size);

    // Check if write_ptr + tx_id_size overflows or goes out of bounds
    // Using checked_add to prevent overflow issues during check.
    let end_ptr = match guest_write_ptr_u32.checked_add(tx_id_size) {
        Some(end) => end,
        None => {
            println!("get_tx_hash_helper: Out of bounds (pointer + size overflow)");
            return Ok(vec![WasmValue::from_i64(OUT_OF_BOUNDS)]);
        }
    };

    if end_ptr > memory_size {
        println!(
            "get_tx_hash_helper: Out of bounds (ptr {} + size {} > memory {})",
            guest_write_ptr_u32, tx_id_size, memory_size
        );
        // Return OUT_OF_BOUNDS as i64
        return Ok(vec![WasmValue::from_i64(OUT_OF_BOUNDS)]);
    }

    // 6. Write to Memory (Matches WRITE_WASM_MEMORY_AND_RETURN)
    // println!(
    //     "get_tx_hash_helper: Writing {} bytes to pointer {}",
    //     tx_id_size, guest_write_ptr_u32
    // );

    let data: [u8; 32] = tx_id.data()[..32].try_into().unwrap();
    let result = memory.write(guest_write_ptr_u32 as usize, data);
    match result {
        Some(()) => {
            // println!("Wasm memory write succeeded!");
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
    // let return_code = tx_id_size as i64;
    // println!("get_tx_hash_helper: Success, wrote {return_code} bytes");
    // TODO: What should WASM Get here? Hooks uses return codes, so we probably need something that's close to `OK`?
    // Ok(vec![WasmValue::from_i64(return_code)])
    Ok(vec![])
}
