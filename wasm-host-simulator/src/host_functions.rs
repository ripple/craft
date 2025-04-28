use crate::host_utils;
use crate::types::core::{ApplyContext, Hash256, Transaction};
use log::error;
use wasmedge_sdk::error::CoreExecutionError;
use wasmedge_sdk::{CallingFrame, Instance, ValType, WasmValue, error::CoreError};
use xrpl_std_lib::core::field_codes::{SF_ACCOUNT, SF_TRANSACTION_TYPE};

// NOTE: This file emulates a host by implementing expected host functions. These implementations
// are wired into the WASM VM from the main.rs, and expect to be called by that code only.

/// Given a pointer to memory in WASM, writes the current EscrowFinish transactions `transactionId`
/// into WASM guest memory using a supplied pointer passed from the user's program.
pub fn get_tx_hash(
    _: &mut (),
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    // This block simulates a Transaction
    let apply_context = ApplyContext { tx: Transaction {} };
    host_utils::escrow_finish_utils::get_tx_hash_helper(apply_context, _caller, inputs)
}

pub fn get_current_escrow_finish_field(
    _: &mut (),
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let apply_ctx = ApplyContext { tx: Transaction {} };

    // check the number of inputs
    if inputs.len() != 3 {
        return Err(CoreError::Execution(CoreExecutionError::FuncSigMismatch));
    }

    // parse the first input of WebAssembly value type into Rust built-in value type
    let guest_write_ptr = if inputs[0].ty() == ValType::I32 {
        inputs[0].to_i32()
    } else {
        return Err(CoreError::Execution(CoreExecutionError::FuncSigMismatch));
    };

    let guest_write_ptr_u32: u32 = guest_write_ptr as u32;

    // parse the second input of WebAssembly value type into Rust built-in value type
    // TODO: What are the times where the guest/WASM should specify this length? The host should
    // arguably know this value, so maybe the WASM doesn't need to specify it?
    // let guest_write_len = if inputs[1].ty() == ValType::I32 {
    //     inputs[1].to_i32()
    // } else {
    //     return Err(CoreError::Execution(CoreExecutionError::FuncSigMismatch));
    // };

    // parse the third input of WebAssembly value type into Rust built-in value type
    let field_code = if inputs[2].ty() == ValType::I32 {
        inputs[2].to_i32()
    } else {
        return Err(CoreError::Execution(CoreExecutionError::FuncSigMismatch));
    };

    // 2. Context Retrieval (Replaces reinterpret_cast<hook::HookContext*>(data_ptr) and HOOK_SETUP)
    // Get memory. Assumes memory index 0.
    let mut memory = _caller.memory_mut(0).ok_or_else(|| {
        eprintln!("get_tx_hash_helper: Error: Failed to get memory instance");
        CoreError::Execution(CoreExecutionError::MemoryOutOfBounds)
    })?;

    // 3. Get Transaction ID (Matches C++ logic)
    let tx_id: Hash256 = apply_ctx.tx.get_transaction_id();
    let tx_id_size = tx_id.size() as u32;

    // 4. Bounds Check (Matches NOT_IN_BOUNDS)
    let memory_size = memory.size() * 65536; // Memory size is in pages (64KiB)

    // Check if write_ptr + tx_id_size overflows or goes out of bounds
    // Using checked_add to prevent overflow issues during check.
    let end_ptr = match guest_write_ptr_u32.checked_add(tx_id_size) {
        Some(end) => end,
        None => {
            println!("get_tx_hash_helper: Out of bounds (pointer + size overflow)");
            return Ok(vec![WasmValue::from_i64(
                host_utils::escrow_finish_utils::OUT_OF_BOUNDS,
            )]);
        }
    };

    if end_ptr > memory_size {
        println!(
            "get_tx_hash_helper: Out of bounds (ptr {} + size {} > memory {})",
            guest_write_ptr_u32, tx_id_size, memory_size
        );
        // Return OUT_OF_BOUNDS as i64
        return Ok(vec![WasmValue::from_i64(
            host_utils::escrow_finish_utils::OUT_OF_BOUNDS,
        )]);
    }

    // Write into WASM Memory
    let data: Vec<u8> = get_field_bytes(&apply_ctx.tx, field_code)?;
    // let data: [u8; 32] = tx_id.data()[..32].try_into().unwrap();
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

fn get_field_bytes(tx: &Transaction, field_code: i32) -> Result<Vec<u8>, CoreError> {
    match field_code {
        SF_ACCOUNT => Ok(vec![]),
        SF_TRANSACTION_TYPE => Ok(vec![]),
        _ => Err(CoreError::Execution(CoreExecutionError::UndefinedElement)),
    }
}

pub fn trace(
    _: &mut (),
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    input: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    // Expect 5 inputs.
    // msg_read_ptr: i32, msg_read_len: i32,
    // data_read_ptr: i32, data_read_len: i32, data_as_hex: i32

    // check the number of inputs
    if input.len() != 5 {
        return Err(CoreError::Execution(CoreExecutionError::FuncSigMismatch));
    }

    let msg_read_ptr = if input[0].ty() == ValType::I32 {
        input[0].to_i32()
    } else {
        return Err(CoreError::Execution(CoreExecutionError::FuncSigMismatch));
    };

    let msg_read_len = if input[1].ty() == ValType::I32 {
        input[1].to_i32()
    } else {
        return Err(CoreError::Execution(CoreExecutionError::FuncSigMismatch));
    };

    let data_read_ptr = if input[2].ty() == ValType::I32 {
        input[2].to_i32()
    } else {
        return Err(CoreError::Execution(CoreExecutionError::FuncSigMismatch));
    };

    let data_read_len = if input[3].ty() == ValType::I32 {
        input[3].to_i32()
    } else {
        return Err(CoreError::Execution(CoreExecutionError::FuncSigMismatch));
    };

    let data_as_hex = if input[4].ty() == ValType::I32 {
        // Get the i32 value
        let value_i32 = input[4].to_i32(); // Assuming this directly returns i32
        // Match the value to convert to bool or return an error
        match value_i32 {
            0 => false,
            1 => true,
            // If an invalid value is supplied, assume `true`
            _ => true,
        }
    } else {
        return Err(CoreError::Execution(CoreExecutionError::FuncSigMismatch));
    };

    // Access the memory instance from the caller context (Memory index 0 is usually the
    // default/primary memory.
    let memory = _caller.memory_ref(0).ok_or_else(|| {
        error!("Failed to get memory instance 0 from caller.");
        CoreError::Execution(CoreExecutionError::MemoryOutOfBounds) // Or a more specific error if available
    })?;

    // Optional: Cast pointers and lengths safely. Pointers/lengths in Wasm memory are usually u32.
    // We assume they are non-negative here. A robust implementation might add checks.
    // let data_read_ptr_u32 = data_read_ptr as u32;
    // let data_read_len_u32 = data_read_len as u32;

    // Read the data from memory.
    let message_vec: Vec<u8> = memory
        .get_data(msg_read_ptr as u32, msg_read_len as u32)
        .map_err(|err| {
            error!(
                "Failed to read memory at ptr={} len={}: {}",
                msg_read_ptr, msg_read_len, err
            );
            // Map the wasmedge MemoryError to your CoreError::Execution type
            CoreError::Execution(CoreExecutionError::MemoryOutOfBounds) // Example mapping
        })?;

    // Convert the byte slice to a Rust String.
    //  Using from_utf8_lossy is robust against invalid UTF-8 sequences from Wasm.
    //  It replaces invalid sequences with the U+FFFD replacement character.
    // let message: Cow<str> = String::from_utf8_lossy(&data);
    let message = String::from_utf8(message_vec)
        .map_err(|err| {
            error!(
                "Failed to read string from memory at ptr={} len={}: {}",
                msg_read_ptr, msg_read_len, err
            );
        })
        .unwrap();

    // Read the message from memory.
    let data_vec: Vec<u8> = memory
        .get_data(data_read_ptr as u32, data_read_len as u32)
        .map_err(|err| {
            error!(
                "Failed to read memory at ptr={} len={}: {}",
                data_read_ptr, data_read_len, err
            );
            // Map the wasmedge MemoryError to your CoreError::Execution type
            CoreError::Execution(CoreExecutionError::MemoryOutOfBounds) // Example mapping
        })?;

    // Convert the byte slice to a Rust String.
    //  Using from_utf8_lossy is robust against invalid UTF-8 sequences from Wasm.
    //  It replaces invalid sequences with the U+FFFD replacement character.

    let data_string = if data_as_hex {
        hex::encode_upper(&data_vec)
    } else {
        String::from_utf8(data_vec)
            // Side-effect logging
            .inspect_err(|err| error!(
                "Failed to read string from memory at ptr={} len={}: {}",
                    data_read_ptr, data_read_len, err)
            )
            .expect("Data was not valid UTF-8")
    };

    // 5. Print the message (or use a proper logging framework).
    println!("{message} {data_string}");

    // --- Return Void ---
    // Return an empty vec! to satisfy the `void` return type.
    Ok(vec![WasmValue::from_i64((data_read_len + msg_read_len + 1) as i64)])
}
