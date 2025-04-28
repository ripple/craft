use log::{debug, error, info};
use std::borrow::Cow;
use wasmedge_sdk::error::CoreExecutionError;
use wasmedge_sdk::{CallingFrame, Instance, ValType, WasmValue, error::CoreError};
use crate::mock_data::LocatorUnpacker;

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

// helper function to write data to the VM memory
pub fn write_data(caller: &mut CallingFrame, data: &[u8]) -> Result<WasmValue, CoreError> {
    // Access memory
    let mut memory = caller
        .memory_mut(0)
        .ok_or_else(|| CoreError::Execution(wasmedge_sdk::error::CoreExecutionError::HostFuncFailed))?;

    // Write the actual data
    let start_addr = memory.size() * 65536;
    info!("write_data: start_addr = {:?}", start_addr);

    // Allocate memory for the data
    memory.grow(1).map_err(|err| {
        error!("Failed to grow memory: {}", err);
        CoreError::Execution(CoreExecutionError::MemoryOutOfBounds)
    })?;
    info!("write_data: memory size after grow = {:?}", memory.size());
    info!("write_data: data = {:?}, {:?}", data.as_ptr() as usize, start_addr);

    memory.set_data(data, start_addr).map_err(|err| {
        error!("Failed to write data to memory: {}", err);
        CoreError::Execution(CoreExecutionError::MemoryOutOfBounds)
    })?;

    info!("write_data: data written to memory");

    // print all the memory
    // info!("memory = {:?}", memory.get_data(0, 18 * 65536));

    // Return the start address of the allocated buffer
    Ok(WasmValue::from_i32(start_addr as i32))
}

// function signature:
// get_current_tx_field(
//     field: i32 // SField value
// )
pub fn get_current_tx_field(
    _: &mut (),
    _inst: &mut Instance,
    caller: &mut CallingFrame,
    input: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    if input.len() != 1 {
        return Err(CoreError::Execution(
            wasmedge_sdk::error::CoreExecutionError::FuncSigMismatch,
        ));
    }

    // parse the first input of WebAssembly value type into Rust built-in value type
    let _sfield = if input[0].ty() == ValType::I32 {
        input[0].to_i32()
    } else {
        return Err(CoreError::Execution(
            wasmedge_sdk::error::CoreExecutionError::FuncSigMismatch,
        ));
    };

    let field_data = b"rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh";

    let ret_ptr = write_data(caller, field_data)?;

    info!("get_current_tx_field: field_data = {:?}", field_data);
    info!("get_current_tx_field: ret_ptr = {:?}", ret_ptr.to_i32() as u32);

    // double check that you can read
    let length = 34;

    let mut memory = caller
        .memory_ref(0)
        .ok_or_else(|| CoreError::Execution(wasmedge_sdk::error::CoreExecutionError::HostFuncFailed))?;

    info!("Printing (a={}, b={})", ret_ptr.to_i32() as u32, length);

    let data: Vec<u8> = memory.get_data(ret_ptr.to_i32() as u32, length).map_err(|err| {
        error!(
            "Failed to read memory at ptr={} len={}: {}",
            ret_ptr.to_i32(),
            length,
            err
        );
        // Map the wasmedge MemoryError to your CoreError::Execution type
        CoreError::Execution(CoreExecutionError::MemoryOutOfBounds) // Example mapping
    })?;
    info!("Printing (data={:?})", data);

    // 4. Convert the byte slice to a Rust String.
    //    Using from_utf8_lossy is robust against invalid UTF-8 sequences from Wasm.
    //    It replaces invalid sequences with the U+FFFD replacement character.
    let message: Cow<str> = String::from_utf8_lossy(&data);

    // 5. Print the message (or use a proper logging framework).
    info!("{}", message);

    Ok(vec![ret_ptr])
}


// helper function to write data to the VM memory
// mostly copied from write_data(), except memory was pre-allocated at start_addr
pub fn write_data_peng(caller: &mut CallingFrame, data: &[u8], start_addr: u32) -> Result<WasmValue, CoreError> {
    if data.len() > 4096 {
        return Err(CoreError::Execution(
            wasmedge_sdk::error::CoreExecutionError::MemoryOutOfBounds
        ));
    }
    // Access memory
    let mut memory = caller
        .memory_mut(0)
        .ok_or_else(|| CoreError::Execution(wasmedge_sdk::error::CoreExecutionError::HostFuncFailed))?;

    // Write the actual data
    // let start_addr = memory.size() * 65536;
    info!("write_data: start_addr = {:?}", start_addr);

    // // Allocate memory for the data
    // memory.grow(1).map_err(|err| {
    //     error!("Failed to grow memory: {}", err);
    //     CoreError::Execution(CoreExecutionError::MemoryOutOfBounds)
    // })?;
    // info!("write_data: memory size after grow = {:?}", memory.size());
    // info!("write_data: data = {:?}, {:?}", data.as_ptr() as usize, start_addr);

    memory.set_data(data, start_addr).map_err(|err| {
        error!("Failed to write data to memory: {}", err);
        CoreError::Execution(CoreExecutionError::MemoryOutOfBounds)
    })?;

    info!("write_data: data written to memory");

    // print all the memory
    // info!("memory = {:?}", memory.get_data(0, 18 * 65536));

    // Return the start address of the allocated buffer
    Ok(WasmValue::from_i32(start_addr as i32))
}

pub fn get_current_tx_field_peng(
    _: &mut (),
    _inst: &mut Instance,
    caller: &mut CallingFrame,
    input: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    //////////////////////////////copied from log()///////////////////////////////////
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
    let memory = caller.memory_ref(0).ok_or_else(|| {
        error!("Failed to get memory instance 0 from caller.");
        CoreError::Execution(CoreExecutionError::MemoryOutOfBounds) // Or a more specific error if available
    })?;

    // 2. Cast pointer and length safely. Pointers/lengths in Wasm memory are usually u32.
    //    We assume they are non-negative here. A robust implementation might add checks.
    let ptr = message_ptr as u32;
    let length = len as u32;

    info!("Printing (a={}, b={})", ptr, length);

    // 3. Read the data from memory.
    //    `read` returns a Result<Vec<u8>, MemoryError>. Map the error.
    let data: Vec<u8> = memory.get_data(ptr, length).map_err(|err| {
        error!("Failed to read memory at ptr={} len={}: {}", ptr, length, err);
        // Map the wasmedge MemoryError to your CoreError::Execution type
        CoreError::Execution(CoreExecutionError::MemoryOutOfBounds) // Example mapping
    })?;
    info!("Printing (data={:?})", data);
    //////////////////////////////end of copied from log()///////////////////////////////////

    let unpacker = match LocatorUnpacker::unpack(data) {
        Some(unpacker) => unpacker,
        None => {
            return Err(CoreError::Execution(
                wasmedge_sdk::error::CoreExecutionError::HostFuncFailed
            ));
        }
    };
    debug!("unpacker = {:?}", unpacker);

    //find data using locators in unpacker

    let field_data = b"rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh";

    let ret_ptr = write_data_peng(caller, field_data, ptr)?;

    //////////////////////////////copied from get_current_tx_field()//////////////////////
    info!("get_current_tx_field: field_data = {:?}", field_data);
    info!("get_current_tx_field: ret_ptr = {:?}", ret_ptr.to_i32() as u32);

    // double check that you can read
    let length = 34;

    let mut memory = caller
        .memory_ref(0)
        .ok_or_else(|| CoreError::Execution(wasmedge_sdk::error::CoreExecutionError::HostFuncFailed))?;

    info!("Printing (a={}, b={})", ret_ptr.to_i32() as u32, length);

    let data: Vec<u8> = memory.get_data(ret_ptr.to_i32() as u32, length).map_err(|err| {
        error!(
            "Failed to read memory at ptr={} len={}: {}",
            ret_ptr.to_i32(),
            length,
            err
        );
        // Map the wasmedge MemoryError to your CoreError::Execution type
        CoreError::Execution(CoreExecutionError::MemoryOutOfBounds) // Example mapping
    })?;
    info!("Printing (data={:?})", data);

    // 4. Convert the byte slice to a Rust String.
    //    Using from_utf8_lossy is robust against invalid UTF-8 sequences from Wasm.
    //    It replaces invalid sequences with the U+FFFD replacement character.
    let message: Cow<str> = String::from_utf8_lossy(&data);

    // 5. Print the message (or use a proper logging framework).
    info!("{}", message);
    //////////////////////////////end of copied from get_current_tx_field() ///////////////////

    Ok(vec![WasmValue::from_i32(field_data.len() as i32)])
}

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

    info!("Printing (a={}, b={})", ptr, length);

    // 3. Read the data from memory.
    //    `read` returns a Result<Vec<u8>, MemoryError>. Map the error.
    let data: Vec<u8> = memory.get_data(ptr, length).map_err(|err| {
        error!("Failed to read memory at ptr={} len={}: {}", ptr, length, err);
        // Map the wasmedge MemoryError to your CoreError::Execution type
        CoreError::Execution(CoreExecutionError::MemoryOutOfBounds) // Example mapping
    })?;
    info!("Printing (data={:?})", data);

    // 4. Convert the byte slice to a Rust String.
    //    Using from_utf8_lossy is robust against invalid UTF-8 sequences from Wasm.
    //    It replaces invalid sequences with the U+FFFD replacement character.
    let message: Cow<str> = String::from_utf8_lossy(&data);

    // 5. Print the message (or use a proper logging framework).
    info!("LOGGING: {}", message);

    // --- Return Void ---
    // Return an empty vec! to satisfy the `void` return type.
    Ok(vec![])
}
