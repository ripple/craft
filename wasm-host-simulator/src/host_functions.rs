use crate::types::core::{ApplyContext, Transaction};
use crate::types::wrappers::AmountWrapper;
use log::{debug, error};
use wasmedge_sdk::error::CoreExecutionError;
use wasmedge_sdk::{CallingFrame, Instance, ValType, WasmValue, error::CoreError};
use xrpl_std_lib::core::amount::Amount;
use xrpl_std_lib::core::amount::xrp_amount::XrpAmount;
use xrpl_std_lib::core::constants::ACCOUNT_ONE;
use xrpl_std_lib::core::error_codes::OUT_OF_BOUNDS;
use xrpl_std_lib::core::field_codes::{SF_ACCOUNT, SF_FEE, SF_TRANSACTION_TYPE};
use xrpl_std_lib::core::types::{Hash256, TransactionType};
// NOTE: This file emulates a host by implementing expected host functions. These implementations
// are wired into the WASM VM from the main.rs, and expect to be called by that code only.

const DEFAULT_TX: Transaction = Transaction {
    transaction_id: Hash256([0xFF; 32]),
    account_id: ACCOUNT_ONE,
    transaction_type: TransactionType::EscrowFinish,
    fee: Amount::Xrp(XrpAmount(12)),
};

/// Given a pointer to memory in WASM, writes the current EscrowFinish transactions `transactionId`
/// into WASM guest memory using a supplied pointer passed from the user's program.
pub fn get_tx_hash(
    _: &mut (),
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    // This block simulates a Transaction
    let apply_ctx = ApplyContext { tx: DEFAULT_TX };

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
    debug!("memory.size (pages): {}", memory.size());

    // 3. Get Transaction ID (Matches C++ logic)
    let tx_id: Hash256 = apply_ctx.tx.transaction_id;
    // match write!(writer, "{:02X}", byte) {
    // info!("Simulated tx_id from apply_ctx: {:02X}", tx_id);

    let tx_id_size = 32u32;

    // 4. Bounds Check (Matches NOT_IN_BOUNDS)
    let memory_size = memory.size() * 65536; // Memory size is in pages (64KiB)
    debug!("memory_size (KiB): {}", memory_size);

    // Check if write_ptr + tx_id_size overflows or goes out of bounds
    // Using checked_add to prevent overflow issues during check.
    let end_ptr = match guest_write_ptr_u32.checked_add(tx_id_size) {
        Some(end) => end,
        None => {
            println!("get_tx_hash_helper: Out of bounds (pointer + size overflow)");
            return Ok(vec![WasmValue::from_i64(OUT_OF_BOUNDS as i64)]);
        }
    };

    if end_ptr > memory_size {
        println!(
            "get_tx_hash_helper: Out of bounds (ptr {} + size {} > memory {})",
            guest_write_ptr_u32, tx_id_size, memory_size
        );
        // Return OUT_OF_BOUNDS as i64
        return Ok(vec![WasmValue::from_i64(OUT_OF_BOUNDS as i64)]);
    }

    // 6. Write to Memory (Matches WRITE_WASM_MEMORY_AND_RETURN)
    debug!(
        "get_tx_hash_helper: Writing {} bytes to pointer {}",
        tx_id_size, guest_write_ptr_u32
    );

    let data: [u8; 32] = tx_id.0[..32].try_into().unwrap();
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

pub fn get_current_escrow_finish_field(
    _: &mut (),
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    // This block simulates a Transaction
    let apply_ctx = ApplyContext { tx: DEFAULT_TX };

    // check the number of inputs
    if inputs.len() != 3 {
        return Err(CoreError::Execution(CoreExecutionError::FuncSigMismatch));
    }

    // This a pointer to the memory allocated by WASM (i.e., guest memory)
    let guest_write_ptr = if inputs[0].ty() == ValType::I32 {
        inputs[0].to_i32()
    } else {
        return Err(CoreError::Execution(CoreExecutionError::FuncSigMismatch));
    };

    let guest_write_ptr_u32: u32 = guest_write_ptr as u32;
    // println!("guest_write_ptr_u32: {}", guest_write_ptr_u32);

    let guest_write_len = if inputs[1].ty() == ValType::I32 {
        inputs[1].to_i32()
    } else {
        return Err(CoreError::Execution(CoreExecutionError::FuncSigMismatch));
    };
    // println!("guest_write_len: {}", guest_write_len);

    // parse the third input of WebAssembly value type into Rust built-in value type
    let field_code = if inputs[2].ty() == ValType::I32 {
        inputs[2].to_i32()
    } else {
        return Err(CoreError::Execution(CoreExecutionError::FuncSigMismatch));
    };
    // println!("field_code: {} (0x{:0x})", field_code, field_code);

    // 2. Context Retrieval (Replaces reinterpret_cast<hook::HookContext*>(data_ptr) and HOOK_SETUP)
    // Get memory. Assumes memory index 0.
    let mut memory = _caller.memory_mut(0).ok_or_else(|| {
        eprintln!("get_tx_hash_helper: Error: Failed to get memory instance");
        CoreError::Execution(CoreExecutionError::MemoryOutOfBounds)
    })?;

    // 4. Bounds Check (Matches NOT_IN_BOUNDS)
    let memory_size = memory.size() * 65536; // Memory size is in pages (64KiB)

    // Check if write_ptr + tx_id_size overflows or goes out of bounds
    // Using checked_add to prevent overflow issues during check.
    let end_ptr = match guest_write_ptr_u32.checked_add(guest_write_len as u32) {
        Some(end) => end,
        None => {
            println!("get_tx_hash_helper: Out of bounds (pointer + size overflow)");
            return Ok(vec![WasmValue::from_i64(OUT_OF_BOUNDS as i64)]);
        }
    };

    if end_ptr > memory_size {
        println!(
            "get_tx_hash_helper: Out of bounds (ptr {} + size {} > memory {})",
            guest_write_ptr_u32, guest_write_len, memory_size
        );
        // Return OUT_OF_BOUNDS as i64
        return Ok(vec![WasmValue::from_i64(OUT_OF_BOUNDS as i64)]);
    }

    // Write into WASM Memory
    // let data_to_write: [u8; 20] = apply_ctx.tx.account_id.0.clone();
    let data_to_write: Vec<u8> = get_field_bytes(apply_ctx.tx, field_code)?;
    // This is unsafe if an emulated VM supports 128-bit addressing
    let data_to_write_len = data_to_write.as_slice().len(); //as u64 as i64;

    // println!("Data: {}", hex::encode(&data_to_write));
    // println!("Guest write_ptr: {}", guest_write_ptr_u32);
    // println!("Guest write_len: {}", guest_write_len); // Add logging
    // println!("Actual data len: {}", data_to_write_len); // Add logging

    // This check ensures the _actual_ data len doesn't exceed what the guest is indicating.
    if data_to_write_len > guest_write_len as usize {
        eprintln!(
            "Error: Data size ({}) exceeds guest buffer size ({}).",
            data_to_write.len(),
            guest_write_len
        );
        // Return an appropriate error code to WASM.
        // Using a custom error code might be better than reusing OUT_OF_BOUNDS.
        // For example, define a BUFFER_TOO_SMALL = -7 or similar.
        // Let's use OUT_OF_BOUNDS for now as an example if you haven't defined others.
        // Or a new BUFFER_TOO_SMALL code
        return Ok(vec![WasmValue::from_i64(OUT_OF_BOUNDS as i64)]);
    }

    debug!(
        "WRITING (ptr={} len={}) DATA: {:?}",
        guest_write_ptr_u32,
        &data_to_write.len(),
        data_to_write
    );

    let result = memory.set_data(&data_to_write, guest_write_ptr_u32);

    match result {
        Ok(()) => {
            debug!(
                "Success Wasm memory wrote (ptr={} len={}) DATA: {:?}",
                guest_write_ptr_u32, guest_write_len, data_to_write
            );
        }
        Err(error) => {
            eprintln!("Error: Wasm memory write failed: {}", error);
            return Err(CoreError::Execution(CoreExecutionError::MemoryOutOfBounds));
        }
    }

    Ok(vec![WasmValue::from_i64(guest_write_len as i64)])
}

fn get_field_bytes(tx: Transaction, field_code: i32) -> Result<Vec<u8>, CoreError> {
    match field_code {
        SF_ACCOUNT => {
            let account = tx.account_id.0;
            debug!("AccountID: {}", hex::encode(account));
            Ok(account.clone().into())
        }
        SF_TRANSACTION_TYPE => {
            let tx_type: TransactionType = tx.transaction_type;
            let vec: Vec<u8> = tx_type.into();
            debug!("TransactionType: {}", hex::encode(&vec)); // Add this
            assert_eq!(vec.len(), 2);
            Ok(vec)
        }
        SF_FEE => {
            let fee = tx.fee;
            let fee_wrapper = AmountWrapper(fee);
            let vec: Vec<u8> = fee_wrapper.into();
            Ok(vec)
        }
        // SF_HASH => {
        //     let tx_type: TransactionType = tx.;
        //     let vec: Vec<u8> = tx_type.into();
        //     assert_eq!(vec.len(), 2);
        //     Ok(vec)
        // }
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

    debug!(
        "trace() params: msg_read_ptr={} msg_read_len={} data_read_ptr={} data_read_len={}",
        msg_read_ptr, msg_read_len, data_read_ptr, data_read_len
    );

    let message = read_utf8_from_wasm(_caller, msg_read_ptr, msg_read_len)?;
    let data_string = read_hex_from_wasm(_caller, data_read_ptr, data_read_len, data_as_hex)?;
    if data_read_len > 0 {
        // 5. Print the message (or use a proper logging framework).
        println!("WASM TRACE: {message} ({data_string} | {} data bytes)", data_read_len);
    } else {
        // 5. Print the message (or use a proper logging framework).
        println!("WASM TRACE: {message}");
    }

    // --- Return Void ---
    // Return an empty vec! to satisfy the `void` return type.
    Ok(vec![WasmValue::from_i64((data_read_len + msg_read_len + 1) as i64)])
}

pub fn trace_num(
    _: &mut (),
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    input: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    // Expect 3 inputs.

    // check the number of inputs
    if input.len() != 3 {
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

    let number = if input[2].ty() == ValType::I64 {
        input[2].to_i64()
    } else {
        return Err(CoreError::Execution(CoreExecutionError::FuncSigMismatch));
    };

    debug!(
        "trace() params: msg_read_ptr={} msg_read_len={} number={} ",
        msg_read_ptr, msg_read_len, number
    );

    let message = read_utf8_from_wasm(_caller, msg_read_ptr, msg_read_len)?;
    // 5. Print the message (or use a proper logging framework).
    println!("WASM TRACE: {message} {number}");

    Ok(vec![WasmValue::from_i64(0)])
}

fn read_utf8_from_wasm(_caller: &mut CallingFrame, msg_read_ptr: i32, msg_read_len: i32) -> Result<String, CoreError> {
    // Read the data from memory.
    let message_vec: Vec<u8> = read_bytes_from_wasm(_caller, msg_read_ptr, msg_read_len)?;

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

    Ok(message)
}

fn read_hex_from_wasm(
    _caller: &mut CallingFrame,
    data_read_ptr: i32,
    data_read_len: i32,
    data_as_hex: bool,
) -> Result<String, CoreError> {
    return if data_as_hex {
        // Read the data from memory.
        let bytes_vec: Vec<u8> = read_bytes_from_wasm(_caller, data_read_ptr, data_read_len)?;
        let mut final_hex_string = "0x".to_owned();
        let hex_data = hex::encode_upper(&bytes_vec);
        final_hex_string.push_str(hex_data.as_str());
        Ok(final_hex_string)
    } else {
        return read_utf8_from_wasm(_caller, data_read_ptr, data_read_len);
    };
}

fn read_bytes_from_wasm(
    _caller: &mut CallingFrame,
    data_read_ptr: i32,
    data_read_len: i32,
) -> Result<Vec<u8>, CoreError> {
    // Access the memory instance from the caller context (Memory index 0 is usually the
    // default/primary memory.
    let memory = _caller.memory_ref(0).ok_or_else(|| {
        error!("Failed to get memory instance 0 from caller.");
        CoreError::Execution(CoreExecutionError::MemoryOutOfBounds) // Or a more specific error if available
    })?;
    // println!("The memory size (in pages) : {}", memory.size());

    // Read the message from memory.
    memory
        .get_data(data_read_ptr as u32, data_read_len as u32)
        .map_err(|err| {
            error!(
                "Failed to read memory at ptr={} len={}: {}",
                data_read_ptr, data_read_len, err
            );
            // Map the wasmedge MemoryError to your CoreError::Execution type
            CoreError::Execution(CoreExecutionError::MemoryOutOfBounds) // Example mapping
        })
}
