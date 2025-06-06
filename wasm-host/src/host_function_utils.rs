use log::error;
<<<<<<< HEAD
use wasmedge_sdk::error::{CoreError, CoreExecutionError};
use wasmedge_sdk::CallingFrame;
=======
use wasmedge_sdk::CallingFrame;
use wasmedge_sdk::error::{CoreError, CoreExecutionError};
>>>>>>> origin/main

/// Read a message the WASM guest and treat is as a UTF-8 string.  
pub(crate) fn read_utf8_from_wasm(
    _caller: &mut CallingFrame,
    msg_read_ptr: i32,
    msg_read_len: i32,
) -> Result<String, CoreError> {
    // Read the data from memory.
    let message_vec: Vec<u8> = read_bytes_from_wasm_helper(_caller, msg_read_ptr, msg_read_len)?;

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

/// Read data the WASM guest and treat is as HEX bytes.
pub(crate) fn read_hex_from_wasm(
    _caller: &mut CallingFrame,
    data_read_ptr: i32,
    data_read_len: i32,
    data_as_hex: bool,
) -> Result<String, CoreError> {
    if data_as_hex {
        // Read the data from memory.
        let bytes_vec: Vec<u8> =
            read_bytes_from_wasm_helper(_caller, data_read_ptr, data_read_len)?;
        let mut final_hex_string = "0x".to_owned();
        let hex_data = hex::encode_upper(&bytes_vec);
        final_hex_string.push_str(hex_data.as_str());
        Ok(final_hex_string)
    } else {
        read_utf8_from_wasm(_caller, data_read_ptr, data_read_len)
    }
}

/// Read `data_read_len` bytes from the WASM guest starting at the memory location pointed to by `data_read_ptr`.
fn read_bytes_from_wasm_helper(
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
