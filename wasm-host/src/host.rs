use log::{error, info};
use std::borrow::Cow;
use wasmedge_sdk::error::CoreExecutionError;
use wasmedge_sdk::{error::CoreError, CallingFrame, Instance, ValType, WasmValue};

/// NOTE: This file emulates a host by implementing expected host functions. These implementations
/// are wired into the WASM VM from the main.rs, and expect to be called by that code only.

// pub fn host_print(
//     _: &mut (),
//     _inst: &mut Instance,
//     _caller: &mut CallingFrame,
//     input: Vec<WasmValue>,
// ) -> Result<Vec<WasmValue>, CoreError> {
//     // check the number of inputs
//     if input.len() != 2 {
//         return Err(CoreError::Execution(
//             wasmedge_sdk::error::CoreExecutionError::FuncSigMismatch,
//         ));
//     }
//
//     // parse the first input of WebAssembly value type into Rust built-in value type
//     let a = if input[0].ty() == ValType::I32 {
//         input[0].to_i32()
//     } else {
//         return Err(CoreError::Execution(
//             wasmedge_sdk::error::CoreExecutionError::FuncSigMismatch,
//         ));
//     };
//
//     // parse the second input of WebAssembly value type into Rust built-in value type
//     let b = if input[1].ty() == ValType::I32 {
//         input[1].to_i32()
//     } else {
//         return Err(CoreError::Execution(
//             wasmedge_sdk::error::CoreExecutionError::FuncSigMismatch,
//         ));
//     };
//
//     // TODO:
//     let c = 0;
//     Ok(vec![WasmValue::from_i32(c)])
// }

pub fn add(
    _: &mut (),
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    input: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    // check the number of inputs
    if input.len() != 2 {
        return Err(CoreError::Execution(
            wasmedge_sdk::error::CoreExecutionError::FuncSigMismatch,
        ));
    }

    // parse the first input of WebAssembly value type into Rust built-in value type
    let a = if input[0].ty() == ValType::I32 {
        input[0].to_i32()
    } else {
        return Err(CoreError::Execution(
            wasmedge_sdk::error::CoreExecutionError::FuncSigMismatch,
        ));
    };

    // parse the second input of WebAssembly value type into Rust built-in value type
    let b = if input[1].ty() == ValType::I32 {
        input[1].to_i32()
    } else {
        return Err(CoreError::Execution(
            wasmedge_sdk::error::CoreExecutionError::FuncSigMismatch,
        ));
    };

    info!("Adding (a={}, b={})", a, b);
    let c = a + b;

    Ok(vec![WasmValue::from_i32(c)])
}

/// Logs a debug message, with behavior depending on the target architecture.
pub fn log(
    _: &mut (),
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    input: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    // check the number of inputs
    if input.len() != 2 {
        return Err(CoreError::Execution(
            wasmedge_sdk::error::CoreExecutionError::FuncSigMismatch,
        ));
    }

    // parse the first input of WebAssembly value type into Rust built-in value type
    let message_ptr = if input[0].ty() == ValType::I32 {
        input[0].to_i32()
    } else {
        return Err(CoreError::Execution(
            wasmedge_sdk::error::CoreExecutionError::FuncSigMismatch,
        ));
    };

    // parse the second input of WebAssembly value type into Rust built-in value type
    let len = if input[1].ty() == ValType::I32 {
        input[1].to_i32()
    } else {
        return Err(CoreError::Execution(
            wasmedge_sdk::error::CoreExecutionError::FuncSigMismatch,
        ));
    };

    // --- Access Memory and Read String ---
    info!("Printing value statically allocated from inside WebAssembly running in WASM VM...");
    // 1. Get the memory instance from the caller context.
    //    Memory index 0 is usually the default/primary memory.
    let memory = _caller.memory_ref(0).ok_or_else(|| {
        error!("Failed to get memory instance 0 from caller.");
        CoreError::Execution(CoreExecutionError::MemoryOutOfBounds) // Or a more specific error if available
    })?;

    // 2. Cast pointer and length safely. Pointers/lengths in Wasm memory are usually u32.
    //    We assume they are non-negative here. A robust implementation might add checks.
    let ptr = message_ptr as u32;
    let length = len as u32;

    // 3. Read the data from memory.
    //    `read` returns a Result<Vec<u8>, MemoryError>. Map the error.
    let data: Vec<u8> = memory.get_data(ptr, length).map_err(|err| {
        error!("Failed to read memory at ptr={} len={}: {}", ptr, length, err);
        // Map the wasmedge MemoryError to your CoreError::Execution type
        CoreError::Execution(CoreExecutionError::MemoryOutOfBounds) // Example mapping
    })?;

    // 4. Convert the byte slice to a Rust String.
    //    Using from_utf8_lossy is robust against invalid UTF-8 sequences from Wasm.
    //    It replaces invalid sequences with the U+FFFD replacement character.
    let message: Cow<str> = String::from_utf8_lossy(&data);

    // 5. Print the message (or use a proper logging framework).
    info!("{}", message);

    // --- Return Void ---
    // Return an empty vec! to satisfy the `void` return type.
    Ok(vec![])
}

